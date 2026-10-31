//! 真实Sandbox proxy/ACP原调用的Before控制：不在ToolContext中复制command-only guard。
use super::super::*;
use agent_client_protocol::{Agent as AcpAgent, ByteStreams, Error as RpcError, Responder};
use echo_agent::sandbox::{ResourceLimits, SandboxCommand, SandboxExecutor};
use echo_sdk_protocol::methods::{
    AgentComponentCallResultWire, AgentComponentResultWire, ExtensionCancelNotice,
    SandboxStreamCompleteWire,
};
use serde_json::json;
use std::sync::atomic::AtomicUsize;
use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};

struct DropProof(Arc<AtomicUsize>);
impl Drop for DropProof {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::AcqRel);
    }
}

fn execution_value() -> Result<WireValue, String> {
    WireValue::from_json(json!({"exit_code":0,"stdout":"中文🙂","stderr":"","duration":{"secs":0,"nanos":0},"sandbox_type":"fixture","timed_out":false,"cancelled":false,"output_truncated":false,"stdout_bytes":10,"stderr_bytes":0}))
        .map_err(|error| error.to_string())
}

type ObservingTask =
    tokio::task::JoinHandle<echo_agent::error::Result<echo_agent::sandbox::ExecutionResult>>;
type ObservingSlot = Arc<Mutex<Option<ObservingTask>>>;
type ObservingResult =
    Result<echo_agent::error::Result<echo_agent::sandbox::ExecutionResult>, tokio::task::JoinError>;

/// 只记录原Host writer实际写出的字节，不把mock入口计数当作RPC提交事实。
struct RecordedHostWriter {
    inner: tokio::io::WriteHalf<tokio::io::DuplexStream>,
    bytes: Arc<Mutex<Vec<u8>>>,
}

impl tokio::io::AsyncWrite for RecordedHostWriter {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        bytes: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        match tokio::io::AsyncWrite::poll_write(std::pin::Pin::new(&mut self.inner), cx, bytes) {
            std::task::Poll::Ready(Ok(count)) => {
                let Some(written) = bytes.get(..count) else {
                    return std::task::Poll::Ready(Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "writer returned an invalid byte count",
                    )));
                };
                self.bytes
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .extend_from_slice(written);
                std::task::Poll::Ready(Ok(count))
            }
            result => result,
        }
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        tokio::io::AsyncWrite::poll_flush(std::pin::Pin::new(&mut self.inner), cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        tokio::io::AsyncWrite::poll_shutdown(std::pin::Pin::new(&mut self.inner), cx)
    }
}

fn recorded_host_invocations(
    bytes: &[u8],
) -> Result<Vec<(serde_json::Value, AgentComponentOperationWire)>, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let mut invocations = Vec::new();
    for line in text.lines().filter(|line| !line.is_empty()) {
        let frame: serde_json::Value =
            serde_json::from_str(line).map_err(|error| error.to_string())?;
        if frame.get("method").and_then(serde_json::Value::as_str)
            != Some("_echo_agent/extension/invoke")
        {
            continue;
        }
        let id = frame
            .get("id")
            .filter(|id| !id.is_null())
            .cloned()
            .ok_or("real Host request ID missing")?;
        let call: ExtensionInvokeCall = serde_json::from_value(
            frame
                .get("params")
                .cloned()
                .ok_or("real Host request params missing")?,
        )
        .map_err(|error| error.to_string())?;
        let input = match call.invocation {
            ExtensionInvocation::AgentComponentCall(input)
            | ExtensionInvocation::AgentComponentCallStream(input) => input,
            _ => return Err("unexpected Host invocation in command-only fixture".to_string()),
        };
        invocations.push((id, input.call.operation()));
    }
    Ok(invocations)
}

