//! Extension bridge end-to-end acceptance (supreme plan 06, todo
//! `implement-trait-proxies-and-streams` / `prove-bridge-reliability-and-docs`).
//!
//! A real official ACP Client plays the language-SDK role against the real
//! `echo-agent-sdk-host` child process compiled with the extension bridge:
//! the client registers host-language implementations and answers the Host's
//! reverse `_echo_agent/extension/invoke` requests, delivering stream chunks
//! through `_echo_agent/extension/stream`. The scenarios cover the full
//! registration → invocation → unregister lifecycle, the fail-closed matrix,
//! typed stream terminals, backpressure, deadlines, late responses,
//! cancellation notices and owner disconnect.

#![cfg(feature = "sdk-extension-bridge")]

use agent_client_protocol::UntypedMessage;
use agent_client_protocol::schema::{ProtocolVersion, v1};
use agent_client_protocol::{
    BoxFuture, ByteStreams, Client, ConnectionTo, Error as RpcError, Responder,
};
use echo_sdk_protocol::capability::{
    EchoAgentCapability, EchoAgentClientHello, ExtensionCapability,
};
use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::handle::{HandleKind, WireHandle};
use echo_sdk_protocol::methods::*;
use echo_sdk_protocol::scalar::{WireDuration, WireNonZeroU64, WirePath, WireU64, WireValue};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpListener;

mod support;

const SENTINEL_SECRET: &str = "sdk-bridge-sentinel-secret";

fn binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_echo-agent-sdk-host"))
}

const SOURCE_CONTRACT_JSON: &str = include_str!("../../contracts/sdk/source-contract.json");

fn source_contract_digest() -> String {
    let document: serde_json::Value =
        serde_json::from_str(SOURCE_CONTRACT_JSON).expect("embedded source contract parses");
    document
        .get("aggregate_digest")
        .and_then(serde_json::Value::as_str)
        .expect("aggregate_digest present")
        .to_string()
}

fn facade_digest(operation: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../contracts/sdk/facade-operation-catalog.json");
    let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    catalog
        .get("routes")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .find(|route| route.get("operation").and_then(serde_json::Value::as_str) == Some(operation))
        .and_then(|route| route.get("signature_digests"))
        .and_then(serde_json::Value::as_array)
        .and_then(|digests| digests.first())
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("missing facade digest for {operation}").into())
}

fn client_hello() -> EchoAgentClientHello {
    EchoAgentClientHello {
        extension_protocol_version: echo_sdk_protocol::EXTENSION_PROTOCOL_VERSION,
        contract_digest: echo_sdk_protocol::schema::extension_contract_digest(),
        source_contract_digest: source_contract_digest(),
        required_features: Vec::new(),
        required_capabilities: vec![ExtensionCapability::ExtensionBridge],
    }
}

fn initialize_request(hello: Option<EchoAgentClientHello>) -> v1::InitializeRequest {
    let mut request = v1::InitializeRequest::new(ProtocolVersion::V1);
    if let Some(hello) = hello {
        let value = serde_json::to_value(&hello).expect("hello JSON");
        let mut meta = v1::Meta::new();
        meta.insert("echo_agent".to_string(), value);
        request.client_capabilities.meta = Some(meta);
    }
    request
}

fn write_config(directory: &Path, endpoint: &str, state_root: &Path) -> PathBuf {
    write_config_with_features(directory, endpoint, state_root, false, false, false)
}

fn write_config_with_features(
    directory: &Path,
    endpoint: &str,
    state_root: &Path,
    enable_memory: bool,
    enable_human_in_loop: bool,
    enable_subagent: bool,
) -> PathBuf {
    let document = serde_json::json!({
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "fixture-model",
                "base_url": endpoint,
                "api_protocol": "chat_completions",
                "auth_token": SENTINEL_SECRET
            },
            "agent": {
                "name": "fixture-agent",
                "system_prompt": "Answer the user directly.",
                "max_iterations": 6,
                "enable_tools": true,
                "enable_memory": enable_memory,
                "enable_human_in_loop": enable_human_in_loop,
                "enable_subagent": enable_subagent,
                "register_agent_dispatch_tool": enable_subagent,
                "memory_path": state_root.join("memory.json").display().to_string()
            }
        },
        "sdk_profile": {
            "state_root": state_root.display().to_string(),
            "limits": {}
        }
    });
    let path = directory.join("host.json");
    std::fs::write(
        &path,
        serde_json::to_vec_pretty(&document).expect("config JSON"),
    )
    .expect("write config");
    path
}

fn set_profile_limit(
    path: &Path,
    name: &str,
    value: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut document: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
    let limits = document
        .pointer_mut("/sdk_profile/limits")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| std::io::Error::other("sdk_profile.limits must be an object"))?;
    limits.insert(name.to_string(), serde_json::json!(value));
    std::fs::write(path, serde_json::to_vec_pretty(&document)?)?;
    Ok(())
}

/// Scripted loopback model server: each chat completion receives the next
/// scripted SSE stream (tool-call turn, then final turn).
async fn start_scripted_model(
    scripts: Vec<Vec<serde_json::Value>>,
) -> Result<String, Box<dyn std::error::Error>> {
    start_scripted_model_with_delay(scripts, Duration::ZERO).await
}

async fn start_scripted_model_with_delay(
    scripts: Vec<Vec<serde_json::Value>>,
    response_delay: Duration,
) -> Result<String, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?.to_string();
    let seen = Arc::new(AtomicUsize::new(0));
    let total = scripts.len();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let scripts = scripts.clone();
            let seen = seen.clone();
            tokio::spawn(async move {
                if support::read_http_request(&mut socket).await.is_err() {
                    return;
                }
                if !response_delay.is_zero() {
                    tokio::time::sleep(response_delay).await;
                }
                let index = seen.fetch_add(1, Ordering::AcqRel);
                let Some(events) = scripts.get(index.min(total.saturating_sub(1))) else {
                    return;
                };
                let mut body = String::new();
                for event in events {
                    body.push_str(&format!(
                        "data: {}\n\n",
                        serde_json::to_string(event).unwrap_or_default()
                    ));
                }
                body.push_str("data: [DONE]\n\n");
                let headers = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = socket.write_all(headers.as_bytes()).await;
                let _ = socket.write_all(body.as_bytes()).await;
                let _ = socket.shutdown().await;
            });
        }
    });
    Ok(address)
}

fn tool_call_script(tool: &str, arguments: &str) -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({
            "id": "chatcmpl-fixture",
            "choices": [{
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "tool_calls": [{
                        "index": 0,
                        "id": "call-1",
                        "type": "function",
                        "function": {"name": tool, "arguments": arguments}
                    }]
                },
                "finish_reason": null
            }]
        }),
        serde_json::json!({
            "id": "chatcmpl-fixture",
            "choices": [{
                "index": 0,
                "delta": {},
                "finish_reason": "tool_calls"
            }]
        }),
    ]
}

fn final_script(text: &str) -> Vec<serde_json::Value> {
    vec![serde_json::json!({
        "id": "chatcmpl-fixture",
        "choices": [{
            "index": 0,
            "delta": {"role": "assistant", "content": text},
            "finish_reason": "stop"
        }]
    })]
}

// ── Host process plumbing (same shape as the core profile harness) ─────────

type SharedVec<T> = Arc<Mutex<Vec<T>>>;

/// Test-side observations only: an absent timestamp is not proof that a pipe
/// remains open. EOF is recorded only after an actual nonempty read request.
struct HostPipeObservations {
    started: std::time::Instant,
    stdin_dropped: AtomicU64,
    stdout_eof: AtomicU64,
    stderr_eof: AtomicU64,
    stdout_read_error: AtomicU64,
    stderr_read_error: AtomicU64,
}

impl HostPipeObservations {
    fn new() -> Self {
        Self {
            started: std::time::Instant::now(),
            stdin_dropped: AtomicU64::new(0),
            stdout_eof: AtomicU64::new(0),
            stderr_eof: AtomicU64::new(0),
            stdout_read_error: AtomicU64::new(0),
            stderr_read_error: AtomicU64::new(0),
        }
    }

    fn mark(&self, observation: &AtomicU64) {
        // Zero means unobserved; retain the first observation without locking.
        let nanos = u64::try_from(self.started.elapsed().as_nanos())
            .unwrap_or(u64::MAX)
            .saturating_add(1);
        let _ = observation.compare_exchange(0, nanos, Ordering::AcqRel, Ordering::Acquire);
    }

    fn snapshot(&self) -> [(&'static str, Option<u64>); 5] {
        [
            ("stdin_dropped", &self.stdin_dropped),
            ("stdout_eof", &self.stdout_eof),
            ("stderr_eof", &self.stderr_eof),
            ("stdout_read_error", &self.stdout_read_error),
            ("stderr_read_error", &self.stderr_read_error),
        ]
        .map(|(name, observation)| (name, observation.load(Ordering::Acquire).checked_sub(1)))
    }
}

// Declared after the ChildStdin field below: Rust drops that destination before
// this marker, so this is pipe-owner destruction, not merely transport shutdown.
struct RecordHostStdinDrop(Arc<HostPipeObservations>);

impl Drop for RecordHostStdinDrop {
    fn drop(&mut self) {
        self.0.mark(&self.0.stdin_dropped);
    }
}

struct HostProcess {
    child: tokio::process::Child,
    stderr: SharedVec<u8>,
    stdout: SharedVec<u8>,
    stdin: SharedVec<u8>,
    pipes: Arc<HostPipeObservations>,
}

async fn spawn_host(config: &Path) -> Result<HostProcess, Box<dyn std::error::Error>> {
    let mut child = tokio::process::Command::new(binary())
        .arg("--config")
        .arg(config)
        .env(
            "RUST_LOG",
            std::env::var("RUST_LOG").unwrap_or_else(|_| "echo_sdk_host=debug".to_string()),
        )
        // Fixture model servers are loopback; never route them through a
        // developer proxy (reqwest follows the system proxy otherwise).
        .env("NO_PROXY", "127.0.0.1,localhost")
        .env("no_proxy", "127.0.0.1,localhost")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stderr_handle = child.stderr.take().expect("host stderr piped");
    let stderr: SharedVec<u8> = Arc::new(Mutex::new(Vec::new()));
    let sink = stderr.clone();
    let pipes = Arc::new(HostPipeObservations::new());
    let stderr_pipes = pipes.clone();
    tokio::spawn(async move {
        let mut stderr_handle = stderr_handle;
        let mut buffer = [0_u8; 4096];
        loop {
            match stderr_handle.read(&mut buffer).await {
                Ok(0) => {
                    stderr_pipes.mark(&stderr_pipes.stderr_eof);
                    break;
                }
                Err(_) => {
                    stderr_pipes.mark(&stderr_pipes.stderr_read_error);
                    break;
                }
                Ok(read) => sink
                    .lock()
                    .expect("stderr sink")
                    .extend_from_slice(&buffer[..read]),
            }
        }
    });
    Ok(HostProcess {
        child,
        stderr,
        stdout: Arc::new(Mutex::new(Vec::new())),
        stdin: Arc::new(Mutex::new(Vec::new())),
        pipes,
    })
}

/// Record exact stdout bytes while preserving the same poll and transport
/// lifecycle. Failed reverse-call decoding can therefore be reproduced from
/// the real frame, instead of being guessed from a timeout.
struct RecordedHostStdout {
    source: tokio::process::ChildStdout,
    bytes: SharedVec<u8>,
    pipes: Arc<HostPipeObservations>,
}

impl tokio::io::AsyncRead for RecordedHostStdout {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buffer: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let previous = buffer.filled().len();
        let requested = buffer.remaining();
        let result =
            tokio::io::AsyncRead::poll_read(std::pin::Pin::new(&mut self.source), cx, buffer);
        if matches!(result, std::task::Poll::Ready(Err(_))) {
            self.pipes.mark(&self.pipes.stdout_read_error);
        }
        if matches!(result, std::task::Poll::Ready(Ok(()))) {
            if requested > 0 && buffer.filled().len() == previous {
                self.pipes.mark(&self.pipes.stdout_eof);
            }
            let mut bytes = self.bytes.lock().unwrap_or_else(|error| error.into_inner());
            if bytes.len() < 2_097_152 {
                bytes.extend_from_slice(buffer.filled().get(previous..).unwrap_or_default());
            }
        }
        result
    }
}

struct RecordedHostStdin {
    destination: tokio::process::ChildStdin,
    bytes: SharedVec<u8>,
    // Must remain after destination: its Drop records that owner's release.
    _drop_record: RecordHostStdinDrop,
}
impl tokio::io::AsyncWrite for RecordedHostStdin {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buffer: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let result = tokio::io::AsyncWrite::poll_write(
            std::pin::Pin::new(&mut self.destination),
            cx,
            buffer,
        );
        if let std::task::Poll::Ready(Ok(written)) = result {
            let mut bytes = self.bytes.lock().unwrap_or_else(|error| error.into_inner());
            if bytes.len() < 2_097_152 {
                bytes.extend_from_slice(buffer.get(..written).unwrap_or_default());
            }
        }
        result
    }
    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        tokio::io::AsyncWrite::poll_flush(std::pin::Pin::new(&mut self.destination), cx)
    }
    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        tokio::io::AsyncWrite::poll_shutdown(std::pin::Pin::new(&mut self.destination), cx)
    }
}

fn host_transport(
    host: &mut HostProcess,
) -> ByteStreams<
    tokio_util::compat::Compat<RecordedHostStdin>,
    tokio_util::compat::Compat<RecordedHostStdout>,
> {
    let stdin = host.child.stdin.take().expect("host stdin piped");
    let stdout = host.child.stdout.take().expect("host stdout piped");
    ByteStreams::new(
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(RecordedHostStdin {
            destination: stdin,
            bytes: host.stdin.clone(),
            _drop_record: RecordHostStdinDrop(host.pipes.clone()),
        }),
        tokio_util::compat::TokioAsyncReadCompatExt::compat(RecordedHostStdout {
            source: stdout,
            bytes: host.stdout.clone(),
            pipes: host.pipes.clone(),
        }),
    )
}

fn reverse_frames(host: &HostProcess) -> Vec<serde_json::Value> {
    let bytes = host
        .stdout
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    bytes
        .split(|byte| *byte == b'\n')
        .filter_map(|frame| serde_json::from_slice::<serde_json::Value>(frame).ok())
        .filter(|frame| {
            frame.get("method").and_then(serde_json::Value::as_str)
                == Some("_echo_agent/extension/invoke")
        })
        .collect()
}

fn stderr_text(host: &HostProcess) -> String {
    String::from_utf8_lossy(
        host.stderr
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .as_slice(),
    )
    .to_string()
}

type SharedRawStore = Arc<Mutex<std::collections::BTreeMap<(Vec<String>, String), WireValue>>>;

/// The fake SDK dispatcher state shared by client handlers.
#[derive(Default)]
struct SdkDispatch {
    operations: SharedVec<String>,
    compressor_extensions: SharedVec<String>,
    component_operations: SharedVec<String>,
    cancel_notices: SharedVec<String>,
    /// Hold responders that must never answer (timeout/cancel scenarios).
    silent: Arc<Mutex<Vec<HeldFixtureInvocation>>>,
    stream_producers: Arc<Mutex<Vec<HeldFixtureProducer>>>,
    /// Operations that must hang instead of answering.
    hang: Arc<Mutex<Vec<&'static str>>>,
    hang_once: Arc<std::sync::atomic::AtomicBool>,
    malformed_stream: Arc<std::sync::atomic::AtomicBool>,
    out_of_order_stream: Arc<std::sync::atomic::AtomicBool>,
    duplicate_terminal: Arc<std::sync::atomic::AtomicBool>,
    omit_stream_terminal: Arc<std::sync::atomic::AtomicBool>,
    oversized_stream: Arc<std::sync::atomic::AtomicBool>,
    flood_stream: Arc<std::sync::atomic::AtomicBool>,
    missing_finish_reason: Arc<std::sync::atomic::AtomicBool>,
    hang_sandbox_cancel: Arc<std::sync::atomic::AtomicBool>,
    delayed_sandbox_cleanup: Arc<std::sync::atomic::AtomicBool>,
    sandbox_cleanups_completed: Arc<AtomicUsize>,
    reentrant_mutation: Arc<Mutex<Option<(WireHandle, WireHandle)>>>,
    reentrant_conflicts: Arc<AtomicUsize>,
    tokenizer_counts: Arc<AtomicUsize>,
    checkpoints: Arc<CheckpointDispatch>,
    managed: Arc<ManagedStoreDispatch>,
    events: SharedVec<echo_sdk_protocol::event::WireEventEnvelope>,
    factory_instances: Arc<Mutex<std::collections::BTreeMap<String, String>>>,
    factory_created: SharedVec<String>,
    factory_closed: SharedVec<String>,
    task_store: Arc<TaskStoreDispatch>,
    interrupted_callbacks: SharedVec<CallbackToolInterruptedInput>,
    raw_store: SharedRawStore,
}

/// A parked original request, never a synthesized retry or replacement call.
struct HeldFixtureInvocation {
    call: ExtensionInvokeCall,
    responder: Responder<ExtensionInvokeOutcome>,
}

impl HeldFixtureInvocation {
    fn respond(self, outcome: ExtensionInvokeOutcome) -> Result<(), RpcError> {
        self.responder.respond(outcome)
    }
}

fn validate_factory_instance(
    call: &ExtensionInvokeCall,
    instances: &Mutex<std::collections::BTreeMap<String, String>>,
) -> Result<(), Box<EchoSdkError>> {
    if let Some(origin) = &call.factory_origin
        && instances
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(&call.extension.id)
            != Some(&origin.id)
    {
        return Err(Box::new(EchoSdkError::new(
            ExtensionErrorCode::ExtensionRejected,
            "factory instance does not belong to this registered parent",
            Retryability::Never,
        )));
    }
    Ok(())
}

fn complete_fixture_agent_close(
    call: &ExtensionInvokeCall,
    instances: &Mutex<std::collections::BTreeMap<String, String>>,
    closed: &Mutex<Vec<String>>,
) -> Result<ExtensionInvokeOutcome, Box<EchoSdkError>> {
    validate_factory_instance(call, instances)?;
    if call.factory_origin.is_some() {
        instances
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&call.extension.id);
        closed
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .push(call.extension.id.clone());
    }
    Ok(ExtensionInvokeOutcome::Result {
        result: ExtensionResult::AgentClose(ExtensionUnit),
    })
}

async fn wait_fixture_notice(
    dispatch: &SdkDispatch,
    operation: ExtensionOperation,
    reason: &str,
) -> Result<String, RpcError> {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            // Snapshot only fixture routing identities. Never hold a fixture
            // lock while waiting for transport or answering the original call.
            let mut candidates: Vec<String> = dispatch
                .silent
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .iter()
                .filter(|held| held.call.invocation.operation() == operation)
                .map(|held| held.call.invocation_id.clone())
                .collect();
            candidates.extend(
                dispatch
                    .stream_producers
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .iter()
                    .filter(|producer| producer.call.invocation.operation() == operation)
                    .map(|producer| producer.call.invocation_id.clone()),
            );
            let noticed = {
                let notices = dispatch
                    .cancel_notices
                    .lock()
                    .unwrap_or_else(|error| error.into_inner());
                candidates.into_iter().find(|id| {
                    notices
                        .iter()
                        .any(|notice| notice == &format!("{id}:{reason}"))
                })
            };
            if let Some(id) = noticed {
                return Ok(id);
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .map_err(|_| RpcError::internal_error().data("original fixture cancel notice missing"))?
}

fn take_fixture_invocation(
    dispatch: &SdkDispatch,
    id: &str,
    operation: ExtensionOperation,
) -> Result<HeldFixtureInvocation, RpcError> {
    let mut pending = dispatch
        .silent
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let index = pending
        .iter()
        .position(|held| {
            held.call.invocation_id == id && held.call.invocation.operation() == operation
        })
        .ok_or_else(|| RpcError::internal_error().data("original parked invocation missing"))?;
    Ok(pending.remove(index))
}

struct HeldFixtureProducer {
    call: ExtensionInvokeCall,
    chunks_sent: tokio_util::sync::CancellationToken,
    release: tokio_util::sync::CancellationToken,
    finished: tokio_util::sync::CancellationToken,
    result: Arc<Mutex<Option<Result<(), RpcError>>>>,
    task: tokio::task::JoinHandle<()>,
}

fn start_held_fixture_producer(
    call: ExtensionInvokeCall,
    responder: Responder<ExtensionInvokeOutcome>,
    connection: ConnectionTo<agent_client_protocol::Agent>,
    producers: &Mutex<Vec<HeldFixtureProducer>>,
) -> Result<(), RpcError> {
    let stream = call
        .stream
        .clone()
        .ok_or_else(|| RpcError::internal_error().data("original producer stream missing"))?;
    let llm = matches!(call.invocation, ExtensionInvocation::LlmChatStream(_));
    let chunks_sent = tokio_util::sync::CancellationToken::new();
    let release = tokio_util::sync::CancellationToken::new();
    let ready = chunks_sent.clone();
    let wait_release = release.clone();
    let finished = tokio_util::sync::CancellationToken::new();
    let result = Arc::new(Mutex::new(None));
    let done = finished.clone();
    let task_result = result.clone();
    let task = tokio::spawn(async move {
        let outcome = async move {
            responder.respond(ExtensionInvokeOutcome::Stream {
                stream: stream.clone(),
            })?;
            if llm {
                for (sequence, text) in [(1, "streamed "), (2, "answer")] {
                    connection.send_notification(ExtensionStreamEvent::Chunk {
                        stream: stream.clone(),
                        sequence: nonzero(sequence),
                        value: ExtensionStreamChunkValue::Llm(chat_stream_chunk_wire(text)),
                    })?;
                }
            } else {
                connection.send_notification(ExtensionStreamEvent::Chunk {
                    stream: stream.clone(),
                    sequence: nonzero(1),
                    value: ExtensionStreamChunkValue::Agent(AgentStreamChunkWire::Token {
                        text: "SDK custom event".to_string(),
                    }),
                })?;
            }
            // This signal follows the actual original ACK and chunk sends. The
            // original producer remains here until explicitly released; only this
            // task can send its final event, and the scenario joins this same task.
            ready.cancel();
            wait_release.cancelled().await;
            connection.send_notification(ExtensionStreamEvent::Complete {
                stream,
                sequence: nonzero(if llm { 3 } else { 2 }),
                value: if llm {
                    ExtensionStreamCompleteValue::Llm(chat_stream_complete_wire("stop"))
                } else {
                    ExtensionStreamCompleteValue::Agent(AgentStreamTerminalWire::Cancelled)
                },
            })
        }
        .await;
        *task_result
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = Some(outcome);
        done.cancel();
    });
    producers
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .push(HeldFixtureProducer {
            call,
            chunks_sent,
            release,
            finished,
            result,
            task,
        });
    Ok(())
}

fn original_producer_result(result: &Mutex<Option<Result<(), RpcError>>>) -> Result<(), RpcError> {
    result
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone()
        .ok_or_else(|| {
            RpcError::internal_error().data("original producer completed without its result")
        })?
}

async fn wait_fixture_producer_ready(
    dispatch: &SdkDispatch,
    operation: ExtensionOperation,
    id: Option<&str>,
) -> Result<(), RpcError> {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let producer = dispatch.stream_producers.lock().unwrap_or_else(|error| error.into_inner())
                .iter().find(|producer| producer.call.invocation.operation() == operation && id.is_none_or(|id| producer.call.invocation_id == id))
                .map(|producer| (producer.chunks_sent.clone(), producer.finished.clone(), producer.result.clone()));
            if let Some((ready, finished, result)) = producer {
                tokio::select! {
                    biased;
                    _ = finished.cancelled() => {
                        original_producer_result(&result)?;
                        return Err(RpcError::internal_error().data("original producer ended before explicit release"));
                    }
                    _ = ready.cancelled() => return Ok(()),
                }
            }
            tokio::task::yield_now().await;
        }
    }).await.map_err(|_| RpcError::internal_error().data("original producer ACK/chunks did not complete"))?
}

async fn finish_fixture_stream(dispatch: &SdkDispatch, id: &str) -> Result<(), RpcError> {
    let operation = dispatch
        .stream_producers
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .iter()
        .find(|producer| producer.call.invocation_id == id)
        .map(|producer| producer.call.invocation.operation())
        .ok_or_else(|| RpcError::internal_error().data("original stream producer missing"))?;
    // Readiness failure leaves the exact release address and original handle
    // in the fixture owner. It never consumes/detaches a failed or live job.
    wait_fixture_producer_ready(dispatch, operation, Some(id)).await?;
    let mut producer = {
        let mut pending = dispatch
            .stream_producers
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let index = pending
            .iter()
            .position(|producer| producer.call.invocation_id == id)
            .ok_or_else(|| RpcError::internal_error().data("original stream producer missing"))?;
        pending.remove(index)
    };
    if producer.task.is_finished() {
        let result = original_producer_result(&producer.result);
        dispatch
            .stream_producers
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .push(producer);
        result?;
        return Err(
            RpcError::internal_error().data("original producer ended before explicit release")
        );
    }
    producer.release.cancel();
    match tokio::time::timeout(Duration::from_secs(10), &mut producer.task).await {
        Ok(joined) => {
            joined.map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            original_producer_result(&producer.result)
        }
        Err(_) => {
            // Preserve the same released-but-not-yet-joined job for cleanup
            // or diagnosis. A timeout cannot manufacture physical closure.
            dispatch
                .stream_producers
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .push(producer);
            Err(RpcError::internal_error().data("original producer join timed out"))
        }
    }
}

#[derive(Default)]
struct TaskStoreDispatch {
    graphs: Mutex<std::collections::BTreeMap<String, RevisionedTaskGraphWire>>,
    execution_commits: AtomicUsize,
    reject_execution: std::sync::atomic::AtomicBool,
    relation_commits: AtomicUsize,
}

