//! Subagent facade handlers (plan 07, todo 3).
//!
//! `_echo_agent/subagent/dispatch|await|control` share the Session Agent's
//! own [`SubagentExecutor`] — captured before boxing (see
//! [`crate::factory::SessionAuthorityServices`]) — so RPC dispatches, the
//! in-conversation delegation tools and task execution share one control
//! plane. Control verbs are exactly the executor's real semantics
//! (message / guidance / interrupt / cancel); there is deliberately no
//! pause/resume.
//!
//! Dispatches run as controlled background attempts with Host-minted
//! [`SubagentAttemptIdentity`]s: the Subagent handle id is the execution id,
//! and every control verb addresses that exact live attempt. Results are
//! memoized per record because a background join is single-consumption.

use agent_client_protocol::{Client, ConnectionTo, Responder};
use echo_agent::agent::subagent::{DispatchRequest, SubagentAttemptIdentity, SubagentExecutor};
use echo_sdk_protocol::capability::ExtensionCapability;
use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::handle::{HandleKind, WireHandle};
use echo_sdk_protocol::methods::{
    SubagentAwaitRequest, SubagentAwaitResponse, SubagentControlAction, SubagentControlRequest,
    SubagentControlResponse, SubagentDispatchRequest, SubagentDispatchResponse,
};
use echo_sdk_protocol::scalar::{WireU64, WireValue};
use std::sync::Arc;

use super::super::handler::{require_capability, require_extended};
use super::super::state::CoreProfileState;
use super::super::wire;

/// One live RPC-dispatched subagent attempt. Holds its own executor clone so
/// control and await never depend on the Session staying open for identity
/// resolution (settlement still follows the framework's own semantics).
pub(crate) struct SubagentDispatchRecord {
    pub executor: Arc<SubagentExecutor>,
    /// Admission reserves this same record before the framework may construct
    /// an Agent. One original setup/collector fills the cell; retries only wait.
    pub background: tokio::sync::OnceCell<
        Result<echo_agent::agent::subagent::BackgroundSubagentHandle, EchoSdkError>,
    >,
    pub execution_id: String,
    pub cancel: tokio_util::sync::CancellationToken,
    /// Control identity of the bound attempt.
    pub attempt: u32,
    /// Memoized terminal result (a background join is single-consumption).
    pub settled: tokio::sync::Mutex<
        Option<echo_agent::error::Result<echo_agent::agent::subagent::SubagentResult>>,
    >,
    /// A local join may report nonterminal knowledge. This remains on the
    /// original record, consumes quota and never becomes a terminal result.
    pub observation: tokio::sync::Mutex<Option<EchoSdkError>>,
    /// Result collection outlives any individual RPC deadline.
    pub settled_notify: tokio::sync::Notify,
    /// ACP session that owns the dispatch (records are session-scoped;
    /// session close cascades through it in the family closeout).
    pub owner_session: String,
}

pub(crate) struct SubagentDispatchReceipt {
    pub request: WireValue,
    pub handle: WireHandle,
}

/// The accepted RPC data fields of DispatchRequest. Process-local resources
/// (a parent Store and cancellation token) cannot be recreated from JSON.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct DispatchInput {
    agent_name: String,
    task: String,
    mode_override: Option<echo_agent::agent::subagent::ExecutionMode>,
    parent_agent: Option<String>,
    parent_context: Option<serde_json::Value>,
    delegation_policy: Option<echo_agent::tasks::NestedDelegationPolicy>,
    runtime_context: Option<serde_json::Value>,
    message: Option<echo_sdk_protocol::methods::LlmMessageWire>,
    prompt_payload: Option<serde_json::Value>,
    prompt_context: Option<serde_json::Value>,
    #[serde(default)]
    constraints: Vec<String>,
    background: Option<bool>,
}

impl SubagentDispatchRecord {
    async fn wait_ready(&self) -> Result<(), EchoSdkError> {
        loop {
            let notified = self.settled_notify.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            if let Some(background) = self.background.get() {
                return background.as_ref().map(|_| ()).map_err(Clone::clone);
            }
            notified.await;
        }
    }

