//! Real official ACP calls using the same deterministic scenarios and artifact
//! assertions exercised independently by each language SDK. The fixture owns
//! test input/HTTP only; framework tools own every result and local side effect.

#![cfg(all(
    feature = "sdk-facade-adapters",
    feature = "framework-data",
    feature = "framework-chart",
    feature = "framework-database",
    feature = "framework-git",
    feature = "framework-media",
    feature = "framework-research"
))]

use agent_client_protocol::schema::{ProtocolVersion, v1};
use agent_client_protocol::{ByteStreams, Client, UntypedMessage};
use echo_sdk_protocol::capability::{EchoAgentClientHello, ExtensionCapability};
use echo_sdk_protocol::methods::{
    AgentConfigWire, AgentCreateRequest, FeatureOperationRequest, FeatureOperationResponse,
    SessionCloseRequest, SessionCreateRequest,
};
use echo_sdk_protocol::scalar::{WirePath, WireValue};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};

type TestError = Box<dyn std::error::Error>;

#[tokio::test]
async fn six_tool_families_preserve_exact_results_failures_and_actual_local_effects()
-> Result<(), TestError> {
    let directory = tempfile::tempdir()?;
    let work = directory.path().to_path_buf();
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../sdks/shared/testing/tool_behavior_fixture.py");
    let mut fixture = tokio::process::Command::new("python3")
        .arg(&fixture_path)
        .arg("--work")
        .arg(&work)
        .kill_on_drop(true)
        .spawn()?;
    tokio::time::timeout(Duration::from_secs(10), async {
        while !work.join("ready").exists() {
            if fixture.try_wait()?.is_some() {
                return Err(std::io::Error::other(
                    "tool fixture exited before preparation",
                ));
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, std::io::Error>(())
    })
    .await??;
    let scenario: Value = serde_json::from_slice(&std::fs::read(work.join("scenario.json"))?)?;
    let digests: Value =
        serde_json::from_str(include_str!("../../sdks/shared/contract-digests.json"))?;
    let catalog: Value = serde_json::from_str(include_str!(
        "../../sdks/shared/facade-operation-catalog.json"
    ))?;
    let hello = EchoAgentClientHello {
        extension_protocol_version: echo_sdk_protocol::EXTENSION_PROTOCOL_VERSION,
        contract_digest: digests
            .get("contract_digest")
            .and_then(Value::as_str)
            .ok_or("missing contract digest")?
            .to_string(),
        source_contract_digest: digests
            .get("source_contract_digest")
            .and_then(Value::as_str)
            .ok_or("missing source digest")?
            .to_string(),
        required_features: vec![
            "chart".to_string(),
            "data".to_string(),
            "database".to_string(),
            "git".to_string(),
            "media".to_string(),
            "research".to_string(),
        ],
        required_capabilities: vec![
            ExtensionCapability::AgentLifecycle,
            ExtensionCapability::SessionHandles,
        ],
    };
    // The normal Cargo test chain compiles the current candidate Host, while
    // language gates independently exercise their configured immutable binary.
    let mut host = tokio::process::Command::new(env!("CARGO_BIN_EXE_echo-agent-sdk-host"))
        .arg("--config")
        .arg(work.join("host.json"))
        .env("TMPDIR", work.join("artifact-temp"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?;
    let input = host.stdin.take().ok_or("Host stdin absent")?;
    let output = host.stdout.take().ok_or("Host stdout absent")?;
    let transport = ByteStreams::new(input.compat_write(), output.compat());
    let results = Client.builder().connect_with(transport, async move |connection| {
        let failure = |error: String| agent_client_protocol::Error::internal_error().data(error);
        let mut request = v1::InitializeRequest::new(ProtocolVersion::V1);
        let mut meta = v1::Meta::new();
        meta.insert("echo_agent".to_string(), serde_json::to_value(hello).map_err(|error| failure(error.to_string()))?);
        request.client_capabilities.meta = Some(meta);
        connection.send_request(request).block_task().await?;
        let agent = connection.send_request(AgentCreateRequest {config: AgentConfigWire::HostDefault, idempotency_id: None}).block_task().await?.agent;
        let mut sessions = HashMap::new();
        for name in ["a", "b"] {
            let cwd = scenario.get("sessions").and_then(|sessions| sessions.get(name)).and_then(Value::as_str)
                .ok_or_else(|| failure("scenario Session cwd absent".to_string()))?;
            let session = connection.send_request(SessionCreateRequest {
                agent: agent.clone(), working_dir: Some(WirePath::Utf8 {path: cwd.to_string()}), session_id: None, idempotency_id: None,
            }).block_task().await?.session;
            sessions.insert(name.to_string(), session);
        }
        let mut rows = Vec::new();
        let steps = scenario.get("steps").and_then(Value::as_array).ok_or_else(|| failure("scenario steps absent".to_string()))?;
        for step in steps {
            let id = step.get("id").and_then(Value::as_str).ok_or_else(|| failure("scenario id absent".to_string()))?;
            let name = step.get("session").and_then(Value::as_str).ok_or_else(|| failure("scenario Session absent".to_string()))?;
            let session = sessions.get(name).cloned().ok_or_else(|| failure("Session lookup failed".to_string()))?;
            if step.get("action").and_then(Value::as_str) == Some("close") {
                connection.send_request(SessionCloseRequest {session}).block_task().await?;
                rows.push(json!({"id": id, "outcome":"closed"}));
                continue;
            }
            let family = step.get("family").and_then(Value::as_str).ok_or_else(|| failure("scenario family absent".to_string()))?;
            let operation = step.get("operation").and_then(Value::as_str).ok_or_else(|| failure("scenario operation absent".to_string()))?;
            let method = format!("_echo_agent/{family}/op");
            let signature = catalog.get("routes").and_then(Value::as_array).into_iter().flatten()
                .find(|route| route.get("method").and_then(Value::as_str) == Some(method.as_str()) && route.get("operation").is_some_and(Value::is_null))
                .and_then(|route| route.get("operation_signatures")).and_then(Value::as_array).into_iter().flatten()
                .find(|entry|entry.get("operation").and_then(Value::as_str)==Some(operation))
                .and_then(|entry|entry.get("signature_digests")).and_then(Value::as_array).and_then(|digests|digests.first()).and_then(Value::as_str)
                .ok_or_else(||failure(format!("accepted signature absent: {family}/{operation}")))?;
            let arguments = step.get("arguments").and_then(Value::as_array).ok_or_else(||failure("scenario arguments absent".to_string()))?
                .iter().cloned().map(WireValue::from_json).collect::<Result<Vec<_>,_>>().map_err(|error|failure(error.to_string()))?;
            let request = FeatureOperationRequest {operation:operation.to_string(), signature_digest: signature.to_string(), handle: Some(session), arguments};
            let request = UntypedMessage::new(&method,serde_json::to_value(request).map_err(|error|failure(error.to_string()))?)?;
            match connection.send_request(request).block_task().await {
                Ok(result) => {
                    let response:FeatureOperationResponse=serde_json::from_value(result).map_err(|error|failure(error.to_string()))?;
                    let value=response.value.into_json().map_err(|error|failure(error.to_string()))?;
                    rows.push(json!({"id":id,"outcome":"result","value":value}));
                }
                Err(error) => {
                    let typed=echo_sdk_protocol::error::EchoSdkError::from_jsonrpc_data(error.data.as_ref())
                        .map_err(|message|failure(message.to_string()))?;
                    rows.push(json!({"id":id,"outcome":"error","code":typed.code,"message":typed.message,"operation":typed.operation,"retryable":typed.retryable,"details":typed.details}));
                }
            }
        }
        let other=sessions.get("b").cloned().ok_or_else(||failure("remaining Session absent".to_string()))?;
        connection.send_request(SessionCloseRequest {session:other}).block_task().await?;
        connection.send_request(echo_sdk_protocol::methods::AgentCloseRequest {agent}).block_task().await?;
        Ok(rows)
    }).await?;
    let status = tokio::time::timeout(Duration::from_secs(10), host.wait()).await??;
    assert!(status.success());
    std::fs::write(work.join("results.json"), serde_json::to_vec(&results)?)?;
    let verification = tokio::process::Command::new("python3")
        .arg(&fixture_path)
        .arg("--work")
        .arg(&work)
        .arg("--verify")
        .arg(work.join("results.json"))
        .output()
        .await?;
    assert!(
        verification.status.success(),
        "{}",
        String::from_utf8_lossy(&verification.stderr)
    );
    let evidence: Value = serde_json::from_slice(&verification.stdout)?;
    assert_eq!(evidence.get("capability_families"), Some(&json!(6)));
    assert_eq!(evidence.get("native_calls"), Some(&json!(29)));
    assert_eq!(evidence.get("failure_paths"), Some(&json!(10)));
    println!("Tool behavior verified: {}", evidence);
    fixture.kill().await?;
    Ok(())
}