impl TaskStoreDispatch {
    fn executions(
        graph: &RevisionedTaskGraphWire,
    ) -> std::collections::BTreeMap<String, TaskExecutionWire> {
        graph
            .snapshot
            .tasks
            .iter()
            .map(|task| (task.spec.id.clone(), task.execution.clone()))
            .collect()
    }
    fn handle(
        &self,
        call: &AgentComponentCallInputWire,
    ) -> Result<AgentComponentCallResultWire, Box<EchoSdkError>> {
        let mut graphs = self
            .graphs
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        match call {
            AgentComponentCallInputWire::RevisionedTaskLoad { scope_id } => {
                Ok(AgentComponentCallResultWire::RevisionedTaskLoad {
                    graph: graphs.get(scope_id).cloned(),
                })
            }
            AgentComponentCallInputWire::RevisionedTaskCompareAndCommit { scope_id, commit } => {
                let previous = graphs.get(scope_id);
                let actual = previous.map(|graph| graph.snapshot.revision.clone());
                if actual != commit.expected_revision
                    || commit.expected_executions.as_ref().is_some_and(|expected| {
                        previous.is_none_or(|graph| &Self::executions(graph) != expected)
                    })
                {
                    return Err(Self::conflict(
                        commit.expected_revision.as_ref(),
                        actual.as_ref(),
                    ));
                }
                if actual
                    .as_ref()
                    .and_then(WireU64::to_u64)
                    .unwrap_or(0)
                    .checked_add(1)
                    != commit.next.snapshot.revision.to_u64()
                {
                    return Err(ManagedStoreDispatch::error("corrupt_state"));
                }
                graphs.insert(scope_id.clone(), commit.next.clone());
                self.relation_commits.fetch_add(1, Ordering::AcqRel);
                Ok(
                    AgentComponentCallResultWire::RevisionedTaskCompareAndCommit {
                        graph: commit.next.clone(),
                    },
                )
            }
            AgentComponentCallInputWire::RevisionedTaskCompareAndCommitExecution {
                scope_id,
                commit,
            } => {
                if self.reject_execution.load(Ordering::Acquire) {
                    return Err(Box::new(EchoSdkError::new(
                        ExtensionErrorCode::ExtensionFailed,
                        "task execution persistence failed",
                        Retryability::Never,
                    )));
                }
                let previous = graphs
                    .get(scope_id)
                    .ok_or_else(|| Self::conflict(Some(&commit.expected_revision), None))?;
                if previous.snapshot.revision != commit.expected_revision
                    || Self::executions(previous) != commit.expected_executions
                {
                    return Err(Self::conflict(
                        Some(&commit.expected_revision),
                        Some(&previous.snapshot.revision),
                    ));
                }
                if previous.snapshot.revision != commit.next.snapshot.revision
                    || previous.context != commit.next.context
                    || previous
                        .snapshot
                        .tasks
                        .iter()
                        .map(|task| &task.spec)
                        .ne(commit.next.snapshot.tasks.iter().map(|task| &task.spec))
                {
                    return Err(ManagedStoreDispatch::error("corrupt_state"));
                }
                graphs.insert(scope_id.clone(), commit.next.clone());
                self.execution_commits.fetch_add(1, Ordering::AcqRel);
                Ok(
                    AgentComponentCallResultWire::RevisionedTaskCompareAndCommitExecution {
                        graph: commit.next.clone(),
                    },
                )
            }
            _ => Err(ManagedStoreDispatch::error("unsupported")),
        }
    }
    fn conflict(expected: Option<&WireU64>, current: Option<&WireU64>) -> Box<EchoSdkError> {
        let mut error = EchoSdkError::new(
            ExtensionErrorCode::ExtensionConflict,
            "atomic task backend conflict",
            Retryability::Never,
        );
        error.details = Some(echo_sdk_protocol::error::ErrorDetails {
            fields: Some(vec![
                echo_sdk_protocol::error::DetailField {
                    key: "task_store_error_class".to_string(),
                    value: "conflict".to_string(),
                },
                echo_sdk_protocol::error::DetailField {
                    key: "expected_revision".to_string(),
                    value: expected
                        .map(|value| value.as_str())
                        .unwrap_or("absent")
                        .to_string(),
                },
                echo_sdk_protocol::error::DetailField {
                    key: "current_revision".to_string(),
                    value: current
                        .map(|value| value.as_str())
                        .unwrap_or("absent")
                        .to_string(),
                },
            ]),
            ..Default::default()
        });
        Box::new(error)
    }
}

/// Test-only remote backend. The receipt map models the backend's atomic
/// idempotent commit, while the real framework performs persistence recovery.
#[derive(Default)]
struct ManagedStoreDispatch {
    enabled: std::sync::atomic::AtomicBool,
    delay_before_commit: std::sync::atomic::AtomicBool,
    lose_first_apply_response: std::sync::atomic::AtomicBool,
    conversation: Mutex<Option<ConversationProjectionAuthorityWire>>,
    runtime: Mutex<std::collections::BTreeMap<(String, String), ManagedRuntimeStateSnapshotWire>>,
    scopes: Mutex<std::collections::BTreeMap<String, RuntimeScopeAuthorityWire>>,
    receipts: Mutex<std::collections::BTreeMap<String, TranscriptProjectionApplyReceiptWire>>,
    expired: AtomicUsize,
    mutations: AtomicUsize,
    projected_items: AtomicUsize,
    duplicate_applies: AtomicUsize,
}

impl ManagedStoreDispatch {
    fn deadline(call: &AgentComponentCallInputWire) -> Option<&PersistenceCallContextWire> {
        match call {
            AgentComponentCallInputWire::ConversationEnsureProjectionEpochWithContext {
                context,
                ..
            }
            | AgentComponentCallInputWire::ConversationGetProjectionAuthorityWithContext {
                context,
                ..
            }
            | AgentComponentCallInputWire::ConversationApplyTranscriptProjectionWithContext {
                context,
                ..
            }
            | AgentComponentCallInputWire::ConversationDeleteManagedWithContext {
                context, ..
            }
            | AgentComponentCallInputWire::RuntimeLoadManagedStateWithContext { context, .. }
            | AgentComponentCallInputWire::RuntimeLoadScopeAuthorityWithContext {
                context, ..
            }
            | AgentComponentCallInputWire::RuntimeCompareAndSaveCheckpointWithContext {
                context,
                ..
            }
            | AgentComponentCallInputWire::RuntimeAcknowledgeTranscriptProjectionWithContext {
                context,
                ..
            }
            | AgentComponentCallInputWire::RuntimeRetireGenerationWithContext { context, .. }
            | AgentComponentCallInputWire::RuntimeBeginScopeRetirementWithContext {
                context, ..
            }
            | AgentComponentCallInputWire::RuntimeContinueScopeRetirementWithContext {
                context,
                ..
            } => Some(context),
            _ => None,
        }
    }

    fn error(class: &str) -> Box<EchoSdkError> {
        let mut error = EchoSdkError::new(
            ExtensionErrorCode::ExtensionFailed,
            format!("managed fixture {class}"),
            Retryability::Never,
        );
        error.details = Some(echo_sdk_protocol::error::ErrorDetails {
            fields: Some(vec![echo_sdk_protocol::error::DetailField {
                key: "persistence_error_class".to_string(),
                value: class.to_string(),
            }]),
            ..Default::default()
        });
        Box::new(error)
    }

    fn now_ms() -> Result<i64, RpcError> {
        let elapsed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
        i64::try_from(elapsed.as_millis())
            .map_err(|error| RpcError::internal_error().data(error.to_string()))
    }

    fn handle(
        &self,
        call: &AgentComponentCallInputWire,
    ) -> Result<AgentComponentCallResultWire, Box<EchoSdkError>> {
        let context = Self::deadline(call).ok_or_else(|| Self::error("unsupported"))?;
        let now = Self::now_ms().map_err(|_| Self::error("invalid_configuration"))?;
        if context
            .absolute_deadline_unix_ms
            .to_i64()
            .is_none_or(|deadline| deadline <= now)
        {
            self.expired.fetch_add(1, Ordering::AcqRel);
            return Err(Self::error("deadline_exceeded"));
        }
        match call {
            AgentComponentCallInputWire::ConversationEnsureProjectionEpochWithContext {
                request,
                ..
            } => {
                let mut slot = self
                    .conversation
                    .lock()
                    .unwrap_or_else(|error| error.into_inner());
                let existed = slot.is_some();
                let authority = slot
                    .get_or_insert_with(|| ConversationProjectionAuthorityWire {
                        conversation_id: request.conversation.conversation_id.clone(),
                        epoch: WireU64::from_u64(1),
                        revision: WireU64::from_u64(1),
                        lifecycle: ConversationProjectionLifecycleWire::Live,
                        delete_receipt_retention_floor_epoch: WireU64::from_u64(0),
                    })
                    .clone();
                if !existed {
                    self.mutations.fetch_add(1, Ordering::AcqRel);
                }
                Ok(
                    AgentComponentCallResultWire::ConversationEnsureProjectionEpochWithContext {
                        receipt: ConversationProjectionEpochReceiptWire {
                            authority,
                            status: if existed {
                                ConversationProjectionEpochStatusWire::Existing
                            } else {
                                ConversationProjectionEpochStatusWire::Created
                            },
                        },
                    },
                )
            }
            AgentComponentCallInputWire::ConversationGetProjectionAuthorityWithContext {
                ..
            } => Ok(
                AgentComponentCallResultWire::ConversationGetProjectionAuthorityWithContext {
                    authority: self
                        .conversation
                        .lock()
                        .unwrap_or_else(|error| error.into_inner())
                        .clone(),
                },
            ),
            AgentComponentCallInputWire::RuntimeLoadManagedStateWithContext {
                scope_id,
                runtime_state_id,
                ..
            } => Ok(
                AgentComponentCallResultWire::RuntimeLoadManagedStateWithContext {
                    snapshot: self
                        .runtime
                        .lock()
                        .unwrap_or_else(|error| error.into_inner())
                        .get(&(scope_id.clone(), runtime_state_id.clone()))
                        .cloned(),
                },
            ),
            AgentComponentCallInputWire::RuntimeLoadScopeAuthorityWithContext {
                scope_id, ..
            } => Ok(
                AgentComponentCallResultWire::RuntimeLoadScopeAuthorityWithContext {
                    authority: self
                        .scopes
                        .lock()
                        .unwrap_or_else(|error| error.into_inner())
                        .get(scope_id)
                        .cloned(),
                },
            ),
            AgentComponentCallInputWire::RuntimeCompareAndSaveCheckpointWithContext {
                request,
                ..
            } => self.save(
                (&request.scope_id, &request.runtime_state_id),
                &request.expected_scope_revision,
                &request.expected_state_version,
                request.conversation_epoch.clone(),
                request.checkpoint.clone(),
                false,
            ),
            AgentComponentCallInputWire::RuntimeAcknowledgeTranscriptProjectionWithContext {
                request,
                ..
            } => self.save(
                (&request.scope_id, &request.runtime_state_id),
                &request.expected_scope_revision,
                &request.expected_state_version,
                Some(request.conversation_epoch.clone()),
                request.checkpoint.clone(),
                true,
            ),
            AgentComponentCallInputWire::ConversationApplyTranscriptProjectionWithContext {
                batch,
                ..
            } => {
                let mut receipts = self
                    .receipts
                    .lock()
                    .unwrap_or_else(|error| error.into_inner());
                if let Some(receipt) = receipts.get(&batch.operation_id) {
                    if receipt.payload_digest != batch.payload_digest {
                        return Err(Self::error("corrupt_state"));
                    }
                    self.duplicate_applies.fetch_add(1, Ordering::AcqRel);
                    let mut receipt = receipt.clone();
                    receipt.status = TranscriptProjectionApplyStatusWire::AlreadyApplied;
                    return Ok(AgentComponentCallResultWire::ConversationApplyTranscriptProjectionWithContext { receipt });
                }
                let mut slot = self
                    .conversation
                    .lock()
                    .unwrap_or_else(|error| error.into_inner());
                let authority = slot.as_mut().ok_or_else(|| Self::error("corrupt_state"))?;
                let revision = authority
                    .revision
                    .to_u64()
                    .and_then(|revision| revision.checked_add(1))
                    .ok_or_else(|| Self::error("corrupt_state"))?;
                authority.revision = WireU64::from_u64(revision);
                let receipt = TranscriptProjectionApplyReceiptWire {
                    operation_id: batch.operation_id.clone(),
                    payload_digest: batch.payload_digest.clone(),
                    authority: authority.clone(),
                    status: TranscriptProjectionApplyStatusWire::Applied,
                };
                receipts.insert(batch.operation_id.clone(), receipt.clone());
                self.projected_items
                    .fetch_add(batch.items.len(), Ordering::AcqRel);
                self.mutations.fetch_add(1, Ordering::AcqRel);
                Ok(AgentComponentCallResultWire::ConversationApplyTranscriptProjectionWithContext { receipt })
            }
            _ => Err(Self::error("unsupported")),
        }
    }

    fn save(
        &self,
        identity: (&str, &str),
        expected_scope_revision: &WireU64,
        expected_state_version: &RuntimeStateExpectedVersionWire,
        epoch: Option<WireU64>,
        checkpoint: AgentCheckpointWire,
        ack: bool,
    ) -> Result<AgentComponentCallResultWire, Box<EchoSdkError>> {
        let (scope_id, runtime_state_id) = identity;
        let mut states = self
            .runtime
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let key = (scope_id.to_string(), runtime_state_id.to_string());
        let previous = states.get(&key);
        let expected = expected_scope_revision
            .to_u64()
            .ok_or_else(|| Self::error("corrupt_state"))?;
        let mut scopes = self
            .scopes
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let actual = scopes
            .get(scope_id)
            .and_then(|scope| scope.revision.to_u64())
            .unwrap_or(0);
        let state_matches = match (
            previous.map(|snapshot| &snapshot.version),
            expected_state_version,
        ) {
            (None, RuntimeStateExpectedVersionWire::Absent) => true,
            (
                Some(RuntimeStateVersionWire::Managed { revision: actual }),
                RuntimeStateExpectedVersionWire::Managed { revision: expected },
            ) => actual == expected,
            _ => false,
        };
        if !state_matches {
            return Err(Self::error("corrupt_state"));
        }
        if actual != expected {
            return Err(Self::error("corrupt_state"));
        }
        let revision = actual
            .checked_add(1)
            .ok_or_else(|| Self::error("corrupt_state"))?;
        let state_revision = match previous.map(|snapshot| &snapshot.version) {
            Some(RuntimeStateVersionWire::Managed { revision }) => revision
                .to_u64()
                .and_then(|revision| revision.checked_add(1))
                .ok_or_else(|| Self::error("corrupt_state"))?,
            _ => 1,
        };
        let scope = RuntimeScopeAuthorityWire {
            scope_id: scope_id.to_string(),
            conversation_epoch: epoch,
            revision: WireU64::from_u64(revision),
            lifecycle: RuntimeScopeLifecycleWire::Active,
        };
        let version = RuntimeStateVersionWire::Managed {
            revision: WireU64::from_u64(state_revision),
        };
        scopes.insert(scope_id.to_string(), scope.clone());
        states.insert(
            key,
            ManagedRuntimeStateSnapshotWire {
                scope: scope.clone(),
                runtime_state_id: runtime_state_id.to_string(),
                version: version.clone(),
                checkpoint: Some(checkpoint),
            },
        );
        self.mutations.fetch_add(1, Ordering::AcqRel);
        let receipt = RuntimeCheckpointCasReceiptWire {
            scope,
            runtime_state_id: runtime_state_id.to_string(),
            version,
            status: RuntimeCheckpointCasStatusWire::Applied,
        };
        Ok(if ack {
            AgentComponentCallResultWire::RuntimeAcknowledgeTranscriptProjectionWithContext {
                receipt,
            }
        } else {
            AgentComponentCallResultWire::RuntimeCompareAndSaveCheckpointWithContext { receipt }
        })
    }
}

#[derive(Default)]
struct CheckpointDispatch {
    pending: Mutex<Option<WireValue>>,
    claimed: Mutex<Option<(String, String, WireValue, bool)>>,
    renewals: AtomicUsize,
}

type CheckpointDispatchResult<T> = Result<T, Box<EchoSdkError>>;

impl CheckpointDispatch {
    fn invalid(message: impl Into<String>) -> Box<EchoSdkError> {
        Box::new(EchoSdkError::new(
            ExtensionErrorCode::InvalidValue,
            message.into(),
            Retryability::Never,
        ))
    }

    fn json(value: &WireValue) -> CheckpointDispatchResult<serde_json::Value> {
        value
            .clone()
            .into_json()
            .map_err(|error| Self::invalid(error.to_string()))
    }

