//! Extension contract tests: fixtures, catalog and schema invariants.
//!
//! These mirror the golden fixtures exported to
//! `contracts/sdk/fixtures/extension/v1/`: valid samples must round-trip
//! losslessly through the DTO they target, invalid samples must be rejected
//! by serde or the DTO's validation. The catalog and schema boundary rules
//! are enforced mechanically as well, so the exported contract can never
//! drift from the code that defines it.

use echo_sdk_protocol::capability::EchoAgentCapability;
use echo_sdk_protocol::catalog::{self, Direction, METHOD_CATALOG, MethodDescriptor};
use echo_sdk_protocol::error::EchoSdkError;
use echo_sdk_protocol::event::{GapNotification, ReplayRequest, ReplayResponse, WireEventEnvelope};
use echo_sdk_protocol::handle::WireHandle;
use echo_sdk_protocol::methods::{
    AgentComponentCapabilitiesWire, ExtensionInvokeOutcome, ExtensionStreamEvent, ToolRiskLevelWire,
};
use echo_sdk_protocol::scalar::{WireBytes, WirePath, WireU64, WireValue};
use echo_sdk_protocol::schema::{
    self, FixtureKind, build_extension_schema_doc, build_extension_validator,
    validate_schema_boundaries,
};

fn fixtures_of(target: &str) -> Vec<schema::Fixture> {
    schema::all_fixtures()
        .into_iter()
        .filter(|f| f.target == target)
        .collect()
}

fn parse_fixture<T: serde::de::DeserializeOwned>(fixture: &schema::Fixture) -> Result<T, String> {
    serde_json::from_value(fixture.payload.clone())
        .map_err(|error| format!("fixture {} rejected: {error}", fixture.name))
}

#[test]
fn valid_scalar_fixtures_round_trip_losslessly() -> Result<(), String> {
    for fixture in fixtures_of("WireU64")
        .into_iter()
        .filter(|f| f.kind == FixtureKind::Valid)
    {
        let value: WireU64 = parse_fixture(&fixture)?;
        let back = serde_json::to_value(&value).unwrap_or(serde_json::Value::Null);
        assert_eq!(
            back, fixture.payload,
            "fixture {} is not lossless",
            fixture.name
        );
    }
    Ok(())
}

#[test]
fn invalid_scalar_fixtures_are_rejected() {
    for fixture in fixtures_of("WireU64")
        .into_iter()
        .filter(|f| f.kind == FixtureKind::Invalid)
    {
        let parsed: Result<WireU64, _> = serde_json::from_value(fixture.payload.clone());
        assert!(parsed.is_err(), "fixture {} must be rejected", fixture.name);
    }
}

#[test]
fn every_invalid_fixture_declares_a_stable_error_code() {
    let allowed = ["invalid_value", "invalid_request"];
    for fixture in schema::all_fixtures()
        .into_iter()
        .filter(|fixture| fixture.kind == FixtureKind::Invalid)
    {
        let code = fixture.expect_error.as_deref().unwrap_or_default();
        assert!(
            allowed.contains(&code),
            "fixture {} has invalid error code",
            fixture.name
        );
    }
}

#[test]
fn path_fixtures_round_trip_across_encodings() -> Result<(), String> {
    for fixture in fixtures_of("WirePath") {
        let parsed: Result<WirePath, _> = serde_json::from_value(fixture.payload.clone());
        match (fixture.kind, parsed) {
            (FixtureKind::Valid, Ok(path)) => {
                assert!(path.validate().is_ok(), "fixture {} invalid", fixture.name);
                let back = serde_json::to_value(&path).unwrap_or(serde_json::Value::Null);
                assert_eq!(
                    back, fixture.payload,
                    "fixture {} is not lossless",
                    fixture.name
                );
            }
            (FixtureKind::Invalid, Ok(path)) => {
                assert!(
                    path.validate().is_err(),
                    "fixture {} must fail validation",
                    fixture.name
                );
            }
            (FixtureKind::Invalid, Err(_)) => {}
            (FixtureKind::Valid, Err(error)) => {
                return Err(format!("fixture {} rejected: {error}", fixture.name));
            }
        }
    }
    Ok(())
}

#[test]
fn binary_and_unknown_fixtures_enforce_bounds() {
    for fixture in fixtures_of("WireBytes") {
        let value: WireBytes =
            serde_json::from_value(fixture.payload.clone()).unwrap_or(WireBytes {
                base64: String::new(),
            });
        assert_eq!(value.validate().is_ok(), fixture.kind == FixtureKind::Valid);
    }
    for fixture in fixtures_of("WireValue") {
        let value: Result<WireValue, _> = serde_json::from_value(fixture.payload.clone());
        let valid = value.as_ref().is_ok_and(|value| value.validate().is_ok());
        assert_eq!(valid, fixture.kind == FixtureKind::Valid);
    }
}

#[test]
fn extension_outcome_and_stream_are_tagged() {
    for fixture in fixtures_of("ExtensionInvokeOutcome") {
        let parsed: Result<ExtensionInvokeOutcome, _> =
            serde_json::from_value(fixture.payload.clone());
        let valid = parsed
            .as_ref()
            .is_ok_and(|outcome| outcome.validate().is_ok());
        assert_eq!(valid, fixture.kind == FixtureKind::Valid);
    }
    for fixture in fixtures_of("ExtensionStreamEvent") {
        let parsed: Result<ExtensionStreamEvent, _> =
            serde_json::from_value(fixture.payload.clone());
        let valid = parsed.as_ref().is_ok_and(|event| event.validate().is_ok());
        assert_eq!(valid, fixture.kind == FixtureKind::Valid);
        if let Ok(event) = parsed
            && fixture.kind == FixtureKind::Valid
            && let Ok(value) = serde_json::to_value(&event)
        {
            assert_eq!(
                value, fixture.payload,
                "fixture {} is not lossless",
                fixture.name
            );
        }
    }
}

#[test]
fn agent_component_stream_rejects_inner_terminal_confusion() {
    let stream = serde_json::json!({
        "id": "stream-1",
        "generation": "1",
        "kind": "stream"
    });
    let terminal_as_chunk = serde_json::json!({
        "event": "chunk",
        "stream": stream,
        "sequence": "1",
        "value": {
            "kind": "agent_component",
            "value": {
                "component": "sandbox",
                "event": {
                    "terminal": "failed",
                    "failure": {"kind": "cancelled", "message": "cancelled"}
                }
            }
        }
    });
    assert!(
        serde_json::from_value::<ExtensionStreamEvent>(terminal_as_chunk).is_err(),
        "a terminal Sandbox event must not be accepted as a stream chunk"
    );

    let chunk_as_terminal = serde_json::json!({
        "event": "complete",
        "stream": {
            "id": "stream-1",
            "generation": "1",
            "kind": "stream"
        },
        "sequence": "1",
        "value": {
            "kind": "agent_component",
            "value": {
                "component": "workflow",
                "terminal": {
                    "event": "node_start",
                    "node_name": "start",
                    "step_index": "0"
                }
            }
        }
    });
    assert!(
        serde_json::from_value::<ExtensionStreamEvent>(chunk_as_terminal).is_err(),
        "a non-terminal Workflow event must not be accepted as stream completion"
    );
}

#[test]
fn bridge_register_fixtures_enforce_typed_descriptors() {
    use echo_sdk_protocol::methods::ExtensionRegisterRequest;
    for fixture in fixtures_of("ExtensionRegisterRequest") {
        let parsed: Result<ExtensionRegisterRequest, _> =
            serde_json::from_value(fixture.payload.clone());
        let valid = parsed
            .as_ref()
            .is_ok_and(|request| request.validate().is_ok());
        assert_eq!(
            valid,
            fixture.kind == FixtureKind::Valid,
            "fixture {} classified incorrectly",
            fixture.name
        );
        if let Ok(request) = parsed
            && fixture.kind == FixtureKind::Valid
        {
            assert_eq!(
                request.descriptor.kind(),
                request.kind,
                "fixture {} descriptor kind must match",
                fixture.name
            );
        }
    }
}