    /// This original record owns both setup and the single-consumption join.
    /// Dropping an RPC waiter never drops a submitted construction future.
    fn dispatch_original(
        record: Arc<Self>,
        state: Arc<CoreProfileState>,
        dispatch: DispatchRequest,
        identity: SubagentAttemptIdentity,
    ) {
        tokio::spawn(async move {
            match record
                .executor
                .dispatch_background_attempt(dispatch, identity)
                .await
            {
                Ok(background) => {
                    if record.background.set(Ok(background)).is_err() {
                        // No second caller can initialize this private cell.
                        // Retain admission if that invariant is ever violated.
                        *record.observation.lock().await = Some(wire::not_settled_error(
                            &record.owner_session,
                            Some(&record.execution_id),
                            "_echo_agent/subagent/dispatch",
                        ));
                        record.settled_notify.notify_waiters();
                        return;
                    }
                    record.settled_notify.notify_waiters();
                    record.collect_original_result().await;
                }
                Err(error) => {
                    let observer = wire::framework_error(&error, "_echo_agent/subagent/dispatch");
                    let unknown = matches!(
                        error,
                        echo_agent::error::ReactError::ExecutionNotSettled { .. }
                    );
                    let _ = record.background.set(Err(observer.clone()));
                    if unknown {
                        *record.observation.lock().await = Some(observer);
                    } else {
                        // The framework returns Err from dispatch only before
                        // spawning its attempt. Only that proven pre-effect
                        // admission failure may retire the reserved SDK slot.
                        *record.settled.lock().await = Some(Err(error));
                        state
                            .facade
                            .retire_unsubmitted_subagent(&record.execution_id, &record);
                    }
                    record.settled_notify.notify_waiters();
                }
            }
        });
    }

    async fn collect_original_result(&self) {
        let Some(Ok(background)) = self.background.get() else {
            return;
        };
        let result = background.join().await;
        match result {
            Err(error @ echo_agent::error::ReactError::ExecutionNotSettled { .. }) => {
                *self.observation.lock().await =
                    Some(wire::framework_error(&error, "_echo_agent/subagent/await"));
            }
            terminal => *self.settled.lock().await = Some(terminal),
        }
        self.settled_notify.notify_waiters();
    }

    pub async fn wait_settled(&self) -> Result<(), EchoSdkError> {
        loop {
            // Subscribe before reading so a terminal write between the read
            // and wait cannot be lost.
            let notified = self.settled_notify.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            if let Some(observation) = self.observation.lock().await.as_ref() {
                return Err(observation.clone());
            }
            if self.settled.lock().await.is_some() {
                return Ok(());
            }
            notified.await;
        }
    }

    #[cfg(test)]
    fn collect_result(record: Arc<Self>) {
        tokio::spawn(async move {
            record.collect_original_result().await;
        });
    }
}

fn subagent_error(
    code: ExtensionErrorCode,
    message: impl Into<String>,
    method: &str,
) -> EchoSdkError {
    wire::sdk_error(code, message, Retryability::Never, method)
}

fn record_of(
    state: &CoreProfileState,
    subagent: &WireHandle,
    method: &str,
) -> Result<Arc<SubagentDispatchRecord>, EchoSdkError> {
    state
        .handles
        .check_shape_and_generation(subagent, HandleKind::Subagent, method)?;
    let record = state.facade.subagent_of(&subagent.id).ok_or_else(|| {
        subagent_error(
            ExtensionErrorCode::InvalidValue,
            "subagent handle was never issued by this Host",
            method,
        )
    })?;
    let Some(owner) = state.handles.session_handle_for_acp(&record.owner_session) else {
        return Err(subagent_error(
            ExtensionErrorCode::ClosedHandle,
            "subagent owner Session is no longer live",
            method,
        ));
    };
    let owner = state.handles.session(&owner)?;
    if owner.closing.load(std::sync::atomic::Ordering::Acquire) {
        return Err(subagent_error(
            ExtensionErrorCode::ClosedHandle,
            "subagent owner Session is closing",
            method,
        ));
    }
    Ok(record)
}