    fn checkpoint_id(value: &WireValue) -> CheckpointDispatchResult<String> {
        Self::json(value)?
            .get("id")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string)
            .ok_or_else(|| Self::invalid("checkpoint has no id"))
    }

    fn with_attempt(
        value: &WireValue,
        attempt_id: Option<&str>,
    ) -> CheckpointDispatchResult<WireValue> {
        let mut json = Self::json(value)?;
        let object = json
            .as_object_mut()
            .ok_or_else(|| Self::invalid("checkpoint is not an object"))?;
        object.insert(
            "resume_attempt_id".to_string(),
            attempt_id
                .map(|value| serde_json::Value::String(value.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        WireValue::from_json(json).map_err(|error| Self::invalid(error.to_string()))
    }

    fn save(&self, checkpoint: WireValue) -> CheckpointDispatchResult<()> {
        let id = Self::checkpoint_id(&checkpoint)?;
        let claimed = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        Self::check_mutation(&claimed, Some(&id))?;
        *self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = Some(checkpoint);
        Ok(())
    }

    // The original fixture owns one checkpoint claim. Protect that exact
    // attempt under the same lock used by mutations; advertise only this real
    // implementation, not a neutral Unit response that permits deletion.
    fn check_mutation(
        claimed: &Option<(String, String, WireValue, bool)>,
        id: Option<&str>,
    ) -> CheckpointDispatchResult<()> {
        if claimed
            .as_ref()
            .is_some_and(|(stored, _, _, protected)| *protected && id.is_none_or(|id| stored == id))
        {
            return Err(Self::invalid("checkpoint claim is protected"));
        }
        Ok(())
    }

    fn protect(&self, id: &str, attempt_id: &str) -> CheckpointDispatchResult<()> {
        let mut claimed = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let Some((stored_id, stored_attempt, _, protected)) = claimed.as_mut() else {
            return Err(Self::invalid("checkpoint claim owner mismatch"));
        };
        if stored_id != id || stored_attempt != attempt_id {
            return Err(Self::invalid("checkpoint claim owner mismatch"));
        }
        *protected = true;
        Ok(())
    }

    fn delete(&self, id: &str) -> CheckpointDispatchResult<()> {
        let mut claimed = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        Self::check_mutation(&claimed, Some(id))?;
        let mut pending = self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if pending
            .as_ref()
            .is_some_and(|value| Self::checkpoint_id(value).is_ok_and(|stored| stored == id))
        {
            pending.take();
        }
        if claimed
            .as_ref()
            .is_some_and(|(stored, _, _, _)| stored == id)
        {
            claimed.take();
        }
        Ok(())
    }

    fn clear(&self) -> CheckpointDispatchResult<()> {
        let mut claimed = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        Self::check_mutation(&claimed, None)?;
        *self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = None;
        *claimed = None;
        Ok(())
    }

    fn load(&self, id: &str) -> CheckpointDispatchResult<Option<WireValue>> {
        if let Some(checkpoint) = self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .as_ref()
            && Self::checkpoint_id(checkpoint)? == id
        {
            return Ok(Some(checkpoint.clone()));
        }
        let claimed = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        Ok(claimed
            .as_ref()
            .filter(|(checkpoint_id, _, _, _)| checkpoint_id == id)
            .map(|(_, _, checkpoint, _)| checkpoint.clone()))
    }

    fn claim(&self, id: &str) -> CheckpointDispatchResult<Option<WireValue>> {
        let mut original_claim = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if original_claim.is_some() {
            return Err(Self::invalid("checkpoint is already claimed"));
        }
        let mut pending = self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let Some(checkpoint) = pending.as_ref() else {
            return Ok(None);
        };
        if Self::checkpoint_id(checkpoint)? != id {
            return Ok(None);
        }
        let checkpoint = pending
            .take()
            .ok_or_else(|| Self::invalid("checkpoint claim disappeared"))?;
        drop(pending);
        let attempt_id = format!("sdk-resume-attempt-{}", uuid::Uuid::new_v4());
        let claimed = Self::with_attempt(&checkpoint, Some(&attempt_id))?;
        *original_claim = Some((id.to_string(), attempt_id, claimed.clone(), false));
        Ok(Some(claimed))
    }

    fn settle_claim(
        &self,
        id: &str,
        attempt_id: &str,
        requeue: bool,
    ) -> CheckpointDispatchResult<()> {
        let mut claimed = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let matches = claimed
            .as_ref()
            .is_some_and(|(stored_id, stored_attempt, _, _)| {
                stored_id == id && stored_attempt == attempt_id
            });
        if !matches {
            return Err(Self::invalid("checkpoint claim owner mismatch"));
        }
        let checkpoint = claimed
            .as_ref()
            .map(|(_, _, checkpoint, _)| checkpoint)
            .ok_or_else(|| Self::invalid("checkpoint claim disappeared"))?;
        // Validate the replacement before consuming the original protected
        // claim. Requeue/ack belong to the matching attempt, not CRUD bypass.
        let requeued = if requeue {
            Some(Self::with_attempt(checkpoint, None)?)
        } else {
            None
        };
        if let Some(checkpoint) = requeued {
            *self
                .pending
                .lock()
                .unwrap_or_else(|error| error.into_inner()) = Some(checkpoint);
        }
        claimed.take();
        Ok(())
    }

    fn renew(&self, id: &str, attempt_id: &str) -> CheckpointDispatchResult<()> {
        let claimed = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if !claimed
            .as_ref()
            .is_some_and(|(stored_id, stored_attempt, _, _)| {
                stored_id == id && stored_attempt == attempt_id
            })
        {
            return Err(Self::invalid("checkpoint claim owner mismatch"));
        }
        self.renewals.fetch_add(1, Ordering::AcqRel);
        Ok(())
    }

    fn save_if_generation(
        &self,
        checkpoint: WireValue,
        expected_generation: WireU64,
    ) -> CheckpointDispatchResult<bool> {
        let Some(expected_generation) = expected_generation.to_u64() else {
            return Err(Self::invalid("expected_generation is out of range"));
        };
        let id = Self::checkpoint_id(&checkpoint)?;
        let claimed = self
            .claimed
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        Self::check_mutation(&claimed, Some(&id))?;
        let mut pending = self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let Some(current) = pending.as_ref() else {
            return Ok(false);
        };
        let generation = Self::json(current)?
            .get("generation")
            .and_then(serde_json::Value::as_u64);
        if generation != Some(expected_generation) {
            return Ok(false);
        }
        *pending = Some(checkpoint);
        Ok(true)
    }
}

#[test]
fn checkpoint_dispatch_rejects_stale_attempt_and_protected_mutations()
-> Result<(), Box<dyn std::error::Error>> {
    let dispatch = CheckpointDispatch::default();
    let checkpoint = WireValue::from_json(serde_json::json!({"id":"checkpoint-a","generation":1}))?;
    dispatch
        .save(checkpoint.clone())
        .map_err(|error| std::io::Error::other(error.message))?;
    let attempt = |value: &WireValue| -> Result<String, Box<dyn std::error::Error>> {
        let json = value.clone().into_json()?;
        Ok(json
            .get("resume_attempt_id")
            .and_then(serde_json::Value::as_str)
            .ok_or("missing original claim attempt")?
            .to_string())
    };
    let first = dispatch
        .claim("checkpoint-a")
        .map_err(|error| std::io::Error::other(error.message))?
        .ok_or("missing first claim")?;
    let first_attempt = attempt(&first)?;
    dispatch
        .settle_claim("checkpoint-a", &first_attempt, true)
        .map_err(|error| std::io::Error::other(error.message))?;
    let second = dispatch
        .claim("checkpoint-a")
        .map_err(|error| std::io::Error::other(error.message))?
        .ok_or("missing fresh claim")?;
    let second_attempt = attempt(&second)?;
    assert_ne!(first_attempt, second_attempt);
    assert!(dispatch.protect("checkpoint-a", &first_attempt).is_err());
    assert!(dispatch.renew("checkpoint-a", &first_attempt).is_err());
    assert!(
        dispatch
            .settle_claim("checkpoint-a", &first_attempt, false)
            .is_err()
    );
    assert!(
        dispatch
            .settle_claim("checkpoint-a", &first_attempt, true)
            .is_err()
    );
    assert_eq!(
        dispatch
            .load("checkpoint-a")
            .map_err(|error| std::io::Error::other(error.message))?,
        Some(second.clone())
    );
    dispatch
        .protect("checkpoint-a", &second_attempt)
        .map_err(|error| std::io::Error::other(error.message))?;
    assert!(dispatch.save(checkpoint.clone()).is_err());
    assert!(
        dispatch
            .save_if_generation(checkpoint, WireU64::from_u64(1))
            .is_err()
    );
    assert!(dispatch.delete("checkpoint-a").is_err());
    assert!(dispatch.clear().is_err());
    assert_eq!(
        dispatch
            .load("checkpoint-a")
            .map_err(|error| std::io::Error::other(error.message))?,
        Some(second)
    );
    dispatch
        .renew("checkpoint-a", &second_attempt)
        .map_err(|error| std::io::Error::other(error.message))?;
    dispatch
        .settle_claim("checkpoint-a", &second_attempt, false)
        .map_err(|error| std::io::Error::other(error.message))?;
    assert!(
        dispatch
            .load("checkpoint-a")
            .map_err(|error| std::io::Error::other(error.message))?
            .is_none()
    );
    Ok(())
}

fn tool_descriptor(name: &str) -> ExtensionDescriptor {
    ExtensionDescriptor::Tool {
        descriptor_version: 1,
        name: name.to_string(),
        description: "host-language fixture tool".to_string(),
        parameters: WireValue::from_json(serde_json::json!({
            "type": "object",
            "properties": {"query": {"type": "string"}}
        }))
        .expect("schema wire value"),
        schema_revision: WireU64::from_u64(1),
        required_input_modalities: Vec::new(),
        required_permissions: Vec::new(),
        risk_level: echo_sdk_protocol::methods::ToolRiskLevelWire::ReadOnly,
        supports_streaming: false,
        exempt_from_batch_timeout: false,
        allows_parallel_batch_execution: true,
        manages_own_timeout: false,
    }
}

fn llm_descriptor(model: &str) -> ExtensionDescriptor {
    llm_descriptor_with_streaming(model, true)
}

fn llm_descriptor_with_streaming(model: &str, supports_streaming: bool) -> ExtensionDescriptor {
    ExtensionDescriptor::LlmClient {
        descriptor_version: 1,
        model_name: model.to_string(),
        supports_streaming,
        capabilities: echo_sdk_protocol::methods::LlmCapabilitiesWire::default(),
    }
}

fn critic_descriptor(name: &str) -> ExtensionDescriptor {
    ExtensionDescriptor::Critic {
        descriptor_version: 1,
        name: name.to_string(),
    }
}

fn channel_handler_descriptor(handler_id: &str) -> ExtensionDescriptor {
    ExtensionDescriptor::ChannelMessageHandler(ChannelMessageHandlerDescriptorWire {
        descriptor_version: 1,
        handler_id: handler_id.to_string(),
    })
}

#[cfg(feature = "framework-channels")]
fn channel_plugin_descriptor(channel_id: &str, handler_id: &str) -> ExtensionDescriptor {
    ExtensionDescriptor::ChannelPlugin(ChannelPluginDescriptorWire {
        descriptor_version: 1,
        channel_id: channel_id.to_string(),
        label: "SDK channel".to_string(),
        capabilities: ChannelCapabilitiesWire {
            chat_types: vec![ChannelChatTypeWire::Direct],
            supports_media: false,
            supports_threads: false,
        },
        handler_id: handler_id.to_string(),
    })
}

#[cfg(feature = "framework-channels")]
fn channel_outbound_fixture(text: &str) -> ChannelOutboundMessageWire {
    ChannelOutboundMessageWire {
        channel_id: "sdk-channel".to_string(),
        to: "chat".to_string(),
        chat_type: ChannelChatTypeWire::Direct,
        text: text.to_string(),
        reply_to: None,
        attachments: Vec::new(),
    }
}

/// Connect the fake SDK client with reverse-invocation handlers and run the
/// scenario to completion.
async fn drive_sdk<T, F>(
    host: &mut HostProcess,
    dispatch: Arc<SdkDispatch>,
    scenario: F,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: Send + 'static,
    F: FnOnce(
            ConnectionTo<agent_client_protocol::Agent>,
        ) -> BoxFuture<'static, agent_client_protocol::Result<T>>
        + Send
        + 'static,
{
    let transport = host_transport(host);
    let operations = dispatch.operations.clone();
    let compressor_extensions = dispatch.compressor_extensions.clone();
    let component_operations = dispatch.component_operations.clone();
    let cancel_notices = dispatch.cancel_notices.clone();
    let silent = dispatch.silent.clone();
    let stream_producers = dispatch.stream_producers.clone();
    let hang = dispatch.hang.clone();
    let hang_once = dispatch.hang_once.clone();
    let malformed_stream = dispatch.malformed_stream.clone();
    let out_of_order_stream = dispatch.out_of_order_stream.clone();
    let duplicate_terminal = dispatch.duplicate_terminal.clone();
    let omit_stream_terminal = dispatch.omit_stream_terminal.clone();
    let oversized_stream = dispatch.oversized_stream.clone();
    let flood_stream = dispatch.flood_stream.clone();
    let missing_finish_reason = dispatch.missing_finish_reason.clone();
    let hang_sandbox_cancel = dispatch.hang_sandbox_cancel.clone();
    let delayed_sandbox_cleanup = dispatch.delayed_sandbox_cleanup.clone();
    let sandbox_cleanups_completed = dispatch.sandbox_cleanups_completed.clone();
    let reentrant_mutation = dispatch.reentrant_mutation.clone();
    let reentrant_conflicts = dispatch.reentrant_conflicts.clone();
    let tokenizer_counts = dispatch.tokenizer_counts.clone();
    let checkpoints = dispatch.checkpoints.clone();
    let managed = dispatch.managed.clone();
    let event_log = dispatch.events.clone();
    let factory_instances = dispatch.factory_instances.clone();
    let factory_created = dispatch.factory_created.clone();
    let factory_closed = dispatch.factory_closed.clone();
    let task_store = dispatch.task_store.clone();
    let interrupted_callbacks = dispatch.interrupted_callbacks.clone();
    let raw_store = dispatch.raw_store.clone();
    let connect = Client
        .builder()
        .on_receive_request(
            {
                let operations = operations.clone();
                let compressor_extensions = compressor_extensions.clone();
                let component_operations = component_operations.clone();
                let silent = silent.clone();
                let stream_producers = stream_producers.clone();
                let hang = hang.clone();
                let hang_once = hang_once.clone();
                let malformed_stream = malformed_stream.clone();
                let out_of_order_stream = out_of_order_stream.clone();
                let duplicate_terminal = duplicate_terminal.clone();
                let omit_stream_terminal = omit_stream_terminal.clone();
                let oversized_stream = oversized_stream.clone();
                let flood_stream = flood_stream.clone();
                let missing_finish_reason = missing_finish_reason.clone();
                let hang_sandbox_cancel = hang_sandbox_cancel.clone();
                let delayed_sandbox_cleanup = delayed_sandbox_cleanup.clone();
                let sandbox_cleanups_completed = sandbox_cleanups_completed.clone();
                let reentrant_mutation = reentrant_mutation.clone();
                let reentrant_conflicts = reentrant_conflicts.clone();
                let tokenizer_counts = tokenizer_counts.clone();
                let checkpoints = checkpoints.clone();
                let managed = managed.clone();
                let factory_instances = factory_instances.clone();
                let factory_created = factory_created.clone();
                let factory_closed = factory_closed.clone();
                let task_store = task_store.clone();
                let interrupted_callbacks = interrupted_callbacks.clone();
                let raw_store = raw_store.clone();
                move |call: ExtensionInvokeCall,
                      responder: Responder<ExtensionInvokeOutcome>,
                      connection: ConnectionTo<agent_client_protocol::Agent>| {
                    let operations = operations.clone();
                    let compressor_extensions = compressor_extensions.clone();
                    let component_operations = component_operations.clone();
                    let silent = silent.clone();
                    let stream_producers = stream_producers.clone();
                    let hang = hang.clone();
                    let hang_once = hang_once.clone();
                    let malformed_stream = malformed_stream.clone();
                    let out_of_order_stream = out_of_order_stream.clone();
                    let duplicate_terminal = duplicate_terminal.clone();
                    let omit_stream_terminal = omit_stream_terminal.clone();
                    let oversized_stream = oversized_stream.clone();
                    let flood_stream = flood_stream.clone();
                    let missing_finish_reason = missing_finish_reason.clone();
                    let hang_sandbox_cancel = hang_sandbox_cancel.clone();
                    let delayed_sandbox_cleanup = delayed_sandbox_cleanup.clone();
                    let sandbox_cleanups_completed = sandbox_cleanups_completed.clone();
                    let reentrant_mutation = reentrant_mutation.clone();
                    let reentrant_conflicts = reentrant_conflicts.clone();
                    let tokenizer_counts = tokenizer_counts.clone();
                    let checkpoints = checkpoints.clone();
                    let managed = managed.clone();
                    let factory_instances = factory_instances.clone();
                    let factory_created = factory_created.clone();
                    let factory_closed = factory_closed.clone();
                    let task_store = task_store.clone();
                    let interrupted_callbacks = interrupted_callbacks.clone();
                    let raw_store = raw_store.clone();
                    async move {
                        let operation = call.invocation.operation();
                        let original_call = call.clone();
                        operations
                            .lock()
                            .expect("operations lock")
                            .push(operation.as_str().to_string());
                        let hold_original = {
                            let mut held_operations = hang.lock().unwrap_or_else(|error| error.into_inner());
                            let found = held_operations.iter().position(|candidate| *candidate == operation.as_str());
                            if let Some(index) = found {
                                if hang_once.load(Ordering::Acquire) { held_operations.remove(index); }
                                true
                            } else { false }
                        };
                        if hold_original {
                            silent.lock().expect("silent lock").push(HeldFixtureInvocation { call: original_call.clone(), responder });
                            return Ok(());
                        }
                        if let Err(error) = validate_factory_instance(&call, &factory_instances) {
                            return responder.respond(ExtensionInvokeOutcome::Error { error: *error });
                        }
                        match call.invocation {
                            ExtensionInvocation::ToolExecute(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::ToolExecute(tool_result_wire(
                                        "3 documents matched",
                                    )),
                                })
                            }
                            ExtensionInvocation::ToolExecuteStream(_) => {
                                let stream = call.stream.ok_or_else(|| RpcError::internal_error().data("original retry Tool stream missing"))?;
                                responder.respond(ExtensionInvokeOutcome::Stream { stream: stream.clone() })?;
                                // The framework retry owns a distinct actual
                                // callback producer; it does not replace the
                                // first callback's physical completion.
                                tokio::spawn(async move {
                                    connection.send_notification(ExtensionStreamEvent::Complete {
                                        stream, sequence: nonzero(1),
                                        value: ExtensionStreamCompleteValue::Tool(tool_result_wire("original retry completed")),
                                    })
                                });
                                Ok(())
                            }
                            ExtensionInvocation::ToolValidateParameters(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::ToolValidateParameters(None),
                                })
                            }
                            ExtensionInvocation::StorePut(input) => {
                                raw_store.lock().unwrap_or_else(|error| error.into_inner()).insert((input.namespace, input.key), input.value);
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::StorePut(ExtensionUnit),
                                })
                            }
                            ExtensionInvocation::StoreGet(input) => {
                                let value = raw_store.lock().unwrap_or_else(|error| error.into_inner()).get(&(input.namespace.clone(), input.key.clone())).cloned();
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::StoreGet(value.map(|value| StoreItemWire { namespace: input.namespace, key: input.key, value, created_at: WireU64::from_u64(1), updated_at: WireU64::from_u64(1), score: None, importance: 5.0, last_accessed: None, expires_at: None })),
                                })
                            }
                            ExtensionInvocation::StoreSearch(_)
                            | ExtensionInvocation::StoreSearchWith(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: if operation == ExtensionOperation::StoreSearch {
                                        ExtensionResult::StoreSearch(Vec::new())
                                    } else {
                                        ExtensionResult::StoreSearchWith(Vec::new())
                                    },
                                })
                            }
                            ExtensionInvocation::StoreDelete(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::StoreDelete(true),
                                })
                            }
                            ExtensionInvocation::StoreListNamespaces(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::StoreListNamespaces(vec![vec![
                                        "sdk".to_string(),
                                    ]]),
                                })
                            }
                            ExtensionInvocation::StoreList(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::StoreList(Vec::new()),
                                })
                            }
                            ExtensionInvocation::StorePruneExpired(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::StorePruneExpired(WireU64::from_u64(
                                        0,
                                    )),
                                })
                            }
                            ExtensionInvocation::StoreDedupByContent(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::StoreDedupByContent(
                                        WireU64::from_u64(0),
                                    ),
                                })
                            }
                            ExtensionInvocation::LlmChat(_) => {
                                let mut response = chat_response_wire("fixture chat answer");
                                if missing_finish_reason.load(Ordering::Acquire) {
                                    response.finish_reason = None;
                                }
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::LlmChat(response),
                                })
                            }
                            ExtensionInvocation::CompressorCompress(input) => {
                                compressor_extensions
                                    .lock()
                                    .unwrap_or_else(|error| error.into_inner())
                                    .push(call.extension.id.clone());
                                assert_eq!(
                                    input.focus_instructions.as_deref(),
                                    Some("preserve decisions")
                                );
                                let tokenizer_operation =
                                    "echo_core::tokenizer::Tokenizer::count_tokens";
                                let signature_digest = facade_digest(tokenizer_operation)
                                    .map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?;
                                let counts = tokenizer_counts.clone();
                                let tokenizer = input.tokenizer.resource.clone();
                                let tokenizer_owner = input.tokenizer.owner_session_id.clone();
                                tokio::spawn(async move {
                                    let Ok(message) = UntypedMessage::new(
                                        "_echo_agent/facade/invoke",
                                        serde_json::json!({
                                            "operation": tokenizer_operation,
                                            "signature_digest": signature_digest,
                                            "handle": tokenizer,
                                            "arguments": [
                                                {"kind": "string", "value": tokenizer_owner},
                                                {"kind": "string", "value": "hello"}
                                            ],
                                        }),
                                    ) else {
                                        return;
                                    };
                                    let Ok(counted) = connection.send_request(message).block_task().await else {
                                        return;
                                    };
                                    let Ok(counted) = serde_json::from_value::<FeatureOperationResponse>(counted) else {
                                        return;
                                    };
                                    if counted.value.into_json().ok() == Some(serde_json::json!(1)) {
                                        counts.fetch_add(1, Ordering::AcqRel);
                                    }
                                });
                                tokio::time::sleep(Duration::from_millis(25)).await;
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::CompressorCompress(
                                        CompressionOutputWire {
                                            messages: input.messages,
                                            evicted: Vec::new(),
                                            checkpoint: None,
                                        },
                                    ),
                                })
                            }
                            ExtensionInvocation::AgentComponentCall(input) => {
                                let operation = input.call.operation();
                                component_operations
                                    .lock()
                                    .unwrap_or_else(|error| error.into_inner())
                                    .push(format!("{:?}:{:?}", input.component, operation));
                                if operation
                                    == AgentComponentOperationWire::SandboxExecuteWithLimitsAndCancel
                                    && hang_sandbox_cancel.load(Ordering::Acquire)
                                {
                                    silent
                                        .lock()
                                        .unwrap_or_else(|error| error.into_inner())
                                        .push(HeldFixtureInvocation { call: original_call.clone(), responder });
                                    return Ok(());
                                }
                                if operation == AgentComponentOperationWire::SandboxCleanup {
                                    if delayed_sandbox_cleanup.load(Ordering::Acquire) {
                                        tokio::time::sleep(Duration::from_millis(250)).await;
                                    }
                                    // This is the acknowledged resource cleanup side effect,
                                    // deliberately later than callback admission.
                                    sandbox_cleanups_completed.fetch_add(1, Ordering::AcqRel);
                                }
                                let reentrant = reentrant_mutation
                                    .lock()
                                    .unwrap_or_else(|error| error.into_inner())
                                    .clone();
                                if operation == AgentComponentOperationWire::AuditLog
                                    && let Some((agent, session)) = reentrant
                                {
                                    let mutation =
                                        "echo_agent::agent::react::ReactAgent::set_plan_mode";
                                    let signature_digest = facade_digest(mutation)
                                        .map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?;
                                    let conflicts = reentrant_conflicts.clone();
                                    tokio::spawn(async move {
                                        let Ok(message) = UntypedMessage::new(
                                            "_echo_agent/facade/invoke",
                                            serde_json::json!({
                                                "operation": mutation,
                                                "signature_digest": signature_digest,
                                                "handle": agent,
                                                "arguments": [
                                                    {"kind": "handle", "value": session},
                                                    {"kind": "bool", "value": true}
                                                ],
                                            }),
                                        ) else {
                                            return;
                                        };
                                        if let Err(error) = connection
                                            .send_request(message)
                                            .block_task()
                                            .await
                                            && EchoSdkError::from_jsonrpc_data(error.data.as_ref())
                                                .is_ok_and(|typed| typed.code == ExtensionErrorCode::ExtensionConflict)
                                        {
                                            conflicts.fetch_add(1, Ordering::AcqRel);
                                        }
                                    });
                                    tokio::time::sleep(Duration::from_millis(25)).await;
                                }
                                if managed.enabled.load(Ordering::Acquire) && ManagedStoreDispatch::deadline(&input.call).is_some() {
                                    if managed.delay_before_commit.swap(false, Ordering::AcqRel)
                                        && let Some(context) = ManagedStoreDispatch::deadline(&input.call) {
                                        let now = ManagedStoreDispatch::now_ms()?;
                                        let deadline = context.absolute_deadline_unix_ms.to_i64().ok_or_else(RpcError::invalid_params)?;
                                        let delay = deadline.checked_sub(now).and_then(|value| u64::try_from(value).ok()).unwrap_or(0).saturating_add(30);
                                        tokio::time::sleep(Duration::from_millis(delay)).await;
                                    }
                                    let result = managed.handle(&input.call);
                                    if result.is_ok() && matches!(input.call, AgentComponentCallInputWire::ConversationApplyTranscriptProjectionWithContext { .. })
                                        && managed.lose_first_apply_response.swap(false, Ordering::AcqRel) {
                                        silent.lock().unwrap_or_else(|error| error.into_inner()).push(HeldFixtureInvocation { call: original_call.clone(), responder });
                                        return Ok(());
                                    }
                                    return responder.respond(match result {
                                        Ok(result) => ExtensionInvokeOutcome::Result { result: ExtensionResult::AgentComponentCall(AgentComponentResultWire { component: input.component, result }) },
                                        Err(error) => ExtensionInvokeOutcome::Error { error: *error },
                                    });
                                }
                                let result = match input.call {
                                    AgentComponentCallInputWire::ConversationCreate { conversation } => {
                                        AgentComponentCallResultWire::ConversationCreate { conversation }
                                    }
                                    AgentComponentCallInputWire::ConversationGet { .. } => {
                                        AgentComponentCallResultWire::ConversationGet { conversation: None }
                                    }
                                    AgentComponentCallInputWire::ConversationList { .. } => {
                                        AgentComponentCallResultWire::ConversationList { conversations: Vec::new() }
                                    }
                                    AgentComponentCallInputWire::ConversationUpdate { .. } => AgentComponentCallResultWire::ConversationUpdate,
                                    AgentComponentCallInputWire::ConversationDelete { .. } => AgentComponentCallResultWire::ConversationDelete,
                                    AgentComponentCallInputWire::ConversationSaveMessages { .. } => AgentComponentCallResultWire::ConversationSaveMessages,
                                    AgentComponentCallInputWire::ConversationGetMessages { .. } => {
                                        AgentComponentCallResultWire::ConversationGetMessages { messages: Vec::new() }
                                    }
                                    AgentComponentCallInputWire::ConversationCountMessages { .. } => {
                                        AgentComponentCallResultWire::ConversationCountMessages { count: WireU64::from_u64(0) }
                                    }
                                    AgentComponentCallInputWire::ConversationEnsure { conversation } => {
                                        AgentComponentCallResultWire::ConversationEnsure { conversation }
                                    }
                                    AgentComponentCallInputWire::ConversationSearch { .. } => {
                                        AgentComponentCallResultWire::ConversationSearch { conversations: Vec::new() }
                                    }
                                    AgentComponentCallInputWire::RunSave { .. } => AgentComponentCallResultWire::RunSave,
                                    AgentComponentCallInputWire::RunLoad { .. } => AgentComponentCallResultWire::RunLoad { run: None },
                                    AgentComponentCallInputWire::RunListBySession { .. } => AgentComponentCallResultWire::RunListBySession { runs: Vec::new() },
                                    AgentComponentCallInputWire::RunListAll { .. } => AgentComponentCallResultWire::RunListAll { runs: Vec::new() },
                                    AgentComponentCallInputWire::RunAppendEvent { .. } => AgentComponentCallResultWire::RunAppendEvent,
                                    AgentComponentCallInputWire::RunListByParent { .. } => AgentComponentCallResultWire::RunListByParent { runs: Vec::new() },
                                    AgentComponentCallInputWire::RuntimeGetCheckpoint { .. } => AgentComponentCallResultWire::RuntimeGetCheckpoint { checkpoint: None },
                                    AgentComponentCallInputWire::RuntimeSaveCheckpoint { .. } => AgentComponentCallResultWire::RuntimeSaveCheckpoint,
                                    AgentComponentCallInputWire::RuntimeSaveCheckpointForScope { .. } => AgentComponentCallResultWire::RuntimeSaveCheckpointForScope,
                                    AgentComponentCallInputWire::RuntimeStateIds { .. } => AgentComponentCallResultWire::RuntimeStateIds { state_ids: Vec::new() },
                                    AgentComponentCallInputWire::RuntimeClearState { .. } => {
                                        let receipt = WireValue::from_json(serde_json::json!({
                                            "scope_id": "scope",
                                            "runtime_state_id": "runtime",
                                            "checkpoint_removed": false
                                        }))
                                        .map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?;
                                        AgentComponentCallResultWire::RuntimeClearState { receipt }
                                    }
                                    AgentComponentCallInputWire::RuntimeClearScope { .. } => {
                                        let receipt = WireValue::from_json(serde_json::json!({
                                            "scope_id": "scope",
                                            "runtime_state_ids": []
                                        }))
                                        .map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?;
                                        AgentComponentCallResultWire::RuntimeClearScope { receipt }
                                    }
                                    AgentComponentCallInputWire::RuntimeClearConversation { .. } => AgentComponentCallResultWire::RuntimeClearConversation,
                                    AgentComponentCallInputWire::AuditLog { .. } => AgentComponentCallResultWire::AuditLog,
                                    AgentComponentCallInputWire::AuditQuery { .. } => AgentComponentCallResultWire::AuditQuery { events: Vec::new() },
                                    AgentComponentCallInputWire::ContextProject { .. } => AgentComponentCallResultWire::ContextProject { projections: Vec::new() },
                                    AgentComponentCallInputWire::MemoryTrigger { .. } => AgentComponentCallResultWire::MemoryTrigger { disposition: "persist".to_string() },
                                    AgentComponentCallInputWire::GuardCheck { .. } => {
                                        AgentComponentCallResultWire::GuardCheck {
                                            result: WireValue::String("Pass".to_string()),
                                        }
                                    }
                                    AgentComponentCallInputWire::SearchProviderSearch { .. } => {
                                        AgentComponentCallResultWire::SearchProviderSearch {
                                            results: Vec::new(),
                                        }
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointSave { checkpoint } => {
                                        if let Err(error) = checkpoints.save(checkpoint) {
                                            return responder.respond(ExtensionInvokeOutcome::Error { error: *error });
                                        }
                                        AgentComponentCallResultWire::WorkflowCheckpointSave
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointSaveIfGeneration { checkpoint, expected_generation } => {
                                        let committed = match checkpoints.save_if_generation(checkpoint, expected_generation) {
                                            Ok(committed) => committed,
                                            Err(error) => return responder.respond(ExtensionInvokeOutcome::Error { error: *error }),
                                        };
                                        AgentComponentCallResultWire::WorkflowCheckpointSaveIfGeneration { committed }
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointLoad { checkpoint_id } => {
                                        let checkpoint = match checkpoints.load(&checkpoint_id) {
                                            Ok(checkpoint) => checkpoint,
                                            Err(error) => return responder.respond(ExtensionInvokeOutcome::Error { error: *error }),
                                        };
                                        AgentComponentCallResultWire::WorkflowCheckpointLoad { checkpoint }
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointClaim { checkpoint_id } => {
                                        let checkpoint = match checkpoints.claim(&checkpoint_id) {
                                            Ok(checkpoint) => checkpoint,
                                            Err(error) => return responder.respond(ExtensionInvokeOutcome::Error { error: *error }),
                                        };
                                        AgentComponentCallResultWire::WorkflowCheckpointClaim { checkpoint }
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointProtectClaim { checkpoint_id, attempt_id } => {
                                        if let Err(error) = checkpoints.protect(&checkpoint_id, &attempt_id) {
                                            return responder.respond(ExtensionInvokeOutcome::Error { error: *error });
                                        }
                                        AgentComponentCallResultWire::WorkflowCheckpointProtectClaim
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointAckClaim { checkpoint_id, attempt_id } => {
                                        if let Err(error) = checkpoints.settle_claim(&checkpoint_id, &attempt_id, false) {
                                            return responder.respond(ExtensionInvokeOutcome::Error { error: *error });
                                        }
                                        AgentComponentCallResultWire::WorkflowCheckpointAckClaim
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointRequeueClaim { checkpoint_id, attempt_id } => {
                                        if let Err(error) = checkpoints.settle_claim(&checkpoint_id, &attempt_id, true) {
                                            return responder.respond(ExtensionInvokeOutcome::Error { error: *error });
                                        }
                                        AgentComponentCallResultWire::WorkflowCheckpointRequeueClaim
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointRenewClaim { checkpoint_id, attempt_id } => {
                                        if let Err(error) = checkpoints.renew(&checkpoint_id, &attempt_id) {
                                            return responder.respond(ExtensionInvokeOutcome::Error { error: *error });
                                        }
                                        AgentComponentCallResultWire::WorkflowCheckpointRenewClaim
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointList => AgentComponentCallResultWire::WorkflowCheckpointList { checkpoints: Vec::new() },
                                    AgentComponentCallInputWire::WorkflowCheckpointListByGraph { .. } => AgentComponentCallResultWire::WorkflowCheckpointListByGraph { checkpoints: Vec::new() },
                                    AgentComponentCallInputWire::WorkflowCheckpointListFiltered { .. } => AgentComponentCallResultWire::WorkflowCheckpointListFiltered { checkpoints: Vec::new() },
                                    AgentComponentCallInputWire::WorkflowCheckpointDelete { checkpoint_id } => {
                                        if let Err(error) = checkpoints.delete(&checkpoint_id) {
                                            return responder.respond(ExtensionInvokeOutcome::Error { error: *error });
                                        }
                                        AgentComponentCallResultWire::WorkflowCheckpointDelete
                                    }
                                    AgentComponentCallInputWire::WorkflowCheckpointClear => {
                                        if let Err(error) = checkpoints.clear() {
                                            return responder.respond(ExtensionInvokeOutcome::Error { error: *error });
                                        }
                                        AgentComponentCallResultWire::WorkflowCheckpointClear
                                    },
                                    AgentComponentCallInputWire::RevisionedTaskLoad { .. }
                                    | AgentComponentCallInputWire::RevisionedTaskCompareAndCommit { .. }
                                    | AgentComponentCallInputWire::RevisionedTaskCompareAndCommitExecution { .. } => {
                                        match task_store.handle(&input.call) {
                                            Ok(result) => result,
                                            Err(error) => return responder.respond(ExtensionInvokeOutcome::Error { error: *error }),
                                        }
                                    },
                                    AgentComponentCallInputWire::SandboxIsAvailable => AgentComponentCallResultWire::SandboxIsAvailable { available: true },
                                    AgentComponentCallInputWire::SandboxExecute { .. } => AgentComponentCallResultWire::SandboxExecute {
                                        result: WireValue::from_json(serde_json::json!({
                                            "exit_code": 0,
                                            "stdout": "sandbox answer",
                                            "stderr": "",
                                            "duration": {"secs": 0, "nanos": 1_000_000},
                                            "sandbox_type": "sdk",
                                            "timed_out": false,
                                            "cancelled": false,
                                            "output_truncated": false,
                                            "stdout_bytes": 14,
                                            "stderr_bytes": 0
                                        })).map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?,
                                    },
                                    AgentComponentCallInputWire::SandboxExecuteWithLimits { .. } => AgentComponentCallResultWire::SandboxExecuteWithLimits {
                                        result: WireValue::from_json(serde_json::json!({
                                            "exit_code": 0,
                                            "stdout": "sandbox answer",
                                            "stderr": "",
                                            "duration": {"secs": 0, "nanos": 1_000_000},
                                            "sandbox_type": "sdk",
                                            "timed_out": false,
                                            "cancelled": false,
                                            "output_truncated": false,
                                            "stdout_bytes": 14,
                                            "stderr_bytes": 0
                                        })).map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?,
                                    },
                                    AgentComponentCallInputWire::SandboxExecuteWithLimitsAndCancel { .. } => AgentComponentCallResultWire::SandboxExecuteWithLimitsAndCancel {
                                        result: WireValue::from_json(serde_json::json!({
                                            "exit_code": 0,
                                            "stdout": "sandbox answer",
                                            "stderr": "",
                                            "duration": {"secs": 0, "nanos": 1_000_000},
                                            "sandbox_type": "sdk",
                                            "timed_out": false,
                                            "cancelled": false,
                                            "output_truncated": false,
                                            "stdout_bytes": 14,
                                            "stderr_bytes": 0
                                        })).map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?,
                                    },
                                    AgentComponentCallInputWire::SandboxCleanup => AgentComponentCallResultWire::SandboxCleanup,
                                    AgentComponentCallInputWire::McpTransportSend { request } => {
                                        let Ok(request) = request.into_json() else {
                                            return responder.respond(ExtensionInvokeOutcome::Error {
                                                error: EchoSdkError::new(
                                                    ExtensionErrorCode::InvalidValue,
                                                    "MCP request is not JSON",
                                                    Retryability::Never,
                                                ),
                                            });
                                        };
                                        let Ok(response) = WireValue::from_json(serde_json::json!({
                                            "jsonrpc": "2.0",
                                            "id": request.get("id").cloned(),
                                            "result": {
                                                "protocolVersion": "2025-11-25",
                                                "capabilities": {},
                                                "serverInfo": {
                                                    "name": "sdk-transport",
                                                    "version": "1.0.0"
                                                }
                                            }
                                        })) else {
                                            return responder.respond(ExtensionInvokeOutcome::Error {
                                                error: EchoSdkError::new(
                                                    ExtensionErrorCode::SerializationViolation,
                                                    "MCP response could not be encoded",
                                                    Retryability::Never,
                                                ),
                                            });
                                        };
                                        AgentComponentCallResultWire::McpTransportSend { response }
                                    },
                                    AgentComponentCallInputWire::McpTransportNotify { .. } => AgentComponentCallResultWire::McpTransportNotify,
                                    AgentComponentCallInputWire::McpTransportClose => AgentComponentCallResultWire::McpTransportClose,
                                    AgentComponentCallInputWire::McpTransportTryNotification => AgentComponentCallResultWire::McpTransportTryNotification { notification: None },
                                    AgentComponentCallInputWire::EmbedderEmbed { .. } => AgentComponentCallResultWire::EmbedderEmbed { vector: vec![0.0, 1.0] },
                                    AgentComponentCallInputWire::MemoryPromoterPromote { evicted } => AgentComponentCallResultWire::MemoryPromoterPromote {
                                        submitted: WireU64::from_u64(u64::try_from(evicted.len()).unwrap_or(u64::MAX)),
                                        promoted: WireU64::from_u64(0),
                                        deduplicated: WireU64::from_u64(0),
                                    },
                                    AgentComponentCallInputWire::WorkflowRun { .. } => AgentComponentCallResultWire::WorkflowRun { output: WireValue::Null },
                                    AgentComponentCallInputWire::IntentClassify { .. } => AgentComponentCallResultWire::IntentClassify {
                                        intent: WireValue::String("Fallback".to_string()),
                                    },
                                    AgentComponentCallInputWire::SkillLoadAllows { descriptor } => {
                                        AgentComponentCallResultWire::SkillLoadAllows {
                                            allowed: descriptor.name != "blocked",
                                        }
                                    }
                                    AgentComponentCallInputWire::SandboxExecuteStream { .. }
                                    | AgentComponentCallInputWire::WorkflowRunStream { .. } => {
                                        return responder.respond(ExtensionInvokeOutcome::Error {
                                            error: EchoSdkError::new(
                                                ExtensionErrorCode::InvalidValue,
                                                "streaming component used non-streaming invocation",
                                                Retryability::Never,
                                            ),
                                        });
                                    }
                                    _ => return responder.respond(ExtensionInvokeOutcome::Error {
                                        error: EchoSdkError::new(ExtensionErrorCode::FeatureUnavailable, "fixture component operation is not implemented", Retryability::Never),
                                    }),
                                };
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::AgentComponentCall(
                                        AgentComponentResultWire {
                                            component: input.component,
                                            result,
                                        },
                                    ),
                                })
                            }
                            ExtensionInvocation::AgentComponentCallStream(input) => {
                                let Some(stream) = call.stream.clone() else {
                                    return responder.respond(ExtensionInvokeOutcome::Error {
                                        error: EchoSdkError::new(
                                            ExtensionErrorCode::ExtensionFailed,
                                            "missing Agent component stream handle",
                                            Retryability::Never,
                                        ),
                                    });
                                };
                                responder.respond(ExtensionInvokeOutcome::Stream {
                                    stream: stream.clone(),
                                })?;
                                tokio::spawn(async move {
                                    let value = match input.call {
                                        AgentComponentCallInputWire::SandboxExecuteStream { .. } => {
                                            let Ok(result) = WireValue::from_json(serde_json::json!({
                                                "exit_code": 0,
                                                "stdout": "streamed sandbox",
                                                "stderr": "",
                                                "duration": {"secs": 0, "nanos": 1_000_000},
                                                "sandbox_type": "sdk",
                                                "timed_out": false,
                                                "cancelled": false,
                                                "output_truncated": false,
                                                "stdout_bytes": 16,
                                                "stderr_bytes": 0
                                            })) else {
                                                return;
                                            };
                                            ExtensionStreamCompleteValue::AgentComponent(
                                                AgentComponentStreamCompleteWire::Sandbox(
                                                    SandboxStreamCompleteWire::Complete { result },
                                                ),
                                            )
                                        }
                                        AgentComponentCallInputWire::WorkflowRunStream { .. } => {
                                            ExtensionStreamCompleteValue::AgentComponent(
                                                AgentComponentStreamCompleteWire::Workflow(
                                                    WorkflowStreamCompleteWire {
                                                        result: "streamed workflow".to_string(),
                                                        total_steps: WireU64::from_u64(1),
                                                        elapsed: WireDuration {
                                                            seconds: WireU64::from_u64(0),
                                                            nanos: 1_000_000,
                                                        },
                                                    },
                                                ),
                                            )
                                        }
                                        _ => return,
                                    };
                                    let _ = connection.send_notification(
                                        ExtensionStreamEvent::Complete {
                                            stream,
                                            sequence: nonzero(1),
                                            value,
                                        },
                                    );
                                });
                                Ok(())
                            }
                            ExtensionInvocation::CriticCritique(input) => {
                                assert_eq!(input.task, "solve");
                                assert_eq!(input.answer, "42");
                                assert_eq!(input.context, "math");
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::CriticCritique(CritiqueWire {
                                        score: 9.5,
                                        passed: true,
                                        feedback: "good".to_string(),
                                        suggestions: Vec::new(),
                                    }),
                                })
                            }
                            ExtensionInvocation::LlmChatStream(_) => {
                                let Some(stream) = call.stream.clone() else {
                                    return responder.respond(ExtensionInvokeOutcome::Error {
                                        error: EchoSdkError::new(
                                            ExtensionErrorCode::ExtensionFailed,
                                            "missing stream handle",
                                            Retryability::Never,
                                        ),
                                    });
                                };
                                if flood_stream.load(Ordering::Acquire) {
                                    // Deliver the first burst before answering
                                    // the reverse request. The Host has already
                                    // minted and registered the sink, but the
                                    // framework stream consumer cannot start
                                    // draining until the stream outcome is
                                    // received. This makes the bounded-mailbox
                                    // branch deterministic instead of racing
                                    // the consumer scheduler.
                                    for sequence in 1_u64..=2 {
                                        connection.send_notification(
                                            ExtensionStreamEvent::Chunk {
                                                stream: stream.clone(),
                                                sequence: nonzero(sequence),
                                                value: ExtensionStreamChunkValue::Llm(
                                                    chat_stream_chunk_wire("x"),
                                                ),
                                            },
                                        )?;
                                    }
                                    // Both valid chunks precede the ACK and exercise
                                    // the bounded mailbox. A real final event from
                                    // this same producer supplies physical closure.
                                    connection.send_notification(ExtensionStreamEvent::Complete {
                                        stream: stream.clone(), sequence: nonzero(3),
                                        value: ExtensionStreamCompleteValue::Llm(chat_stream_complete_wire("stop")),
                                    })?;
                                    return responder
                                        .respond(ExtensionInvokeOutcome::Stream { stream });
                                }
                                if omit_stream_terminal.load(Ordering::Acquire) {
                                    return start_held_fixture_producer(original_call, responder, connection, &stream_producers);
                                }
                                responder.respond(ExtensionInvokeOutcome::Stream {
                                    stream: stream.clone(),
                                })?;
                                // Deliver the chunks from a spawned task so the
                                // client dispatch loop is never blocked by the
                                // callback's own stream production (design §12.3:
                                // the reader loop keeps dispatching).
                                tokio::spawn(async move {
                                    let reentrant = reentrant_mutation
                                        .lock()
                                        .unwrap_or_else(|error| error.into_inner())
                                        .clone();
                                    if let Some((agent, session)) = reentrant {
                                        let mutation = "echo_agent::agent::react::ReactAgent::set_plan_mode";
                                        let signature_digest =
                                            facade_digest(mutation).ok();
                                        let message = signature_digest.and_then(|signature_digest| {
                                            UntypedMessage::new(
                                                "_echo_agent/facade/invoke",
                                                serde_json::json!({
                                                    "operation": mutation,
                                                    "signature_digest": signature_digest,
                                                    "handle": agent,
                                                    "arguments": [
                                                        {"kind": "handle", "value": session},
                                                        {"kind": "bool", "value": true}
                                                    ],
                                                }),
                                            )
                                            .ok()
                                        });
                                        if let Some(message) = message
                                            && let Err(error) = connection
                                                .send_request(message)
                                                .block_task()
                                                .await
                                            && EchoSdkError::from_jsonrpc_data(error.data.as_ref())
                                                .is_ok_and(|typed| {
                                                    typed.code
                                                        == ExtensionErrorCode::ExtensionConflict
                                                })
                                        {
                                            reentrant_conflicts.fetch_add(1, Ordering::AcqRel);
                                        }
                                    }
                                    if oversized_stream.load(Ordering::Acquire) {
                                        let _ = connection.send_notification(
                                            ExtensionStreamEvent::Chunk {
                                                stream: stream.clone(),
                                                sequence: nonzero(1),
                                                value: ExtensionStreamChunkValue::Llm(
                                                    chat_stream_chunk_wire(&"x".repeat(300_000)),
                                                ),
                                            },
                                        );
                                        let _ = connection.send_notification(ExtensionStreamEvent::Complete {
                                            stream, sequence: nonzero(1),
                                            value: ExtensionStreamCompleteValue::Llm(chat_stream_complete_wire("stop")),
                                        });
                                        return;
                                    }
                                    for (sequence, text) in [(1_u64, "streamed "), (2, "answer")] {
                                        let sequence = if out_of_order_stream
                                            .load(Ordering::Acquire)
                                            && sequence == 2
                                        {
                                            1
                                        } else {
                                            sequence
                                        };
                                        let value = if malformed_stream.load(Ordering::Acquire) {
                                            ExtensionStreamChunkValue::Agent(
                                                AgentStreamChunkWire::Token {
                                                    text: text.to_string(),
                                                },
                                            )
                                        } else {
                                            ExtensionStreamChunkValue::Llm(chat_stream_chunk_wire(
                                                text,
                                            ))
                                        };
                                        let event = ExtensionStreamEvent::Chunk {
                                            stream: stream.clone(),
                                            sequence: nonzero(sequence),
                                            value,
                                        };
                                        if connection.send_notification(event).is_err() {
                                            return;
                                        }
                                    }
                                    if omit_stream_terminal.load(Ordering::Acquire) {
                                        return;
                                    }
                                    // Rejected kind/sequence input is not physical
                                    // closure. Finish the original producer with a
                                    // valid event at its next accepted remote index.
                                    let final_sequence = if malformed_stream.load(Ordering::Acquire) {
                                        1
                                    } else if out_of_order_stream.load(Ordering::Acquire) {
                                        2
                                    } else {
                                        3
                                    };
                                    let terminal = ExtensionStreamEvent::Complete {
                                        stream: stream.clone(),
                                        sequence: nonzero(final_sequence),
                                        value: ExtensionStreamCompleteValue::Llm(chat_stream_complete_wire("stop")),
                                    };
                                    let _ = connection.send_notification(terminal.clone());
                                    if duplicate_terminal.load(Ordering::Acquire) {
                                        let _ = connection.send_notification(terminal);
                                    }
                                });
                                Ok(())
                            }
                            // Hooks answer with the neutral result so lifecycle
                            // flow keeps moving; only explicitly-hanging
                            // operations stay silent.
                            ExtensionInvocation::HookRun(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::HookRun(HookResultWire::default()),
                                })
                            }
                            ExtensionInvocation::HumanLoopRequest(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::HumanLoopRequest(
                                        HumanLoopResponseWire::Text {
                                            text: "SDK human-loop answer".to_string(),
                                        },
                                    ),
                                })
                            }
                            ExtensionInvocation::CallbackOnToolInterrupted(input) => {
                                // Admission alone does not prove completion; this
                                // side effect intentionally follows Run cancellation.
                                tokio::time::sleep(Duration::from_millis(250)).await;
                                interrupted_callbacks.lock().unwrap_or_else(|error| error.into_inner()).push(input);
                                responder.respond(ExtensionInvokeOutcome::Result { result: ExtensionResult::CallbackOnToolInterrupted(ExtensionUnit) })
                            }
                            ExtensionInvocation::FactoryCreateAgent(config) => {
                                factory_instances.lock().unwrap_or_else(|error| error.into_inner()).insert(config.instance.id.clone(), call.extension.id.clone());
                                factory_created.lock().unwrap_or_else(|error| error.into_inner()).push(config.instance.id);
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::FactoryCreateAgent(
                                        CustomAgentDescriptorWire {
                                            name: config.name,
                                            model_name: "sdk-custom-model".to_string(),
                                            system_prompt: "SDK custom agent".to_string(),
                                            tool_names: Vec::new(),
                                        },
                                    ),
                                })
                            }
                            ExtensionInvocation::AgentExecute(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::AgentExecute(
                                        "SDK custom execute".to_string(),
                                    ),
                                })
                            }
                            ExtensionInvocation::AgentChat(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::AgentChat(
                                        "SDK custom chat".to_string(),
                                    ),
                                })
                            }
                            ExtensionInvocation::AgentClose(_) => {
                                responder.respond(match complete_fixture_agent_close(&original_call, &factory_instances, &factory_closed) {
                                    Ok(outcome) => outcome,
                                    Err(error) => ExtensionInvokeOutcome::Error { error: *error },
                                })
                            }
                            #[cfg(feature = "framework-channels")]
                            ExtensionInvocation::ChannelStart(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::ChannelStart(ExtensionUnit),
                                })
                            }
                            #[cfg(feature = "framework-channels")]
                            ExtensionInvocation::ChannelStop(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::ChannelStop(ExtensionUnit),
                                })
                            }
                            #[cfg(feature = "framework-channels")]
                            ExtensionInvocation::ChannelSend(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::ChannelSend(ExtensionUnit),
                                })
                            }
                            #[cfg(feature = "framework-channels")]
                            ExtensionInvocation::ChannelHealth(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::ChannelHealth(ExtensionUnit),
                                })
                            }
                            #[cfg(feature = "framework-channels")]
                            ExtensionInvocation::ChannelHandle(input) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::ChannelHandle(
                                        channel_outbound_fixture(&format!(
                                            "reply: {}",
                                            input.message.text
                                        )),
                                    ),
                                })
                            }
                            #[cfg(feature = "framework-channels")]
                            ExtensionInvocation::ChannelHandleStream(input) => {
                                let Some(stream) = call.stream.clone() else {
                                    return responder.respond(ExtensionInvokeOutcome::Error {
                                        error: EchoSdkError::new(
                                            ExtensionErrorCode::ExtensionFailed,
                                            "missing stream handle",
                                            Retryability::Never,
                                        ),
                                    });
                                };
                                let text = format!("stream reply: {}", input.message.text);
                                responder.respond(ExtensionInvokeOutcome::Stream {
                                    stream: stream.clone(),
                                })?;
                                tokio::spawn(async move {
                                    let _ =
                                        connection.send_notification(ExtensionStreamEvent::Chunk {
                                            stream: stream.clone(),
                                            sequence: nonzero(1),
                                            value: ExtensionStreamChunkValue::Channel(
                                                channel_outbound_fixture(&text),
                                            ),
                                        });
                                    let _ = connection.send_notification(
                                        ExtensionStreamEvent::Complete {
                                            stream,
                                            sequence: nonzero(2),
                                            value: ExtensionStreamCompleteValue::Channel(
                                                channel_outbound_fixture("stream done"),
                                            ),
                                        },
                                    );
                                });
                                Ok(())
                            }
                            #[cfg(feature = "framework-channels")]
                            ExtensionInvocation::ChannelReply(_) => {
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: ExtensionResult::ChannelReply(ExtensionUnit),
                                })
                            }
                            ExtensionInvocation::AgentExecuteStream(_)
                            | ExtensionInvocation::AgentChatStream(_) => {
                                let Some(stream) = call.stream.clone() else {
                                    return responder.respond(ExtensionInvokeOutcome::Error {
                                        error: EchoSdkError::new(
                                            ExtensionErrorCode::ExtensionFailed,
                                            "missing stream handle",
                                            Retryability::Never,
                                        ),
                                    });
                                };
                                if omit_stream_terminal.load(Ordering::Acquire) {
                                    return start_held_fixture_producer(original_call, responder, connection, &stream_producers);
                                }
                                responder.respond(ExtensionInvokeOutcome::Stream {
                                    stream: stream.clone(),
                                })?;
                                tokio::spawn(async move {
                                    let _ =
                                        connection.send_notification(ExtensionStreamEvent::Chunk {
                                            stream: stream.clone(),
                                            sequence: nonzero(1),
                                            value: ExtensionStreamChunkValue::Agent(
                                                AgentStreamChunkWire::Token {
                                                    text: "SDK custom event".to_string(),
                                                },
                                            ),
                                        });
                                    if omit_stream_terminal.load(Ordering::Acquire) { return; }
                                    let _ = connection.send_notification(
                                        ExtensionStreamEvent::Complete {
                                            stream,
                                            sequence: nonzero(2),
                                            value: ExtensionStreamCompleteValue::Agent(
                                                AgentStreamTerminalWire::FinalAnswer {
                                                    text: "SDK custom answer".to_string(),
                                                },
                                            ),
                                        },
                                    );
                                });
                                Ok(())
                            }
                            _ => {
                                // Observational callbacks, interventions and
                                // stream variants answer with neutral results.
                                responder.respond(ExtensionInvokeOutcome::Result {
                                    result: neutral_result_wire(operation),
                                })
                            }
                        }
                    }
                }
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_request(
            async move |message: UntypedMessage, responder: Responder<serde_json::Value>, _connection: ConnectionTo<agent_client_protocol::Agent>| {
                let reason = if message.method == "_echo_agent/extension/invoke" {
                    serde_json::from_value::<ExtensionInvokeCall>(message.params).err().map(|error| error.to_string()).unwrap_or_else(|| "unhandled typed invocation".to_string())
                } else { format!("unexpected SDK fixture request {}", message.method) };
                responder.respond_with_error(RpcError::invalid_params().data(reason))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .on_receive_notification(
            {
                let cancel_notices = cancel_notices.clone();
                async move |notice: echo_sdk_protocol::methods::ExtensionCancelNotice,
                            _connection: ConnectionTo<agent_client_protocol::Agent>| {
                    cancel_notices
                        .lock()
                        .expect("cancel lock")
                        .push(format!("{}:{}", notice.invocation_id, notice.reason));
                    Ok(())
                }
            },
            agent_client_protocol::on_receive_notification!(),
        )
        .on_receive_notification(
            async move |_notification: v1::SessionNotification,
                        _connection: ConnectionTo<agent_client_protocol::Agent>| {
                Ok(())
            },
            agent_client_protocol::on_receive_notification!(),
        )
        .on_receive_notification(
            async move |notification: echo_sdk_protocol::event::EventNotification,
                        connection: ConnectionTo<agent_client_protocol::Agent>| {
                let sequence = notification.envelope.sequence.clone();
                event_log.lock().unwrap_or_else(|error| error.into_inner()).push(notification.envelope);
                connection.send_notification(echo_sdk_protocol::event::EventAckNotification {
                    ack: echo_sdk_protocol::event::EventAck { stream: notification.stream, last_processed_sequence: sequence },
                })?;
                Ok(())
            },
            agent_client_protocol::on_receive_notification!(),
        )
        .connect_with(transport, async move |connection| {
            scenario(connection).await
        });
    let outcome = tokio::time::timeout(Duration::from_secs(60), connect)
        .await
        .map_err(|_| {
            let operations = operations
                .lock()
                .map(|value| value.clone())
                .unwrap_or_default();
            format!(
                "SDK scenario timed out; operations={operations:?}; stderr:\n{}",
                stderr_text(host)
            )
        })??;
    Ok(outcome)
}

fn nonzero(value: u64) -> WireNonZeroU64 {
    WireNonZeroU64::try_from(value.to_string()).expect("positive decimal")
}

fn tool_result_wire(output: &str) -> ToolResultWire {
    ToolResultWire {
        kind: ToolResultKindWire::Text,
        success: true,
        output: output.to_string(),
        error: None,
        failure: None,
        data: None,
        truncated: false,
        mime_type: None,
        artifact: None,
        effects: Vec::new(),
        metadata: std::collections::BTreeMap::new(),
        model_content: Vec::new(),
    }
}

fn chat_response_wire(text: &str) -> LlmChatResponseWire {
    LlmChatResponseWire {
        message: LlmMessageWire {
            role: "assistant".to_string(),
            content: WireValue::String(text.to_string()),
            tool_calls: None,
            name: None,
            tool_call_id: None,
            reasoning_content: None,
            reasoning_blocks: None,
        },
        finish_reason: Some("stop".to_string()),
        usage: None,
        raw: WireValue::from_json(serde_json::json!({})).expect("empty raw response"),
    }
}

fn chat_stream_chunk_wire(text: &str) -> LlmStreamChunkWire {
    LlmStreamChunkWire {
        content: (!text.is_empty()).then(|| text.to_string()),
        ..LlmStreamChunkWire::default()
    }
}

fn chat_stream_complete_wire(finish_reason: &str) -> LlmStreamCompleteWire {
    LlmStreamCompleteWire {
        role: None,
        content: None,
        reasoning_content: None,
        reasoning_blocks: None,
        tool_calls: None,
        finish_reason: finish_reason.to_string(),
        usage: None,
    }
}

fn neutral_result_wire(operation: ExtensionOperation) -> ExtensionResult {
    match operation {
        ExtensionOperation::StorePut => ExtensionResult::StorePut(ExtensionUnit),
        ExtensionOperation::StoreGet => ExtensionResult::StoreGet(None),
        ExtensionOperation::StoreSearch => ExtensionResult::StoreSearch(Vec::new()),
        ExtensionOperation::StoreSearchWith => ExtensionResult::StoreSearchWith(Vec::new()),
        ExtensionOperation::StoreDelete => ExtensionResult::StoreDelete(false),
        ExtensionOperation::StoreListNamespaces => ExtensionResult::StoreListNamespaces(Vec::new()),
        ExtensionOperation::StoreList => ExtensionResult::StoreList(Vec::new()),
        ExtensionOperation::StorePruneExpired => {
            ExtensionResult::StorePruneExpired(WireU64::from_u64(0))
        }
        ExtensionOperation::StoreDedupByContent => {
            ExtensionResult::StoreDedupByContent(WireU64::from_u64(0))
        }
        ExtensionOperation::CallbackOnThinkStart => {
            ExtensionResult::CallbackOnThinkStart(ExtensionUnit)
        }
        ExtensionOperation::CallbackOnThinkEnd => {
            ExtensionResult::CallbackOnThinkEnd(ExtensionUnit)
        }
        ExtensionOperation::CallbackOnToolStart => {
            ExtensionResult::CallbackOnToolStart(ExtensionUnit)
        }
        ExtensionOperation::CallbackOnToolEnd => ExtensionResult::CallbackOnToolEnd(ExtensionUnit),
        ExtensionOperation::CallbackOnToolError => {
            ExtensionResult::CallbackOnToolError(ExtensionUnit)
        }
        ExtensionOperation::CallbackOnToolInterrupted => {
            ExtensionResult::CallbackOnToolInterrupted(ExtensionUnit)
        }
        ExtensionOperation::CallbackOnFinalAnswer => {
            ExtensionResult::CallbackOnFinalAnswer(ExtensionUnit)
        }
        ExtensionOperation::CallbackOnIteration => {
            ExtensionResult::CallbackOnIteration(ExtensionUnit)
        }
        ExtensionOperation::InterventionOnToolCall => {
            ExtensionResult::InterventionOnToolCall(InterventionResultWire::default())
        }
        ExtensionOperation::InterventionOnThinkStart => {
            ExtensionResult::InterventionOnThinkStart(InterventionResultWire::default())
        }
        ExtensionOperation::InterventionOnFinalAnswer => {
            ExtensionResult::InterventionOnFinalAnswer(InterventionResultWire::default())
        }
        ExtensionOperation::AgentClose => ExtensionResult::AgentClose(ExtensionUnit),
        ExtensionOperation::CriticCritique => ExtensionResult::CriticCritique(CritiqueWire {
            score: 10.0,
            passed: true,
            feedback: String::new(),
            suggestions: Vec::new(),
        }),
        _ => ExtensionResult::CallbackOnIteration(ExtensionUnit),
    }
}

async fn register_extension(
    connection: &ConnectionTo<agent_client_protocol::Agent>,
    kind: ExtensionKind,
    implementation_id: &str,
    descriptor: ExtensionDescriptor,
    timeout: Option<WireDuration>,
) -> Result<WireHandle, RpcError> {
    let response: ExtensionRegisterResponse = connection
        .send_request(ExtensionRegisterRequest {
            kind,
            implementation_id: implementation_id.to_string(),
            descriptor,
            timeout,
        })
        .block_task()
        .await?;
    Ok(response.extension)
}

async fn unregister(
    connection: &ConnectionTo<agent_client_protocol::Agent>,
    extension: &WireHandle,
) -> Result<bool, RpcError> {
    let response: ExtensionUnregisterResponse = connection
        .send_request(ExtensionUnregisterRequest {
            extension: extension.clone(),
        })
        .block_task()
        .await?;
    Ok(response.released)
}

async fn invoke_source_operation(
    connection: &ConnectionTo<agent_client_protocol::Agent>,
    agent: WireHandle,
    operation: &str,
    arguments: Vec<WireValue>,
) -> Result<FeatureOperationResponse, RpcError> {
    let signature_digest = facade_digest(operation)
        .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
    let request = FeatureOperationRequest {
        operation: operation.to_string(),
        signature_digest,
        handle: Some(agent),
        arguments,
    };
    let value = connection
        .send_request(UntypedMessage::new(
            "_echo_agent/facade/invoke",
            serde_json::to_value(request)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
        )?)
        .block_task()
        .await?;
    serde_json::from_value(value)
        .map_err(|error| RpcError::internal_error().data(error.to_string()))
}

/// Scenario A: negotiation advertises the bridge; a tool round trip drives
/// callbacks, an intervention and a hook; unregister and the conflict /
/// stale matrix behave as contracted.
#[tokio::test]
async fn tool_bridge_round_trip_with_callbacks_intervention_and_hook()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![
        tool_call_script("search_docs", r#"{"query":"bridges"}"#),
        final_script("tool answered"),
    ])
    .await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        Box::pin(async move {
            // Negotiate: the advertisement must declare the bridge with the
            // extension limits.
            let initialize: v1::InitializeResponse = connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let meta = initialize
                .agent_capabilities
                .meta
                .as_ref()
                .and_then(|meta| meta.get("echo_agent").cloned())
                .expect("echo_agent capability present");
            let capability: EchoAgentCapability =
                serde_json::from_value(meta).expect("capability decodes");
            assert!(capability.declares(ExtensionCapability::ExtensionBridge));
            assert!(
                capability
                    .limits
                    .max_registered_extensions
                    .to_u64()
                    .is_some_and(|value| value > 0)
            );

            // Register the extension family BEFORE creating the Session.
            let tool = register_extension(
                &connection,
                ExtensionKind::Tool,
                "sdk-tool",
                tool_descriptor("search_docs"),
                None,
            )
            .await
            .expect("tool registers");
            let callback = register_extension(
                &connection,
                ExtensionKind::AgentCallback,
                "sdk-callback",
                ExtensionDescriptor::AgentCallback {
                    descriptor_version: 1,
                },
                None,
            )
            .await
            .expect("callback registers");
            let intervention = register_extension(
                &connection,
                ExtensionKind::InterventionCallback,
                "sdk-intervention",
                ExtensionDescriptor::InterventionCallback {
                    descriptor_version: 1,
                },
                None,
            )
            .await
            .expect("intervention registers");
            let hook = register_extension(
                &connection,
                ExtensionKind::Hook,
                "sdk-hook",
                ExtensionDescriptor::Hook {
                    descriptor_version: 1,
                    events: Vec::new(),
                },
                None,
            )
            .await
            .expect("hook registers");

            // Same identity + same descriptor is idempotent; a different
            // descriptor is a typed conflict.
            let again = register_extension(
                &connection,
                ExtensionKind::Tool,
                "sdk-tool",
                tool_descriptor("search_docs"),
                None,
            )
            .await
            .expect("idempotent registration");
            assert_eq!(again.id, tool.id);
            let conflict = connection
                .send_request(ExtensionRegisterRequest {
                    kind: ExtensionKind::Tool,
                    implementation_id: "sdk-tool".to_string(),
                    descriptor: tool_descriptor("other_tool"),
                    timeout: None,
                })
                .block_task()
                .await;
            assert!(conflict.is_err(), "descriptor conflict must fail closed");
            let semantic_conflict = connection
                .send_request(ExtensionRegisterRequest {
                    kind: ExtensionKind::Tool,
                    implementation_id: "sdk-tool-alias".to_string(),
                    descriptor: tool_descriptor("search_docs"),
                    timeout: None,
                })
                .block_task()
                .await;
            assert!(
                semantic_conflict.is_err(),
                "a second implementation cannot claim the same tool name"
            );

            // Standard session/prompt flows through the extension tool.
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(
                    directory
                        .path()
                        .canonicalize()
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                ))
                .block_task()
                .await?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id.clone(),
                    vec![v1::ContentBlock::Text(v1::TextContent::new(
                        "search bridges",
                    ))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);

            // Unregister: idempotent release.
            assert!(unregister(&connection, &tool).await?);
            assert!(!unregister(&connection, &tool).await?);
            assert!(unregister(&connection, &callback).await?);
            assert!(unregister(&connection, &intervention).await?);
            assert!(unregister(&connection, &hook).await?);

            // A stale-generation handle fails with the typed ladder. The
            // fresh state root is at generation 1, so generation 0 is stale.
            let stale = WireHandle {
                id: tool.id.clone(),
                generation: WireU64::from_u64(0),
                kind: HandleKind::Extension,
            };
            let stale_result = connection
                .send_request(ExtensionUnregisterRequest { extension: stale })
                .block_task()
                .await;
            assert!(stale_result.is_err());
            Ok(())
        })
    })
    .await;

    assert!(outcome.is_ok(), "scenario failed: {outcome:?}");
    let operations = dispatch
        .operations
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    assert!(
        operations
            .iter()
            .any(|operation| operation == "tool_execute"),
        "the tool call must reach the SDK; got {operations:?}"
    );
    assert!(
        operations
            .iter()
            .any(|operation| operation.starts_with("callback_on_")),
        "observational callbacks must reach the SDK; got {operations:?}"
    );
    assert!(
        operations
            .iter()
            .any(|operation| operation == "intervention_on_tool_call"),
        "the intervention must reach the SDK; got {operations:?}"
    );
    assert!(
        operations.iter().any(|operation| operation == "hook_run"),
        "lifecycle hooks must reach the SDK; got {operations:?}"
    );
    let stderr = stderr_text(&host);
    assert!(
        !stderr.contains(SENTINEL_SECRET),
        "the credential must never reach stderr"
    );
    host.child.kill().await?;
    Ok(())
}