#[test]
fn bridge_invoke_and_cancel_fixtures_enforce_identity_rules() {
    use echo_sdk_protocol::methods::{ExtensionCancelNotice, ExtensionInvokeCall};
    for fixture in fixtures_of("ExtensionInvokeCall") {
        let parsed: Result<ExtensionInvokeCall, _> =
            serde_json::from_value(fixture.payload.clone());
        let valid = parsed.as_ref().is_ok_and(|call| call.validate().is_ok());
        assert_eq!(
            valid,
            fixture.kind == FixtureKind::Valid,
            "fixture {} classified incorrectly",
            fixture.name
        );
    }
    for fixture in fixtures_of("ExtensionCancelNotice") {
        let parsed: Result<ExtensionCancelNotice, _> =
            serde_json::from_value(fixture.payload.clone());
        let valid = parsed
            .as_ref()
            .is_ok_and(|notice| notice.validate().is_ok());
        assert_eq!(
            valid,
            fixture.kind == FixtureKind::Valid,
            "fixture {} classified incorrectly",
            fixture.name
        );
    }
}

#[test]
fn extension_operation_taxonomy_is_closed_and_kind_bound() {
    use echo_sdk_protocol::methods::{
        AgentComponentKindWire, ExtensionDescriptor, ExtensionKind, ExtensionOperation,
    };
    // Every operation resolves to a kind, and streaming membership is stable.
    let all = [
        ExtensionOperation::ToolExecute,
        ExtensionOperation::ToolExecuteStream,
        ExtensionOperation::ToolValidateParameters,
        ExtensionOperation::LlmChat,
        ExtensionOperation::LlmChatStream,
        ExtensionOperation::StorePut,
        ExtensionOperation::StoreGet,
        ExtensionOperation::StoreSearch,
        ExtensionOperation::StoreSearchWith,
        ExtensionOperation::StoreDelete,
        ExtensionOperation::StoreListNamespaces,
        ExtensionOperation::StoreList,
        ExtensionOperation::StorePruneExpired,
        ExtensionOperation::StoreDedupByContent,
        ExtensionOperation::HumanLoopRequest,
        ExtensionOperation::HookRun,
        ExtensionOperation::CallbackOnThinkStart,
        ExtensionOperation::CallbackOnThinkEnd,
        ExtensionOperation::CallbackOnToolStart,
        ExtensionOperation::CallbackOnToolEnd,
        ExtensionOperation::CallbackOnToolError,
        ExtensionOperation::CallbackOnToolInterrupted,
        ExtensionOperation::CallbackOnFinalAnswer,
        ExtensionOperation::CallbackOnIteration,
        ExtensionOperation::InterventionOnToolCall,
        ExtensionOperation::InterventionOnThinkStart,
        ExtensionOperation::InterventionOnFinalAnswer,
        ExtensionOperation::FactoryCreateAgent,
        ExtensionOperation::AgentExecute,
        ExtensionOperation::AgentExecuteStream,
        ExtensionOperation::AgentChat,
        ExtensionOperation::AgentChatStream,
        ExtensionOperation::AgentClose,
        ExtensionOperation::CriticCritique,
        ExtensionOperation::CompressorCompress,
        ExtensionOperation::AgentComponentCall,
    ];
    let mut names = std::collections::BTreeSet::new();
    for operation in all {
        assert!(names.insert(operation.as_str()), "duplicate operation name");
        assert!(
            operation.is_streaming()
                == matches!(
                    operation,
                    ExtensionOperation::ToolExecuteStream
                        | ExtensionOperation::LlmChatStream
                        | ExtensionOperation::AgentExecuteStream
                        | ExtensionOperation::AgentChatStream
                ),
            "streaming membership drifted for {}",
            operation.as_str()
        );
    }
    // Every extension kind owns at least one operation and one descriptor
    // variant; InterventionCallback is its own kind, not an AgentCallback
    // spelling.
    let kinds = [
        ExtensionKind::Tool,
        ExtensionKind::LlmClient,
        ExtensionKind::Store,
        ExtensionKind::HumanLoopProvider,
        ExtensionKind::Hook,
        ExtensionKind::AgentCallback,
        ExtensionKind::InterventionCallback,
        ExtensionKind::AgentFactory,
        ExtensionKind::CustomAgent,
        ExtensionKind::Critic,
        ExtensionKind::ContextCompressor,
        ExtensionKind::AgentComponent,
    ];
    for kind in kinds {
        assert!(
            all.iter().any(|operation| operation.kind() == kind),
            "kind {} has no operation",
            kind.as_str()
        );
        assert_eq!(
            ExtensionDescriptor::HumanLoopProvider {
                descriptor_version: 1
            }
            .kind(),
            ExtensionKind::HumanLoopProvider
        );
        assert_ne!(kind.as_str(), "worker");
    }
    assert_eq!(
        ExtensionDescriptor::InterventionCallback {
            descriptor_version: 1
        }
        .kind(),
        ExtensionKind::InterventionCallback
    );
    assert_eq!(
        ExtensionDescriptor::Critic {
            descriptor_version: 1,
            name: "sdk-critic".to_string(),
        }
        .kind(),
        ExtensionKind::Critic
    );
    assert!(
        ExtensionDescriptor::Critic {
            descriptor_version: 1,
            name: "sdk-critic".to_string(),
        }
        .validate()
        .is_ok()
    );
    assert_eq!(
        ExtensionDescriptor::ContextCompressor {
            descriptor_version: 1,
            name: "sdk-compressor".to_string(),
        }
        .kind(),
        ExtensionKind::ContextCompressor
    );
    assert_eq!(
        ExtensionDescriptor::AgentComponent {
            descriptor_version: 1,
            component: AgentComponentKindWire::ConversationStore,
            name: "sdk-conversation-store".to_string(),
            capabilities: AgentComponentCapabilitiesWire::default(),
        }
        .kind(),
        ExtensionKind::AgentComponent
    );
    // Descriptor fingerprints are canonical: same descriptor, same string.
    let descriptor = ExtensionDescriptor::Tool {
        descriptor_version: 1,
        name: "probe".to_string(),
        description: String::new(),
        parameters: WireValue::Null,
        schema_revision: WireU64::from_u64(0),
        required_input_modalities: Vec::new(),
        required_permissions: Vec::new(),
        risk_level: ToolRiskLevelWire::Standard,
        supports_streaming: false,
        exempt_from_batch_timeout: false,
        allows_parallel_batch_execution: true,
        manages_own_timeout: false,
    };
    assert_eq!(descriptor.fingerprint(), descriptor.clone().fingerprint());
    assert!(descriptor.validate().is_ok());
    // Unknown descriptor versions fail closed.
    assert!(
        ExtensionDescriptor::Hook {
            descriptor_version: 2,
            events: Vec::new(),
        }
        .validate()
        .is_err()
    );
}

#[test]
fn critic_invocation_and_result_are_typed_and_bounded() {
    use echo_sdk_protocol::methods::{
        CritiqueInput, CritiqueWire, ExtensionInvocation, ExtensionOperation, ExtensionResult,
    };
    let input = CritiqueInput {
        task: "solve".to_string(),
        answer: "42".to_string(),
        context: "math".to_string(),
    };
    assert!(input.validate().is_ok());
    let invocation = ExtensionInvocation::CriticCritique(input.clone());
    assert_eq!(invocation.operation(), ExtensionOperation::CriticCritique);
    let result = ExtensionResult::CriticCritique(CritiqueWire {
        score: 9.5,
        passed: true,
        feedback: "good".to_string(),
        suggestions: Vec::new(),
    });
    assert_eq!(result.operation(), ExtensionOperation::CriticCritique);
    assert!(
        CritiqueWire {
            score: 10.0,
            passed: true,
            feedback: String::new(),
            suggestions: Vec::new(),
        }
        .validate()
        .is_ok()
    );
    assert!(
        CritiqueWire {
            score: 10.1,
            passed: true,
            feedback: String::new(),
            suggestions: Vec::new(),
        }
        .validate()
        .is_err()
    );
}

