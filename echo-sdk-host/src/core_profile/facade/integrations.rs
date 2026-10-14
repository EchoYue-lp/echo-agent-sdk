//! Integration family adapters: MCP, A2A, LSP, topology (plan 07, todo 5).
//!
//! Each family resource-izes the framework's own client/manager and calls
//! its real verbs; connection semantics, protocol behavior and failures
//! stay with the framework integrations.
//!
//! Channels are bound when both the framework channel feature and the
//! extension bridge are compiled. Telemetry has its own process-scoped
//! adapter in `facade::telemetry`; it is intentionally not represented by a
//! manager resource in this module.

#[cfg(feature = "framework-lsp")]
use echo_agent::lsp::LspClient as _;
use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::handle::WireHandle;
use echo_sdk_protocol::methods::FeatureOperationRequest;
use echo_sdk_protocol::scalar::{WirePath, WireU64, WireValue};
#[cfg(any(
    feature = "framework-a2a",
    all(feature = "framework-channels", feature = "sdk-extension-bridge")
))]
use futures::StreamExt as _;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::handles::HandleRegistry;
use super::super::wire;

const METHOD: &str = "_echo_agent/integration/op";

/// One MCP manager resource. Connect takes `&mut` and awaits, so the
/// lock must stay Send across the await (tokio Mutex).
pub(crate) struct McpManagerRecord {
    pub manager: tokio::sync::Mutex<echo_agent::mcp::McpManager>,
}

/// One A2A client resource.
#[cfg(feature = "framework-a2a")]
pub(crate) struct A2aClientRecord {
    pub client: echo_agent::a2a::A2AClient,
}

/// One LSP manager resource; status_all awaits under the lock.
#[cfg(feature = "framework-lsp")]
pub(crate) struct LspManagerRecord {
    pub manager: tokio::sync::Mutex<echo_agent::lsp::LspManager>,
}

/// One topology tracker resource.
#[cfg(feature = "framework-topology")]
pub(crate) struct TopologyRecord {
    pub tracker: echo_agent::topology::TopologyTracker,
}

#[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
pub(crate) struct ChannelManagerRecord {
    pub manager: tokio::sync::Mutex<echo_agent::channels::ChannelManager>,
}

/// All integration family resources of one Host connection.
pub(crate) struct IntegrationResources {
    /// Unpublished preparations and publication failures retain framework
    /// cleanup ownership even when the invoking RPC future is dropped.
    #[cfg(feature = "framework-mcp")]
    mcp_preparations:
        Mutex<HashMap<String, (String, echo_agent::mcp::integration::McpPreparationScope)>>,
    #[cfg(feature = "framework-mcp")]
    mcp_cleanup_debt: Mutex<HashMap<String, (String, Arc<echo_agent::mcp::McpClient>)>>,
    #[cfg(feature = "framework-mcp")]
    pub mcp: Mutex<HashMap<String, Arc<McpManagerRecord>>>,
    #[cfg(feature = "framework-mcp")]
    pub mcp_clients: Mutex<HashMap<String, Arc<echo_agent::mcp::McpClient>>>,
    #[cfg(feature = "framework-mcp")]
    mcp_client_managers: Mutex<HashMap<String, String>>,
    #[cfg(feature = "framework-mcp")]
    pub mcp_tools: Mutex<HashMap<String, Arc<dyn echo_agent::tools::Tool>>>,
    #[cfg(feature = "framework-a2a")]
    pub a2a: Mutex<HashMap<String, Arc<A2aClientRecord>>>,
    #[cfg(feature = "framework-lsp")]
    pub lsp: Mutex<HashMap<String, Arc<LspManagerRecord>>>,
    #[cfg(feature = "framework-lsp")]
    pub lsp_clients:
        Mutex<HashMap<String, Arc<tokio::sync::RwLock<echo_agent::lsp::StdioLspClient>>>>,
    #[cfg(feature = "framework-topology")]
    pub topology: Mutex<HashMap<String, Arc<TopologyRecord>>>,
    #[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
    pub channels: Mutex<HashMap<String, Arc<ChannelManagerRecord>>>,
}

impl IntegrationResources {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "framework-mcp")]
            mcp_preparations: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-mcp")]
            mcp_cleanup_debt: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-mcp")]
            mcp: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-mcp")]
            mcp_clients: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-mcp")]
            mcp_client_managers: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-mcp")]
            mcp_tools: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-a2a")]
            a2a: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-lsp")]
            lsp: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-lsp")]
            lsp_clients: Mutex::new(HashMap::new()),
            #[cfg(feature = "framework-topology")]
            topology: Mutex::new(HashMap::new()),
            #[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
            channels: Mutex::new(HashMap::new()),
        }
    }

    /// Close every integration resource (connection teardown, bounded by
    /// the caller's shutdown timeout). MCP managers await `close_all` so
    /// connected child processes are taken down, not just dropped.
    pub async fn close_all(&self, handles: &HandleRegistry) -> Result<(), String> {
        let ids = integration_ids(self);
        let resources = close_session_resources(self, &ids).await;
        #[cfg(feature = "framework-mcp")]
        {
            let debt = self.close_mcp_debt(handles, None).await;
            match (resources, debt) {
                (Ok(()), Ok(())) => Ok(()),
                (Err(first), Err(second)) => Err(format!("{first}; {second}")),
                (Err(error), _) | (_, Err(error)) => Err(error),
            }
        }
        #[cfg(not(feature = "framework-mcp"))]
        {
            let _ = handles;
            resources
        }
    }

    #[cfg(feature = "framework-mcp")]
    pub(crate) async fn prepare_mcp_client(
        &self,
        handles: &HandleRegistry,
        owner: &str,
        max_resources: usize,
        create: impl FnOnce()
            -> Result<echo_agent::mcp::integration::McpClientPreparation, EchoSdkError>,
    ) -> Result<Arc<echo_agent::mcp::McpClient>, EchoSdkError> {
        // Reserve under the one resource authority before calling any
        // constructor. Pending cleanup consumes this same global quota.
        let (reservation, _) = handles.register_facade_resource(
            max_resources,
            "mcp",
            "mcp.pending_client",
            Some(owner),
            METHOD,
        )?;
        let preparation = match create() {
            Ok(preparation) => preparation,
            Err(error) => {
                handles.close_facade_resource(&reservation, METHOD)?;
                return Err(error);
            }
        };
        let ticket = reservation.id.clone();
        let cleanup = preparation.cleanup_scope();
        self.mcp_preparations
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(ticket.clone(), (owner.to_string(), cleanup.clone()));
        let client = match preparation.await {
            Ok(client) => client,
            Err(error) => {
                // Settled initialization failures do not accumulate tombstone
                // debt; a failed close retains both slot and cleanup scope.
                if cleanup.close().await.is_ok() {
                    self.mcp_preparations
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .remove(&ticket);
                    handles.close_facade_resource(&reservation, METHOD)?;
                }
                return Err(framework(error));
            }
        };
        self.mcp_cleanup_debt
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(ticket.clone(), (owner.to_string(), client.clone()));
        self.mcp_preparations
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&ticket);
        Ok(client)
    }

    #[cfg(feature = "framework-mcp")]
    pub(crate) async fn close_mcp_debt(
        &self,
        handles: &HandleRegistry,
        owner: Option<&str>,
    ) -> Result<(), String> {
        let preparations = self
            .mcp_preparations
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|(_, (scope, _))| owner.is_none_or(|owner| owner == scope))
            .map(|(id, (_, preparation))| (id.clone(), preparation.clone()))
            .collect::<Vec<_>>();
        let clients = self
            .mcp_cleanup_debt
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|(_, (scope, _))| owner.is_none_or(|owner| owner == scope))
            .map(|(id, (_, client))| (id.clone(), client.clone()))
            .collect::<Vec<_>>();
        let mut failures = Vec::new();
        for (id, preparation) in preparations {
            match preparation.close().await {
                Ok(()) => {
                    self.mcp_preparations
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .remove(&id);
                    let handle = WireHandle {
                        id,
                        kind: echo_sdk_protocol::handle::HandleKind::FacadeResource,
                        generation: WireU64::from_u64(handles.generation()),
                    };
                    if let Err(error) = handles.close_facade_resource(&handle, METHOD) {
                        failures.push(error.message);
                    }
                }
                Err(error) => failures.push(error.to_string()),
            }
        }
        for (id, client) in clients {
            match client.close().await {
                Ok(()) => {
                    self.mcp_cleanup_debt
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .remove(&id);
                    let handle = WireHandle {
                        id,
                        kind: echo_sdk_protocol::handle::HandleKind::FacadeResource,
                        generation: WireU64::from_u64(handles.generation()),
                    };
                    if let Err(error) = handles.close_facade_resource(&handle, METHOD) {
                        failures.push(error.message);
                    }
                }
                Err(error) => failures.push(error.to_string()),
            }
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures.join("; "))
        }
    }
}

fn invalid(message: impl Into<String>) -> EchoSdkError {
    wire::sdk_error(
        ExtensionErrorCode::InvalidValue,
        message,
        Retryability::Never,
        METHOD,
    )
}

fn framework(message: impl std::fmt::Display) -> EchoSdkError {
    wire::sdk_error(
        ExtensionErrorCode::FrameworkError,
        wire::bounded_framework_message(&message.to_string()),
        Retryability::Never,
        METHOD,
    )
}

#[cfg(feature = "framework-mcp")]
fn cleanup_failure(message: impl std::fmt::Display) -> EchoSdkError {
    wire::sdk_error(
        ExtensionErrorCode::FrameworkError,
        wire::bounded_framework_message(&message.to_string()),
        Retryability::AfterDelay,
        METHOD,
    )
}

/// Owner-checked MCP manager lookup.
fn mcp_of(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    resource: &WireHandle,
    owner: &str,
) -> Result<Arc<McpManagerRecord>, EchoSdkError> {
    super::owned_resource(handles, resource, owner, METHOD)?;
    let record = resources
        .mcp
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&resource.id)
        .cloned()
        .ok_or_else(|| invalid(format!("unknown MCP manager {}", resource.id)))?;
    Ok(record)
}

#[cfg(feature = "framework-mcp")]
fn mcp_client_of(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    client: &WireHandle,
    owner: &str,
) -> Result<Arc<echo_agent::mcp::McpClient>, EchoSdkError> {
    let record = super::owned_resource(handles, client, owner, METHOD)?;
    if record.resource_type != "mcp.client" {
        return Err(invalid("facade resource is not an MCP client"));
    }
    resources
        .mcp_clients
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&client.id)
        .cloned()
        .ok_or_else(|| invalid(format!("unknown MCP client resource {}", client.id)))
}

#[cfg(feature = "framework-mcp")]
pub(crate) fn register_mcp_client(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    client: Arc<echo_agent::mcp::McpClient>,
    owner: &str,
    max_resources: usize,
) -> Result<WireHandle, EchoSdkError> {
    let reserved = resources
        .mcp_cleanup_debt
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .find(|(_, (scope, debt))| scope == owner && Arc::ptr_eq(debt, &client))
        .map(|(id, _)| WireHandle {
            id: id.clone(),
            kind: echo_sdk_protocol::handle::HandleKind::FacadeResource,
            generation: WireU64::from_u64(handles.generation()),
        });
    let handle = match reserved {
        Some(handle) => {
            handles.commit_mcp_preparation(&handle)?;
            handle
        }
        None => {
            handles
                .register_facade_resource(max_resources, "mcp", "mcp.client", Some(owner), METHOD)?
                .0
        }
    };
    resources
        .mcp_clients
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .insert(handle.id.clone(), client.clone());
    resources
        .mcp_cleanup_debt
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .retain(|_, (_, debt)| !Arc::ptr_eq(debt, &client));
    Ok(handle)
}