/// ChannelPlugin and MessageHandler registrations use the same connection
/// owned extension authority as the other reverse traits. The facade manager
/// owns the framework resource, while plugin lifecycle and send operations
/// cross the typed bridge with no local fallback.
#[cfg(feature = "framework-channels")]
#[tokio::test]
async fn channel_plugin_facade_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());

    drive_sdk(&mut host, dispatch, move |connection| {
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let handler = register_extension(
                &connection,
                ExtensionKind::ChannelMessageHandler,
                "sdk-channel-handler",
                channel_handler_descriptor("sdk-channel-handler"),
                None,
            )
            .await?;
            let plugin = register_extension(
                &connection,
                ExtensionKind::ChannelPlugin,
                "sdk-channel-plugin",
                channel_plugin_descriptor("sdk-channel", "sdk-channel-handler"),
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent,
                    working_dir: None,
                    session_id: None,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            let request = |operation: &str, arguments: Vec<WireValue>| FeatureOperationRequest {
                operation: operation.to_string(),
                signature_digest: echo_sdk_protocol::facade::family_operation_signature_digest(
                    "channels", operation,
                ),
                handle: Some(session.session.clone()),
                arguments,
            };
            let to_wire = |value: serde_json::Value| {
                WireValue::from_json(value)
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))
            };
            let invoke = |request: FeatureOperationRequest| async {
                let value = connection
                    .send_request(UntypedMessage::new(
                        "_echo_agent/channels/op",
                        serde_json::to_value(request)
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    )?)
                    .block_task()
                    .await?;
                let response: FeatureOperationResponse = serde_json::from_value(value)
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                response
                    .value
                    .into_json()
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))
            };
            let opened = invoke(request("channels.manager.open", Vec::new())).await?;
            let resource: WireHandle =
                serde_json::from_value(opened.get("resource").cloned().ok_or_else(|| {
                    RpcError::internal_error().data("channel manager did not return a resource")
                })?)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let registered =
                invoke(request(
                    "channels.plugin.register",
                    vec![
                        to_wire(serde_json::to_value(&resource).map_err(|error| {
                            RpcError::internal_error().data(error.to_string())
                        })?)?,
                        to_wire(serde_json::to_value(&plugin).map_err(|error| {
                            RpcError::internal_error().data(error.to_string())
                        })?)?,
                        to_wire(serde_json::to_value(&handler).map_err(|error| {
                            RpcError::internal_error().data(error.to_string())
                        })?)?,
                    ],
                ))
                .await?;
            assert_eq!(registered.get("registered"), Some(&serde_json::json!(true)));
            let started =
                invoke(request(
                    "channels.manager.start",
                    vec![
                        to_wire(serde_json::to_value(&resource).map_err(|error| {
                            RpcError::internal_error().data(error.to_string())
                        })?)?,
                        to_wire(serde_json::to_value(&handler).map_err(|error| {
                            RpcError::internal_error().data(error.to_string())
                        })?)?,
                    ],
                ))
                .await?;
            assert_eq!(started.get("channels"), Some(&serde_json::json!(1)));
            assert_eq!(started.get("failures"), Some(&serde_json::json!(0)));
            let listed = invoke(request(
                "channels.manager.list",
                vec![to_wire(serde_json::to_value(&resource).map_err(
                    |error| RpcError::internal_error().data(error.to_string()),
                )?)?],
            ))
            .await?;
            assert_eq!(
                listed.get("channels"),
                Some(&serde_json::json!(["sdk-channel"]))
            );
            let healthy = invoke(request(
                "channels.manager.health",
                vec![
                    to_wire(
                        serde_json::to_value(&resource)
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    )?,
                    WireValue::String("sdk-channel".to_string()),
                ],
            ))
            .await?;
            assert_eq!(healthy.get("healthy"), Some(&serde_json::json!(true)));
            let sent = invoke(request(
                "channels.plugin.send",
                vec![
                    to_wire(
                        serde_json::to_value(&resource)
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    )?,
                    WireValue::String("sdk-channel".to_string()),
                    to_wire(
                        serde_json::to_value(channel_outbound_fixture("hello"))
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    )?,
                ],
            ))
            .await?;
            assert_eq!(sent.get("sent"), Some(&serde_json::json!(true)));
            let stopped = invoke(request(
                "channels.manager.stop",
                vec![
                    to_wire(
                        serde_json::to_value(&resource)
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    )?,
                    WireValue::String("sdk-channel".to_string()),
                ],
            ))
            .await?;
            assert_eq!(stopped.get("stopped"), Some(&serde_json::json!(true)));
            Ok(())
        })
    })
    .await?;
    let _ = host.child.kill().await;
    Ok(())
}

