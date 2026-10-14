//! Typed `_echo_agent/*` core handlers.
//!
//! Every handler follows the same admission ladder:
//!
//! 1. Extended-mode gate — a connection that stayed Standard answers every
//!    `_echo_agent/*` request with the official method-not-found, so a plain
//!    Client can never half-enter the extension surface;
//! 2. capability gate — negotiated but unadvertised families answer with a
//!    typed capability mismatch;
//! 3. handle resolution — shape → kind → generation → issued/closed;
//! 4. framework work through the shared connection services and the
//!    existing framework authorities only.
//!
//! Long operations (`run/start`, `run/wait`) are spawned on the official
//! connection task instead of blocking the dispatch loop.

use agent_client_protocol::{Client, ConnectionTo, Responder};
use echo_agent::acp::{AcpSession, AcpSessionContext, RunStartSpec};
use echo_agent::agent::EventIdentity;
use echo_agent::llm::types::Message;
use echo_agent::runtime::{TurnMode, TurnRequest};
use echo_sdk_protocol::capability::ExtensionCapability;
use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::event::{ReplayRequest, ReplayResponse};
use echo_sdk_protocol::handle::{HandleKind, WireHandle};
use echo_sdk_protocol::methods::{
    AgentCloseRequest, AgentCloseResponse, AgentConfigWire, AgentCreateRequest,
    AgentCreateResponse, AgentDescribeRequest, AgentDescribeResponse, AgentSnapshotWire,
    EventAckNotification, RecoveredRunWire, RunCancelRequest, RunCancelResponse, RunGetRequest,
    RunGetResponse, RunInput, RunStartRequest, RunStartResponse, RunStatus, RunWaitRequest,
    RunWaitResponse, SessionCloseRequest, SessionCloseResponse, SessionCreateRequest,
    SessionCreateResponse, SessionLoadRequest, SessionLoadResponse,
};
use echo_sdk_protocol::scalar::WireU64;
use std::sync::Arc;
use std::time::Duration;

use super::events::StreamDelivery;
use super::handles::RunRecord;
use super::persistence::recovered_receipt_wire;
use super::state::CoreProfileState;
use super::wire;
use crate::factory::PreparedAgentDefinition;

/// Admission step 1: un-negotiated connections get the official
/// method-not-found, never a typed extension error.
pub(crate) async fn require_extended(
    state: &CoreProfileState,
    method: &str,
) -> std::result::Result<Arc<echo_agent::acp::AcpConnectionServices>, agent_client_protocol::Error>
{
    let services = state
        .services()
        .map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?;
    if !services.is_extended().await {
        return Err(agent_client_protocol::Error::method_not_found().data(method.to_string()));
    }
    if services.ensure_admission().is_err() {
        return Err(wire::into_jsonrpc_error(wire::sdk_error(
            ExtensionErrorCode::HostShuttingDown,
            "ACP Host is shutting down",
            Retryability::Never,
            method,
        )));
    }
    Ok(services)
}

/// Admission step 2: capability gate with typed mismatch errors.
pub(crate) fn require_capability(
    state: &CoreProfileState,
    capability: ExtensionCapability,
    method: &str,
) -> std::result::Result<(), agent_client_protocol::Error> {
    if state.advertisement.declares(capability) {
        Ok(())
    } else {
        Err(wire::into_jsonrpc_error(wire::sdk_error(
            ExtensionErrorCode::ExtensionCapabilityMismatch,
            format!("capability {} is not advertised", capability.as_str()),
            Retryability::Never,
            method,
        )))
    }
}

fn respond_error(
    responder: impl FnOnce(agent_client_protocol::Error) -> Result<(), agent_client_protocol::Error>,
    error: EchoSdkError,
) -> Result<(), agent_client_protocol::Error> {
    responder(wire::into_jsonrpc_error(error))
}

// ── Agent lifecycle ─────────────────────────────────────────────────────────

pub(crate) async fn agent_create(
    state: Arc<CoreProfileState>,
    request: AgentCreateRequest,
    responder: Responder<AgentCreateResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/agent/create";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::AgentLifecycle, method)?;
    if let Err(reason) = request.validate() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidConfig,
                reason,
                Retryability::Never,
                method,
            ),
        );
    }
    let fingerprint = super::handles::HandleRegistry::config_fingerprint(&request.config);
    let definition_state = state.clone();
    let outcome = match &request.idempotency_id {
        Some(idempotency_id) => {
            state
                .handles
                .agent_create_idempotent(idempotency_id, fingerprint.clone(), || {
                    build_definition(&definition_state, request.config.clone())
                })
        }
        None => build_definition(&state, request.config.clone()).and_then(
            |(definition, fingerprint)| {
                state
                    .handles
                    .register_agent(definition, fingerprint)
                    .map(|(agent, record)| (agent, record, true))
            },
        ),
    };
    match outcome {
        Ok((agent, record, _fresh)) => {
            if let Err(error) = state.register_definition(&record.definition) {
                let _ = state.handles.close_agent(&agent);
                return respond_error(
                    |error| responder.respond_with_error(error),
                    wire::sdk_error(
                        ExtensionErrorCode::PayloadTooLarge,
                        error,
                        Retryability::AfterDelay,
                        "_echo_agent/agent/create",
                    ),
                );
            }
            responder.respond(AgentCreateResponse { agent })
        }
        Err(error) => respond_error(|error| responder.respond_with_error(error), error),
    }
}

/// Build an immutable Agent definition from the typed config. The
/// `host_default` branch never transmits credentials; the explicit branch
/// resolves environment credentials at create time and fails closed on
/// unsupported configurations (enforced by the wire DTO grammar).
fn build_definition(
    state: &CoreProfileState,
    config: AgentConfigWire,
) -> std::result::Result<(Arc<PreparedAgentDefinition>, String), EchoSdkError> {
    let fingerprint = super::handles::HandleRegistry::config_fingerprint(&config);
    let definition = match config {
        AgentConfigWire::HostDefault => state.default_definition.clone(),
        AgentConfigWire::Explicit(explicit) => {
            let (framework, client) = wire::framework_from_wire(&explicit).map_err(|error| {
                wire::sdk_error(
                    ExtensionErrorCode::InvalidConfig,
                    error,
                    Retryability::Never,
                    "_echo_agent/agent/create",
                )
            })?;
            Arc::new(PreparedAgentDefinition::new(
                framework,
                client,
                Some(state.state_store.clone()),
                false,
            ))
        }
    };
    Ok((definition, fingerprint))
}

pub(crate) async fn agent_describe(
    state: Arc<CoreProfileState>,
    request: AgentDescribeRequest,
    responder: Responder<AgentDescribeResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/agent/describe";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::AgentLifecycle, method)?;
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.agent, HandleKind::Agent, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    match state.handles.agent(&request.agent) {
        Ok(record) => {
            let snapshot: AgentSnapshotWire = record.definition.snapshot();
            responder.respond(AgentDescribeResponse { snapshot })
        }
        Err(error) => respond_error(|error| responder.respond_with_error(error), error),
    }
}

