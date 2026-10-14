//! Task execution runtime for the facade (plan 07, todo 3).
//!
//! [`FacadeTaskController`] implements the framework's
//! [`RuntimeDagController`] over the Session's own task revision service and
//! [`SubagentExecutor`]: snapshots, claims, retries, interruptions and
//! terminals all settle through the canonical store mutations — the Host
//! never recomputes a frontier, claim or terminal. Dispatch sends each
//! claimed PlanTask to the subagent named by the task extension
//! (`{"subagent": "<name>"}`); the dispatch prompt carries completed
//! dependency outputs like the team runtime does.
//!
//! `task/execute` drives the graph through
//! [`echo_agent::tasks::RuntimeTaskService`] on the official connection
//! task and answers with the TaskRun handle immediately; terminals are
//! observed through `task/list`. `task/control` settles one exact task
//! claim through the same store: pause keeps the task resumable, cancel is
//! terminal, resume clears a pause without consuming retry budget. The
//! durable exact interruption request precedes its live token projection;
//! the graph token is reserved for connection or Session teardown.

use agent_client_protocol::{Client, ConnectionTo, Responder};
use echo_agent::agent::subagent::{
    DispatchRequest, SubagentAttemptControlHandle, SubagentExecutor, SubagentResult, SubagentStatus,
};
use echo_agent::error::{AgentFailure, AgentTerminalKind, ReactError, Result};
use echo_agent::tasks::{
    RuntimeClaimAbandonment, RuntimeDagController, RuntimeInterruptionDisposition,
    RuntimeInterruptionSettlementOutcome, RuntimeTaskService, RuntimeTaskServiceConfig, Task,
    TaskClaim, TaskRevisionService, TaskStatus, TaskSubagentContext,
};
use echo_agent::tasks::{
    RuntimeTaskClaimOutcome, RuntimeTaskResolution, RuntimeTaskResolutionRequest,
    RuntimeTaskSettlementOutcome,
};
use echo_sdk_protocol::capability::ExtensionCapability;
use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::methods::{
    ControlAction, TaskControlRequest, TaskControlResponse, TaskExecuteRequest,
    TaskExecuteResponse, WireTaskStatus,
};
use std::collections::HashSet;
use std::sync::Arc;

use super::super::handler::{require_capability, require_extended};
use super::super::state::CoreProfileState;
use super::super::wire;
use super::task::{session_services, task_scope, task_session_record};

/// The facade DAG controller. One instance per live TaskRun execution.
pub(crate) struct FacadeTaskController {
    /// CRUD, exact claims, results and terminals share the framework service
    /// captured after extension installation, including external backends.
    store: Arc<TaskRevisionService>,
    attempt_control: SubagentAttemptControlHandle,
}

impl FacadeTaskController {
    pub fn new(store: Arc<TaskRevisionService>, executor: Arc<SubagentExecutor>) -> Result<Self> {
        let attempt_control = executor
            .attempt_control_handle(format!("sdk-task-controller-{}", uuid::Uuid::new_v4()))
            .map_err(|error| ReactError::Other(error.to_string()))?;
        Ok(Self {
            store,
            attempt_control,
        })
    }

    fn subagent_name(task: &Task) -> Result<String> {
        task.spec
            .extension
            .get("subagent")
            .and_then(|value| value.as_str())
            .filter(|name| !name.trim().is_empty())
            .map(str::to_string)
            .ok_or_else(|| {
                ReactError::Other(format!(
                    "task '{}' does not name a dispatchable subagent (extension.subagent)",
                    task.spec.id
                ))
            })
    }
}

#[async_trait::async_trait]
impl RuntimeDagController for FacadeTaskController {
    type DispatchOutput = SubagentResult;

    async fn load_snapshot(&self, run_id: &str) -> Result<echo_agent::tasks::RuntimePlanSnapshot> {
        self.store
            .load(run_id)
            .await
            .map_err(|error| ReactError::Other(error.to_string()))?
            .map(|graph| graph.snapshot)
            .ok_or_else(|| {
                ReactError::Other(format!("task graph '{run_id}' has no committed revision"))
            })
    }

    async fn claim_task(
        &self,
        run_id: &str,
        task: &Task,
        expected_revision: u64,
    ) -> Result<RuntimeTaskClaimOutcome> {
        self.store
            .claim_runtime_task(run_id, task, expected_revision)
            .await
            .map_err(|error| ReactError::Other(error.to_string()))
    }