/// 句柄只在真正Ready后取走。借用poll，timeout丢等待不丢原JoinHandle。
async fn join_observing_task(slot: &ObservingSlot) -> Option<ObservingResult> {
    futures::future::poll_fn(|cx| {
        let mut owner = slot.lock().unwrap_or_else(|error| error.into_inner());
        let Some(task) = owner.as_mut() else {
            return std::task::Poll::Ready(None);
        };
        match std::future::Future::poll(std::pin::Pin::new(task), cx) {
            std::task::Poll::Ready(result) => {
                let _ = owner.take();
                std::task::Poll::Ready(Some(result))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    })
    .await
}

fn observe(condition: bool, error: impl Into<String>) -> Result<(), String> {
    if condition { Ok(()) } else { Err(error.into()) }
}

fn preserve_first_error(first: &mut Option<String>, error: impl Into<String>) {
    let error = error.into();
    match first {
        Some(original) => {
            original.push_str("; fixture teardown: ");
            original.push_str(&error);
        }
        None => *first = Some(error),
    }
}

#[derive(Default)]
struct RemoteCompletion {
    started: AtomicUsize,
    completed: AtomicUsize,
    cleanup_calls: AtomicUsize,
    changed: Notify,
    errors: Mutex<Vec<String>>,
}

impl RemoteCompletion {
    /// 只在原handler处理完response/清理、即将Ready时调用，不由DropGuard冒充完成。
    fn returned(&self, result: &Result<(), RpcError>) {
        if let Err(error) = result {
            self.errors
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .push(error.to_string());
        }
        self.completed.fetch_add(1, Ordering::AcqRel);
        self.changed.notify_waiters();
    }

    async fn wait_returned(&self, require_original_entry: bool) {
        loop {
            let changed = self.changed.notified();
            tokio::pin!(changed);
            changed.as_mut().enable();
            let started = self.started.load(Ordering::Acquire);
            if started == self.completed.load(Ordering::Acquire)
                && (!require_original_entry || started != 0)
            {
                return;
            }
            changed.await;
        }
    }
}

/// Only the observing waiter may be aborted, never the remote physical handler.
fn drop_observing_wait(slot: &ObservingSlot) {
    if let Some(task) = slot
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .as_ref()
    {
        task.abort();
    }
}

/// 目标错误先保存；统一放行、等原response/handler与观察任务，再结束连接并报告失败。
async fn command_owner_case(style: &'static str) -> Result<(), String> {
    let runtime_root = std::env::var("ECHO_AGENT_SDK_CONSUMER_ROOT")
        .map_err(|error| format!("explicit task directory required: {error}"))?;
    std::fs::create_dir_all(&runtime_root).map_err(|error| error.to_string())?;
    let directory = tempfile::Builder::new()
        .prefix("sandbox-command-owner-")
        .tempdir_in(runtime_root)
        .map_err(|error| error.to_string())?;
    let config: crate::SdkHostConfig = serde_json::from_value(json!({
        "schema_version":1,"default_agent":{
            "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
            "agent":{"name":"sandbox-owner","enable_tools":true}
        }
    })).map_err(|error| error.to_string())?;
    let prepared = config.prepare().map_err(|error| error.to_string())?;
    let mut limits = crate::config::SdkProfileLimits::default();
    if style == "timeout" {
        limits.callback_timeout_secs = 1;
    }
    let state = CoreProfileState::install(
        directory.path(),
        limits,
        prepared.framework,
        prepared.llm_client,
    )
    .map_err(|error| error.to_string())?;
    let sessions = Arc::new(echo_agent::acp::SessionRegistry::new(
        Arc::new(state.session_factory.clone()),
        4,
    ));
    state.bind_services(Arc::new(echo_agent::acp::AcpConnectionServices::new(
        sessions,
        Arc::new(echo_agent::acp::AcpAdapterConfig::default()),
    )));
    let bridge = state.extension_bridge.clone();
    let entered = CancellationToken::new();
    let release = CancellationToken::new();
    let cleanup = CancellationToken::new();
    let terminal = CancellationToken::new();
    let cancelled = CancellationToken::new();
    let finish = CancellationToken::new();
    let physically_done = Arc::new(AtomicBool::new(false));
    let drops = Arc::new(AtomicUsize::new(0));
    let dropped_at_entry = Arc::new(AtomicUsize::new(usize::MAX));
    let remote = Arc::new(RemoteCompletion::default());
    let observing: ObservingSlot = Arc::new(Mutex::new(None));
    let cleanup_confirmed = Arc::new(AtomicBool::new(false));
    let first_failure = Arc::new(Mutex::new(None::<String>));
    let sdk_finish = finish.clone();
    let (host_io, sdk_io) = tokio::io::duplex(65_536);
    let (host_read, host_write) = tokio::io::split(host_io);
    let host_wire_bytes = Arc::new(Mutex::new(Vec::new()));
    let host_write = RecordedHostWriter {
        inner: host_write,
        bytes: host_wire_bytes.clone(),
    };
    let (sdk_read, sdk_write) = tokio::io::split(sdk_io);
    let sdk = Client
        .builder()
        .on_receive_notification(
            {
                let cancelled = cancelled.clone();
                async move |_notice: ExtensionCancelNotice, _connection: ConnectionTo<AcpAgent>| {
                    cancelled.cancel();
                    Ok(())
                }
            },
            agent_client_protocol::on_receive_notification!(),
        )
        .on_receive_request(
            {
                let entered = entered.clone();
                let release = release.clone();
                let cleanup = cleanup.clone();
                let terminal = terminal.clone();
                let drops = drops.clone();
                let dropped_at_entry = dropped_at_entry.clone();
                let physically_done = physically_done.clone();
                let remote = remote.clone();
                async move |call: ExtensionInvokeCall,
                            responder: Responder<ExtensionInvokeOutcome>,
                            connection: ConnectionTo<AcpAgent>| {
                    // ACP的原请求handler持有顺序dispatch；原mock producer由同连接持有，
                    // 让取消通知可被处理。RemoteCompletion仍在真实response/清理末poll记账。
                    let entered = entered.clone();
                    let release = release.clone();
                    let cleanup = cleanup.clone();
                    let terminal = terminal.clone();
                    let drops = drops.clone();
                    let dropped_at_entry = dropped_at_entry.clone();
                    let physically_done = physically_done.clone();
                    let remote = remote.clone();
                    let producer_connection = connection.clone();
                    connection.spawn(async move {
                        let connection = producer_connection;
                        remote.started.fetch_add(1, Ordering::AcqRel);
                        remote.changed.notify_waiters();
                        let result = async {
                            let streaming = matches!(
                                &call.invocation,
                                ExtensionInvocation::AgentComponentCallStream(_)
                            );
                            let input = match call.invocation {
                                ExtensionInvocation::AgentComponentCall(input)
                                | ExtensionInvocation::AgentComponentCallStream(input) => input,
                                _ => {
                                    return Err(RpcError::internal_error()
                                        .data("wrong SandboxExecutor invocation"));
                                }
                            };
                            if matches!(input.call, AgentComponentCallInputWire::SandboxIsAvailable)
                            {
                                return responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::AgentComponentCall(
                                        AgentComponentResultWire {
                                            component: AgentComponentKindWire::SandboxExecutor,
                                            result:
                                                AgentComponentCallResultWire::SandboxIsAvailable {
                                                    available: true,
                                                },
                                        },
                                    ),
                                });
                            }
                            if matches!(input.call, AgentComponentCallInputWire::SandboxCleanup) {
                                remote.cleanup_calls.fetch_add(1, Ordering::AcqRel);
                                return responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::AgentComponentCall(
                                        AgentComponentResultWire {
                                            component: AgentComponentKindWire::SandboxExecutor,
                                            result: AgentComponentCallResultWire::SandboxCleanup,
                                        },
                                    ),
                                });
                            }
                            let operation = input.call.operation();
                            let command = match input.call {
                            AgentComponentCallInputWire::SandboxExecute { command }
                            | AgentComponentCallInputWire::SandboxExecuteStream { command }
                            | AgentComponentCallInputWire::SandboxExecuteWithLimits {
                                command,
                                ..
                            }
                            | AgentComponentCallInputWire::SandboxExecuteWithLimitsAndCancel {
                                command,
                                ..
                            } => command,
                            _ => {
                                return Err(RpcError::internal_error()
                                    .data("unexpected component operation"));
                            }
                        }
                        .into_json()
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                            if command.get("resource_guards").is_some()
                                || command.get("cancel").is_some()
                            {
                                return Err(RpcError::internal_error()
                                    .data("runtime command fields crossed wire"));
                            }
                            if command.pointer("/kind/Shell") != Some(&json!("printf 中文🙂")) {
                                return Err(RpcError::internal_error()
                                    .data("original command value changed"));
                            }
                            dropped_at_entry
                                .store(drops.load(Ordering::Acquire), Ordering::Release);
                            entered.cancel();
                            let mut pending = Some(responder);
                            let stream = call.stream;
                            if style == "stream_early_ack" {
                                pending
                                    .take()
                                    .ok_or_else(|| {
                                        RpcError::internal_error().data("response absent")
                                    })?
                                    .respond(ExtensionInvokeOutcome::Stream {
                                        stream: stream.clone().ok_or_else(|| {
                                            RpcError::internal_error().data("stream absent")
                                        })?,
                                    })?;
                            }
                            release.cancelled().await;
                            if streaming {
                                if style == "stream_early_ack" {
                                    physically_done.store(true, Ordering::Release);
                                }
                                connection.send_notification(ExtensionStreamEvent::Complete {
                                    stream: stream.clone().ok_or_else(|| {
                                        RpcError::internal_error().data("stream absent")
                                    })?,
                                    sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from(
                                        "1".to_string(),
                                    )
                                    .map_err(|error| {
                                        RpcError::internal_error().data(error.to_string())
                                    })?,
                                    value: ExtensionStreamCompleteValue::AgentComponent(
                                        AgentComponentStreamCompleteWire::Sandbox(
                                            SandboxStreamCompleteWire::Complete {
                                                result: execution_value().map_err(|error| {
                                                    RpcError::internal_error().data(error)
                                                })?,
                                            },
                                        ),
                                    ),
                                })?;
                                terminal.cancel();
                                if style != "stream_early_ack" {
                                    cleanup.cancelled().await;
                                    physically_done.store(true, Ordering::Release);
                                    pending
                                        .take()
                                        .ok_or_else(|| {
                                            RpcError::internal_error().data("response absent")
                                        })?
                                        .respond(ExtensionInvokeOutcome::Stream {
                                            stream: stream.ok_or_else(|| {
                                                RpcError::internal_error().data("stream absent")
                                            })?,
                                        })?;
                                }
                                return Ok(());
                            }
                            cleanup.cancelled().await;
                            physically_done.store(true, Ordering::Release);
                            let value = execution_value()
                                .map_err(|error| RpcError::internal_error().data(error))?;
                            let result = match operation {
                            AgentComponentOperationWire::SandboxExecute => {
                                AgentComponentCallResultWire::SandboxExecute { result: value }
                            }
                            AgentComponentOperationWire::SandboxExecuteWithLimits => {
                                AgentComponentCallResultWire::SandboxExecuteWithLimits {
                                    result: value,
                                }
                            }
                            AgentComponentOperationWire::SandboxExecuteWithLimitsAndCancel => {
                                AgentComponentCallResultWire::SandboxExecuteWithLimitsAndCancel {
                                    result: value,
                                }
                            }
                            _ => {
                                return Err(
                                    RpcError::internal_error().data("wrong nonstream operation")
                                );
                            }
                        };
                            pending
                                .take()
                                .ok_or_else(|| RpcError::internal_error().data("response absent"))?
                                .respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::AgentComponentCall(
                                        AgentComponentResultWire {
                                            component: AgentComponentKindWire::SandboxExecutor,
                                            result,
                                        },
                                    ),
                                })
                        }
                        .await;
                        // response处理完成后、handler同一poll返回Ready前记账；没有后续await或未持有任务。
                        remote.returned(&result);
                        result
                    })?;
                    Ok(())
                }
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_with(
            ByteStreams::new(sdk_write.compat_write(), sdk_read.compat()),
            async move |_| {
                sdk_finish.cancelled().await;
                Ok(())
            },
        );
    let notification_shared = state.extension_shared.clone();
    let host_observing = observing.clone();
    let host_remote = remote.clone();
    let host_cleanup_confirmed = cleanup_confirmed.clone();
    let host_first_failure = first_failure.clone();
    let host = AcpAgent.builder().on_receive_notification(
        async move |event: ExtensionStreamEvent, _connection: ConnectionTo<Client>| {
            notification_shared.deliver_stream_event(event).map_err(|error| RpcError::internal_error().data(error))
        }, agent_client_protocol::on_receive_notification!(),
    ).connect_with(ByteStreams::new(host_write.compat_write(), host_read.compat()), async move |connection| {
        // 仅异常兜底发signal，不用这些DropGuard声明异步收尾已完成。
        let _finish_fallback = finish.clone().drop_guard();
        let _release_fallback = release.clone().drop_guard();
        let _cleanup_fallback = cleanup.clone().drop_guard();
        bridge.shared.bind_connection(connection);
        let mut first = None;
        let setup = (|| {
            let descriptor = serde_json::from_value(json!({"kind":"agent_component","descriptor_version":1,"component":"sandbox_executor","name":"command-only","capabilities":{"supports_streaming":style!="fallback","isolation_level":"process"}})).map_err(|error| error.to_string())?;
            let (extension, _) = state.handles.register_extension(
                super::super::super::handles::ExtensionRegistrationLimits {max_extensions:2,max_descriptor_bytes:65_536},
                ExtensionKind::AgentComponent, "command-only", descriptor, None, false,
            ).map_err(|error| error.message)?;
            ExtensionAgentComponentProxy::new(bridge.clone(), extension, "sandbox-command-session".into()).ok_or_else(|| "actual SandboxExecutor proxy absent".to_string())
        })();
        let mut observing_stream = None;
        if let Ok(proxy) = &setup {
            let command_token = Arc::new(CancellationToken::new());
            let explicit_token = Arc::new(CancellationToken::new());
            let command = SandboxCommand::shell("printf 中文🙂")
                .with_resource_guards(vec![echo_agent::tools::InvocationResourceGuard::new(DropProof(drops.clone()))])
                .with_cancel(Some(command_token.clone()));
            let control_result = {
                let control = async {
                    if style.starts_with("stream") {
                        observing_stream = Some(proxy.execute_stream(command).await.map_err(|error| error.to_string())?);
                        tokio::time::timeout(Duration::from_secs(2), entered.cancelled()).await.map_err(|error| error.to_string())?;
                        observe(dropped_at_entry.load(Ordering::Acquire) == 0, "command-only guard released before actual reverse producer entry")?;
                        observe(drops.load(Ordering::Acquire) == 0, "setup return released command resources")?;
                        if style == "stream_drop" {
                            drop(observing_stream.take());
                            tokio::time::timeout(Duration::from_secs(2), cancelled.cancelled()).await.map_err(|error| error.to_string())?;
                            observe(drops.load(Ordering::Acquire) == 0, "observer drop is not physical stop")?;
                            release.cancel();
                            tokio::time::timeout(Duration::from_secs(2), terminal.cancelled()).await.map_err(|error| error.to_string())?;
                            observe(!physically_done.load(Ordering::Acquire), "late response fixture stopped before cleanup gate")?;
                            observe(drops.load(Ordering::Acquire) == 0, "terminal before original response/cleanup released guard")?;
                        } else {
                            release.cancel();
                            tokio::time::timeout(Duration::from_secs(2), terminal.cancelled()).await.map_err(|error| error.to_string())?;
                            if style != "stream_early_ack" {
                                observe(!physically_done.load(Ordering::Acquire), "late response fixture stopped before cleanup gate")?;
                                observe(drops.load(Ordering::Acquire) == 0, "terminal alone released guard while cleanup remained")?;
                            }
                            cleanup.cancel();
                            let source = observing_stream.as_mut().ok_or("original stream absent")?;
                            let event = tokio::time::timeout(Duration::from_secs(2), source.next()).await.map_err(|error| error.to_string())?
                                .ok_or("original stream terminal absent")?;
                            observe(matches!(event, echo_agent::sandbox::SandboxStreamEvent::Complete(_)), "wrong original stream terminal")?;
                        }
                    } else {
                        if style == "pre_cancel" { command_token.cancel(); }
                        let task_proxy = proxy.clone(); let token = explicit_token.clone();
                        let task = tokio::spawn(async move {
                            match style {
                                "limits" => task_proxy.execute_with_limits(command, ResourceLimits::default()).await,
                                "fallback" => {
                                    let mut source = task_proxy.execute_stream(command).await?;
                                    match source.next().await {
                                        Some(echo_agent::sandbox::SandboxStreamEvent::Complete(result)) => Ok(result),
                                        _ => Err(ReactError::Other("buffered sandbox fallback lost result".into())),
                                    }
                                }
                                "cancel" | "pre_cancel" => task_proxy.execute_with_limits_and_cancel(command, ResourceLimits::default(), None).await,
                                "explicit" => task_proxy.execute_with_limits_and_cancel(command, ResourceLimits::default(), Some(token)).await,
                                _ => task_proxy.execute(command).await,
                            }
                        });
                        *host_observing.lock().unwrap_or_else(|error| error.into_inner()) = Some(task);
                        if style == "pre_cancel" {
                            let result = tokio::time::timeout(Duration::from_millis(500), join_observing_task(&host_observing)).await
                                .map_err(|_| "command-only pre-cancellation did not prevent/settle invocation")?
                                .ok_or("original observing task absent")?.map_err(|error| error.to_string())?;
                            observe(result.is_err(), "pre-cancelled command succeeded")?;
                            observe(!entered.is_cancelled(), "pre-cancelled command started reverse execution")?;
                            observe(drops.load(Ordering::Acquire) == 1, "pre-cancelled non-started command guard did not drop once")?;
                            observe(host_remote.started.load(Ordering::Acquire) == 0, "pre-cancelled command submitted a reverse request")?;
                            observe(host_remote.cleanup_calls.load(Ordering::Acquire) == 0, "pre-cancelled command cleaned a shared executor without submission")?;
                            // 原可用性RPC只作writer顺序屏障，不是物理停止ACK或本次清理。
                            observe(proxy.is_available().await, "original availability barrier failed")?;
                            let invocations = recorded_host_invocations(&host_wire_bytes.lock().unwrap_or_else(|error| error.into_inner()))?;
                            observe(invocations.len() == 1 && invocations.iter().all(|(_, operation)| *operation == AgentComponentOperationWire::SandboxIsAvailable), "pre-cancelled command wrote an execution/cleanup request ID")?;
                            observe(host_remote.cleanup_calls.load(Ordering::Acquire) == 0, "availability barrier caused unexpected cleanup")?;
                            return Ok::<(), String>(());
                        }
                        tokio::time::timeout(Duration::from_secs(2), entered.cancelled()).await.map_err(|error| error.to_string())?;
                        observe(dropped_at_entry.load(Ordering::Acquire) == 0, "guard lifetime relied on a nonexistent ToolContext clone")?;
                        if style == "future_drop" {
                            // 仅本次测试的观察waiter，不是远端physical handler。
                            drop_observing_wait(&host_observing);
                            let result = tokio::time::timeout(Duration::from_secs(2), join_observing_task(&host_observing)).await
                                .map_err(|error| error.to_string())?.ok_or("observing task absent")?;
                            observe(result.is_err(), "observing waiter did not stop")?;
                            tokio::time::timeout(Duration::from_secs(2), cancelled.cancelled()).await.map_err(|error| error.to_string())?;
                            observe(drops.load(Ordering::Acquire) == 0, "dropped observing future released original producer guard")?;
                            return Ok::<(), String>(());
                        }
                        if style == "cancel" { command_token.cancel(); }
                        if style == "connection_cancel_all" {
                            observe(state.services().map_err(|error| error.to_string())?.extensions().cancel_all() > 0, "command-token lease left connection shutdown chain")?;
                        }
                        if style == "explicit" {
                            command_token.cancel();
                            observe(tokio::time::timeout(Duration::from_millis(50), cancelled.cancelled()).await.is_err(), "non-selected command token overrode explicit token")?;
                            explicit_token.cancel();
                        }
                        if matches!(style, "cancel" | "explicit" | "connection_cancel_all" | "timeout") {
                            tokio::time::timeout(Duration::from_secs(3), cancelled.cancelled()).await.map_err(|_| "selected source/connection/timeout cancellation did not reach original callback")?;
                        }
                        observe(drops.load(Ordering::Acquire) == 0, "guard released while original producer held")?;
                        release.cancel(); cleanup.cancel();
                        let result = tokio::time::timeout(Duration::from_secs(2), join_observing_task(&host_observing)).await.map_err(|error| error.to_string())?
                            .ok_or("original observing task absent")?.map_err(|error| error.to_string())?;
                        if matches!(style, "cancel" | "explicit" | "connection_cancel_all" | "timeout") {
                            observe(result.is_err(), "cancelled/timed-out invocation succeeded")?;
                        } else { observe(result.is_ok(), "original invocation did not succeed")?; }
                        if matches!(style, "cancel" | "explicit") {
                            observe(host_remote.cleanup_calls.load(Ordering::Acquire) == 1, "submitted cancelled command did not perform its real cleanup")?;
                        }
                    }
                    Ok::<(), String>(())
                };
                tokio::pin!(control);
                match tokio::time::timeout(Duration::from_secs(6), &mut control).await {
                    Ok(result) => result,
                    Err(_) => Err("fixture control deadline exceeded; original owners retained for teardown".to_string()),
                }
            };
            if let Err(error) = control_result { preserve_first_error(&mut first, error); }
        } else if let Err(error) = &setup {
            preserve_first_error(&mut first, error.clone());
        }

        // 无条件finally：先让原mock穿过工作/cleanup门，保留原观察句柄，不结束连接。
        release.cancel(); cleanup.cancel();
        let observer_drain = tokio::time::timeout(Duration::from_secs(3), join_observing_task(&host_observing)).await;
        if observer_drain.is_err() {
            preserve_first_error(&mut first, "observing waiter did not settle within teardown budget");
        }
        let require_entry = entered.is_cancelled()
            || observing_stream.is_some()
            || host_observing.lock().unwrap_or_else(|error| error.into_inner()).is_some()
            || (setup.is_ok() && style != "pre_cancel");
        let producer_drain = tokio::time::timeout(Duration::from_secs(3), host_remote.wait_returned(require_entry)).await;
        if producer_drain.is_err() {
            preserve_first_error(&mut first, "FIXTURE_CLEANUP_UNCONFIRMED: original remote handler/response has not returned");
        }
        let remote_confirmed = producer_drain.is_ok();
        if remote_confirmed {
            // 原handler已返回才可结束仍挂起的纯观察waiter；绝不abort物理producer。
            if host_observing.lock().unwrap_or_else(|error| error.into_inner()).is_some() {
                drop_observing_wait(&host_observing);
                let _ = join_observing_task(&host_observing).await;
            }
            drop(observing_stream.take());
            let remote_errors = host_remote.errors.lock().unwrap_or_else(|error| error.into_inner()).clone();
            for error in remote_errors { preserve_first_error(&mut first, format!("remote response/handler failed: {error}")); }
            let observing_done = host_observing.lock().unwrap_or_else(|error| error.into_inner()).is_none();
            host_cleanup_confirmed.store(observing_done, Ordering::Release);
            if observing_done {
                if entered.is_cancelled() && !physically_done.load(Ordering::Acquire) {
                    preserve_first_error(&mut first, "remote returned without the fixture's physical cleanup fact");
                }
                match tokio::time::timeout(Duration::from_secs(2), async {
                    while drops.load(Ordering::Acquire) == 0 { tokio::task::yield_now().await; }
                }).await {
                    Ok(()) => {
                        if drops.load(Ordering::Acquire) != 1 { preserve_first_error(&mut first, "command guard did not drop exactly once after cleanup"); }
                    }
                    Err(_) => preserve_first_error(&mut first, "physical producer returned but command guard remains retained"),
                }
            }
        }
        *host_first_failure.lock().unwrap_or_else(|error| error.into_inner()) = first.clone();
        // 正常路径只有真实handler/response及观察任务确认后结束SDK。异常DropGuard是失败兜底。
        if host_cleanup_confirmed.load(Ordering::Acquire) { finish.cancel(); }
        match first {
            Some(error) => Err(RpcError::internal_error().data(error)),
            None => Ok(()),
        }
    });

    // 不再用timeout吃掉整个connect/join future。每个控制和drain有独立预算，连接留到统一收尾。
    tokio::pin!(host, sdk);
    let (host_result, sdk_result) = tokio::join!(&mut host, &mut sdk);
    let primary = first_failure
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    let leftovers = observing
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .is_some();
    if leftovers {
        // 仅异常连接退出后的观察waiter，保句柄直到join；不让JoinHandle Drop变detach。
        drop_observing_wait(&observing);
        let _ = join_observing_task(&observing).await;
    }
    if !cleanup_confirmed.load(Ordering::Acquire) {
        return Err(format!(
            "FIXTURE_CLEANUP_UNCONFIRMED; first failure: {}",
            primary.unwrap_or_else(|| "connection stopped before teardown confirmation".into())
        ));
    }
    if let Some(error) = primary {
        return Err(error);
    }
    host_result.map_err(|error| error.to_string())?;
    sdk_result.map_err(|error| error.to_string())?;
    Ok(())
}