pub(crate) async fn agent_close(
    state: Arc<CoreProfileState>,
    request: AgentCloseRequest,
    responder: Responder<AgentCloseResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/agent/close";
    let services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::AgentLifecycle, method)?;
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.agent, HandleKind::Agent, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let agent_record = match state.handles.agent(&request.agent) {
        Ok(record) => record,
        Err(error) if error.code == ExtensionErrorCode::ClosedHandle => {
            return responder.respond(AgentCloseResponse { released: false });
        }
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let _lifecycle = agent_record.lifecycle.lock().await;
    if state.handles.is_closed(&request.agent) {
        return responder.respond(AgentCloseResponse { released: false });
    }
    agent_record
        .closing
        .store(true, std::sync::atomic::Ordering::Release);
    let sessions = state.handles.sessions_for_agent(&request.agent.id);
    let mut close_error = None;
    for (session_handle, _) in &sessions {
        if let Err(error) = close_session_owned(&state, &services, session_handle, method).await {
            close_error = Some(error);
        }
    }
    let definition_id = agent_record.definition.id().to_string();
    if let Some(error) = close_error {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let closed = match state.handles.close_agent(&request.agent) {
        Ok(released) => released,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    state.session_factory.remove_definition(&definition_id);
    responder.respond(AgentCloseResponse { released: closed })
}

// ── Session handles ─────────────────────────────────────────────────────────

pub(crate) async fn session_create(
    state: Arc<CoreProfileState>,
    request: SessionCreateRequest,
    responder: Responder<SessionCreateResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/session/create";
    let services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::SessionHandles, method)?;
    if let Err(reason) = request.validate() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                reason,
                Retryability::Never,
                method,
            ),
        );
    }
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.agent, HandleKind::Agent, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let record = match state.handles.agent(&request.agent) {
        Ok(record) => record,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let _lifecycle = record.lifecycle.lock().await;
    if state.handles.is_closed(&request.agent)
        || record.closing.load(std::sync::atomic::Ordering::Acquire)
    {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::ClosedHandle,
                "Agent is closing",
                Retryability::Never,
                method,
            ),
        );
    }
    let working_dir = match wire::working_dir_from_wire(request.working_dir.as_ref()) {
        Ok(path) => path,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let session_id = match &request.session_id {
        Some(explicit) => explicit.clone(),
        None => format!("sess_{}", uuid::Uuid::new_v4()),
    };
    match create_session_on(
        &state,
        &services,
        record.definition.as_ref(),
        session_id,
        working_dir,
        method,
    )
    .await
    {
        Ok(acp_session_id) => {
            let cwd = services
                .sessions()
                .get(&agent_client_protocol::schema::v1::SessionId::new(
                    acp_session_id.clone(),
                ))
                .await
                .map(|session| session.context.cwd.clone());
            match state.handles.register_session_with_cwd(
                acp_session_id.clone(),
                request.agent.id.clone(),
                cwd,
            ) {
                Ok((session, task_run, session_record)) => {
                    let cwd_wire = match services
                        .sessions()
                        .get(&agent_client_protocol::schema::v1::SessionId::new(
                            acp_session_id.clone(),
                        ))
                        .await
                    {
                        Some(acp_session) => match wire::path_to_wire(&acp_session.context.cwd) {
                            Ok(path) => path,
                            Err(error) => {
                                let _ = state.handles.close_session(&session);
                                let _ = services
                                    .sessions()
                                    .close_session(
                                        &agent_client_protocol::schema::v1::SessionId::new(
                                            acp_session_id.clone(),
                                        ),
                                    )
                                    .await;
                                return respond_error(
                                    |error| responder.respond_with_error(error),
                                    wire::sdk_error(
                                        ExtensionErrorCode::InvalidValue,
                                        error,
                                        Retryability::Never,
                                        method,
                                    ),
                                );
                            }
                        },
                        None => {
                            let _ = state.handles.close_session(&session);
                            let _ = services
                                .sessions()
                                .close_session(&agent_client_protocol::schema::v1::SessionId::new(
                                    acp_session_id.clone(),
                                ))
                                .await;
                            return respond_error(
                                |error| responder.respond_with_error(error),
                                wire::sdk_error(
                                    ExtensionErrorCode::FrameworkError,
                                    "Session disappeared before persistence",
                                    Retryability::Never,
                                    method,
                                ),
                            );
                        }
                    };
                    if let Err(error) = state.persistence.record_session(
                        &acp_session_id,
                        &session_record.agent_handle_id,
                        &record.config_fingerprint,
                        cwd_wire,
                    ) {
                        let _ = state.handles.close_session(&session);
                        let _ = services
                            .sessions()
                            .close_session(&agent_client_protocol::schema::v1::SessionId::new(
                                acp_session_id.clone(),
                            ))
                            .await;
                        return respond_error(
                            |error| responder.respond_with_error(error),
                            wire::framework_error(&error, method),
                        );
                    }
                    responder.respond(SessionCreateResponse {
                        session,
                        task_run,
                        acp_session_id,
                    })
                }
                Err(error) => {
                    let _ = services
                        .sessions()
                        .close_session(&agent_client_protocol::schema::v1::SessionId::new(
                            acp_session_id,
                        ))
                        .await;
                    respond_error(|error| responder.respond_with_error(error), error)
                }
            }
        }
        Err(error) => respond_error(|error| responder.respond_with_error(error), error),
    }
}

/// Create the ACP Session (independent Agent included) through the shared
/// registry — the exact path standard `session/new` uses.
async fn create_session_on(
    state: &CoreProfileState,
    services: &Arc<echo_agent::acp::AcpConnectionServices>,
    definition: &PreparedAgentDefinition,
    session_id: String,
    working_dir: Option<std::path::PathBuf>,
    method: &'static str,
) -> std::result::Result<String, EchoSdkError> {
    let cwd = match working_dir {
        Some(path) if path.is_absolute() => path,
        Some(path) => {
            return Err(wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                format!("working_dir must be absolute: {}", path.display()),
                Retryability::Never,
                method,
            ));
        }
        None => std::env::current_dir().map_err(|error| {
            wire::sdk_error(
                ExtensionErrorCode::InvalidConfig,
                format!("no working directory available: {error}"),
                Retryability::Never,
                method,
            )
        })?,
    };
    let capabilities = services
        .sessions()
        .capabilities()
        .await
        .map_err(|error| wire::framework_error(&error, method))?;
    // The definition id rides the Session context meta so the connection's
    // single session factory can build this Session from the exact Agent
    // definition the handle references (no second factory, no second map).
    let mut meta = agent_client_protocol::schema::v1::Meta::new();
    meta.insert(
        state.definition_meta_key().to_string(),
        serde_json::Value::String(definition.id().to_string()),
    );
    let context = AcpSessionContext {
        session_id: agent_client_protocol::schema::v1::SessionId::new(session_id.clone()),
        cwd,
        additional_directories: Vec::new(),
        mcp_servers: Vec::new(),
        client_capabilities: capabilities,
        meta: Some(meta),
    };
    let inserted = services
        .insert_session(context)
        .await
        .map_err(|error| wire::framework_error(&error, method))?;
    let reserve = (|| {
        let recovered = state
            .persistence
            .load_recovered_runs()
            .map_err(|error| wire::framework_error(&error, method))?;
        Ok::<_, EchoSdkError>(
            recovered
                .into_iter()
                .filter(|run| run.record.session_id == session_id)
                .map(|run| run.record.run_id)
                .collect::<Vec<_>>(),
        )
    })();
    let reserve = match reserve {
        Ok(run_ids) => match services.sessions().get(&inserted).await {
            Some(session) => session
                .reserve_recovered_turn_ids(&run_ids)
                .map_err(|error| wire::framework_error(&error, method)),
            None => Err(wire::sdk_error(
                ExtensionErrorCode::FrameworkError,
                "Session disappeared before recovered turn identities were reserved",
                Retryability::Never,
                method,
            )),
        },
        Err(error) => Err(error),
    };
    if let Err(error) = reserve {
        // Framework owns both the opaque turn namespace and rollback. Keep
        // the unpublished Session in its registry if cleanup fails so the
        // retained connection close owner can settle it later.
        match services.sessions().close_session(&inserted).await {
            Ok(_) => {
                state.session_factory.remove_session_services(&session_id);
            }
            Err(cleanup) => {
                return Err(wire::sdk_error(
                    ExtensionErrorCode::FrameworkError,
                    format!(
                        "turn identity reservation failed: {}; Session cleanup pending: {cleanup}",
                        error.message
                    ),
                    Retryability::AfterDelay,
                    method,
                ));
            }
        }
        return Err(error);
    }
    Ok(inserted.to_string())
}