#[test]
fn agent_component_extended_methods_and_stream_capabilities_are_closed() {
    use echo_sdk_protocol::methods::{
        AgentComponentCallInputWire, AgentComponentCallResultWire, AgentComponentKindWire,
        AgentComponentOperationWire, ExtensionDescriptor,
    };

    let cases = [
        (
            AgentComponentCallInputWire::ConversationEnsure {
                conversation: WireValue::Null,
            },
            AgentComponentOperationWire::ConversationEnsure,
        ),
        (
            AgentComponentCallInputWire::RunAppendEvent {
                run_id: "run".to_string(),
                event: WireValue::Null,
            },
            AgentComponentOperationWire::RunAppendEvent,
        ),
        (
            AgentComponentCallInputWire::WorkflowCheckpointSaveIfGeneration {
                checkpoint: WireValue::Null,
                expected_generation: WireU64::from_u64(3),
            },
            AgentComponentOperationWire::WorkflowCheckpointSaveIfGeneration,
        ),
        (
            AgentComponentCallInputWire::WorkflowCheckpointAckClaim {
                checkpoint_id: "checkpoint".to_string(),
                attempt_id: "attempt".to_string(),
            },
            AgentComponentOperationWire::WorkflowCheckpointAckClaim,
        ),
        (
            AgentComponentCallInputWire::WorkflowCheckpointRequeueClaim {
                checkpoint_id: "checkpoint".to_string(),
                attempt_id: "attempt".to_string(),
            },
            AgentComponentOperationWire::WorkflowCheckpointRequeueClaim,
        ),
        (
            AgentComponentCallInputWire::WorkflowCheckpointRenewClaim {
                checkpoint_id: "checkpoint".to_string(),
                attempt_id: "attempt".to_string(),
            },
            AgentComponentOperationWire::WorkflowCheckpointRenewClaim,
        ),
        (
            AgentComponentCallInputWire::SandboxExecuteStream {
                command: WireValue::Null,
            },
            AgentComponentOperationWire::SandboxExecuteStream,
        ),
        (
            AgentComponentCallInputWire::WorkflowRunStream {
                input: String::new(),
            },
            AgentComponentOperationWire::WorkflowRunStream,
        ),
        (
            AgentComponentCallInputWire::IntentClassify {
                user_input: String::new(),
                context: Vec::new(),
            },
            AgentComponentOperationWire::IntentClassify,
        ),
    ];
    for (input, operation) in cases {
        assert_eq!(input.operation(), operation);
    }

    for (result, operation) in [
        (
            AgentComponentCallResultWire::WorkflowCheckpointSaveIfGeneration { committed: true },
            AgentComponentOperationWire::WorkflowCheckpointSaveIfGeneration,
        ),
        (
            AgentComponentCallResultWire::WorkflowCheckpointAckClaim,
            AgentComponentOperationWire::WorkflowCheckpointAckClaim,
        ),
        (
            AgentComponentCallResultWire::WorkflowCheckpointRequeueClaim,
            AgentComponentOperationWire::WorkflowCheckpointRequeueClaim,
        ),
        (
            AgentComponentCallResultWire::WorkflowCheckpointRenewClaim,
            AgentComponentOperationWire::WorkflowCheckpointRenewClaim,
        ),
    ] {
        assert_eq!(result.operation(), operation);
        assert_eq!(
            operation.component(),
            AgentComponentKindWire::WorkflowCheckpointStore
        );
    }

    let sandbox = ExtensionDescriptor::AgentComponent {
        descriptor_version: 1,
        component: AgentComponentKindWire::SandboxExecutor,
        name: "sandbox".to_string(),
        capabilities: AgentComponentCapabilitiesWire {
            supports_streaming: true,
            ..AgentComponentCapabilitiesWire::default()
        },
    };
    assert!(sandbox.validate().is_ok());
    let audit = ExtensionDescriptor::AgentComponent {
        descriptor_version: 1,
        component: AgentComponentKindWire::AuditLogger,
        name: "audit".to_string(),
        capabilities: AgentComponentCapabilitiesWire {
            supports_streaming: true,
            ..AgentComponentCapabilitiesWire::default()
        },
    };
    assert!(audit.validate().is_err());
    let checkpoint_store = ExtensionDescriptor::AgentComponent {
        descriptor_version: 1,
        component: AgentComponentKindWire::WorkflowCheckpointStore,
        name: "checkpoints".to_string(),
        capabilities: AgentComponentCapabilitiesWire {
            claim_heartbeat_interval_ms: Some(WireU64::from_u64(30_000)),
            ..AgentComponentCapabilitiesWire::default()
        },
    };
    assert!(checkpoint_store.validate().is_ok());
    let checkpoint_without_heartbeat = ExtensionDescriptor::AgentComponent {
        descriptor_version: 1,
        component: AgentComponentKindWire::WorkflowCheckpointStore,
        name: "checkpoints".to_string(),
        capabilities: AgentComponentCapabilitiesWire::default(),
    };
    assert!(checkpoint_without_heartbeat.validate().is_err());
    let misplaced_heartbeat = ExtensionDescriptor::AgentComponent {
        descriptor_version: 1,
        component: AgentComponentKindWire::AuditLogger,
        name: "audit".to_string(),
        capabilities: AgentComponentCapabilitiesWire {
            claim_heartbeat_interval_ms: Some(WireU64::from_u64(30_000)),
            ..AgentComponentCapabilitiesWire::default()
        },
    };
    assert!(misplaced_heartbeat.validate().is_err());
}

#[test]
fn extension_rpc_types_bind_the_official_jsonrpc_surface() {
    use agent_client_protocol::JsonRpcMessage as _;
    use echo_sdk_protocol::methods::{
        ExtensionCancelNotice, ExtensionInvokeCall, ExtensionRegisterRequest, ExtensionStreamEvent,
        ExtensionUnregisterRequest,
    };
    // The typed RPC derives bind each DTO to its catalog method name, so the
    // official runtime can dispatch and route them without any local parser.
    type MethodMatcher = fn(&str) -> bool;
    let registered: [(&str, MethodMatcher); 4] = [
        (
            "_echo_agent/extension/register",
            ExtensionRegisterRequest::matches_method,
        ),
        (
            "_echo_agent/extension/unregister",
            ExtensionUnregisterRequest::matches_method,
        ),
        (
            "_echo_agent/extension/invoke",
            ExtensionInvokeCall::matches_method,
        ),
        (
            "_echo_agent/extension/cancel",
            ExtensionCancelNotice::matches_method,
        ),
    ];
    for (name, matcher) in registered {
        assert!(matcher(name), "{name} must match its own DTO");
        assert!(!matcher("_echo_agent/extension/stream"));
    }
    assert!(ExtensionStreamEvent::matches_method(
        "_echo_agent/extension/stream"
    ));
    assert!(!ExtensionStreamEvent::matches_method(
        "_echo_agent/extension/cancel"
    ));
}

#[test]
fn handle_fixtures_enforce_non_empty_identities() -> Result<(), String> {
    for fixture in fixtures_of("WireHandle") {
        let handle: WireHandle = parse_fixture(&fixture)?;
        let valid = handle.validate().is_ok();
        match fixture.kind {
            FixtureKind::Valid => assert!(valid, "fixture {} must validate", fixture.name),
            FixtureKind::Invalid => assert!(!valid, "fixture {} must be rejected", fixture.name),
        }
    }
    Ok(())
}

#[test]
fn error_fixtures_round_trip_typed_codes() -> Result<(), String> {
    for fixture in fixtures_of("EchoSdkError") {
        let error: EchoSdkError = parse_fixture(&fixture)?;
        let valid = error.validate().is_ok();
        assert_eq!(valid, fixture.kind == FixtureKind::Valid);
        if valid {
            let back = serde_json::to_value(&error).unwrap_or(serde_json::Value::Null);
            assert_eq!(
                back, fixture.payload,
                "fixture {} is not lossless",
                fixture.name
            );
        }
    }
    Ok(())
}