#[cfg(feature = "framework-mcp")]
pub(crate) fn register_agent_mcp_clients(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    clients: &[Arc<echo_agent::mcp::McpClient>],
    owner: &str,
    max_resources: usize,
    operation: &str,
) -> Result<Vec<WireHandle>, EchoSdkError> {
    let mut aliases = resources
        .mcp_clients
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let generation = handles.generation();
    let existing = |client: &Arc<echo_agent::mcp::McpClient>| {
        aliases.iter().find_map(|(id, current)| {
            if !Arc::ptr_eq(current, client) {
                return None;
            }
            let handle = WireHandle {
                id: id.clone(),
                generation: WireU64::from_u64(generation),
                kind: echo_sdk_protocol::handle::HandleKind::FacadeResource,
            };
            handles
                .facade_resource(&handle, operation)
                .ok()
                .filter(|record| {
                    record.resource_type == "mcp.client"
                        && record.owner_session.as_deref() == Some(owner)
                })
                .map(|_| handle)
        })
    };
    let mut result = Vec::with_capacity(clients.len());
    let mut unpublished = Vec::new();
    for client in clients {
        if let Some(handle) = existing(client) {
            result.push((Some(handle), None));
        } else {
            let position = unpublished
                .iter()
                .position(|current| Arc::ptr_eq(current, client));
            let position = match position {
                Some(position) => position,
                None => {
                    let position = unpublished.len();
                    unpublished.push(client.clone());
                    position
                }
            };
            result.push((None, Some(position)));
        }
    }
    // These clients already belong to the live framework Agent. Publication
    // failure must not disconnect them or consume an unpublished prefix. No
    // preparation/cleanup-debt reservation is transferred by this path.
    let published = handles.register_facade_resource_batch(
        max_resources,
        "mcp",
        "mcp.client",
        Some(owner),
        operation,
        unpublished.len(),
    )?;
    // All quota/owner checks preceded registry insertion. Materialize result
    // addresses using the ordered allocation, before exposing map aliases.
    let result = result
        .into_iter()
        .map(|(existing, position)| {
            existing
                .or_else(|| {
                    position.and_then(|position| {
                        published.get(position).map(|(handle, _)| handle.clone())
                    })
                })
                .ok_or_else(|| {
                    wire::sdk_error(
                        ExtensionErrorCode::FrameworkError,
                        "atomic MCP publication omitted a reserved alias",
                        Retryability::Never,
                        operation,
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>();
    let result = match result {
        Ok(result) => result,
        Err(mut error) => {
            // Only these newly minted SDK addresses are rolled back; existing
            // aliases and framework-owned clients are never closed by name.
            let cleanup = published
                .iter()
                .filter_map(|(handle, _)| {
                    handles
                        .close_facade_resource(handle, operation)
                        .err()
                        .map(|error| error.message)
                })
                .collect::<Vec<_>>();
            if !cleanup.is_empty() {
                error.details = Some(echo_sdk_protocol::error::ErrorDetails {
                    fields: Some(vec![echo_sdk_protocol::error::DetailField {
                        key: "alias_rollback_errors".to_string(),
                        value: cleanup.join("; ").chars().take(2048).collect(),
                    }]),
                    facade: None,
                });
            }
            return Err(error);
        }
    };
    for ((handle, _), client) in published.into_iter().zip(unpublished) {
        aliases.insert(handle.id, client);
    }
    Ok(result)
}

#[cfg(feature = "framework-mcp")]
fn mcp_tool_snapshot(
    tool: &dyn echo_agent::tools::Tool,
    handle: &WireHandle,
    client: Option<&WireHandle>,
) -> Result<WireValue, EchoSdkError> {
    let parameters = WireValue::from_json(tool.parameters())
        .map_err(|error| invalid(format!("MCP tool schema is not wire-safe: {error}")))?;
    let mut fields = vec![
        echo_sdk_protocol::scalar::WireField {
            name: "tool".to_string(),
            value: WireValue::Handle(handle.clone()),
        },
        echo_sdk_protocol::scalar::WireField {
            name: "name".to_string(),
            value: WireValue::String(tool.name().to_string()),
        },
        echo_sdk_protocol::scalar::WireField {
            name: "description".to_string(),
            value: WireValue::String(tool.description().to_string()),
        },
        echo_sdk_protocol::scalar::WireField {
            name: "parameters".to_string(),
            value: parameters,
        },
        echo_sdk_protocol::scalar::WireField {
            name: "schema_revision".to_string(),
            value: WireValue::U64(WireU64::from_u64(tool.schema_revision())),
        },
    ];
    if let Some(client) = client {
        fields.push(echo_sdk_protocol::scalar::WireField {
            name: "client".to_string(),
            value: WireValue::Handle(client.clone()),
        });
    }
    Ok(WireValue::Record {
        type_id: "echo_sdk_protocol::McpToolReference".to_string(),
        fields,
    })
}

#[cfg(feature = "framework-mcp")]
fn register_mcp_tools(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    tools: Vec<Box<dyn echo_agent::tools::Tool>>,
    owner: &str,
    max_resources: usize,
    client: Option<&WireHandle>,
) -> Result<Vec<WireValue>, EchoSdkError> {
    let mut registered = Vec::new();
    let result = (|| {
        let mut snapshots = Vec::with_capacity(tools.len());
        for tool in tools {
            let (handle, _) = handles.register_facade_resource(
                max_resources,
                "mcp",
                "mcp.tool",
                Some(owner),
                METHOD,
            )?;
            let tool: Arc<dyn echo_agent::tools::Tool> = Arc::from(tool);
            let snapshot = mcp_tool_snapshot(tool.as_ref(), &handle, client)?;
            resources
                .mcp_tools
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .insert(handle.id.clone(), tool);
            registered.push(handle);
            snapshots.push(snapshot);
        }
        Ok(snapshots)
    })();
    if result.is_err() {
        let mut tools = resources
            .mcp_tools
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        for handle in registered {
            tools.remove(&handle.id);
            let _ = handles.close_facade_resource(&handle, METHOD);
        }
    }
    result
}

#[cfg(feature = "framework-mcp")]
fn mcp_tool_of(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    tool: &WireHandle,
    owner: &str,
) -> Result<Arc<dyn echo_agent::tools::Tool>, EchoSdkError> {
    let record = super::owned_resource(handles, tool, owner, METHOD)?;
    if record.resource_type != "mcp.tool" {
        return Err(invalid("facade resource is not an executable MCP tool"));
    }
    resources
        .mcp_tools
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&tool.id)
        .cloned()
        .ok_or_else(|| invalid(format!("unknown MCP tool resource {}", tool.id)))
}

#[cfg(feature = "framework-a2a")]
/// Owner-checked A2A client lookup.
fn a2a_of(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    resource: &WireHandle,
    owner: &str,
) -> Result<Arc<A2aClientRecord>, EchoSdkError> {
    super::owned_resource(handles, resource, owner, METHOD)?;
    let record = resources
        .a2a
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&resource.id)
        .cloned()
        .ok_or_else(|| invalid(format!("unknown A2A client {}", resource.id)))?;
    Ok(record)
}

#[cfg(feature = "framework-lsp")]
/// Owner-checked LSP manager lookup.
fn lsp_of(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    resource: &WireHandle,
    owner: &str,
) -> Result<Arc<LspManagerRecord>, EchoSdkError> {
    super::owned_resource(handles, resource, owner, METHOD)?;
    let record = resources
        .lsp
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&resource.id)
        .cloned()
        .ok_or_else(|| invalid(format!("unknown LSP manager {}", resource.id)))?;
    Ok(record)
}

#[cfg(feature = "framework-lsp")]
fn lsp_client_of(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    resource: &WireHandle,
    owner: &str,
) -> Result<Arc<tokio::sync::RwLock<echo_agent::lsp::StdioLspClient>>, EchoSdkError> {
    let record = super::owned_resource(handles, resource, owner, METHOD)?;
    if record.family != "lsp" || record.resource_type != "lsp.client" {
        return Err(invalid("resource does not address an LSP client"));
    }
    resources
        .lsp_clients
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&resource.id)
        .cloned()
        .ok_or_else(|| invalid(format!("unknown LSP client {}", resource.id)))
}

#[cfg(feature = "framework-topology")]
/// Owner-checked topology tracker lookup.
fn topology_of(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    resource: &WireHandle,
    owner: &str,
) -> Result<Arc<TopologyRecord>, EchoSdkError> {
    super::owned_resource(handles, resource, owner, METHOD)?;
    let record = resources
        .topology
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&resource.id)
        .cloned()
        .ok_or_else(|| invalid(format!("unknown topology tracker {}", resource.id)))?;
    Ok(record)
}

#[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
fn channel_of(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    resource: &WireHandle,
    owner: &str,
) -> Result<Arc<ChannelManagerRecord>, EchoSdkError> {
    super::owned_resource(handles, resource, owner, METHOD)?;
    resources
        .channels
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&resource.id)
        .cloned()
        .ok_or_else(|| invalid(format!("unknown channel manager {}", resource.id)))
}

/// Share the existing resource close authority while an original Channel
/// operation can produce effects. Metadata deliberately does not take this
/// permit: a plugin callback may await ids/list on its own manager.
#[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
async fn channel_resource_operation<'a>(
    handles: &HandleRegistry,
    resource: &WireHandle,
    owner: &str,
    record: &'a super::super::handles::FacadeResourceRecord,
) -> Result<tokio::sync::RwLockReadGuard<'a, ()>, EchoSdkError> {
    let permit = record.lifecycle.read().await;
    // A close queued ahead of this request may have settled while we waited.
    // The original handle registry owns the only closed/closing facts.
    super::owned_resource(handles, resource, owner, METHOD)?;
    Ok(permit)
}

#[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
fn channel_forward_error(error: echo_agent::error::ReactError) -> EchoSdkError {
    match &error {
        echo_agent::error::ReactError::Channel(cause) => match cause.as_ref() {
            echo_agent::error::ChannelError::InvalidConfig(_) => invalid(error.to_string()),
            echo_agent::error::ChannelError::StaleInvocation { .. } => wire::sdk_error(
                ExtensionErrorCode::ClosedHandle,
                error.to_string(),
                Retryability::Never,
                METHOD,
            ),
            _ => framework(error),
        },
        _ => framework(error),
    }
}

fn json_of(value: &WireValue, position: usize) -> Result<serde_json::Value, EchoSdkError> {
    super::family_argument_json(value).map_err(|error| {
        invalid(format!(
            "argument {position} is not a lossless wire value: {error}"
        ))
    })
}

fn string_at(
    arguments: &[serde_json::Value],
    position: usize,
    what: &str,
) -> Result<String, EchoSdkError> {
    arguments
        .get(position)
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| invalid(format!("operation requires {what} at argument {position}")))
}

#[cfg(feature = "framework-mcp")]
fn string_map(
    value: Option<&serde_json::Value>,
    field: &str,
) -> Result<HashMap<String, String>, EchoSdkError> {
    match value {
        None | Some(serde_json::Value::Null) => Ok(HashMap::new()),
        Some(serde_json::Value::Object(entries)) => entries
            .iter()
            .map(|(key, value)| {
                value
                    .as_str()
                    .map(|value| (key.clone(), value.to_string()))
                    .ok_or_else(|| invalid(format!("MCP {field}.{key} must be a string")))
            })
            .collect(),
        Some(_) => Err(invalid(format!("MCP {field} must be an object"))),
    }
}