    async fn claim_is_current(
        &self,
        run_id: &str,
        task_id: &str,
        claim: &TaskClaim,
    ) -> Result<bool> {
        self.store
            .runtime_claim_is_current(run_id, task_id, claim)
            .await
            .map_err(|error| ReactError::Other(error.to_string()))
    }

    async fn dispatch_task(
        &self,
        context: TaskSubagentContext,
        task: Task,
    ) -> Result<Self::DispatchOutput> {
        let agent_name = Self::subagent_name(&task)?;
        let mut prompt = task.spec.description.clone();
        let snapshot = self.load_snapshot(context.run_id()).await?;
        for dependency in &task.spec.depends_on {
            if context.waived_dependency_ids().contains(dependency) {
                prompt.push_str(&format!("\n\nDependency '{dependency}' was explicitly skipped; no reusable output is available."));
                continue;
            }
            let output = snapshot.tasks.iter().find(|entry| entry.spec.id == *dependency)
                .and_then(|entry| entry.execution.result.as_ref())
                .and_then(|result| result.get("output"))
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| ReactError::Other(format!("completed dependency '{dependency}' has no reusable output in its canonical task result")))?;
            prompt.push_str("\n\nCompleted dependency ");
            prompt.push_str(dependency);
            prompt.push_str(":\n");
            prompt.push_str(output);
        }
        self.attempt_control
            .dispatch(
                DispatchRequest {
                    agent_name,
                    task: prompt,
                    mode_override: None,
                    cancel: context.cancellation_token().clone(),
                    parent_agent: "sdk-task-run".to_string(),
                    parent_context: None,
                    delegation_policy: context.delegation_policy(),
                    runtime_context: None,
                    message: None,
                    prompt_payload: None,
                    prompt_context: None,
                    constraints: Vec::new(),
                    background: false,
                },
                context,
            )
            .await
    }

    async fn reserve_attempt_control(&self, context: &TaskSubagentContext) -> Result<()> {
        self.attempt_control
            .reserve(context)
            .map_err(|error| ReactError::Other(error.to_string()))
    }

    async fn request_live_interrupt(
        &self,
        run_id: &str,
        task_id: &str,
        claim: &TaskClaim,
    ) -> std::result::Result<
        Option<echo_agent::tasks::RuntimeTaskAttemptInterruptDisposition>,
        echo_agent::tasks::RuntimeTaskAttemptInterruptProjectionError,
    > {
        self.attempt_control
            .project_interrupt(run_id, task_id, claim)
            .map(Some)
    }

    async fn retire_attempt_control(
        &self,
        run_id: &str,
        task: &Task,
        claim: &TaskClaim,
    ) -> echo_agent::tasks::RuntimeAttemptControlCleanupReceipt {
        self.attempt_control.retire(run_id, task, claim)
    }

    async fn reconcile_attempt_control(
        &self,
        run_id: &str,
        current_execution_ids: &HashSet<String>,
    ) -> Result<()> {
        self.attempt_control
            .reconcile(run_id, current_execution_ids)
            .map_err(|error| ReactError::Other(error.to_string()))
    }

    async fn resolve_dispatch(
        &self,
        _run_id: &str,
        _claim: TaskClaim,
        _task: Task,
        dispatch: Result<Self::DispatchOutput>,
    ) -> Result<RuntimeTaskResolutionRequest> {
        Ok(match dispatch {
            Ok(output) if output.outcome.status == SubagentStatus::Completed => {
                RuntimeTaskResolutionRequest::CompletedWithResult {
                    result: serde_json::json!({
                        "agent_name": output.agent_name,
                        "output": output.output,
                        "outcome": output.outcome,
                        "duration": { "seconds": output.duration.as_secs(), "nanos": output.duration.subsec_nanos() },
                        "iterations": output.iterations,
                        "tokens_used": output.tokens_used,
                        "was_truncated": output.was_truncated,
                        "mode": output.mode,
                        "isolation_observed": output.isolation_observed,
                        "llm_usage": output.llm_usage,
                    }),
                }
            }
            Ok(output) if output.outcome.status == SubagentStatus::Cancelled => {
                RuntimeTaskResolutionRequest::Cancelled
            }
            Ok(output) => {
                let error = if output.outcome.summary.is_empty() {
                    output.output
                } else {
                    output.outcome.summary
                };
                if output.outcome.status == SubagentStatus::TimedOut {
                    RuntimeTaskResolutionRequest::TimedOut { error }
                } else {
                    RuntimeTaskResolutionRequest::Failed { error }
                }
            }
            Err(error) => {
                let message = error.to_string();
                match AgentFailure::from(&error).terminal_kind {
                    AgentTerminalKind::Cancelled => RuntimeTaskResolutionRequest::Cancelled,
                    AgentTerminalKind::TimedOut => {
                        RuntimeTaskResolutionRequest::TimedOut { error: message }
                    }
                    AgentTerminalKind::Failed | AgentTerminalKind::PermissionDenied => {
                        RuntimeTaskResolutionRequest::Failed { error: message }
                    }
                }
            }
        })
    }

    async fn settle_resolution(
        &self,
        run_id: &str,
        claim: &TaskClaim,
        task: &Task,
        request: RuntimeTaskResolutionRequest,
    ) -> Result<RuntimeTaskResolution> {
        self.store
            .settle_runtime_resolution(run_id, &task.spec.id, claim, request)
            .await
            .map_err(|error| ReactError::Other(error.to_string()))
    }

    async fn abandon_claim(
        &self,
        run_id: &str,
        claim: &TaskClaim,
        task: &Task,
        abandonment: RuntimeClaimAbandonment,
    ) -> Result<RuntimeTaskSettlementOutcome> {
        let status = match abandonment {
            RuntimeClaimAbandonment::Interrupted { disposition } => match disposition {
                RuntimeInterruptionDisposition::Cancelled => TaskStatus::Cancelled,
                RuntimeInterruptionDisposition::Paused { reason } => TaskStatus::Paused(reason),
            },
            RuntimeClaimAbandonment::Failed { error } => TaskStatus::Failed(error),
        };
        self.store
            .settle_runtime_claim(run_id, &task.spec.id, claim, status)
            .await
            .map_err(|error| ReactError::Other(error.to_string()))
    }

    async fn settle_interruption(
        &self,
        run_id: &str,
        expected_revision: u64,
        disposition: RuntimeInterruptionDisposition,
    ) -> Result<RuntimeInterruptionSettlementOutcome> {
        self.store
            .settle_runtime_interruption(run_id, expected_revision, disposition)
            .await
            .map_err(|error| ReactError::Other(error.to_string()))
    }
}