#[tokio::test]
async fn sandbox_command_owner_execute() -> Result<(), String> {
    command_owner_case("execute").await
}
#[tokio::test]
async fn sandbox_command_owner_limits() -> Result<(), String> {
    command_owner_case("limits").await
}
#[tokio::test]
async fn sandbox_command_owner_nonstream_fallback() -> Result<(), String> {
    command_owner_case("fallback").await
}
#[tokio::test]
async fn sandbox_command_owner_cancel_only() -> Result<(), String> {
    command_owner_case("cancel").await
}
#[tokio::test]
async fn sandbox_command_owner_pre_cancel() -> Result<(), String> {
    command_owner_case("pre_cancel").await
}
#[tokio::test]
async fn sandbox_command_owner_explicit_token_priority() -> Result<(), String> {
    command_owner_case("explicit").await
}
#[tokio::test]
async fn sandbox_command_owner_stream_late_response() -> Result<(), String> {
    command_owner_case("stream_late_response").await
}
#[tokio::test]
async fn sandbox_command_owner_stream_early_ack() -> Result<(), String> {
    command_owner_case("stream_early_ack").await
}
#[tokio::test]
async fn sandbox_command_owner_stream_observer_drop() -> Result<(), String> {
    command_owner_case("stream_drop").await
}