#[cfg(feature = "framework-mcp")]
fn mcp_server_config_at(
    arguments: &[serde_json::Value],
    position: usize,
) -> Result<echo_agent::mcp::McpServerConfig, EchoSdkError> {
    let object = arguments
        .get(position)
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            invalid(format!(
                "MCP config at argument {position} must be an object"
            ))
        })?;
    let name = object
        .get("name")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| invalid("MCP config requires a non-empty name"))?
        .to_string();
    let transport = object
        .get("transport")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| invalid("MCP config requires a transport object"))?;
    let kind = transport
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| invalid("MCP transport requires kind"))?;
    let transport = match kind {
        "stdio" => {
            let command = transport
                .get("command")
                .and_then(serde_json::Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| invalid("MCP stdio transport requires command"))?
                .to_string();
            let args = match transport.get("args") {
                None | Some(serde_json::Value::Null) => Vec::new(),
                Some(serde_json::Value::Array(values)) => values
                    .iter()
                    .map(|value| {
                        value
                            .as_str()
                            .map(str::to_string)
                            .ok_or_else(|| invalid("MCP stdio args must contain strings"))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                Some(_) => return Err(invalid("MCP stdio args must be an array")),
            };
            let env = string_map(transport.get("env"), "transport.env")?
                .into_iter()
                .collect();
            let cwd = transport
                .get("cwd")
                .filter(|value| !value.is_null())
                .map(|value| {
                    serde_json::from_value::<WirePath>(value.clone())
                        .map_err(|error| invalid(format!("MCP stdio cwd is invalid: {error}")))
                        .and_then(|path| wire::path_from_wire(&path).map_err(invalid))
                })
                .transpose()?;
            echo_agent::mcp::TransportConfig::Stdio {
                command,
                args,
                env,
                cwd,
            }
        }
        "http" | "sse" => {
            let base_url = transport
                .get("base_url")
                .and_then(serde_json::Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| invalid("MCP HTTP/SSE transport requires base_url"))?
                .to_string();
            let headers = string_map(transport.get("headers"), "transport.headers")?;
            if kind == "http" {
                echo_agent::mcp::TransportConfig::Http { base_url, headers }
            } else {
                echo_agent::mcp::TransportConfig::Sse { base_url, headers }
            }
        }
        other => {
            return Err(invalid(format!(
                "unknown MCP transport {other}; expected stdio|http|sse"
            )));
        }
    };
    Ok(echo_agent::mcp::McpServerConfig { name, transport })
}

#[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
fn handle_at(
    arguments: &[serde_json::Value],
    position: usize,
    what: &str,
) -> Result<WireHandle, EchoSdkError> {
    arguments
        .get(position)
        .cloned()
        .ok_or_else(|| invalid(format!("operation requires {what} at argument {position}")))
        .and_then(|value| {
            serde_json::from_value(value).map_err(|error| {
                invalid(format!("{what} at argument {position} is invalid: {error}"))
            })
        })
}

/// Dispatch one integration family operation.
pub(crate) async fn dispatch(
    handles: &HandleRegistry,
    family: &str,
    resources: &IntegrationResources,
    _streams: &super::stream::FacadeStreamRuntime,
    owner: &str,
    request: &FeatureOperationRequest,
    max_resources: usize,
) -> Result<WireValue, EchoSdkError> {
    let wire = |value: serde_json::Value| {
        WireValue::from_json(value).map_err(|error| invalid(error.to_string()))
    };
    let arguments: Vec<serde_json::Value> = request
        .arguments
        .iter()
        .enumerate()
        .map(|(position, value)| json_of(value, position))
        .collect::<Result<_, _>>()?;
    match family {
        #[cfg(feature = "framework-mcp")]
        "mcp" => match request.operation.as_str() {
            "mcp.client.open_exact" => {
                let config = mcp_server_config_at(&arguments, 0)?;
                let client = resources
                    .prepare_mcp_client(handles, owner, max_resources, || {
                        Ok(echo_agent::mcp::McpClient::new(config))
                    })
                    .await?;
                let handle = register_mcp_client(handles, resources, client, owner, max_resources)?;
                Ok(WireValue::Handle(handle))
            }
            "mcp.manager.open" | "mcp.manager.open_exact" => {
                let (resource, _record) = handles.register_facade_resource(
                    max_resources,
                    "mcp",
                    "mcp.manager",
                    Some(owner),
                    METHOD,
                )?;
                resources
                    .mcp
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .insert(
                        resource.id.clone(),
                        Arc::new(McpManagerRecord {
                            manager: tokio::sync::Mutex::new(echo_agent::mcp::McpManager::new()),
                        }),
                    );
                if request.operation == "mcp.manager.open_exact" {
                    Ok(WireValue::Handle(resource))
                } else {
                    wire(serde_json::json!({"resource": resource}))
                }
            }
            "mcp.server.connect" | "mcp.server.connect_exact" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let config = if request.operation == "mcp.server.connect_exact" {
                    mcp_server_config_at(&arguments, 1)?
                } else {
                    let name = string_at(&arguments, 1, "a server name")?;
                    let mode = string_at(&arguments, 2, "a transport mode (stdio|http)")?;
                    match mode.as_str() {
                        "stdio" => {
                            let command = string_at(&arguments, 3, "a command")?;
                            let args = match arguments.get(4) {
                                None | Some(serde_json::Value::Null) => Vec::new(),
                                Some(serde_json::Value::Array(items)) => items
                                    .iter()
                                    .map(|item| {
                                        item.as_str().map(str::to_string).ok_or_else(|| {
                                            invalid("MCP command arguments must be strings")
                                        })
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                                Some(_) => {
                                    return Err(invalid("MCP command arguments must be a list"));
                                }
                            };
                            echo_agent::mcp::McpServerConfig::stdio(&name, &command, args)
                        }
                        "http" => {
                            let base_url = string_at(&arguments, 3, "a base URL")?;
                            echo_agent::mcp::McpServerConfig::http(&name, &base_url)
                        }
                        other => {
                            return Err(invalid(format!(
                                "unknown MCP transport {other}; expected stdio|http"
                            )));
                        }
                    }
                };
                let name = config.name.clone();
                let record = mcp_of(handles, resources, &resource, owner)?;
                let mut manager = record.manager.lock().await;
                let tools = manager.connect(config).await.map_err(framework)?;
                let client = manager
                    .get_client(&name)
                    .ok_or_else(|| framework("MCP client was not retained"))?;
                let client_handle = match register_mcp_client(
                    handles,
                    resources,
                    client.clone(),
                    owner,
                    max_resources,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        let _ = manager.disconnect(&name).await;
                        return Err(error);
                    }
                };
                resources
                    .mcp_client_managers
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .insert(client_handle.id.clone(), resource.id.clone());
                if request.operation == "mcp.server.connect_exact" {
                    match register_mcp_tools(
                        handles,
                        resources,
                        tools,
                        owner,
                        max_resources,
                        Some(&client_handle),
                    ) {
                        Ok(tools) => Ok(WireValue::List(tools)),
                        Err(error) => {
                            resources
                                .mcp_clients
                                .lock()
                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                .remove(&client_handle.id);
                            resources
                                .mcp_client_managers
                                .lock()
                                .unwrap_or_else(|e| e.into_inner())
                                .remove(&client_handle.id);
                            let _ = handles.close_facade_resource(&client_handle, METHOD);
                            let _ = manager.disconnect(&name).await;
                            Err(error)
                        }
                    }
                } else {
                    wire(serde_json::json!({
                        "tools": tools.len(),
                        "client": client_handle,
                        "server_name": client.server_name(),
                    }))
                }
            }
            "mcp.server.list" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let record = mcp_of(handles, resources, &resource, owner)?;
                let manager = record.manager.lock().await;
                wire(serde_json::json!({"servers": manager.server_names()}))
            }
            "mcp.manager.connect_from_config" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let config: echo_agent::mcp::McpConfigFile = arguments
                    .get(1)
                    .cloned()
                    .ok_or_else(|| invalid("McpManager::connect_from_config requires a config"))
                    .and_then(|value| {
                        serde_json::from_value(value).map_err(|error| {
                            invalid(format!("MCP config file is malformed: {error}"))
                        })
                    })?;
                let record = mcp_of(handles, resources, &resource, owner)?;
                let tools = record
                    .manager
                    .lock()
                    .await
                    .connect_from_config(&config)
                    .await
                    .map_err(framework)?;
                Ok(WireValue::List(register_mcp_tools(
                    handles,
                    resources,
                    tools,
                    owner,
                    max_resources,
                    None,
                )?))
            }
            "mcp.manager.get_client" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let name = string_at(&arguments, 1, "a server name")?;
                let record = mcp_of(handles, resources, &resource, owner)?;
                let manager = record.manager.lock().await;
                let Some(client) = manager.get_client(&name) else {
                    return Ok(WireValue::Variant {
                        type_id: "core::option::Option<McpClient>".to_string(),
                        variant: "none".to_string(),
                        fields: Vec::new(),
                    });
                };
                let client = register_mcp_client(handles, resources, client, owner, max_resources)?;
                resources
                    .mcp_client_managers
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .insert(client.id.clone(), resource.id.clone());
                Ok(WireValue::Variant {
                    type_id: "core::option::Option<McpClient>".to_string(),
                    variant: "some".to_string(),
                    fields: vec![echo_sdk_protocol::scalar::WireField {
                        name: "client".to_string(),
                        value: WireValue::Handle(client),
                    }],
                })
            }
            "mcp.manager.get_clients" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let record = mcp_of(handles, resources, &resource, owner)?;
                let clients = record.manager.lock().await.get_clients();
                let mut entries = Vec::with_capacity(clients.len());
                let mut registered: Vec<WireHandle> = Vec::with_capacity(clients.len());
                for (name, client) in clients {
                    let client =
                        match register_mcp_client(handles, resources, client, owner, max_resources)
                        {
                            Ok(client) => client,
                            Err(error) => {
                                let mut clients = resources
                                    .mcp_clients
                                    .lock()
                                    .unwrap_or_else(|error| error.into_inner());
                                for registered in registered {
                                    clients.remove(&registered.id);
                                    resources
                                        .mcp_client_managers
                                        .lock()
                                        .unwrap_or_else(|e| e.into_inner())
                                        .remove(&registered.id);
                                    let _ = handles.close_facade_resource(&registered, METHOD);
                                }
                                return Err(error);
                            }
                        };
                    resources
                        .mcp_client_managers
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .insert(client.id.clone(), resource.id.clone());
                    registered.push(client.clone());
                    entries.push(echo_sdk_protocol::scalar::WireMapEntry {
                        key: WireValue::String(name),
                        value: WireValue::Handle(client),
                    });
                }
                Ok(WireValue::Map(entries))
            }
            "mcp.manager.server_names" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let record = mcp_of(handles, resources, &resource, owner)?;
                Ok(WireValue::List(
                    record
                        .manager
                        .lock()
                        .await
                        .server_names()
                        .into_iter()
                        .map(WireValue::String)
                        .collect(),
                ))
            }
            "mcp.manager.get_all_tools" | "mcp.manager.resource_tools" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let record = mcp_of(handles, resources, &resource, owner)?;
                let manager = record.manager.lock().await;
                let tools = if request.operation == "mcp.manager.get_all_tools" {
                    manager.get_all_tools()
                } else {
                    manager.resource_tools()
                };
                Ok(WireValue::List(register_mcp_tools(
                    handles,
                    resources,
                    tools,
                    owner,
                    max_resources,
                    None,
                )?))
            }
            "mcp.manager.reconcile_target" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let name = string_at(&arguments, 1, "a server name")?;
                let desired = match arguments.get(2) {
                    None | Some(serde_json::Value::Null) => None,
                    Some(value) => Some(mcp_server_config_at(std::slice::from_ref(value), 0)?),
                };
                let record = mcp_of(handles, resources, &resource, owner)?;
                let receipt = record
                    .manager
                    .lock()
                    .await
                    .reconcile_target(&name, desired)
                    .await
                    .map_err(framework)?;
                let change = match receipt.change {
                    echo_agent::mcp::McpTargetChange::Connected => "connected",
                    echo_agent::mcp::McpTargetChange::Replaced => "replaced",
                    echo_agent::mcp::McpTargetChange::Unchanged => "unchanged",
                    echo_agent::mcp::McpTargetChange::Disconnected => "disconnected",
                    echo_agent::mcp::McpTargetChange::Absent => "absent",
                };
                Ok(WireValue::Record {
                    type_id: "echo_integration::mcp::McpTargetReceipt".to_string(),
                    fields: vec![
                        echo_sdk_protocol::scalar::WireField {
                            name: "name".to_string(),
                            value: WireValue::String(receipt.name),
                        },
                        echo_sdk_protocol::scalar::WireField {
                            name: "change".to_string(),
                            value: WireValue::String(change.to_string()),
                        },
                        echo_sdk_protocol::scalar::WireField {
                            name: "tools".to_string(),
                            value: WireValue::List(register_mcp_tools(
                                handles,
                                resources,
                                receipt.tools,
                                owner,
                                max_resources,
                                None,
                            )?),
                        },
                    ],
                })
            }
            "mcp.manager.close_all" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let record = mcp_of(handles, resources, &resource, owner)?;
                record
                    .manager
                    .lock()
                    .await
                    .close_all()
                    .await
                    .map_err(cleanup_failure)?;
                wire(serde_json::Value::Null)
            }
            "mcp.server.disconnect" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let name = string_at(&arguments, 1, "a server name")?;
                let record = mcp_of(handles, resources, &resource, owner)?;
                let mut manager = record.manager.lock().await;
                let disconnected = manager.disconnect(&name).await.map_err(cleanup_failure)?;
                if disconnected {
                    let client_ids = resources
                        .mcp_clients
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .iter()
                        .filter(|(id, client)| {
                            client.server_name() == name
                                && resources
                                    .mcp_client_managers
                                    .lock()
                                    .unwrap_or_else(|e| e.into_inner())
                                    .get(*id)
                                    .is_some_and(|manager| manager == &resource.id)
                        })
                        .map(|(id, _)| id.clone())
                        .collect::<Vec<_>>();
                    for id in client_ids {
                        resources
                            .mcp_clients
                            .lock()
                            .unwrap_or_else(|error| error.into_inner())
                            .remove(&id);
                        resources
                            .mcp_client_managers
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .remove(&id);
                        let handle = WireHandle {
                            id,
                            generation: WireU64::from_u64(handles.generation()),
                            kind: echo_sdk_protocol::handle::HandleKind::FacadeResource,
                        };
                        let _ = handles.close_facade_resource(&handle, METHOD);
                    }
                }
                wire(serde_json::json!({"disconnected": disconnected}))
            }
            "mcp.tool.execute" => {
                let tool_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP tool resource", METHOD)?;
                let parameters = arguments
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({}));
                let parameters = serde_json::from_value(parameters).map_err(|error| {
                    invalid(format!("MCP tool parameters are malformed: {error}"))
                })?;
                let tool = mcp_tool_of(handles, resources, &tool_handle, owner)?;
                let result = tool.execute(parameters).await.map_err(framework)?;
                wire(serde_json::to_value(result).map_err(framework)?)
            }
            "echo_agent::mcp::McpClient::call_tool" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let name = string_at(&arguments, 1, "a MCP tool name")?;
                let tool_arguments = arguments
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({}));
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                let result = client
                    .call_tool(&name, tool_arguments)
                    .await
                    .map_err(framework)?;
                wire(serde_json::to_value(result).map_err(|error| invalid(error.to_string()))?)
            }
            "echo_agent::mcp::McpClient::get_prompt" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let name = string_at(&arguments, 1, "an MCP prompt name")?;
                let prompt_arguments = arguments
                    .get(2)
                    .cloned()
                    .map(serde_json::from_value::<std::collections::HashMap<String, String>>)
                    .transpose()
                    .map_err(|error| invalid(format!("prompt arguments are malformed: {error}")))?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                let result = client
                    .get_prompt(&name, prompt_arguments)
                    .await
                    .map_err(framework)?;
                wire(serde_json::to_value(result).map_err(|error| invalid(error.to_string()))?)
            }
            "echo_agent::mcp::McpClient::list_resource_templates" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                let result = client.list_resource_templates().await.map_err(framework)?;
                wire(serde_json::to_value(result).map_err(|error| invalid(error.to_string()))?)
            }
            "echo_agent::mcp::McpClient::list_resources" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                let result = client.list_resources().await.map_err(framework)?;
                wire(serde_json::to_value(result).map_err(|error| invalid(error.to_string()))?)
            }
            "echo_agent::mcp::McpClient::read_resource" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let uri = string_at(&arguments, 1, "a resource URI")?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                let result = client.read_resource(&uri).await.map_err(framework)?;
                wire(serde_json::to_value(result).map_err(|error| invalid(error.to_string()))?)
            }
            "echo_agent::mcp::McpClient::ping" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                client.ping().await.map_err(framework)?;
                wire(serde_json::Value::Null)
            }
            "echo_agent::mcp::McpClient::server_name" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                wire(serde_json::json!(client.server_name()))
            }
            "echo_agent::mcp::McpClient::protocol_version" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                wire(serde_json::json!(client.protocol_version()))
            }
            "echo_agent::mcp::McpClient::server_capabilities" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                wire(
                    serde_json::to_value(client.server_capabilities())
                        .map_err(|error| invalid(error.to_string()))?,
                )
            }
            "echo_agent::mcp::McpClient::supports_resources" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                wire(serde_json::json!(client.supports_resources()))
            }
            "echo_agent::mcp::McpClient::supports_prompts" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                wire(serde_json::json!(client.supports_prompts()))
            }
            "echo_agent::mcp::McpClient::tools" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                wire(
                    serde_json::to_value(client.tools())
                        .map_err(|error| invalid(error.to_string()))?,
                )
            }
            "echo_agent::mcp::McpClient::resources" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                wire(
                    serde_json::to_value(client.resources())
                        .map_err(|error| invalid(error.to_string()))?,
                )
            }
            "echo_agent::mcp::McpClient::prompts" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                wire(
                    serde_json::to_value(client.prompts())
                        .map_err(|error| invalid(error.to_string()))?,
                )
            }
            "echo_agent::mcp::McpClient::close" => {
                let client_handle =
                    super::resource_handle_at(&arguments, 0, "an MCP client resource", METHOD)?;
                let client = mcp_client_of(handles, resources, &client_handle, owner)?;
                client.close().await.map_err(cleanup_failure)?;
                resources
                    .mcp_clients
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .remove(&client_handle.id);
                resources
                    .mcp_client_managers
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .remove(&client_handle.id);
                let _ = handles.close_facade_resource(&client_handle, METHOD);
                wire(serde_json::Value::Null)
            }
            other => Err(invalid(format!(
                "unknown mcp operation {other}; the family surface is closed"
            ))),
        },
        #[cfg(feature = "framework-a2a")]
        "a2a" => match request.operation.as_str() {
            "a2a.client.open" => {
                let (resource, _record) = handles.register_facade_resource(
                    max_resources,
                    "a2a",
                    "a2a.client",
                    Some(owner),
                    METHOD,
                )?;
                resources
                    .a2a
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .insert(
                        resource.id.clone(),
                        Arc::new(A2aClientRecord {
                            client: echo_agent::a2a::A2AClient::new(),
                        }),
                    );
                wire(serde_json::json!({"resource": resource}))
            }
            "a2a.discover" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an A2A client resource", METHOD)?;
                let base_url = string_at(&arguments, 1, "a base URL")?;
                let record = a2a_of(handles, resources, &resource, owner)?;
                let card = record.client.discover(&base_url).await.map_err(framework)?;
                wire(serde_json::to_value(&card).unwrap_or_else(|_| serde_json::Value::Null))
            }
            "a2a.task.send" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an A2A client resource", METHOD)?;
                let agent_url = string_at(&arguments, 1, "an agent URL")?;
                let message = string_at(&arguments, 2, "a message")?;
                let record = a2a_of(handles, resources, &resource, owner)?;
                let session_id = match arguments.get(3) {
                    None | Some(serde_json::Value::Null) => None,
                    Some(serde_json::Value::String(value)) if !value.trim().is_empty() => {
                        Some(value.clone())
                    }
                    _ => return Err(invalid("A2A task session id must be a string or null")),
                };
                let task = record
                    .client
                    .send_task_with_session(&agent_url, &message, session_id)
                    .await
                    .map_err(framework)?;
                wire(serde_json::to_value(&task).unwrap_or(serde_json::Value::Null))
            }
            "a2a.task.get" | "a2a.task.cancel" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an A2A client resource", METHOD)?;
                let agent_url = string_at(&arguments, 1, "an agent URL")?;
                let task_id = string_at(&arguments, 2, "a task id")?;
                let record = a2a_of(handles, resources, &resource, owner)?;
                let task = if request.operation == "a2a.task.cancel" {
                    record
                        .client
                        .cancel_task(&agent_url, &task_id)
                        .await
                        .map_err(framework)?
                } else {
                    record
                        .client
                        .get_task(&agent_url, &task_id)
                        .await
                        .map_err(framework)?
                };
                wire(serde_json::to_value(&task).unwrap_or(serde_json::Value::Null))
            }
            "a2a.task.stream.open" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an A2A client resource", METHOD)?;
                let agent_url = string_at(&arguments, 1, "an agent URL")?;
                let message = string_at(&arguments, 2, "a message")?;
                let session_id = match arguments.get(3) {
                    None | Some(serde_json::Value::Null) => None,
                    Some(serde_json::Value::String(value)) if !value.trim().is_empty() => {
                        Some(value.clone())
                    }
                    _ => return Err(invalid("A2A stream session id must be a string or null")),
                };
                let record = a2a_of(handles, resources, &resource, owner)?;
                let mut source = record
                    .client
                    .send_task_streaming_with_session(&agent_url, &message, session_id)
                    .await
                    .map_err(framework)?;
                let producer = _streams.open(handles, &resource, owner, &request.operation)?;
                let sender = producer.sender.clone();
                let cancel = producer.cleanup.clone();
                let background = tokio::spawn(async move {
                    loop {
                        let item = tokio::select! {
                            () = cancel.cancelled() => break,
                            item = source.next() => item,
                        };
                        let Some(event) = item else { break };
                        let item = serde_json::to_value(event)
                            .map_err(|error| {
                                wire::sdk_error(
                                    ExtensionErrorCode::FrameworkError,
                                    format!("A2A stream event serialization failed: {error}"),
                                    Retryability::Never,
                                    "a2a.task.stream.open",
                                )
                            })
                            .and_then(|value| {
                                WireValue::from_json(value).map_err(|error| {
                                    wire::sdk_error(
                                        ExtensionErrorCode::FrameworkError,
                                        format!("A2A stream event projection failed: {error}"),
                                        Retryability::Never,
                                        "a2a.task.stream.open",
                                    )
                                })
                            });
                        tokio::select! {
                            () = cancel.cancelled() => break,
                            result = sender.send(item) => {
                                if result.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                });
                _streams.attach_background(&producer, background);
                Ok(WireValue::Handle(producer.handle))
            }
            other => Err(invalid(format!(
                "unknown a2a operation {other}; the family surface is closed"
            ))),
        },
        #[cfg(feature = "framework-lsp")]
        "lsp" => match request.operation.as_str() {
            "lsp.manager.open" | "lsp.manager.open_exact" => {
                let (resource, _record) = handles.register_facade_resource(
                    max_resources,
                    "lsp",
                    "lsp.manager",
                    Some(owner),
                    METHOD,
                )?;
                resources
                    .lsp
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .insert(
                        resource.id.clone(),
                        Arc::new(LspManagerRecord {
                            manager: tokio::sync::Mutex::new(echo_agent::lsp::LspManager::new()),
                        }),
                    );
                if request.operation == "lsp.manager.open_exact" {
                    Ok(WireValue::Handle(resource))
                } else {
                    wire(serde_json::json!({"resource": resource}))
                }
            }
            "lsp.manager.load_config" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP manager resource", METHOD)?;
                let config_file: echo_agent::lsp::LspConfigFile = arguments
                    .get(1)
                    .cloned()
                    .ok_or_else(|| invalid("lsp.manager.load_config requires a config"))
                    .and_then(|value| {
                        serde_json::from_value(value)
                            .map_err(|error| invalid(format!("LSP config is malformed: {error}")))
                    })?;
                let config = echo_agent::lsp::LspConfig {
                    servers: config_file.languages,
                };
                let record = lsp_of(handles, resources, &resource, owner)?;
                record
                    .manager
                    .lock()
                    .await
                    .load_config(&config)
                    .map_err(framework)?;
                wire(serde_json::Value::Null)
            }
            "lsp.manager.set_project_root" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP manager resource", METHOD)?;
                let path = arguments
                    .get(1)
                    .cloned()
                    .ok_or_else(|| invalid("lsp.manager.set_project_root requires a path"))
                    .and_then(|value| {
                        serde_json::from_value::<WirePath>(value).map_err(|error| {
                            invalid(format!("LSP project root is invalid: {error}"))
                        })
                    })
                    .and_then(|path| wire::path_from_wire(&path).map_err(invalid))?;
                let record = lsp_of(handles, resources, &resource, owner)?;
                record.manager.lock().await.set_project_root(&path);
                wire(serde_json::Value::Null)
            }
            "lsp.manager.start_server"
            | "lsp.manager.stop_server"
            | "lsp.manager.restart_server" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP manager resource", METHOD)?;
                let language = string_at(&arguments, 1, "a language")?;
                let record = lsp_of(handles, resources, &resource, owner)?;
                let mut manager = record.manager.lock().await;
                match request.operation.as_str() {
                    "lsp.manager.start_server" => manager.start_server(&language).await,
                    "lsp.manager.stop_server" => manager.stop_server(&language).await,
                    _ => manager.restart_server(&language).await,
                }
                .map_err(framework)?;
                wire(serde_json::Value::Null)
            }
            "lsp.manager.shutdown_all" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP manager resource", METHOD)?;
                let record = lsp_of(handles, resources, &resource, owner)?;
                record.manager.lock().await.shutdown_all().await;
                wire(serde_json::Value::Null)
            }
            "lsp.manager.get_client" | "lsp.manager.get_client_for_file" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP manager resource", METHOD)?;
                let selector = string_at(
                    &arguments,
                    1,
                    if request.operation == "lsp.manager.get_client" {
                        "a language"
                    } else {
                        "a file path"
                    },
                )?;
                let record = lsp_of(handles, resources, &resource, owner)?;
                let manager = record.manager.lock().await;
                let selected = if request.operation == "lsp.manager.get_client" {
                    manager.get_client(&selector).map(|client| (None, client))
                } else {
                    manager
                        .get_client_for_file(&selector)
                        .await
                        .map(|(language, client)| (Some(language), client))
                };
                let Some((language, client)) = selected else {
                    return Ok(WireValue::Variant {
                        type_id: "core::option::Option<LspClient>".to_string(),
                        variant: "none".to_string(),
                        fields: Vec::new(),
                    });
                };
                let (client_handle, _) = handles.register_facade_resource(
                    max_resources,
                    "lsp",
                    "lsp.client",
                    Some(owner),
                    METHOD,
                )?;
                resources
                    .lsp_clients
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .insert(client_handle.id.clone(), client);
                let mut fields = vec![echo_sdk_protocol::scalar::WireField {
                    name: "client".to_string(),
                    value: WireValue::Handle(client_handle),
                }];
                if let Some(language) = language {
                    fields.push(echo_sdk_protocol::scalar::WireField {
                        name: "language".to_string(),
                        value: WireValue::String(language),
                    });
                }
                Ok(WireValue::Variant {
                    type_id: "core::option::Option<LspClient>".to_string(),
                    variant: "some".to_string(),
                    fields,
                })
            }
            "lsp.client.language"
            | "lsp.client.is_running"
            | "lsp.client.is_initialized"
            | "lsp.client.status" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP client resource", METHOD)?;
                let client = lsp_client_of(handles, resources, &resource, owner)?;
                let client = client.read().await;
                match request.operation.as_str() {
                    "lsp.client.language" => wire(serde_json::json!(client.language())),
                    "lsp.client.is_running" => wire(serde_json::json!(client.is_running())),
                    "lsp.client.is_initialized" => wire(serde_json::json!(client.is_initialized())),
                    _ => wire(serde_json::to_value(client.status()).map_err(framework)?),
                }
            }
            "lsp.client.initialize" | "lsp.client.shutdown" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP client resource", METHOD)?;
                let client = lsp_client_of(handles, resources, &resource, owner)?;
                let mut client = client.write().await;
                if request.operation == "lsp.client.initialize" {
                    let root_uri = string_at(&arguments, 1, "a root URI")?;
                    client.initialize(&root_uri).await.map_err(framework)?;
                } else {
                    client.shutdown().await.map_err(framework)?;
                }
                wire(serde_json::Value::Null)
            }
            "lsp.client.diagnostics"
            | "lsp.client.goto_definition"
            | "lsp.client.find_references"
            | "lsp.client.hover"
            | "lsp.client.completion"
            | "lsp.client.did_open"
            | "lsp.client.did_change"
            | "lsp.client.did_save"
            | "lsp.client.did_close" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP client resource", METHOD)?;
                let uri = string_at(&arguments, 1, "a document URI")?;
                let client = lsp_client_of(handles, resources, &resource, owner)?;
                let client = client.read().await;
                let value = match request.operation.as_str() {
                    "lsp.client.diagnostics" => {
                        serde_json::to_value(client.diagnostics(&uri).await.map_err(framework)?)
                    }
                    "lsp.client.goto_definition" | "lsp.client.find_references" => {
                        let position: echo_agent::lsp::Position = arguments
                            .get(2)
                            .cloned()
                            .ok_or_else(|| invalid("LSP operation requires a position"))
                            .and_then(|value| {
                                serde_json::from_value(value).map_err(|error| {
                                    invalid(format!("LSP position is malformed: {error}"))
                                })
                            })?;
                        if request.operation == "lsp.client.goto_definition" {
                            serde_json::to_value(
                                client
                                    .goto_definition(&uri, position)
                                    .await
                                    .map_err(framework)?,
                            )
                        } else {
                            serde_json::to_value(
                                client
                                    .find_references(&uri, position)
                                    .await
                                    .map_err(framework)?,
                            )
                        }
                    }
                    "lsp.client.hover" | "lsp.client.completion" => {
                        let position: echo_agent::lsp::Position = arguments
                            .get(2)
                            .cloned()
                            .ok_or_else(|| invalid("LSP operation requires a position"))
                            .and_then(|value| {
                                serde_json::from_value(value).map_err(|error| {
                                    invalid(format!("LSP position is malformed: {error}"))
                                })
                            })?;
                        if request.operation == "lsp.client.hover" {
                            serde_json::to_value(
                                client.hover(&uri, position).await.map_err(framework)?,
                            )
                        } else {
                            serde_json::to_value(
                                client.completion(&uri, position).await.map_err(framework)?,
                            )
                        }
                    }
                    "lsp.client.did_open" => {
                        let language_id = string_at(&arguments, 2, "a language id")?;
                        let text = arguments
                            .get(3)
                            .and_then(serde_json::Value::as_str)
                            .ok_or_else(|| invalid("LSP did_open requires document text"))?;
                        client
                            .did_open(&uri, &language_id, text)
                            .await
                            .map_err(framework)?;
                        Ok(serde_json::Value::Null)
                    }
                    "lsp.client.did_change" => {
                        let changes: Vec<echo_agent::lsp::TextChange> = arguments
                            .get(2)
                            .cloned()
                            .ok_or_else(|| invalid("LSP did_change requires changes"))
                            .and_then(|value| {
                                serde_json::from_value(value).map_err(|error| {
                                    invalid(format!("LSP changes are malformed: {error}"))
                                })
                            })?;
                        client.did_change(&uri, changes).await.map_err(framework)?;
                        Ok(serde_json::Value::Null)
                    }
                    "lsp.client.did_save" => {
                        client.did_save(&uri).await.map_err(framework)?;
                        Ok(serde_json::Value::Null)
                    }
                    _ => {
                        client.did_close(&uri).await.map_err(framework)?;
                        Ok(serde_json::Value::Null)
                    }
                }
                .map_err(framework)?;
                wire(value)
            }
            "lsp.server.status" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let record = lsp_of(handles, resources, &resource, owner)?;
                let manager = record.manager.lock().await;
                let statuses: Vec<serde_json::Value> = manager
                    .status_all()
                    .await
                    .into_iter()
                    .map(|status| serde_json::to_value(&status).unwrap_or_default())
                    .collect();
                wire(serde_json::json!({"servers": statuses}))
            }
            "lsp.language.list" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an MCP manager resource", METHOD)?;
                let record = lsp_of(handles, resources, &resource, owner)?;
                let manager = record.manager.lock().await;
                wire(serde_json::json!({"configured": manager.configured_languages()}))
            }
            "lsp.language.configured" | "lsp.server.running" | "lsp.server.status_all" => {
                let resource =
                    super::resource_handle_at(&arguments, 0, "an LSP manager resource", METHOD)?;
                let record = lsp_of(handles, resources, &resource, owner)?;
                let manager = record.manager.lock().await;
                let value = match request.operation.as_str() {
                    "lsp.language.configured" => {
                        serde_json::to_value(manager.configured_languages())
                    }
                    "lsp.server.running" => serde_json::to_value(manager.running_servers()),
                    _ => serde_json::to_value(manager.status_all().await),
                }
                .map_err(framework)?;
                wire(value)
            }
            other => Err(invalid(format!(
                "unknown lsp operation {other}; the family surface is closed"
            ))),
        },
        #[cfg(feature = "framework-topology")]
        "topology" => match request.operation.as_str() {
            "topology.tracker.open" | "topology.tracker.open_exact" => {
                let (resource, _record) = handles.register_facade_resource(
                    max_resources,
                    "topology",
                    "topology.tracker",
                    Some(owner),
                    METHOD,
                )?;
                resources
                    .topology
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .insert(
                        resource.id.clone(),
                        Arc::new(TopologyRecord {
                            tracker: echo_agent::topology::TopologyTracker::new(),
                        }),
                    );
                if request.operation == "topology.tracker.open_exact" {
                    Ok(WireValue::Handle(resource))
                } else {
                    wire(serde_json::json!({"resource": resource}))
                }
            }
            "topology.node.add" => {
                let resource = super::resource_handle_at(
                    &arguments,
                    0,
                    "a topology tracker resource",
                    METHOD,
                )?;
                let node_id = string_at(&arguments, 1, "a node id")?;
                let node_type = string_at(&arguments, 2, "a node type")?;
                let node_type = match node_type.as_str() {
                    "agent" => echo_agent::topology::NodeType::Agent,
                    "orchestrator" => echo_agent::topology::NodeType::Orchestrator,
                    "subagent" => echo_agent::topology::NodeType::Subagent,
                    "planner" => echo_agent::topology::NodeType::Planner,
                    "external" => echo_agent::topology::NodeType::External,
                    "tool" => echo_agent::topology::NodeType::Tool,
                    other => {
                        return Err(invalid(format!(
                            "unknown topology node type {other}; expected agent|orchestrator|subagent|planner|external|tool"
                        )));
                    }
                };
                let record = topology_of(handles, resources, &resource, owner)?;
                record
                    .tracker
                    .add_node(echo_agent::topology::TopologyNode::new(node_id, node_type));
                wire(serde_json::json!({"ok": true}))
            }
            "topology.node.add_exact" => {
                let resource = super::resource_handle_at(
                    &arguments,
                    0,
                    "a topology tracker resource",
                    METHOD,
                )?;
                let node: echo_agent::topology::TopologyNode = arguments
                    .get(1)
                    .cloned()
                    .ok_or_else(|| invalid("topology.node.add_exact requires a node"))
                    .and_then(|value| {
                        serde_json::from_value(value).map_err(|error| {
                            invalid(format!("topology node is malformed: {error}"))
                        })
                    })?;
                let record = topology_of(handles, resources, &resource, owner)?;
                record.tracker.add_node(node);
                wire(serde_json::Value::Null)
            }
            "topology.call.record" => {
                let resource = super::resource_handle_at(
                    &arguments,
                    0,
                    "a topology tracker resource",
                    METHOD,
                )?;
                let from = string_at(&arguments, 1, "a from node id")?;
                let to = string_at(&arguments, 2, "a to node id")?;
                let label = string_at(&arguments, 3, "a call label")?;
                let record = topology_of(handles, resources, &resource, owner)?;
                record.tracker.record_call(&from, &to, &label);
                wire(serde_json::json!({"ok": true}))
            }
            "topology.call.record_with_duration" => {
                let resource = super::resource_handle_at(
                    &arguments,
                    0,
                    "a topology tracker resource",
                    METHOD,
                )?;
                let from = string_at(&arguments, 1, "a from node id")?;
                let to = string_at(&arguments, 2, "a to node id")?;
                let label = string_at(&arguments, 3, "a call label")?;
                let duration_ms = arguments
                    .get(4)
                    .and_then(|value| {
                        value
                            .as_u64()
                            .or_else(|| value.as_str().and_then(|text| text.parse::<u64>().ok()))
                    })
                    .ok_or_else(|| invalid("topology call duration must be a canonical u64"))?;
                let record = topology_of(handles, resources, &resource, owner)?;
                record
                    .tracker
                    .record_call_with_duration(&from, &to, &label, duration_ms);
                wire(serde_json::Value::Null)
            }
            "topology.clear" => {
                let resource = super::resource_handle_at(
                    &arguments,
                    0,
                    "a topology tracker resource",
                    METHOD,
                )?;
                let record = topology_of(handles, resources, &resource, owner)?;
                record.tracker.clear();
                wire(serde_json::Value::Null)
            }
            "topology.to_mermaid" | "topology.to_json" | "topology.to_dot" => {
                let resource = super::resource_handle_at(
                    &arguments,
                    0,
                    "a topology tracker resource",
                    METHOD,
                )?;
                let record = topology_of(handles, resources, &resource, owner)?;
                let value = match request.operation.as_str() {
                    "topology.to_mermaid" => record.tracker.to_mermaid(),
                    "topology.to_dot" => record.tracker.to_dot(),
                    _ => record
                        .tracker
                        .to_json()
                        .map_err(|error| framework(error.to_string()))?,
                };
                wire(serde_json::Value::String(value))
            }
            "topology.snapshot" => {
                let resource = super::resource_handle_at(
                    &arguments,
                    0,
                    "a topology tracker resource",
                    METHOD,
                )?;
                let record = topology_of(handles, resources, &resource, owner)?;
                wire(serde_json::json!({
                    "nodes": record.tracker.nodes(),
                    "edges": record.tracker.edges(),
                    "stats": serde_json::to_value(record.tracker.stats()).unwrap_or_default(),
                }))
            }
            "topology.nodes" | "topology.edges" | "topology.stats" => {
                let resource = super::resource_handle_at(
                    &arguments,
                    0,
                    "a topology tracker resource",
                    METHOD,
                )?;
                let record = topology_of(handles, resources, &resource, owner)?;
                let value = match request.operation.as_str() {
                    "topology.nodes" => serde_json::to_value(record.tracker.nodes()),
                    "topology.edges" => serde_json::to_value(record.tracker.edges()),
                    _ => serde_json::to_value(record.tracker.stats()),
                }
                .map_err(framework)?;
                wire(value)
            }
            other => Err(invalid(format!(
                "unknown topology operation {other}; the family surface is closed"
            ))),
        },
        _ => Err(invalid(format!(
            "integration family {family} is not available in this Host build"
        ))),
    }
}