fn runtime_error(message: impl std::fmt::Display, method: &str) -> EchoSdkError {
    wire::sdk_error(
        ExtensionErrorCode::FrameworkError,
        wire::bounded_framework_message(&message.to_string()),
        Retryability::Never,
        method,
    )
}

fn wire_status(status: &TaskStatus) -> WireTaskStatus {
    match status {
        TaskStatus::Pending => WireTaskStatus::Pending,
        TaskStatus::Running => WireTaskStatus::Running,
        TaskStatus::Blocked(reason) => WireTaskStatus::Blocked {
            reason: reason.clone(),
        },
        TaskStatus::Completed => WireTaskStatus::Completed,
        TaskStatus::Failed(error) => WireTaskStatus::Failed {
            error: error.clone(),
        },
        TaskStatus::Skipped => WireTaskStatus::Skipped,
        TaskStatus::Cancelled => WireTaskStatus::Cancelled,
        TaskStatus::TimedOut { error } => WireTaskStatus::TimedOut {
            error: error.clone(),
        },
        TaskStatus::Retrying {
            attempt,
            last_error,
        } => WireTaskStatus::Retrying {
            attempt: *attempt,
            last_error: last_error.clone(),
        },
        TaskStatus::Paused(reason) => WireTaskStatus::Paused {
            reason: reason.clone(),
        },
    }
}