pub(crate) async fn session_load(
    state: Arc<CoreProfileState>,
    request: SessionLoadRequest,
    responder: Responder<SessionLoadResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/session/load";
    let services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::SessionHandles, method)?;
    if let Err(reason) = request.validate() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                reason,
                Retryability::Never,
                method,
            ),
        );
    }
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.agent, HandleKind::Agent, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let record = match state.handles.agent(&request.agent) {
        Ok(record) => record,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let _lifecycle = record.lifecycle.lock().await;
    if state.handles.is_closed(&request.agent)
        || record.closing.load(std::sync::atomic::Ordering::Acquire)
    {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::ClosedHandle,
                "Agent is closing",
                Retryability::Never,
                method,
            ),
        );
    }
    let requested_working_dir = match wire::working_dir_from_wire(request.working_dir.as_ref()) {
        Ok(path) => path,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let persisted_session = match state.persistence.load_session(&request.session_id) {
        Ok(session) => session,
        Err(error) => {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::framework_error(&error, method),
            );
        }
    };
    // Only sessions present in the durable session/run indexes can be loaded;
    // unknown identities fail closed instead of silently starting an empty Session.
    let recovered = state.recovered_runs_of_session(&request.session_id);
    if recovered.is_empty() && persisted_session.is_none() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                format!(
                    "session {} is not present in the configured state root",
                    request.session_id
                ),
                Retryability::Never,
                method,
            ),
        );
    }
    if persisted_session.as_ref().is_some_and(|session| {
        !session.agent_config_fingerprint.is_empty()
            && session.agent_config_fingerprint != record.config_fingerprint
    }) || recovered.iter().any(|run| {
        !run.record.agent_config_fingerprint.is_empty()
            && run.record.agent_config_fingerprint != record.config_fingerprint
    }) {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidConfig,
                "session recovery Agent definition does not match the requested Agent handle",
                Retryability::Never,
                method,
            ),
        );
    }
    let persisted_cwd = persisted_session
        .as_ref()
        .map(|session| session.cwd.clone())
        .or_else(|| recovered.first().and_then(|run| run.record.cwd.clone()))
        .and_then(|cwd| wire::path_from_wire(&cwd).ok());
    if let (Some(requested), Some(persisted)) = (&requested_working_dir, &persisted_cwd)
        && requested != persisted
    {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidConfig,
                "session recovery working_dir does not match the persisted Session directory",
                Retryability::Never,
                method,
            ),
        );
    }
    let effective_working_dir = requested_working_dir.or(persisted_cwd);
    if effective_working_dir.is_none() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidConfig,
                "recovered Session has no persisted working_dir",
                Retryability::Never,
                method,
            ),
        );
    }
    let acp_session_id = match create_session_on(
        &state,
        &services,
        record.definition.as_ref(),
        request.session_id.clone(),
        effective_working_dir,
        method,
    )
    .await
    {
        Ok(acp_session_id) => acp_session_id,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let mut runs = Vec::with_capacity(recovered.len());
    let mut registered_run_ids = Vec::with_capacity(recovered.len());
    let mut recovered_sequence = 0u64;
    for run in &recovered {
        recovered_sequence = recovered_sequence.max(run.last_sequence);
        match state.handles.register_recovered_run(
            run.record.run_id.clone(),
            run.record.stream_id.clone(),
            run.record.session_id.clone(),
            run.status,
            run.last_sequence,
            run.terminal.clone(),
            recovered_receipt_wire(run),
        ) {
            Ok((run_handle, stream_handle, _record)) => {
                registered_run_ids.push(run.record.run_id.clone());
                runs.push(RecoveredRunWire {
                    run: run_handle,
                    stream: stream_handle,
                    status: run.status,
                    last_sequence: WireU64::from_u64(run.last_sequence),
                    terminal: run.terminal.clone(),
                });
            }
            Err(error) => {
                for run_id in registered_run_ids {
                    state.handles.remove_run(&run_id);
                }
                let _ = services
                    .sessions()
                    .close_session(&agent_client_protocol::schema::v1::SessionId::new(
                        acp_session_id.clone(),
                    ))
                    .await;
                return respond_error(|error| responder.respond_with_error(error), error);
            }
        }
    }
    match state.handles.register_session_with_cwd(
        acp_session_id.clone(),
        request.agent.id.clone(),
        services
            .sessions()
            .get(&agent_client_protocol::schema::v1::SessionId::new(
                acp_session_id.clone(),
            ))
            .await
            .map(|session| session.context.cwd.clone()),
    ) {
        Ok((session, task_run, session_record)) => {
            let cwd_wire = services
                .sessions()
                .get(&agent_client_protocol::schema::v1::SessionId::new(
                    acp_session_id.clone(),
                ))
                .await
                .ok_or_else(|| {
                    agent_client_protocol::Error::internal_error().data("Session disappeared")
                })
                .and_then(|acp_session| {
                    wire::path_to_wire(&acp_session.context.cwd)
                        .map_err(|error| agent_client_protocol::Error::internal_error().data(error))
                })?;
            state
                .persistence
                .record_session(
                    &acp_session_id,
                    &session_record.agent_handle_id,
                    &record.config_fingerprint,
                    cwd_wire,
                )
                .map_err(|error| {
                    agent_client_protocol::Error::internal_error().data(error.to_string())
                })?;
            responder.respond(SessionLoadResponse {
                session,
                task_run,
                acp_session_id,
                recovered_sequence: (recovered_sequence > 0)
                    .then(|| WireU64::from_u64(recovered_sequence)),
                runs,
            })
        }
        Err(error) => {
            for run_id in registered_run_ids {
                state.handles.remove_run(&run_id);
            }
            let _ = services
                .sessions()
                .close_session(&agent_client_protocol::schema::v1::SessionId::new(
                    acp_session_id,
                ))
                .await;
            respond_error(|error| responder.respond_with_error(error), error)
        }
    }
}

