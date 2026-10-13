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
    pub background: echo_agent::agent::subagent::BackgroundSubagentHandle,
    /// Control identity of the bound attempt.
    pub attempt: u32,
    /// Memoized terminal result (a background join is single-consumption).
    pub settled: tokio::sync::Mutex<
        Option<echo_agent::error::Result<echo_agent::agent::subagent::SubagentResult>>,
    >,
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
    pub async fn wait_settled(&self) {
        loop {
            // Subscribe before reading so a terminal write between the read
            // and wait cannot be lost.
            let notified = self.settled_notify.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            if self.settled.lock().await.is_some() {
                return;
            }
            notified.await;
        }
    }

    fn collect_result(record: Arc<Self>) {
        tokio::spawn(async move {
            let result = record.background.join().await;
            *record.settled.lock().await = Some(result);
            record.settled_notify.notify_waiters();
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
            if state.facade.subagent_of(&receipt.handle.id).is_none() {
                fail!(subagent_error(
                    ExtensionErrorCode::ClosedHandle,
                    "idempotent subagent dispatch result has been retired",
                    method
                ));
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
        cancel,
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
    let executor = authorities.subagent_executor.clone();
    let dispatched = tokio::select! {
        result = executor.dispatch_background_attempt(dispatch, identity) => result,
        _ = session.cancel_operations.cancelled() => fail!(subagent_error(ExtensionErrorCode::ClosedHandle, "subagent owner Session is closing", method)),
    };
    let record = match dispatched {
        Ok(background) => Arc::new(SubagentDispatchRecord {
            executor,
            background,
            attempt: 1,
            settled: tokio::sync::Mutex::new(None),
            settled_notify: tokio::sync::Notify::new(),
            owner_session: acp_session_id,
        }),
        Err(error) => {
            fail!(wire::framework_error(&error, method));
        }
    };
    let handle = WireHandle {
        id: record.background.execution_id.clone(),
        generation: WireU64::from_u64(state.handles.generation()),
        kind: HandleKind::Subagent,
    };
    // The advertised live-subagent bound is a hard limit: when the record
    // cannot be admitted, the attempt is cancelled so no dispatch runs
    // without a control identity (never a leaked background subagent).
    if let Err(error) = state
        .facade
        .register_subagent(handle.id.clone(), record.clone())
    {
        record.background.cancel();
        SubagentDispatchRecord::collect_result(record.clone());
        fail!(error);
    }
    SubagentDispatchRecord::collect_result(record.clone());
    if let Some(id) = request.idempotency_id {
        receipts.insert(
            id,
            SubagentDispatchReceipt {
                request: request.request,
                handle: handle.clone(),
            },
        );
    }
    drop(receipts);
    let published = responder.respond(SubagentDispatchResponse { subagent: handle });
    if published.is_err() {
        record.background.cancel();
        record.wait_settled().await;
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
    if tokio::time::timeout_at(deadline, record.wait_settled())
        .await
        .is_err()
    {
        return responder.respond(SubagentAwaitResponse {
            settled: false,
            result: None,
        });
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
    let execution_id = record.background.execution_id.clone();
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
            record.background.cancel();
            Ok(true)
        }
    };
    match accepted {
        Ok(accepted) => responder.respond(SubagentControlResponse { accepted }),
        Err(error) => {
            fail!(subagent_error(
                ExtensionErrorCode::FrameworkError,
                wire::bounded_framework_message(&error.to_string()),
                method,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DispatchInput;

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