pub(crate) async fn task_execute(
    state: Arc<CoreProfileState>,
    request: TaskExecuteRequest,
    responder: Responder<TaskExecuteResponse>,
    connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/task/execute";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::TaskGraph, method)?;
    macro_rules! fail {
        ($error:expr) => {
            return responder.respond_with_error(wire::into_jsonrpc_error($error))
        };
    }
    let session = match task_session_record(&state, &request.task_run, method) {
        Ok(session) => session,
        Err(error) => fail!(error),
    };
    let _session_permit = session.lifecycle.read().await;
    if session.closing.load(std::sync::atomic::Ordering::Acquire) {
        fail!(wire::sdk_error(
            ExtensionErrorCode::ClosedHandle,
            "task owner Session is closing",
            Retryability::Never,
            method
        ));
    }
    let authorities = match session_services(&state, &request.task_run, method) {
        Ok(authorities) => authorities,
        Err(error) => fail!(error),
    };
    let store = authorities.task_revision_service.clone();
    // Backend execution support is an explicit capability. Probe no graph
    // mutations and publish no running handle for a CRUD-only backend.
    if !store.supports_runtime_execution() {
        fail!(
            wire::sdk_error(
                ExtensionErrorCode::FeatureUnavailable,
                "the registered Task Store does not support execution CAS",
                Retryability::Never,
                method,
            )
            .with_handle(request.task_run.clone())
        );
    }
    let executor = authorities.subagent_executor.clone();
    let scope = match task_scope(&state, &request.task_run, method) {
        Ok(scope) => scope,
        Err(error) => fail!(error),
    };
    // One live execution per TaskRun scope: a second execute while the first
    // is still running is a typed conflict, never a silent takeover. Without
    // this the tracking map would be overwritten and the old DAG would keep
    // running untracked (no pause/cancel reachable, cleanup races).
    // One controller per live run; the spawn captures it directly so no
    // later lookup can race the execution.
    let controller = match FacadeTaskController::new(store, executor) {
        Ok(controller) => Arc::new(controller),
        Err(error) => fail!(runtime_error(error, method)),
    };
    let runtime = Arc::new(RuntimeTaskService::new(
        controller.clone(),
        RuntimeTaskServiceConfig::default(),
    ));
    let cancel = tokio_util::sync::CancellationToken::new();
    let completion = Arc::new(super::TaskRunCompletion {
        settled: std::sync::atomic::AtomicBool::new(false),
        notify: tokio::sync::Notify::new(),
    });
    if let Err(error) = state.facade.try_register_task_execution(
        scope.clone(),
        super::TaskRunExecution {
            cancel: cancel.clone(),
            runtime: runtime.clone(),
            completion: completion.clone(),
        },
    ) {
        fail!(error);
    }
    // Clear only after unique execution admission succeeds, so a rejected
    // competing execute cannot erase the previous caller-visible failure.
    *authorities
        .latest_task_execution_error
        .lock()
        .unwrap_or_else(|error| error.into_inner()) = None;
    let state_for_task = state.clone();
    let scope_for_task = scope.clone();
    let authorities_for_task = authorities.clone();
    let task_run_for_task = request.task_run.clone();
    // The execution rides the official connection task: connection teardown
    // takes it down with the transport instead of leaving a detached DAG
    // running past EOF (plan 07 todo 2 step 3). A failed spawn is reported
    // as host-shutting-down — never a fake success for a run that never
    // started.
    let cancel_for_task = cancel.clone();
    let spawned = connection.spawn(async move {
        if let Err(error) = runtime.execute(&scope_for_task, cancel_for_task).await {
            tracing::warn!("task graph execution for {scope_for_task} failed: {error}");
            *authorities_for_task
                .latest_task_execution_error
                .lock()
                .unwrap_or_else(|error| error.into_inner()) =
                Some(wire::framework_error(&error, method).with_handle(task_run_for_task));
        }
        state_for_task.facade.remove_task_execution(&scope_for_task);
        // Settle exactly once so bounded teardown can observe the end.
        completion
            .settled
            .store(true, std::sync::atomic::Ordering::Release);
        completion.notify.notify_waiters();
        Ok::<(), agent_client_protocol::Error>(())
    });
    if let Err(error) = spawned {
        state.facade.remove_task_execution(&scope);
        let error = wire::sdk_error(
            ExtensionErrorCode::HostShuttingDown,
            format!("connection can no longer host task execution: {error}"),
            Retryability::Never,
            method,
        )
        .with_handle(request.task_run.clone());
        *authorities
            .latest_task_execution_error
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(error.clone());
        fail!(error);
    }
    let published = responder.respond(TaskExecuteResponse {
        run: request.task_run,
    });
    if published.is_err() {
        cancel.cancel();
    }
    published
}