#[test]
fn event_fixtures_preserve_every_envelope_fact() -> Result<(), String> {
    for fixture in fixtures_of("WireEventEnvelope") {
        let parsed: Result<WireEventEnvelope, _> = serde_json::from_value(fixture.payload.clone());
        match (fixture.kind, parsed) {
            (FixtureKind::Valid, Ok(envelope)) => {
                let valid = envelope.validate().is_ok();
                assert!(valid, "fixture {} must validate", fixture.name);
                let back = serde_json::to_value(&envelope).unwrap_or(serde_json::Value::Null);
                assert_eq!(
                    back, fixture.payload,
                    "fixture {} is not lossless",
                    fixture.name
                );
            }
            (FixtureKind::Invalid, Ok(envelope)) => {
                assert!(
                    envelope.validate().is_err(),
                    "fixture {} must be rejected",
                    fixture.name
                );
            }
            (FixtureKind::Invalid, Err(_)) => {}
            (FixtureKind::Valid, Err(error)) => {
                return Err(format!("fixture {} rejected: {error}", fixture.name));
            }
        }
    }
    Ok(())
}

#[test]
fn replay_fixtures_round_trip_with_gap() -> Result<(), String> {
    for fixture in fixtures_of("ReplayResponse") {
        let response: ReplayResponse = parse_fixture(&fixture)?;
        assert!(response.validate().is_ok());
        let back = serde_json::to_value(&response).unwrap_or(serde_json::Value::Null);
        assert_eq!(
            back, fixture.payload,
            "fixture {} is not lossless",
            fixture.name
        );
        assert!(
            response.gap.is_some(),
            "fixture {} should carry a gap",
            fixture.name
        );
    }
    Ok(())
}

#[test]
fn live_gap_identifies_its_stream() -> Result<(), String> {
    for fixture in fixtures_of("GapNotification") {
        let notification: GapNotification = parse_fixture(&fixture)?;
        assert_eq!(
            notification.validate().is_ok(),
            fixture.kind == FixtureKind::Valid
        );
    }
    Ok(())
}

#[test]
fn replay_requests_validate_stream_and_bounds() -> Result<(), String> {
    for fixture in fixtures_of("ReplayRequest") {
        let parsed: Result<ReplayRequest, _> = serde_json::from_value(fixture.payload.clone());
        let valid = parsed
            .as_ref()
            .is_ok_and(|request| request.validate().is_ok());
        assert_eq!(valid, fixture.kind == FixtureKind::Valid);
    }
    Ok(())
}

#[test]
fn capability_fixtures_validate_shape_rules() -> Result<(), String> {
    for fixture in fixtures_of("EchoAgentCapability") {
        let capability: EchoAgentCapability = parse_fixture(&fixture)?;
        let problems = capability.validate_shape();
        match fixture.kind {
            FixtureKind::Valid => {
                assert!(
                    problems.is_empty(),
                    "fixture {} invalid: {problems:?}",
                    fixture.name
                );
            }
            FixtureKind::Invalid => {
                assert!(
                    !problems.is_empty(),
                    "fixture {} must be rejected by shape validation",
                    fixture.name
                );
            }
        }
    }
    Ok(())
}

#[test]
fn boundary_fixtures_reject_standard_and_foreign_methods() {
    for fixture in fixtures_of("MethodCatalog") {
        let method = fixture
            .payload
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let in_extension = METHOD_CATALOG.iter().any(|m| m.name == method);
        // Neither a standard ACP method nor an un-namespaced method may be
        // part of the extension contract; the first is owned by the official
        // schema, the second violates the `_echo_agent/` prefix rule.
        assert!(
            !in_extension,
            "fixture {} names a method inside the extension catalog",
            fixture.name
        );
        let is_standard = catalog::official_acp_v1_methods().contains(method);
        let is_namespaced = method.starts_with(echo_sdk_protocol::EXTENSION_NAMESPACE);
        // "session/prompt" is standard-owned; "run/start" is un-namespaced.
        // Both must stay outside the extension surface.
        assert!(
            !in_extension && !is_namespaced && (is_standard || !is_namespaced),
            "fixture {} must stay outside the extension contract",
            fixture.name
        );
    }
}

#[test]
fn catalog_passes_mechanical_validation() {
    assert!(catalog::validate_catalog(METHOD_CATALOG).is_empty());
    // Every catalog entry is uniquely named and namespaced.
    let mut names = std::collections::BTreeSet::new();
    for method in METHOD_CATALOG {
        assert!(names.insert(method.name), "duplicate {}", method.name);
        assert!(method.name.starts_with("_echo_agent/"));
    }
}

#[test]
fn reverse_callback_stream_directions_are_consistent() {
    let direction = |name: &str| {
        METHOD_CATALOG
            .iter()
            .find(|method| method.name == name)
            .map(|method| method.direction)
    };
    assert_eq!(
        direction("_echo_agent/extension/invoke"),
        Some(Direction::ReverseRequest)
    );
    assert_eq!(
        direction("_echo_agent/extension/stream"),
        Some(Direction::ClientNotification)
    );
    assert_eq!(
        direction("_echo_agent/extension/cancel"),
        Some(Direction::HostNotification)
    );
}

#[test]
fn schema_document_is_deterministic_and_within_boundaries() -> Result<(), String> {
    let first = build_extension_schema_doc();
    let second = build_extension_schema_doc();
    assert_eq!(first, second);
    assert!(validate_schema_boundaries(&first).is_empty());
    // The catalog embedded in the schema lists every method with direction.
    let embedded = first
        .get("method_catalog")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "method_catalog missing".to_string())?;
    assert_eq!(embedded.len(), METHOD_CATALOG.len());
    let validator = build_extension_validator(&first);
    assert!(
        validator.is_ok(),
        "generated schema has unresolved references"
    );
    let definitions = first
        .get("definitions")
        .and_then(|value| value.as_object())
        .ok_or_else(|| "definitions missing".to_string())?;
    for method in METHOD_CATALOG {
        assert!(definitions.contains_key(method.params_schema()));
        if let Some(result) = method.result_schema() {
            assert!(definitions.contains_key(result));
        }
        if let Some(error) = method.error_schema() {
            assert!(definitions.contains_key(error));
        }
    }
    for reusable in [
        "ExtensionInvocation",
        "ExtensionResult",
        "ExtensionStreamChunkValue",
        "ExtensionStreamCompleteValue",
        "AgentEventWire",
        "AgentStreamChunkWire",
        "AgentStreamTerminalWire",
    ] {
        assert!(
            definitions.contains_key(reusable),
            "public extension type {reusable} must have a named schema definition"
        );
    }
    let invocation_ref = definitions
        .get("ExtensionInvokeCall")
        .and_then(|schema| schema.pointer("/properties/invocation/$ref"))
        .or_else(|| {
            definitions
                .get("ExtensionInvokeCall")
                .and_then(|schema| schema.pointer("/properties/invocation/allOf/0/$ref"))
        })
        .and_then(|value| value.as_str());
    assert_eq!(
        invocation_ref,
        Some("#/definitions/ExtensionInvocation"),
        "ExtensionInvokeCall must reference the reusable invocation union"
    );
    Ok(())
}