/// Dispatch the channel family through the reverse extension bridge. This is
/// separate from the framework-owned integrations dispatcher because channel
/// plugins and handlers require the connection-scoped bridge authority.
#[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
pub(crate) async fn dispatch_channels(
    handles: &HandleRegistry,
    resources: &IntegrationResources,
    streams: &super::stream::FacadeStreamRuntime,
    bridge: &Arc<crate::core_profile::extension_bridge::ExtensionBridge>,
    owner: &str,
    request: &FeatureOperationRequest,
    max_resources: usize,
) -> Result<WireValue, EchoSdkError> {
    let wire = |value: serde_json::Value| {
        WireValue::from_json(value).map_err(|error| invalid(error.to_string()))
    };
    let arguments: Vec<serde_json::Value> = request
        .arguments
        .iter()
        .enumerate()
        .map(|(position, value)| json_of(value, position))
        .collect::<Result<_, _>>()?;
    if matches!(
        request.operation.as_str(),
        "channels.stream.next" | "channels.stream.cancel" | "channels.stream.close"
    ) {
        return super::stream::dispatch_control(streams, handles, owner, "channels", request).await;
    }
    let expected_arguments = match request.operation.as_str() {
        "channels.handler.handle" | "channels.handler.handle_stream" | "channels.handler.reply" => {
            Some(3)
        }
        "channels.plugin.register" => Some(3),
        "channels.manager.start" => Some(2),
        _ => None,
    };
    if expected_arguments.is_some_and(|expected| arguments.len() != expected) {
        return Err(invalid(
            "channel operation received missing or extra arguments",
        ));
    }
    let operation_record = match request.operation.as_str() {
        "channels.plugin.register"
        | "channels.manager.start"
        | "channels.manager.stop"
        | "channels.manager.stop_exact"
        | "channels.manager.stop_all"
        | "channels.manager.health"
        | "channels.plugin.send"
        | "channels.handler.handle"
        | "channels.handler.reply" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            Some((
                resource.clone(),
                super::owned_resource(handles, &resource, owner, METHOD)?,
            ))
        }
        _ => None,
    };
    let _resource_operation = match &operation_record {
        Some((resource, record)) => {
            Some(channel_resource_operation(handles, resource, owner, record).await?)
        }
        None => None,
    };
    match request.operation.as_str() {
        "channels.manager.open" => {
            let (resource, _record) = handles.register_facade_resource(
                max_resources,
                "channels",
                "channels.manager",
                Some(owner),
                METHOD,
            )?;
            resources
                .channels
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .insert(
                    resource.id.clone(),
                    Arc::new(ChannelManagerRecord {
                        manager: tokio::sync::Mutex::new(
                            echo_agent::channels::ChannelManager::new(),
                        ),
                    }),
                );
            wire(serde_json::json!({"resource": resource}))
        }
        "channels.plugin.register" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let plugin = handle_at(&arguments, 1, "a ChannelPlugin extension handle")?;
            let handler = handle_at(&arguments, 2, "a MessageHandler extension handle")?;
            let plugin_record = handles.extension(&plugin)?;
            let descriptor = match &plugin_record.descriptor {
                echo_sdk_protocol::methods::ExtensionDescriptor::ChannelPlugin(value) => {
                    value.clone()
                }
                _ => return Err(invalid("extension handle is not a ChannelPlugin")),
            };
            let handler_record = handles.extension(&handler)?;
            if handler_record.kind
                != echo_sdk_protocol::methods::ExtensionKind::ChannelMessageHandler
                || handler_record.implementation_id != descriptor.handler_id
            {
                return Err(invalid(
                    "MessageHandler extension does not match the ChannelPlugin descriptor",
                ));
            }
            let address = super::owned_resource(handles, &resource, owner, METHOD)?;
            let binding = super::super::handles::ChannelExtensionBinding {
                channel_id: descriptor.channel_id.clone(),
                plugin: plugin.clone(),
                handler: handler.clone(),
            };
            let record = channel_of(handles, resources, &resource, owner)?;
            let proxy = crate::core_profile::extension_bridge::ExtensionChannelPluginProxy::new(
                bridge.clone(),
                plugin,
                descriptor,
            );
            let mut manager = record.manager.lock().await;
            manager
                .register(Box::new(proxy))
                .map_err(channel_forward_error)?;
            address
                .channel_bindings
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .push(binding);
            wire(serde_json::json!({"registered": true}))
        }
        "channels.manager.start" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let handler = handle_at(&arguments, 1, "a MessageHandler extension handle")?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let bridge = bridge.clone();
            // Capture framework-owned operations under a short registry guard.
            // Reverse callbacks may await metadata on this same manager.
            let start = {
                let manager = record.manager.lock().await;
                let address = super::owned_resource(handles, &resource, owner, METHOD)?;
                let bindings = address
                    .channel_bindings
                    .lock()
                    .unwrap_or_else(|error| error.into_inner())
                    .clone();
                let ids = manager.channel_ids();
                if ids.len() != bindings.len()
                    || ids
                        .iter()
                        .any(|id| !bindings.iter().any(|binding| &binding.channel_id == id))
                {
                    return Err(invalid("channel registration binding is missing"));
                }
                // Validate every complete original identity before starting
                // any plugin, so one mismatch cannot leave a started prefix.
                for binding in &bindings {
                    if binding.handler != handler {
                        return Err(invalid(
                            "batch start handler differs from the registered extension identity",
                        ));
                    }
                    handles.extension(&binding.plugin)?;
                    let target = handles.extension(&binding.handler)?;
                    if target.kind
                        != echo_sdk_protocol::methods::ExtensionKind::ChannelMessageHandler
                    {
                        return Err(invalid("bound extension is not a Channel MessageHandler"));
                    }
                }
                manager.start_all_owned(move |_| {
                    Arc::new(
                        crate::core_profile::extension_bridge::ExtensionChannelMessageHandlerProxy::new(
                            bridge.clone(),
                            handler.clone(),
                        ),
                    ) as Arc<dyn echo_agent::channels::MessageHandler>
                })
            };
            let results = start.await;
            let failures = results
                .iter()
                .filter(|result| result.result.is_err())
                .count();
            wire(serde_json::json!({"channels": results.len(), "failures": failures}))
        }
        "channels.manager.stop" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let channel_id = string_at(&arguments, 1, "a channel id")?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let stop = {
                let manager = record.manager.lock().await;
                manager.stop_owned(&channel_id)
            };
            stop.await.map_err(channel_forward_error)?;
            wire(serde_json::json!({"stopped": true}))
        }
        "channels.manager.stop_exact" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let channel_id = string_at(&arguments, 1, "a channel id")?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let stop = {
                let manager = record.manager.lock().await;
                manager.stop_owned(&channel_id)
            };
            stop.await.map_err(channel_forward_error)?;
            Ok(WireValue::Null)
        }
        "channels.manager.stop_all" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let stop = {
                let manager = record.manager.lock().await;
                manager.stop_all_owned()
            };
            stop.await.map_err(channel_forward_error)?;
            Ok(WireValue::Null)
        }
        "channels.manager.health" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let channel_id = string_at(&arguments, 1, "a channel id")?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let channel = {
                let manager = record.manager.lock().await;
                manager.get_owned(&channel_id)
            }
            .ok_or_else(|| invalid(format!("channel {channel_id} is not registered")))?;
            channel
                .health_check()
                .await
                .map_err(channel_forward_error)?;
            wire(serde_json::json!({"healthy": true}))
        }
        "channels.handler.handle" | "channels.handler.reply" | "channels.handler.handle_stream" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let channel_id = string_at(&arguments, 1, "a channel id")?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let channel = {
                let manager = record.manager.lock().await;
                manager.get_owned(&channel_id)
            }
            .ok_or_else(|| invalid(format!("channel {channel_id} is not registered")))?;
            let input = arguments
                .get(2)
                .cloned()
                .ok_or_else(|| invalid("handler operation requires a typed message"))?;
            if request.operation == "channels.handler.reply" {
                let message: echo_sdk_protocol::methods::ChannelOutboundMessageWire =
                    serde_json::from_value(input).map_err(|error| {
                        invalid(format!("outbound message is malformed: {error}"))
                    })?;
                if message.channel_id != channel_id {
                    return Err(invalid(
                        "outbound channel_id does not match the selected entry",
                    ));
                }
                let message =
                    crate::core_profile::extension_bridge::channel_outbound_from_wire(message)
                        .map_err(|error| invalid(error.to_string()))?;
                let cancel = super::owned_resource(handles, &resource, owner, METHOD)?
                    .cancel_operations
                    .child_token();
                let _cancel_on_drop = cancel.clone().drop_guard();
                let result = channel.reply_message(message, cancel).await;
                super::owned_resource(handles, &resource, owner, METHOD)?;
                result.map_err(channel_forward_error)?;
                return Ok(WireValue::Null);
            }
            let message: echo_sdk_protocol::methods::ChannelInboundMessageWire =
                serde_json::from_value(input)
                    .map_err(|error| invalid(format!("inbound message is malformed: {error}")))?;
            if message.channel_id != channel_id {
                return Err(invalid(
                    "inbound channel_id does not match the selected entry",
                ));
            }
            let message = crate::core_profile::extension_bridge::channel_inbound_from_wire(message)
                .map_err(|error| invalid(error.to_string()))?;
            if request.operation == "channels.handler.handle" {
                let cancel = super::owned_resource(handles, &resource, owner, METHOD)?
                    .cancel_operations
                    .child_token();
                let _cancel_on_drop = cancel.clone().drop_guard();
                let result = channel.handle_message(message, cancel).await;
                super::owned_resource(handles, &resource, owner, METHOD)?;
                let output = result.map_err(channel_forward_error)?;
                let output =
                    crate::core_profile::extension_bridge::channel_outbound_to_wire(&output);
                return wire(serde_json::to_value(output).map_err(framework)?);
            }
            // Reserve the original registry stream/quota before invoking the
            // handler. This same manager address anchors channels controls.
            let address = super::owned_resource(handles, &resource, owner, METHOD)?;
            let permit = address.lifecycle.clone().read_owned().await;
            super::owned_resource(handles, &resource, owner, METHOD)?;
            let producer =
                streams.open_cooperative(handles, &resource, owner, &request.operation)?;
            let sender = producer.sender.clone();
            let cancel = producer.cancel.clone();
            let cleanup = producer.cleanup.clone();
            let cancel_setup_on_drop = producer.cancel.clone().drop_guard();
            let (setup_sender, setup_receiver) = tokio::sync::oneshot::channel();
            let resource_cancel = address.cancel_operations.child_token();
            let background = tokio::spawn(async move {
                let _permit = permit;
                let handler_cancel = cancel.child_token();
                let mut setup = channel.handle_message_stream(message, handler_cancel.clone());
                let source = tokio::select! {
                    () = resource_cancel.cancelled() => {
                        cancel.cancel();
                        setup.as_mut().await
                    }
                    () = cleanup.cancelled() => {
                        handler_cancel.cancel();
                        setup.as_mut().await
                    }
                    result = setup.as_mut() => result,
                };
                let mut source = match source {
                    Ok(source) => source,
                    Err(error) => {
                        let _ = setup_sender.send(Err(channel_forward_error(error)));
                        return;
                    }
                };
                let invocation_cancel = source.cancellation.clone();
                if setup_sender.send(Ok(())).is_err() {
                    cancel.cancel();
                    while source.next().await.is_some() {}
                    return;
                }
                loop {
                    let item = tokio::select! { () = cancel.cancelled() => break, () = resource_cancel.cancelled() => { cancel.cancel(); break; }, () = invocation_cancel.cancelled() => { cancel.cancel(); break; }, () = cleanup.cancelled() => break, item = source.next() => item };
                    let Some(item) = item else { break };
                    let output = item
                        .map_err(framework)
                        .and_then(|message| {
                            serde_json::to_value(
                                crate::core_profile::extension_bridge::channel_outbound_to_wire(
                                    &message,
                                ),
                            )
                            .map_err(framework)
                        })
                        .and_then(|value| WireValue::from_json(value).map_err(framework));
                    let failed = output.is_err();
                    tokio::select! { () = cancel.cancelled() => break, () = resource_cancel.cancelled() => { cancel.cancel(); break; }, () = invocation_cancel.cancelled() => { cancel.cancel(); break; }, () = cleanup.cancelled() => break, result = sender.send(output) => { if result.is_err() { break; } } }
                    if failed {
                        break;
                    }
                }
                // Every exit after setup, including a conversion error or a
                // closed output receiver, must settle the original driver.
                // Cancel only its invocation child, retaining a pending error
                // as failed rather than rewriting registry semantic cancel.
                invocation_cancel.cancel();
                while source.next().await.is_some() {}
            });
            streams.attach_background(&producer, background);
            let setup = setup_receiver
                .await
                .map_err(|_| invalid("channel stream producer exited during setup"));
            let setup = setup.and_then(|result| result).and_then(|()| {
                super::owned_resource(handles, &resource, owner, METHOD).map(|_| ())
            });
            if let Err(error) = setup {
                producer.cancel.cancel();
                if let Err(cleanup) = streams
                    .close(
                        handles,
                        &producer.handle,
                        owner,
                        "channels",
                        &request.operation,
                    )
                    .await
                {
                    return Err(cleanup.with_handle(producer.handle));
                }
                return Err(error);
            }
            let _ = cancel_setup_on_drop.disarm();
            Ok(WireValue::Handle(producer.handle))
        }
        "channels.manager.list" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let manager = record.manager.lock().await;
            wire(serde_json::json!({"channels": manager.channel_ids()}))
        }
        "channels.manager.ids" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let manager = record.manager.lock().await;
            wire(serde_json::to_value(manager.channel_ids()).map_err(framework)?)
        }
        "channels.plugin.send" => {
            let resource =
                super::resource_handle_at(&arguments, 0, "a channel manager resource", METHOD)?;
            let channel_id = string_at(&arguments, 1, "a channel id")?;
            let message: echo_sdk_protocol::methods::ChannelOutboundMessageWire =
                serde_json::from_value(
                    arguments.get(2).cloned().ok_or_else(|| {
                        invalid("channels.plugin.send requires an outbound message")
                    })?,
                )
                .map_err(|error| invalid(format!("outbound message is malformed: {error}")))?;
            let record = channel_of(handles, resources, &resource, owner)?;
            let channel = {
                let manager = record.manager.lock().await;
                manager.get_owned(&channel_id)
            }
            .ok_or_else(|| invalid(format!("channel {channel_id} is not registered")))?;
            channel
                .send(
                    crate::core_profile::extension_bridge::channel_outbound_from_wire(message)
                        .map_err(framework)?,
                )
                .await
                .map_err(framework)?;
            wire(serde_json::json!({"sent": true}))
        }
        other => Err(invalid(format!(
            "unknown channels operation {other}; the family surface is closed"
        ))),
    }
}