pub(crate) async fn session_close(
    state: Arc<CoreProfileState>,
    request: SessionCloseRequest,
    responder: Responder<SessionCloseResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/session/close";
    let services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::SessionHandles, method)?;
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.session, HandleKind::Session, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    match close_session_owned(&state, &services, &request.session, method).await {
        Ok(released) => responder.respond(SessionCloseResponse { released }),
        Err(error) => respond_error(|error| responder.respond_with_error(error), error),
    }
}

/// Agent cascade and explicit Session close use the same settlement path.
/// Keep the issued owner until all fallible cleanup succeeds, so cancellation
/// and timeout leave a retryable close address instead of orphaned debt.
async fn close_session_owned(
    state: &CoreProfileState,
    services: &echo_agent::acp::AcpConnectionServices,
    handle: &echo_sdk_protocol::handle::WireHandle,
    method: &str,
) -> Result<bool, EchoSdkError> {
    let record = match state.handles.session(handle) {
        Ok(record) => record,
        Err(error) if error.code == ExtensionErrorCode::ClosedHandle => return Ok(false),
        Err(error) => return Err(error),
    };
    record
        .closing
        .store(true, std::sync::atomic::Ordering::Release);
    record.cancel_operations.cancel();
    let _lifecycle = record.lifecycle.write().await;
    if state.handles.is_closed(handle) {
        return Ok(false);
    }
    let owner = &record.acp_session_id;
    #[cfg(feature = "sdk-facade-adapters")]
    {
        let ids = state.handles.facade_resource_ids_of(owner);
        state
            .facade
            .drop_session_resources_of(&state.handles, owner, &ids)
            .await
            .map_err(|error| {
                wire::sdk_error(
                    ExtensionErrorCode::FrameworkError,
                    error,
                    Retryability::AfterDelay,
                    method,
                )
            })?;
        state.handles.close_facade_resources_of(owner);
    }
    services
        .sessions()
        .close_session(&agent_client_protocol::schema::v1::SessionId::new(
            owner.clone(),
        ))
        .await
        .map_err(|error| wire::framework_error(&error, method))?;
    for run_id in state.handles.runs_for_session(owner) {
        let run = wire::handle(run_id.clone(), HandleKind::Run, state.handles.generation());
        if let Ok(record) = state.handles.run(&run)
            && let RunRecord::Live { entry } = record.as_ref()
        {
            let receipt = entry.wait_receipt().await;
            state
                .persist_settled_receipt(&run_id, &receipt, entry.ledger.last_sequence())
                .map_err(|error| {
                    wire::sdk_error(
                        ExtensionErrorCode::FrameworkError,
                        error,
                        Retryability::AfterDelay,
                        method,
                    )
                })?;
        }
        services.remove_run(&run_id).await;
        state.handles.remove_run(&run_id);
        state.remove_delivery_for_run(&run_id);
    }
    state.session_factory.remove_session_services(owner);
    state
        .handles
        .close_session(handle)
        .map(|(released, _)| released)
}

// ── Runs ────────────────────────────────────────────────────────────────────

