//! Memory family adapter (plan 07, todo 4).
//!
//! `_echo_agent/memory/op` routes the closed store-operation set onto the
//! Session Agent's own [`echo_agent::memory::Store`] authority — the same
//! instance the in-conversation memory tools use. Operations are exact
//! identities (`memory.store.put`, …), never wildcards; results keep the
//! framework's shapes and errors verbatim, bounded by the negotiated page
//! limit.

use echo_sdk_protocol::error::{EchoSdkError, ExtensionErrorCode, Retryability};
use echo_sdk_protocol::methods::FeatureOperationRequest;
use echo_sdk_protocol::scalar::WireValue;
use std::collections::HashMap;
use std::sync::Arc;

use super::super::wire;
use super::owned_resource;
use super::source_operations::MemoryStoreAuthority;
use crate::core_profile::handles::HandleRegistry;
use crate::factory::SessionAuthorityServices;

const METHOD: &str = "_echo_agent/memory/op";

fn invalid(message: impl Into<String>) -> EchoSdkError {
    wire::sdk_error(
        ExtensionErrorCode::InvalidValue,
        message,
        Retryability::Never,
        METHOD,
    )
}

fn json_of(value: &WireValue, position: usize) -> Result<serde_json::Value, EchoSdkError> {
    super::family_argument_json(value).map_err(|error| {
        invalid(format!(
            "memory argument {position} is not a lossless wire value: {error}"
        ))
    })
}

fn namespace_of(value: &serde_json::Value) -> Result<Vec<String>, EchoSdkError> {
    let items = value
        .as_array()
        .filter(|items| !items.is_empty())
        .ok_or_else(|| invalid("memory namespace must be a non-empty string array"))?;
    items
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| invalid("every memory namespace component must be a string"))
        })
        .collect()
}

fn limit_of(value: Option<&serde_json::Value>, page_limit: usize) -> Result<usize, EchoSdkError> {
    match value {
        None | Some(serde_json::Value::Null) => Ok(page_limit.min(64)),
        Some(value) => {
            let limit = value
                .as_u64()
                .ok_or_else(|| invalid("memory limit must be an unsigned integer"))?;
            Ok(usize::try_from(limit).unwrap_or(usize::MAX).min(page_limit))
        }
    }
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
        .ok_or_else(|| {
            invalid(format!(
                "memory operation requires {what} at argument {position}"
            ))
        })
}

fn item_to_json(item: &echo_agent::memory::StoreItem) -> serde_json::Value {
    serde_json::json!({
        "namespace": item.namespace,
        "key": item.key,
        "value": item.value,
        "created_at": item.created_at,
        "updated_at": item.updated_at,
        "score": item.score,
        "importance": item.importance,
        "last_accessed": item.last_accessed,
        "expires_at": item.expires_at,
    })
}

async fn store_for_request<'a>(
    authorities: &Arc<SessionAuthorityServices>,
    resource_stores: &std::sync::Mutex<HashMap<String, Arc<MemoryStoreAuthority>>>,
    handles: &HandleRegistry,
    owner: &str,
    request: &'a FeatureOperationRequest,
) -> Result<(Arc<dyn echo_agent::memory::Store>, usize, &'a str), EchoSdkError> {
    if !request.operation.starts_with("memory.resource.") {
        // Resolve the live Agent authority: accepted set_store operations can
        // replace the factory-time store without creating a second authority.
        let store = authorities
            .agent_handle
            .read(|agent| agent.store().cloned())
            .await
            .ok_or_else(|| invalid("session has no memory store"))?;
        return Ok((store, 0, request.operation.as_str()));
    }
    let resource = match request.arguments.first() {
        Some(WireValue::Handle(handle)) => handle,
        _ => return Err(invalid("memory.resource operation requires a Store handle")),
    };
    let record = owned_resource(handles, resource, owner, METHOD)?;
    if record.family != "memory" || record.resource_type != "memory.store" {
        return Err(invalid("memory resource handle is not a Store"));
    }
    let store = resource_stores
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&resource.id)
        .cloned()
        .ok_or_else(|| invalid("memory Store resource is closed"))?
        .store();
    let operation = match request.operation.as_str() {
        "memory.resource.put" => "memory.store.put",
        "memory.resource.get" => "memory.store.get",
        "memory.resource.search" => "memory.store.search",
        "memory.resource.search_with" => "memory.store.search_with",
        "memory.resource.delete" => "memory.store.delete",
        "memory.resource.list_namespaces" => "memory.store.list_namespaces",
        "memory.resource.list" => "memory.store.list",
        "memory.resource.prune_expired" => "memory.store.prune_expired",
        "memory.resource.dedup_by_content" => "memory.store.dedup_by_content",
        _ => return Err(invalid("unknown memory resource operation")),
    };
    Ok((store, 1, operation))
}