#[test]
fn schema_rejects_structurally_invalid_fixtures() -> Result<(), String> {
    let document = build_extension_schema_doc();
    let definitions = document
        .get("definitions")
        .cloned()
        .ok_or_else(|| "definitions missing".to_string())?;
    let schema_checked = [
        "scalar-u64-leading-zeros-invalid",
        "scalar-u64-float-invalid",
        "scalar-u64-overflow-invalid",
        "scalar-path-relative-invalid",
        "scalar-path-base64-invalid",
        "scalar-path-unix-relative-encoded-invalid",
        "scalar-path-windows-relative-encoded-invalid",
        "scalar-path-native-empty-invalid",
        "scalar-bytes-base64-invalid",
        "scalar-bytes-length-invalid",
        "scalar-unknown-empty-tag-invalid",
        "handle-empty-id-invalid",
        "event-envelope-sequence-zero-invalid",
        "event-replay-empty-stream-invalid",
        "event-replay-zero-limit-invalid",
        "extension-outcome-empty-invalid",
        "extension-cancel-notice-empty-reason-invalid",
        "extension-stream-sequence-zero-invalid",
    ];
    for fixture in schema::all_fixtures()
        .into_iter()
        .filter(|fixture| schema_checked.contains(&fixture.name.as_str()))
    {
        let target_schema = serde_json::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$ref": format!("#/definitions/{}", fixture.target),
            "definitions": definitions.clone(),
        });
        let validator = build_extension_validator(&target_schema)
            .map_err(|error| format!("schema for {} failed: {error}", fixture.target))?;
        assert!(
            !validator.is_valid(&fixture.payload),
            "schema accepted invalid fixture {}",
            fixture.name
        );
    }
    Ok(())
}

#[test]
fn digest_covers_schema_and_catalog() {
    let digest = schema::extension_contract_digest();
    assert_eq!(digest, schema::extension_contract_digest());
    // Any catalog change must move the digest: prove sensitivity by feeding a
    // mutated catalog document through the same digest function.
    let mut mutated = build_extension_schema_doc();
    if let Some(catalog_value) = mutated
        .get_mut("method_catalog")
        .and_then(|v| v.as_array_mut())
    {
        catalog_value.push(serde_json::json!({
            "name": "_echo_agent/probe/extra",
            "direction": Direction::Request.as_str(),
            "capability": "runs",
            "summary": "digest sensitivity probe",
        }));
    }
    assert_ne!(digest, schema::digest_of(&mutated));
}

#[test]
fn fixtures_directory_content_matches_in_memory_set() {
    // The committed fixture files must be exactly what the code generates;
    // the export tool enforces byte equality, here we check structure.
    let files = schema::build_fixtures();
    assert!(!files.is_empty());
    let all = schema::all_fixtures();
    let names: Vec<&str> = all.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(files.len(), names.len());
    for (path, content) in &files {
        assert!(path.starts_with("contracts/sdk/fixtures/extension/v1/"));
        let parsed: Result<schema::Fixture, _> = serde_json::from_str(content);
        assert!(parsed.is_ok(), "fixture file {path} not parseable");
    }
}

#[test]
fn catalog_helper_detects_violations() {
    let bad = vec![MethodDescriptor {
        name: "session/prompt",
        direction: Direction::Request,
        capability: echo_sdk_protocol::capability::ExtensionCapability::Runs,
        summary: "",
    }];
    let problems = catalog::validate_catalog(&bad);
    assert!(problems.iter().any(|p| p.contains("underscore")));
    assert!(problems.iter().any(|p| p.contains("standard ACP method")));
}

// ── Facade operation fixtures (plan 07 todo 1) ──────────────────────────────

#[test]
fn facade_operation_fixtures_round_trip_and_reject() {
    use echo_sdk_protocol::error::FacadeFailureDetail;
    use echo_sdk_protocol::methods::FeatureOperationRequest;

    for fixture in fixtures_of("FeatureOperationRequest") {
        match fixture.kind {
            FixtureKind::Valid => {
                let parsed: FeatureOperationRequest =
                    parse_fixture(&fixture).unwrap_or_else(|error| panic!("{error}"));
                assert!(parsed.validate().is_ok(), "fixture {}", fixture.name);
                let back = serde_json::to_value(&parsed).unwrap_or(serde_json::Value::Null);
                assert_eq!(back, fixture.payload, "fixture {}", fixture.name);
            }
            FixtureKind::Invalid => {
                let parsed: Result<FeatureOperationRequest, _> =
                    serde_json::from_value(fixture.payload.clone());
                let rejected = match parsed {
                    Err(_) => true,
                    Ok(request) => request.validate().is_err(),
                };
                assert!(rejected, "fixture {} must be rejected", fixture.name);
            }
        }
    }
    for fixture in fixtures_of("FacadeFailureDetail") {
        match fixture.kind {
            FixtureKind::Valid => {
                let parsed: FacadeFailureDetail =
                    parse_fixture(&fixture).unwrap_or_else(|error| panic!("{error}"));
                assert!(parsed.validate().is_ok(), "fixture {}", fixture.name);
            }
            FixtureKind::Invalid => {
                let parsed: Result<FacadeFailureDetail, _> =
                    serde_json::from_value(fixture.payload.clone());
                let rejected = match parsed {
                    Err(_) => true,
                    Ok(detail) => detail.validate().is_err(),
                };
                assert!(rejected, "fixture {} must be rejected", fixture.name);
            }
        }
    }
    // Wildcards are never executable identities: even a well-formed request
    // carrying one fails closed.
    let wildcard = FeatureOperationRequest {
        operation: "_echo_agent/task/*".to_string(),
        signature_digest: "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .to_string(),
        handle: None,
        arguments: Vec::new(),
    };
    assert!(wildcard.validate().is_err());
}

#[test]
fn authoritative_tool_facts_and_interruption_identity_are_lossless() -> Result<(), String> {
    use echo_sdk_protocol::methods::{ExtensionInvocation, ToolResultWire};
    let result = serde_json::json!({
        "kind":{"kind":"text"},"success":true,"output":"已完成 🧪","truncated":false,
        "effects":[
            {"effect":"file_read","path":"文档/计划.md"},
            {"effect":"test_run","command":"cargo test","passed":false,"failure_count":"9007199254740993"}
        ]
    });
    let parsed: ToolResultWire =
        serde_json::from_value(result.clone()).map_err(|e| e.to_string())?;
    assert_eq!(
        serde_json::to_value(parsed).map_err(|e| e.to_string())?,
        result
    );
    let unknown = serde_json::json!({
        "kind":{"kind":"text"},"success":true,"output":"ok","truncated":false,
        "effects":[{"effect":"unknown_effect","path":"x"}]
    });
    assert!(serde_json::from_value::<ToolResultWire>(unknown).is_err());
    let interrupted = serde_json::json!({"operation":"callback_on_tool_interrupted","input":{
        "agent":"智能体","call_id":"canonical-call","tool":"edit","input":{"kind":"null"},"error":"取消"
    }});
    let parsed: ExtensionInvocation =
        serde_json::from_value(interrupted.clone()).map_err(|e| e.to_string())?;
    assert_eq!(
        serde_json::to_value(parsed).map_err(|e| e.to_string())?,
        interrupted
    );
    Ok(())
}

#[test]
fn managed_projection_authority_and_deadline_preserve_full_integer_range() -> Result<(), String> {
    use echo_sdk_protocol::methods::{
        AgentComponentCallInputWire, AgentComponentCapabilitiesWire,
        ConversationProjectionAuthorityWire, ConversationProjectionCapabilityWire,
        PersistenceCallCapabilityWire, RuntimeStateCapabilityWire,
    };
    let authority = serde_json::json!({"conversation_id":"会话 🧭","epoch":"9007199254740993",
        "revision":"18446744073709551615","lifecycle":"live","delete_receipt_retention_floor_epoch":"0"});
    let parsed: ConversationProjectionAuthorityWire =
        serde_json::from_value(authority.clone()).map_err(|e| e.to_string())?;
    assert_eq!(
        serde_json::to_value(parsed).map_err(|e| e.to_string())?,
        authority
    );
    let request = serde_json::json!({"operation":"conversation_get_projection_authority_with_context","input":{
        "conversation_id":"会话 🧭","context":{"absolute_deadline_unix_ms":"9223372036854775807"}}});
    let parsed: AgentComponentCallInputWire =
        serde_json::from_value(request.clone()).map_err(|e| e.to_string())?;
    assert_eq!(
        serde_json::to_value(parsed).map_err(|e| e.to_string())?,
        request
    );
    let caps: AgentComponentCapabilitiesWire =
        serde_json::from_value(serde_json::json!({})).map_err(|e| e.to_string())?;
    assert_eq!(
        caps.conversation_projection,
        ConversationProjectionCapabilityWire::Unsupported
    );
    assert_eq!(caps.runtime_state, RuntimeStateCapabilityWire::Unsupported);
    assert_eq!(
        caps.persistence_call,
        PersistenceCallCapabilityWire::Unsupported
    );
    for malformed in [
        serde_json::json!({"operation":"conversation_get_projection_authority_with_context","input":{"conversation_id":"c"}}),
        serde_json::json!({"operation":"conversation_get_projection_authority_with_context","input":{"conversation_id":"c","context":{"absolute_deadline_unix_ms":9223372036854775807_i64}}}),
    ] {
        assert!(serde_json::from_value::<AgentComponentCallInputWire>(malformed).is_err());
    }
    Ok(())
}