/// Close every integration resource owned by one session. MCP managers are
/// explicitly awaited so stdio child processes cannot outlive an explicit
/// session close; connection teardown uses the same close authority.
fn integration_ids(resources: &IntegrationResources) -> Vec<String> {
    #[allow(unused_mut)]
    let mut ids = Vec::new();
    macro_rules! collect_ids {
        ($field:ident) => {
            ids.extend(
                resources
                    .$field
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .keys()
                    .cloned(),
            );
        };
    }
    #[cfg(feature = "framework-mcp")]
    {
        collect_ids!(mcp);
        collect_ids!(mcp_clients);
        collect_ids!(mcp_tools);
    }
    #[cfg(feature = "framework-a2a")]
    collect_ids!(a2a);
    #[cfg(feature = "framework-lsp")]
    {
        collect_ids!(lsp);
        collect_ids!(lsp_clients);
    }
    #[cfg(feature = "framework-topology")]
    collect_ids!(topology);
    #[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
    collect_ids!(channels);
    ids
}

pub(crate) async fn close_session_resources(
    resources: &IntegrationResources,
    closed: &[String],
) -> Result<(), String> {
    // Snapshot Arc owners but never remove them before an await. A cancelled
    // or failed close leaves every unsettled resource on its original owner.
    let mut failures = Vec::<String>::new();
    #[cfg(feature = "framework-mcp")]
    {
        let managers = resources
            .mcp
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|(id, _)| closed.contains(id))
            .map(|(id, value)| (id.clone(), value.clone()))
            .collect::<Vec<_>>();
        for (id, record) in managers {
            match record.manager.lock().await.close_all().await {
                Ok(()) => {
                    resources
                        .mcp
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .remove(&id);
                }
                Err(error) => failures.push(error.to_string()),
            }
        }
        let clients = resources
            .mcp_clients
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|(id, _)| closed.contains(id))
            .map(|(id, value)| (id.clone(), value.clone()))
            .collect::<Vec<_>>();
        for (id, client) in clients {
            match client.close().await {
                Ok(()) => {
                    resources
                        .mcp_clients
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .remove(&id);
                }
                Err(error) => failures.push(error.to_string()),
            }
        }
        resources
            .mcp_client_managers
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|id, manager| !closed.contains(id) && !closed.contains(manager));
        resources
            .mcp_tools
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|id, _| !closed.contains(id));
    }
    #[cfg(feature = "framework-lsp")]
    {
        let managers = resources
            .lsp
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|(id, _)| closed.contains(id))
            .map(|(id, value)| (id.clone(), value.clone()))
            .collect::<Vec<_>>();
        for (id, record) in managers {
            record.manager.lock().await.shutdown_all().await;
            resources
                .lsp
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&id);
        }
        // Manager shutdown owns transport settlement. Client handles are
        // aliases to its fenced clients, not independent child owners.
        resources
            .lsp_clients
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|id, _| !closed.contains(id));
    }
    #[cfg(all(feature = "framework-channels", feature = "sdk-extension-bridge"))]
    {
        let managers = resources
            .channels
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|(id, _)| closed.contains(id))
            .map(|(id, value)| (id.clone(), value.clone()))
            .collect::<Vec<_>>();
        for (id, record) in managers {
            let stop = {
                let manager = record.manager.lock().await;
                manager.stop_all_owned()
            };
            match stop.await {
                Ok(()) => {
                    resources
                        .channels
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .remove(&id);
                }
                Err(error) => failures.push(error.to_string()),
            }
        }
    }
    #[cfg(feature = "framework-a2a")]
    resources
        .a2a
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .retain(|id, _| !closed.contains(id));
    #[cfg(feature = "framework-topology")]
    resources
        .topology
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .retain(|id, _| !closed.contains(id));
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; "))
    }
}