pub(crate) async fn run_start(
    state: Arc<CoreProfileState>,
    request: RunStartRequest,
    responder: Responder<RunStartResponse>,
    connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/run/start";
    let services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::Runs, method)?;
    if let Err(reason) = request.validate() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                reason,
                Retryability::Never,
                method,
            ),
        );
    }
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.session, HandleKind::Session, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let (session, acp_session_id) = match resolve_session(&state, &request.session).await {
        Ok(resolved) => resolved,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let session_record = match state.handles.session(&request.session) {
        Ok(record) => record,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let _publication = session_record.lifecycle.read().await;
    if session_record
        .closing
        .load(std::sync::atomic::Ordering::Acquire)
    {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::handle_error(
                ExtensionErrorCode::ClosedHandle,
                "Session is closing",
                method,
                &request.session,
            ),
        );
    }
    let agent_handle_id = session_record.agent_handle_id.clone();
    let agent_fingerprint = match state.handles.agent_fingerprint(&wire::handle(
        agent_handle_id.clone(),
        HandleKind::Agent,
        state.handles.generation(),
    )) {
        Ok(fingerprint) => fingerprint,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let active = match session.begin_turn() {
        Ok(active) => active,
        Err(error) => {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::sdk_error(
                    ExtensionErrorCode::InvalidRequest,
                    error.to_string(),
                    Retryability::AfterDelay,
                    method,
                ),
            );
        }
    };
    let run_id = active.turn.id().to_string();
    let durable = match state.reserve_durable_settlement(&run_id) {
        Ok(reservation) => reservation,
        Err(error) => {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::sdk_error(
                    ExtensionErrorCode::PayloadTooLarge,
                    error,
                    Retryability::AfterDelay,
                    method,
                ),
            );
        }
    };
    let identity = match EventIdentity::for_chat(
        Some(acp_session_id.clone()),
        run_id.clone(),
        format!("{run_id}:message"),
        Some(run_id.clone()),
    ) {
        Ok(identity) => identity,
        Err(error) => {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::framework_error(&error, method),
            );
        }
    };
    // The stream handle id is exactly the envelope stream identity, so live
    // notifications, replay cursors and journal records all address one id.
    let stream_id = identity.stream_id.as_str().to_string();
    let turn = match &request.input {
        RunInput::Chat { text } => TurnRequest::new(identity, text.clone()),
        RunInput::ChatMessage { message } => {
            let message = wire::message_from_wire(message.clone()).map_err(|error| {
                wire::sdk_error(
                    ExtensionErrorCode::InvalidValue,
                    error,
                    Retryability::Never,
                    method,
                )
            });
            let message = match message {
                Ok(message) => message,
                Err(error) => {
                    return respond_error(|error| responder.respond_with_error(error), error);
                }
            };
            TurnRequest::from_message(identity, message)
        }
        RunInput::ExecuteMessage { message } => {
            let message = wire::message_from_wire(message.clone()).map_err(|error| {
                wire::sdk_error(
                    ExtensionErrorCode::InvalidValue,
                    error,
                    Retryability::Never,
                    method,
                )
            });
            let message = match message {
                Ok(message) => message,
                Err(error) => {
                    return respond_error(|error| responder.respond_with_error(error), error);
                }
            };
            TurnRequest::from_message(identity, message).mode(TurnMode::Execute)
        }
        RunInput::Execute { task } => {
            TurnRequest::new(identity, task.clone()).mode(TurnMode::Execute)
        }
    }
    .cancel(active.turn.cancellation());
    let journal: Option<
        std::sync::Arc<
            dyn echo_agent::state::journal::EventJournal<echo_agent::agent::EventEnvelope>,
        >,
    > = match state.journal(&run_id) {
        Ok(journal) => {
            let coerced: std::sync::Arc<
                dyn echo_agent::state::journal::EventJournal<echo_agent::agent::EventEnvelope>,
            > = journal;
            Some(coerced)
        }
        Err(error) => {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::framework_error(&error, method),
            );
        }
    };
    let stream_handle = wire::handle(
        stream_id.clone(),
        HandleKind::Stream,
        state.handles.generation(),
    );
    let delivery = StreamDelivery::new(
        state.clone(),
        &run_id,
        connection.clone(),
        stream_handle.clone(),
        state.limits.max_outstanding_live_events,
        state
            .limits
            .max_event_bytes
            .saturating_mul(state.limits.max_outstanding_live_events),
        state.limits.max_event_bytes,
    );
    let cwd = match wire::path_to_wire(&session.context.cwd) {
        Ok(cwd) => cwd,
        Err(error) => {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::sdk_error(
                    ExtensionErrorCode::InvalidValue,
                    error,
                    Retryability::Never,
                    method,
                ),
            );
        }
    };
    if let Err(error) = state.persistence.record_run_started(
        &run_id,
        &stream_id,
        &acp_session_id,
        &agent_handle_id,
        &agent_fingerprint,
        Some(cwd),
        input_kind(&request.input),
    ) {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::framework_error(&error, method),
        );
    }
    let spec = RunStartSpec {
        session: session.clone(),
        active,
        run_id: run_id.clone(),
        stream_id: stream_id.clone(),
        turn,
        projector: Some(services.projector(
            agent_client_protocol::schema::v1::SessionId::new(acp_session_id.clone()),
            connection.clone(),
        )),
        journal,
        observers: vec![delivery.clone()],
    };
    let (entry, task) = match services.prepare_run(spec).await {
        Ok(pair) => pair,
        Err(error) => {
            state.remove_delivery_for_run(&run_id);
            state.remove_journal(&run_id);
            if let Err(cleanup) = state.persistence.remove_run(&run_id) {
                tracing::warn!(
                    "failed to remove run {} after prepare failure: {cleanup}",
                    run_id
                );
            }
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::framework_error(&error, method),
            );
        }
    };
    let run_handle = match state.handles.register_live_run(
        run_id.clone(),
        entry.clone(),
        Some(&request.session.id),
    ) {
        Ok((run, _record)) => run,
        Err(error) => {
            let _ = services.remove_run(&run_id).await;
            state.remove_delivery_for_run(&run_id);
            state.remove_journal(&run_id);
            if let Err(cleanup) = state.persistence.remove_run(&run_id) {
                tracing::warn!(
                    "failed to remove run {} after handle failure: {cleanup}",
                    run_id
                );
            }
            drop(task);
            return respond_error(|error| responder.respond_with_error(error), error);
        }
    };
    let stream_record = state
        .handles
        .register_live_stream(stream_id.clone(), run_id.clone());
    if let Err(error) = stream_record {
        let _ = services.remove_run(&run_id).await;
        state.handles.remove_run(&run_id);
        state.remove_delivery_for_run(&run_id);
        state.remove_journal(&run_id);
        if let Err(cleanup) = state.persistence.remove_run(&run_id) {
            tracing::warn!(
                "failed to remove run {} after stream registration failure: {cleanup}",
                run_id
            );
        }
        drop(task);
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    state.register_delivery(delivery);
    state.begin_settlement();
    if let Err(error) = connection.spawn(task) {
        state.finish_settlement(Err(format!("failed to spawn run driver: {error}")));
        let _ = services.remove_run(&run_id).await;
        state.handles.remove_run(&run_id);
        state.remove_delivery_for_run(&run_id);
        state.remove_journal(&run_id);
        if let Err(cleanup) = state.persistence.remove_run(&run_id) {
            tracing::warn!(
                "failed to remove run {} after spawn failure: {cleanup}",
                run_id
            );
        }
        return responder.respond_with_error(error);
    }
    durable.admit();
    let settle_state = state.clone();
    let settle_entry = entry.clone();
    tokio::spawn(async move {
        let receipt = settle_entry.wait_receipt().await;
        let result = settle_state.persist_settled_receipt(
            &settle_entry.run_id,
            &receipt,
            settle_entry.ledger.last_sequence(),
        );
        if let Err(error) = result {
            tracing::warn!("durable Run settlement pending: {error}");
        }
        // Pending durable projection is retained separately by Run identity;
        // count completion is not a claim that the write succeeded.
        settle_state.finish_settlement(Ok(()));
    });
    responder.respond(RunStartResponse {
        run: run_handle,
        stream: stream_handle,
        first_event: None,
    })
}

fn input_kind(input: &RunInput) -> &'static str {
    match input {
        RunInput::Chat { .. } | RunInput::ChatMessage { .. } => "chat",
        RunInput::ExecuteMessage { .. } | RunInput::Execute { .. } => "execute",
    }
}

fn persist_receipt_now(
    state: &CoreProfileState,
    entry: &echo_agent::acp::RunEntry,
    receipt: &echo_agent::runtime::TurnReceipt,
) -> std::result::Result<(), EchoSdkError> {
    state
        .persist_settled_receipt(&entry.run_id, receipt, entry.ledger.last_sequence())
        .map_err(|error| {
            wire::sdk_error(
                ExtensionErrorCode::FrameworkError,
                error,
                Retryability::AfterDelay,
                "_echo_agent/run/wait",
            )
        })
}

/// Resolve a session handle to the shared ACP Session.
async fn resolve_session(
    state: &CoreProfileState,
    handle: &WireHandle,
) -> std::result::Result<(Arc<AcpSession>, String), EchoSdkError> {
    let record = state.handles.session(handle)?;
    if record.closing.load(std::sync::atomic::Ordering::Acquire) {
        return Err(wire::handle_error(
            ExtensionErrorCode::ClosedHandle,
            "Session is closing",
            "_echo_agent/session",
            handle,
        ));
    }
    let services = state.services().map_err(|error| {
        wire::sdk_error(
            ExtensionErrorCode::HostShuttingDown,
            error.to_string(),
            Retryability::Never,
            "_echo_agent/session",
        )
    })?;
    let session_id =
        agent_client_protocol::schema::v1::SessionId::new(record.acp_session_id.clone());
    let session = services.sessions().get(&session_id).await.ok_or_else(|| {
        wire::handle_error(
            ExtensionErrorCode::ClosedHandle,
            "the underlying ACP Session is gone",
            "_echo_agent/run/start",
            handle,
        )
    })?;
    Ok((session, record.acp_session_id.clone()))
}

