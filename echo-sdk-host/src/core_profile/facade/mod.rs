//! Facade adapter runtime (plan 07, todo 2).
//!
//! One raw dispatcher owns every facade wire method that is compiled into
//! this build: the generic `_echo_agent/facade/invoke` and the family
//! methods whose handler families have landed (`features::
//! compiled_facade_families`). The dispatcher attaches to the same
//! official builder chain as the typed core handlers
//! ([`super::SdkCoreProfile::attach`]), so all profiles share one
//! transport, writer, Session/Run and close chain.
//!
//! Methods the dispatcher does not own fall through
//! (`Handled::No`) and the official runtime answers method-not-found —
//! exactly the fail-closed contract for plain clients and for families
//! this build did not compile.
//!
//! The admission ladder (mirroring the core handlers):
//!
//! 1. Extended-mode gate — Standard connections get the official
//!    method-not-found (the dispatcher does not claim at all);
//! 2. capability gate — `feature_surfaces` must be advertised;
//! 3. request validation — exact operation identity, sha256 signature
//!    digest, no wildcards, bounded typed arguments;
//! 4. route resolution — family method binding or exact invoke identity
//!    through the embedded canonical catalog; unknown operations fail
//!    closed as `invalid_value` with typed detail;
//! 5. feature gate — the family's root leaf feature must be advertised;
//! 6. family dispatch — compiled family handlers route to the real
//!    framework services; families this build did not compile answer with
//!    a typed `feature_unavailable` (never a partial or simulated result).

pub(crate) mod integrations;
pub(crate) mod memory;
pub(crate) mod observability;
#[cfg(feature = "framework-human-loop")]
pub(crate) mod permission;
pub(crate) mod registry;
pub(crate) mod source_operations;
pub(crate) mod state_delivery;
pub(crate) mod stream;
pub(crate) mod structured_output;
#[cfg(feature = "framework-subagent")]
pub(crate) mod subagent;
pub(crate) mod task;
#[cfg(feature = "framework-subagent")]
pub(crate) mod task_runtime;
#[cfg(feature = "framework-telemetry")]
pub(crate) mod telemetry;
pub(crate) mod tools;
pub(crate) mod workflow;

use agent_client_protocol::{Client, ConnectionTo, Dispatch, Error, HandleDispatchFrom, Handled};
use echo_sdk_protocol::capability::ExtensionCapability;
use echo_sdk_protocol::error::{
    EchoSdkError, ErrorDetails, ExtensionErrorCode, FacadeFailureDetail, Retryability,
};
use echo_sdk_protocol::handle::{HandleKind, WireHandle};
use echo_sdk_protocol::methods::{FeatureOperationRequest, FeatureOperationResponse};
use echo_sdk_protocol::scalar::WireValue;
use std::sync::Arc;
#[cfg(feature = "framework-subagent")]
use std::sync::atomic::{AtomicBool, Ordering};

/// One family handler outcome: the typed wire result or typed failure.
type WireResult = Result<WireValue, EchoSdkError>;

/// Project accepted resource addresses for the family's existing JSON parser.
/// The resulting DTO still passes the unified generation/owner/issuance ladder;
/// this does not grant a JSON projection to other process-local typed values.
fn family_argument_json(
    value: &echo_sdk_protocol::scalar::WireValue,
) -> Result<serde_json::Value, echo_sdk_protocol::scalar::ScalarError> {
    match value {
        echo_sdk_protocol::scalar::WireValue::Handle(handle) => serde_json::to_value(handle)
            .map_err(|_| {
                echo_sdk_protocol::scalar::ScalarError::ValueLimit(
                    "resource handle serialization failed",
                )
            }),
        _ => value.clone().into_json(),
    }
}

/// Parse one [`WireHandle`] argument addressing an open facade resource.
/// Family operations address their resources through Host-issued handles
/// only (design §8): shape, kind, generation and issuance are checked by
/// the unified authority on every access.
pub(crate) fn resource_handle_at(
    arguments: &[serde_json::Value],
    position: usize,
    what: &str,
    method: &str,
) -> Result<WireHandle, EchoSdkError> {
    let value = arguments.get(position).cloned().ok_or_else(|| {
        wire::sdk_error(
            ExtensionErrorCode::InvalidValue,
            format!("operation requires {what} at argument {position}"),
            Retryability::Never,
            method,
        )
    })?;
    serde_json::from_value::<WireHandle>(value).map_err(|error| {
        wire::sdk_error(
            ExtensionErrorCode::InvalidValue,
            format!("{what} at argument {position} is not a resource handle: {error}"),
            Retryability::Never,
            method,
        )
    })
}

/// Resolve one facade resource through the unified authority and prove it
/// belongs to the requesting session.
pub(crate) fn owned_resource(
    handles: &HandleRegistry,
    resource: &WireHandle,
    owner: &str,
    method: &str,
) -> Result<Arc<FacadeResourceRecord>, EchoSdkError> {
    let record = handles.facade_resource(resource, method)?;
    if record.owner_session.as_deref() != Some(owner) {
        return Err(wire::sdk_error(
            ExtensionErrorCode::InvalidValue,
            "facade resource belongs to another session",
            Retryability::Never,
            method,
        ));
    }
    Ok(record)
}

use super::handles::{FacadeResourceRecord, HandleRegistry};
use super::state::CoreProfileState;
use super::wire;
use crate::config::SdkProfileLimits;
use registry::CompiledOperationCatalog;

/// Connection-level facade runtime: the resource handle surface in
/// [`super::handles::HandleRegistry`] and the RPC subagent dispatch records.
/// Stream handles are also owned by that registry; this runtime only keeps
/// family business maps and bounded execution records.
pub(crate) struct SessionFacadeRuntime {
    /// File leases and identity guards issued by the source-operation
    /// adapter. Their Rust values stay alive until the unified facade
    /// resource handle is closed, preserving the framework's descriptor and
    /// lease lifetime semantics without introducing another handle registry.
    file_authorities: std::sync::Mutex<
        std::collections::HashMap<String, Arc<source_operations::FileAuthorityRecord>>,
    >,
    pub(crate) tokenizer_authorities: std::sync::Mutex<
        std::collections::HashMap<String, Arc<dyn echo_agent::tokenizer::Tokenizer>>,
    >,
    #[cfg(feature = "framework-content-guard")]
    pub(crate) content_guard_authorities: std::sync::Mutex<
        std::collections::HashMap<String, source_operations::ContentGuardAuthorityRecord>,
    >,
    #[cfg(feature = "framework-project-rules")]
    pub(crate) instruction_resolver_authorities: std::sync::Mutex<
        std::collections::HashMap<String, source_operations::InstructionResolverAuthorityRecord>,
    >,
    streams: stream::FacadeStreamRuntime,
    #[cfg(feature = "framework-subagent")]
    subagents:
        std::sync::Mutex<std::collections::HashMap<String, Arc<subagent::SubagentDispatchRecord>>>,
    /// Advertised live-subagent bound (`max_open_handles`): the map never
    /// grows past it, so a chatty client cannot exhaust the Host.
    #[cfg(feature = "framework-subagent")]
    max_subagents: usize,
    #[cfg(feature = "framework-subagent")]
    task_executions: std::sync::Mutex<std::collections::HashMap<String, TaskRunExecution>>,
    // Workflow family resources (todo 4): compiled graphs and standalone
    // shared states, owner-checked per session. The maps hold addressing
    // only; the graph engine keeps its own state.
    workflow_graphs:
        std::sync::Mutex<std::collections::HashMap<String, Arc<workflow::WorkflowGraphRecord>>>,
    workflow_states:
        std::sync::Mutex<std::collections::HashMap<String, Arc<workflow::WorkflowStateRecord>>>,
    // Delivery ledgers and trace stores (todo 4 step 3): framework
    // services held as owner-checked resources.
    delivery_ledgers: std::sync::Mutex<
        std::collections::HashMap<String, Arc<state_delivery::DeliveryLedgerRecord>>,
    >,
    trace_stores:
        std::sync::Mutex<std::collections::HashMap<String, Arc<observability::TraceStoreRecord>>>,
    #[cfg(feature = "framework-eval")]
    pub(crate) eval_runners:
        std::sync::Mutex<std::collections::HashMap<String, Arc<echo_agent::eval::EvalRunner>>>,
    #[cfg(feature = "framework-eval")]
    pub(crate) llm_graders:
        std::sync::Mutex<std::collections::HashMap<String, Arc<echo_agent::eval::LlmGrader>>>,
    #[cfg(feature = "framework-improve")]
    pub(crate) trajectory_savers: std::sync::Mutex<
        std::collections::HashMap<String, Arc<echo_agent::improve::TrajectorySaver>>,
    >,
    #[cfg(feature = "framework-improve")]
    pub(crate) improvement_loops: std::sync::Mutex<
        std::collections::HashMap<String, Arc<echo_agent::improve::ImprovementLoop>>,
    >,
    pub(crate) plugin_registries: std::sync::Mutex<
        std::collections::HashMap<
            String,
            Arc<tokio::sync::Mutex<echo_agent::plugin::PluginRegistry>>,
        >,
    >,
    pub(crate) memory_store_resources: std::sync::Mutex<
        std::collections::HashMap<String, Arc<source_operations::MemoryStoreAuthority>>,
    >,
    pub(crate) prompt_contexts: std::sync::Mutex<
        std::collections::HashMap<String, Arc<echo_agent::skills::external::PromptContext>>,
    >,
    /// RAG index/search dependencies scoped to one ACP session. The map is
    /// deliberately connection-owned and never process-global: closing a
    /// session drops both its in-memory index and embedder client.
    #[cfg(feature = "framework-rag")]
    rag_tools: std::sync::Mutex<std::collections::HashMap<String, Arc<tools::RagToolSet>>>,
    // Integration family resources (todo 5): MCP managers, A2A clients,
    // LSP managers and topology trackers, owner-checked per session.
    pub integrations: integrations::IntegrationResources,
}