#[cfg(all(test, feature = "framework-mcp"))]
mod tests {
    use super::*;

    pub(super) fn owned_registry() -> Result<HandleRegistry, Box<dyn std::error::Error>> {
        let config: crate::SdkHostConfig = serde_json::from_value(serde_json::json!({
            "schema_version":1,"default_agent":{"model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},"agent":{"name":"fixture","enable_tools":true}}
        }))?;
        let prepared = config.prepare()?;
        let handles = HandleRegistry::new(1, 64);
        let (agent, _) = handles
            .register_agent(
                Arc::new(crate::factory::PreparedAgentDefinition::new(
                    prepared.framework,
                    prepared.llm_client,
                    None,
                    true,
                )),
                "fixture".to_string(),
            )
            .map_err(|e| e.message)?;
        handles
            .register_session_with_cwd("session-a".to_string(), agent.id, None)
            .map_err(|e| e.message)?;
        Ok(handles)
    }

    struct CleanupTransport {
        fail_close: std::sync::atomic::AtomicBool,
        hold_initialize: bool,
        entered: tokio::sync::Notify,
        closes: std::sync::atomic::AtomicUsize,
    }

    impl echo_agent::mcp::integration::transport::McpTransport for CleanupTransport {
        fn send(
            &self,
            request: echo_agent::mcp::integration::types::JsonRpcRequest,
        ) -> futures::future::BoxFuture<
            '_,
            echo_agent::error::Result<echo_agent::mcp::integration::types::JsonRpcResponse>,
        > {
            Box::pin(async move {
                self.entered.notify_one();
                if self.hold_initialize {
                    std::future::pending::<()>().await;
                }
                let result = match request.method.as_str() {
                    "initialize" => {
                        serde_json::json!({"protocolVersion":"2024-11-05", "capabilities": {}, "serverInfo":{"name":"cleanup-fixture", "version":"1"}})
                    }
                    "tools/list" => serde_json::json!({"tools": []}),
                    "resources/list" => serde_json::json!({"resources": []}),
                    "prompts/list" => serde_json::json!({"prompts": []}),
                    _ => serde_json::Value::Null,
                };
                Ok(echo_agent::mcp::integration::types::JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(result),
                    error: None,
                })
            })
        }
        fn notify(
            &self,
            _: echo_agent::mcp::integration::types::JsonRpcNotification,
        ) -> futures::future::BoxFuture<'_, echo_agent::error::Result<()>> {
            Box::pin(async { Ok(()) })
        }
        fn close(&self) -> futures::future::BoxFuture<'_, echo_agent::error::Result<()>> {
            Box::pin(async move {
                self.closes
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if self.fail_close.load(std::sync::atomic::Ordering::Acquire) {
                    Err(echo_agent::error::ReactError::Other(
                        "fixture close failed".to_string(),
                    ))
                } else {
                    Ok(())
                }
            })
        }
        fn notification_rx(
            &self,
        ) -> Option<Arc<dyn echo_agent::mcp::integration::types::JsonRpcNotificationReceiver>>
        {
            None
        }
    }

    #[tokio::test]
    async fn unpublished_mcp_cleanup_debt_consumes_resource_quota_before_construction()
    -> Result<(), Box<dyn std::error::Error>> {
        let resources = IntegrationResources::new();
        let handles = owned_registry()?;
        let transport = Arc::new(CleanupTransport {
            fail_close: std::sync::atomic::AtomicBool::new(true),
            hold_initialize: false,
            entered: tokio::sync::Notify::new(),
            closes: std::sync::atomic::AtomicUsize::new(0),
        });
        resources
            .prepare_mcp_client(&handles, "session-a", 1, || {
                echo_agent::mcp::McpClient::from_transport("debt", transport.clone())
                    .map_err(framework)
            })
            .await
            .map_err(|e| e.message)?;
        let constructed = std::sync::atomic::AtomicBool::new(false);
        let refused = resources
            .prepare_mcp_client(&handles, "session-a", 1, || {
                constructed.store(true, std::sync::atomic::Ordering::Release);
                Ok(echo_agent::mcp::McpClient::new(
                    echo_agent::mcp::McpServerConfig::stdio(
                        "should-not-spawn",
                        "no-such-command",
                        Vec::<String>::new(),
                    ),
                ))
            })
            .await;
        assert!(refused.is_err_and(|e| e.code == ExtensionErrorCode::PayloadTooLarge));
        assert!(!constructed.load(std::sync::atomic::Ordering::Acquire));
        assert!(resources.close_all(&handles).await.is_err());
        assert!(
            handles
                .register_facade_resource(1, "fixture", "resource", Some("session-a"), METHOD)
                .is_err()
        );
        transport
            .fail_close
            .store(false, std::sync::atomic::Ordering::Release);
        resources.close_all(&handles).await?;
        assert!(
            handles
                .register_facade_resource(1, "fixture", "resource", Some("session-a"), METHOD)
                .is_ok()
        );
        Ok(())
    }

    #[tokio::test]
    async fn failed_close_retains_host_resource_for_retry() -> Result<(), Box<dyn std::error::Error>>
    {
        let resources = IntegrationResources::new();
        let handles = owned_registry()?;
        let transport = Arc::new(CleanupTransport {
            fail_close: std::sync::atomic::AtomicBool::new(true),
            hold_initialize: false,
            entered: tokio::sync::Notify::new(),
            closes: std::sync::atomic::AtomicUsize::new(0),
        });
        let preparation = echo_agent::mcp::McpClient::from_transport("fixture", transport.clone())?;
        let client = resources
            .prepare_mcp_client(&handles, "session-a", 8, || Ok(preparation))
            .await
            .map_err(|e| e.message)?;
        let issued = register_mcp_client(&handles, &resources, client, "session-a", 8)
            .map_err(|e| e.message)?;
        let ids = [issued.id.clone()];
        assert!(close_session_resources(&resources, &ids).await.is_err());
        assert!(
            resources
                .mcp_clients
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .contains_key(&issued.id)
        );
        transport
            .fail_close
            .store(false, std::sync::atomic::Ordering::Release);
        close_session_resources(&resources, &ids).await?;
        resources.close_all(&handles).await?;
        assert!(
            resources
                .mcp_clients
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_empty()
        );
        assert!(
            resources
                .mcp_cleanup_debt
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_empty()
        );
        Ok(())
    }

    #[tokio::test]
    async fn dropped_preparation_retains_scope_for_connection_cleanup()
    -> Result<(), Box<dyn std::error::Error>> {
        let resources = IntegrationResources::new();
        let handles = owned_registry()?;
        let transport = Arc::new(CleanupTransport {
            fail_close: std::sync::atomic::AtomicBool::new(false),
            hold_initialize: true,
            entered: tokio::sync::Notify::new(),
            closes: std::sync::atomic::AtomicUsize::new(0),
        });
        let preparation = echo_agent::mcp::McpClient::from_transport("fixture", transport.clone())?;
        {
            let opening =
                resources.prepare_mcp_client(&handles, "session-a", 8, || Ok(preparation));
            tokio::pin!(opening);
            tokio::select! {
                _ = transport.entered.notified() => {},
                result = &mut opening => return Err(format!("preparation returned early: {}", result.is_ok()).into()),
            }
        }
        assert_eq!(
            resources
                .mcp_preparations
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .len(),
            1
        );
        resources.close_all(&handles).await?;
        assert!(
            resources
                .mcp_preparations
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_empty()
        );
        assert_eq!(
            transport.closes.load(std::sync::atomic::Ordering::SeqCst),
            1
        );
        Ok(())
    }

    #[test]
    fn exact_mcp_config_preserves_every_transport_field() -> Result<(), Box<dyn std::error::Error>>
    {
        let cwd = tempfile::tempdir()?;
        let cwd_wire = serde_json::to_value(WirePath::Utf8 {
            path: cwd.path().display().to_string(),
        })?;
        let stdio = vec![serde_json::json!({
            "name": "stdio-server",
            "transport": {
                "kind": "stdio",
                "command": "server-bin",
                "args": ["--flag"],
                "env": {"TOKEN": "secret-ref"},
                "cwd": cwd_wire,
            }
        })];
        let parsed = mcp_server_config_at(&stdio, 0).map_err(|error| error.message)?;
        let echo_agent::mcp::TransportConfig::Stdio {
            command,
            args,
            env,
            cwd: parsed_cwd,
        } = parsed.transport
        else {
            return Err("stdio config changed transport".into());
        };
        assert_eq!(command, "server-bin");
        assert_eq!(args, vec!["--flag"]);
        assert_eq!(env, vec![("TOKEN".to_string(), "secret-ref".to_string())]);
        assert_eq!(parsed_cwd.as_deref(), Some(cwd.path()));

        for kind in ["http", "sse"] {
            let config = vec![serde_json::json!({
                "name": format!("{kind}-server"),
                "transport": {
                    "kind": kind,
                    "base_url": "https://example.test/mcp",
                    "headers": {"Authorization": "secret-ref"},
                }
            })];
            let parsed = mcp_server_config_at(&config, 0).map_err(|error| error.message)?;
            let (base_url, headers) = match parsed.transport {
                echo_agent::mcp::TransportConfig::Http { base_url, headers } if kind == "http" => {
                    (base_url, headers)
                }
                echo_agent::mcp::TransportConfig::Sse { base_url, headers } if kind == "sse" => {
                    (base_url, headers)
                }
                _ => return Err(format!("{kind} config changed transport").into()),
            };
            assert_eq!(base_url, "https://example.test/mcp");
            assert_eq!(
                headers.get("Authorization").map(String::as_str),
                Some("secret-ref")
            );
        }
        Ok(())
    }
}