pub(crate) async fn subagent_dispatch(
    state: Arc<CoreProfileState>,
    request: SubagentDispatchRequest,
    responder: Responder<SubagentDispatchResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/subagent/dispatch";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::Subagents, method)?;
    macro_rules! fail {
        ($error:expr) => {
            return responder.respond_with_error(wire::into_jsonrpc_error($error))
        };
    }
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.session, HandleKind::Session, method)
    {
        fail!(error);
    }
    let session = match state.handles.session(&request.session) {
        Ok(record) => record,
        Err(error) => fail!(error),
    };
    let _session_permit = session.lifecycle.read().await;
    if session.closing.load(std::sync::atomic::Ordering::Acquire) {
        fail!(subagent_error(
            ExtensionErrorCode::ClosedHandle,
            "subagent owner Session is closing",
            method
        ));
    }
    let acp_session_id = session.acp_session_id.clone();
    let Some(authorities) = state.session_factory.session_services(&acp_session_id) else {
        fail!(subagent_error(
            ExtensionErrorCode::ClosedHandle,
            "session authority services are no longer live",
            method,
        ));
    };
    if request
        .idempotency_id
        .as_ref()
        .is_some_and(|id| id.trim().is_empty() || id.chars().count() > 256)
    {
        fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            "idempotency_id must be non-empty and at most 256 characters",
            method
        ));
    }
    // Serialize receipt admission through publication. Concurrent retries
    // with the same identity must never start a second background attempt.
    let mut receipts = tokio::select! {
        receipts = authorities.subagent_dispatch_receipts.lock() => receipts,
        _ = session.cancel_operations.cancelled() => fail!(subagent_error(ExtensionErrorCode::ClosedHandle, "subagent owner Session is closing", method)),
    };
    if let Some(id) = request.idempotency_id.as_ref() {
        if let Some(receipt) = receipts.get(id) {
            if receipt.request != request.request {
                fail!(subagent_error(
                    ExtensionErrorCode::InvalidValue,
                    "idempotency id was already used with a different subagent request",
                    method
                ));
            }
            let Some(original) = state.facade.subagent_of(&receipt.handle.id) else {
                fail!(subagent_error(
                    ExtensionErrorCode::ClosedHandle,
                    "idempotent subagent dispatch result has been retired",
                    method
                ));
            };
            if let Err(error) = original.wait_ready().await {
                fail!(error);
            }
            return responder.respond(SubagentDispatchResponse {
                subagent: receipt.handle.clone(),
            });
        }
        if receipts.len() >= state.limits.max_open_handles {
            fail!(wire::sdk_error(
                ExtensionErrorCode::PayloadTooLarge,
                "session subagent idempotency receipt limit reached",
                Retryability::Never,
                method
            ));
        }
    }
    let spec = match request.request.clone().into_json() {
        Ok(value) => value,
        Err(error) => {
            fail!(subagent_error(
                ExtensionErrorCode::InvalidValue,
                format!("subagent request is not a lossless wire value: {error}"),
                method,
            ));
        }
    };
    if let Some(policy) = spec
        .get("delegation_policy")
        .and_then(serde_json::Value::as_object)
        && policy.keys().any(|field| {
            ![
                "can_spawn_subagents",
                "delegate_depth",
                "max_delegate_depth",
            ]
            .contains(&field.as_str())
        })
    {
        fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            "delegation_policy contains an unknown field",
            method
        ));
    }
    let input = match serde_json::from_value::<DispatchInput>(spec) {
        Ok(input) => input,
        Err(error) => fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            format!("subagent dispatch input is malformed or unsupported: {error}"),
            method
        )),
    };
    if input.agent_name.trim().is_empty() {
        fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            "subagent dispatch requires a non-empty agent_name",
            method,
        ));
    };
    if input.task.trim().is_empty() {
        fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            "subagent dispatch requires a non-empty task",
            method,
        ));
    };
    if input
        .parent_context
        .as_ref()
        .is_some_and(|value| !value.is_null())
    {
        fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            "parent_context contains process-local resources and is not accepted on the dispatch RPC; use message and prompt_context",
            method
        ));
    }
    if input.background == Some(false) {
        fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            "subagent dispatch RPC always creates a background attempt",
            method
        ));
    }
    let mut runtime_context =
        match super::source_operations::runtime_context_from_json(input.runtime_context) {
            Ok(context) => context.unwrap_or_default(),
            Err(error) => fail!(error.with_operation(method)),
        };
    if runtime_context.execution_id.is_some()
        || runtime_context
            .run_id
            .as_ref()
            .is_some_and(|run| run != &acp_session_id)
    {
        fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            "dispatch execution identity and Session run scope are owned by the Host",
            method
        ));
    }
    let prompt_context =
        match super::source_operations::prompt_context_from_json(input.prompt_context) {
            Ok(context) => context,
            Err(error) => fail!(error.with_operation(method)),
        };
    let message = match input.message.map(wire::message_from_wire).transpose() {
        Ok(message) => message,
        Err(error) => fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            error,
            method
        )),
    };

    let identity = match SubagentAttemptIdentity::new(
        format!("sdk-task-{}", uuid::Uuid::new_v4()),
        format!("sdk-exec-{}", uuid::Uuid::new_v4()),
        1,
    ) {
        Ok(identity) => identity,
        Err(error) => fail!(wire::framework_error(
            &echo_agent::error::ReactError::Other(error.to_string()),
            method
        )),
    };
    let cancel = tokio_util::sync::CancellationToken::new();
    runtime_context.run_id = Some(acp_session_id.clone());
    runtime_context.execution_id = Some(identity.execution_id.clone());
    runtime_context.cancel = Some(Arc::new(cancel.clone()));
    let dispatch = DispatchRequest {
        agent_name: input.agent_name,
        task: input.task,
        mode_override: input.mode_override,
        cancel: cancel.clone(),
        parent_agent: input.parent_agent.unwrap_or_else(|| "sdk-host".to_string()),
        parent_context: None,
        delegation_policy: input.delegation_policy.unwrap_or_default(),
        runtime_context: Some(runtime_context),
        message,
        prompt_payload: input.prompt_payload,
        prompt_context,
        constraints: input.constraints,
        background: true,
    };
    let handle = WireHandle {
        id: identity.execution_id.clone(),
        generation: WireU64::from_u64(state.handles.generation()),
        kind: HandleKind::Subagent,
    };
    let record = Arc::new(SubagentDispatchRecord {
        executor: authorities.subagent_executor.clone(),
        background: tokio::sync::OnceCell::new(),
        execution_id: handle.id.clone(),
        cancel,
        attempt: 1,
        settled: tokio::sync::Mutex::new(None),
        observation: tokio::sync::Mutex::new(None),
        settled_notify: tokio::sync::Notify::new(),
        owner_session: acp_session_id,
    });
    // Admission and quota checking are atomic in the original SDK map.
    // A rejected reservation has not invoked any framework constructor/Agent.
    if let Err(error) = state
        .facade
        .register_subagent(handle.id.clone(), record.clone())
    {
        fail!(error);
    }
    if let Some(id) = request.idempotency_id {
        receipts.insert(
            id,
            SubagentDispatchReceipt {
                request: request.request,
                handle: handle.clone(),
            },
        );
    }
    // Bind idempotency before the first await or setup side effect. A retry
    // resolves this exact reservation, never a replacement execution.
    SubagentDispatchRecord::dispatch_original(record.clone(), state.clone(), dispatch, identity);
    drop(receipts);
    if let Err(error) = record.wait_ready().await {
        fail!(error);
    }
    let published = responder.respond(SubagentDispatchResponse { subagent: handle });
    if published.is_err() {
        record.cancel.cancel();
        let _ = record.wait_settled().await;
    }
    published
}