/// Dispatch one memory family operation. Returns the typed wire result.
pub(crate) async fn dispatch(
    authorities: &Arc<SessionAuthorityServices>,
    resource_stores: &std::sync::Mutex<HashMap<String, Arc<MemoryStoreAuthority>>>,
    handles: &HandleRegistry,
    owner: &str,
    request: &FeatureOperationRequest,
    page_limit: usize,
) -> Result<WireValue, EchoSdkError> {
    let (store, offset, operation) =
        store_for_request(authorities, resource_stores, handles, owner, request).await?;
    let arguments: Vec<serde_json::Value> = request
        .arguments
        .iter()
        .enumerate()
        .map(|(position, value)| {
            if position < offset {
                Ok(serde_json::Value::Null)
            } else {
                json_of(value, position)
            }
        })
        .collect::<Result<_, _>>()?;
    if operation == "memory.store.list_namespaces" {
        let prefix = match arguments.get(offset) {
            None | Some(serde_json::Value::Null) => None,
            Some(value) => Some(namespace_of(value)?),
        };
        let prefix_refs = prefix
            .as_ref()
            .map(|parts| parts.iter().map(String::as_str).collect::<Vec<_>>());
        let namespaces = store
            .list_namespaces(prefix_refs.as_deref())
            .await
            .map_err(framework)?;
        let truncated = namespaces.len() > page_limit;
        return WireValue::from_json(serde_json::json!({
            "namespaces": namespaces.into_iter().take(page_limit).collect::<Vec<_>>(),
            "truncated": truncated,
        }))
        .map_err(|error| invalid(error.to_string()));
    }
    let namespace_owned = arguments
        .get(offset)
        .ok_or_else(|| invalid("memory operations require a namespace argument"))?;
    let namespace = namespace_of(namespace_owned)?;
    let namespace_refs: Vec<&str> = namespace.iter().map(String::as_str).collect();
    match operation {
        "memory.store.put" => {
            let key = string_at(&arguments, offset.saturating_add(1), "a key")?;
            let value = arguments
                .get(offset.saturating_add(2))
                .cloned()
                .ok_or_else(|| invalid("memory.store.put requires a value"))?;
            store
                .put(&namespace_refs, &key, value)
                .await
                .map_err(framework)?;
            WireValue::from_json(serde_json::json!({"ok": true}))
                .map_err(|error| invalid(error.to_string()))
        }
        "memory.store.get" => {
            let key = string_at(&arguments, offset.saturating_add(1), "a key")?;
            let found = store.get(&namespace_refs, &key).await.map_err(framework)?;
            let value = found
                .as_ref()
                .map(item_to_json)
                .unwrap_or(serde_json::Value::Null);
            WireValue::from_json(value).map_err(|error| invalid(error.to_string()))
        }
        "memory.store.delete" => {
            let key = string_at(&arguments, offset.saturating_add(1), "a key")?;
            let deleted = store
                .delete(&namespace_refs, &key)
                .await
                .map_err(framework)?;
            WireValue::from_json(serde_json::json!({"deleted": deleted}))
                .map_err(|error| invalid(error.to_string()))
        }
        "memory.store.list" => {
            let items = store.list(&namespace_refs).await.map_err(framework)?;
            let truncated = items.len() > page_limit;
            let page: Vec<serde_json::Value> =
                items.iter().take(page_limit).map(item_to_json).collect();
            WireValue::from_json(serde_json::json!({
                "items": page,
                "truncated": truncated,
            }))
            .map_err(|error| invalid(error.to_string()))
        }
        "memory.store.search" => {
            let query = string_at(&arguments, offset.saturating_add(1), "a query")?;
            let limit = limit_of(arguments.get(offset.saturating_add(2)), page_limit)?;
            let items = store
                .search_with(
                    &namespace_refs,
                    echo_agent::memory::SearchQuery {
                        text: query.as_str(),
                        limit,
                        mode: echo_agent::memory::SearchMode::Keyword,
                    },
                )
                .await
                .map_err(framework)?;
            let page: Vec<serde_json::Value> = items.iter().take(limit).map(item_to_json).collect();
            WireValue::from_json(serde_json::json!({"items": page}))
                .map_err(|error| invalid(error.to_string()))
        }
        "memory.store.search_with" => {
            let query = arguments
                .get(offset.saturating_add(1))
                .and_then(serde_json::Value::as_object)
                .ok_or_else(|| invalid("memory.store.search_with requires a query object"))?;
            let text = query
                .get("text")
                .and_then(serde_json::Value::as_str)
                .filter(|text| !text.trim().is_empty())
                .ok_or_else(|| invalid("memory search query requires non-empty text"))?;
            let limit = limit_of(query.get("limit"), page_limit)?;
            let mode = query
                .get("mode")
                .cloned()
                .map(serde_json::from_value::<echo_agent::memory::SearchMode>)
                .transpose()
                .map_err(|error| invalid(format!("memory search mode is malformed: {error}")))?
                .unwrap_or(echo_agent::memory::SearchMode::Keyword);
            let items = store
                .search_with(
                    &namespace_refs,
                    echo_agent::memory::SearchQuery { text, limit, mode },
                )
                .await
                .map_err(framework)?;
            let page: Vec<serde_json::Value> = items.iter().take(limit).map(item_to_json).collect();
            WireValue::from_json(serde_json::json!({"items": page}))
                .map_err(|error| invalid(error.to_string()))
        }
        "memory.store.prune_expired" => {
            let removed = store
                .prune_expired(&namespace_refs)
                .await
                .map_err(framework)?;
            Ok(WireValue::U64(
                echo_sdk_protocol::scalar::WireU64::from_u64(removed),
            ))
        }
        "memory.store.dedup_by_content" => {
            let removed = store
                .dedup_by_content(&namespace_refs)
                .await
                .map_err(framework)?;
            Ok(WireValue::U64(
                echo_sdk_protocol::scalar::WireU64::from_u64(removed),
            ))
        }
        other => Err(invalid(format!(
            "unknown memory operation {other}; the family surface is closed"
        ))),
    }
}