#[cfg(all(
    test,
    feature = "framework-mcp",
    feature = "framework-channels",
    feature = "sdk-extension-bridge"
))]
mod channel_resource_admission_tests {
    use super::*;
    use echo_agent::channels::{
        ChannelCapabilities, ChannelPlugin, MessageHandler, OutboundMessage,
    };
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    struct NativeChannel {
        running: Arc<AtomicBool>,
        starts: Arc<AtomicUsize>,
        stops: Arc<AtomicUsize>,
        capabilities: ChannelCapabilities,
    }

    #[async_trait::async_trait]
    impl ChannelPlugin for NativeChannel {
        fn id(&self) -> &str {
            "native-admission"
        }
        fn capabilities(&self) -> &ChannelCapabilities {
            &self.capabilities
        }
        async fn start(&mut self, _: Arc<dyn MessageHandler>) -> echo_agent::error::Result<()> {
            self.starts.fetch_add(1, Ordering::SeqCst);
            self.running.store(true, Ordering::SeqCst);
            Ok(())
        }
        async fn stop(&mut self) -> echo_agent::error::Result<()> {
            self.stops.fetch_add(1, Ordering::SeqCst);
            self.running.store(false, Ordering::SeqCst);
            Ok(())
        }
        async fn send(&self, _: OutboundMessage) -> echo_agent::error::Result<()> {
            Ok(())
        }
    }