#[test]
fn factory_origin_requires_distinct_same_generation_agent_target() -> Result<(), String> {
    use echo_sdk_protocol::methods::ExtensionInvokeCall;
    let original = serde_json::json!({
        "extension":{"id":"child","generation":"7","kind":"extension"},
        "factory_origin":{"id":"factory","generation":"7","kind":"extension"},
        "invocation_id":"invocation","invocation":{"operation":"agent_execute","input":{"task":"任务"}},
        "deadline":{"seconds":"20","nanos":0}
    });
    let parsed: ExtensionInvokeCall =
        serde_json::from_value(original.clone()).map_err(|e| e.to_string())?;
    parsed.validate().map_err(str::to_string)?;
    assert_eq!(
        serde_json::to_value(parsed).map_err(|e| e.to_string())?,
        original
    );
    for mutation in [
        serde_json::json!({"id":"child","generation":"7","kind":"extension"}),
        serde_json::json!({"id":"factory","generation":"8","kind":"extension"}),
        serde_json::json!({"id":"factory","generation":"7","kind":"run"}),
    ] {
        let mut changed = original.clone();
        changed
            .as_object_mut()
            .ok_or("factory fixture must be an object")?
            .insert("factory_origin".to_string(), mutation);
        let parsed: ExtensionInvokeCall =
            serde_json::from_value(changed).map_err(|e| e.to_string())?;
        assert!(parsed.validate().is_err());
    }
    let mut wrong_operation = original;
    wrong_operation.as_object_mut().ok_or("factory fixture must be an object")?.insert("invocation".to_string(),serde_json::json!({"operation":"tool_validate_parameters","input":{"parameters":{"kind":"null"}}}));
    let parsed: ExtensionInvokeCall =
        serde_json::from_value(wrong_operation).map_err(|e| e.to_string())?;
    assert!(parsed.validate().is_err());
    Ok(())
}

#[test]
fn delivery_claim_wire_shape_changes_its_admission_signature() -> Result<(), String> {
    use echo_sdk_protocol::facade::{
        family_operation_signature_digest, family_operation_wire_schema,
    };
    use sha2::{Digest as _, Sha256};
    for operation in [
        "delivery.claim_next",
        "delivery.transition",
        "delivery.defer",
        "delivery.settle",
    ] {
        let old =
            serde_json::to_vec(&serde_json::json!({"family":"delivery","operation":operation}))
                .map_err(|e| e.to_string())?;
        let old_digest = format!("sha256:{:x}", Sha256::digest(old));
        let new_digest = family_operation_signature_digest("delivery", operation);
        assert_ne!(
            new_digest, old_digest,
            "old id-only contract must not retain admission signature"
        );
        assert!(family_operation_wire_schema("delivery", operation).is_some());
    }
    for (family, operation) in [
        ("memory", "memory.store.get"),
        ("delivery", "delivery.enqueue"),
        ("workflow", "workflow.graph.run"),
    ] {
        let old = serde_json::to_vec(&serde_json::json!({"family":family,"operation":operation}))
            .map_err(|e| e.to_string())?;
        assert_eq!(
            family_operation_signature_digest(family, operation),
            format!("sha256:{:x}", Sha256::digest(old))
        );
    }
    let schema = family_operation_wire_schema("delivery", "delivery.transition")
        .ok_or("missing delivery schema")?;
    let validator = jsonschema::validator_for(schema.get("input").ok_or("missing input schema")?)
        .map_err(|e| e.to_string())?;
    let handle = serde_json::json!({"id":"ledger","generation":"3","kind":"facade_resource"});
    let claim = serde_json::json!({"message_id":"消息","route":"inbox","payload":{"text":"数据"},"attempt_id":"attempt-1","attempt":1,"claimed_at":"2026-10-12T00:00:00Z"});
    let valid_input = serde_json::json!([handle.clone(), claim, "effect_started", "turn-1"]);
    let errors = validator
        .iter_errors(&valid_input)
        .map(|error| error.to_string())
        .collect::<Vec<_>>();
    assert!(
        errors.is_empty(),
        "valid delivery claim rejected: {errors:?}"
    );
    assert!(!validator.is_valid(&serde_json::json!([
        handle,
        "消息",
        "effect_started",
        "turn-1"
    ])));
    // Mutating the DTO-derived accepted shape changes the same signature input.
    let mut mutated = schema;
    mutated
        .as_object_mut()
        .ok_or("shape must be object")?
        .insert(
            "input".to_string(),
            serde_json::json!({"type":"array","items":{"type":"string"}}),
        );
    let canonical = serde_json::json!({"family":"delivery","operation":"delivery.transition","wire_schema":mutated});
    let bytes = serde_json::to_vec(&canonical).map_err(|e| e.to_string())?;
    assert_ne!(
        family_operation_signature_digest("delivery", "delivery.transition"),
        format!("sha256:{:x}", Sha256::digest(bytes))
    );
    Ok(())
}

#[test]
fn factory_create_binds_construction_to_the_reserved_instance() -> Result<(), String> {
    use echo_sdk_protocol::methods::ExtensionInvokeCall;
    let valid = serde_json::json!({
        "extension":{"id":"factory","generation":"7","kind":"extension"},
        "invocation_id":"create-1","invocation":{"operation":"factory_create_agent","input":{
            "instance":{"id":"child","generation":"7","kind":"extension"},
            "model":"model","name":"agent","system_prompt":"prompt","tool_count":"0"}},
        "deadline":{"seconds":"20","nanos":0}
    });
    let parsed: ExtensionInvokeCall =
        serde_json::from_value(valid.clone()).map_err(|e| e.to_string())?;
    parsed.validate().map_err(str::to_string)?;
    assert_eq!(
        serde_json::to_value(parsed).map_err(|e| e.to_string())?,
        valid
    );
    let mut missing = valid.clone();
    missing
        .pointer_mut("/invocation/input")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or("missing fixture input")?
        .remove("instance");
    assert!(serde_json::from_value::<ExtensionInvokeCall>(missing).is_err());
    let mut conflicting = valid;
    conflicting
        .pointer_mut("/invocation/input/instance")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or("missing fixture instance")?
        .insert("id".to_string(), serde_json::json!("factory"));
    let parsed: ExtensionInvokeCall =
        serde_json::from_value(conflicting).map_err(|e| e.to_string())?;
    assert!(parsed.validate().is_err());
    Ok(())
}