#[tokio::test]
async fn sandbox_command_owner_future_observer_drop() -> Result<(), String> {
    command_owner_case("future_drop").await
}

#[tokio::test]
async fn sandbox_command_owner_connection_cancel_all() -> Result<(), String> {
    command_owner_case("connection_cancel_all").await
}

#[tokio::test]
async fn sandbox_command_owner_timeout_keeps_physical_guard() -> Result<(), String> {
    command_owner_case("timeout").await
}

// worker_threads是Tokio固定宏参数，不是本项目角色；两线程让独立调用者故障闸不阻塞transport driver。
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sandbox_command_owner_response_registration_failure_retains_original_resources()
-> Result<(), String> {
    let runtime_root =
        std::env::var("ECHO_AGENT_SDK_CONSUMER_ROOT").map_err(|error| error.to_string())?;
    let directory = tempfile::Builder::new()
        .prefix("sandbox-registration-failure-")
        .tempdir_in(runtime_root)
        .map_err(|error| error.to_string())?;
    let config: crate::SdkHostConfig = serde_json::from_value(json!({"schema_version":1,"default_agent":{
        "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
        "agent":{"name":"registration-failure","enable_tools":true}}})).map_err(|error| error.to_string())?;
    let prepared = config.prepare().map_err(|error| error.to_string())?;
    let state = CoreProfileState::install(
        directory.path(),
        crate::config::SdkProfileLimits::default(),
        prepared.framework,
        prepared.llm_client,
    )
    .map_err(|error| error.to_string())?;
    state.bind_services(Arc::new(echo_agent::acp::AcpConnectionServices::new(
        Arc::new(echo_agent::acp::SessionRegistry::new(
            Arc::new(state.session_factory.clone()),
            4,
        )),
        Arc::new(echo_agent::acp::AcpAdapterConfig::default()),
    )));
    let bridge = state.extension_bridge.clone();
    let entered = CancellationToken::new();
    let release = CancellationToken::new();
    let control_stop = CancellationToken::new();
    let physically_done = Arc::new(AtomicBool::new(false));
    let drops = Arc::new(AtomicUsize::new(0));
    let physical = Arc::new(Mutex::new(
        None::<tokio::task::JoinHandle<Result<(), RpcError>>>,
    ));
    let original_owner = Arc::new(Mutex::new(None::<std::sync::Weak<CallbackInvocationOwner>>));
    let hook_closed_driver = Arc::new(AtomicBool::new(false));
    let (host_io, sdk_io) = tokio::io::duplex(65_536);
    let (host_read, host_write) = tokio::io::split(host_io);
    let (sdk_read, sdk_write) = tokio::io::split(sdk_io);
    let sdk = Client
        .builder()
        .on_receive_request(
            {
                let entered = entered.clone();
                let release = release.clone();
                let physically_done = physically_done.clone();
                let physical = physical.clone();
                async move |call: ExtensionInvokeCall,
                            responder: Responder<ExtensionInvokeOutcome>,
                            _connection: ConnectionTo<AcpAgent>| {
                    if !matches!(
                        call.invocation,
                        ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {
                            component: AgentComponentKindWire::SandboxExecutor,
                            call: AgentComponentCallInputWire::SandboxExecute { .. },
                        })
                    ) {
                        return Err(RpcError::internal_error()
                            .data("unexpected registration-failure callback"));
                    }
                    let entered = entered.clone();
                    let release = release.clone();
                    let physically_done = physically_done.clone();
                    // 这是会继续跨transport故障运行的原外部工作；保存原JoinHandle，不交给driver取消来假装停止。
                    let task = tokio::spawn(async move {
                        entered.cancel();
                        release.cancelled().await;
                        physically_done.store(true, Ordering::Release);
                        responder.respond(ExtensionInvokeOutcome::Result {
                            result: ExtensionResult::AgentComponentCall(AgentComponentResultWire {
                                component: AgentComponentKindWire::SandboxExecutor,
                                result: AgentComponentCallResultWire::SandboxExecute {
                                    result: execution_value()
                                        .map_err(|error| RpcError::internal_error().data(error))?,
                                },
                            }),
                        })
                    });
                    *physical.lock().unwrap_or_else(|error| error.into_inner()) = Some(task);
                    Ok(())
                }
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_with(
            ByteStreams::new(sdk_write.compat_write(), sdk_read.compat()),
            async move |_| std::future::pending::<Result<(), RpcError>>().await,
        );
    let sdk_driver = tokio::spawn(sdk);
    let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();
    let host_bridge = bridge.clone();
    let host = AcpAgent.builder().connect_with(
        ByteStreams::new(host_write.compat_write(), host_read.compat()),
        async move |connection| {
            host_bridge.shared.bind_connection(connection);
            ready_tx.send(()).map_err(|_| {
                RpcError::internal_error().data("registration fixture readiness receiver lost")
            })?;
            std::future::pending::<Result<(), RpcError>>().await
        },
    );
    let mut host_driver = Some(tokio::spawn(host));
    let mut control = None;
    let primary = async {
        ready_rx.await.map_err(|error| error.to_string())?;
        let descriptor = serde_json::from_value(json!({"kind":"agent_component","descriptor_version":1,"component":"sandbox_executor","name":"registration-failure","capabilities":{"isolation_level":"process"}})).map_err(|error| error.to_string())?;
        let (extension, _) = state.handles.register_extension(
            super::super::super::handles::ExtensionRegistrationLimits { max_extensions: 2, max_descriptor_bytes: 65_536 },
            ExtensionKind::AgentComponent, "registration-failure", descriptor, None, false,
        ).map_err(|error| error.message)?;
        let proxy = ExtensionAgentComponentProxy::new(bridge.clone(), extension.clone(), "registration-failure-session".to_string()).ok_or("original Sandbox proxy absent")?;
        let (closed_tx, closed_rx) = std::sync::mpsc::sync_channel(1);
        let driver = host_driver.take().ok_or("original Host driver missing")?;
        let failure_connection = bridge.shared.connection().map_err(|error| error.message)?;
        let control_entered = entered.clone(); let stop = control_stop.clone();
        control = Some(tokio::spawn(async move {
            let entered = tokio::select! { () = control_entered.cancelled() => true, () = stop.cancelled() => false };
            if entered {
                // 库自身关闭测试的真实task_actor失败路径；外层driver取消不等于task_rx关闭。
                let scheduled = failure_connection.spawn(async { Err(RpcError::internal_error().data("intentional response-registration task actor fault")) }).is_ok();
                let rejected = tokio::time::timeout(Duration::from_secs(2), async {
                    loop {
                        if failure_connection.spawn(async { Ok(()) }).is_err() { return; }
                        tokio::task::yield_now().await;
                    }
                }).await.is_ok();
                // 实际注册API拒绝才是故障前提；随后join外层driver只回收transport观察者。
                driver.abort(); let _ = driver.await;
                eprintln!("registration fault: real_peer_entered={entered}, task_failure_scheduled={scheduled}, task_registration_rejected={rejected}");
                let _ = closed_tx.send(scheduled && rejected);
            } else {
                driver.abort();
                let _ = driver.await;
                let _ = closed_tx.send(false);
            }
        }));
        let captured = original_owner.clone(); let observed = hook_closed_driver.clone(); let stop = control_stop.clone();
        *bridge.shared.before_response_registration_test_hook.lock().unwrap_or_else(|error| error.into_inner()) = Some(Box::new(move |weak| {
            *captured.lock().unwrap_or_else(|error| error.into_inner()) = Some(weak);
            let result = closed_rx.recv_timeout(Duration::from_secs(2));
            if let Ok(closed) = result { observed.store(closed, Ordering::Release); }
            else {
                stop.cancel();
                let _ = closed_rx.recv_timeout(Duration::from_secs(2));
            }
        }));
        let command = SandboxCommand::shell("printf 中文🙂").with_resource_guards(vec![echo_agent::tools::InvocationResourceGuard::new(DropProof(drops.clone()))]);
        let mut invocation = tokio::spawn(async move { proxy.execute(command).await });
        let result = match tokio::time::timeout(Duration::from_secs(6), &mut invocation).await {
            Ok(result) => result.map_err(|error| error.to_string())?,
            Err(_) => { invocation.abort(); let _ = invocation.await; return Err("registration fault did not finish; observing task joined".to_string()); }
        };
        observe(hook_closed_driver.load(Ordering::Acquire) && entered.is_cancelled() && !physically_done.load(Ordering::Acquire), "registration fault did not reach held original work and closed task actor")?;
        observe(matches!(result, Err(ReactError::Other(_))), "registration failure changed the original transport error mapping")?;
        observe(drops.load(Ordering::Acquire) == 0, "response registration failure released command guard while original work remained")?;
        let weak = original_owner.lock().unwrap_or_else(|error| error.into_inner()).clone().ok_or("original owner weak observation missing")?;
        let record = state.handles.extension_for_cleanup(&extension).map_err(|error| error.message)?;
        let retained = record.callback_owners.lock().unwrap_or_else(|error| error.into_inner()).iter().any(|owner| std::sync::Weak::ptr_eq(&Arc::downgrade(owner), &weak));
        observe(retained, "registration failure did not retain the same original owner as unknown")?;
        let owner = weak.upgrade().ok_or("original owner disappeared")?;
        observe(owner.observed() && !owner.physical_closed.load(Ordering::Acquire) && !owner.finished.is_cancelled(), "driver loss fabricated physical closure")?;
        Ok::<(), String>(())
    }.await;
    // 所有目标错误都先保存，之后无条件放行并真正join原physical task。
    control_stop.cancel();
    release.cancel();
    if let Some(control) = control {
        control.await.map_err(|error| error.to_string())?;
    }
    if let Some(driver) = host_driver {
        driver.abort();
        let _ = driver.await;
    }
    sdk_driver.abort();
    let _ = sdk_driver.await;
    // driver已join，不再可能登记新physical句柄；这个独立原任务仍必须自行越过release并join。
    let original = physical
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .take();
    if let Some(original) = original {
        let _ = original
            .await
            .map_err(|error| format!("FIXTURE_CLEANUP_UNCONFIRMED: {error}"))?;
    }
    primary?;
    observe(
        physically_done.load(Ordering::Acquire),
        "original physical cleanup was not joined",
    )?;
    // 原回复hook已丢失，真实清理在测试中已发生也不能替Host伪造原物理receipt。
    let weak = original_owner
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone()
        .ok_or("original owner observation absent after cleanup")?;
    let held = weak
        .upgrade()
        .ok_or("unknown owner disappeared after transport loss")?;
    observe(
        !held.physical_closed.load(Ordering::Acquire) && drops.load(Ordering::Acquire) == 0,
        "cleanup without an original Host proof released unknown owner",
    )?;
    drop(held);
    // 仅回收已真实结束工作的测试状态，不调用私有physical-close来制造生产结算。
    drop(bridge);
    drop(state);
    observe(
        drops.load(Ordering::Acquire) == 1,
        "fixture state teardown did not release its joined original resource once",
    )?;
    Ok(())
}

#[cfg(feature = "framework-shell")]
async fn public_sandbox_operation(
    connection: &ConnectionTo<AcpAgent>,
    session: &WireHandle,
    name: &str,
    arguments: Vec<WireValue>,
) -> Result<echo_sdk_protocol::methods::FeatureOperationResponse, RpcError> {
    let request = echo_sdk_protocol::methods::FeatureOperationRequest {
        operation: name.to_string(),
        signature_digest: echo_sdk_protocol::facade::family_operation_signature_digest(
            "shell", name,
        ),
        handle: Some(session.clone()),
        arguments,
    };
    let params = serde_json::to_value(request)
        .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
    let value = connection
        .send_request(agent_client_protocol::UntypedMessage::new(
            "_echo_agent/shell/op",
            params,
        )?)
        .block_task()
        .await?;
    serde_json::from_value(value)
        .map_err(|error| RpcError::internal_error().data(error.to_string()))
}

/// 真正协商、创建Session并注册可调用SandboxExecutor，拒绝不能借无效handle冒充。
#[cfg(feature = "framework-shell")]
async fn public_sandbox_projection_case(injected: Option<&'static str>) -> Result<(), String> {
    use echo_sdk_protocol::methods::{
        AgentConfigWire, AgentCreateRequest, ExtensionRegisterRequest, SessionCreateRequest,
    };
    let runtime_root =
        std::env::var("ECHO_AGENT_SDK_CONSUMER_ROOT").map_err(|error| error.to_string())?;
    let directory = tempfile::Builder::new()
        .prefix("sandbox-public-projection-")
        .tempdir_in(runtime_root)
        .map_err(|error| error.to_string())?;
    let config: crate::SdkHostConfig = serde_json::from_value(json!({"schema_version":1,"default_agent":{
        "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
        "agent":{"name":"sandbox-public-projection","enable_tools":true}}})).map_err(|error| error.to_string())?;
    let prepared = config.prepare().map_err(|error| error.to_string())?;
    let state = CoreProfileState::install(
        directory.path(),
        crate::config::SdkProfileLimits::default(),
        prepared.framework,
        prepared.llm_client,
    )
    .map_err(|error| error.to_string())?;
    let adapter = echo_agent::acp::AcpAgentAdapter::new(state.session_factory.clone())
        .with_profile(super::super::super::SdkCoreProfile {
            state: state.clone(),
        });
    let close_owner = adapter.close_owner();
    let calls = Arc::new(AtomicUsize::new(0));
    let returned = CancellationToken::new();
    let value_preserved = Arc::new(AtomicBool::new(false));
    let (host_io, client_io) = tokio::io::duplex(65_536);
    let (host_read, host_write) = tokio::io::split(host_io);
    let (client_read, client_write) = tokio::io::split(client_io);
    let host = agent_client_protocol::ConnectTo::connect_to(
        adapter,
        ByteStreams::new(host_write.compat_write(), host_read.compat()),
    );
    let evidence = Arc::new(Mutex::new(None::<Result<(), String>>));
    let saved_evidence = evidence.clone();
    let client = Client.builder().on_receive_request({
        let calls = calls.clone(); let returned = returned.clone(); let value_preserved = value_preserved.clone();
        async move |call: ExtensionInvokeCall, responder: Responder<ExtensionInvokeOutcome>, connection: ConnectionTo<AcpAgent>| {
            let result = (|| {
                let command = match call.invocation {
                    ExtensionInvocation::AgentComponentCallStream(AgentComponentCallWire { component: AgentComponentKindWire::SandboxExecutor, call: AgentComponentCallInputWire::SandboxExecuteStream { command } }) => command,
                    _ => return Err(RpcError::internal_error().data("unexpected public sandbox callback")),
                };
                calls.fetch_add(1, Ordering::AcqRel);
                let command = command.into_json().map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                value_preserved.store(command == json!({"kind":{"Shell":"printf 中文🙂"},"minimum_isolation":null,"working_dir":null,"env":{"中文":"🙂"},"timeout":{"secs":1,"nanos":0},"stdin":null}), Ordering::Release);
                let stream = call.stream.ok_or_else(|| RpcError::internal_error().data("public sandbox stream absent"))?;
                responder.respond(ExtensionInvokeOutcome::Stream { stream: stream.clone() })?;
                connection.send_notification(ExtensionStreamEvent::Complete {
                    stream, sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string()).map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    value: ExtensionStreamCompleteValue::AgentComponent(AgentComponentStreamCompleteWire::Sandbox(SandboxStreamCompleteWire::Complete { result: execution_value().map_err(|error| RpcError::internal_error().data(error))? })),
                })
            })();
            returned.cancel();
            result
        }
    }, agent_client_protocol::on_receive_request!()).connect_with(ByteStreams::new(client_write.compat_write(), client_read.compat()), {
        let state = state.clone(); let calls = calls.clone(); let returned = returned.clone(); let value_preserved = value_preserved.clone();
        async move |connection| {
            let result = async {
                let mut initialize = agent_client_protocol::schema::v1::InitializeRequest::new(agent_client_protocol::schema::ProtocolVersion::V1);
                let mut meta = agent_client_protocol::schema::v1::Meta::new();
                meta.insert("echo_agent".to_string(), serde_json::to_value(echo_sdk_protocol::capability::EchoAgentClientHello {
                    extension_protocol_version: echo_sdk_protocol::EXTENSION_PROTOCOL_VERSION,
                    contract_digest: state.advertisement.contract_digest.clone(), source_contract_digest: state.advertisement.source_contract_digest.clone(),
                    required_features: Vec::new(), required_capabilities: Vec::new(),
                }).map_err(|error| error.to_string())?);
                initialize.client_capabilities.meta = Some(meta);
                connection.send_request(initialize).block_task().await.map_err(|error| error.to_string())?;
                let agent = connection.send_request(AgentCreateRequest { config: AgentConfigWire::HostDefault, idempotency_id: None }).block_task().await.map_err(|error| error.to_string())?.agent;
                let session = connection.send_request(SessionCreateRequest { agent, working_dir: None, session_id: None, idempotency_id: None }).block_task().await.map_err(|error| error.to_string())?.session;
                let extension = connection.send_request(ExtensionRegisterRequest {
                    kind: ExtensionKind::AgentComponent, implementation_id: "public-sandbox-command".to_string(),
                    descriptor: ExtensionDescriptor::AgentComponent { descriptor_version: 1, component: AgentComponentKindWire::SandboxExecutor, name: "public-sandbox-command".to_string(),
                        capabilities: AgentComponentCapabilitiesWire { supports_streaming: true, isolation_level: Some("process".to_string()), ..Default::default() } }, timeout: None,
                }).block_task().await.map_err(|error| error.to_string())?.extension;
                let mut command = json!({"kind":{"Shell":"printf 中文🙂"},"minimum_isolation":null,"working_dir":null,"env":{"中文":"🙂"},"timeout":{"secs":1,"nanos":0},"stdin":null});
                if let Some(name) = injected {
                    command.as_object_mut().ok_or("command object absent")?.insert(name.to_string(), match name { "resource_guards" => json!([]), "cancel" => json!(null), _ => json!(true) });
                }
                let response = public_sandbox_operation(&connection, &session, "sandbox.extension.run_stream", vec![WireValue::Handle(extension), WireValue::from_json(command).map_err(|error| error.to_string())?]).await;
                // 即使R01错误接纳注入，也先等真实producer/lease并关闭原stream再报目标首错。
                let accepted = match response {
                    Ok(response) => {
                        let stream = match response.value { WireValue::Handle(stream) => stream, _ => return Err("public sandbox returned no original stream".to_string()) };
                        tokio::time::timeout(Duration::from_secs(2), returned.cancelled()).await.map_err(|error| format!("FIXTURE_CLEANUP_UNCONFIRMED: {error}"))?;
                        tokio::time::timeout(Duration::from_secs(2), async {
                            loop {
                                if state.services().map_err(|error| error.to_string())?.extensions().in_flight() == 0 { return Ok::<(), String>(()); }
                                tokio::task::yield_now().await;
                            }
                        }).await.map_err(|error| format!("FIXTURE_CLEANUP_UNCONFIRMED: {error}"))??;
                        public_sandbox_operation(&connection, &session, "sandbox.stream.close", vec![WireValue::Handle(stream)]).await.map_err(|error| format!("FIXTURE_CLEANUP_UNCONFIRMED: {error}"))?;
                        true
                    }
                    Err(error) => {
                        let error = EchoSdkError::from_jsonrpc_data(error.data.as_ref())
                            .map_err(|decode| format!("public facade prerequisite: {decode}; rpc={error:?}"))?;
                        observe(error.code == ExtensionErrorCode::InvalidValue, format!("public sandbox failed before the command boundary: {error:?}"))?;
                        false
                    }
                };
                if injected.is_some() {
                    observe(!accepted, "public sandbox admitted runtime-only/unknown command field")?;
                    observe(calls.load(Ordering::Acquire) == 0, "rejected command invoked reverse sandbox execution")?;
                } else {
                    observe(accepted && calls.load(Ordering::Acquire) == 1 && value_preserved.load(Ordering::Acquire), "valid public sandbox command did not reach original executor losslessly")?;
                }
                Ok(())
            }.await;
            let close = close_owner.close().await.map_err(|error| error.to_string());
            *saved_evidence.lock().unwrap_or_else(|error| error.into_inner()) = Some(result.and(close));
            Ok(())
        }
    });
    let (host_result, client_result) = tokio::join!(host, client);
    let result = evidence
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .take()
        .ok_or("public sandbox evidence absent")?;
    result?;
    host_result.map_err(|error| error.to_string())?;
    client_result.map_err(|error| error.to_string())?;
    Ok(())
}

#[tokio::test]
#[cfg(feature = "framework-shell")]
async fn sandbox_command_owner_public_facade_valid_six_fields() -> Result<(), String> {
    public_sandbox_projection_case(None).await
}
#[tokio::test]
#[cfg(feature = "framework-shell")]
async fn sandbox_command_owner_public_facade_rejects_resource_guards() -> Result<(), String> {
    public_sandbox_projection_case(Some("resource_guards")).await
}
#[tokio::test]
#[cfg(feature = "framework-shell")]
async fn sandbox_command_owner_public_facade_rejects_cancel() -> Result<(), String> {
    public_sandbox_projection_case(Some("cancel")).await
}
#[tokio::test]
#[cfg(feature = "framework-shell")]
async fn sandbox_command_owner_public_facade_rejects_unknown() -> Result<(), String> {
    public_sandbox_projection_case(Some("unknown_command_field")).await
}