impl SessionFacadeRuntime {
    pub fn new(limits: &SdkProfileLimits) -> Self {
        Self {
            file_authorities: std::sync::Mutex::new(std::collections::HashMap::new()),
            tokenizer_authorities: std::sync::Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "framework-content-guard")]
            content_guard_authorities: std::sync::Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "framework-project-rules")]
            instruction_resolver_authorities: std::sync::Mutex::new(
                std::collections::HashMap::new(),
            ),
            streams: stream::FacadeStreamRuntime::new(
                limits.max_facade_streams,
                limits.shutdown_timeout_secs,
            ),
            #[cfg(feature = "framework-subagent")]
            subagents: std::sync::Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "framework-subagent")]
            max_subagents: limits.max_open_handles,
            #[cfg(feature = "framework-subagent")]
            task_executions: std::sync::Mutex::new(std::collections::HashMap::new()),
            workflow_graphs: std::sync::Mutex::new(std::collections::HashMap::new()),
            workflow_states: std::sync::Mutex::new(std::collections::HashMap::new()),
            delivery_ledgers: std::sync::Mutex::new(std::collections::HashMap::new()),
            trace_stores: std::sync::Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "framework-eval")]
            eval_runners: std::sync::Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "framework-eval")]
            llm_graders: std::sync::Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "framework-improve")]
            trajectory_savers: std::sync::Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "framework-improve")]
            improvement_loops: std::sync::Mutex::new(std::collections::HashMap::new()),
            plugin_registries: std::sync::Mutex::new(std::collections::HashMap::new()),
            memory_store_resources: std::sync::Mutex::new(std::collections::HashMap::new()),
            prompt_contexts: std::sync::Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "framework-rag")]
            rag_tools: std::sync::Mutex::new(std::collections::HashMap::new()),
            integrations: integrations::IntegrationResources::new(),
        }
    }

    #[cfg(all(test, feature = "framework-subagent", feature = "sdk-extension-bridge"))]
    pub(crate) fn with_subagent_capacity_for_test(
        limits: &crate::config::SdkProfileLimits,
        capacity: usize,
    ) -> Self {
        let mut runtime = Self::new(limits);
        runtime.max_subagents = capacity;
        runtime
    }

    /// Register one RPC-dispatched subagent record by execution id. The
    /// advertised live-subagent bound is a hard limit: settled records are
    /// retired first, and a full map rejects the dispatch with a typed
    /// error instead of growing without end.
    #[cfg(feature = "framework-subagent")]
    pub fn register_subagent(
        &self,
        execution_id: String,
        record: Arc<subagent::SubagentDispatchRecord>,
    ) -> Result<(), EchoSdkError> {
        let mut subagents = self
            .subagents
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if subagents.len() >= self.max_subagents {
            // Drop settled records first; unsettled (or currently-joining)
            // attempts stay, and a still-full map rejects the new dispatch.
            subagents.retain(|_, record| {
                !record
                    .settled
                    .try_lock()
                    .is_ok_and(|settled| settled.is_some())
            });
            if subagents.len() >= self.max_subagents {
                return Err(wire::sdk_error(
                    ExtensionErrorCode::PayloadTooLarge,
                    format!(
                        "live subagent dispatch limit {} reached",
                        self.max_subagents
                    ),
                    Retryability::AfterDelay,
                    "_echo_agent/subagent/dispatch",
                ));
            }
        }
        subagents.insert(execution_id, record);
        Ok(())
    }

    /// Retire only the exact reservation whose framework setup proved that
    /// no attempt was spawned. Never use this for a collector observation.
    #[cfg(feature = "framework-subagent")]
    pub(super) fn retire_unsubmitted_subagent(
        &self,
        execution_id: &str,
        original: &Arc<subagent::SubagentDispatchRecord>,
    ) {
        let mut subagents = self
            .subagents
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if subagents
            .get(execution_id)
            .is_some_and(|record| Arc::ptr_eq(record, original))
        {
            subagents.remove(execution_id);
        }
    }

    /// Resolve one subagent record by execution id.
    #[cfg(feature = "framework-subagent")]
    pub fn subagent_of(&self, execution_id: &str) -> Option<Arc<subagent::SubagentDispatchRecord>> {
        self.subagents
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(execution_id)
            .cloned()
    }

    /// One live task-graph execution per TaskRun scope, addressable for
    /// run-level pause/cancel through the shared cancel token. Insertion is
    /// atomic: a second execute of a live scope fails instead of silently
    /// replacing the tracked execution (the old DAG would otherwise keep
    /// running untracked and its cleanup would delete the new record).
    #[cfg(feature = "framework-subagent")]
    pub fn try_register_task_execution(
        &self,
        scope: String,
        execution: TaskRunExecution,
    ) -> Result<(), EchoSdkError> {
        let mut executions = self
            .task_executions
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if executions.contains_key(&scope) {
            return Err(wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                "task run is already executing; cancel it before re-executing",
                Retryability::Never,
                "_echo_agent/task/execute",
            ));
        }
        executions.insert(scope, execution);
        Ok(())
    }

    #[cfg(feature = "framework-subagent")]
    pub fn task_execution_of(&self, scope: &str) -> Option<TaskRunExecution> {
        self.task_executions
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(scope)
            .cloned()
    }

    #[cfg(feature = "framework-subagent")]
    pub fn remove_task_execution(&self, scope: &str) {
        self.task_executions
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(scope);
    }

    /// Lazily create the RAG tool set for one ACP session. Configuration is
    /// checked before insertion, so an unavailable embedder never appears as
    /// a successful or shared process-global capability.
    #[cfg(feature = "framework-rag")]
    pub fn rag_tools_for_session(
        &self,
        owner: &str,
        operation: &str,
    ) -> Result<Arc<tools::RagToolSet>, EchoSdkError> {
        let mut rag_tools = self
            .rag_tools
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if let Some(existing) = rag_tools.get(owner) {
            return Ok(existing.clone());
        }
        let created = Arc::new(
            tools::RagToolSet::from_env().map_err(|error| error.into_sdk_error(operation))?,
        );
        rag_tools.insert(owner.to_string(), created.clone());
        Ok(created)
    }

    /// Cancel and remove every task execution owned by one session scope;
    /// session close must never leave a DAG running untracked.
    #[cfg(feature = "framework-subagent")]
    pub async fn drop_task_executions_of(&self, scope: &str) -> Result<(), EchoSdkError> {
        if let Some(execution) = self.task_execution_of(scope) {
            execution.cancel.cancel();
            while !execution.completion.settled.load(Ordering::Acquire) {
                let notified = execution.completion.notify.notified();
                tokio::pin!(notified);
                notified.as_mut().enable();
                if let Some(observation) = execution
                    .completion
                    .observation
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .as_ref()
                {
                    return Err(observation.clone());
                }
                if execution.completion.settled.load(Ordering::Acquire) {
                    break;
                }
                notified.await;
            }
            self.remove_task_execution(scope);
        }
        Ok(())
    }

    /// Cancel every live task execution and wait for each spawned service
    /// task to settle (connection teardown; the caller's shutdown timeout
    /// bounds the whole wait).
    #[cfg(feature = "framework-subagent")]
    pub async fn cancel_all_task_executions_and_wait(&self) -> Result<(), EchoSdkError> {
        let scopes = self
            .task_executions
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        for scope in &scopes {
            if let Some(execution) = self.task_execution_of(scope) {
                execution.cancel.cancel();
            }
        }
        for scope in scopes {
            self.drop_task_executions_of(&scope).await?;
        }
        Ok(())
    }

    /// Cancel every RPC subagent dispatch attempt owned by one session.
    #[cfg(feature = "framework-subagent")]
    pub async fn cancel_subagents_of(&self, owner: &str) -> Result<(), EchoSdkError> {
        let records = self
            .subagents
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|(_, record)| record.owner_session == owner)
            .map(|(id, record)| (id.clone(), record.clone()))
            .collect::<Vec<_>>();
        for (_, record) in &records {
            record.cancel.cancel();
        }
        for (id, record) in records {
            record.wait_settled().await?;
            self.subagents
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&id);
        }
        Ok(())
    }

    /// The collector is the sole join consumer. Teardown waits for its
    /// settlement projection without racing it for the framework receipt.
    #[cfg(feature = "framework-subagent")]
    pub async fn cancel_all_subagents_and_wait(&self) -> Result<(), EchoSdkError> {
        let owners = self
            .subagents
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .values()
            .map(|record| record.owner_session.clone())
            .collect::<Vec<_>>();
        for owner in owners {
            self.cancel_subagents_of(&owner).await?;
        }
        Ok(())
    }

    /// Drop and cancel every facade resource whose handle the unified
    /// authority just closed for this session; called when the session
    /// closes so family resources never outlive their owner. Task
    /// executions of the session's graph scope and its live subagent
    /// dispatches are cancelled too.
    pub async fn drop_session_resources_of(
        &self,
        handles: &HandleRegistry,
        owner: &str,
        closed: &[String],
    ) -> Result<(), EchoSdkError> {
        #[cfg(feature = "framework-subagent")]
        {
            self.drop_task_executions_of(owner).await?;
            self.cancel_subagents_of(owner).await?;
        }
        self.streams.close_owner(owner).await?;
        integrations::close_session_resources(&self.integrations, closed)
            .await
            .map_err(|error| {
                wire::sdk_error(
                    ExtensionErrorCode::FrameworkError,
                    error,
                    Retryability::AfterDelay,
                    "_echo_agent/session/close",
                )
            })?;
        #[cfg(feature = "framework-mcp")]
        self.integrations
            .close_mcp_debt(handles, Some(owner))
            .await
            .map_err(|error| {
                wire::sdk_error(
                    ExtensionErrorCode::FrameworkError,
                    error,
                    Retryability::AfterDelay,
                    "_echo_agent/session/close",
                )
            })?;
        source_operations::drop_file_authorities(&self.file_authorities, closed);
        source_operations::drop_tokenizer_authorities(&self.tokenizer_authorities, closed);
        #[cfg(feature = "framework-content-guard")]
        source_operations::drop_value_authorities(&self.content_guard_authorities, closed);
        #[cfg(feature = "framework-project-rules")]
        source_operations::drop_value_authorities(&self.instruction_resolver_authorities, closed);
        workflow::drop_session_resources(&self.workflow_graphs, &self.workflow_states, closed);
        state_delivery::drop_session_ledgers(&self.delivery_ledgers, closed);
        observability::drop_session_stores(&self.trace_stores, closed);
        #[cfg(feature = "framework-eval")]
        self.eval_runners
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .retain(|id, _| !closed.iter().any(|closed_id| closed_id == id));
        #[cfg(feature = "framework-eval")]
        self.llm_graders
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .retain(|id, _| !closed.iter().any(|closed_id| closed_id == id));
        #[cfg(feature = "framework-improve")]
        self.trajectory_savers
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .retain(|id, _| !closed.iter().any(|closed_id| closed_id == id));
        #[cfg(feature = "framework-improve")]
        self.improvement_loops
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .retain(|id, _| !closed.iter().any(|closed_id| closed_id == id));
        self.plugin_registries
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .retain(|id, _| !closed.iter().any(|closed_id| closed_id == id));
        self.memory_store_resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .retain(|id, _| !closed.iter().any(|closed_id| closed_id == id));
        self.prompt_contexts
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .retain(|id, _| !closed.iter().any(|closed_id| closed_id == id));
        #[cfg(feature = "framework-rag")]
        self.rag_tools
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(owner);
        Ok(())
    }

    /// Close one owner-checked facade resource and release its concrete Rust
    /// business object. Fence new resolution before cleanup but keep the
    /// issued owner until successful settlement, allowing retry on failure.
    pub async fn close_resource(
        &self,
        handles: &HandleRegistry,
        owner: &str,
        resource: &WireHandle,
        operation: &str,
    ) -> Result<bool, EchoSdkError> {
        if handles.is_closed(resource) {
            return Ok(false);
        }
        let _record = handles.facade_resource_for_close(resource, operation)?;
        if _record.owner_session.as_deref() != Some(owner) {
            return Err(wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                "facade resource belongs to another session",
                Retryability::Never,
                operation,
            ));
        }
        // Fence admission and cancel streams before waiting for the shared
        // permits retained by their producers. The original collector owns
        // settlement; a closing flag is never reopened on failed cleanup.
        _record
            .closing
            .store(true, std::sync::atomic::Ordering::Release);
        _record.cancel_operations.cancel();
        self.streams.close_resource(&resource.id).await?;
        let _closing = _record.lifecycle.write().await;
        if handles.is_closed(resource) {
            return Ok(false);
        }
        let ids = [resource.id.clone()];
        integrations::close_session_resources(&self.integrations, &ids)
            .await
            .map_err(|error| {
                wire::sdk_error(
                    ExtensionErrorCode::FrameworkError,
                    error,
                    Retryability::AfterDelay,
                    operation,
                )
            })?;
        let closed = handles.close_facade_resource(resource, operation)?;
        if !closed {
            return Ok(false);
        }
        source_operations::drop_file_authorities(&self.file_authorities, &ids);
        source_operations::drop_tokenizer_authorities(&self.tokenizer_authorities, &ids);
        #[cfg(feature = "framework-content-guard")]
        source_operations::drop_value_authorities(&self.content_guard_authorities, &ids);
        #[cfg(feature = "framework-project-rules")]
        source_operations::drop_value_authorities(&self.instruction_resolver_authorities, &ids);
        workflow::drop_session_resources(&self.workflow_graphs, &self.workflow_states, &ids);
        state_delivery::drop_session_ledgers(&self.delivery_ledgers, &ids);
        observability::drop_session_stores(&self.trace_stores, &ids);
        #[cfg(feature = "framework-eval")]
        self.eval_runners
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&resource.id);
        #[cfg(feature = "framework-eval")]
        self.llm_graders
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&resource.id);
        #[cfg(feature = "framework-improve")]
        self.trajectory_savers
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&resource.id);
        #[cfg(feature = "framework-improve")]
        self.improvement_loops
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&resource.id);
        self.plugin_registries
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&resource.id);
        self.memory_store_resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&resource.id);
        self.prompt_contexts
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&resource.id);
        #[cfg(feature = "framework-rag")]
        if _record.family == "rag" {
            self.rag_tools
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .remove(owner);
        }
        Ok(true)
    }

    /// Connection teardown: cancel task executions and subagent dispatches
    /// and wait for their bounded settlement, close every resource handle
    /// in the unified authority, await integration close (MCP child
    /// processes included) and release the remaining business maps. The
    /// caller runs this inside the bounded shutdown chain.
    pub async fn close_all(&self, handles: &HandleRegistry) -> Result<(), String> {
        #[cfg(feature = "framework-subagent")]
        {
            self.cancel_all_task_executions_and_wait()
                .await
                .map_err(|error| error.message)?;
            self.cancel_all_subagents_and_wait()
                .await
                .map_err(|error| error.message)?;
        }
        self.streams
            .close_all()
            .await
            .map_err(|error| error.message)?;
        self.integrations.close_all(handles).await?;
        handles.close_all_facade_resources();
        std::mem::take(
            &mut *self
                .file_authorities
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        std::mem::take(
            &mut *self
                .memory_store_resources
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        std::mem::take(
            &mut *self
                .prompt_contexts
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        std::mem::take(
            &mut *self
                .plugin_registries
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        #[cfg(feature = "framework-eval")]
        std::mem::take(
            &mut *self
                .llm_graders
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        std::mem::take(
            &mut *self
                .tokenizer_authorities
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        #[cfg(feature = "framework-content-guard")]
        std::mem::take(
            &mut *self
                .content_guard_authorities
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        #[cfg(feature = "framework-project-rules")]
        std::mem::take(
            &mut *self
                .instruction_resolver_authorities
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        std::mem::take(
            &mut *self
                .workflow_graphs
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        std::mem::take(
            &mut *self
                .workflow_states
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        std::mem::take(
            &mut *self
                .delivery_ledgers
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        std::mem::take(
            &mut *self
                .trace_stores
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        #[cfg(feature = "framework-eval")]
        std::mem::take(
            &mut *self
                .eval_runners
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        #[cfg(feature = "framework-improve")]
        std::mem::take(
            &mut *self
                .trajectory_savers
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        #[cfg(feature = "framework-improve")]
        std::mem::take(
            &mut *self
                .improvement_loops
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        #[cfg(feature = "framework-rag")]
        {
            std::mem::take(
                &mut *self
                    .rag_tools
                    .lock()
                    .unwrap_or_else(|error| error.into_inner()),
            );
        }
        Ok(())
    }
}

/// Per-request bounds passed to every family handler: the advertised facade
/// resource limit and the page bound. Family handlers enforce these instead
/// of private magic constants so the real limit always matches the
/// `initialize` advertisement (design §15 — payload, queue and shutdown
/// bounds are all explicit).
#[derive(Debug, Clone, Copy)]
pub(crate) struct FacadeFamilyLimits {
    pub max_resources: usize,
    pub page: usize,
}

/// A live task-graph execution (cheap clone: token + controller Arcs).
/// `done` fires exactly once when the execution task settles, so bounded
/// teardown can wait for real settlement instead of abandoning the DAG.
#[cfg(feature = "framework-subagent")]
pub(crate) struct TaskRunCompletion {
    pub settled: AtomicBool,
    pub observation: std::sync::Mutex<Option<EchoSdkError>>,
    pub notify: tokio::sync::Notify,
}

#[cfg(feature = "framework-subagent")]
#[derive(Clone)]
pub(crate) struct TaskRunExecution {
    pub cancel: tokio_util::sync::CancellationToken,
    pub runtime: Arc<echo_agent::tasks::RuntimeTaskService<task_runtime::FacadeTaskController>>,
    pub completion: Arc<TaskRunCompletion>,
}

/// Typed facade admission error carrying [`FacadeFailureDetail`].
fn facade_error(
    code: ExtensionErrorCode,
    message: impl Into<String>,
    retryable: Retryability,
    operation: &str,
    detail: FacadeFailureDetail,
) -> EchoSdkError {
    let mut error = EchoSdkError::new(code, message, retryable).with_operation(operation);
    error.details = Some(ErrorDetails {
        fields: None,
        facade: Some(detail),
    });
    error
}

/// Whether this build's dispatcher owns one wire method. Family methods
/// are owned once their handler family compiled in; the generic invoke is
/// owned by the facade runtime itself.
fn owns_method(method: &str) -> bool {
    if method == "_echo_agent/facade/invoke" {
        return true;
    }
    if method == "_echo_agent/structured_output/validate" {
        return true;
    }
    let compiled = crate::features::compiled_facade_families();
    CompiledOperationCatalog::global()
        .ok()
        .and_then(|catalog| catalog.family_method(method))
        .is_some_and(|summary| compiled.contains(&summary.family.as_str()))
}

/// The raw facade dispatcher attached to the official builder chain.
pub(crate) struct FacadeDispatcher {
    state: Arc<CoreProfileState>,
}

impl FacadeDispatcher {
    pub fn new(state: Arc<CoreProfileState>) -> Self {
        Self { state }
    }

    async fn dispatch(
        &self,
        method: &str,
        params: &serde_json::Value,
        respond: impl FnOnce(Result<FeatureOperationResponse, Error>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        match facade_admission(&self.state, method, params).await {
            Ok(response) => respond(Ok(response)),
            Err(error) => respond(Err(wire::into_jsonrpc_error(error))),
        }
    }
}

impl HandleDispatchFrom<Client> for FacadeDispatcher {
    async fn handle_dispatch_from(
        &mut self,
        message: Dispatch,
        connection: ConnectionTo<Client>,
    ) -> Result<Handled<Dispatch>, Error> {
        let method = message.method().to_string();
        let extended = match self.state.services() {
            Ok(services) => services.is_extended().await,
            Err(_) => false,
        };
        // Standard connections — and methods this build does not own —
        // fall through to the official method-not-found.
        if !extended || !owns_method(&method) {
            return Ok(Handled::No {
                message,
                retry: false,
            });
        }
        let Dispatch::Request(request, responder) = message else {
            return Ok(Handled::No {
                message,
                retry: false,
            });
        };
        let params = request.params().clone();
        let state = self.state.clone();
        let method_for_task = method.clone();
        let connection_for_task = connection.clone();
        // Never block the dispatch loop: the ladder itself is fast, but the
        // family work rides the official connection task so connection
        // teardown takes in-flight facade work down with the transport
        // instead of leaving detached tasks behind (plan 07 todo 2 step 3).
        // The responder lives in a shared cell: whoever settles first (the
        // spawned task or the spawn-failure path) answers exactly once, so
        // a failed spawn still delivers the typed host-shutting-down
        // failure while the request is alive.
        let spawn_method = method_for_task.clone();
        let responder_cell = Arc::new(std::sync::Mutex::new(Some(responder)));
        let cell_for_task = responder_cell.clone();
        if let Err(error) = connection.spawn(async move {
            // Direct Agent chat/execute source operations reuse the typed
            // RunStart handler, including EventEnvelope delivery, stream
            // registration, cancellation and exactly-one terminal semantics.
            match source_operations::run_start_request(&state, &spawn_method, &params) {
                Ok(Some(run_request)) => {
                    let responder = cell_for_task
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .take()
                        .ok_or_else(|| {
                            Error::internal_error().data("facade response was already answered")
                        })?;
                    let run_responder = responder.wrap_params(|_method, result| {
                        result.and_then(|run| {
                            let value = serde_json::to_value(FeatureOperationResponse {
                                value: WireValue::from_json(serde_json::to_value(run).map_err(
                                    |error| {
                                        Error::internal_error()
                                            .data(format!("run response encoding failed: {error}"))
                                    },
                                )?)
                                .map_err(|error| {
                                    Error::internal_error()
                                        .data(format!("run response wire encoding failed: {error}"))
                                })?,
                            })
                            .map_err(|error| {
                                Error::internal_error()
                                    .data(format!("facade response encoding failed: {error}"))
                            })?;
                            Ok(value)
                        })
                    });
                    crate::core_profile::handler::run_start(
                        state.clone(),
                        run_request,
                        run_responder,
                        connection_for_task.clone(),
                    )
                    .await?;
                    return Ok(());
                }
                Ok(None) => {}
                Err(error) => {
                    let responder = cell_for_task
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .take();
                    if let Some(responder) = responder {
                        responder.respond_with_error(wire::into_jsonrpc_error(error))?;
                    }
                    return Ok(());
                }
            }
            match source_operations::run_steer_request(&state, &spawn_method, &params) {
                Ok(Some(steer_request)) => {
                    let responder = cell_for_task
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .take()
                        .ok_or_else(|| {
                            Error::internal_error().data("facade response was already answered")
                        })?;
                    let steer_responder = responder.wrap_params(|_method, result| {
                        result.and_then(|steer| {
                            let value = serde_json::to_value(FeatureOperationResponse {
                                value: WireValue::from_json(serde_json::to_value(steer).map_err(
                                    |error| {
                                        Error::internal_error().data(format!(
                                            "steer response encoding failed: {error}"
                                        ))
                                    },
                                )?)
                                .map_err(|error| {
                                    Error::internal_error().data(format!(
                                        "steer response wire encoding failed: {error}"
                                    ))
                                })?,
                            })
                            .map_err(|error| {
                                Error::internal_error()
                                    .data(format!("facade response encoding failed: {error}"))
                            })?;
                            Ok(value)
                        })
                    });
                    crate::core_profile::handler::run_steer(
                        state.clone(),
                        steer_request,
                        steer_responder,
                        connection_for_task.clone(),
                    )
                    .await?;
                    return Ok(());
                }
                Ok(None) => {}
                Err(error) => {
                    let responder = cell_for_task
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .take();
                    if let Some(responder) = responder {
                        responder.respond_with_error(wire::into_jsonrpc_error(error))?;
                    }
                    return Ok(());
                }
            }
            match source_operations::agent_close_request(&state, &spawn_method, &params) {
                Ok(Some(close_request)) => {
                    let responder = cell_for_task
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .take()
                        .ok_or_else(|| {
                            Error::internal_error().data("facade response was already answered")
                        })?;
                    let close_responder = responder.wrap_params(|_method, result| {
                        result.and_then(|closed| {
                            let value = serde_json::to_value(FeatureOperationResponse {
                                value: WireValue::from_json(serde_json::to_value(closed).map_err(
                                    |error| {
                                        Error::internal_error().data(format!(
                                            "agent close response encoding failed: {error}"
                                        ))
                                    },
                                )?)
                                .map_err(|error| {
                                    Error::internal_error().data(format!(
                                        "agent close response wire encoding failed: {error}"
                                    ))
                                })?,
                            })
                            .map_err(|error| {
                                Error::internal_error()
                                    .data(format!("facade response encoding failed: {error}"))
                            })?;
                            Ok(value)
                        })
                    });
                    crate::core_profile::handler::agent_close(
                        state.clone(),
                        close_request,
                        close_responder,
                        connection_for_task.clone(),
                    )
                    .await?;
                    return Ok(());
                }
                Ok(None) => {}
                Err(error) => {
                    let responder = cell_for_task
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .take();
                    if let Some(responder) = responder {
                        responder.respond_with_error(wire::into_jsonrpc_error(error))?;
                    }
                    return Ok(());
                }
            }
            let dispatcher = FacadeDispatcher { state };
            let respond = |result: Result<FeatureOperationResponse, Error>| {
                let responder = cell_for_task
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .take();
                match responder {
                    Some(responder) => match result {
                        Ok(response) => match serde_json::to_value(response) {
                            Ok(value) => responder.respond(value),
                            Err(error) => responder.respond_with_error(
                                Error::internal_error()
                                    .data(format!("facade response encoding failed: {error}")),
                            ),
                        },
                        Err(error) => responder.respond_with_error(error),
                    },
                    None => {
                        Err(Error::internal_error().data("facade response was already answered"))
                    }
                }
            };
            if let Err(error) = dispatcher.dispatch(&spawn_method, &params, respond).await {
                tracing::warn!("facade dispatch for {spawn_method} failed: {error}");
            }
            Ok(())
        }) {
            tracing::warn!("facade dispatch for {method_for_task} could not be spawned: {error}");
            if let Some(responder) = responder_cell
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .take()
            {
                let _ = responder.respond_with_error(wire::into_jsonrpc_error(EchoSdkError::new(
                    ExtensionErrorCode::HostShuttingDown,
                    format!("connection can no longer host facade dispatch: {error}"),
                    Retryability::Never,
                )));
            }
        }
        Ok(Handled::Yes)
    }

    fn describe_chain(&self) -> impl std::fmt::Debug {
        "FacadeDispatcher"
    }
}

/// The unified facade admission ladder. Returns the typed response or the
/// typed extension error; both are answered on the claimed request.
async fn facade_admission(
    state: &Arc<CoreProfileState>,
    method: &str,
    params: &serde_json::Value,
) -> Result<FeatureOperationResponse, EchoSdkError> {
    let services = state.services().map_err(|error| {
        EchoSdkError::new(
            ExtensionErrorCode::HostShuttingDown,
            error.to_string(),
            Retryability::Never,
        )
    })?;
    if !services.is_extended().await {
        return Err(EchoSdkError::new(
            ExtensionErrorCode::InvalidRequest,
            "facade methods require a negotiated extension connection",
            Retryability::Never,
        ));
    }
    if services.ensure_admission().is_err() {
        return Err(EchoSdkError::new(
            ExtensionErrorCode::HostShuttingDown,
            "ACP Host is shutting down",
            Retryability::Never,
        ));
    }
    if !state
        .advertisement
        .declares(ExtensionCapability::FeatureSurfaces)
    {
        return Err(EchoSdkError::new(
            ExtensionErrorCode::ExtensionCapabilityMismatch,
            "capability feature_surfaces is not advertised",
            Retryability::Never,
        ));
    }
    let mut request: FeatureOperationRequest =
        serde_json::from_value(params.clone()).map_err(|error| {
            facade_error(
                ExtensionErrorCode::InvalidRequest,
                format!("facade request payload is malformed: {error}"),
                Retryability::Never,
                method,
                FacadeFailureDetail::default(),
            )
        })?;
    if let Err(reason) = request.validate() {
        return Err(facade_error(
            ExtensionErrorCode::InvalidValue,
            reason,
            Retryability::Never,
            method,
            FacadeFailureDetail {
                operation: Some(request.operation.clone()),
                signature_digest: Some(request.signature_digest.clone()),
                ..FacadeFailureDetail::default()
            },
        ));
    }
    if request.arguments.len() > state.limits.max_facade_operation_args {
        return Err(facade_error(
            ExtensionErrorCode::PayloadTooLarge,
            format!(
                "facade operation has {} arguments; limit is {}",
                request.arguments.len(),
                state.limits.max_facade_operation_args
            ),
            Retryability::Never,
            method,
            FacadeFailureDetail {
                operation: Some(request.operation.clone()),
                ..FacadeFailureDetail::default()
            },
        ));
    }
    let catalog = CompiledOperationCatalog::global().map_err(|reason| {
        EchoSdkError::new(
            ExtensionErrorCode::InvalidConfig,
            reason,
            Retryability::Never,
        )
    })?;
    let (family, required_feature, handler_operation) = if method == "_echo_agent/facade/invoke" {
        // Exact source identities resolve through the invoke table; closed
        // family-operation identities dispatch through the same family
        // handler as `<family>/op` — the generic invoke surface is
        // executable for every frozen family operation (plan 07 todo 1).
        let (route_family, route_feature, handler_operation, digests, required_features, semantics) =
            match catalog.invoke_route(&request.operation) {
                Some(route) => (
                    route.family.clone(),
                    route.required_feature.clone(),
                    route.handler_operation.clone(),
                    route.signature_digests.clone(),
                    route.required_features.clone(),
                    route.feature_semantics.clone(),
                ),
                None => match catalog.family_for_operation(&request.operation) {
                    Some(operation_family) => {
                        let family_method = if operation_family == "invoke" {
                            "_echo_agent/facade/invoke".to_string()
                        } else {
                            format!("_echo_agent/{operation_family}/op")
                        };
                        let summary = catalog.family_method(&family_method).ok_or_else(|| {
                            facade_error(
                                ExtensionErrorCode::InvalidValue,
                                format!(
                                    "operation {} has no executable family surface",
                                    request.operation
                                ),
                                Retryability::Never,
                                method,
                                FacadeFailureDetail {
                                    operation: Some(request.operation.clone()),
                                    ..FacadeFailureDetail::default()
                                },
                            )
                        })?;
                        (
                            operation_family.to_string(),
                            summary.required_feature.clone(),
                            None,
                            summary
                                .operation_signatures
                                .get(&request.operation)
                                .cloned()
                                .unwrap_or_default(),
                            summary.required_features.clone(),
                            summary.feature_semantics.clone(),
                        )
                    }
                    None => {
                        return Err(facade_error(
                            ExtensionErrorCode::InvalidValue,
                            format!(
                                "operation {} is not a canonical route of this contract",
                                request.operation
                            ),
                            Retryability::Never,
                            method,
                            FacadeFailureDetail {
                                operation: Some(request.operation.clone()),
                                ..FacadeFailureDetail::default()
                            },
                        ));
                    }
                },
            };
        if !digests.is_empty()
            && !digests
                .iter()
                .any(|digest| digest == &request.signature_digest)
        {
            return Err(facade_error(
                ExtensionErrorCode::InvalidValue,
                format!(
                    "signature digest for {} is not part of the canonical route",
                    request.operation
                ),
                Retryability::Never,
                method,
                FacadeFailureDetail {
                    operation: Some(request.operation.clone()),
                    signature_digest: Some(request.signature_digest.clone()),
                    ..FacadeFailureDetail::default()
                },
            ));
        }
        let compiled = state.advertisement.features.iter();
        let all_compiled = |required: &[String]| {
            required
                .iter()
                .all(|feature| compiled.clone().any(|f| f == feature))
        };
        let any_compiled = |required: &[String]| {
            required.is_empty()
                || required
                    .iter()
                    .any(|feature| state.advertisement.features.iter().any(|f| f == feature))
        };
        let requirements_met = match semantics.as_str() {
            "all_of" => all_compiled(&required_features),
            "any_of" => any_compiled(&required_features),
            _ => all_compiled(&required_features),
        };
        if !requirements_met {
            return Err(facade_error(
                ExtensionErrorCode::FeatureUnavailable,
                format!(
                    "required feature set for {} is not compiled",
                    request.operation
                ),
                Retryability::Never,
                method,
                FacadeFailureDetail {
                    operation: Some(request.operation.clone()),
                    required_feature: route_feature.clone(),
                    ..FacadeFailureDetail::default()
                },
            ));
        }
        (route_family, route_feature, handler_operation)
    } else {
        let summary = catalog.family_method(method).ok_or_else(|| {
            EchoSdkError::new(
                ExtensionErrorCode::InvalidRequest,
                format!("method {method} is not a facade family surface"),
                Retryability::Never,
            )
        })?;
        let family_qualified_tool =
            crate::core_profile::facade::tools::TOOL_FAMILIES.contains(&summary.family.as_str())
                && summary.operations.iter().any(|operation| {
                    request.operation == format!("{}.{operation}", summary.family)
                });
        let operation_key = if family_qualified_tool {
            request
                .operation
                .split_once('.')
                .map(|(_, operation)| operation)
                .unwrap_or(request.operation.as_str())
        } else {
            request.operation.as_str()
        };
        if !summary.operations.is_empty()
            && !summary.operation_signatures.contains_key(operation_key)
        {
            return Err(facade_error(
                ExtensionErrorCode::InvalidValue,
                format!(
                    "operation {} is not implemented by family {}",
                    request.operation, summary.family
                ),
                Retryability::Never,
                method,
                FacadeFailureDetail {
                    operation: Some(request.operation.clone()),
                    signature_digest: Some(request.signature_digest.clone()),
                    ..FacadeFailureDetail::default()
                },
            ));
        }
        // Operation-bearing families compare the request digest against the
        // frozen per-operation envelope digest; protocol-native families
        // (structured output validation) carry no family operations and
        // their typed DTO is the contract.
        if !summary.operations.is_empty() {
            let allowed_digests =
                summary
                    .operation_signatures
                    .get(operation_key)
                    .ok_or_else(|| {
                        facade_error(
                            ExtensionErrorCode::InvalidValue,
                            format!(
                                "operation {} has no executable signature",
                                request.operation
                            ),
                            Retryability::Never,
                            method,
                            FacadeFailureDetail {
                                operation: Some(request.operation.clone()),
                                signature_digest: Some(request.signature_digest.clone()),
                                ..FacadeFailureDetail::default()
                            },
                        )
                    })?;
            if !allowed_digests
                .iter()
                .any(|digest| digest == &request.signature_digest)
            {
                return Err(facade_error(
                    ExtensionErrorCode::InvalidValue,
                    format!(
                        "signature digest for {} is not canonical",
                        request.operation
                    ),
                    Retryability::Never,
                    method,
                    FacadeFailureDetail {
                        operation: Some(request.operation.clone()),
                        signature_digest: Some(request.signature_digest.clone()),
                        ..FacadeFailureDetail::default()
                    },
                ));
            }
        }
        // The family surface's frozen feature requirement uses the same
        // all-of/any-of rules as invoke routes (design §13): advertisement,
        // preflight and handler must agree on one semantics.
        let requirements_met = match summary.feature_semantics.as_str() {
            "all_of" => summary
                .required_features
                .iter()
                .all(|feature| state.advertisement.features.iter().any(|f| f == feature)),
            "any_of" => {
                summary.required_features.is_empty()
                    || summary
                        .required_features
                        .iter()
                        .any(|feature| state.advertisement.features.iter().any(|f| f == feature))
            }
            _ => summary
                .required_features
                .iter()
                .all(|feature| state.advertisement.features.iter().any(|f| f == feature)),
        };
        if !requirements_met {
            return Err(facade_error(
                ExtensionErrorCode::FeatureUnavailable,
                format!("required feature set for {method} is not compiled"),
                Retryability::Never,
                method,
                FacadeFailureDetail {
                    operation: Some(request.operation.clone()),
                    required_feature: summary.required_feature.clone(),
                    ..FacadeFailureDetail::default()
                },
            ));
        }
        (
            summary.family.clone(),
            summary.required_feature.clone(),
            None,
        )
    };
    if let Some(feature) = &required_feature
        && !state.advertisement.features.iter().any(|f| f == feature)
    {
        return Err(facade_error(
            ExtensionErrorCode::FeatureUnavailable,
            format!("feature {feature} is not compiled into this Host"),
            Retryability::Never,
            method,
            FacadeFailureDetail {
                required_feature: Some(feature.clone()),
                ..FacadeFailureDetail::default()
            },
        ));
    }
    if let Some(handler_operation) = handler_operation {
        if matches!(
            family.as_str(),
            "workflow"
                | "delivery"
                | "trace"
                | "permission"
                | "mcp"
                | "a2a"
                | "lsp"
                | "channels"
                | "topology"
                | "project_rules"
        ) {
            let agent = request
                .handle
                .as_ref()
                .ok_or_else(|| {
                    facade_error(
                        ExtensionErrorCode::InvalidValue,
                        "source family operation requires an Agent receiver",
                        Retryability::Never,
                        method,
                        FacadeFailureDetail {
                            operation: Some(request.operation.clone()),
                            ..FacadeFailureDetail::default()
                        },
                    )
                })?
                .clone();
            state
                .handles
                .check_shape_and_generation(&agent, HandleKind::Agent, method)?;
            state.handles.agent(&agent)?;
            let session = match request.arguments.first() {
                Some(WireValue::Handle(handle)) if handle.kind == HandleKind::Session => {
                    handle.clone()
                }
                _ => {
                    return Err(facade_error(
                        ExtensionErrorCode::InvalidValue,
                        "source family operation requires a Session receiver argument",
                        Retryability::Never,
                        method,
                        FacadeFailureDetail {
                            operation: Some(request.operation.clone()),
                            ..FacadeFailureDetail::default()
                        },
                    ));
                }
            };
            state
                .handles
                .check_shape_and_generation(&session, HandleKind::Session, method)?;
            let session_record = state.handles.session(&session)?;
            if session_record.agent_handle_id != agent.id {
                return Err(facade_error(
                    ExtensionErrorCode::InvalidValue,
                    "source family Session does not belong to the Agent receiver",
                    Retryability::Never,
                    method,
                    FacadeFailureDetail {
                        operation: Some(request.operation.clone()),
                        ..FacadeFailureDetail::default()
                    },
                ));
            }
            request.handle = Some(session);
            request.arguments.remove(0);
            for argument in &mut request.arguments {
                if let WireValue::Handle(handle) = argument {
                    let value = serde_json::to_value(handle.clone()).map_err(|error| {
                        facade_error(
                            ExtensionErrorCode::FrameworkError,
                            format!("source family resource handle projection failed: {error}"),
                            Retryability::Never,
                            method,
                            FacadeFailureDetail {
                                operation: Some(request.operation.clone()),
                                ..FacadeFailureDetail::default()
                            },
                        )
                    })?;
                    *argument = WireValue::from_json(value).map_err(|error| {
                        facade_error(
                            ExtensionErrorCode::FrameworkError,
                            format!("source family resource handle projection failed: {error}"),
                            Retryability::Never,
                            method,
                            FacadeFailureDetail {
                                operation: Some(request.operation.clone()),
                                ..FacadeFailureDetail::default()
                            },
                        )
                    })?;
                }
            }
        }
        request.operation = handler_operation;
    }
    // Structured-output validation is contract-level and self-contained:
    // it ships with the facade runtime itself (todo 3).
    if method == "_echo_agent/structured_output/validate" {
        return match structured_output::validate(&request) {
            Ok(value) => Ok(echo_sdk_protocol::methods::FeatureOperationResponse { value }),
            Err(error) => Err(error),
        };
    }
    // Family dispatch (todos 4–5): compiled families route to their real
    // framework services; the rest answer with the typed feature failure
    // instead of simulating a result. Families that need a session resolve
    // its authorities here; resource-free families (state, eval, improve)
    // dispatch without one.
    let owner_handle = request
        .handle
        .as_ref()
        .filter(|h| h.kind == HandleKind::Session)
        .or_else(|| match request.arguments.first() {
            Some(WireValue::Handle(handle)) if handle.kind == HandleKind::Session => Some(handle),
            _ => None,
        });
    let session_record = owner_handle
        .map(|handle| {
            state
                .handles
                .check_shape_and_generation(handle, HandleKind::Session, method)?;
            state.handles.session(handle)
        })
        .transpose()?;
    let _operation = match &session_record {
        Some(record) => {
            // Close owns the write guard while awaiting resource callbacks.
            // Reentry must fail before waiting for that guard; the check below
            // still rejects a close that races with read-guard admission.
            if record.closing.load(std::sync::atomic::Ordering::Acquire) {
                return Err(wire::sdk_error(
                    ExtensionErrorCode::ClosedHandle,
                    "Session is closing",
                    Retryability::Never,
                    method,
                ));
            }
            Some(record.lifecycle.read().await)
        }
        None => None,
    };
    if session_record
        .as_ref()
        .is_some_and(|record| record.closing.load(std::sync::atomic::Ordering::Acquire))
    {
        return Err(wire::sdk_error(
            ExtensionErrorCode::ClosedHandle,
            "Session is closing",
            Retryability::Never,
            method,
        ));
    }
    let session = session_authorities_of(state, &request).await;
    let response = move |value: WireResult| {
        value.map(|value| echo_sdk_protocol::methods::FeatureOperationResponse { value })
    };
    let missing_session = || {
        facade_error(
            ExtensionErrorCode::InvalidValue,
            "family operation requires a session handle",
            Retryability::Never,
            method,
            FacadeFailureDetail {
                operation: Some(request.operation.clone()),
                ..FacadeFailureDetail::default()
            },
        )
    };
    let limits = FacadeFamilyLimits {
        max_resources: state.limits.max_facade_resources,
        page: state.limits.max_facade_page_items,
    };
    // Keep large family coroutines on the heap rather than nesting their
    // debug frames in the admission coroutine on Tokio's bounded stack.
    let dispatch = Box::pin(async {
        match family.as_str() {
            "invoke" if request.operation == "facade.resource.close" => {
                let (_authorities, owner) = session.ok_or_else(missing_session)?;
                if request.arguments.len() != 1 {
                    return Err(facade_error(
                        ExtensionErrorCode::InvalidValue,
                        "facade.resource.close accepts exactly one FacadeResource argument",
                        Retryability::Never,
                        method,
                        FacadeFailureDetail {
                            operation: Some(request.operation.clone()),
                            ..FacadeFailureDetail::default()
                        },
                    ));
                }
                let resource = match request.arguments.first() {
                    Some(WireValue::Handle(handle)) => handle,
                    _ => {
                        return Err(facade_error(
                            ExtensionErrorCode::InvalidValue,
                            "facade.resource.close requires a FacadeResource handle",
                            Retryability::Never,
                            method,
                            FacadeFailureDetail {
                                operation: Some(request.operation.clone()),
                                ..FacadeFailureDetail::default()
                            },
                        ));
                    }
                };
                response(
                    state
                        .facade
                        .close_resource(&state.handles, &owner, resource, &request.operation)
                        .await
                        .map(WireValue::Bool),
                )
            }
            "workflow" | "a2a" | "shell"
                if request.operation.ends_with(".stream.next")
                    || request.operation.ends_with(".stream.cancel")
                    || request.operation.ends_with(".stream.close") =>
            {
                let (_authorities, owner) = session.ok_or_else(missing_session)?;
                response(
                    stream::dispatch_control(
                        &state.facade.streams,
                        &state.handles,
                        &owner,
                        &family,
                        &request,
                    )
                    .await,
                )
            }
            "memory" => {
                let (authorities, owner) = session.ok_or_else(missing_session)?;
                response(
                    memory::dispatch(
                        &authorities,
                        &state.facade.memory_store_resources,
                        &state.handles,
                        &owner,
                        &request,
                        limits.page,
                    )
                    .await,
                )
            }
            "workflow" => {
                let (authorities, owner) = session.ok_or_else(missing_session)?;
                #[cfg(feature = "sdk-extension-bridge")]
                if matches!(
                    request.operation.as_str(),
                    "workflow.extension.run" | "workflow.extension.run_stream"
                ) {
                    use echo_agent::workflow::Workflow as _;
                    let extension = match request.arguments.first() {
                        Some(WireValue::Handle(handle)) if handle.kind == HandleKind::Extension => {
                            handle.clone()
                        }
                        _ => {
                            return Err(facade_error(
                                ExtensionErrorCode::InvalidValue,
                                "workflow.extension.run requires a Workflow extension handle",
                                Retryability::Never,
                                method,
                                FacadeFailureDetail::default(),
                            ));
                        }
                    };
                    let input = match request.arguments.get(1) {
                        Some(WireValue::String(input)) => input.clone(),
                        _ => {
                            return Err(facade_error(
                                ExtensionErrorCode::InvalidValue,
                                "workflow.extension.run requires input text",
                                Retryability::Never,
                                method,
                                FacadeFailureDetail::default(),
                            ));
                        }
                    };
                    let mut proxy =
                        crate::core_profile::extension_bridge::ExtensionAgentComponentProxy::new(
                            state.extension_bridge.clone(),
                            extension,
                            owner.clone(),
                        )
                        .filter(|proxy| {
                            proxy.component()
                                == echo_sdk_protocol::methods::AgentComponentKindWire::Workflow
                        })
                        .ok_or_else(|| {
                            facade_error(
                                ExtensionErrorCode::InvalidValue,
                                "extension is not a Workflow component",
                                Retryability::Never,
                                method,
                                FacadeFailureDetail::default(),
                            )
                        })?;
                    if request.operation == "workflow.extension.run_stream" {
                        use futures::StreamExt as _;
                        let (anchor, _) = state.handles.register_facade_resource(
                            state.limits.max_facade_resources,
                            "workflow",
                            "workflow.extension_stream",
                            Some(&owner),
                            method,
                        )?;
                        let producer = match state.facade.streams.open_ephemeral(
                            &state.handles,
                            &anchor,
                            &owner,
                            &request.operation,
                        ) {
                            Ok(producer) => producer,
                            Err(error) => {
                                let _ = state.handles.close_facade_resource(&anchor, method);
                                return Err(error);
                            }
                        };
                        let sender = producer.sender.clone();
                        let cancel = producer.cleanup.clone();
                        let original_owner = producer.clone();
                        let background = tokio::spawn(async move {
                            let source = proxy
                                .run_stream_observed(
                                    &input,
                                    cancel.clone(),
                                    Some(original_owner.clone()),
                                )
                                .await;
                            let mut source = match source {
                                Ok(source) => source.fuse(),
                                Err(error) => {
                                    let error =
                                        wire::framework_error(&error, "_echo_agent/workflow/op");
                                    if !original_owner.record_observation(&error) {
                                        let _ = sender.send(Err(error)).await;
                                    }
                                    return;
                                }
                            };
                            loop {
                                let item = tokio::select! {
                                    () = cancel.cancelled() => break,
                                    item = source.next() => item,
                                };
                                let Some(item) = item else { return };
                                let item = item
                                    .map_err(|error| {
                                        wire::framework_error(&error, "_echo_agent/workflow/op")
                                    })
                                    .and_then(workflow::workflow_event_value);
                                if let Err(error) = &item {
                                    if original_owner.record_observation(error) {
                                        // Observation leaves the original producer alive;
                                        // keep forwarding its eventual Completed payload.
                                        continue;
                                    }
                                }
                                let sent = tokio::select! {
                                    () = cancel.cancelled() => false,
                                    result = sender.send(item) => result.is_ok(),
                                };
                                if !sent {
                                    cancel.cancel();
                                    break;
                                }
                            }
                            while let Some(event) = source.next().await {
                                if let Err(error) = event {
                                    original_owner.record_observation(&wire::framework_error(
                                        &error,
                                        "_echo_agent/workflow/op",
                                    ));
                                }
                            }
                        });
                        state
                            .facade
                            .streams
                            .attach_background(&producer, background);
                        return response(Ok(WireValue::Handle(producer.handle)));
                    }
                    let output = proxy.run(&input).await.map_err(|error| {
                        if matches!(
                            &error,
                            echo_agent::error::ReactError::ExecutionNotSettled { .. }
                        ) {
                            return wire::framework_error(&error, method);
                        }
                        wire::sdk_error(
                            ExtensionErrorCode::FrameworkError,
                            wire::bounded_framework_message(&error.to_string()),
                            Retryability::Never,
                            method,
                        )
                    })?;
                    let output = serde_json::to_value(output).map_err(|error| {
                        wire::sdk_error(
                            ExtensionErrorCode::SerializationViolation,
                            error.to_string(),
                            Retryability::Never,
                            method,
                        )
                    })?;
                    return response(WireValue::from_json(output).map_err(|error| {
                        wire::sdk_error(
                            ExtensionErrorCode::SerializationViolation,
                            error.to_string(),
                            Retryability::Never,
                            method,
                        )
                    }));
                }
                #[cfg(feature = "sdk-extension-bridge")]
                let checkpoint_store =
                    crate::core_profile::extension_bridge::latest_agent_component_proxy(
                        state,
                        state.extension_bridge.clone(),
                        &owner,
                        echo_sdk_protocol::methods::AgentComponentKindWire::WorkflowCheckpointStore,
                    )
                    .map(|proxy| Arc::new(proxy) as Arc<dyn echo_agent::workflow::CheckpointStore>);
                #[cfg(not(feature = "sdk-extension-bridge"))]
                let checkpoint_store = None;
                response(
                    workflow::dispatch(
                        &state.handles,
                        &state.facade.workflow_graphs,
                        &state.facade.workflow_states,
                        &state.facade.streams,
                        &authorities,
                        checkpoint_store,
                        &owner,
                        &request,
                        limits,
                    )
                    .await,
                )
            }
            "state" => response(state_delivery::dispatch_state(&state.state_store, &request).await),
            #[cfg(feature = "framework-telemetry")]
            "telemetry" => response(telemetry::dispatch(state.clone(), &request).await),
            "delivery" => {
                let (_authorities, owner) = session.ok_or_else(missing_session)?;
                response(
                    state_delivery::dispatch_delivery(
                        &state.handles,
                        &state.facade.delivery_ledgers,
                        &owner,
                        &request,
                        limits,
                    )
                    .await,
                )
            }
            "trace" => {
                let (_authorities, owner) = session.ok_or_else(missing_session)?;
                response(
                    observability::dispatch_trace(
                        &state.handles,
                        &state.facade.trace_stores,
                        &owner,
                        &request,
                        limits,
                    )
                    .await,
                )
            }
            #[cfg(feature = "framework-eval")]
            "eval" => response(observability::dispatch_eval(&request).await),
            #[cfg(feature = "framework-improve")]
            "improve" => response(observability::dispatch_improve(&request).await),
            #[cfg(feature = "framework-human-loop")]
            "permission" => {
                let (_authorities, _owner) = session.ok_or_else(missing_session)?;
                response(permission::dispatch(state, &request).await)
            }
            #[cfg(feature = "framework-content-guard")]
            "content_guard" => response(tools::dispatch_content_guard(&request)),
            #[cfg(feature = "framework-project-rules")]
            "project_rules" => {
                let (authorities, _owner) = session.ok_or_else(missing_session)?;
                response(tools::dispatch_project_rules(
                    &authorities.working_dir,
                    &request,
                ))
            }
            "mcp" | "a2a" | "lsp" | "topology" => {
                let (_authorities, owner) = session.ok_or_else(missing_session)?;
                response(
                    integrations::dispatch(
                        &state.handles,
                        &family,
                        &state.facade.integrations,
                        &state.facade.streams,
                        &owner,
                        &request,
                        limits.max_resources,
                    )
                    .await,
                )
            }
            #[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
            "channels" => {
                let (_authorities, owner) = session.ok_or_else(missing_session)?;
                response(
                    integrations::dispatch_channels(
                        &state.handles,
                        &state.facade.integrations,
                        &state.facade.streams,
                        &state.extension_bridge,
                        &owner,
                        &request,
                        limits.max_resources,
                    )
                    .await,
                )
            }
            #[cfg(feature = "sdk-extension-bridge")]
            "shell"
                if matches!(
                    request.operation.as_str(),
                    "sandbox.extension.run_stream" | "shell.sandbox.extension.run_stream"
                ) =>
            {
                let (_authorities, owner) = session.ok_or_else(missing_session)?;
                if request.arguments.len() != 2 {
                    return Err(facade_error(
                        ExtensionErrorCode::InvalidValue,
                        "sandbox.extension.run_stream requires [SandboxExecutor extension, command]",
                        Retryability::Never,
                        method,
                        FacadeFailureDetail::default(),
                    ));
                }
                let extension = match request.arguments.first() {
                    Some(WireValue::Handle(handle)) if handle.kind == HandleKind::Extension => {
                        handle.clone()
                    }
                    _ => {
                        return Err(facade_error(
                            ExtensionErrorCode::InvalidValue,
                            "sandbox.extension.run_stream requires an Extension handle",
                            Retryability::Never,
                            method,
                            FacadeFailureDetail::default(),
                        ));
                    }
                };
                let command = request
                    .arguments
                    .get(1)
                    .cloned()
                    .ok_or_else(|| {
                        facade_error(
                            ExtensionErrorCode::InvalidValue,
                            "sandbox command is missing",
                            Retryability::Never,
                            method,
                            FacadeFailureDetail::default(),
                        )
                    })?
                    .into_json()
                    .map_err(|error| {
                        facade_error(
                            ExtensionErrorCode::InvalidValue,
                            error.to_string(),
                            Retryability::Never,
                            method,
                            FacadeFailureDetail::default(),
                        )
                    })
                    .and_then(|value| {
                        serde_json::from_value::<echo_agent::sandbox::SandboxCommand>(value)
                            .map_err(|error| {
                                facade_error(
                                    ExtensionErrorCode::InvalidValue,
                                    format!("sandbox command is malformed: {error}"),
                                    Retryability::Never,
                                    method,
                                    FacadeFailureDetail::default(),
                                )
                            })
                    })?;
                let proxy =
                    crate::core_profile::extension_bridge::ExtensionAgentComponentProxy::new(
                        state.extension_bridge.clone(),
                        extension,
                        owner.clone(),
                    )
                    .filter(|proxy| {
                        proxy.component()
                            == echo_sdk_protocol::methods::AgentComponentKindWire::SandboxExecutor
                    })
                    .ok_or_else(|| {
                        facade_error(
                            ExtensionErrorCode::InvalidValue,
                            "extension is not a SandboxExecutor component",
                            Retryability::Never,
                            method,
                            FacadeFailureDetail::default(),
                        )
                    })?;
                let (anchor, _) = state.handles.register_facade_resource(
                    state.limits.max_facade_resources,
                    "shell",
                    "shell.sandbox_extension_stream",
                    Some(&owner),
                    method,
                )?;
                let producer = match state.facade.streams.open_ephemeral_cooperative(
                    &state.handles,
                    &anchor,
                    &owner,
                    &request.operation,
                ) {
                    Ok(producer) => producer,
                    Err(error) => {
                        let _ = state.handles.close_facade_resource(&anchor, method);
                        return Err(error);
                    }
                };
                let sender = producer.sender.clone();
                let cancel = producer.cleanup.clone();
                let original_owner = producer.clone();
                let background = tokio::spawn(async move {
                    let source = proxy
                        .execute_stream_with_cancel(
                            command,
                            cancel.clone(),
                            Some(original_owner.clone()),
                        )
                        .await;
                    let source = match source {
                        Ok(source) => source,
                        Err(error) => {
                            let error = wire::framework_error(&error, "_echo_agent/shell/op");
                            if !original_owner.record_observation(&error) {
                                let _ = sender.send(Err(error)).await;
                            }
                            return;
                        }
                    };
                    drive_sandbox_facade_source(source, sender, cancel).await;
                });
                state
                    .facade
                    .streams
                    .attach_background(&producer, background);
                response(Ok(WireValue::Handle(producer.handle)))
            }
            "source_operation" => response(
                Box::pin(source_operations::dispatch(state, &state.handles, &request)).await,
            ),
            tool_family if tools::TOOL_FAMILIES.contains(&tool_family) => {
                let (authorities, owner) = session.ok_or_else(missing_session)?;
                response(
                    tools::dispatch_tool(
                        tool_family,
                        &authorities,
                        &owner,
                        &state.facade,
                        &request,
                    )
                    .await,
                )
            }
            _ => Err(facade_error(
                ExtensionErrorCode::FeatureUnavailable,
                format!("facade family {family} is not available in this Host build"),
                Retryability::Never,
                method,
                FacadeFailureDetail {
                    required_feature,
                    ..FacadeFailureDetail::default()
                },
            )),
        }
    });
    if let Some(record) = session_record.as_ref() {
        tokio::select! {
            result = dispatch => result,
            _ = record.cancel_operations.cancelled() => Err(wire::sdk_error(
                ExtensionErrorCode::ClosedHandle, "Session closed during operation", Retryability::Never, method)),
        }
    } else {
        dispatch.await
    }
}

/// Resolve the session authority services addressed by one family
/// operation, together with the owning ACP session id (workflow-family
/// resources are owner-checked against it). Family operations address
/// their session through the request handle (kind `session`); without it
/// the Host refuses to guess.
/// Project and drain this one original Sandbox producer. The EOF regression
/// uses the production path, with no second stream lifecycle implementation.
#[cfg(feature = "sdk-extension-bridge")]
async fn drive_sandbox_facade_source<S>(
    source: S,
    sender: tokio::sync::mpsc::Sender<stream::FacadeStreamItem>,
    cancel: tokio_util::sync::CancellationToken,
) where
    S: futures::Stream<Item = echo_agent::sandbox::SandboxStreamEvent> + Unpin,
{
    use futures::StreamExt as _;
    // Draining cancellation shares this same source. A
    // natural EOF must remain EOF when cleanup polls again.
    let mut source = source.fuse();
    loop {
        let item = tokio::select! {
            () = cancel.cancelled() => break,
            item = source.next() => item,
        };
        // Natural EOF closes the sender and preserves queued payloads. It
        // does not request cancellation from the facade consumer.
        let Some(event) = item else { return };
        let item = serde_json::to_value(event)
            .map_err(|error| {
                wire::sdk_error(
                    ExtensionErrorCode::SerializationViolation,
                    error.to_string(),
                    Retryability::Never,
                    "_echo_agent/shell/op",
                )
            })
            .and_then(|value| {
                WireValue::from_json(value).map_err(|error| {
                    wire::sdk_error(
                        ExtensionErrorCode::SerializationViolation,
                        error.to_string(),
                        Retryability::Never,
                        "_echo_agent/shell/op",
                    )
                })
            });
        let sent = tokio::select! {
            () = cancel.cancelled() => false,
            sent = sender.send(item) => sent.is_ok(),
        };
        if !sent {
            break;
        }
    }
    cancel.cancel();
    while source.next().await.is_some() {}
}

async fn session_authorities_of(
    state: &CoreProfileState,
    request: &echo_sdk_protocol::methods::FeatureOperationRequest,
) -> Option<(
    std::sync::Arc<crate::factory::SessionAuthorityServices>,
    String,
)> {
    let handle = request.handle.as_ref()?;
    if state
        .handles
        .check_shape_and_generation(
            handle,
            echo_sdk_protocol::handle::HandleKind::Session,
            "_echo_agent/facade/invoke",
        )
        .is_err()
    {
        return None;
    }
    let services = state.services().ok()?;
    let acp_session_id = state.handles.session(handle).ok()?.acp_session_id.clone();
    let _ = services;
    let authorities = state.session_factory.session_services(&acp_session_id)?;
    Some((authorities, acp_session_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real official ACP/profile forward -> reverse -> forward projection.
    /// The foreign work starts inside its callback and stays held; this tests
    /// error identity and original admission, not normal transport shutdown.
    #[cfg(feature = "sdk-extension-bridge")]
    #[tokio::test]
    async fn workflow_direct_run_preserves_observation_while_original_work_is_held()
    -> Result<(), String> {
        use agent_client_protocol::{
            Agent as AcpAgent, ByteStreams, Client, ConnectionTo, Responder, UntypedMessage,
        };
        use echo_sdk_protocol::methods::*;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};
        use tokio_util::sync::CancellationToken;
        struct OriginalWorkGuard(Arc<AtomicUsize>);
        impl Drop for OriginalWorkGuard {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::AcqRel);
            }
        }
        let root = tempfile::tempdir().map_err(|error| error.to_string())?;
        let config: crate::SdkHostConfig = serde_json::from_value(serde_json::json!({"schema_version":1,"default_agent":{
            "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
            "agent":{"name":"fixture","enable_tools":true}}})).map_err(|error| error.to_string())?;
        let prepared = config.prepare().map_err(|error| error.to_string())?;
        let state = CoreProfileState::install(
            root.path(),
            crate::config::SdkProfileLimits::default(),
            prepared.framework,
            prepared.llm_client,
        )
        .map_err(|error| error.to_string())?;
        let profile = super::super::SdkCoreProfile {
            state: state.clone(),
        };
        let adapter = echo_agent::acp::AcpAgentAdapter::new(state.session_factory.clone())
            .with_profile(profile);
        let close_owner = adapter.close_owner();
        let entered = CancellationToken::new();
        let release = CancellationToken::new();
        let finish = CancellationToken::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let calls = Arc::new(AtomicUsize::new(0));
        let original = Arc::new(std::sync::Mutex::new(None::<tokio::task::JoinHandle<()>>));
        let known_error = Arc::new(std::sync::Mutex::new(None::<EchoSdkError>));
        let (host_io, client_io) = tokio::io::duplex(32_768);
        let (host_read, host_write) = tokio::io::split(host_io);
        let (client_read, client_write) = tokio::io::split(client_io);
        let host = agent_client_protocol::ConnectTo::connect_to(
            adapter,
            ByteStreams::new(host_write.compat_write(), host_read.compat()),
        );
        let (observed_tx, observed_rx) = tokio::sync::oneshot::channel::<Result<(), String>>();
        let client = Client.builder().on_receive_request({
            let entered=entered.clone();let release=release.clone();let drops=drops.clone();let calls=calls.clone();let original=original.clone();let known_error=known_error.clone();
            async move |call: ExtensionInvokeCall, responder: Responder<ExtensionInvokeOutcome>, _connection: ConnectionTo<AcpAgent>| {
                let error = match call.invocation {
                    ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {component: AgentComponentKindWire::Workflow, call: AgentComponentCallInputWire::WorkflowRun {input}}) => {
                        calls.fetch_add(1, Ordering::AcqRel);
                        if input == "ordinary" {
                            wire::sdk_error(ExtensionErrorCode::ExtensionFailed, "ordinary workflow failure 中文🙂", Retryability::Never, "_echo_agent/extension/invoke")
                        } else {
                            // Identities belong to this actual foreign work admission,
                            // never to the JSON-RPC request id or a guessed SDK Task.
                            let run_id=format!("workflow-original-{}-中文🙂",uuid::Uuid::new_v4());
                            let execution_id=format!("workflow-execution-{}-🧪",uuid::Uuid::new_v4());
                            let error=wire::not_settled_error(&run_id,Some(&execution_id),"_echo_agent/extension/invoke");
                            *known_error.lock().map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?=Some(error.clone());
                            let guard=OriginalWorkGuard(drops.clone());let owner_release=release.clone();let owner_entered=entered.clone();
                            let task=tokio::spawn(async move {let _guard=guard;owner_entered.cancel();owner_release.cancelled().await;});
                            *original.lock().map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?=Some(task);
                            entered.cancelled().await;
                            error
                        }
                    }
                    _ => return Err(agent_client_protocol::Error::internal_error().data("unexpected Workflow projection callback")),
                };
                responder.respond(ExtensionInvokeOutcome::Error {error})
            }
        }, agent_client_protocol::on_receive_request!()).connect_with(ByteStreams::new(client_write.compat_write(), client_read.compat()), {
            let finish=finish.clone();let release=release.clone();let drops=drops.clone();let entered=entered.clone();let calls=calls.clone();let known_error=known_error.clone();
            async move |connection| {
                let _release_on_exit=release.clone().drop_guard();
                let observed=async {
                    let mut initialize=agent_client_protocol::schema::v1::InitializeRequest::new(agent_client_protocol::schema::ProtocolVersion::V1);
                    let mut meta=agent_client_protocol::schema::v1::Meta::new();
                    meta.insert("echo_agent".to_string(),serde_json::to_value(echo_sdk_protocol::capability::EchoAgentClientHello {
                        extension_protocol_version:echo_sdk_protocol::EXTENSION_PROTOCOL_VERSION,contract_digest:state.advertisement.contract_digest.clone(),source_contract_digest:state.advertisement.source_contract_digest.clone(),required_features:Vec::new(),required_capabilities:Vec::new()
                    }).map_err(|error| error.to_string())?);
                    initialize.client_capabilities.meta=Some(meta);connection.send_request(initialize).block_task().await.map_err(|error| error.to_string())?;
                    let agent=connection.send_request(AgentCreateRequest {config:AgentConfigWire::HostDefault,idempotency_id:None}).block_task().await.map_err(|error| error.to_string())?.agent;
                    let session=connection.send_request(SessionCreateRequest {agent,working_dir:None,session_id:None,idempotency_id:None}).block_task().await.map_err(|error| error.to_string())?.session;
                    for mode in ["ordinary","observation"] {
                        let extension=connection.send_request(ExtensionRegisterRequest {kind:ExtensionKind::AgentComponent,implementation_id:format!("workflow-projection-{mode}"),descriptor:ExtensionDescriptor::AgentComponent {descriptor_version:1,component:AgentComponentKindWire::Workflow,name:format!("workflow-projection-{mode}"),capabilities:Default::default()},timeout:None}).block_task().await.map_err(|error| error.to_string())?.extension;
                        let params=serde_json::to_value(FeatureOperationRequest {operation:"workflow.extension.run".to_string(),signature_digest:echo_sdk_protocol::facade::family_operation_signature_digest("workflow","workflow.extension.run"),handle:Some(session.clone()),arguments:vec![WireValue::Handle(extension),WireValue::String(mode.to_string())]}).map_err(|error| error.to_string())?;
                        let result=connection.send_request(UntypedMessage::new("_echo_agent/workflow/op",params).map_err(|error| error.to_string())?).block_task().await;
                        let rpc_error=match result {Ok(_)=>return Err("Workflow projection manufactured success".to_string()),Err(error)=>error};
                        let observed=EchoSdkError::from_jsonrpc_data(rpc_error.data.as_ref())?;
                        let expected=if mode=="observation" {
                            if !entered.is_cancelled()||release.is_cancelled()||drops.load(Ordering::Acquire)!=0 {return Err("observation did not originate in still-held callback work".to_string());}
                            let mut expected=known_error.lock().map_err(|error| error.to_string())?.clone().ok_or("original callback error missing")?;
                            expected.operation=Some("_echo_agent/workflow/op".to_string());expected
                        } else {
                            wire::sdk_error(ExtensionErrorCode::FrameworkError,"extension bridge failure extension_failed: ordinary workflow failure 中文🙂",Retryability::Never,"_echo_agent/workflow/op")
                        };
                        if observed!=expected {return Err(format!("Workflow direct-run projection lost original typed facts for {mode}: observed={observed:?}; expected={expected:?}"));}
                        let expected_in_flight=usize::from(mode=="observation");
                        if state.services().map_err(|error| error.to_string())?.extensions().in_flight()!=expected_in_flight||drops.load(Ordering::Acquire)!=0 {return Err("direct run changed the ordinary settlement or original unknown admission/physical work".to_string());}
                    }
                    if calls.load(Ordering::Acquire)!=2 {return Err("projection counter did not execute exactly its two real callbacks".to_string());}
                    Ok(())
                }.await;
                observed_tx.send(observed).map_err(|_| agent_client_protocol::Error::internal_error().data("Workflow projection result observer disappeared"))?;
                finish.cancelled().await;
                Ok(())
            }
        });
        tokio::pin!(host, client, observed_rx);
        let observed = tokio::select! {
            biased;
            result=&mut observed_rx=>result.unwrap_or_else(|error| Err(error.to_string())),
            result=&mut host=>Err(format!("Workflow projection Host transport ended before evidence: {result:?}")),
            result=&mut client=>Err(format!("Workflow projection SDK transport ended before evidence: {result:?}")),
            ()=tokio::time::sleep(std::time::Duration::from_secs(8))=>Err("Workflow projection actual RPC did not finish".to_string()),
        };
        release.cancel();
        let task = original.lock().map_err(|error| error.to_string())?.take();
        if let Some(task) = task {
            tokio::time::timeout(std::time::Duration::from_secs(2), task)
                .await
                .map_err(|error| error.to_string())?
                .map_err(|error| error.to_string())?;
            if drops.load(Ordering::Acquire) != 1 {
                return Err("original callback work did not join exactly once".to_string());
            }
        }
        finish.cancel();
        // Dropping these test-owned transports is teardown, not a physical
        // receipt or a claim that permanent foreign uncertainty was settled.
        drop(close_owner);
        observed
    }

    #[test]
    fn family_handle_argument_retains_opaque_resource_address() -> Result<(), String> {
        let handle = echo_sdk_protocol::handle::WireHandle {
            id: "resource-中文-🧭".to_string(),
            generation: echo_sdk_protocol::scalar::WireU64::from_u64(u64::MAX),
            kind: echo_sdk_protocol::handle::HandleKind::FacadeResource,
        };
        let value = echo_sdk_protocol::scalar::WireValue::Handle(handle.clone());
        let arguments = [family_argument_json(&value).map_err(|error| error.to_string())?];
        let restored = resource_handle_at(&arguments, 0, "a resource", "_echo_agent/lsp/op")
            .map_err(|error| error.message)?;
        assert_eq!(restored, handle);
        Ok(())
    }

    #[test]
    fn only_compiled_methods_are_owned() {
        // The dispatcher owns the generic invoke surface, structured-output
        // validation and every compiled family handler (todo 4 stateful
        // families; eval/improve follow their Cargo features).
        assert!(owns_method("_echo_agent/facade/invoke"));
        assert!(owns_method("_echo_agent/memory/op"));
        assert!(owns_method("_echo_agent/workflow/op"));
        assert!(owns_method("_echo_agent/state/op"));
        assert!(owns_method("_echo_agent/delivery/op"));
        assert!(owns_method("_echo_agent/trace/op"));
        assert_eq!(
            owns_method("_echo_agent/eval/op"),
            cfg!(feature = "framework-eval")
        );
        assert_eq!(
            owns_method("_echo_agent/improve/op"),
            cfg!(feature = "framework-improve")
        );
        // Family ownership follows the compiled adapter authority. Channels
        // additionally require the reverse bridge because their plugins and
        // handlers are consumer implementations; testing has no facade
        // adapter and remains method-not-found.
        assert_eq!(
            owns_method("_echo_agent/channels/op"),
            cfg!(all(
                feature = "framework-channels",
                feature = "sdk-extension-bridge"
            ))
        );
        assert_eq!(
            owns_method("_echo_agent/telemetry/op"),
            cfg!(feature = "framework-telemetry")
        );
        assert!(!owns_method("_echo_agent/testing/op"));
        assert!(!owns_method("_echo_agent/task/create"));
        assert!(!owns_method("session/prompt"));
        assert!(!owns_method("_echo_agent/agent/create"));
    }

    #[cfg(feature = "framework-rag")]
    #[tokio::test]
    async fn rag_tools_are_removed_with_session_and_connection_close() {
        let runtime = SessionFacadeRuntime::new(&SdkProfileLimits::default());
        runtime
            .rag_tools
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(
                "session-a".to_string(),
                Arc::new(tools::RagToolSet::for_test()),
            );
        runtime
            .rag_tools
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(
                "session-b".to_string(),
                Arc::new(tools::RagToolSet::for_test()),
            );

        assert!(
            runtime
                .drop_session_resources_of(&HandleRegistry::new(1, 8), "session-a", &[])
                .await
                .is_ok()
        );
        {
            let rag_tools = runtime
                .rag_tools
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            assert!(!rag_tools.contains_key("session-a"));
            assert!(rag_tools.contains_key("session-b"));
        }

        assert!(runtime.close_all(&HandleRegistry::new(1, 8)).await.is_ok());
        assert!(
            runtime
                .rag_tools
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .is_empty()
        );
    }

    #[tokio::test]
    async fn sandbox_producer_eof_and_cleanup_drain_share_a_fused_actual_source()
    -> Result<(), String> {
        use futures::StreamExt as _;
        let source = futures::stream::unfold(false, |emitted| async move {
            if emitted {
                None
            } else {
                Some(("actual-output", true))
            }
        });
        let mut source = Box::pin(source.fuse());
        let mut observed = Vec::new();
        while let Some(value) = source.next().await {
            observed.push(value);
        }
        // This is the production normal-EOF then cleanup-drain order. Plain
        // Unfold panics on this additional poll after its completed None.
        while source.next().await.is_some() {}
        assert_eq!(observed, vec!["actual-output"]);
        Ok(())
    }
    #[cfg(feature = "sdk-extension-bridge")]
    #[tokio::test]
    async fn sandbox_natural_eof_preserves_queued_complete_without_cleanup_cancellation()
    -> Result<(), String> {
        let result = echo_agent::sandbox::ExecutionResult {
            exit_code: 0,
            stdout: "completed output".to_string(),
            stderr: String::new(),
            duration: std::time::Duration::ZERO,
            sandbox_type: "original".to_string(),
            timed_out: false,
            cancelled: false,
            output_truncated: false,
            stdout_bytes: 16,
            stderr_bytes: 0,
        };
        let source = futures::stream::unfold(Some(result), |pending| async move {
            pending.map(|value| {
                (
                    echo_agent::sandbox::SandboxStreamEvent::Complete(value),
                    None,
                )
            })
        });
        let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
        let cleanup = tokio_util::sync::CancellationToken::new();
        // Wait for actual natural EOF while its Complete payload remains
        // queued. This deterministically exposes the erroneous wake signal
        // before the facade consumer selects recv versus cleanup cancellation.
        drive_sandbox_facade_source(Box::pin(source), sender, cleanup.clone()).await;
        assert!(
            !cleanup.is_cancelled(),
            "natural EOF must not cancel cleanup while Complete is still queued"
        );
        let value = receiver
            .recv()
            .await
            .ok_or("Complete was lost")?
            .map_err(|error| error.message)?;
        let json = value.into_json().map_err(|error| error.to_string())?;
        assert_eq!(json.get("event_type"), Some(&serde_json::json!("complete")));
        assert!(receiver.recv().await.is_none());
        Ok(())
    }
}