#[test]
fn task_store_execution_facts_preserve_claim_revision_and_absent_result() -> Result<(), String> {
    use echo_sdk_protocol::methods::{TaskExecutionCommitWire, TaskExecutionWire};
    let execution = serde_json::json!({"task_id":"任务 🧪","status":{"status":"running"},"retry_count":0,
        "failure_fingerprint":null,"claim":{"claim_id":"claim-1","revision":"9007199254740993","attempt":1,"spec_hash":"digest"},
        "interruption_requested":{"paused":{"reason":"暂停"}}});
    let parsed: TaskExecutionWire =
        serde_json::from_value(execution.clone()).map_err(|e| e.to_string())?;
    assert!(parsed.result.is_none());
    assert_eq!(
        serde_json::to_value(&parsed).map_err(|e| e.to_string())?,
        execution
    );
    let mut supplied = execution.clone();
    supplied
        .as_object_mut()
        .ok_or("execution must be object")?
        .insert("result".to_string(), serde_json::json!({"kind":"null"}));
    let parsed: TaskExecutionWire =
        serde_json::from_value(supplied.clone()).map_err(|e| e.to_string())?;
    assert_eq!(parsed.result, Some(WireValue::Null));
    assert_eq!(
        serde_json::to_value(&parsed).map_err(|e| e.to_string())?,
        supplied
    );
    let commit = serde_json::json!({"expected_revision":"9007199254740993","expected_executions":{"任务 🧪":execution},
        "next":{"snapshot":{"revision":"9007199254740993","tasks":[]},
            "context":{"goal":"目标","assumptions":[],"risks":[],"execution_mode":"parallel","metadata":{"kind":"null"}}}});
    let parsed: TaskExecutionCommitWire =
        serde_json::from_value(commit.clone()).map_err(|e| e.to_string())?;
    assert_eq!(
        serde_json::to_value(parsed).map_err(|e| e.to_string())?,
        commit
    );
    let mut unsafe_number = commit;
    unsafe_number
        .as_object_mut()
        .ok_or("commit must be object")?
        .insert(
            "expected_revision".to_string(),
            serde_json::json!(9007199254740993_u64),
        );
    assert!(serde_json::from_value::<TaskExecutionCommitWire>(unsafe_number).is_err());
    Ok(())
}

#[test]
fn execution_cas_capability_requires_explicit_opt_in_and_matching_component() -> Result<(), String>
{
    use echo_sdk_protocol::methods::{AgentComponentCapabilitiesWire, ExtensionDescriptor};
    let absent: AgentComponentCapabilitiesWire =
        serde_json::from_value(serde_json::json!({})).map_err(|e| e.to_string())?;
    assert!(!absent.execution_cas_v1);
    let disabled: AgentComponentCapabilitiesWire =
        serde_json::from_value(serde_json::json!({"execution_cas_v1":false}))
            .map_err(|e| e.to_string())?;
    assert!(!disabled.execution_cas_v1);
    let enabled: AgentComponentCapabilitiesWire =
        serde_json::from_value(serde_json::json!({"execution_cas_v1":true}))
            .map_err(|e| e.to_string())?;
    assert!(enabled.execution_cas_v1);
    assert_eq!(
        serde_json::to_value(&enabled)
            .map_err(|e| e.to_string())?
            .get("execution_cas_v1"),
        Some(&serde_json::json!(true))
    );
    for illegal in [
        serde_json::json!("true"),
        serde_json::json!(1),
        serde_json::json!(null),
    ] {
        assert!(
            serde_json::from_value::<AgentComponentCapabilitiesWire>(
                serde_json::json!({"execution_cas_v1":illegal})
            )
            .is_err()
        );
    }
    for (component, valid) in [
        ("revisioned_task_store", true),
        ("conversation_store", false),
        ("audit_logger", false),
    ] {
        let descriptor: ExtensionDescriptor = serde_json::from_value(
            serde_json::json!({"kind":"agent_component","descriptor_version":1,
            "component":component,"name":"store","capabilities":{"execution_cas_v1":true}}),
        )
        .map_err(|e| e.to_string())?;
        assert_eq!(descriptor.validate().is_ok(), valid);
    }
    Ok(())
}

#[test]
fn channels_forward_operations_freeze_typed_admission_shapes() -> Result<(), String> {
    use echo_sdk_protocol::facade::{
        FacadeFamily, family_operation_signature_digest, family_operation_wire_schema,
    };
    use echo_sdk_protocol::methods::{ChannelInboundMessageWire, ChannelOutboundMessageWire};
    use sha2::{Digest as _, Sha256};
    let operations = [
        "channels.handler.handle",
        "channels.handler.handle_stream",
        "channels.handler.reply",
        "channels.stream.next",
        "channels.stream.cancel",
        "channels.stream.close",
    ];
    for operation in operations {
        assert!(FacadeFamily::Channels.operations().contains(&operation));
        let shape =
            family_operation_wire_schema("channels", operation).ok_or("missing channels shape")?;
        let old =
            serde_json::to_vec(&serde_json::json!({"family":"channels","operation":operation}))
                .map_err(|error| error.to_string())?;
        assert_ne!(
            family_operation_signature_digest("channels", operation),
            format!("sha256:{:x}", Sha256::digest(old))
        );
        let canonical = serde_json::to_vec(
            &serde_json::json!({"family":"channels","operation":operation,"wire_schema":shape}),
        )
        .map_err(|error| error.to_string())?;
        assert_eq!(
            family_operation_signature_digest("channels", operation),
            format!("sha256:{:x}", Sha256::digest(canonical))
        );
    }
    assert!(family_operation_wire_schema("channels", "channels.handler.other").is_none());
    assert!(
        !FacadeFamily::Channels
            .operations()
            .contains(&"channels.handler.other")
    );
    let handle = serde_json::json!({"id":"manager","generation":"3","kind":"facade_resource"});
    let stream = serde_json::json!({"id":"stream","generation":"3","kind":"stream"});
    let inbound = serde_json::json!({"channel_id":"channel-中文","sender_id":"sender","chat_id":"chat","chat_type":"direct","text":"输入🌊","message_id":"message","timestamp":"18446744073709551615","attachments":[{"kind":"image","data_base64":"AP9BgA==","filename":""}]});
    let outbound = serde_json::json!({"channel_id":"channel-中文","to":"chat","chat_type":"group","text":"回复🌊","reply_to":"","attachments":[{"kind":"file","data_base64":"","filename":null}]});
    let parsed: ChannelInboundMessageWire =
        serde_json::from_value(inbound.clone()).map_err(|error| error.to_string())?;
    assert_eq!(parsed.timestamp.to_u64(), Some(u64::MAX));
    let parsed: ChannelOutboundMessageWire =
        serde_json::from_value(outbound.clone()).map_err(|error| error.to_string())?;
    assert_eq!(parsed.reply_to.as_deref(), Some(""));
    for (operation, message) in [
        ("channels.handler.handle", inbound.clone()),
        ("channels.handler.handle_stream", inbound.clone()),
        ("channels.handler.reply", outbound.clone()),
    ] {
        let shape =
            family_operation_wire_schema("channels", operation).ok_or("missing forward shape")?;
        let validator = jsonschema::validator_for(shape.get("input").ok_or("missing input")?)
            .map_err(|error| error.to_string())?;
        let arguments = serde_json::json!([handle.clone(), "channel-中文", message]);
        let errors: Vec<String> = validator
            .iter_errors(&arguments)
            .map(|error| error.to_string())
            .collect();
        assert!(errors.is_empty(), "valid forward args rejected: {errors:?}");
        assert!(!validator.is_valid(&serde_json::json!([handle.clone(), "channel-中文"])));
        assert!(!validator.is_valid(&serde_json::json!([
            handle.clone(),
            "channel-中文",
            message,
            null
        ])));
        assert!(!validator.is_valid(&serde_json::json!([
            stream.clone(),
            "channel-中文",
            message
        ])));
        if let Some(object) = message.as_object() {
            for required in object
                .keys()
                .filter(|key| key.as_str() != "attachments" && key.as_str() != "reply_to")
            {
                let mut missing = message.clone();
                missing
                    .as_object_mut()
                    .ok_or("message is not object")?
                    .remove(required);
                assert!(
                    !validator.is_valid(&serde_json::json!([
                        handle.clone(),
                        "channel-中文",
                        missing
                    ])),
                    "required field {required} must not disappear"
                );
                let mut null = message.clone();
                null.as_object_mut()
                    .ok_or("message is not object")?
                    .insert(required.clone(), serde_json::Value::Null);
                assert!(
                    !validator.is_valid(&serde_json::json!([handle.clone(), "channel-中文", null])),
                    "required field {required} must not be null"
                );
            }
        }
        let mut malformed = message;
        malformed
            .as_object_mut()
            .ok_or("message is not object")?
            .insert("unknown".to_string(), serde_json::json!(true));
        assert!(!validator.is_valid(&serde_json::json!([
            handle.clone(),
            "channel-中文",
            malformed
        ])));
    }
    let shape = family_operation_wire_schema("channels", "channels.handler.handle")
        .ok_or("missing handle shape")?;
    let validator = jsonschema::validator_for(shape.get("input").ok_or("missing input")?)
        .map_err(|error| error.to_string())?;
    for timestamp in [
        serde_json::json!(1),
        serde_json::json!("-1"),
        serde_json::json!("01"),
        serde_json::json!("18446744073709551616"),
        serde_json::Value::Null,
    ] {
        let mut malformed = inbound.clone();
        malformed
            .as_object_mut()
            .ok_or("message is not object")?
            .insert("timestamp".to_string(), timestamp);
        // Canonical/overflow admission is also enforced by the reused WireU64 DTO.
        assert!(serde_json::from_value::<ChannelInboundMessageWire>(malformed.clone()).is_err());
        assert!(!validator.is_valid(&serde_json::json!([
            handle.clone(),
            "channel-中文",
            malformed
        ])));
    }
    for (kind, data) in [
        ("unknown", "AP9BgA=="),
        ("image", "-_=="),
        ("image", "AP9BgA"),
        ("image", "AB=="),
    ] {
        let mut malformed = inbound.clone();
        malformed
            .as_object_mut()
            .ok_or("message is not object")?
            .insert(
                "attachments".to_string(),
                serde_json::json!([{"kind":kind,"data_base64":data}]),
            );
        assert!(!validator.is_valid(&serde_json::json!([
            handle.clone(),
            "channel-中文",
            malformed
        ])));
    }
    for kind in ["image", "file", "audio", "video"] {
        for filename in [
            None,
            Some(serde_json::Value::Null),
            Some(serde_json::json!("")),
            Some(serde_json::json!("附件🌊")),
        ] {
            let mut attachment = serde_json::json!({"kind":kind,"data_base64":"AP9BgA=="});
            if let Some(filename) = filename {
                attachment
                    .as_object_mut()
                    .ok_or("attachment is not object")?
                    .insert("filename".to_string(), filename);
            }
            let mut message = inbound.clone();
            message
                .as_object_mut()
                .ok_or("message is not object")?
                .insert("attachments".to_string(), serde_json::json!([attachment]));
            assert!(validator.is_valid(&serde_json::json!([
                handle.clone(),
                "channel-中文",
                message
            ])));
        }
    }
    let mut malformed = inbound.clone();
    malformed
        .as_object_mut()
        .ok_or("message is not object")?
        .insert("attachments".to_string(), serde_json::Value::Null);
    assert!(!validator.is_valid(&serde_json::json!([
        handle.clone(),
        "channel-中文",
        malformed
    ])));
    let mut minimal = inbound.clone();
    minimal
        .as_object_mut()
        .ok_or("message is not object")?
        .remove("attachments");
    assert!(validator.is_valid(&serde_json::json!([
        handle.clone(),
        "channel-中文",
        minimal
    ])));
    for operation in [
        "channels.stream.next",
        "channels.stream.cancel",
        "channels.stream.close",
    ] {
        let shape =
            family_operation_wire_schema("channels", operation).ok_or("missing control shape")?;
        let validator =
            jsonschema::validator_for(shape.get("input").ok_or("missing control input")?)
                .map_err(|error| error.to_string())?;
        assert!(validator.is_valid(&serde_json::json!([stream.clone()])));
        assert!(!validator.is_valid(&serde_json::json!([handle.clone()])));
        assert!(!validator.is_valid(&serde_json::json!([stream.clone(), stream.clone()])));
    }
    Ok(())
}