pub(crate) async fn run_get(
    state: Arc<CoreProfileState>,
    request: RunGetRequest,
    responder: Responder<RunGetResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/run/get";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::Runs, method)?;
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.run, HandleKind::Run, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let record = match state.handles.run(&request.run) {
        Ok(record) => record,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let generation = state.handles.generation();
    let response = match record.as_ref() {
        RunRecord::Live { entry } => {
            let receipt = entry.receipt();
            let (terminal, receipt_wire) = match receipt.as_deref() {
                Some(receipt) => match (wire::terminal_of(receipt), wire::receipt_wire(receipt)) {
                    (Ok(terminal), Ok(receipt_wire)) => (Some(terminal), Some(receipt_wire)),
                    (Err(error), _) | (_, Err(error)) => {
                        return respond_error(
                            |error| responder.respond_with_error(error),
                            wire::sdk_error(
                                ExtensionErrorCode::SerializationViolation,
                                error,
                                Retryability::Never,
                                method,
                            ),
                        );
                    }
                },
                None => (None, None),
            };
            RunGetResponse {
                status: wire::status_of(receipt.as_deref()),
                last_sequence: WireU64::from_u64(entry.ledger.last_sequence()),
                stream: Some(wire::handle(
                    entry.stream_id.clone(),
                    HandleKind::Stream,
                    generation,
                )),
                terminal,
                receipt: receipt_wire,
            }
        }
        RunRecord::Recovered(recovered) => RunGetResponse {
            status: recovered.status,
            last_sequence: WireU64::from_u64(recovered.last_sequence),
            stream: Some(wire::handle(
                recovered.stream_id.clone(),
                HandleKind::Stream,
                generation,
            )),
            terminal: recovered.terminal.clone(),
            receipt: recovered.receipt.clone(),
        },
    };
    responder.respond(response)
}

pub(crate) async fn run_wait(
    state: Arc<CoreProfileState>,
    request: RunWaitRequest,
    responder: Responder<RunWaitResponse>,
    connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/run/wait";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::Runs, method)?;
    if let Err(reason) = request.validate() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                reason,
                Retryability::Never,
                method,
            ),
        );
    }
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.run, HandleKind::Run, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let record = match state.handles.run(&request.run) {
        Ok(record) => record,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    match record.as_ref() {
        RunRecord::Live { entry } => {
            let entry = entry.clone();
            if let Some(receipt) = entry.receipt() {
                if let Err(error) = persist_receipt_now(&state, &entry, &receipt) {
                    return respond_error(|error| responder.respond_with_error(error), error);
                }
                let terminal = match wire::terminal_of(&receipt) {
                    Ok(terminal) => terminal,
                    Err(error) => {
                        return respond_error(
                            |error| responder.respond_with_error(error),
                            wire::sdk_error(
                                ExtensionErrorCode::SerializationViolation,
                                error,
                                Retryability::Never,
                                method,
                            ),
                        );
                    }
                };
                let receipt_wire = match wire::receipt_wire(&receipt) {
                    Ok(receipt) => receipt,
                    Err(error) => {
                        return respond_error(
                            |error| responder.respond_with_error(error),
                            wire::sdk_error(
                                ExtensionErrorCode::SerializationViolation,
                                error,
                                Retryability::Never,
                                method,
                            ),
                        );
                    }
                };
                return responder.respond(RunWaitResponse {
                    settled: true,
                    terminal: Some(terminal),
                    receipt: Some(receipt_wire),
                });
            }
            let timeout = request.timeout.as_ref().map(|duration| {
                Duration::new(
                    duration.seconds.to_u64().unwrap_or(u64::MAX),
                    duration.nanos,
                )
            });
            let wait_state = state.clone();
            connection.spawn(async move {
                let waited = super::events::wait_with_timeout(entry.wait_receipt(), timeout).await;
                match waited {
                    Some(receipt) => {
                        match persist_receipt_now(&wait_state, &entry, &receipt)
                            .and_then(|_| {
                                wire::terminal_of(&receipt).map_err(|error| {
                                    wire::sdk_error(
                                        ExtensionErrorCode::SerializationViolation,
                                        error,
                                        Retryability::Never,
                                        method,
                                    )
                                })
                            })
                            .and_then(|terminal| {
                                wire::receipt_wire(&receipt)
                                    .map(|receipt_wire| (terminal, receipt_wire))
                                    .map_err(|error| {
                                        wire::sdk_error(
                                            ExtensionErrorCode::SerializationViolation,
                                            error,
                                            Retryability::Never,
                                            method,
                                        )
                                    })
                            }) {
                            Ok((terminal, receipt_wire)) => responder.respond(RunWaitResponse {
                                settled: true,
                                terminal: Some(terminal),
                                receipt: Some(receipt_wire),
                            }),
                            Err(error) => {
                                responder.respond_with_error(wire::into_jsonrpc_error(error))
                            }
                        }
                    }
                    None => responder.respond(RunWaitResponse {
                        settled: false,
                        terminal: None,
                        receipt: None,
                    }),
                }
            })
        }
        RunRecord::Recovered(recovered) => {
            let (status, terminal, receipt) = (
                recovered.status,
                recovered.terminal.clone(),
                recovered.receipt.clone(),
            );
            if status == RunStatus::Interrupted {
                // A crashed predecessor can never be waited into success.
                return respond_error(
                    |error| responder.respond_with_error(error),
                    wire::handle_error(
                        ExtensionErrorCode::HostExited,
                        "the run was interrupted when the previous Host process exited",
                        method,
                        &request.run,
                    ),
                );
            }
            responder.respond(RunWaitResponse {
                settled: true,
                terminal,
                receipt,
            })
        }
    }
}

pub(crate) async fn run_cancel(
    state: Arc<CoreProfileState>,
    request: RunCancelRequest,
    responder: Responder<RunCancelResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/run/cancel";
    let services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::Runs, method)?;
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.run, HandleKind::Run, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let record = match state.handles.run(&request.run) {
        Ok(record) => record,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    match record.as_ref() {
        RunRecord::Live { entry } => {
            let initiated = entry.cancel();
            let status = wire::status_of(entry.receipt().as_deref());
            responder.respond(RunCancelResponse {
                cancellation_initiated: initiated,
                status,
            })
        }
        RunRecord::Recovered(recovered) => {
            let _ = services;
            responder.respond(RunCancelResponse {
                cancellation_initiated: false,
                status: recovered.status,
            })
        }
    }
}

pub(crate) async fn run_steer(
    state: Arc<CoreProfileState>,
    request: echo_sdk_protocol::methods::RunSteerRequest,
    responder: Responder<echo_sdk_protocol::methods::RunSteerResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/run/steer";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::Runs, method)?;
    if let Err(reason) = request.validate() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                reason,
                Retryability::Never,
                method,
            ),
        );
    }
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.run, HandleKind::Run, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let record = match state.handles.run(&request.run) {
        Ok(record) => record,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let entry = match record.as_ref() {
        RunRecord::Live { entry } => entry.clone(),
        RunRecord::Recovered(recovered) => {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::handle_error(
                    ExtensionErrorCode::HostExited,
                    format!("run status {:?} cannot be steered", recovered.status),
                    method,
                    &request.run,
                ),
            );
        }
    };
    let services = state
        .services()
        .map_err(|error| agent_client_protocol::Error::internal_error().data(error.to_string()))?;
    let session = services
        .sessions()
        .get(&entry.session_id)
        .await
        .ok_or_else(|| {
            agent_client_protocol::Error::internal_error().data("run session missing")
        })?;
    let receipt = session
        .agent
        .steer_input_tracked(Some(&entry.run_id), Message::user(request.text));
    match receipt {
        Ok(receipt) => responder.respond(echo_sdk_protocol::methods::RunSteerResponse {
            accepted: true,
            steer_id: Some(receipt.steer_id().to_string()),
        }),
        Err(error) => responder
            .respond(echo_sdk_protocol::methods::RunSteerResponse {
                accepted: false,
                steer_id: None,
            })
            .map(|_| {
                tracing::debug!("steer rejected: {error}");
            }),
    }
}