pub(crate) async fn subagent_await(
    state: Arc<CoreProfileState>,
    request: SubagentAwaitRequest,
    responder: Responder<SubagentAwaitResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/subagent/await";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::Subagents, method)?;
    macro_rules! fail {
        ($error:expr) => {
            return responder.respond_with_error(wire::into_jsonrpc_error($error))
        };
    }
    let record = match record_of(&state, &request.subagent, method) {
        Ok(record) => record,
        Err(error) => fail!(error),
    };
    let timeout = match request.timeout.as_ref() {
        Some(duration) => {
            if let Err(error) = duration.validate() {
                fail!(subagent_error(
                    ExtensionErrorCode::InvalidValue,
                    error.to_string(),
                    method
                ));
            }
            let seconds = match duration.seconds.to_u64() {
                Some(seconds) => seconds,
                None => fail!(subagent_error(
                    ExtensionErrorCode::InvalidValue,
                    "subagent await seconds are outside the u64 wire range",
                    method
                )),
            };
            std::time::Duration::new(seconds, duration.nanos)
        }
        None => std::time::Duration::from_secs(30),
    };
    let Some(deadline) = tokio::time::Instant::now().checked_add(timeout) else {
        fail!(subagent_error(
            ExtensionErrorCode::InvalidValue,
            "subagent await timeout exceeds clock range",
            method
        ));
    };
    match tokio::time::timeout_at(deadline, record.wait_settled()).await {
        Err(_) => {
            return responder.respond(SubagentAwaitResponse {
                settled: false,
                result: None,
            });
        }
        Ok(Err(error)) => fail!(
            error
                .with_operation(method)
                .with_handle(request.subagent.clone())
        ),
        Ok(Ok(())) => {}
    }
    let settled = record.settled.lock().await;
    match settled.as_ref() {
        Some(Ok(result)) => {
            let projection = serde_json::json!({
                "agent_name": result.agent_name,
                "output": result.output,
                "status": result.outcome.status.as_str(),
                "summary": result.outcome.summary,
                "iterations": result.iterations,
                "duration": { "seconds": result.duration.as_secs(), "nanos": result.duration.subsec_nanos() },
                "tokens_used": result.tokens_used,
                "was_truncated": result.was_truncated,
                "mode": result.mode,
                "isolation_observed": result.isolation_observed,
                "llm_usage": result.llm_usage,
                "outcome": result.outcome,
            });
            match WireValue::from_json(projection) {
                Ok(value) => responder.respond(SubagentAwaitResponse {
                    settled: true,
                    result: Some(value),
                }),
                Err(error) => {
                    fail!(subagent_error(
                        ExtensionErrorCode::SerializationViolation,
                        format!("subagent result is not a lossless wire value: {error}"),
                        method,
                    ));
                }
            }
        }
        Some(Err(error)) => fail!(wire::framework_error(error, method)),
        None => responder.respond(SubagentAwaitResponse {
            settled: false,
            result: None,
        }),
    }
}