fn framework(error: echo_agent::error::ReactError) -> EchoSdkError {
    wire::framework_error(&error, METHOD)
}

#[cfg(test)]
mod tests {
    use super::*;
    use echo_agent::memory::Store as _;
    use serde_json::json;

    fn request(
        operation: &str,
        values: Vec<serde_json::Value>,
    ) -> Result<FeatureOperationRequest, Box<dyn std::error::Error>> {
        Ok(FeatureOperationRequest {
            operation: operation.to_string(),
            signature_digest: String::new(),
            handle: None,
            arguments: values
                .into_iter()
                .map(WireValue::from_json)
                .collect::<Result<_, _>>()?,
        })
    }

    #[tokio::test]
    async fn malformed_namespace_never_writes_and_replacement_store_is_live_authority()
    -> Result<(), Box<dyn std::error::Error>> {
        let first = Arc::new(echo_agent::memory::InMemoryStore::new());
        let second = Arc::new(echo_agent::memory::InMemoryStore::new());
        let mut agent = echo_agent::agent::ReactAgent::new(echo_agent::agent::AgentConfig::new(
            "unused", "test", "test",
        ));
        agent.set_store(first.clone());
        let handle = echo_agent::agent::AgentHandle::new(agent);
        let authorities = Arc::new(
            handle
                .read(|agent| {
                    SessionAuthorityServices::of(handle.clone(), agent, std::path::PathBuf::new())
                })
                .await,
        );
        let stores = std::sync::Mutex::new(HashMap::new());
        let handles = HandleRegistry::new(1, 8);
        let malformed = request(
            "memory.store.put",
            vec![json!(["合法", 7]), json!("key"), json!("value")],
        )?;
        assert!(
            dispatch(&authorities, &stores, &handles, "owner", &malformed, 8)
                .await
                .is_err_and(|error| error.code == ExtensionErrorCode::InvalidValue)
        );
        assert!(first.get(&["合法"], "key").await?.is_none());
        handle.write(|agent| agent.set_store(second.clone())).await;
        let put = request(
            "memory.store.put",
            vec![json!(["中文🌳"]), json!("key"), json!("B")],
        )?;
        dispatch(&authorities, &stores, &handles, "owner", &put, 8)
            .await
            .map_err(|error| std::io::Error::other(error.message))?;
        assert!(first.get(&["中文🌳"], "key").await?.is_none());
        assert!(second.get(&["中文🌳"], "key").await?.is_some());
        let get = request("memory.store.get", vec![json!(["中文🌳"]), json!("key")])?;
        let result = dispatch(&authorities, &stores, &handles, "owner", &get, 8)
            .await
            .map_err(|error| std::io::Error::other(error.message))?
            .into_json()?;
        assert_eq!(result.get("value"), Some(&json!("B")));
        Ok(())
    }