/// A bridge-only Host must reject channel registrations because it has no
/// compiled framework channel authority to consume them.
#[cfg(not(feature = "framework-channels"))]
#[tokio::test]
async fn channel_extension_requires_framework_feature() -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    drive_sdk(
        &mut host,
        Arc::new(SdkDispatch::default()),
        move |connection| {
            Box::pin(async move {
                connection
                    .send_request(initialize_request(Some(client_hello())))
                    .block_task()
                    .await?;
                let result = register_extension(
                    &connection,
                    ExtensionKind::ChannelMessageHandler,
                    "sdk-channel-handler",
                    channel_handler_descriptor("sdk-channel-handler"),
                    None,
                )
                .await;
                assert!(
                    result.is_err(),
                    "channel registration must require the feature"
                );
                Ok(())
            })
        },
    )
    .await?;
    let _ = host.child.kill().await;
    Ok(())
}

/// Scenario A2: memory and human-loop providers are real reverse extensions,
/// not aliases of the Tool or Intervention callback contracts.
#[tokio::test]
async fn store_and_human_loop_extensions_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![
        tool_call_script("recall", r#"{"query":"bridge memory"}"#),
        final_script("remembered"),
    ])
    .await?;
    let config = write_config_with_features(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
        true,
        false,
        false,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let store = register_extension(
                &connection,
                ExtensionKind::Store,
                "sdk-store",
                ExtensionDescriptor::Store {
                    descriptor_version: 1,
                    search_modes: vec![SearchModeWire::Keyword],
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent,
                    working_dir: Some(WirePath::Utf8 {
                        path: session_dir.display().to_string(),
                    }),
                    session_id: None,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            let request = FeatureOperationRequest {
                operation: "memory.store.put".to_string(),
                signature_digest: echo_sdk_protocol::facade::family_operation_signature_digest(
                    "memory",
                    "memory.store.put",
                ),
                handle: Some(session.session.clone()),
                arguments: vec![
                    WireValue::List(vec![WireValue::String("raw-kv".to_string())]),
                    WireValue::String("fixture".to_string()),
                    WireValue::String("bridge memory".to_string()),
                ],
            };
            connection
                .send_request(UntypedMessage::new(
                    "_echo_agent/memory/op",
                    serde_json::to_value(request)
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                )?)
                .block_task()
                .await?;
            let got = connection
                .send_request(UntypedMessage::new(
                    "_echo_agent/memory/op",
                    serde_json::to_value(FeatureOperationRequest {
                        operation: "memory.store.get".to_string(),
                        signature_digest:
                            echo_sdk_protocol::facade::family_operation_signature_digest(
                                "memory",
                                "memory.store.get",
                            ),
                        handle: Some(session.session.clone()),
                        arguments: vec![
                            WireValue::List(vec![WireValue::String("raw-kv".to_string())]),
                            WireValue::String("fixture".to_string()),
                        ],
                    })
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                )?)
                .block_task()
                .await?;
            let got: FeatureOperationResponse = serde_json::from_value(got)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            assert!(
                got.value
                    .into_json()
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?
                    .to_string()
                    .contains("bridge memory"),
                "explicit raw put/get must use the same registered Store"
            );
            let acp_session_id = v1::SessionId::new(session.acp_session_id);
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    acp_session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new(
                        "remember this",
                    ))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            assert!(unregister(&connection, &store).await?);
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "store scenario failed: {outcome:?}; stderr={}",
        stderr_text(&host)
    );
    assert!(
        dispatch
            .operations
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .iter()
            .any(|operation| operation == "store_put")
    );
    host.child.kill().await?;

    let model = start_scripted_model(vec![
        tool_call_script(
            "human_in_loop",
            r#"{"reasoning":"need confirmation","approval_type":"LLM"}"#,
        ),
        final_script("confirmed"),
    ])
    .await?;
    let config = write_config_with_features(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
        false,
        true,
        false,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let human = register_extension(
                &connection,
                ExtensionKind::HumanLoopProvider,
                "sdk-human",
                ExtensionDescriptor::HumanLoopProvider {
                    descriptor_version: 1,
                },
                None,
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new("ask"))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            assert!(unregister(&connection, &human).await?);
            Ok(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "human-loop scenario failed: {outcome:?}");
    assert!(
        dispatch
            .operations
            .lock()
            .expect("operations")
            .iter()
            .any(|operation| operation == "human_loop_request")
    );
    host.child.kill().await?;
    Ok(())
}

/// Scenario A3: an AgentFactory creates a correctly typed CustomAgent handle,
/// then the regular agent dispatch tool invokes that instance.
#[tokio::test]
async fn factory_and_custom_agent_extensions_round_trip() -> Result<(), Box<dyn std::error::Error>>
{
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![
        tool_call_script(
            "agent_tool",
            r#"{"agent_name":"sdk-factory","task":"delegate first"}"#,
        ),
        final_script("delegated first"),
        tool_call_script(
            "agent_tool",
            r#"{"agent_name":"sdk-factory","task":"delegate second"}"#,
        ),
        final_script("delegated second"),
    ])
    .await?;
    let config = write_config_with_features(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
        false,
        false,
        true,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let factory = register_extension(
                &connection,
                ExtensionKind::AgentFactory,
                "sdk-factory",
                ExtensionDescriptor::AgentFactory {
                    descriptor_version: 1,
                },
                None,
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id.clone(),
                    vec![v1::ContentBlock::Text(v1::TextContent::new("delegate"))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new(
                        "delegate again",
                    ))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            assert!(unregister(&connection, &factory).await?);
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "factory scenario failed: {outcome:?}; operations={:?}",
        dispatch.operations.lock().expect("operations")
    );
    let operations = dispatch.operations.lock().expect("operations").clone();
    assert!(
        operations
            .iter()
            .filter(|operation| operation.as_str() == "factory_create_agent")
            .count()
            >= 2,
        "operations={operations:?}"
    );
    assert!(
        operations
            .iter()
            .filter(|operation| {
                operation.as_str() == "agent_execute"
                    || operation.as_str() == "agent_execute_stream"
            })
            .count()
            >= 2,
        "operations={operations:?}"
    );
    assert!(
        operations
            .iter()
            .filter(|operation| operation.as_str() == "agent_close")
            .count()
            >= 2,
        "factory instances must close before dispatch completes: {operations:?}"
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn cancelled_factory_stream_closes_and_releases_instance()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![tool_call_script(
        "agent_tool",
        r#"{"agent_name":"sdk-factory","task":"hang then cancel"}"#,
    )])
    .await?;
    let config = write_config_with_features(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
        false,
        false,
        true,
    );
    set_profile_limit(&config, "max_registered_extensions", 2)?;
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.omit_stream_terminal.store(true, Ordering::Release);
    dispatch.hang.lock().expect("hang lock").push("agent_close");
    let operations = dispatch.operations.clone();
    let scenario_dispatch = dispatch.clone();
    let closed = dispatch.factory_closed.clone();
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let operations = operations.clone();
        let dispatch = scenario_dispatch.clone();
        let closed = closed.clone();
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            register_extension(
                &connection,
                ExtensionKind::AgentFactory,
                "sdk-factory",
                ExtensionDescriptor::AgentFactory {
                    descriptor_version: 1,
                },
                Some(WireDuration {
                    seconds: WireU64::from_u64(1),
                    nanos: 0,
                }),
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir.clone()))
                .block_task()
                .await?;
            let prompt = connection.send_request(v1::PromptRequest::new(
                session.session_id.clone(),
                vec![v1::ContentBlock::Text(v1::TextContent::new("delegate"))],
            ));
            for _ in 0..100 {
                if operations
                    .lock()
                    .expect("operations lock")
                    .iter()
                    .any(|operation| operation == "agent_execute_stream")
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            assert!(
                operations
                    .lock()
                    .expect("operations lock")
                    .iter()
                    .any(|operation| operation == "agent_execute_stream"),
                "factory stream did not start"
            );

            // Construct another Session while the factory instance is live.
            // Session construction must see only direct registrations.
            let _second: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            connection.send_notification(v1::CancelNotification::new(session.session_id))?;
            let producer = wait_fixture_notice(
                &dispatch,
                ExtensionOperation::AgentExecuteStream,
                "cancelled",
            )
            .await?;
            finish_fixture_stream(&dispatch, &producer).await?;
            let _ = tokio::time::timeout(Duration::from_secs(5), prompt.block_task()).await;

            // A timeout is not proof of remote cleanup. The child retains
            // its quota until the same bound identity acknowledges close.
            tokio::time::sleep(Duration::from_millis(1_500)).await;
            let rejected = register_extension(
                &connection,
                ExtensionKind::Tool,
                "cleanup-probe",
                tool_descriptor("cleanup_probe"),
                None,
            )
            .await;
            let is_quota_pending = |error: &RpcError| {
                EchoSdkError::from_jsonrpc_data(error.data.as_ref()).is_ok_and(|typed| {
                    typed.code == ExtensionErrorCode::PayloadTooLarge
                        && typed.retryable == Retryability::AfterDelay
                        && typed.operation.as_deref() == Some("_echo_agent/extension/register")
                        && typed.message == "registered extension limit 2 reached"
                })
            };
            let error = rejected.err().ok_or_else(|| {
                RpcError::internal_error()
                    .data("unacknowledged remote cleanup released its owned quota")
            })?;
            assert!(
                is_quota_pending(&error),
                "expected exact held-child quota rejection: {error:?}"
            );
            let close_id =
                wait_fixture_notice(&dispatch, ExtensionOperation::AgentClose, "timeout").await?;
            let original =
                take_fixture_invocation(&dispatch, &close_id, ExtensionOperation::AgentClose)?;
            let close_outcome = complete_fixture_agent_close(
                &original.call,
                &dispatch.factory_instances,
                &dispatch.factory_closed,
            )
            .map_err(|error| RpcError::internal_error().data(error.message))?;
            let original_child = original.call.extension.id.clone();
            original.respond(close_outcome)?;
            assert_eq!(
                closed
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .iter()
                    .filter(|child| *child == &original_child)
                    .count(),
                1
            );
            assert_eq!(
                operations
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .iter()
                    .filter(|operation| operation.as_str() == "agent_close")
                    .count(),
                1,
                "the held original close must not be replaced by another callback"
            );
            // The SDK callback marker precedes the actual close response.
            // Observe the Host's quota state under the same original 10s budget.
            // register_extension_owned rejects this exact limit before publishing
            // a handle, so only that typed, side-effect-free condition may retry.
            let mut last_quota_error = None;
            let probe = tokio::time::timeout(Duration::from_secs(10), async {
                while closed
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .is_empty()
                {
                    tokio::task::yield_now().await;
                }
                loop {
                    match register_extension(
                        &connection,
                        ExtensionKind::Tool,
                        "cleanup-probe",
                        tool_descriptor("cleanup_probe"),
                        None,
                    )
                    .await
                    {
                        Ok(probe) => return Ok(probe),
                        Err(error) if is_quota_pending(&error) => {
                            last_quota_error = Some(error.to_string());
                            tokio::task::yield_now().await;
                        }
                        Err(error) => return Err(error),
                    }
                }
            })
            .await
            .map_err(|error| {
                RpcError::internal_error().data(format!(
                    "original child quota did not settle: {error}; last={last_quota_error:?}"
                ))
            })??;
            assert!(unregister(&connection, &probe).await?);
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "factory cancellation scenario failed: {outcome:?}"
    );
    let operations = dispatch.operations.lock().expect("operations").clone();
    assert!(
        operations
            .iter()
            .any(|operation| operation == "agent_close")
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn directly_registered_custom_agent_round_trips_and_unregisters()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![
        tool_call_script(
            "agent_tool",
            r#"{"agent_name":"sdk-custom","task":"delegate this"}"#,
        ),
        final_script("delegated"),
    ])
    .await?;
    let config = write_config_with_features(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
        false,
        false,
        true,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let custom = register_extension(
                &connection,
                ExtensionKind::CustomAgent,
                "sdk-custom-implementation",
                ExtensionDescriptor::CustomAgent {
                    descriptor_version: 1,
                    name: "sdk-custom".to_string(),
                    model_name: "sdk-custom-model".to_string(),
                    system_prompt: "SDK custom agent".to_string(),
                    tool_names: Vec::new(),
                },
                None,
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new("delegate"))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            assert!(unregister(&connection, &custom).await?);
            Ok(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "custom Agent scenario failed: {outcome:?}");
    let operations = dispatch.operations.lock().expect("operations").clone();
    assert!(
        operations
            .iter()
            .any(|operation| operation == "agent_execute" || operation == "agent_execute_stream")
    );
    host.child.kill().await?;
    Ok(())
}

/// Critic registration is connection-owned and injects a typed proxy into
/// newly created Agents. Host defaults keep verifier_enabled=false, so a
/// normal prompt must remain fail-open without silently invoking the callback;
/// the protocol contract test covers the Critique DTO shape independently.
#[tokio::test]
async fn critic_registration_preserves_default_verifier_disabled_behavior()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("answer")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let critic = register_extension(
                &connection,
                ExtensionKind::Critic,
                "sdk-critic",
                critic_descriptor("sdk-critic"),
                None,
            )
            .await?;
            let dto = CritiqueInput {
                task: "solve".to_string(),
                answer: "42".to_string(),
                context: "math".to_string(),
            };
            let invocation = ExtensionInvocation::CriticCritique(dto.clone());
            assert_eq!(invocation.operation(), ExtensionOperation::CriticCritique);
            let round_trip: CritiqueInput = serde_json::from_value(serde_json::to_value(dto)?)?;
            assert_eq!(round_trip.task, "solve");
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new("solve"))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            assert!(unregister(&connection, &critic).await?);
            Ok::<_, RpcError>(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "critic scenario failed: {outcome:?}");
    let operations = dispatch.operations.lock().expect("operations").clone();
    assert!(
        !operations
            .iter()
            .any(|operation| operation == "critic_critique"),
        "default verifier disabled must not invoke the registered Critic: {operations:?}"
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn context_compressor_registration_drives_session_compression()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let first_compressor = register_extension(
                &connection,
                ExtensionKind::ContextCompressor,
                "sdk-context-compressor-first",
                ExtensionDescriptor::ContextCompressor {
                    descriptor_version: 1,
                    name: "sdk-context-compressor-first".to_string(),
                },
                None,
            )
            .await?;
            let second_compressor = register_extension(
                &connection,
                ExtensionKind::ContextCompressor,
                "sdk-context-compressor-second",
                ExtensionDescriptor::ContextCompressor {
                    descriptor_version: 1,
                    name: "sdk-context-compressor-second".to_string(),
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let first_session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: None,
                    session_id: None,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            let second_session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: None,
                    session_id: None,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            let bind_operation = "echo_agent::agent::react::ReactAgent::set_compressor";
            let bound = invoke_source_operation(
                &connection,
                agent.clone(),
                bind_operation,
                vec![
                    WireValue::Handle(first_session.session.clone()),
                    WireValue::Handle(first_compressor.clone()),
                ],
            )
            .await?;
            assert_eq!(bound.value, WireValue::Null);
            let operation =
                "echo_agent::agent::react::ReactAgent::force_compress_with_focus_and_hooks";
            for session in [first_session.session, second_session.session] {
                let response = invoke_source_operation(
                    &connection,
                    agent.clone(),
                    operation,
                    vec![
                        WireValue::Handle(session),
                        WireValue::String("preserve decisions".to_string()),
                        WireValue::U64(WireU64::from_u64(64)),
                        WireValue::String("manual".to_string()),
                    ],
                )
                .await?;
                assert!(matches!(response.value, WireValue::Map(_)));
            }
            assert!(unregister(&connection, &first_compressor).await?);
            assert!(unregister(&connection, &second_compressor).await?);
            Ok::<_, RpcError>(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "compressor scenario failed: {outcome:?}");
    let operations = dispatch.operations.lock().expect("operations").clone();
    assert!(
        operations
            .iter()
            .any(|operation| operation == "compressor_compress"),
        "custom compressor was not invoked: {operations:?}"
    );
    let compressor_extensions = dispatch
        .compressor_extensions
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    assert_eq!(
        compressor_extensions.len(),
        2,
        "both Session compressors must be invoked: {compressor_extensions:?}"
    );
    assert_ne!(
        compressor_extensions.first(),
        compressor_extensions.get(1),
        "explicit Session binding must override the default latest registration"
    );
    assert_eq!(
        dispatch.tokenizer_counts.load(Ordering::Acquire),
        2,
        "each compressor callback must reach its Host-owned tokenizer"
    );
    host.child.kill().await?;
    Ok(())
}

#[cfg(all(feature = "framework-web", feature = "framework-shell"))]
#[tokio::test]
async fn agent_components_are_consumed_by_new_session_agents()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model_with_delay(
        vec![
            vec![serde_json::json!({"invalid": true})],
            final_script("checkpoint resume answer"),
            tool_call_script("web_search", r#"{"query":"bridge"}"#),
            final_script("search component answer"),
            tool_call_script("shell", r#"{"command":"printf sandbox"}"#),
            final_script("sandbox component answer"),
        ],
        Duration::from_millis(10),
    )
    .await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.managed.enabled.store(true, Ordering::Release);
    let reentrant_mutation = dispatch.reentrant_mutation.clone();
    let session_dir = directory.path().to_path_buf();
    let skill_dir = directory.path().join("policy-skills");
    for name in ["allowed", "blocked"] {
        let directory = skill_dir.join(name);
        std::fs::create_dir_all(&directory)?;
        std::fs::write(
            directory.join("SKILL.md"),
            format!("---\nname: {name}\ndescription: {name} skill\n---\nbody"),
        )?;
    }
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let session_dir = session_dir.clone();
        let skill_dir = skill_dir.clone();
        let reentrant_mutation = reentrant_mutation.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let audit = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-audit",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::AuditLogger,
                    name: "sdk-audit".to_string(),
                    capabilities: AgentComponentCapabilitiesWire::default(),
                },
                None,
            )
            .await?;
            let conversation = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-conversation",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::ConversationStore,
                    name: "sdk-conversation".to_string(),
                    capabilities: AgentComponentCapabilitiesWire {
                        conversation_projection: ConversationProjectionCapabilityWire::AtomicV1,
                        persistence_call: PersistenceCallCapabilityWire::AbsoluteDeadlineV1,
                        ..Default::default()
                    },
                },
                None,
            )
            .await?;
            let run_store = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-run-store",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::RunStore,
                    name: "sdk-run-store".to_string(),
                    capabilities: AgentComponentCapabilitiesWire::default(),
                },
                None,
            )
            .await?;
            let projector = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-projector",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::ContextProjector,
                    name: "sdk-projector".to_string(),
                    capabilities: AgentComponentCapabilitiesWire::default(),
                },
                None,
            )
            .await?;
            let guard = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-guard",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::Guard,
                    name: "sdk-guard".to_string(),
                    capabilities: AgentComponentCapabilitiesWire::default(),
                },
                None,
            )
            .await?;
            let search = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-search",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::SearchProvider,
                    name: "sdk-search".to_string(),
                    capabilities: AgentComponentCapabilitiesWire::default(),
                },
                None,
            )
            .await?;
            let sandbox = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-sandbox",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::SandboxExecutor,
                    name: "sdk-sandbox".to_string(),
                    capabilities: AgentComponentCapabilitiesWire {
                        isolation_level: Some("process".to_string()),
                        ..AgentComponentCapabilitiesWire::default()
                    },
                },
                None,
            )
            .await?;
            let intent = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-intent",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::IntentClassifier,
                    name: "sdk-intent".to_string(),
                    capabilities: AgentComponentCapabilitiesWire::default(),
                },
                None,
            )
            .await?;
            let skill_policy = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-skill-policy",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::SkillLoadPolicy,
                    name: "sdk-skill-policy".to_string(),
                    capabilities: AgentComponentCapabilitiesWire::default(),
                },
                None,
            )
            .await?;
            let checkpoint_store = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-workflow-checkpoints",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::WorkflowCheckpointStore,
                    name: "sdk-workflow-checkpoints".to_string(),
                    capabilities: AgentComponentCapabilitiesWire {
                        protected_claim_v1: true,
                        claim_heartbeat_interval_ms: Some(WireU64::from_u64(1)),
                        ..AgentComponentCapabilitiesWire::default()
                    },
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: Some("agent-component-reentry-agent".to_string()),
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: Some(WirePath::Utf8 {
                        path: session_dir.display().to_string(),
                    }),
                    session_id: None,
                    idempotency_id: Some("agent-component-reentry-session".to_string()),
                })
                .block_task()
                .await?;
            *reentrant_mutation
                .lock()
                .unwrap_or_else(|error| error.into_inner()) =
                Some((agent, session.session.clone()));
            let agent_handle = reentrant_mutation
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .as_ref()
                .map(|(agent, _)| agent.clone())
                .ok_or_else(|| RpcError::internal_error().data("missing Agent handle"))?;
            let workflow_call = |operation: &str, arguments: Vec<WireValue>| {
                UntypedMessage::new(
                    "_echo_agent/workflow/op",
                    serde_json::to_value(FeatureOperationRequest {
                        operation: operation.to_string(),
                        signature_digest:
                            echo_sdk_protocol::facade::family_operation_signature_digest(
                                "workflow",
                                operation,
                            ),
                        handle: Some(session.session.clone()),
                        arguments,
                    })
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                )
            };
            let definition = serde_json::json!({
                "name": "remote_checkpoint_settlement",
                "nodes": [{
                    "name": "work",
                    "type": "agent",
                    "system_prompt": "return a short answer",
                    "input_key": "task",
                    "output_key": "answer"
                }],
                "edges": [],
                "entry": "work",
                "finish": ["work"],
                "interrupt_before": ["work"]
            })
            .to_string();
            let built = connection
                .send_request(workflow_call(
                    "workflow.graph.build",
                    vec![WireValue::String(definition)],
                )?)
                .block_task()
                .await?;
            let built: FeatureOperationResponse = serde_json::from_value(built)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let built = built
                .value
                .into_json()
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let graph: WireHandle = serde_json::from_value(
                built
                    .get("resource")
                    .cloned()
                    .ok_or_else(|| RpcError::internal_error().data("missing graph resource"))?,
            )
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let graph_argument = WireValue::from_json(
                serde_json::to_value(&graph)
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
            )
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let interrupted = connection
                .send_request(workflow_call(
                    "workflow.graph.run_until_interrupt",
                    vec![
                        graph_argument.clone(),
                        WireValue::from_json(serde_json::json!({"task": "checkpoint"}))
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    ],
                )?)
                .block_task()
                .await?;
            let interrupted: FeatureOperationResponse = serde_json::from_value(interrupted)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let interrupted = interrupted
                .value
                .into_json()
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let checkpoint_id = interrupted
                .get("checkpoint")
                .and_then(|checkpoint| checkpoint.get("id"))
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| RpcError::internal_error().data("missing checkpoint id"))?
                .to_string();
            connection
                .send_request(workflow_call(
                    "workflow.graph.tag_checkpoint",
                    vec![
                        graph_argument.clone(),
                        WireValue::String(checkpoint_id.clone()),
                        WireValue::String("remote".to_string()),
                        WireValue::List(vec![WireValue::String("sdk".to_string())]),
                    ],
                )?)
                .block_task()
                .await?;
            let first_resume = connection
                .send_request(workflow_call(
                    "workflow.graph.resume",
                    vec![
                        graph_argument.clone(),
                        WireValue::String(checkpoint_id.clone()),
                        WireValue::String("approve".to_string()),
                    ],
                )?)
                .block_task()
                .await;
            assert!(
                first_resume.is_err(),
                "malformed provider response must fail and requeue the remote claim"
            );
            let resumed = connection
                .send_request(workflow_call(
                    "workflow.graph.resume",
                    vec![
                        graph_argument,
                        WireValue::String(checkpoint_id),
                        WireValue::String("approve".to_string()),
                    ],
                )?)
                .block_task()
                .await?;
            let resumed: FeatureOperationResponse = serde_json::from_value(resumed)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            assert!(resumed
                .value
                .into_json()
                .is_ok_and(|value| value.get("outcome") == Some(&serde_json::json!("completed"))));
            let tool_names = invoke_source_operation(
                &connection,
                agent_handle.clone(),
                "echo_core::agent::Agent::tool_names",
                vec![WireValue::Handle(session.session.clone())],
            )
            .await?;
            let WireValue::List(tool_names) = tool_names.value else {
                return Err(RpcError::internal_error().data("tool_names was not a list"));
            };
            for expected in ["web_search", "shell"] {
                assert!(
                    tool_names
                        .iter()
                        .any(|value| matches!(value, WireValue::String(name) if name == expected)),
                    "{expected} tool was not installed: {tool_names:?}"
                );
            }
            invoke_source_operation(
                &connection,
                agent_handle.clone(),
                "echo_agent::agent::react::ReactAgent::set_permission_mode",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::String("full-auto".to_string()),
                ],
            )
            .await?;
            let discovered = invoke_source_operation(
                &connection,
                agent_handle.clone(),
                "echo_agent::agent::react::ReactAgent::discover_skills",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::List(vec![WireValue::Path(WirePath::Utf8 {
                        path: skill_dir.display().to_string(),
                    })]),
                ],
            )
            .await?;
            assert!(
                matches!(discovered.value, WireValue::List(ref values)
                    if values == &vec![WireValue::String("allowed".to_string())]),
                "SkillLoadPolicy did not filter discovery: {:?}",
                discovered.value
            );
            let reconciled = invoke_source_operation(
                &connection,
                agent_handle.clone(),
                "echo_agent::agent::react::ReactAgent::reconcile_skill_load_policy",
                vec![WireValue::Handle(session.session.clone())],
            )
            .await?;
            assert!(
                matches!(reconciled.value, WireValue::List(ref values) if values.is_empty()),
                "unchanged SkillLoadPolicy should not remove allowed skills: {:?}",
                reconciled.value
            );
            let prompt_context = invoke_source_operation(
                &connection,
                agent_handle.clone(),
                "echo_execution::skills::external::prompt_exec::PromptContext",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::String(session_dir.display().to_string()),
                    WireValue::String(session.acp_session_id.clone()),
                    WireValue::List(Vec::new()),
                    WireValue::Null,
                    WireValue::Duration(WireDuration::from_nanos(30_000_000_000)),
                    WireValue::String("local".to_string()),
                    WireValue::Handle(sandbox.clone()),
                ],
            )
            .await?;
            let WireValue::Handle(prompt_context) = prompt_context.value else {
                return Err(RpcError::internal_error().data("no PromptContext resource"));
            };
            let rendered = invoke_source_operation(
                &connection,
                agent_handle,
                "echo_execution::skills::external::prompt_exec::process_skill_content",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::String("Version: !`echo test`".to_string()),
                    WireValue::Handle(prompt_context),
                ],
            )
            .await?;
            assert!(
                matches!(rendered.value, WireValue::String(ref value) if value.contains("sandbox answer"))
            );
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    v1::SessionId::new(session.acp_session_id.clone()),
                    vec![v1::ContentBlock::Text(v1::TextContent::new(
                        "use components",
                    ))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    v1::SessionId::new(session.acp_session_id),
                    vec![v1::ContentBlock::Text(v1::TextContent::new(
                        "use the shell",
                    ))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            assert!(unregister(&connection, &audit).await?);
            assert!(unregister(&connection, &conversation).await?);
            assert!(unregister(&connection, &run_store).await?);
            assert!(unregister(&connection, &projector).await?);
            assert!(unregister(&connection, &guard).await?);
            assert!(unregister(&connection, &search).await?);
            assert!(unregister(&connection, &sandbox).await?);
            assert!(unregister(&connection, &intent).await?);
            assert!(unregister(&connection, &skill_policy).await?);
            assert!(unregister(&connection, &checkpoint_store).await?);
            Ok::<_, RpcError>(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "component scenario failed: {outcome:?}; stderr: {}",
        stderr_text(&host)
    );
    assert!(
        dispatch.reentrant_conflicts.load(Ordering::Acquire) > 0,
        "same-Session callback mutation did not return extension_conflict"
    );
    let operations = dispatch
        .component_operations
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    assert!(
        operations
            .iter()
            .any(|operation| operation.contains("AuditLogger:AuditLog")),
        "AuditLogger was not consumed: {operations:?}"
    );
    assert!(
        operations
            .iter()
            .any(|operation| operation.contains("ContextProjector:ContextProject")),
        "ContextProjector was not consumed: {operations:?}"
    );
    for expected in [
        "Guard:GuardCheck",
        "SearchProvider:SearchProviderSearch",
        "SandboxExecutor:SandboxExecute",
        "IntentClassifier:IntentClassify",
        "SkillLoadPolicy:SkillLoadAllows",
        "ConversationStore:ConversationEnsureProjectionEpochWithContext",
        "RunStore:RunAppendEvent",
        "WorkflowCheckpointStore:WorkflowCheckpointSave",
        "WorkflowCheckpointStore:WorkflowCheckpointSaveIfGeneration",
        "WorkflowCheckpointStore:WorkflowCheckpointClaim",
        "WorkflowCheckpointStore:WorkflowCheckpointProtectClaim",
        "WorkflowCheckpointStore:WorkflowCheckpointRenewClaim",
        "WorkflowCheckpointStore:WorkflowCheckpointRequeueClaim",
        "WorkflowCheckpointStore:WorkflowCheckpointAckClaim",
    ] {
        assert!(
            operations
                .iter()
                .any(|operation| operation.contains(expected)),
            "{expected} was not consumed: {operations:?}"
        );
    }
    assert!(
        dispatch.checkpoints.renewals.load(Ordering::Acquire) > 0,
        "remote checkpoint claim was not renewed: {operations:?}"
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn mcp_transport_is_closed_when_handle_publication_exceeds_quota()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    set_profile_limit(&config, "max_facade_resources", 1)?;
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let observed = dispatch.component_operations.clone();
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let observed = observed.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let transport = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-mcp-transport",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::McpTransport,
                    name: "sdk-mcp-transport".to_string(),
                    capabilities: AgentComponentCapabilitiesWire::default(),
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: Some("mcp-publication-agent".to_string()),
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: None,
                    session_id: None,
                    idempotency_id: Some("mcp-publication-session".to_string()),
                })
                .block_task()
                .await?;
            let family_call =
                |family: &str, method: &str, operation: &str, arguments: Vec<WireValue>| {
                    UntypedMessage::new(
                        method,
                        serde_json::to_value(FeatureOperationRequest {
                            operation: operation.to_string(),
                            signature_digest:
                                echo_sdk_protocol::facade::family_operation_signature_digest(
                                    family, operation,
                                ),
                            handle: Some(session.session.clone()),
                            arguments,
                        })
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    )
                };
            let state = connection
                .send_request(family_call(
                    "workflow",
                    "_echo_agent/workflow/op",
                    "workflow.state.new",
                    Vec::new(),
                )?)
                .block_task()
                .await?;
            let state: FeatureOperationResponse = serde_json::from_value(state)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let state = state
                .value
                .into_json()
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let state_resource: WireHandle =
                serde_json::from_value(state.get("resource").cloned().ok_or_else(|| {
                    RpcError::internal_error().data("workflow state resource missing")
                })?)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;

            let arguments = vec![
                WireValue::Handle(session.session.clone()),
                WireValue::String("quota-mcp".to_string()),
                WireValue::Handle(transport.clone()),
            ];
            let failed = invoke_source_operation(
                &connection,
                agent.clone(),
                "echo_integration::mcp::client::McpClient::from_transport",
                arguments.clone(),
            )
            .await;
            let Err(error) = failed else {
                return Err(RpcError::internal_error()
                    .data("MCP publication unexpectedly succeeded at the resource limit"));
            };
            let typed = EchoSdkError::from_jsonrpc_data(error.data.as_ref())
                .map_err(|decode| RpcError::internal_error().data(decode.to_string()))?;
            assert_eq!(typed.code, ExtensionErrorCode::PayloadTooLarge);

            assert!(
                observed
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .iter()
                    .all(|operation| !operation.starts_with("McpTransport:")),
                "quota admission must reject before constructing or opening a transport"
            );
            connection
                .send_request(family_call(
                    "invoke",
                    "_echo_agent/facade/invoke",
                    "facade.resource.close",
                    vec![WireValue::Handle(state_resource)],
                )?)
                .block_task()
                .await?;
            let reopened = invoke_source_operation(
                &connection,
                agent,
                "echo_integration::mcp::client::McpClient::from_transport",
                arguments,
            )
            .await?;
            let WireValue::Handle(client) = reopened.value else {
                return Err(RpcError::internal_error().data("MCP client resource missing"));
            };
            connection
                .send_request(family_call(
                    "invoke",
                    "_echo_agent/facade/invoke",
                    "facade.resource.close",
                    vec![WireValue::Handle(client)],
                )?)
                .block_task()
                .await?;
            assert!(unregister(&connection, &transport).await?);
            Ok::<_, RpcError>(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "MCP publication rollback scenario failed: {outcome:?}; stderr: {}",
        stderr_text(&host)
    );
    let operations = dispatch
        .component_operations
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    let closes = operations
        .iter()
        .filter(|operation| operation.contains("McpTransport:McpTransportClose"))
        .count();
    assert_eq!(
        closes, 1,
        "only the successfully admitted transport is opened and explicitly closed: {operations:?}"
    );
    host.child.kill().await?;
    Ok(())
}