// ── Replay and ACK ──────────────────────────────────────────────────────────

pub(crate) async fn run_replay(
    state: Arc<CoreProfileState>,
    request: ReplayRequest,
    responder: Responder<ReplayResponse>,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/run/replay";
    let _services = require_extended(&state, method).await?;
    require_capability(&state, ExtensionCapability::EventReplay, method)?;
    if let Err(error) = request.validate() {
        return respond_error(
            |error| responder.respond_with_error(error),
            wire::sdk_error(
                ExtensionErrorCode::InvalidValue,
                format!("invalid replay request: {error}"),
                Retryability::Never,
                method,
            ),
        );
    }
    if let Err(error) =
        state
            .handles
            .check_shape_and_generation(&request.stream, HandleKind::Stream, method)
    {
        return respond_error(|error| responder.respond_with_error(error), error);
    }
    let stream_record = match state.handles.stream(&request.stream) {
        Ok(record) => record,
        Err(error) => return respond_error(|error| responder.respond_with_error(error), error),
    };
    let run_id = stream_record.run_handle_id.clone();
    let journal = match state.journal(&run_id) {
        Ok(journal) => journal,
        Err(error) => {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::framework_error(&error, method),
            );
        }
    };
    let after = request.after_sequence.to_u64().unwrap_or_default();
    let requested_limit = request
        .max_events
        .as_ref()
        .and_then(|limit| limit.to_u64())
        .unwrap_or(u64::MAX);
    let limit = usize::try_from(requested_limit.min(state.limits.max_replay_events as u64))
        .unwrap_or(usize::MAX);
    match super::events::bounded_replay(
        state.limits.max_replay_bytes,
        journal.as_ref(),
        &request.stream,
        after,
        limit,
    ) {
        Ok(response) => match state.delivery(&request.stream.id) {
            Some(delivery) => {
                delivery
                    .publish_replay(&request.stream, response, |response| {
                        responder.respond(response)
                    })
                    .await
            }
            None => responder.respond(response),
        },
        Err(error) => respond_error(|error| responder.respond_with_error(error), error),
    }
}

pub(crate) async fn event_ack(
    state: Arc<CoreProfileState>,
    notification: EventAckNotification,
    _connection: ConnectionTo<Client>,
) -> std::result::Result<(), agent_client_protocol::Error> {
    let method = "_echo_agent/event/ack";
    // Notifications never produce responses; malformed or unknown ACKs are
    // ignored (ACP notification rules) with a bounded stderr diagnostic.
    if let Err(error) = notification.ack.validate() {
        tracing::debug!("ignoring invalid {method}: {error}");
        return Ok(());
    }
    if let Err(error) = state.handles.check_shape_and_generation(
        &notification.ack.stream,
        HandleKind::Stream,
        method,
    ) {
        tracing::debug!("ignoring unmatched {method}: {}", error.message);
        return Ok(());
    }
    let Some(delivery) = state.delivery(&notification.ack.stream.id) else {
        tracing::debug!(
            "ignoring {method} for unknown stream {}",
            notification.ack.stream.id
        );
        return Ok(());
    };
    if let Err(error) = delivery.acknowledge(&notification.ack).await {
        tracing::warn!("ack resume failed: {error}");
    }
    Ok(())
}

// ── Extension bridge (feature `sdk-extension-bridge`) ───────────────────────

#[cfg(feature = "sdk-extension-bridge")]
mod extension_handlers {
    use super::*;
    use echo_sdk_protocol::methods::{
        ExtensionRegisterRequest, ExtensionRegisterResponse, ExtensionStreamEvent,
        ExtensionUnregisterRequest, ExtensionUnregisterResponse,
    };

    /// `_echo_agent/extension/register`: registration is connection-owned.
    /// The transport handle captured here serves every proxy invocation of
    /// this connection. Idempotent per identity + registration fingerprint
    /// (descriptor and default timeout); a different snapshot for the same
    /// identity is a typed conflict.
    pub(crate) async fn extension_register(
        state: Arc<CoreProfileState>,
        request: ExtensionRegisterRequest,
        responder: Responder<ExtensionRegisterResponse>,
        connection: ConnectionTo<Client>,
    ) -> std::result::Result<(), agent_client_protocol::Error> {
        let method = "_echo_agent/extension/register";
        require_extended(&state, method).await?;
        require_capability(&state, ExtensionCapability::ExtensionBridge, method)?;
        if let Err(reason) = request.validate() {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::sdk_error(
                    ExtensionErrorCode::InvalidConfig,
                    reason,
                    Retryability::Never,
                    method,
                ),
            );
        }
        #[cfg(not(feature = "framework-channels"))]
        if matches!(
            request.kind,
            echo_sdk_protocol::methods::ExtensionKind::ChannelPlugin
                | echo_sdk_protocol::methods::ExtensionKind::ChannelMessageHandler
        ) {
            return respond_error(
                |error| responder.respond_with_error(error),
                wire::sdk_error(
                    ExtensionErrorCode::FeatureUnavailable,
                    "channel extensions require the framework-channels Host feature",
                    Retryability::Never,
                    method,
                ),
            );
        }
        // Bind the official transport once; every later reverse invocation
        // of this connection goes through this single handle.
        state.extension_shared.bind_connection(connection.clone());
        match state.handles.register_extension(
            crate::core_profile::handles::ExtensionRegistrationLimits {
                max_extensions: state.limits.max_registered_extensions,
                max_descriptor_bytes: state.limits.max_extension_descriptor_bytes,
            },
            request.kind,
            &request.implementation_id,
            request.descriptor,
            request.timeout,
            false,
        ) {
            Ok((extension, _record)) => responder.respond(ExtensionRegisterResponse { extension }),
            Err(error) => respond_error(
                |error| responder.respond_with_error(error),
                error.with_operation(method),
            ),
        }
    }

    /// `_echo_agent/extension/unregister`: idempotent release. In-flight
    /// invocations of the released registration settle through their own
    /// lease lifecycle; new invocations fail closed.
    pub(crate) async fn extension_unregister(
        state: Arc<CoreProfileState>,
        request: ExtensionUnregisterRequest,
        responder: Responder<ExtensionUnregisterResponse>,
        _connection: ConnectionTo<Client>,
    ) -> std::result::Result<(), agent_client_protocol::Error> {
        let method = "_echo_agent/extension/unregister";
        require_extended(&state, method).await?;
        require_capability(&state, ExtensionCapability::ExtensionBridge, method)?;
        if let Err(error) = state.handles.check_shape_and_generation(
            &request.extension,
            HandleKind::Extension,
            method,
        ) {
            return respond_error(|error| responder.respond_with_error(error), error);
        }
        match state.handles.close_extension(&request.extension) {
            Ok(released) => {
                if released {
                    state
                        .extension_shared
                        .remove_streams_for_extension(&request.extension.id);
                }
                responder.respond(ExtensionUnregisterResponse { released })
            }
            Err(error) => respond_error(
                |error| responder.respond_with_error(error),
                error.with_operation(method),
            ),
        }
    }

    /// `_echo_agent/extension/stream` (client → host notification): route
    /// one chunk or terminal to its stream sink. Unknown streams and late
    /// events are discarded with bounded diagnostics.
    pub(crate) async fn extension_stream(
        state: Arc<CoreProfileState>,
        event: ExtensionStreamEvent,
    ) -> std::result::Result<(), String> {
        let services = state.services().map_err(|error| error.to_string())?;
        if !services.is_extended().await {
            return Ok(());
        }
        let method = "_echo_agent/extension/stream";
        if state
            .handles
            .check_shape_and_generation(event.stream(), HandleKind::Stream, method)
            .is_err()
        {
            return Ok(());
        }
        if services.ensure_admission().is_err() {
            // Teardown closes new work, while an already admitted producer
            // must still deliver its canonical terminal to settle that owner.
            // No resource/invocation is created by this narrow notification.
            if !event.is_terminal() {
                return Ok(());
            }
            let Ok(stream) = state.handles.stream(event.stream()) else {
                return Ok(());
            };
            if stream.facade || stream.resource_id.is_some() {
                return Ok(());
            }
            let extension = echo_sdk_protocol::handle::WireHandle {
                id: stream.run_handle_id.clone(),
                kind: HandleKind::Extension,
                generation: event.stream().generation.clone(),
            };
            if state.handles.extension_for_cleanup(&extension).is_err()
                || !state
                    .extension_shared
                    .owns_settlement_stream(event.stream(), &extension.id)
            {
                return Ok(());
            }
        }
        let encoded = serde_json::to_vec(&event)
            .map_err(|error| format!("stream event is not encodable: {error}"))?;
        if encoded.len() > state.limits.max_extension_stream_bytes {
            state.extension_shared.fail_stream(
                event.stream(),
                "extension stream event exceeds the configured byte bound",
            );
            return Err("extension stream event exceeds the configured byte bound".to_string());
        }
        state.extension_shared.deliver_stream_event(event)
    }
}