#[test]
fn generated_channels_forward_fixtures_retain_session_and_full_owner_handles() -> Result<(), String>
{
    use echo_sdk_protocol::facade::family_operation_signature_digest;
    use echo_sdk_protocol::methods::FeatureOperationRequest;
    let fixtures: Vec<_> = schema::all_fixtures()
        .into_iter()
        .filter(|fixture| fixture.name.starts_with("channel-forward-"))
        .collect();
    assert_eq!(
        fixtures.len(),
        6,
        "six approved forward operations require fixture proof"
    );
    for fixture in fixtures {
        let request: FeatureOperationRequest = parse_fixture(&fixture)?;
        request.validate().map_err(str::to_string)?;
        assert_eq!(
            request.signature_digest,
            family_operation_signature_digest("channels", &request.operation)
        );
        assert!(request.handle.is_some_and(|handle| handle.kind == echo_sdk_protocol::handle::HandleKind::Session));
        let resource = request
            .arguments
            .first()
            .ok_or("fixture has no complete owner handle")?;
        assert!(
            matches!(resource, WireValue::Handle(handle) if handle.generation.to_u64() == Some(7))
        );
        assert_eq!(
            request.arguments.len(),
            if request.operation.starts_with("channels.handler.") {
                3
            } else {
                1
            }
        );
    }
    Ok(())
}

#[test]
fn channels_forward_result_shapes_preserve_the_existing_stream_envelope() -> Result<(), String> {
    use echo_sdk_protocol::facade::family_operation_wire_schema;
    let result_validator = |operation| -> Result<jsonschema::Validator, String> {
        let shape =
            family_operation_wire_schema("channels", operation).ok_or("missing result shape")?;
        jsonschema::validator_for(shape.get("result").ok_or("missing result")?)
            .map_err(|error| error.to_string())
    };
    let stream = serde_json::json!({"id":"stream","generation":"7","kind":"stream"});
    let opened = result_validator("channels.handler.handle_stream")?;
    assert!(opened.is_valid(&stream));
    assert!(
        !opened.is_valid(
            &serde_json::json!({"id":"manager","generation":"7","kind":"facade_resource"})
        )
    );
    assert!(result_validator("channels.handler.reply")?.is_valid(&serde_json::Value::Null));
    assert!(!result_validator("channels.handler.reply")?.is_valid(&serde_json::json!(true)));
    for operation in ["channels.stream.cancel", "channels.stream.close"] {
        assert!(result_validator(operation)?.is_valid(&serde_json::json!(true)));
        assert!(result_validator(operation)?.is_valid(&serde_json::json!(false)));
        assert!(!result_validator(operation)?.is_valid(&serde_json::Value::Null));
    }
    let validator = result_validator("channels.stream.next")?;
    let fields = vec![
        serde_json::json!({"name":"stream","value":{"kind":"handle","value":stream}}),
        serde_json::json!({"name":"sequence","value":{"kind":"u64","value":"1"}}),
    ];
    for variant in ["complete", "cancelled"] {
        assert!(validator.is_valid(&serde_json::json!({"kind":"variant","value":{"type_id":"echo_sdk::FacadeStreamEvent","variant":variant,"fields":fields}})));
    }
    for variant in ["item", "failed"] {
        let mut values = fields.clone();
        values.push(serde_json::json!({"name":"value","value":{"kind":"map","value":[]}}));
        let event = serde_json::json!({"kind":"variant","value":{"type_id":"echo_sdk::FacadeStreamEvent","variant":variant,"fields":values}});
        let errors: Vec<_> = validator
            .iter_errors(&event)
            .map(|error| error.to_string())
            .collect();
        assert!(
            errors.is_empty(),
            "existing stream envelope rejected: {errors:?}"
        );
        assert!(!validator.is_valid(&serde_json::json!({"kind":"variant","value":{"type_id":"echo_sdk::FacadeStreamEvent","variant":variant,"fields":fields}})));
    }
    assert!(!validator.is_valid(&serde_json::json!({"kind":"bool","value":true})));
    assert!(!validator.is_valid(&serde_json::json!({"kind":"variant","value":{"type_id":"different","variant":"complete","fields":fields}})));
    assert!(!validator.is_valid(&serde_json::json!({"kind":"variant","value":{"type_id":"echo_sdk::FacadeStreamEvent","variant":"other","fields":fields}})));
    Ok(())
}