#[cfg(feature = "framework-shell")]
#[tokio::test]
async fn sandbox_cancel_bridge_waits_for_cleanup_and_preserves_cancelled_terminal()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![tool_call_script(
        "run_code",
        r#"{"language":"python","code":"print(1)"}"#,
    )])
    .await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.hang_sandbox_cancel.store(true, Ordering::Release);
    dispatch
        .delayed_sandbox_cleanup
        .store(true, Ordering::Release);
    let component_operations = dispatch.component_operations.clone();
    let sandbox_cleanups_completed = dispatch.sandbox_cleanups_completed.clone();
    let silent = dispatch.silent.clone();
    let cancel_notices = dispatch.cancel_notices.clone();
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let component_operations = component_operations.clone();
        let sandbox_cleanups_completed = sandbox_cleanups_completed.clone();
        let silent = silent.clone();
        let cancel_notices = cancel_notices.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let sandbox = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-cancellable-sandbox",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::SandboxExecutor,
                    name: "sdk-cancellable-sandbox".to_string(),
                    capabilities: AgentComponentCapabilitiesWire {
                        isolation_level: Some("os-sandbox".to_string()),
                        ..AgentComponentCapabilitiesWire::default()
                    },
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: Some("sandbox-cancel-agent".to_string()),
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: Some(WirePath::Utf8 {
                        path: directory.path().display().to_string(),
                    }),
                    session_id: None,
                    idempotency_id: Some("sandbox-cancel-session".to_string()),
                })
                .block_task()
                .await?;
            invoke_source_operation(
                &connection,
                agent,
                "echo_agent::agent::react::ReactAgent::set_permission_mode",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::String("full-auto".to_string()),
                ],
            )
            .await?;
            let started = connection
                .send_request(RunStartRequest {
                    session: session.session,
                    input: RunInput::Chat {
                        text: "run the code".to_string(),
                    },
                    idempotency_id: Some("sandbox-cancel-run".to_string()),
                })
                .block_task()
                .await?;
            tokio::time::timeout(Duration::from_secs(10), async {
                loop {
                    if component_operations
                        .lock()
                        .unwrap_or_else(|error| error.into_inner())
                        .iter()
                        .filter(|operation| {
                            operation.contains("SandboxExecutor:SandboxExecuteWithLimitsAndCancel")
                        })
                        .count()
                        == 1
                        && silent
                            .lock()
                            .unwrap_or_else(|error| error.into_inner())
                            .len()
                            == 1
                    {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .map_err(|_| {
                let operations = component_operations
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .clone();
                RpcError::internal_error()
                    .data(format!("sandbox callback did not start: {operations:?}"))
            })?;
            connection
                .send_request(RunCancelRequest {
                    run: started.run.clone(),
                })
                .block_task()
                .await?;
            tokio::time::timeout(Duration::from_secs(10), async {
                loop {
                    let observed = cancel_notices
                        .lock()
                        .unwrap_or_else(|error| error.into_inner())
                        .iter()
                        .any(|notice| notice.ends_with(":cancelled"));
                    if observed {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .map_err(|_| RpcError::internal_error().data("sandbox cancel notice missing"))?;
            let held = connection
                .send_request(RunWaitRequest {
                    run: started.run.clone(),
                    timeout: Some(WireDuration::from_nanos(50_000_000)),
                })
                .block_task()
                .await?;
            assert!(!held.settled && held.terminal.is_none());
            assert_eq!(sandbox_cleanups_completed.load(Ordering::Acquire), 0);
            // Cancellation alone cannot finish this original callback. Its
            // actual peer response permits cleanup and the original receipt.
            let original = {
                let mut pending = silent.lock().unwrap_or_else(|error| error.into_inner());
                assert_eq!(pending.len(), 1);
                pending.pop().ok_or_else(|| {
                    RpcError::internal_error().data("original sandbox responder disappeared")
                })?
            };
            original.respond(ExtensionInvokeOutcome::Error {
                error: EchoSdkError::new(
                    ExtensionErrorCode::Cancelled,
                    "original sandbox callback completed cancellation",
                    Retryability::Never,
                ),
            })?;
            let waited = connection
                .send_request(RunWaitRequest {
                    run: started.run,
                    timeout: Some(WireDuration::from_nanos(10_000_000_000)),
                })
                .block_task()
                .await?;
            assert!(
                matches!(waited.terminal, Some(RunTerminal::Cancelled)),
                "sandbox cancellation must preserve the Run cancelled terminal: {:?}",
                waited.terminal
            );
            assert_eq!(
                sandbox_cleanups_completed.load(Ordering::Acquire),
                1,
                "Run terminal must follow the delayed cleanup side effect and its response"
            );
            assert!(
                component_operations
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .iter()
                    .any(|operation| operation.contains("SandboxExecutor:SandboxCleanup")),
                "sandbox cancellation must wait for cleanup"
            );
            assert!(unregister(&connection, &sandbox).await?);
            Ok::<_, RpcError>(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "sandbox cancellation scenario failed: {outcome:?}; stderr: {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

#[cfg(all(feature = "framework-eval", feature = "framework-improve"))]
#[tokio::test]
async fn eval_and_improve_source_adapters_invoke_agent_factory_lazily()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let workspace = directory.path().join("eval-workspace");
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let workspace = workspace.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let factory = register_extension(
                &connection,
                ExtensionKind::AgentFactory,
                "sdk-eval-factory",
                ExtensionDescriptor::AgentFactory {
                    descriptor_version: 1,
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: Some("eval-factory-agent".to_string()),
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: Some(WirePath::Utf8 {
                        path: workspace.display().to_string(),
                    }),
                    session_id: None,
                    idempotency_id: Some("eval-factory-session".to_string()),
                })
                .block_task()
                .await?;
            let cases = ["case-1", "case-2"]
                .into_iter()
                .map(|id| {
                    WireValue::from_json(serde_json::json!({
                        "id": id,
                        "name": id,
                        "description": "lazy factory",
                        "domain": null,
                        "task": "return SDK output",
                        "project_fixture": null,
                        "success_criteria": {
                            "type": "output_contains",
                            "substring": "SDK"
                        },
                        "constraints": {}
                    }))
                    .map_err(|error| RpcError::invalid_params().data(error.to_string()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let runner = invoke_source_operation(
                &connection,
                agent.clone(),
                "echo_agent::eval::runner::EvalRunner::new",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::Path(WirePath::Utf8 {
                        path: workspace.display().to_string(),
                    }),
                ],
            )
            .await?;
            let WireValue::Handle(runner) = runner.value else {
                return Err(RpcError::internal_error().data("no EvalRunner resource"));
            };
            let configured_runner = invoke_source_operation(
                &connection,
                agent.clone(),
                "echo_agent::eval::runner::EvalRunner::timeout_secs",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::Handle(runner),
                    WireValue::U64(WireU64::from_u64(7)),
                ],
            )
            .await?;
            let WireValue::Handle(runner) = configured_runner.value else {
                return Err(RpcError::internal_error().data("no configured EvalRunner"));
            };
            invoke_source_operation(
                &connection,
                agent.clone(),
                "echo_agent::eval::runner::EvalRunner::run_all",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::Handle(runner),
                    WireValue::List(cases.clone()),
                    WireValue::Handle(factory.clone()),
                ],
            )
            .await?;
            let improvement = invoke_source_operation(
                &connection,
                agent.clone(),
                "echo_agent::improve::loop::ImprovementLoop::new",
                vec![WireValue::Handle(session.session.clone())],
            )
            .await?;
            let WireValue::Handle(improvement) = improvement.value else {
                return Err(RpcError::internal_error().data("no ImprovementLoop resource"));
            };
            let configured_improvement = invoke_source_operation(
                &connection,
                agent.clone(),
                "echo_agent::improve::loop::ImprovementLoop::max_iterations",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::Handle(improvement),
                    WireValue::U64(WireU64::from_u64(1)),
                ],
            )
            .await?;
            let WireValue::Handle(improvement) = configured_improvement.value else {
                return Err(RpcError::internal_error().data("no configured ImprovementLoop"));
            };
            invoke_source_operation(
                &connection,
                agent,
                "echo_agent::improve::loop::ImprovementLoop::run",
                vec![
                    WireValue::Handle(session.session),
                    WireValue::Handle(improvement),
                    WireValue::List(cases),
                    WireValue::Handle(factory.clone()),
                    WireValue::Null,
                ],
            )
            .await?;
            assert!(unregister(&connection, &factory).await?);
            Ok::<_, RpcError>(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "lazy factory scenario failed: {outcome:?}");
    let operations = dispatch
        .operations
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    let factory_calls = operations
        .iter()
        .filter(|operation| operation.as_str() == "factory_create_agent")
        .count();
    assert_eq!(
        factory_calls, 4,
        "EvalRunner and early-stopped ImprovementLoop must construct exactly one Agent per executed case"
    );
    let created = dispatch
        .factory_created
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    let closed = dispatch
        .factory_closed
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    assert_eq!(created.len(), 4);
    assert_eq!(
        created
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        4,
        "each lazy constructor must receive its own preallocated identity"
    );
    assert_eq!(
        closed.iter().collect::<std::collections::BTreeSet<_>>(),
        created.iter().collect::<std::collections::BTreeSet<_>>(),
        "each created factory object must be closed through the same parent-bound identity"
    );
    assert!(
        dispatch
            .factory_instances
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .is_empty()
    );
    host.child.kill().await?;
    Ok(())
}

#[cfg(feature = "framework-eval")]
#[tokio::test]
async fn same_factory_constructs_independent_instances_across_concurrent_sessions()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let created = dispatch.factory_created.clone();
    let closed = dispatch.factory_closed.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let created = created.clone(); let closed = closed.clone();
        Box::pin(async move {
            connection.send_request(initialize_request(Some(client_hello()))).block_task().await?;
            let factory = register_extension(&connection, ExtensionKind::AgentFactory, "shared-factory", ExtensionDescriptor::AgentFactory { descriptor_version: 1 }, None).await?;
            let mut invocations = Vec::new();
            for id in ["first", "second"] {
                let agent = connection.send_request(AgentCreateRequest { config: AgentConfigWire::HostDefault, idempotency_id: None }).block_task().await?.agent;
                let session = connection.send_request(SessionCreateRequest { agent: agent.clone(), working_dir: None, session_id: None, idempotency_id: None }).block_task().await?;
                let runner = invoke_source_operation(&connection, agent.clone(), "echo_agent::eval::runner::EvalRunner::new", vec![WireValue::Handle(session.session.clone()), WireValue::Path(WirePath::Utf8 { path: std::env::temp_dir().join(id).display().to_string() })]).await?;
                let WireValue::Handle(runner) = runner.value else { return Err(RpcError::internal_error().data("no runner")); };
                let case = WireValue::from_json(serde_json::json!({"id":id,"name":"same descriptor","description":"independent factory states","domain":null,"task":"return SDK output","project_fixture":null,"success_criteria":{"type":"output_contains","substring":"SDK"},"constraints":{}})).map_err(|error| RpcError::invalid_params().data(error.to_string()))?;
                invocations.push((agent, vec![WireValue::Handle(session.session), WireValue::Handle(runner), WireValue::List(vec![case]), WireValue::Handle(factory.clone())]));
            }
            let first = invocations.first().cloned().ok_or_else(|| RpcError::internal_error().data("missing first runner"))?;
            let second = invocations.get(1).cloned().ok_or_else(|| RpcError::internal_error().data("missing second runner"))?;
            let (first, second) = tokio::join!(invoke_source_operation(&connection, first.0, "echo_agent::eval::runner::EvalRunner::run_all", first.1), invoke_source_operation(&connection, second.0, "echo_agent::eval::runner::EvalRunner::run_all", second.1));
            for result in [first?, second?] {
                let data = result.value.into_json().map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                assert!(!data.to_string().contains("extension_conflict"), "independent factory creations must not share the child execution lock; actual result: {data}");
            }
            let created = created.lock().unwrap_or_else(|error| error.into_inner()).clone();
            let closed = closed.lock().unwrap_or_else(|error| error.into_inner()).clone();
            assert_eq!(created.len(), 2); assert_eq!(created.iter().collect::<std::collections::BTreeSet<_>>().len(), 2);
            assert_eq!(closed.iter().collect::<std::collections::BTreeSet<_>>(), created.iter().collect::<std::collections::BTreeSet<_>>());
            assert!(unregister(&connection, &factory).await?);
            Ok(())
        })
    }).await;
    assert!(
        outcome.is_ok(),
        "parallel factory construction failed: {outcome:?}; {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

#[cfg(feature = "framework-shell")]
#[tokio::test]
async fn workflow_agent_component_stream_flows_through_facade_pull_stream()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let workflow = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-stream-workflow",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::Workflow,
                    name: "sdk-stream-workflow".to_string(),
                    capabilities: AgentComponentCapabilitiesWire {
                        supports_streaming: true,
                        ..AgentComponentCapabilitiesWire::default()
                    },
                },
                None,
            )
            .await?;
            let sandbox = register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "sdk-stream-sandbox",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::SandboxExecutor,
                    name: "sdk-stream-sandbox".to_string(),
                    capabilities: AgentComponentCapabilitiesWire {
                        isolation_level: Some("process".to_string()),
                        supports_streaming: true,
                        ..AgentComponentCapabilitiesWire::default()
                    },
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: Some("workflow-stream-agent".to_string()),
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent,
                    working_dir: None,
                    session_id: None,
                    idempotency_id: Some("workflow-stream-session".to_string()),
                })
                .block_task()
                .await?;
            let call = |operation: &str, arguments: Vec<WireValue>| {
                UntypedMessage::new(
                    "_echo_agent/workflow/op",
                    serde_json::to_value(FeatureOperationRequest {
                        operation: operation.to_string(),
                        signature_digest:
                            echo_sdk_protocol::facade::family_operation_signature_digest(
                                "workflow",
                                operation,
                            ),
                        handle: Some(session.session.clone()),
                        arguments,
                    })
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                )
            };
            let opened = connection
                .send_request(call(
                    "workflow.extension.run_stream",
                    vec![
                        WireValue::Handle(workflow.clone()),
                        WireValue::String("start".to_string()),
                    ],
                )?)
                .block_task()
                .await?;
            let opened: FeatureOperationResponse = serde_json::from_value(opened)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let WireValue::Handle(stream) = opened.value else {
                return Err(RpcError::internal_error().data("no workflow stream"));
            };
            for expected in ["item", "complete"] {
                let next = connection
                    .send_request(call(
                        "workflow.stream.next",
                        vec![WireValue::Handle(stream.clone())],
                    )?)
                    .block_task()
                    .await?;
                let next: FeatureOperationResponse = serde_json::from_value(next)
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                assert!(
                    matches!(next.value, WireValue::Variant { ref variant, .. } if variant == expected),
                    "workflow stream did not yield {expected}: {:?}",
                    next.value
                );
                if expected == "item" {
                    let WireValue::Variant { fields, .. } = &next.value else {
                        return Err(RpcError::internal_error().data("workflow item is not a variant"));
                    };
                    let Some(value) = fields
                        .iter()
                        .find(|field| field.name == "value")
                        .map(|field| &field.value)
                    else {
                        return Err(RpcError::internal_error().data("workflow item has no value"));
                    };
                    assert!(
                        matches!(value, WireValue::Variant { type_id, variant, .. }
                            if type_id == "echo_orchestration::workflow::WorkflowEvent"
                                && variant == "completed"),
                        "extension WorkflowEvent must use the canonical graph stream shape: {value:?}"
                    );
                }
            }
            let shell_call = |operation: &str, arguments: Vec<WireValue>| {
                UntypedMessage::new(
                    "_echo_agent/shell/op",
                    serde_json::to_value(FeatureOperationRequest {
                        operation: operation.to_string(),
                        signature_digest:
                            echo_sdk_protocol::facade::family_operation_signature_digest(
                                "shell",
                                operation.strip_prefix("shell.").unwrap_or(operation),
                            ),
                        handle: Some(session.session.clone()),
                        arguments,
                    })
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                )
            };
            let command = WireValue::from_json(serde_json::json!({
                "kind": {"Shell": "printf stream"},
                "minimum_isolation": null,
                "working_dir": null,
                "env": {},
                "timeout": {"secs": 30, "nanos": 0},
                "stdin": null
            }))
            .map_err(|error| RpcError::invalid_params().data(error.to_string()))?;
            let opened = connection
                .send_request(shell_call(
                    "shell.sandbox.extension.run_stream",
                    vec![WireValue::Handle(sandbox.clone()), command],
                )?)
                .block_task()
                .await?;
            let opened: FeatureOperationResponse = serde_json::from_value(opened)
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let WireValue::Handle(stream) = opened.value else {
                return Err(RpcError::internal_error().data("no sandbox stream"));
            };
            for expected in ["item", "complete"] {
                let next = connection
                    .send_request(shell_call(
                        "shell.sandbox.stream.next",
                        vec![WireValue::Handle(stream.clone())],
                    )?)
                    .block_task()
                    .await?;
                let next: FeatureOperationResponse = serde_json::from_value(next)
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                assert!(
                    matches!(next.value, WireValue::Variant { ref variant, .. } if variant == expected),
                    "sandbox stream did not yield {expected}: {:?}",
                    next.value
                );
            }
            assert!(unregister(&connection, &workflow).await?);
            assert!(unregister(&connection, &sandbox).await?);
            Ok::<_, RpcError>(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "workflow stream scenario failed: {outcome:?}"
    );
    assert!(
        dispatch
            .operations
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .iter()
            .filter(|operation| operation.as_str() == "agent_component_call_stream")
            .count()
            >= 2
    );
    host.child.kill().await?;
    Ok(())
}

/// Scenario B: a registered LlmClient replaces the model transport; the
/// streaming callback delivers chunks and the Host accepts exactly one of
/// two duplicate wire terminals.
#[tokio::test]
async fn llm_client_stream_extension_answers_prompts() -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    // The model server stays unused: the extension replaces the transport.
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.duplicate_terminal.store(true, Ordering::Release);
    let reentrant_mutation = dispatch.reentrant_mutation.clone();

    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let reentrant_mutation = reentrant_mutation.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let llm = register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "sdk-llm",
                llm_descriptor("sdk-fixture-model"),
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: Some("stream-reentry-agent".to_string()),
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: Some(WirePath::Utf8 {
                        path: directory
                            .path()
                            .canonicalize()
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?
                            .display()
                            .to_string(),
                    }),
                    session_id: None,
                    idempotency_id: Some("stream-reentry-session".to_string()),
                })
                .block_task()
                .await?;
            *reentrant_mutation
                .lock()
                .unwrap_or_else(|error| error.into_inner()) =
                Some((agent, session.session.clone()));
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    v1::SessionId::new(session.acp_session_id),
                    vec![v1::ContentBlock::Text(v1::TextContent::new("hello"))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            assert!(unregister(&connection, &llm).await?);
            Ok(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "scenario failed: {outcome:?}");
    let operations = dispatch
        .operations
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    assert!(
        operations
            .iter()
            .any(|operation| operation == "llm_chat_stream" || operation == "llm_chat"),
        "the model call must route through the bridge; got {operations:?}"
    );
    assert!(
        dispatch.reentrant_conflicts.load(Ordering::Acquire) > 0,
        "stream callback mutation after the initial outcome did not return extension_conflict"
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn non_streaming_llm_extension_adapts_chat_to_framework_stream()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "sdk-llm-non-streaming",
                llm_descriptor_with_streaming("sdk-chat-only-model", false),
                None,
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new("chat only"))],
                ))
                .block_task()
                .await?;
            assert_eq!(prompt.stop_reason, v1::StopReason::EndTurn);
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "non-streaming LLM scenario failed: {outcome:?}"
    );
    let operations = dispatch.operations.lock().expect("operations").clone();
    assert!(operations.iter().any(|operation| operation == "llm_chat"));
    assert!(
        !operations
            .iter()
            .any(|operation| operation == "llm_chat_stream")
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn non_streaming_llm_without_finish_reason_fails_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch
        .missing_finish_reason
        .store(true, Ordering::Release);
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "sdk-llm-missing-finish",
                llm_descriptor_with_streaming("sdk-chat-only-model", false),
                None,
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new(
                        "missing finish",
                    ))],
                ))
                .block_task()
                .await;
            assert!(prompt.is_err());
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "missing-finish scenario failed: {outcome:?}"
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn malformed_stream_kind_fails_without_waiting_for_deadline()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.malformed_stream.store(true, Ordering::Release);
    let session_dir = directory.path().canonicalize()?;
    let started = std::time::Instant::now();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let extension = register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "sdk-llm-malformed",
                llm_descriptor("sdk-malformed-model"),
                Some(WireDuration {
                    seconds: WireU64::from_u64(30),
                    nanos: 0,
                }),
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let session_handle: WireHandle = serde_json::from_value(
                session
                    .meta
                    .as_ref()
                    .and_then(|meta| meta.get("echo_agent"))
                    .and_then(|meta| meta.get("session"))
                    .cloned()
                    .ok_or_else(|| {
                        RpcError::internal_error().data("missing original Session bridge handle")
                    })?,
            )
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new("malformed"))],
                ))
                .block_task()
                .await;
            assert!(prompt.is_err());
            let error = prompt.err().ok_or_else(|| {
                RpcError::internal_error().data("original invalid producer did not fail")
            })?;
            assert_eq!(error.code, agent_client_protocol::ErrorCode::InternalError);
            let message = error
                .data
                .as_ref()
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    RpcError::internal_error()
                        .data("standard prompt failure lacks original failure text")
                })?;
            assert!(
                message.contains("extension bridge failure serialization_violation:"),
                "first error code was replaced: {error:?}"
            );
            assert!(
                message.contains("extension stream payload kind does not match the invocation"),
                "first rejection reason was replaced: {error:?}"
            );
            // Registration release proves this callback's inflight lease is
            // gone. Closing its original Session checks cascade cleanup only;
            // neither operation is a substitute for a canonical Run receipt.
            assert!(unregister(&connection, &extension).await?);
            assert!(
                connection
                    .send_request(echo_sdk_protocol::methods::SessionCloseRequest {
                        session: session_handle,
                    })
                    .block_task()
                    .await?
                    .released
            );
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "malformed stream scenario failed: {outcome:?}"
    );
    assert!(started.elapsed() < Duration::from_secs(5));
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn out_of_order_stream_fails_without_waiting_for_deadline()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.out_of_order_stream.store(true, Ordering::Release);
    let session_dir = directory.path().canonicalize()?;
    let started = std::time::Instant::now();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let extension = register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "sdk-llm-out-of-order",
                llm_descriptor("sdk-out-of-order-model"),
                Some(WireDuration {
                    seconds: WireU64::from_u64(30),
                    nanos: 0,
                }),
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let session_handle: WireHandle = serde_json::from_value(
                session
                    .meta
                    .as_ref()
                    .and_then(|meta| meta.get("echo_agent"))
                    .and_then(|meta| meta.get("session"))
                    .cloned()
                    .ok_or_else(|| {
                        RpcError::internal_error().data("missing original Session bridge handle")
                    })?,
            )
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new("out of order"))],
                ))
                .block_task()
                .await;
            assert!(prompt.is_err());
            let error = prompt.err().ok_or_else(|| {
                RpcError::internal_error().data("original invalid producer did not fail")
            })?;
            assert_eq!(error.code, agent_client_protocol::ErrorCode::InternalError);
            let message = error
                .data
                .as_ref()
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    RpcError::internal_error()
                        .data("standard prompt failure lacks original failure text")
                })?;
            assert!(
                message.contains("extension bridge failure serialization_violation:"),
                "first error code was replaced: {error:?}"
            );
            assert!(
                message.contains("extension stream remote sequence is not contiguous"),
                "first rejection reason was replaced: {error:?}"
            );
            // Registration release proves this callback's inflight lease is
            // gone. Closing its original Session checks cascade cleanup only;
            // neither operation is a substitute for a canonical Run receipt.
            assert!(unregister(&connection, &extension).await?);
            assert!(
                connection
                    .send_request(echo_sdk_protocol::methods::SessionCloseRequest {
                        session: session_handle,
                    })
                    .block_task()
                    .await?
                    .released
            );
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "out-of-order stream scenario failed: {outcome:?}"
    );
    assert!(started.elapsed() < Duration::from_secs(5));
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn oversized_stream_chunk_fails_without_waiting_for_deadline()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.oversized_stream.store(true, Ordering::Release);
    let session_dir = directory.path().canonicalize()?;
    let started = std::time::Instant::now();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let extension = register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "sdk-llm-oversized",
                llm_descriptor("sdk-oversized-model"),
                Some(WireDuration {
                    seconds: WireU64::from_u64(30),
                    nanos: 0,
                }),
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let session_handle: WireHandle = serde_json::from_value(
                session
                    .meta
                    .as_ref()
                    .and_then(|meta| meta.get("echo_agent"))
                    .and_then(|meta| meta.get("session"))
                    .cloned()
                    .ok_or_else(|| {
                        RpcError::internal_error().data("missing original Session bridge handle")
                    })?,
            )
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new("oversized"))],
                ))
                .block_task()
                .await;
            assert!(prompt.is_err());
            let error = prompt.err().ok_or_else(|| {
                RpcError::internal_error().data("original invalid producer did not fail")
            })?;
            assert_eq!(error.code, agent_client_protocol::ErrorCode::InternalError);
            let message = error
                .data
                .as_ref()
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    RpcError::internal_error()
                        .data("standard prompt failure lacks original failure text")
                })?;
            assert!(
                message.contains("extension bridge failure serialization_violation:"),
                "first error code was replaced: {error:?}"
            );
            assert!(
                message.contains("extension stream event exceeds the configured byte bound"),
                "first rejection reason was replaced: {error:?}"
            );
            // Registration release proves this callback's inflight lease is
            // gone. Closing its original Session checks cascade cleanup only;
            // neither operation is a substitute for a canonical Run receipt.
            assert!(unregister(&connection, &extension).await?);
            assert!(
                connection
                    .send_request(echo_sdk_protocol::methods::SessionCloseRequest {
                        session: session_handle,
                    })
                    .block_task()
                    .await?
                    .released
            );
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "oversized stream scenario failed: {outcome:?}"
    );
    assert!(started.elapsed() < Duration::from_secs(5));
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn stream_flood_hits_bounded_host_backpressure() -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    set_profile_limit(&config, "max_outstanding_live_events", 1)?;
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.flood_stream.store(true, Ordering::Release);
    let session_dir = directory.path().canonicalize()?;
    let started = std::time::Instant::now();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let extension = register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "sdk-llm-flood",
                llm_descriptor("sdk-flood-model"),
                Some(WireDuration {
                    seconds: WireU64::from_u64(30),
                    nanos: 0,
                }),
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let session_handle: WireHandle = serde_json::from_value(
                session
                    .meta
                    .as_ref()
                    .and_then(|meta| meta.get("echo_agent"))
                    .and_then(|meta| meta.get("session"))
                    .cloned()
                    .ok_or_else(|| {
                        RpcError::internal_error().data("missing original Session bridge handle")
                    })?,
            )
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            let prompt = connection
                .send_request(v1::PromptRequest::new(
                    session.session_id,
                    vec![v1::ContentBlock::Text(v1::TextContent::new("flood"))],
                ))
                .block_task()
                .await;
            assert!(
                prompt.is_err(),
                "a flooded callback must fail, not buffer without bound"
            );
            let error = prompt.err().ok_or_else(|| {
                RpcError::internal_error().data("original invalid producer did not fail")
            })?;
            assert_eq!(error.code, agent_client_protocol::ErrorCode::InternalError);
            let message = error
                .data
                .as_ref()
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    RpcError::internal_error()
                        .data("standard prompt failure lacks original failure text")
                })?;
            assert!(
                message.contains("extension bridge failure extension_failed:"),
                "first error code was replaced: {error:?}"
            );
            assert!(
                message.contains("extension stream consumer exceeded its bounded mailbox"),
                "first rejection reason was replaced: {error:?}"
            );
            // Registration release proves this callback's inflight lease is
            // gone. Closing its original Session checks cascade cleanup only;
            // neither operation is a substitute for a canonical Run receipt.
            assert!(unregister(&connection, &extension).await?);
            assert!(
                connection
                    .send_request(echo_sdk_protocol::methods::SessionCloseRequest {
                        session: session_handle,
                    })
                    .block_task()
                    .await?
                    .released
            );
            Ok(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "backpressure scenario failed: {outcome:?}");
    assert!(started.elapsed() < Duration::from_secs(10));
    let stderr = stderr_text(&host);
    assert!(
        stderr.contains("extension stream consumer exceeded its bounded mailbox"),
        "the real Host must report the extension mailbox backpressure branch: {stderr}"
    );
    assert!(!stderr.contains(SENTINEL_SECRET));
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn sdk_disconnect_cancels_an_unterminated_stream_and_exits_host()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.omit_stream_terminal.store(true, Ordering::Release);
    let session_dir = directory.path().canonicalize()?;
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let session_dir = session_dir.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "sdk-llm-disconnect",
                llm_descriptor("sdk-disconnect-model"),
                Some(WireDuration {
                    seconds: WireU64::from_u64(30),
                    nanos: 0,
                }),
            )
            .await?;
            let session: v1::NewSessionResponse = connection
                .send_request(v1::NewSessionRequest::new(session_dir))
                .block_task()
                .await?;
            let prompt = connection.send_request(v1::PromptRequest::new(
                session.session_id,
                vec![v1::ContentBlock::Text(v1::TextContent::new("disconnect"))],
            ));
            tokio::time::sleep(Duration::from_millis(300)).await;
            drop(prompt);
            Ok(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "disconnect scenario failed: {outcome:?}");
    // The Host's internal shutdown bound remains five seconds. Keep a small
    // process-reaping margin here so scheduler/pipe teardown jitter does not
    // race the assertion at the exact internal deadline.
    let status = tokio::time::timeout(Duration::from_secs(10), host.child.wait())
        .await
        .map_err(|error| {
            format!(
                "Host did not exit after owner disconnect: {error}; stderr:\n{}",
                stderr_text(&host)
            )
        })??;
    let stderr = stderr_text(&host);
    assert!(
        !status.success(),
        "an owner disconnect during an active response must surface as a transport failure"
    );
    assert!(!stderr.contains(SENTINEL_SECRET));
    Ok(())
}