pub(crate) async fn task_control(
    state: Arc<CoreProfileState>,
    request: TaskControlRequest,
    responder: Responder<TaskControlResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/task/control";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::TaskGraph, method)?;
    macro_rules! fail {
        ($error:expr) => {
            return responder.respond_with_error(wire::into_jsonrpc_error($error))
        };
    }
    let scope = match task_scope(&state, &request.task_run, method) {
        Ok(scope) => scope,
        Err(error) => fail!(error),
    };
    let plan_task = match state
        .handles
        .plan_task_for_run(&request.task, &request.task_run, method)
    {
        Ok(plan_task) => plan_task,
        Err(error) => fail!(error),
    };
    let session = match task_session_record(&state, &request.task_run, method) {
        Ok(session) => session,
        Err(error) => fail!(error),
    };
    let _session_permit = session.lifecycle.read().await;
    if session.closing.load(std::sync::atomic::Ordering::Acquire) {
        fail!(wire::sdk_error(
            ExtensionErrorCode::ClosedHandle,
            "task owner Session is closing",
            Retryability::Never,
            method
        ));
    }
    let authorities = match session_services(&state, &request.task_run, method) {
        Ok(authorities) => authorities,
        Err(error) => fail!(error),
    };
    let store = authorities.task_revision_service.clone();
    let graph = match tokio::select! {
        result = store.load(&scope) => result,
        _ = session.cancel_operations.cancelled() => fail!(wire::sdk_error(ExtensionErrorCode::ClosedHandle, "task owner Session is closing", Retryability::Never, method)),
    } {
        Ok(Some(graph)) => graph,
        Ok(None) => {
            fail!(runtime_error(
                "task graph has no committed revision",
                method
            ));
        }
        Err(error) => {
            fail!(runtime_error(error, method));
        }
    };
    let Some(task) = graph
        .snapshot
        .tasks
        .iter()
        .find(|task| task.execution.task_id == plan_task.task_id)
        .cloned()
    else {
        fail!(runtime_error("task id is not part of the graph", method));
    };
    let revision = graph.snapshot.revision;
    let control = async {
        let accepted = match request.action {
            ControlAction::Resume => match store.resume_runtime_task(&scope, &task, revision).await
            {
                Ok(echo_agent::tasks::RuntimeTaskResumeOutcome::Resumed) => true,
                Ok(echo_agent::tasks::RuntimeTaskResumeOutcome::Superseded) => false,
                Err(error) => return Err(runtime_error(error, method)),
            },
            ControlAction::Pause | ControlAction::Cancel => {
                if let Some(claim) = task
                    .execution
                    .claim
                    .as_ref()
                    .filter(|_| task.execution.status == TaskStatus::Running)
                {
                    let Some(execution) = state.facade.task_execution_of(&scope) else {
                        return Err(runtime_error(
                            "task has a durable claim but no live execution; recover its graph before control",
                            method,
                        ));
                    };
                    let disposition = match request.action {
                        ControlAction::Pause => RuntimeInterruptionDisposition::Paused {
                            reason: "paused by rpc task control".to_string(),
                        },
                        _ => RuntimeInterruptionDisposition::Cancelled,
                    };
                    let requested = match store
                        .request_runtime_task_interruption(
                            &scope,
                            &plan_task.task_id,
                            claim,
                            disposition,
                        )
                        .await
                    {
                        Ok(requested) => requested,
                        Err(error) => return Err(runtime_error(error, method)),
                    };
                    if requested {
                        match execution.runtime.request_attempt_interrupt(&scope, &plan_task.task_id, claim).await {
                        Ok(_) | Err(echo_agent::tasks::RuntimeTaskAttemptInterruptError::StaleOrSettledClaim { .. }) => {},
                        Err(error) => return Err(runtime_error(error, method)),
                    }
                    }
                    requested
                } else {
                    false
                }
            }
        };
        Ok::<_, EchoSdkError>(accepted)
    };
    let accepted = match tokio::select! {
        result = control => result,
        _ = session.cancel_operations.cancelled() => Err(wire::sdk_error(ExtensionErrorCode::ClosedHandle, "task owner Session is closing; durable control may already have been recorded", Retryability::Never, method)),
    } {
        Ok(accepted) => accepted,
        Err(error) => fail!(error),
    };

    let updated = match tokio::select! {
        result = store.load(&scope) => result,
        _ = session.cancel_operations.cancelled() => fail!(wire::sdk_error(ExtensionErrorCode::ClosedHandle, "task owner Session is closing", Retryability::Never, method)),
    } {
        Ok(Some(graph)) => graph,
        _ => {
            fail!(runtime_error("task graph vanished during control", method));
        }
    };
    let Some(status) = updated
        .snapshot
        .tasks
        .iter()
        .find(|task| task.execution.task_id == plan_task.task_id)
        .map(|task| task.execution.status.clone())
    else {
        fail!(runtime_error(
            "controlled task vanished from its graph",
            method
        ));
    };
    responder.respond(TaskControlResponse {
        accepted,
        status: wire_status(&status),
    })
}