    struct NativeHandler;
    #[async_trait::async_trait]
    impl MessageHandler for NativeHandler {
        async fn handle(
            &self,
            _: echo_agent::channels::InboundMessage,
        ) -> echo_agent::error::Result<OutboundMessage> {
            Err(echo_agent::error::ReactError::Other(
                "no inbound consumer in this regression".to_string(),
            ))
        }
        async fn reply(&self, _: OutboundMessage) -> echo_agent::error::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn captured_channel_start_is_settled_before_resource_close()
    -> Result<(), Box<dyn std::error::Error>> {
        let handles = super::tests::owned_registry()?;
        let runtime =
            super::super::SessionFacadeRuntime::new(&crate::config::SdkProfileLimits::default());
        let (resource, resource_record) = handles
            .register_facade_resource(4, "channels", "channels.manager", Some("session-a"), METHOD)
            .map_err(|error| error.message)?;
        let running = Arc::new(AtomicBool::new(false));
        let starts = Arc::new(AtomicUsize::new(0));
        let stops = Arc::new(AtomicUsize::new(0));
        let mut manager = echo_agent::channels::ChannelManager::new();
        manager.register(Box::new(NativeChannel {
            running: running.clone(),
            starts: starts.clone(),
            stops: stops.clone(),
            capabilities: ChannelCapabilities {
                chat_types: &[],
                supports_media: false,
                supports_threads: false,
            },
        }))?;
        let record = Arc::new(ChannelManagerRecord {
            manager: tokio::sync::Mutex::new(manager),
        });
        runtime
            .integrations
            .channels
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(resource.id.clone(), record.clone());
        let permit = channel_resource_operation(&handles, &resource, "session-a", &resource_record)
            .await
            .map_err(|error| error.message)?;
        let start = {
            let manager = record.manager.lock().await;
            manager.start_all_owned(|_| Arc::new(NativeHandler))
        };
        let mut close = Box::pin(runtime.close_resource(
            &handles,
            "session-a",
            &resource,
            "facade.resource.close",
        ));
        let early_close = futures::poll!(close.as_mut());
        if let std::task::Poll::Ready(result) = early_close {
            assert!(result.map_err(|error| error.message)?);
            assert!(handles.is_closed(&resource));
            let results = start.await;
            for result in results {
                result.result?;
            }
            eprintln!(
                "resource tombstoned before captured start: starts={}, stops={}, running={}",
                starts.load(Ordering::SeqCst),
                stops.load(Ordering::SeqCst),
                running.load(Ordering::SeqCst)
            );
            assert!(
                !running.load(Ordering::SeqCst),
                "captured owned start restarted a plugin after its resource handle was closed"
            );
            return Ok(());
        }
        let results = start.await;
        for result in results {
            result.result?;
        }
        assert!(running.load(Ordering::SeqCst));
        drop(permit);
        assert!(close.await.map_err(|error| error.message)?);
        assert!(handles.is_closed(&resource));
        assert!(!running.load(Ordering::SeqCst));
        assert_eq!(starts.load(Ordering::SeqCst), 1);
        assert_eq!(stops.load(Ordering::SeqCst), 1);
        Ok(())
    }

    #[tokio::test]
    async fn channel_dispatch_pending_start_rejects_early_close_fence_without_restart()
    -> Result<(), Box<dyn std::error::Error>> {
        let handles = super::tests::owned_registry()?;
        let runtime =
            super::super::SessionFacadeRuntime::new(&crate::config::SdkProfileLimits::default());
        let (resource, resource_record) = handles
            .register_facade_resource(4, "channels", "channels.manager", Some("session-a"), METHOD)
            .map_err(|error| error.message)?;
        let running = Arc::new(AtomicBool::new(false));
        let starts = Arc::new(AtomicUsize::new(0));
        let stops = Arc::new(AtomicUsize::new(0));
        let mut manager = echo_agent::channels::ChannelManager::new();
        manager.register(Box::new(NativeChannel {
            running: running.clone(),
            starts: starts.clone(),
            stops: stops.clone(),
            capabilities: ChannelCapabilities {
                chat_types: &[],
                supports_media: false,
                supports_threads: false,
            },
        }))?;
        let record = Arc::new(ChannelManagerRecord {
            manager: tokio::sync::Mutex::new(manager),
        });
        runtime
            .integrations
            .channels
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(resource.id.clone(), record.clone());
        let bridge = Arc::new(
            crate::core_profile::extension_bridge::ExtensionBridge::unbound(Arc::new(
                crate::core_profile::extension_bridge::ExtensionBridgeShared::new(),
            )),
        );
        let handler = WireHandle {
            id: "native-handler".to_string(),
            kind: echo_sdk_protocol::handle::HandleKind::Extension,
            generation: resource.generation.clone(),
        };
        let request = FeatureOperationRequest {
            operation: "channels.manager.start".to_string(),
            signature_digest: String::new(),
            handle: None,
            arguments: vec![
                WireValue::Handle(resource.clone()),
                WireValue::Handle(handler),
            ],
        };
        let manager_guard = record.manager.lock().await;
        let mut start = Box::pin(dispatch_channels(
            &handles,
            &runtime.integrations,
            &runtime.streams,
            &bridge,
            "session-a",
            &request,
            4,
        ));
        assert!(futures::poll!(start.as_mut()).is_pending());
        let mut close = Box::pin(runtime.close_resource(
            &handles,
            "session-a",
            &resource,
            "facade.resource.close",
        ));
        assert!(futures::poll!(close.as_mut()).is_pending());
        assert!(resource_record.closing.load(Ordering::Acquire));
        assert!(!handles.is_closed(&resource));
        drop(manager_guard);
        assert!(
            start
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::ClosedHandle)
        );
        assert!(!running.load(Ordering::SeqCst));
        // Metadata is outside the operation gate, even with a close writer queued.
        let ids_request = FeatureOperationRequest {
            operation: "channels.manager.ids".to_string(),
            signature_digest: String::new(),
            handle: None,
            arguments: vec![WireValue::Handle(resource.clone())],
        };
        let ids = dispatch_channels(
            &handles,
            &runtime.integrations,
            &runtime.streams,
            &bridge,
            "session-a",
            &ids_request,
            4,
        )
        .await;
        assert!(ids.is_err_and(|error| error.code == ExtensionErrorCode::ClosedHandle));
        assert!(close.await.map_err(|error| error.message)?);
        assert!(handles.is_closed(&resource));
        assert!(!running.load(Ordering::SeqCst));
        let late_start = dispatch_channels(
            &handles,
            &runtime.integrations,
            &runtime.streams,
            &bridge,
            "session-a",
            &request,
            4,
        )
        .await;
        assert!(late_start.is_err_and(|error| error.code == ExtensionErrorCode::ClosedHandle));
        assert_eq!(starts.load(Ordering::SeqCst), 0);
        assert_eq!(stops.load(Ordering::SeqCst), 1);
        Ok(())
    }

    #[tokio::test]
    async fn channel_resource_operation_rechecks_tombstone_after_waiting_for_close()
    -> Result<(), Box<dyn std::error::Error>> {
        let handles = super::tests::owned_registry()?;
        let (resource, record) = handles
            .register_facade_resource(4, "channels", "channels.manager", Some("session-a"), METHOD)
            .map_err(|error| error.message)?;
        let close_permit = record.lifecycle.write().await;
        let mut admitted = Box::pin(channel_resource_operation(
            &handles,
            &resource,
            "session-a",
            &record,
        ));
        assert!(futures::poll!(admitted.as_mut()).is_pending());
        record.closing.store(true, Ordering::Release);
        assert!(
            handles
                .close_facade_resource(&resource, METHOD)
                .map_err(|error| error.message)?
        );
        drop(close_permit);
        assert!(
            admitted
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::ClosedHandle)
        );
        Ok(())
    }

    #[tokio::test]
    async fn channel_resource_close_cancels_original_call_before_waiting_for_permit()
    -> Result<(), Box<dyn std::error::Error>> {
        let handles = super::tests::owned_registry()?;
        let runtime =
            super::super::SessionFacadeRuntime::new(&crate::config::SdkProfileLimits::default());
        let (resource, record) = handles
            .register_facade_resource(4, "channels", "channels.manager", Some("session-a"), METHOD)
            .map_err(|error| error.message)?;
        let permit = channel_resource_operation(&handles, &resource, "session-a", &record)
            .await
            .map_err(|error| error.message)?;
        let first = record.cancel_operations.child_token();
        let independent = record.cancel_operations.child_token();
        first.cancel();
        assert!(first.is_cancelled());
        assert!(!independent.is_cancelled());
        assert!(!record.cancel_operations.is_cancelled());
        let mut close = Box::pin(runtime.close_resource(
            &handles,
            "session-a",
            &resource,
            "facade.resource.close",
        ));
        assert!(futures::poll!(close.as_mut()).is_pending());
        assert!(record.closing.load(Ordering::Acquire));
        assert!(independent.is_cancelled());
        assert!(!handles.is_closed(&resource));
        drop(permit);
        assert!(close.await.map_err(|error| error.message)?);
        assert!(handles.is_closed(&resource));
        Ok(())
    }

    #[tokio::test]
    async fn channel_batch_start_binding_mismatch_rejects_every_plugin_before_effects()
    -> Result<(), Box<dyn std::error::Error>> {
        let handles = super::tests::owned_registry()?;
        let runtime =
            super::super::SessionFacadeRuntime::new(&crate::config::SdkProfileLimits::default());
        let (resource, address) = handles
            .register_facade_resource(4, "channels", "channels.manager", Some("session-a"), METHOD)
            .map_err(|error| error.message)?;
        let running = Arc::new(AtomicBool::new(false));
        let starts = Arc::new(AtomicUsize::new(0));
        let stops = Arc::new(AtomicUsize::new(0));
        let mut manager = echo_agent::channels::ChannelManager::new();
        manager.register(Box::new(NativeChannel {
            running: running.clone(),
            starts: starts.clone(),
            stops: stops.clone(),
            capabilities: ChannelCapabilities {
                chat_types: &[],
                supports_media: false,
                supports_threads: false,
            },
        }))?;
        runtime
            .integrations
            .channels
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(
                resource.id.clone(),
                Arc::new(ChannelManagerRecord {
                    manager: tokio::sync::Mutex::new(manager),
                }),
            );
        let handler = WireHandle {
            id: "registered-original-handler".to_string(),
            kind: echo_sdk_protocol::handle::HandleKind::Extension,
            generation: resource.generation.clone(),
        };
        address
            .channel_bindings
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .push(super::super::super::handles::ChannelExtensionBinding {
                channel_id: "native-admission".to_string(),
                plugin: WireHandle {
                    id: "original-plugin".to_string(),
                    kind: echo_sdk_protocol::handle::HandleKind::Extension,
                    generation: resource.generation.clone(),
                },
                handler: handler.clone(),
            });
        let bridge = Arc::new(
            crate::core_profile::extension_bridge::ExtensionBridge::unbound(Arc::new(
                crate::core_profile::extension_bridge::ExtensionBridgeShared::new(),
            )),
        );
        for supplied in [
            WireHandle {
                id: "other-handler".to_string(),
                ..handler.clone()
            },
            WireHandle {
                generation: WireU64::from_u64(2),
                ..handler.clone()
            },
        ] {
            let request = FeatureOperationRequest {
                operation: "channels.manager.start".to_string(),
                signature_digest: String::new(),
                handle: None,
                arguments: vec![
                    WireValue::Handle(resource.clone()),
                    WireValue::Handle(supplied),
                ],
            };
            assert!(
                dispatch_channels(
                    &handles,
                    &runtime.integrations,
                    &runtime.streams,
                    &bridge,
                    "session-a",
                    &request,
                    4
                )
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::InvalidValue)
            );
            assert_eq!(starts.load(Ordering::SeqCst), 0);
            assert!(!running.load(Ordering::SeqCst));
        }
        runtime
            .close_resource(&handles, "session-a", &resource, "facade.resource.close")
            .await
            .map_err(|error| error.message)?;
        Ok(())
    }
}