/// Scenario C: an original held stream receives its exact timeout or cancel
/// notice. Its original producer then closes; standard ACP reports Cancelled
/// with the same Run receipt. This is separate from typed invocation errors.
#[tokio::test]
async fn deadline_and_cancellation_settle_typed_outcomes() -> Result<(), Box<dyn std::error::Error>>
{
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    for timeout in [true, false] {
        let mut host = spawn_host(&config).await?;
        let dispatch = Arc::new(SdkDispatch::default());
        dispatch.omit_stream_terminal.store(true, Ordering::Release);
        let scenario_dispatch = dispatch.clone();
        let working_dir = directory.path().canonicalize()?;
        let outcome = drive_sdk(&mut host, dispatch.clone(), move |connection| {
            let dispatch = scenario_dispatch.clone();
            Box::pin(async move {
                connection
                    .send_request(initialize_request(Some(client_hello())))
                    .block_task()
                    .await?;
                register_extension(
                    &connection,
                    ExtensionKind::LlmClient,
                    "sdk-llm-held",
                    llm_descriptor("sdk-held"),
                    Some(WireDuration {
                        seconds: WireU64::from_u64(if timeout { 1 } else { 60 }),
                        nanos: 0,
                    }),
                )
                .await?;
                let session: v1::NewSessionResponse = connection
                    .send_request(v1::NewSessionRequest::new(working_dir))
                    .block_task()
                    .await?;
                let prompt = connection.send_request(v1::PromptRequest::new(
                    session.session_id.clone(),
                    vec![v1::ContentBlock::Text(v1::TextContent::new(
                        "held original stream",
                    ))],
                ));
                if !timeout {
                    wait_fixture_producer_ready(&dispatch, ExtensionOperation::LlmChatStream, None)
                        .await?;
                    connection
                        .send_notification(v1::CancelNotification::new(session.session_id))?;
                }
                let id = wait_fixture_notice(
                    &dispatch,
                    ExtensionOperation::LlmChatStream,
                    if timeout { "timeout" } else { "cancelled" },
                )
                .await?;
                finish_fixture_stream(&dispatch, &id).await?;
                let response = tokio::time::timeout(Duration::from_secs(20), prompt.block_task())
                    .await
                    .map_err(|_| {
                        RpcError::internal_error().data("original cancelled prompt did not respond")
                    })??;
                assert_eq!(response.stop_reason, v1::StopReason::Cancelled);
                let run: WireHandle = serde_json::from_value(
                    response
                        .meta
                        .as_ref()
                        .and_then(|meta| meta.get("echo_agent"))
                        .and_then(|meta| meta.get("run"))
                        .cloned()
                        .ok_or_else(|| {
                            RpcError::internal_error().data("missing original Run bridge")
                        })?,
                )
                .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                let settled = connection
                    .send_request(RunWaitRequest {
                        run,
                        timeout: Some(WireDuration::from_nanos(10_000_000_000)),
                    })
                    .block_task()
                    .await?;
                assert!(settled.settled);
                assert!(matches!(settled.terminal, Some(RunTerminal::Cancelled)));
                assert!(
                    settled
                        .receipt
                        .is_some_and(|receipt| receipt.outcome == "cancelled")
                );
                Ok(())
            })
        })
        .await;
        assert!(
            outcome.is_ok(),
            "original deadline/cancel scenario failed: {outcome:?}"
        );
        assert!(
            host.child.try_wait()?.is_none(),
            "late original callback response killed the Host"
        );
        host.child.kill().await?;
    }
    Ok(())
}