    #[test]
    fn explicit_invalid_limits_are_not_defaulted() {
        for value in [json!("5"), json!(-1), json!(2.5), json!(true)] {
            assert!(limit_of(Some(&value), 8).is_err());
        }
        assert_eq!(limit_of(None, 8).ok(), Some(8));
        assert_eq!(limit_of(Some(&json!(3)), 8).ok(), Some(3));
    }
}

#[cfg(test)]
mod failure_fidelity_regressions {
    use super::*;
    use echo_agent::memory::Store as _;

    #[tokio::test]
    async fn unsupported_store_operation_keeps_memory_cause_without_mutation()
    -> Result<(), Box<dyn std::error::Error>> {
        let store = Arc::new(echo_agent::memory::InMemoryStore::new());
        store
            .put(&["中文🙂"], "before", serde_json::json!("preserved"))
            .await?;
        let mut agent = echo_agent::agent::ReactAgent::new(echo_agent::agent::AgentConfig::new(
            "unused", "test", "test",
        ));
        agent.set_store(store.clone());
        let handle = echo_agent::agent::AgentHandle::new(agent);
        let authorities = Arc::new(
            handle
                .read(|agent| {
                    SessionAuthorityServices::of(handle.clone(), agent, std::path::PathBuf::new())
                })
                .await,
        );
        let request = FeatureOperationRequest {
            operation: "memory.store.search_with".to_string(),
            signature_digest: String::new(),
            handle: None,
            arguments: vec![
                WireValue::from_json(serde_json::json!(["中文🙂"]))?,
                WireValue::from_json(
                    serde_json::json!({"text":"query", "mode":"Semantic", "limit":1}),
                )?,
            ],
        };
        let error = match dispatch(
            &authorities,
            &std::sync::Mutex::new(HashMap::new()),
            &HandleRegistry::new(1, 8),
            "owner",
            &request,
            8,
        )
        .await
        {
            Ok(value) => {
                return Err(format!(
                    "unsupported semantic Store search unexpectedly returned {value:?}"
                )
                .into());
            }
            Err(error) => error,
        };
        assert_eq!(error.code, ExtensionErrorCode::FrameworkError);
        assert_eq!(error.retryable, Retryability::Never);
        assert_eq!(error.operation.as_deref(), Some(METHOD));
        let fields = error
            .details
            .and_then(|details| details.fields)
            .ok_or("Store failure lost its canonical cause")?;
        for (key, expected) in [
            ("framework_failure_category", "memory"),
            ("framework_terminal_kind", "failed"),
            ("framework_failure_code", "memory"),
            ("framework_retryable", "false"),
        ] {
            assert_eq!(
                fields
                    .iter()
                    .find(|field| field.key == key)
                    .map(|field| field.value.as_str()),
                Some(expected)
            );
        }
        let before = store
            .get(&["中文🙂"], "before")
            .await?
            .ok_or("Store failure deleted the existing record")?;
        assert_eq!(before.value, serde_json::json!("preserved"));
        assert_eq!(store.list(&["中文🙂"]).await?.len(), 1);
        Ok(())
    }
}