pub(crate) async fn subagent_control(
    state: Arc<CoreProfileState>,
    request: SubagentControlRequest,
    responder: Responder<SubagentControlResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/subagent/control";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::Subagents, method)?;
    macro_rules! fail {
        ($error:expr) => {
            return responder.respond_with_error(wire::into_jsonrpc_error($error))
        };
    }
    let record = match record_of(&state, &request.subagent, method) {
        Ok(record) => record,
        Err(error) => fail!(error),
    };
    if record.settled.lock().await.is_some() {
        return responder.respond(SubagentControlResponse { accepted: false });
    }
    if let Err(error) = record.wait_ready().await {
        fail!(error.with_handle(request.subagent.clone()));
    }
    let execution_id = record.execution_id.clone();
    let attempt = record.attempt;
    // Message and guidance carry text; interrupt and cancel address the
    // attempt itself and take no payload.
    let payload = match request.action {
        SubagentControlAction::Message | SubagentControlAction::Guidance => {
            match request
                .payload
                .clone()
                .filter(|text| !text.trim().is_empty())
            {
                Some(payload) => payload,
                None => fail!(subagent_error(
                    ExtensionErrorCode::InvalidValue,
                    "message and guidance control require a non-empty payload",
                    method,
                )),
            }
        }
        SubagentControlAction::Interrupt | SubagentControlAction::Cancel => String::new(),
    };
    let accepted = match request.action {
        SubagentControlAction::Message => record
            .executor
            .send_message_tracked(&execution_id, attempt, payload)
            .await
            .map(|_| true),
        // The accepted RPC contract targets the next planning safe point of
        // this attempt, so use live steering rather than a future-attempt
        // queue that no subsequent RPC dispatch would ever consume.
        SubagentControlAction::Guidance => record
            .executor
            .send_message_tracked(&execution_id, attempt, payload)
            .await
            .map(|_| true),
        SubagentControlAction::Interrupt => record
            .executor
            .interrupt_subagent(&execution_id, attempt)
            .await
            .map(|outcome| outcome.settled),
        SubagentControlAction::Cancel => {
            record.cancel.cancel();
            Ok(true)
        }
    };
    match accepted {
        Ok(accepted) => responder.respond(SubagentControlResponse { accepted }),
        Err(error) => {
            let error = match &error {
                echo_agent::agent::subagent::SubagentControlError::ExecutionNotSettled {
                    run_id,
                    execution_id,
                } => wire::not_settled_error(run_id, Some(execution_id), method),
                _ => subagent_error(
                    ExtensionErrorCode::FrameworkError,
                    wire::bounded_framework_message(&error.to_string()),
                    method,
                ),
            };
            fail!(error.with_handle(request.subagent.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DispatchInput;

    /// This is a real in-process ACP/profile RPC test, not a public Host
    /// configuration: global handles remain ample, original facade capacity=1.
    #[cfg(feature = "sdk-extension-bridge")]
    #[tokio::test]
    async fn full_rpc_admission_precedes_factory_and_execution_and_reopens_after_settlement()
    -> Result<(), String> {
        use agent_client_protocol::{
            Agent as AcpAgent, ByteStreams, Client, ConnectionTo, Responder,
        };
        use echo_sdk_protocol::methods::*;
        use echo_sdk_protocol::scalar::WireValue;
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};
        use tokio_util::sync::CancellationToken;
        let root = tempfile::tempdir().map_err(|error| error.to_string())?;
        let config:crate::SdkHostConfig=serde_json::from_value(serde_json::json!({"schema_version":1,"default_agent":{
            "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
            "agent":{"name":"fixture","enable_tools":true}}})).map_err(|error| error.to_string())?;
        let prepared = config.prepare().map_err(|error| error.to_string())?;
        let state = super::CoreProfileState::install_for_subagent_admission_test(
            root.path(),
            crate::config::SdkProfileLimits {
                max_open_handles: 64,
                ..Default::default()
            },
            prepared.framework,
            prepared.llm_client,
            1,
        )
        .map_err(|error| error.to_string())?;
        let profile = super::super::super::SdkCoreProfile {
            state: state.clone(),
        };
        let adapter = echo_agent::acp::AcpAgentAdapter::new(state.session_factory.clone())
            .with_profile(profile);
        let close_owner = adapter.close_owner();
        let constructors = Arc::new(AtomicUsize::new(0));
        let callback_constructors = constructors.clone();
        let executions = Arc::new(AtomicUsize::new(0));
        let callback_executions = executions.clone();
        let entered = CancellationToken::new();
        let callback_entered = entered.clone();
        let release = CancellationToken::new();
        let callback_release = release.clone();
        let release_after = release.clone();
        let (host_io, client_io) = tokio::io::duplex(32_768);
        let (host_read, host_write) = tokio::io::split(host_io);
        let (client_read, client_write) = tokio::io::split(client_io);
        let host = agent_client_protocol::ConnectTo::connect_to(
            adapter,
            ByteStreams::new(host_write.compat_write(), host_read.compat()),
        );
        let client=Client.builder().on_receive_request(async move |call:ExtensionInvokeCall,responder:Responder<ExtensionInvokeOutcome>,connection:ConnectionTo<AcpAgent>| {
            match call.invocation {
                ExtensionInvocation::FactoryCreateAgent(input)=>{
                    callback_constructors.fetch_add(1,Ordering::AcqRel);
                    responder.respond(ExtensionInvokeOutcome::Result{result:ExtensionResult::FactoryCreateAgent(CustomAgentDescriptorWire{
                        name:input.name,model_name:"fixture".to_string(),system_prompt:"original admission".to_string(),tool_names:Vec::new()})})
                }
                ExtensionInvocation::AgentExecuteStream(_)=>{
                    callback_executions.fetch_add(1,Ordering::AcqRel);
                    let stream=call.stream.ok_or_else(|| agent_client_protocol::Error::internal_error().data("missing original stream"))?;
                    responder.respond(ExtensionInvokeOutcome::Stream{stream:stream.clone()})?;
                    let release=callback_release.clone();let entered=callback_entered.clone();let producer=connection.clone();
                    connection.spawn(async move {
                        entered.cancel();release.cancelled().await;
                        producer.send_notification(ExtensionStreamEvent::Complete {stream,sequence:echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string()).map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?,
                            value:ExtensionStreamCompleteValue::Agent(AgentStreamTerminalWire::FinalAnswer{text:"original settled".to_string()})})
                    })
                }
                ExtensionInvocation::AgentClose(_)=>responder.respond(ExtensionInvokeOutcome::Result{result:ExtensionResult::AgentClose(ExtensionUnit)}),
                _=>Err(agent_client_protocol::Error::internal_error().data("unexpected quota test callback")),
            }
        },agent_client_protocol::on_receive_request!()).connect_with(ByteStreams::new(client_write.compat_write(),client_read.compat()),async move |connection| {
            let _release_on_exit=release.clone().drop_guard();
            let mut initialize=agent_client_protocol::schema::v1::InitializeRequest::new(agent_client_protocol::schema::ProtocolVersion::V1);
            let mut meta=agent_client_protocol::schema::v1::Meta::new();
            meta.insert("echo_agent".to_string(),serde_json::to_value(echo_sdk_protocol::capability::EchoAgentClientHello {
                extension_protocol_version:echo_sdk_protocol::EXTENSION_PROTOCOL_VERSION,contract_digest:state.advertisement.contract_digest.clone(),source_contract_digest:state.advertisement.source_contract_digest.clone(),required_features:Vec::new(),required_capabilities:Vec::new()
            }).map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?);
            initialize.client_capabilities.meta=Some(meta);connection.send_request(initialize).block_task().await?;
            let agent=connection.send_request(AgentCreateRequest {config:AgentConfigWire::HostDefault,idempotency_id:None}).block_task().await?.agent;
            for name in ["quota-original","quota-rejected"] {
                connection.send_request(ExtensionRegisterRequest {kind:ExtensionKind::AgentFactory,implementation_id:name.to_string(),descriptor:ExtensionDescriptor::AgentFactory{descriptor_version:1},timeout:None}).block_task().await?;
            }
            let session=connection.send_request(SessionCreateRequest {agent,working_dir:None,session_id:None,idempotency_id:None}).block_task().await?;
            let original_request=SubagentDispatchRequest {session:session.session.clone(),request:WireValue::from_json(serde_json::json!({"agent_name":"quota-original","task":"held original"})).map_err(|error| agent_client_protocol::Error::invalid_params().data(error.to_string()))?,idempotency_id:Some("original-receipt".to_string())};
            let first=connection.send_request(original_request.clone()).block_task().await?.subagent;
            entered.cancelled().await;
            assert_eq!(constructors.load(Ordering::Acquire),1);assert_eq!(executions.load(Ordering::Acquire),1);
            let retry=connection.send_request(original_request).block_task().await?.subagent;
            assert_eq!(retry,first,"same receipt started another execution");
            let rejected=connection.send_request(SubagentDispatchRequest {session:session.session.clone(),request:WireValue::from_json(serde_json::json!({"agent_name":"quota-rejected","task":"must never construct or execute"})).map_err(|error| agent_client_protocol::Error::invalid_params().data(error.to_string()))?,idempotency_id:None}).block_task().await;
            let error=rejected.err().ok_or_else(|| agent_client_protocol::Error::internal_error().data("full original map admitted a second attempt"))?;
            let error=echo_sdk_protocol::error::EchoSdkError::from_jsonrpc_data(error.data.as_ref()).map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?;
            assert_eq!(error.code,echo_sdk_protocol::error::ExtensionErrorCode::PayloadTooLarge);
            assert_eq!(error.operation.as_deref(),Some("_echo_agent/subagent/dispatch"));
            assert_eq!(constructors.load(Ordering::Acquire),1,"rejected RPC constructed a child");assert_eq!(executions.load(Ordering::Acquire),1,"rejected RPC executed a child");
            release.cancel();
            assert!(connection.send_request(SubagentAwaitRequest {subagent:first,timeout:None}).block_task().await?.settled);
            assert_eq!(constructors.load(Ordering::Acquire),1);assert_eq!(executions.load(Ordering::Acquire),1);
            let fresh=connection.send_request(SubagentDispatchRequest {session:session.session.clone(),request:WireValue::from_json(serde_json::json!({"agent_name":"quota-rejected","task":"admitted after original physical settlement"})).map_err(|error| agent_client_protocol::Error::invalid_params().data(error.to_string()))?,idempotency_id:None}).block_task().await?.subagent;
            assert!(connection.send_request(SubagentAwaitRequest {subagent:fresh,timeout:None}).block_task().await?.settled);
            assert_eq!(constructors.load(Ordering::Acquire),2);assert_eq!(executions.load(Ordering::Acquire),2);
            assert!(connection.send_request(SessionCloseRequest {session:session.session}).block_task().await?.released);
            Ok(())
        });
        let outcome = tokio::time::timeout(std::time::Duration::from_secs(10), async {
            tokio::join!(host, client)
        })
        .await;
        release_after.cancel();
        let (host, client) =
            outcome.map_err(|_| "original admission real ACP test timed out".to_string())?;
        host.map_err(|error| error.to_string())?;
        client.map_err(|error| error.to_string())?;
        drop(close_owner);
        Ok(())
    }

    #[tokio::test]
    async fn background_unknown_retains_original_sdk_record_and_capacity() -> Result<(), String> {
        use super::{
            DispatchRequest, SubagentAttemptIdentity, SubagentDispatchRecord, SubagentExecutor,
        };
        use echo_agent::agent::subagent::{
            FnAgentFactory, SubagentDefinition, SubagentExecutorConfig, SubagentRegistry,
        };
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        for unknown in [true, false] {
            let calls = Arc::new(AtomicUsize::new(0));
            let registry = Arc::new(SubagentRegistry::new());
            let factory_calls = calls.clone();
            let factory = FnAgentFactory::new(move || -> futures::future::BoxFuture<'static, echo_agent::error::Result<Box<dyn echo_agent::agent::Agent>>> {
                let calls = factory_calls.clone();
                Box::pin(async move {
                    calls.fetch_add(1, Ordering::AcqRel);
                    Err(if unknown { echo_agent::error::ReactError::ExecutionNotSettled {
                        run_id: "original-run-中文🙂".to_string(), execution_id: Some("original-execution".to_string()),
                    } } else { echo_agent::error::ReactError::Other("genuine completed constructor error".to_string()) })
                })
            });
            assert!(registry.register_factory_sync(
                SubagentDefinition::new("original", "controlled original factory"),
                Arc::new(factory)
            ));
            let executor = Arc::new(SubagentExecutor::new(
                registry,
                SubagentExecutorConfig::default(),
            ));
            let background = executor
                .dispatch_background_attempt(
                    DispatchRequest {
                        agent_name: "original".to_string(),
                        task: "original dispatch".to_string(),
                        mode_override: None,
                        cancel: tokio_util::sync::CancellationToken::new(),
                        parent_agent: "sdk-collector-test".to_string(),
                        parent_context: None,
                        delegation_policy: Default::default(),
                        runtime_context: None,
                        message: None,
                        prompt_payload: None,
                        prompt_context: None,
                        constraints: Vec::new(),
                        background: true,
                    },
                    SubagentAttemptIdentity::new("controlled-task", "controlled-execution", 1)
                        .map_err(|error| error.to_string())?,
                )
                .await
                .map_err(|error| error.to_string())?;
            let execution = background.execution_id.clone();
            let ready = tokio::sync::OnceCell::new();
            ready
                .set(Ok(background))
                .map_err(|_| "test initialized background twice")?;
            let record = Arc::new(SubagentDispatchRecord {
                executor,
                background: ready,
                execution_id: execution.clone(),
                cancel: tokio_util::sync::CancellationToken::new(),
                attempt: 1,
                settled: tokio::sync::Mutex::new(None),
                observation: tokio::sync::Mutex::new(None),
                settled_notify: tokio::sync::Notify::new(),
                owner_session: "owner-session".to_string(),
            });
            let facade =
                super::super::SessionFacadeRuntime::new(&crate::config::SdkProfileLimits {
                    max_open_handles: 1,
                    ..Default::default()
                });
            facade
                .register_subagent(execution.clone(), record.clone())
                .map_err(|error| error.message)?;
            SubagentDispatchRecord::collect_result(record.clone());
            let result =
                tokio::time::timeout(std::time::Duration::from_secs(2), record.wait_settled())
                    .await
                    .map_err(|error| error.to_string())?;
            assert_eq!(calls.load(Ordering::Acquire), 1);
            if unknown {
                let error = result.err().ok_or("unknown became settled")?;
                assert!(
                    matches!(super::wire::observation_from_sdk_error(&error), Some(echo_agent::error::ReactError::ExecutionNotSettled { run_id, execution_id }) if run_id == "original-run-中文🙂" && execution_id.as_deref() == Some("original-execution"))
                );
                assert!(record.settled.lock().await.is_none());
                assert!(
                    facade
                        .register_subagent("competing-slot".to_string(), record.clone())
                        .is_err()
                );
                assert!(facade.cancel_subagents_of("owner-session").await.is_err());
                assert!(
                    facade
                        .subagent_of(&execution)
                        .is_some_and(|retained| Arc::ptr_eq(&retained, &record))
                );
                assert_eq!(
                    calls.load(Ordering::Acquire),
                    1,
                    "retry must not construct or dispatch again"
                );
            } else {
                result.map_err(|error| error.message)?;
                assert!(record.settled.lock().await.is_some());
                facade
                    .cancel_subagents_of("owner-session")
                    .await
                    .map_err(|error| error.message)?;
                assert!(facade.subagent_of(&execution).is_none());
            }
        }
        Ok(())
    }

    #[test]
    fn dispatch_fields_are_typed_and_unknown_fields_are_rejected() -> Result<(), String> {
        let accepted: DispatchInput = serde_json::from_value(serde_json::json!({
            "agent_name": "检索 🧪", "task": "检查运行", "mode_override": "fork",
            "constraints": ["只读"],
            "delegation_policy": { "can_spawn_subagents": true, "delegate_depth": 1, "max_delegate_depth": 3 },
            "prompt_context": { "acceptance_criteria": ["证据"] },
            "runtime_context": { "conversation_id": "会话" },
            "background": true,
        })).map_err(|error| error.to_string())?;
        assert_eq!(accepted.constraints, vec!["只读"]);
        assert_eq!(
            accepted.mode_override,
            Some(echo_agent::agent::subagent::ExecutionMode::Fork)
        );
        assert!(
            accepted
                .delegation_policy
                .is_some_and(|policy| policy.can_spawn_subagents && policy.delegate_depth == 1)
        );
        for rejected in [
            serde_json::json!({"agent_name":"a","task":"t","constraints":[1]}),
            serde_json::json!({"agent_name":"a","task":"t","unknown":true}),
            serde_json::json!({"agent_name":"a","task":"t","mode_override":"unsupported"}),
        ] {
            assert!(serde_json::from_value::<DispatchInput>(rejected).is_err());
        }
        let context =
            super::super::source_operations::prompt_context_from_json(accepted.prompt_context)
                .map_err(|error| error.message)?
                .ok_or_else(|| "prompt_context disappeared during conversion".to_string())?;
        assert_eq!(context.acceptance_criteria, vec!["证据"]);
        Ok(())
    }
}