#[tokio::test]
async fn cancelled_tool_waits_for_canonical_interrupted_callback()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![tool_call_script(
        "pending_tool",
        r#"{"query":"原始🙂"}"#,
    )])
    .await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch
        .hang
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .push("tool_execute");
    let pending = dispatch.silent.clone();
    let interrupted = dispatch.interrupted_callbacks.clone();
    let scenario_dispatch = dispatch.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let dispatch = scenario_dispatch.clone();
        let pending = pending.clone();
        let interrupted = interrupted.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let mut descriptor = tool_descriptor("pending_tool");
            if let ExtensionDescriptor::Tool { risk_level, .. } = &mut descriptor {
                *risk_level = ToolRiskLevelWire::Standard;
            }
            register_extension(
                &connection,
                ExtensionKind::Tool,
                "interrupted-tool",
                descriptor,
                None,
            )
            .await?;
            register_extension(
                &connection,
                ExtensionKind::AgentCallback,
                "canonical-callback",
                ExtensionDescriptor::AgentCallback {
                    descriptor_version: 1,
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: None,
                    session_id: None,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            invoke_source_operation(
                &connection,
                agent,
                "echo_agent::agent::react::ReactAgent::set_permission_mode",
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::String("full-auto".to_string()),
                ],
            )
            .await?;
            let started = connection
                .send_request(RunStartRequest {
                    session: session.session,
                    input: RunInput::Chat {
                        text: "cancel the tool".to_string(),
                    },
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            tokio::time::timeout(Duration::from_secs(10), async {
                while pending
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .is_empty()
                {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            connection
                .send_request(RunCancelRequest {
                    run: started.run.clone(),
                })
                .block_task()
                .await?;
            let original_id =
                wait_fixture_notice(&dispatch, ExtensionOperation::ToolExecute, "cancelled")
                    .await?;
            let held = connection
                .send_request(RunWaitRequest {
                    run: started.run.clone(),
                    timeout: Some(WireDuration::from_nanos(50_000_000)),
                })
                .block_task()
                .await?;
            assert!(
                !held.settled && held.terminal.is_none() && held.receipt.is_none(),
                "notice alone cannot settle the original Tool callback"
            );
            assert!(
                interrupted
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .is_empty(),
                "canonical interrupted callback must follow original Tool closure"
            );
            let original =
                take_fixture_invocation(&dispatch, &original_id, ExtensionOperation::ToolExecute)?;
            assert_eq!(
                original.call.invocation.operation(),
                ExtensionOperation::ToolExecute
            );
            original.respond(ExtensionInvokeOutcome::Error {
                error: EchoSdkError::new(
                    ExtensionErrorCode::Cancelled,
                    "original fixture callback completed cancellation",
                    Retryability::Never,
                ),
            })?;
            let terminal = connection
                .send_request(RunWaitRequest {
                    run: started.run,
                    timeout: Some(WireDuration::from_nanos(10_000_000_000)),
                })
                .block_task()
                .await?;
            assert!(matches!(terminal.terminal, Some(RunTerminal::Cancelled)));
            let interrupted = interrupted
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            assert_eq!(
                interrupted.len(),
                1,
                "Run terminal must follow the interrupted callback's delayed completed side effect"
            );
            let callback = interrupted
                .first()
                .ok_or_else(|| RpcError::internal_error().data("missing interrupted callback"))?;
            assert!(!callback.call_id.is_empty());
            assert_eq!(callback.tool, "pending_tool");
            assert_eq!(
                callback
                    .input
                    .clone()
                    .into_json()
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                serde_json::json!({"query":"原始🙂"})
            );
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "terminal callback dropped on cancelled Run: {outcome:?}; {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

/// Payload admission rejects a constructor before submission. Its reserved
/// child is released locally, so unregister has no phantom cleanup debt.
#[cfg(feature = "framework-eval")]
#[tokio::test]
async fn factory_presubmit_failure_releases_unseen_child() -> Result<(), Box<dyn std::error::Error>>
{
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    set_profile_limit(&config, "max_extension_payload_bytes", 1)?;
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let created = dispatch.factory_created.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let created = created.clone();
        Box::pin(async move {
            connection.send_request(initialize_request(Some(client_hello()))).block_task().await?;
            let factory = register_extension(&connection, ExtensionKind::AgentFactory, "presubmit-factory", ExtensionDescriptor::AgentFactory { descriptor_version: 1 }, None).await?;
            let agent = connection.send_request(AgentCreateRequest { config: AgentConfigWire::HostDefault, idempotency_id: None }).block_task().await?.agent;
            let session = connection.send_request(SessionCreateRequest { agent: agent.clone(), working_dir: None, session_id: None, idempotency_id: None }).block_task().await?;
            let runner = invoke_source_operation(&connection, agent.clone(), "echo_agent::eval::runner::EvalRunner::new", vec![WireValue::Handle(session.session.clone()), WireValue::Path(WirePath::Utf8 { path: std::env::temp_dir().display().to_string() })]).await?;
            let WireValue::Handle(runner) = runner.value else { return Err(RpcError::internal_error().data("missing EvalRunner")); };
            let case = WireValue::from_json(serde_json::json!({"id":"unseen","name":"unseen","description":"presubmit payload bound","domain":null,"task":"never construct","project_fixture":null,"success_criteria":{"type":"output_contains","substring":"SDK"},"constraints":{}})).map_err(|error| RpcError::invalid_params().data(error.to_string()))?;
            let _result = invoke_source_operation(&connection, agent, "echo_agent::eval::runner::EvalRunner::run_all", vec![WireValue::Handle(session.session), WireValue::Handle(runner), WireValue::List(vec![case]), WireValue::Handle(factory.clone())]).await;
            assert!(created.lock().unwrap_or_else(|error| error.into_inner()).is_empty(), "rejected constructor must never reach the language implementation");
            assert!(unregister(&connection, &factory).await?, "an unseen child must not become remote cleanup debt");
            Ok(())
        })
    }).await;
    assert!(
        outcome.is_ok(),
        "factory pre-submit reservation leaked: {outcome:?}; {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn crud_only_task_store_rejects_execute_before_runtime_side_effects()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let backend = dispatch.task_store.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let backend = backend.clone();
        Box::pin(async move {
            connection.send_request(initialize_request(Some(client_hello()))).block_task().await?;
            register_extension(&connection, ExtensionKind::AgentComponent, "crud-only", ExtensionDescriptor::AgentComponent { descriptor_version: 1, component: AgentComponentKindWire::RevisionedTaskStore, name: "crud-only".to_string(), capabilities: Default::default() }, None).await?;
            let agent = connection.send_request(AgentCreateRequest { config: AgentConfigWire::HostDefault, idempotency_id: None }).block_task().await?.agent;
            let session = connection.send_request(SessionCreateRequest { agent, working_dir: None, session_id: None, idempotency_id: None }).block_task().await?;
            connection.send_request(TaskCreateRequest { task_run: session.task_run.clone(), spec: WireValue::from_json(serde_json::json!({"tasks":[{"id":"pending","title":"pending","description":"must not dispatch"}]})).map_err(|error| RpcError::invalid_params().data(error.to_string()))? }).block_task().await?;
            let error = connection.send_request(TaskExecuteRequest { task_run: session.task_run.clone() }).block_task().await.err().ok_or_else(|| RpcError::internal_error().data("CRUD-only execution unexpectedly accepted"))?;
            let typed = EchoSdkError::from_jsonrpc_data(error.data.as_ref()).map_err(|error| RpcError::internal_error().data(error))?;
            assert_eq!(typed.code, ExtensionErrorCode::FeatureUnavailable);
            let listed = connection.send_request(TaskListRequest { task_run: session.task_run }).block_task().await?;
            assert!(listed.tasks.iter().all(|task| matches!(task.status, WireTaskStatus::Pending)));
            assert_eq!(backend.execution_commits.load(Ordering::Acquire), 0, "capability preflight must not probe a runtime mutation");
            Ok(())
        })
    }).await;
    assert!(
        outcome.is_ok(),
        "CRUD-only backend rejection failed: {outcome:?}; {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn task_execution_backend_failure_remains_queryable_as_typed_error()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch
        .task_store
        .reject_execution
        .store(true, Ordering::Release);
    let backend = dispatch.task_store.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let backend = backend.clone();
        Box::pin(async move {
            connection.send_request(initialize_request(Some(client_hello()))).block_task().await?;
            register_extension(&connection, ExtensionKind::AgentComponent, "failing-backend", ExtensionDescriptor::AgentComponent { descriptor_version: 1, component: AgentComponentKindWire::RevisionedTaskStore, name: "failing-backend".to_string(), capabilities: AgentComponentCapabilitiesWire { execution_cas_v1: true, ..Default::default() } }, None).await?;
            let agent = connection.send_request(AgentCreateRequest { config: AgentConfigWire::HostDefault, idempotency_id: None }).block_task().await?.agent;
            let session = connection.send_request(SessionCreateRequest { agent, working_dir: None, session_id: None, idempotency_id: None }).block_task().await?;
            connection.send_request(TaskCreateRequest { task_run: session.task_run.clone(), spec: WireValue::from_json(serde_json::json!({"tasks":[{"id":"pending","title":"pending","description":"backend failure"}]})).map_err(|error| RpcError::invalid_params().data(error.to_string()))? }).block_task().await?;
            connection.send_request(TaskExecuteRequest { task_run: session.task_run.clone() }).block_task().await?;
            let failure = tokio::time::timeout(Duration::from_secs(10), async {
                loop {
                    match connection.send_request(TaskListRequest { task_run: session.task_run.clone() }).block_task().await {
                        Ok(_) => tokio::time::sleep(Duration::from_millis(10)).await,
                        Err(error) => return Ok::<_, RpcError>(error),
                    }
                }
            }).await.map_err(|error| RpcError::internal_error().data(error.to_string()))??;
            let typed = EchoSdkError::from_jsonrpc_data(failure.data.as_ref()).map_err(|error| RpcError::internal_error().data(error))?;
            assert_eq!(typed.code, ExtensionErrorCode::FrameworkError);
            assert_eq!(typed.operation.as_deref(), Some("_echo_agent/task/list"));
            assert_eq!(typed.handle, Some(session.task_run.clone()));
            assert!(typed.details.as_ref().and_then(|details| details.fields.as_ref()).is_some_and(|fields| fields.iter().any(|field| field.key == "origin_operation" && field.value == "_echo_agent/task/execute")), "the current query must preserve the originating execution failure identity");
            let repeated = connection.send_request(TaskListRequest { task_run: session.task_run.clone() }).block_task().await.err().ok_or_else(|| RpcError::internal_error().data("lost TaskRun execution error on repeated query"))?;
            assert_eq!(EchoSdkError::from_jsonrpc_data(repeated.data.as_ref()).map_err(|error| RpcError::internal_error().data(error))?, typed);
            let graphs = backend.graphs.lock().unwrap_or_else(|error| error.into_inner());
            assert!(graphs.values().flat_map(|graph| graph.snapshot.tasks.iter()).all(|task| matches!(task.execution.status, WireTaskStatus::Pending)), "diagnostic projection must not invent a canonical Failed task");
            Ok(())
        })
    }).await;
    assert!(
        outcome.is_ok(),
        "backend execution error became invisible: {outcome:?}; {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

/// External CRUD and execution callbacks observe one backend graph; its
/// stored Completed state contains the exact Subagent result in the same CAS.
#[tokio::test]
async fn revisioned_task_extension_is_shared_by_crud_and_execution()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let diagnostics = dispatch.component_operations.clone();
    let backend = dispatch.task_store.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let backend = backend.clone();
        Box::pin(async move {
            connection.send_request(initialize_request(Some(client_hello()))).block_task().await?;
            register_extension(&connection, ExtensionKind::AgentComponent, "task-backend", ExtensionDescriptor::AgentComponent { descriptor_version: 1, component: AgentComponentKindWire::RevisionedTaskStore, name: "task-backend".to_string(), capabilities: AgentComponentCapabilitiesWire { execution_cas_v1: true, ..Default::default() } }, None).await?;
            register_extension(&connection, ExtensionKind::CustomAgent, "task-agent", ExtensionDescriptor::CustomAgent { descriptor_version: 1, name: "task-agent".to_string(), model_name: "sdk".to_string(), system_prompt: String::new(), tool_names: Vec::new() }, None).await?;
            let agent = connection.send_request(AgentCreateRequest { config: AgentConfigWire::HostDefault, idempotency_id: None }).block_task().await?.agent;
            let session = connection.send_request(SessionCreateRequest { agent, working_dir: None, session_id: None, idempotency_id: None }).block_task().await?;
            let created = connection.send_request(TaskCreateRequest { task_run: session.task_run.clone(), spec: WireValue::from_json(serde_json::json!({"execution_mode":"sequential","tasks":[{"id":"A","title":"A","description":"first","extension":{"subagent":"task-agent"}},{"id":"B","title":"B","description":"second","depends_on":["A"],"extension":{"subagent":"task-agent"}}]})).map_err(|error| RpcError::invalid_params().data(error.to_string()))? }).block_task().await?;
            assert_eq!(created.tasks.len(), 2);
            assert_eq!(backend.relation_commits.load(Ordering::Acquire), 1);
            connection.send_request(TaskExecuteRequest { task_run: session.task_run.clone() }).block_task().await?;
            tokio::time::timeout(Duration::from_secs(10), async {
                loop {
                    let listed = connection.send_request(TaskListRequest { task_run: session.task_run.clone() }).block_task().await?;
                    if listed.tasks.iter().all(|task| matches!(task.status, WireTaskStatus::Completed)) && listed.tasks.len() == 2 { return Ok::<_, RpcError>(()); }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }).await.map_err(|error| RpcError::internal_error().data(error.to_string()))??;
            assert!(backend.execution_commits.load(Ordering::Acquire) >= 4, "claim and terminal writes must use the registered backend");
            let graphs = backend.graphs.lock().unwrap_or_else(|error| error.into_inner());
            assert_eq!(graphs.len(), 1);
            let graph = graphs.values().next().ok_or_else(|| RpcError::internal_error().data("backend graph missing"))?;
            assert_eq!(graph.snapshot.revision.to_u64(), Some(1), "runtime execution CAS must not bump relation revision");
            assert!(graph.snapshot.tasks.iter().all(|task| matches!(task.execution.status, WireTaskStatus::Completed) && task.execution.result.is_some()), "Completed must carry its durable result in the backend transaction");
            Ok(())
        })
    }).await;
    assert!(
        outcome.is_ok(),
        "registered task backend diverged: {outcome:?}; stderr: {}; operations: {:?}; reverse frames: {:?}; callback replies: {}",
        stderr_text(&host),
        diagnostics
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        reverse_frames(&host),
        String::from_utf8_lossy(&host.stdin.lock().unwrap_or_else(|error| error.into_inner()))
    );
    host.child.kill().await?;
    Ok(())
}

#[tokio::test]
async fn missing_managed_capability_fails_before_store_mutation()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![final_script("must not commit")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    let operations = dispatch.component_operations.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let operations = operations.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            register_extension(
                &connection,
                ExtensionKind::AgentComponent,
                "legacy-conversation",
                ExtensionDescriptor::AgentComponent {
                    descriptor_version: 1,
                    component: AgentComponentKindWire::ConversationStore,
                    name: "legacy-conversation".to_string(),
                    capabilities: Default::default(),
                },
                None,
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent,
                    working_dir: None,
                    session_id: None,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            let started = connection
                .send_request(RunStartRequest {
                    session: session.session,
                    input: RunInput::Chat {
                        text: "cannot persist".to_string(),
                    },
                    idempotency_id: None,
                })
                .block_task()
                .await;
            match started {
                Err(error) => assert!(EchoSdkError::from_jsonrpc_data(error.data.as_ref()).is_ok()),
                Ok(started) => {
                    let terminal = connection
                        .send_request(RunWaitRequest {
                            run: started.run,
                            timeout: Some(WireDuration::from_nanos(10_000_000_000)),
                        })
                        .block_task()
                        .await?;
                    assert!(
                        matches!(terminal.terminal, Some(RunTerminal::Failed { .. })),
                        "missing persistence capability must fail explicitly"
                    );
                }
            }
            assert!(
                operations
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .iter()
                    .all(|operation| !operation.starts_with("ConversationStore:")),
                "capability rejection must precede every reverse store side effect"
            );
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "missing managed capability was silently accepted: {outcome:?}; {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

/// Managed persistence is tested through its real production Run path, with
/// a remote backend that loses an acknowledgement after its atomic commit.
#[tokio::test]
async fn managed_persistence_deadline_and_lost_ack_preserve_commit_authority()
-> Result<(), Box<dyn std::error::Error>> {
    for lose_ack in [false, true] {
        let directory = tempfile::tempdir()?;
        let model =
            start_scripted_model(vec![final_script("first"), final_script("second")]).await?;
        let config = write_config(
            directory.path(),
            &format!("http://{model}/v1/chat/completions"),
            &directory.path().join("state"),
        );
        let mut host = spawn_host(&config).await?;
        let dispatch = Arc::new(SdkDispatch::default());
        dispatch.managed.enabled.store(true, Ordering::Release);
        dispatch
            .managed
            .delay_before_commit
            .store(!lose_ack, Ordering::Release);
        dispatch
            .managed
            .lose_first_apply_response
            .store(lose_ack, Ordering::Release);
        let managed = dispatch.managed.clone();
        let events = dispatch.events.clone();
        let outcome = drive_sdk(&mut host, dispatch, move |connection| {
            let managed = managed.clone();
            let events = events.clone();
            Box::pin(async move {
                connection
                    .send_request(initialize_request(Some(client_hello())))
                    .block_task()
                    .await?;
                for component in [
                    AgentComponentKindWire::ConversationStore,
                    AgentComponentKindWire::RuntimeStateStore,
                ] {
                    register_extension(
                        &connection,
                        ExtensionKind::AgentComponent,
                        &format!("managed-{component:?}"),
                        ExtensionDescriptor::AgentComponent {
                            descriptor_version: 1,
                            component,
                            name: format!("managed-{component:?}"),
                            capabilities: AgentComponentCapabilitiesWire {
                                conversation_projection: if component
                                    == AgentComponentKindWire::ConversationStore
                                {
                                    ConversationProjectionCapabilityWire::AtomicV1
                                } else {
                                    ConversationProjectionCapabilityWire::Unsupported
                                },
                                runtime_state: if component
                                    == AgentComponentKindWire::RuntimeStateStore
                                {
                                    RuntimeStateCapabilityWire::RevisionedV1
                                } else {
                                    RuntimeStateCapabilityWire::Unsupported
                                },
                                persistence_call: PersistenceCallCapabilityWire::AbsoluteDeadlineV1,
                                ..Default::default()
                            },
                        },
                        Some(WireDuration::from_nanos(30_000_000_000)),
                    )
                    .await?;
                }
                let agent = connection
                    .send_request(AgentCreateRequest {
                        config: AgentConfigWire::HostDefault,
                        idempotency_id: None,
                    })
                    .block_task()
                    .await?
                    .agent;
                let session = connection
                    .send_request(SessionCreateRequest {
                        agent,
                        working_dir: None,
                        session_id: None,
                        idempotency_id: None,
                    })
                    .block_task()
                    .await?;
                let started = connection
                    .send_request(RunStartRequest {
                        session: session.session.clone(),
                        input: RunInput::Chat {
                            text: "first".to_string(),
                        },
                        idempotency_id: None,
                    })
                    .block_task()
                    .await?;
                let terminal = connection
                    .send_request(RunWaitRequest {
                        run: started.run,
                        timeout: Some(WireDuration::from_nanos(20_000_000_000)),
                    })
                    .block_task()
                    .await?;
                assert!(terminal.terminal.is_some());
                if !lose_ack {
                    tokio::time::timeout(Duration::from_secs(2), async {
                        while managed.expired.load(Ordering::Acquire) == 0 {
                            tokio::time::sleep(Duration::from_millis(10)).await;
                        }
                    })
                    .await
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                    assert_eq!(
                        managed.mutations.load(Ordering::Acquire),
                        0,
                        "an expired absolute deadline must never start a durable mutation"
                    );
                    return Ok(());
                }
                let committed_receipts = managed
                    .receipts
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .clone();
                let committed = managed.projected_items.load(Ordering::Acquire);
                assert!(
                    committed > 0,
                    "fixture must commit before withholding its response"
                );
                let pending = managed
                    .runtime
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .values()
                    .filter_map(|snapshot| snapshot.checkpoint.as_ref())
                    .any(|checkpoint| {
                        serde_json::from_str::<serde_json::Value>(&checkpoint.messages_json)
                            .ok()
                            .and_then(|payload| {
                                payload.get("pending_transcript_projection").cloned()
                            })
                            .is_some_and(|pending| !pending.is_null())
                    });
                assert!(
                    pending,
                    "lost response must retain the durable pending effect for recovery"
                );
                let deferred_unknown = events.lock().unwrap_or_else(|error| error.into_inner()).iter().any(|event| {
                    event.payload.event_type == "transcript_projection_settlement"
                        && event.payload.data.clone().and_then(|data| data.into_json().ok())
                            .and_then(|data| serde_json::from_value::<TranscriptProjectionSettlementWire>(data).ok())
                            .is_some_and(|settlement| settlement.status == TranscriptProjectionSettlementStatusWire::Deferred
                                && settlement.error_class == Some(TranscriptProjectionErrorClassWire::DeadlineExceeded)
                                && settlement.operation_id.as_ref().is_some_and(|operation| committed_receipts.contains_key(operation)))
                });
                // The framework's shared absolute deadline classifies a
                // timeout as DeadlineExceeded. It retains a durable pending
                // intent because expiry after dispatch does not prove no commit.
                assert!(deferred_unknown, "post-dispatch deadline must preserve a typed deferred effect with its committed operation identity");
                let second = connection
                    .send_request(RunStartRequest {
                        session: session.session,
                        input: RunInput::Chat {
                            text: "second".to_string(),
                        },
                        idempotency_id: None,
                    })
                    .block_task()
                    .await?;
                let settled = connection
                    .send_request(RunWaitRequest {
                        run: second.run,
                        timeout: Some(WireDuration::from_nanos(20_000_000_000)),
                    })
                    .block_task()
                    .await?;
                assert!(
                    matches!(settled.terminal, Some(RunTerminal::Completed { .. })),
                    "recovery must reconcile the known receipt before continuing: {:?}",
                    settled.terminal
                );
                assert!(
                    managed.duplicate_applies.load(Ordering::Acquire) > 0,
                    "framework must recover using the same durable batch identity"
                );
                let receipt_count = managed
                    .receipts
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .len();
                assert!(receipt_count > 0);
                let receipts = managed
                    .receipts
                    .lock()
                    .unwrap_or_else(|error| error.into_inner());
                for (operation, original) in committed_receipts {
                    assert_eq!(
                        receipts.get(&operation),
                        Some(&original),
                        "recovery must preserve the exact atomic commit receipt"
                    );
                }
                Ok(())
            })
        })
        .await;
        assert!(
            outcome.is_ok(),
            "managed persistence scenario lose_ack={lose_ack} failed: {outcome:?}; stderr: {}",
            stderr_text(&host)
        );
        host.child.kill().await?;
    }
    Ok(())
}

/// A backend commit survives an actual ACP disconnect and Host restart.
/// Recovery reuses its operation identity and exact receipt instead of
/// treating an unanswered callback as a known no-commit failure.
#[tokio::test]
async fn managed_commit_disconnect_restart_replays_same_receipt()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![
        final_script("before disconnect"),
        final_script("after restart"),
    ])
    .await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch.managed.enabled.store(true, Ordering::Release);
    dispatch
        .managed
        .lose_first_apply_response
        .store(true, Ordering::Release);
    let mut host = spawn_host(&config).await?;
    let backend = dispatch.managed.clone();
    let first = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let backend = backend.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            register_managed_stores(&connection).await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent,
                    working_dir: None,
                    session_id: Some("managed-restart".to_string()),
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            connection
                .send_request(RunStartRequest {
                    session: session.session,
                    input: RunInput::Chat {
                        text: "commit then disconnect".to_string(),
                    },
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            tokio::time::timeout(Duration::from_secs(10), async {
                while backend.projected_items.load(Ordering::Acquire) == 0 {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            // Returning closes the official ACP transport while the reverse
            // response is still held. This is a real disconnect, not an error DTO.
            Ok(())
        })
    })
    .await;
    assert!(
        first.is_ok(),
        "disconnect fixture failed: {first:?}; {}",
        stderr_text(&host)
    );
    let exit_started = std::time::Instant::now();
    let disconnected_pid = host.child.id();
    let _ = tokio::time::timeout(Duration::from_secs(10), host.child.wait())
        .await
        .map_err(|error| {
            // Report the original passive-exit failure without extending its
            // deadline or replacing disconnect with a signal/forced exit.
            let observed_status = host.child.try_wait();
            let held_responses = dispatch.silent.try_lock().ok().map(|held| held.len());
            let receipt_count = dispatch
                .managed
                .receipts
                .try_lock()
                .ok()
                .map(|receipts| receipts.len());
            format!(
                "managed restart phase=first-host-exit failed: {error}; \
                 pid={disconnected_pid:?}; elapsed={:?}; status={observed_status:?}; \
                 stdin_taken={}; stdout_taken={}; held_responses={held_responses:?}; \
                 projected_items={}; mutations={}; receipts={receipt_count:?}; \
                 pipe_observations_since_capture_start_ns={:?}; stderr:\n{}",
                exit_started.elapsed(),
                host.child.stdin.is_none(),
                host.child.stdout.is_none(),
                dispatch.managed.projected_items.load(Ordering::Acquire),
                dispatch.managed.mutations.load(Ordering::Acquire),
                host.pipes.snapshot(),
                stderr_text(&host),
            )
        })??;
    let committed_receipts = dispatch
        .managed
        .receipts
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clone();
    assert!(!committed_receipts.is_empty());
    let pending = dispatch
        .managed
        .runtime
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .values()
        .filter_map(|snapshot| snapshot.checkpoint.as_ref())
        .any(|checkpoint| {
            serde_json::from_str::<serde_json::Value>(&checkpoint.messages_json)
                .ok()
                .and_then(|payload| payload.get("pending_transcript_projection").cloned())
                .is_some_and(|pending| !pending.is_null())
        });
    assert!(
        pending,
        "disconnect must retain framework-owned pending intent"
    );
    // Keep the backend's durable committed state across the new transport.
    let mut host = spawn_host(&config).await?;
    let backend = dispatch.managed.clone();
    let resumed = drive_sdk(&mut host, dispatch.clone(), move |connection| {
        let backend = backend.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            register_managed_stores(&connection).await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionLoadRequest {
                    agent,
                    session_id: "managed-restart".to_string(),
                    working_dir: None,
                })
                .block_task()
                .await?;
            let run = connection
                .send_request(RunStartRequest {
                    session: session.session,
                    input: RunInput::Chat {
                        text: "continue after restart".to_string(),
                    },
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            let terminal = connection
                .send_request(RunWaitRequest {
                    run: run.run,
                    timeout: Some(WireDuration::from_nanos(20_000_000_000)),
                })
                .block_task()
                .await?;
            assert!(
                matches!(terminal.terminal, Some(RunTerminal::Completed { .. })),
                "restart recovery failed: {:?}",
                terminal.terminal
            );
            assert!(
                backend.duplicate_applies.load(Ordering::Acquire) > 0,
                "restart must reconcile the same operation instead of creating a replacement batch"
            );
            Ok(())
        })
    })
    .await;
    assert!(
        resumed.is_ok(),
        "managed restart fixture failed: {resumed:?}; {}",
        stderr_text(&host)
    );
    {
        let receipts = dispatch
            .managed
            .receipts
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        for (operation, original) in committed_receipts {
            assert_eq!(receipts.get(&operation), Some(&original));
        }
    }
    host.child.kill().await?;
    Ok(())
}

async fn register_managed_stores(
    connection: &ConnectionTo<agent_client_protocol::Agent>,
) -> Result<(), RpcError> {
    for component in [
        AgentComponentKindWire::ConversationStore,
        AgentComponentKindWire::RuntimeStateStore,
    ] {
        register_extension(
            connection,
            ExtensionKind::AgentComponent,
            &format!("managed-{component:?}"),
            ExtensionDescriptor::AgentComponent {
                descriptor_version: 1,
                component,
                name: format!("managed-{component:?}"),
                capabilities: AgentComponentCapabilitiesWire {
                    conversation_projection: if component
                        == AgentComponentKindWire::ConversationStore
                    {
                        ConversationProjectionCapabilityWire::AtomicV1
                    } else {
                        ConversationProjectionCapabilityWire::Unsupported
                    },
                    runtime_state: if component == AgentComponentKindWire::RuntimeStateStore {
                        RuntimeStateCapabilityWire::RevisionedV1
                    } else {
                        RuntimeStateCapabilityWire::Unsupported
                    },
                    persistence_call: PersistenceCallCapabilityWire::AbsoluteDeadlineV1,
                    ..Default::default()
                },
            },
            Some(WireDuration::from_nanos(30_000_000_000)),
        )
        .await?;
    }
    Ok(())
}

/// The ordinary ToolManager timeout drops the reverse request before its
/// first response. Both Session reentry and handle quota must become usable.
#[tokio::test]
async fn tool_manager_timeout_before_stream_ack_releases_callback_owner()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![
        tool_call_script("pending_tool", r#"{"query":"timeout"}"#),
        final_script("recovered"),
    ])
    .await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let mut document: serde_json::Value = serde_json::from_slice(&std::fs::read(&config)?)?;
    let settings = document
        .pointer_mut("/default_agent/agent")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or("missing agent settings")?;
    settings.insert("tool_timeout_ms".to_string(), serde_json::json!(50));
    std::fs::write(&config, serde_json::to_vec_pretty(&document)?)?;
    // One default Agent + extension + explicit Agent + Session + TaskRun +
    // Run + Run stream + pending callback exhaust this budget exactly.
    set_profile_limit(&config, "max_open_handles", 8)?;
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch
        .hang
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .push("tool_execute_stream");
    dispatch.hang_once.store(true, Ordering::Release);
    let pending = dispatch.silent.clone();
    let scenario_dispatch = dispatch.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let dispatch = scenario_dispatch.clone();
        let pending = pending.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            let mut descriptor = tool_descriptor("pending_tool");
            if let ExtensionDescriptor::Tool {
                supports_streaming, ..
            } = &mut descriptor
            {
                *supports_streaming = true;
            }
            let extension = register_extension(
                &connection,
                ExtensionKind::Tool,
                "pending-tool",
                descriptor,
                Some(WireDuration::from_nanos(30_000_000_000)),
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: None,
                    session_id: None,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            let started = connection
                .send_request(RunStartRequest {
                    session: session.session.clone(),
                    input: RunInput::Chat {
                        text: "call the tool".to_string(),
                    },
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            tokio::time::timeout(Duration::from_secs(10), async {
                while pending
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .is_empty()
                {
                    tokio::task::yield_now().await;
                }
            })
            .await
            .map_err(|_| RpcError::internal_error().data("original Tool callback did not start"))?;
            let original_id = wait_fixture_notice(
                &dispatch,
                ExtensionOperation::ToolExecuteStream,
                "cancelled",
            )
            .await?;
            let held = connection
                .send_request(RunWaitRequest {
                    run: started.run.clone(),
                    timeout: Some(WireDuration::from_nanos(50_000_000)),
                })
                .block_task()
                .await?;
            assert!(
                !held.settled && held.terminal.is_none() && held.receipt.is_none(),
                "notice alone cannot settle the original Tool callback"
            );
            let original = take_fixture_invocation(
                &dispatch,
                &original_id,
                ExtensionOperation::ToolExecuteStream,
            )?;
            assert_eq!(
                original.call.invocation.operation(),
                ExtensionOperation::ToolExecuteStream
            );
            original.respond(ExtensionInvokeOutcome::Error {
                error: EchoSdkError::new(
                    ExtensionErrorCode::Cancelled,
                    "original fixture callback completed cancellation",
                    Retryability::Never,
                ),
            })?;
            let terminal = connection
                .send_request(RunWaitRequest {
                    run: started.run,
                    timeout: Some(WireDuration::from_nanos(10_000_000_000)),
                })
                .block_task()
                .await?;
            assert!(terminal.terminal.is_some());
            assert_eq!(
                dispatch
                    .operations
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .iter()
                    .filter(|operation| operation.as_str() == "tool_execute_stream")
                    .count(),
                2,
                "the framework retry must reach a distinct real second callback"
            );
            assert!(
                pending
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .is_empty(),
                "the same original callback must have physically answered"
            );
            invoke_source_operation(
                &connection,
                agent,
                "echo_agent::agent::react::ReactAgent::set_plan_mode",
                vec![WireValue::Handle(session.session), WireValue::Bool(true)],
            )
            .await?;
            connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            assert!(unregister(&connection, &extension).await?);
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "ToolManager timeout leaked pending stream: {outcome:?}; stderr: {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

/// Cancellation before the reverse stream acknowledgement releases the
/// pending Session callback owner, so the next exclusive mutation can run.
#[tokio::test]
async fn cancelled_initial_stream_response_releases_session_callback_owner()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &directory.path().join("state"),
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());
    dispatch
        .hang
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .push("llm_chat_stream");
    let pending = dispatch.silent.clone();
    let scenario_dispatch = dispatch.clone();
    let outcome = drive_sdk(&mut host, dispatch, move |connection| {
        let dispatch = scenario_dispatch.clone();
        let pending = pending.clone();
        Box::pin(async move {
            connection
                .send_request(initialize_request(Some(client_hello())))
                .block_task()
                .await?;
            register_extension(
                &connection,
                ExtensionKind::LlmClient,
                "pending-llm",
                llm_descriptor("pending"),
                Some(WireDuration::from_nanos(30_000_000_000)),
            )
            .await?;
            let agent = connection
                .send_request(AgentCreateRequest {
                    config: AgentConfigWire::HostDefault,
                    idempotency_id: None,
                })
                .block_task()
                .await?
                .agent;
            let session = connection
                .send_request(SessionCreateRequest {
                    agent: agent.clone(),
                    working_dir: None,
                    session_id: None,
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            let started = connection
                .send_request(RunStartRequest {
                    session: session.session.clone(),
                    input: RunInput::Chat {
                        text: "wait for initial callback".to_string(),
                    },
                    idempotency_id: None,
                })
                .block_task()
                .await?;
            tokio::time::timeout(Duration::from_secs(10), async {
                loop {
                    if !pending
                        .lock()
                        .unwrap_or_else(|error| error.into_inner())
                        .is_empty()
                    {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            connection
                .send_request(RunCancelRequest {
                    run: started.run.clone(),
                })
                .block_task()
                .await?;
            let original_id =
                wait_fixture_notice(&dispatch, ExtensionOperation::LlmChatStream, "cancelled")
                    .await?;
            // The driver can publish its original cancellation receipt while
            // the independently retained callback still blocks mutation.
            let held_run = connection
                .send_request(RunWaitRequest {
                    run: started.run.clone(),
                    timeout: Some(WireDuration::from_nanos(10_000_000_000)),
                })
                .block_task()
                .await?;
            assert!(held_run.settled && matches!(held_run.terminal, Some(RunTerminal::Cancelled)));
            assert!(
                held_run
                    .receipt
                    .as_ref()
                    .is_some_and(|receipt| receipt.outcome == "cancelled")
            );
            let mutation = "echo_agent::agent::react::ReactAgent::set_plan_mode";
            let rejected = invoke_source_operation(
                &connection,
                agent.clone(),
                mutation,
                vec![
                    WireValue::Handle(session.session.clone()),
                    WireValue::Bool(true),
                ],
            )
            .await
            .err()
            .ok_or_else(|| {
                RpcError::internal_error().data("held original callback allowed mutation")
            })?;
            let typed = EchoSdkError::from_jsonrpc_data(rejected.data.as_ref())
                .map_err(|error| RpcError::internal_error().data(error))?;
            assert_eq!(typed.code, ExtensionErrorCode::ExtensionConflict);
            assert_eq!(typed.operation.as_deref(), Some(mutation));
            let original = take_fixture_invocation(
                &dispatch,
                &original_id,
                ExtensionOperation::LlmChatStream,
            )?;
            assert_eq!(
                original.call.invocation.operation(),
                ExtensionOperation::LlmChatStream
            );
            original.respond(ExtensionInvokeOutcome::Error {
                error: EchoSdkError::new(
                    ExtensionErrorCode::Cancelled,
                    "original fixture callback completed cancellation",
                    Retryability::Never,
                ),
            })?;
            let terminal = connection
                .send_request(RunWaitRequest {
                    run: started.run,
                    timeout: Some(WireDuration::from_nanos(10_000_000_000)),
                })
                .block_task()
                .await?;
            assert!(matches!(terminal.terminal, Some(RunTerminal::Cancelled)));
            invoke_source_operation(
                &connection,
                agent,
                "echo_agent::agent::react::ReactAgent::set_plan_mode",
                vec![WireValue::Handle(session.session), WireValue::Bool(true)],
            )
            .await?;
            Ok(())
        })
    })
    .await;
    assert!(
        outcome.is_ok(),
        "pending stream owner leaked: {outcome:?}; stderr: {}",
        stderr_text(&host)
    );
    host.child.kill().await?;
    Ok(())
}

/// Scenario D: a plain standard Client never sees the bridge surface.
#[tokio::test]
async fn plain_clients_get_method_not_found_for_the_bridge()
-> Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let state_root = directory.path().join("state");
    let model = start_scripted_model(vec![final_script("unused")]).await?;
    let config = write_config(
        directory.path(),
        &format!("http://{model}/v1/chat/completions"),
        &state_root,
    );
    let mut host = spawn_host(&config).await?;
    let dispatch = Arc::new(SdkDispatch::default());

    let outcome = drive_sdk(&mut host, dispatch.clone(), |connection| {
        Box::pin(async move {
            connection
                .send_request(initialize_request(None))
                .block_task()
                .await?;
            let result = connection
                .send_request(ExtensionRegisterRequest {
                    kind: ExtensionKind::Tool,
                    implementation_id: "plain-tool".to_string(),
                    descriptor: tool_descriptor("plain_tool"),
                    timeout: None,
                })
                .block_task()
                .await;
            assert!(
                result.is_err(),
                "a plain Client must never reach the extension surface"
            );
            Ok(())
        })
    })
    .await;
    assert!(outcome.is_ok(), "plain-client scenario failed: {outcome:?}");
    host.child.kill().await?;
    Ok(())
}