#[cfg(feature = "sdk-extension-bridge")]
pub(crate) use extension_handlers::{extension_register, extension_stream, extension_unregister};

#[cfg(test)]
mod recovered_identity_tests {
    use super::*;
    use echo_agent::agent::{Agent, AgentEvent};
    use futures::{future::BoxFuture, stream::BoxStream};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    #[derive(Default)]
    struct ReservationCloseProbe {
        fail: AtomicBool,
        attempts: AtomicUsize,
    }
    struct ReservationCloseAgent(Arc<ReservationCloseProbe>);
    impl Agent for ReservationCloseAgent {
        fn name(&self) -> &str {
            "recovery-close-fixture"
        }
        fn model_name(&self) -> &str {
            "fixture"
        }
        fn system_prompt(&self) -> &str {
            "fixture"
        }
        fn execute<'a>(&'a self, _: &'a str) -> BoxFuture<'a, echo_agent::error::Result<String>> {
            Box::pin(async { Ok("unused".to_string()) })
        }
        fn execute_stream<'a>(
            &'a self,
            _: &'a str,
        ) -> BoxFuture<
            'a,
            echo_agent::error::Result<BoxStream<'a, echo_agent::error::Result<AgentEvent>>>,
        > {
            Box::pin(async { Ok(Box::pin(futures::stream::empty()) as BoxStream<'a, _>) })
        }
        fn close(&self) -> BoxFuture<'_, echo_agent::error::Result<()>> {
            Box::pin(async move {
                self.0.attempts.fetch_add(1, Ordering::AcqRel);
                if self.0.fail.load(Ordering::Acquire) {
                    return Err(echo_agent::error::ReactError::Other(
                        "reservation cleanup fixture failure".to_string(),
                    ));
                }
                Ok(())
            })
        }
    }

    #[tokio::test]
    async fn failed_reservation_retains_unpublished_framework_session_for_same_owner_cleanup()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = tempfile::tempdir()?;
        let config: crate::SdkHostConfig = serde_json::from_value(serde_json::json!({
            "schema_version":1,"default_agent":{
                "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
                "agent":{"name":"fixture","enable_tools":true}
            }
        }))?;
        let prepared = config.prepare()?;
        let state = CoreProfileState::install(
            root.path(),
            crate::config::SdkProfileLimits::default(),
            prepared.framework,
            prepared.llm_client,
        )?;
        let probe = Arc::new(ReservationCloseProbe::default());
        probe.fail.store(true, Ordering::Release);
        let factory = {
            let probe = probe.clone();
            move |_: AcpSessionContext| {
                let probe = probe.clone();
                async move { Ok(Box::new(ReservationCloseAgent(probe)) as Box<dyn Agent>) }
            }
        };
        let registry = Arc::new(echo_agent::acp::SessionRegistry::new(Arc::new(factory), 1));
        registry
            .initialize(agent_client_protocol::schema::v1::ClientCapabilities::default())
            .await;
        let services = Arc::new(echo_agent::acp::AcpConnectionServices::new(
            registry,
            Arc::new(echo_agent::acp::AcpAdapterConfig::default()),
        ));
        state.bind_services(services.clone());
        let stable_id = "cleanup-reservation-session";
        state.persistence.record_run_started(
            "cleanup-reservation-session:turn:01",
            "fixture-stream",
            stable_id,
            "historical-agent",
            "",
            None,
            "chat",
        )?;
        let result = create_session_on(
            &state,
            &services,
            state.default_definition.as_ref(),
            stable_id.to_string(),
            Some(root.path().to_path_buf()),
            "_echo_agent/session/load",
        )
        .await;
        let error = match result {
            Err(error) => error,
            Ok(_) => return Err("invalid historical identity admitted a Session".into()),
        };
        assert_eq!(error.operation.as_deref(), Some("_echo_agent/session/load"));
        assert_eq!(error.retryable, Retryability::AfterDelay);
        assert!(error.message.contains("cleanup pending"));
        assert!(state.handles.session_handle_for_acp(stable_id).is_none());
        let id = agent_client_protocol::schema::v1::SessionId::new(stable_id.to_string());
        let retained = services
            .sessions()
            .get(&id)
            .await
            .ok_or("failed close lost framework Session owner")?;
        assert_eq!(probe.attempts.load(Ordering::Acquire), 1);
        probe.fail.store(false, Ordering::Release);
        services.close_sessions().await?;
        assert!(services.sessions().get(&id).await.is_none());
        assert_eq!(probe.attempts.load(Ordering::Acquire), 2);
        drop(retained);
        Ok(())
    }
}
