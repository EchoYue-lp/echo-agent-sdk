//! Lossless conversions between framework facts and `_echo_agent/*` wire
//! DTOs, plus the single extension-error construction helpers.
//!
//! Every conversion preserves framework identity: sequences, hashes, ids and
//! failure categories travel verbatim. Errors are always built through
//! [`sdk_error`] so the fixed JSON-RPC code, the bounded message and the
//! operation/handle identity stay consistent across handlers.

use echo_agent::agent::EventEnvelope;
use echo_agent::error::{AgentFailure, ReactError};
#[cfg(feature = "sdk-facade-adapters")]
use echo_agent::error::{AgentFailureCategory, AgentTerminalKind};
use echo_agent::llm::types::{FunctionCall, Message, MessageContent, ReasoningBlock, ToolCall};
use echo_agent::runtime::{TurnDeliveryOutcome, TurnOutcome, TurnReceipt};
use echo_agent::tools::ToolResult;
use echo_sdk_protocol::error::{
    AgentFailureWire, EchoSdkError, ExtensionErrorCode, MAX_FAILURE_MESSAGE_CHARS, Retryability,
};
use echo_sdk_protocol::event::{EventWireError, WireEventEnvelope, WireEventPayload};
use echo_sdk_protocol::handle::{HandleKind, WireHandle};
use echo_sdk_protocol::methods::{
    LlmMessageWire, LlmReasoningBlockWire, RunReceiptWire, RunStatus, RunTerminal, ToolEffectWire,
    ToolResultContentWire, ToolResultKindWire, ToolResultWire,
};
#[cfg(any(
    feature = "sdk-extension-bridge",
    feature = "sdk-facade-adapters",
    test
))]
use echo_sdk_protocol::methods::{ToolFailureWire, ToolOutputArtifactRefWire};
use echo_sdk_protocol::scalar::{WireI64, WireNonZeroU64, WireTimestamp, WireU64, WireValue};

/// Wire handle of a live object issued by this Host generation.
pub(crate) fn handle(id: impl Into<String>, kind: HandleKind, generation: u64) -> WireHandle {
    WireHandle {
        id: id.into(),
        generation: WireU64::from_u64(generation),
        kind,
    }
}

/// Typed extension error with operation identity.
pub(crate) fn sdk_error(
    code: ExtensionErrorCode,
    message: impl Into<String>,
    retryable: Retryability,
    operation: &str,
) -> EchoSdkError {
    EchoSdkError::new(code, message, retryable).with_operation(operation)
}

/// Typed extension error bound to the handle it refers to.
pub(crate) fn handle_error(
    code: ExtensionErrorCode,
    message: impl Into<String>,
    operation: &str,
    handle: &WireHandle,
) -> EchoSdkError {
    EchoSdkError::new(code, message, Retryability::Never)
        .with_operation(operation)
        .with_handle(handle.clone())
}

/// Encode a validated [`EchoSdkError`] as the official JSON-RPC error.
pub(crate) fn into_jsonrpc_error(error: EchoSdkError) -> agent_client_protocol::Error {
    error.into_jsonrpc_error()
}

/// Map a framework error onto the typed extension envelope. The framework
/// message is truncated UTF-8 safely before emission; secrets never enter
/// these messages because framework errors already redact credentials.
pub(crate) fn framework_error(error: &ReactError, operation: &str) -> EchoSdkError {
    let failure = match AgentFailure::try_from(error) {
        Ok(failure) => failure,
        Err(observation) => {
            if let ReactError::ExecutionNotSettled {
                run_id,
                execution_id,
            } = &observation
            {
                return not_settled_error(run_id, execution_id.as_deref(), operation);
            }
            // A refusal is never evidence of a terminal. Keep a future unknown
            // refusal explicit rather than inventing AgentFailure fields.
            return sdk_error(
                ExtensionErrorCode::SerializationViolation,
                "framework terminal classification refused this observation",
                Retryability::Never,
                operation,
            );
        }
    };
    let projected = failure_wire(&failure);
    let code = if failure.terminal_kind == echo_agent::error::AgentTerminalKind::Cancelled {
        ExtensionErrorCode::Cancelled
    } else {
        // ExtensionTimeout describes a reverse-extension deadline. Framework
        // timeouts keep their domain and canonical terminal cause below.
        ExtensionErrorCode::FrameworkError
    };
    let mut error = sdk_error(
        code,
        bounded_message(&error.to_string()),
        // A retryable cause does not by itself authorize repeating an entire
        // operation that may already have produced effects. Preserve that
        // cause as a fact without changing the operation's retry contract.
        Retryability::Never,
        operation,
    );
    let field = |key: &str, value: String| echo_sdk_protocol::error::DetailField {
        key: key.to_string(),
        value,
    };
    let mut fields = vec![
        field("framework_failure_category", projected.category),
        field("framework_terminal_kind", projected.terminal_kind),
        field("framework_failure_code", projected.code),
        field("framework_retryable", projected.retryable.to_string()),
    ];
    if let Some(status) = projected.http_status {
        fields.push(field("framework_http_status", status.to_string()));
    }
    error.details = Some(echo_sdk_protocol::error::ErrorDetails {
        fields: Some(fields),
        ..Default::default()
    });
    error
}

/// Project observer knowledge through the existing envelope. Identities are
/// never bounded as display text: the protocol rejects an oversized value.
pub(crate) fn not_settled_error(
    run_id: &str,
    execution_id: Option<&str>,
    operation: &str,
) -> EchoSdkError {
    let mut fields = vec![
        echo_sdk_protocol::error::DetailField {
            key: "observation".to_string(),
            value: "not_settled".to_string(),
        },
        echo_sdk_protocol::error::DetailField {
            key: "run_id".to_string(),
            value: run_id.to_string(),
        },
    ];
    if let Some(execution_id) = execution_id {
        fields.push(echo_sdk_protocol::error::DetailField {
            key: "execution_id".to_string(),
            value: execution_id.to_string(),
        });
    }
    let mut error = sdk_error(
        ExtensionErrorCode::FrameworkError,
        "original execution has not physically settled",
        Retryability::Never,
        operation,
    );
    error.details = Some(echo_sdk_protocol::error::ErrorDetails {
        fields: Some(fields),
        ..Default::default()
    });
    error
}

/// A hint fences the original owner before validation; it is not a trusted
/// identity and cannot authorize a terminal or resource release.
#[cfg(any(feature = "sdk-facade-adapters", test))]
pub(crate) fn has_observation_hint(error: &EchoSdkError) -> bool {
    error
        .details
        .as_ref()
        .and_then(|details| details.fields.as_ref())
        .is_some_and(|fields| {
            fields
                .iter()
                .any(|field| field.key == "observation" && field.value == "not_settled")
        })
}

#[cfg(any(feature = "sdk-extension-bridge", test))]
pub(crate) fn observation_from_sdk_error(error: &EchoSdkError) -> Option<ReactError> {
    error.validate().ok()?;
    if error.code != ExtensionErrorCode::FrameworkError || error.retryable != Retryability::Never {
        return None;
    }
    let fields = error.details.as_ref()?.fields.as_ref()?;
    let value = |key: &str| {
        fields
            .iter()
            .find(|field| field.key == key)
            .map(|field| field.value.as_str())
    };
    if value("observation")? != "not_settled" {
        return None;
    }
    let run_id = value("run_id")?;
    if run_id.trim().is_empty() {
        return None;
    }
    let execution_id = value("execution_id");
    if execution_id.is_some_and(|value| value.trim().is_empty()) {
        return None;
    }
    Some(ReactError::ExecutionNotSettled {
        run_id: run_id.to_string(),
        execution_id: execution_id.map(str::to_string),
    })
}

#[allow(dead_code)]
pub(crate) fn bounded_framework_message(message: &str) -> String {
    bounded_message(message)
}

pub(crate) fn bounded_message(message: &str) -> String {
    const MAX_MESSAGE_CHARS: usize = 2048;
    message.chars().take(MAX_MESSAGE_CHARS).collect()
}

/// Restore tool facts without discarding failure, artifact or model payloads.
#[cfg_attr(not(feature = "sdk-extension-bridge"), allow(dead_code))]
pub(crate) fn tool_result_from_wire(
    wire: ToolResultWire,
) -> std::result::Result<ToolResult, String> {
    use echo_agent::tools::{
        ToolFailure, ToolFailureCategory, ToolRecoveryAction, ToolResultContent, ToolResultKind,
        ToolSideEffect,
    };
    let kind = match wire.kind {
        ToolResultKindWire::Text => ToolResultKind::Text,
        ToolResultKindWire::Json => ToolResultKind::Json,
        ToolResultKindWire::Image { mime_type } => ToolResultKind::Image { mime_type },
        ToolResultKindWire::Table { columns, rows } => ToolResultKind::Table { columns, rows },
        ToolResultKindWire::Diff { unified_diff } => ToolResultKind::Diff { unified_diff },
        ToolResultKindWire::FileReference { path } => ToolResultKind::FileReference { path },
        ToolResultKindWire::CommandOutput { exit_code } => {
            ToolResultKind::CommandOutput { exit_code }
        }
        ToolResultKindWire::SkillActivation { name } => ToolResultKind::SkillActivation { name },
        ToolResultKindWire::StructuredError { error_code } => {
            ToolResultKind::StructuredError { error_code }
        }
    };
    let failure = wire
        .failure
        .map(|failure| -> std::result::Result<ToolFailure, String> {
            let category = match failure.category.as_str() {
                "invalid_arguments" => Ok(ToolFailureCategory::InvalidArguments),
                "unavailable" => Ok(ToolFailureCategory::Unavailable),
                "timeout" => Ok(ToolFailureCategory::Timeout),
                "cancelled" => Ok(ToolFailureCategory::Cancelled),
                "transient" => Ok(ToolFailureCategory::Transient),
                "permanent" => Ok(ToolFailureCategory::Permanent),
                "partial_side_effect" => Ok(ToolFailureCategory::PartialSideEffect),
                _ => Err("unknown tool failure category".to_string()),
            }?;
            let recovery = match failure.recovery.as_str() {
                "correct_arguments" => Ok(ToolRecoveryAction::CorrectArguments),
                "retry" => Ok(ToolRecoveryAction::Retry),
                "restore_then_retry" => Ok(ToolRecoveryAction::RestoreThenRetry),
                "verify_then_retry" => Ok(ToolRecoveryAction::VerifyThenRetry),
                "stop" => Ok(ToolRecoveryAction::Stop),
                _ => Err("unknown tool recovery action".to_string()),
            }?;
            let side_effect = match failure.side_effect.as_str() {
                "none" => Ok(ToolSideEffect::None),
                "possible" => Ok(ToolSideEffect::Possible),
                "confirmed" => Ok(ToolSideEffect::Confirmed),
                _ => Err("unknown tool side-effect status".to_string()),
            }?;
            Ok(ToolFailure {
                category,
                recovery,
                side_effect,
                retry_after_ms: failure
                    .retry_after_ms
                    .map(|value| {
                        value
                            .to_u64()
                            .ok_or_else(|| "invalid retry_after_ms".to_string())
                    })
                    .transpose()?,
                idempotency_key: failure.idempotency_key,
                postcondition: failure.postcondition,
            })
        })
        .transpose()?;
    let artifact =
        wire.artifact
            .map(
                |artifact| -> std::result::Result<
                    echo_agent::tools::artifact::ToolOutputArtifactRef,
                    String,
                > {
                    Ok(echo_agent::tools::artifact::ToolOutputArtifactRef {
                        path: path_from_wire(&artifact.path)?,
                        artifact_bytes: artifact
                            .artifact_bytes
                            .to_u64()
                            .ok_or_else(|| "invalid artifact byte count".to_string())?,
                        payload_bytes: artifact
                            .payload_bytes
                            .to_u64()
                            .ok_or_else(|| "invalid payload byte count".to_string())?,
                        sha256: artifact.sha256,
                        retention: artifact.retention,
                    })
                },
            )
            .transpose()?;
    Ok(ToolResult {
        kind,
        success: wire.success,
        output: wire.output,
        error: wire.error,
        failure,
        data: wire
            .data
            .map(|value| value.into_json().map_err(|error| error.to_string()))
            .transpose()?,
        truncated: wire.truncated,
        mime_type: wire.mime_type,
        artifact,
        effects: wire
            .effects
            .into_iter()
            .map(|effect| {
                use echo_agent::tools::ToolEffect;
                Ok(match effect {
                    ToolEffectWire::FileRead { path } => ToolEffect::FileRead { path },
                    ToolEffectWire::FileEdit { path } => ToolEffect::FileEdit { path },
                    ToolEffectWire::TestRun {
                        command,
                        passed,
                        failure_count,
                    } => ToolEffect::TestRun {
                        command,
                        passed,
                        failure_count: failure_count
                            .map(|count| {
                                count
                                    .to_u64()
                                    .and_then(|value| usize::try_from(value).ok())
                                    .ok_or_else(|| {
                                        "tool failure_count exceeds platform range".to_string()
                                    })
                            })
                            .transpose()?,
                    },
                    ToolEffectWire::SubagentRun {
                        agent_name,
                        task,
                        outcome,
                    } => ToolEffect::SubagentRun {
                        agent_name,
                        task,
                        outcome,
                    },
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        metadata: wire.metadata.into_iter().collect(),
        model_content: wire
            .model_content
            .into_iter()
            .map(|content| match content {
                ToolResultContentWire::ImageUrl { url, detail } => {
                    ToolResultContent::ImageUrl { url, detail }
                }
            })
            .collect(),
    })
}

/// Share one complete tool projection across core and reverse-extension paths.
#[cfg(any(
    feature = "sdk-extension-bridge",
    feature = "sdk-facade-adapters",
    test
))]
pub(crate) fn tool_result_to_wire(
    result: &ToolResult,
) -> std::result::Result<ToolResultWire, String> {
    use echo_agent::tools::ToolResultKind;
    let kind = match &result.kind {
        ToolResultKind::Text => ToolResultKindWire::Text,
        ToolResultKind::Json => ToolResultKindWire::Json,
        ToolResultKind::Image { mime_type } => ToolResultKindWire::Image {
            mime_type: mime_type.clone(),
        },
        ToolResultKind::Table { columns, rows } => ToolResultKindWire::Table {
            columns: columns.clone(),
            rows: rows.clone(),
        },
        ToolResultKind::Diff { unified_diff } => ToolResultKindWire::Diff {
            unified_diff: unified_diff.clone(),
        },
        ToolResultKind::FileReference { path } => {
            ToolResultKindWire::FileReference { path: path.clone() }
        }
        ToolResultKind::CommandOutput { exit_code } => ToolResultKindWire::CommandOutput {
            exit_code: *exit_code,
        },
        ToolResultKind::SkillActivation { name } => {
            ToolResultKindWire::SkillActivation { name: name.clone() }
        }
        ToolResultKind::StructuredError { error_code } => ToolResultKindWire::StructuredError {
            error_code: error_code.clone(),
        },
    };
    let failure = result.failure.as_ref().map(|failure| ToolFailureWire {
        category: failure.category.as_str().to_string(),
        recovery: failure.recovery.as_str().to_string(),
        side_effect: match failure.side_effect {
            echo_agent::tools::ToolSideEffect::None => "none",
            echo_agent::tools::ToolSideEffect::Possible => "possible",
            echo_agent::tools::ToolSideEffect::Confirmed => "confirmed",
        }
        .to_string(),
        retry_after_ms: failure.retry_after_ms.map(WireU64::from_u64),
        idempotency_key: failure.idempotency_key.clone(),
        postcondition: failure.postcondition.clone(),
    });
    let artifact = result
        .artifact
        .as_ref()
        .map(|artifact| -> Result<ToolOutputArtifactRefWire, String> {
            Ok(ToolOutputArtifactRefWire {
                path: path_to_wire(&artifact.path)?,
                artifact_bytes: WireU64::from_u64(artifact.artifact_bytes),
                payload_bytes: WireU64::from_u64(artifact.payload_bytes),
                sha256: artifact.sha256.clone(),
                retention: artifact.retention.clone(),
            })
        })
        .transpose()?;
    let model_content = result
        .model_content
        .iter()
        .map(|content| match content {
            echo_agent::tools::ToolResultContent::ImageUrl { url, detail } => {
                ToolResultContentWire::ImageUrl {
                    url: url.clone(),
                    detail: detail.clone(),
                }
            }
        })
        .collect();
    Ok(ToolResultWire {
        kind,
        success: result.success,
        output: result.output.clone(),
        error: result.error.clone(),
        failure,
        data: result
            .data
            .as_ref()
            .map(|data| WireValue::from_json(data.clone()).map_err(|error| error.to_string()))
            .transpose()?,
        truncated: result.truncated,
        mime_type: result.mime_type.clone(),
        artifact,
        effects: result
            .effects
            .iter()
            .map(|effect| {
                use echo_agent::tools::ToolEffect;
                Ok(match effect {
                    ToolEffect::FileRead { path } => {
                        ToolEffectWire::FileRead { path: path.clone() }
                    }
                    ToolEffect::FileEdit { path } => {
                        ToolEffectWire::FileEdit { path: path.clone() }
                    }
                    ToolEffect::TestRun {
                        command,
                        passed,
                        failure_count,
                    } => ToolEffectWire::TestRun {
                        command: command.clone(),
                        passed: *passed,
                        failure_count: failure_count
                            .map(|count| {
                                u64::try_from(count).map(WireU64::from_u64).map_err(|_| {
                                    "tool failure_count exceeds wire range".to_string()
                                })
                            })
                            .transpose()?,
                    },
                    ToolEffect::SubagentRun {
                        agent_name,
                        task,
                        outcome,
                    } => ToolEffectWire::SubagentRun {
                        agent_name: agent_name.clone(),
                        task: task.clone(),
                        outcome: outcome.clone(),
                    },
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        metadata: result
            .metadata
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
        model_content,
    })
}

/// Project one framework failure into its lossless wire record. This adapter
/// belongs to the Host so the protocol crate never depends on framework types.
pub(crate) fn failure_wire(failure: &AgentFailure) -> AgentFailureWire {
    fn serde_name(value: &impl serde::Serialize) -> String {
        serde_json::to_value(value)
            .ok()
            .and_then(|value| value.as_str().map(str::to_string))
            .unwrap_or_default()
    }

    AgentFailureWire {
        category: serde_name(&failure.category),
        terminal_kind: serde_name(&failure.terminal_kind),
        retryable: failure.retryable,
        code: failure.code.chars().take(128).collect(),
        http_status: failure.http_status,
        message: failure
            .message
            .chars()
            .take(MAX_FAILURE_MESSAGE_CHARS)
            .collect(),
    }
}

/// Restore a framework failure from the closed wire value without flattening
/// category, terminal kind, retryability, code, status, or message.
#[cfg(feature = "sdk-facade-adapters")]
pub(crate) fn failure_from_wire(wire: AgentFailureWire) -> Result<AgentFailure, String> {
    let category = match wire.category.as_str() {
        "llm" => AgentFailureCategory::Llm,
        "tool" => AgentFailureCategory::Tool,
        "parse" => AgentFailureCategory::Parse,
        "agent" => AgentFailureCategory::Agent,
        "config" => AgentFailureCategory::Config,
        "mcp" => AgentFailureCategory::Mcp,
        "memory" => AgentFailureCategory::Memory,
        "sandbox" => AgentFailureCategory::Sandbox,
        "runtime_state" => AgentFailureCategory::RuntimeState,
        "channel" => AgentFailureCategory::Channel,
        "io" => AgentFailureCategory::Io,
        "other" => AgentFailureCategory::Other,
        value => return Err(format!("unknown AgentFailure category {value}")),
    };
    let terminal_kind = match wire.terminal_kind.as_str() {
        "failed" => AgentTerminalKind::Failed,
        "cancelled" => AgentTerminalKind::Cancelled,
        "timed_out" => AgentTerminalKind::TimedOut,
        "permission_denied" => AgentTerminalKind::PermissionDenied,
        value => return Err(format!("unknown AgentFailure terminal_kind {value}")),
    };
    wire.validate()
        .map_err(|error| format!("invalid AgentFailureWire: {error}"))?;
    if wire.code.trim().is_empty() {
        return Err("AgentFailure code must be non-empty".to_string());
    }
    if wire.message.trim().is_empty() {
        return Err("AgentFailure message must be non-empty".to_string());
    }
    Ok(AgentFailure {
        category,
        terminal_kind,
        retryable: wire.retryable,
        code: wire.code,
        http_status: wire.http_status,
        message: wire.message,
    })
}

/// Convert the shared lossless Message wire DTO into the framework Message
/// used by TurnRequest::from_message. This lives outside the extension bridge
/// so core source-operation streams can consume the same DTO even when no
/// reverse extension feature is enabled.
pub(crate) fn message_from_wire(message: LlmMessageWire) -> Result<Message, String> {
    let content = message
        .content
        .into_json()
        .map_err(|error| error.to_string())
        .and_then(|value| {
            serde_json::from_value::<MessageContent>(value).map_err(|error| error.to_string())
        })?;
    let tool_calls = message.tool_calls.map(|calls| {
        calls
            .into_iter()
            .map(|call| ToolCall {
                id: call.id,
                call_type: call.call_type,
                function: FunctionCall {
                    name: call.function_name,
                    arguments: call.arguments,
                },
            })
            .collect::<Vec<_>>()
    });
    let reasoning_blocks = message.reasoning_blocks.map(|blocks| {
        blocks
            .into_iter()
            .map(|block| match block {
                LlmReasoningBlockWire::Signed {
                    thinking,
                    signature,
                } => ReasoningBlock::Signed {
                    thinking,
                    signature,
                },
                LlmReasoningBlockWire::Redacted { data } => ReasoningBlock::Redacted { data },
                LlmReasoningBlockWire::Opaque {
                    provider,
                    id,
                    data,
                    summary,
                } => ReasoningBlock::Opaque {
                    provider,
                    id,
                    data,
                    summary,
                },
            })
            .collect::<Vec<_>>()
    });
    Ok(Message {
        role: message.role.as_str().into(),
        content,
        tool_calls,
        name: message.name,
        tool_call_id: message.tool_call_id,
        reasoning_content: message.reasoning_content,
        reasoning_blocks,
    })
}

/// The single authoritative terminal projected from a framework receipt.
/// There is deliberately no `interrupted` mapping here: interruption is a
/// run status without a terminal, so a crashed run can never be mistaken
/// for success (design §15).
pub(crate) fn terminal_of(receipt: &TurnReceipt) -> std::result::Result<RunTerminal, String> {
    let terminal = match &receipt.outcome {
        TurnOutcome::Completed => RunTerminal::Completed {
            final_answer: receipt.final_answer.clone(),
        },
        TurnOutcome::Cancelled => RunTerminal::Cancelled,
        TurnOutcome::Failed(failure) => RunTerminal::Failed {
            failure: failure_wire(failure),
        },
    };
    terminal.validate().map_err(|error| error.to_string())?;
    Ok(terminal)
}

/// Bounded receipt projection; counters stay lossless integer strings.
pub(crate) fn receipt_wire(receipt: &TurnReceipt) -> std::result::Result<RunReceiptWire, String> {
    let delivery = match &receipt.delivery {
        TurnDeliveryOutcome::NotAttempted => "not_attempted",
        TurnDeliveryOutcome::Delivered => "delivered",
        TurnDeliveryOutcome::Closed => "closed",
        TurnDeliveryOutcome::Failed(_) => "failed",
    };
    let wire = RunReceiptWire {
        turn_id: receipt.turn_id.as_str().to_string(),
        outcome: receipt.status().to_string(),
        delivery: Some(delivery.to_string()),
        delivery_error: match &receipt.delivery {
            TurnDeliveryOutcome::Failed(failure) => Some(failure_wire(failure)),
            _ => None,
        },
        final_answer: receipt.final_answer.clone(),
        final_message_id: receipt
            .final_message_id
            .as_ref()
            .map(|id| id.as_str().to_string()),
        prompt_tokens: WireU64::from_u64(receipt.prompt_tokens),
        completion_tokens: WireU64::from_u64(receipt.completion_tokens),
        llm_calls: WireU64::from_u64(receipt.llm_calls),
        compaction_count: WireU64::from_u64(receipt.compaction_count),
        last_event_sequence: WireU64::from_u64(receipt.last_event_sequence),
        elapsed_ms: WireU64::from_u64(
            u64::try_from(receipt.elapsed.as_millis()).unwrap_or(u64::MAX),
        ),
    };
    wire.validate().map_err(|error| error.to_string())?;
    Ok(wire)
}

/// Live run status from a framework receipt (None = still running).
pub(crate) fn status_of(receipt: Option<&TurnReceipt>) -> RunStatus {
    match receipt {
        None => RunStatus::Running,
        Some(receipt) => match &receipt.outcome {
            TurnOutcome::Completed => RunStatus::Completed,
            TurnOutcome::Cancelled => RunStatus::Cancelled,
            TurnOutcome::Failed(_) => RunStatus::Failed,
        },
    }
}

/// Full `EventEnvelope` projection. Identity fields, sequence, content hash
/// and payload are preserved verbatim; conversion failure is surfaced as a
/// delivery error instead of shipping a lossy event.
pub(crate) fn wire_envelope(envelope: &EventEnvelope) -> Result<WireEventEnvelope, EchoSdkError> {
    project_event_envelope(envelope).map_err(|error: EventWireError| {
        sdk_error(
            ExtensionErrorCode::SerializationViolation,
            format!("failed to project framework event: {error}"),
            Retryability::Never,
            "_echo_agent/event",
        )
    })
}

fn project_event_envelope(envelope: &EventEnvelope) -> Result<WireEventEnvelope, EventWireError> {
    let payload_value = serde_json::to_value(&envelope.payload)
        .map_err(|error| EventWireError::InvalidPayload(error.to_string()))?;
    let mut payload_object = payload_value
        .as_object()
        .cloned()
        .ok_or_else(|| EventWireError::InvalidPayload("expected tagged object".to_string()))?;
    let event_type = payload_object
        .remove("type")
        .and_then(|value| value.as_str().map(str::to_string))
        .ok_or_else(|| EventWireError::InvalidPayload("missing type tag".to_string()))?;
    let data = payload_object
        .remove("data")
        .map(WireValue::from_json)
        .transpose()?;
    if !payload_object.is_empty() {
        return Err(EventWireError::InvalidPayload(
            "unexpected fields outside type/data".to_string(),
        ));
    }
    let wire = WireEventEnvelope {
        schema_version: envelope.schema_version,
        event_id: envelope.event_id.as_str().to_string(),
        content_hash: envelope.content_hash.clone(),
        sequence: WireNonZeroU64::try_from(envelope.sequence.to_string())?,
        stream_id: envelope.stream_id.as_str().to_string(),
        conversation_id: envelope
            .conversation_id
            .as_ref()
            .map(|value| value.as_str().to_string()),
        run_id: envelope
            .run_id
            .as_ref()
            .map(|value| value.as_str().to_string()),
        turn_id: envelope.turn_id.as_str().to_string(),
        message_id: envelope
            .message_id
            .as_ref()
            .map(|value| value.as_str().to_string()),
        execution_id: envelope
            .execution_id
            .as_ref()
            .map(|value| value.as_str().to_string()),
        parent_event_id: envelope
            .parent_event_id
            .as_ref()
            .map(|value| value.as_str().to_string()),
        timestamp: WireTimestamp {
            unix_seconds: WireI64::from_i64(envelope.timestamp.timestamp()),
            nanos: envelope.timestamp.timestamp_subsec_nanos(),
            rfc3339: Some(envelope.timestamp.to_rfc3339()),
        },
        payload: WireEventPayload { event_type, data },
    };
    wire.validate()?;
    Ok(wire)
}

// ── Wire → framework conversions ───────────────────────────────────────────

/// Convert a wire working directory into a native path. UTF-8 paths convert
/// losslessly; encoded Unix-bytes/Windows-UTF-16 paths convert through the
/// OS encoding and fail closed with a typed error when impossible.
pub(crate) fn working_dir_from_wire(
    path: Option<&echo_sdk_protocol::scalar::WirePath>,
) -> std::result::Result<Option<std::path::PathBuf>, EchoSdkError> {
    let Some(path) = path else {
        return Ok(None);
    };
    to_path_buf(path).map(Some).map_err(|error| {
        sdk_error(
            ExtensionErrorCode::InvalidValue,
            format!("working_dir cannot be represented on this platform: {error}"),
            Retryability::Never,
            "_echo_agent/session",
        )
    })
}

/// Encode a native absolute path without lossy UTF-8 conversion. Unix paths
/// that are not valid UTF-8 use the byte-preserving wire variant; Windows
/// paths use UTF-16 code units as required by the protocol.
pub(crate) fn path_to_wire(
    path: &std::path::Path,
) -> std::result::Result<echo_sdk_protocol::scalar::WirePath, String> {
    use base64::Engine as _;
    use echo_sdk_protocol::scalar::WirePath;
    if !path.is_absolute() {
        return Err("path must be absolute".to_string());
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;
        let bytes = path.as_os_str().as_bytes();
        if let Some(text) = path.to_str() {
            return Ok(WirePath::Utf8 {
                path: text.to_string(),
            });
        }
        Ok(WirePath::Unix {
            bytes_base64: base64::engine::general_purpose::STANDARD_NO_PAD.encode(bytes),
            display: None,
        })
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;
        let units: Vec<u16> = path.as_os_str().encode_wide().collect();
        let mut bytes = Vec::with_capacity(units.len().saturating_mul(2));
        for unit in units {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        Ok(WirePath::Windows {
            utf16_base64: base64::engine::general_purpose::STANDARD_NO_PAD.encode(bytes),
            display: path.to_str().map(str::to_string),
        })
    }
    #[cfg(not(any(unix, windows)))]
    {
        Err("native paths are unsupported on this platform".to_string())
    }
}

pub(crate) fn path_from_wire(
    path: &echo_sdk_protocol::scalar::WirePath,
) -> std::result::Result<std::path::PathBuf, String> {
    path.validate()
        .map_err(|error| format!("invalid wire path: {error}"))?;
    to_path_buf(path)
}

/// Lossless-enough native conversion of a [`WirePath`]. UTF-8 paths convert
/// directly; Unix byte paths must be valid WTF-8 on this platform; Windows
/// UTF-16 paths only convert on Windows. Anything else fails closed with a
/// typed error instead of losing bytes.
fn to_path_buf(
    path: &echo_sdk_protocol::scalar::WirePath,
) -> std::result::Result<std::path::PathBuf, String> {
    use base64::Engine as _;
    use echo_sdk_protocol::scalar::WirePath;
    match path {
        WirePath::Utf8 { path } => Ok(std::path::PathBuf::from(path)),
        WirePath::Unix { bytes_base64, .. } => {
            let bytes = base64::engine::general_purpose::STANDARD_NO_PAD
                .decode(bytes_base64)
                .map_err(|error| format!("invalid base64 path encoding: {error}"))?;
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStringExt as _;
                Ok(std::path::PathBuf::from(std::ffi::OsString::from_vec(
                    bytes,
                )))
            }
            #[cfg(not(unix))]
            {
                let _ = bytes;
                Err("Unix byte paths require a Unix platform".to_string())
            }
        }
        WirePath::Windows { utf16_base64, .. } => {
            let bytes = base64::engine::general_purpose::STANDARD_NO_PAD
                .decode(utf16_base64)
                .map_err(|error| format!("invalid base64 path encoding: {error}"))?;
            #[cfg(windows)]
            {
                use std::os::windows::ffi::OsStringExt as _;
                let units: Vec<u16> = bytes
                    .chunks_exact(2)
                    .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                    .collect();
                Ok(std::path::PathBuf::from(std::ffi::OsString::from_wide(
                    &units,
                )))
            }
            #[cfg(not(windows))]
            {
                let _ = bytes;
                Err("Windows UTF-16 paths require a Windows platform".to_string())
            }
        }
    }
}

/// Build the framework model+agent config and its LLM client from the
/// explicit wire projection. Exactly one credential source arrives by
/// grammar; environment credentials are resolved here so secrets never sit
/// in the framework config longer than needed.
pub(crate) fn framework_from_wire(
    config: &echo_sdk_protocol::methods::AgentConfigExplicitWire,
) -> std::result::Result<
    (
        echo_agent::config::FrameworkConfig,
        std::sync::Arc<dyn echo_agent::llm::LlmClient>,
        echo_agent::llm::LlmConfig,
    ),
    String,
> {
    use echo_agent::config::{AgentSettings, FrameworkConfig, ModelConfig};
    use echo_agent::llm::{LlmApiProtocol, LlmConfig};
    use echo_sdk_protocol::methods::{CredentialSourceWire, LlmApiProtocolWire};

    const SUPPORTED_CONFIG_VERSION: u32 = 1;
    if config.config_version != SUPPORTED_CONFIG_VERSION {
        return Err(format!(
            "unsupported agent config_version {}; expected {SUPPORTED_CONFIG_VERSION}",
            config.config_version
        ));
    }
    let api_protocol = match config.model.api_protocol {
        LlmApiProtocolWire::ChatCompletions => LlmApiProtocol::ChatCompletions,
        LlmApiProtocolWire::Responses => LlmApiProtocol::Responses,
        LlmApiProtocolWire::Anthropic => LlmApiProtocol::Anthropic,
    };
    let credential_token = match &config.model.credential {
        None => None,
        Some(CredentialSourceWire::Inline { token }) => Some(token.clone()),
        Some(CredentialSourceWire::Env { variable }) => {
            Some(std::env::var(variable).map_err(|_| {
                format!("credential environment variable {variable} is unavailable")
            })?)
        }
    };
    let temperature = match &config.model.temperature {
        None => None,
        Some(text) => Some(
            text.parse::<f32>()
                .map_err(|_| format!("temperature must be a decimal number, got {text:?}"))?,
        ),
    };
    let context_window = match &config.model.context_window {
        None => None,
        Some(value) => {
            let raw = value.to_u64().ok_or("context_window is out of range")?;
            Some(u32::try_from(raw).map_err(|_| "context_window exceeds u32".to_string())?)
        }
    };
    let base_url = config.model.base_url.clone();
    let model = ModelConfig {
        provider: config.model.provider.clone(),
        name: config.model.name.clone(),
        // Public descriptive config remains scrubbed. Only the private
        // original LlmConfig below retains this client's construction payload;
        // do not reconstruct it from public snapshots or reread the environment.
        auth_token: None,
        base_url: Some(base_url.clone()),
        api_protocol: Some(api_protocol),
        max_tokens: config.model.max_tokens,
        temperature,
        context_window,
    };
    let agent = AgentSettings {
        name: config.agent.name.clone(),
        system_prompt: config.agent.system_prompt.clone(),
        max_iterations: usize::try_from(config.agent.max_iterations)
            .map_err(|_| "max_iterations exceeds the platform width".to_string())?,
        enable_tools: true,
        ..AgentSettings::default()
    };
    let llm_config = LlmConfig::for_provider(
        &config.model.provider,
        &base_url,
        credential_token.as_deref().unwrap_or(""),
        &config.model.name,
        api_protocol,
    )
    .map_err(|error| format!("failed to build model client: {error}"))?;
    let llm_client: std::sync::Arc<dyn echo_agent::llm::LlmClient> = std::sync::Arc::from(
        llm_config
            .build_client()
            .map_err(|error| format!("failed to build model client: {error}"))?,
    );
    Ok((FrameworkConfig { model, agent }, llm_client, llm_config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonterminal_observation_preserves_exact_identity_and_has_no_failure_fields()
    -> Result<(), String> {
        for execution_id in [None, Some("原执行🙂".repeat(120))] {
            let run_id = "原run🙂".repeat(120);
            let source = ReactError::ExecutionNotSettled {
                run_id: run_id.clone(),
                execution_id: execution_id.clone(),
            };
            let error = framework_error(&source, "_echo_agent/subagent/await");
            let restored =
                EchoSdkError::from_jsonrpc_data(error.clone().into_jsonrpc_error().data.as_ref())?;
            assert_eq!(restored, error);
            assert_eq!(restored.code, ExtensionErrorCode::FrameworkError);
            assert_eq!(restored.retryable, Retryability::Never);
            assert!(has_observation_hint(&restored));
            assert!(
                matches!(observation_from_sdk_error(&restored), Some(ReactError::ExecutionNotSettled { run_id: actual, execution_id: actual_execution }) if actual == run_id && actual_execution == execution_id)
            );
            assert!(
                !restored
                    .details
                    .as_ref()
                    .and_then(|details| details.fields.as_ref())
                    .is_some_and(|fields| fields
                        .iter()
                        .any(|field| field.key.starts_with("framework_terminal")))
            );
        }
        let oversized = not_settled_error(&"🙂".repeat(2049), None, "_echo_agent/subagent/await");
        assert!(has_observation_hint(&oversized));
        assert!(observation_from_sdk_error(&oversized).is_none());
        assert_eq!(
            EchoSdkError::from_jsonrpc_data(oversized.into_jsonrpc_error().data.as_ref())?.code,
            ExtensionErrorCode::SerializationViolation
        );
        Ok(())
    }

    #[test]
    fn framework_error_preserves_cause_without_reclassifying_reverse_deadlines()
    -> Result<(), String> {
        use echo_agent::error::{AgentError, LlmError, MemoryError};
        let cases = [
            (
                ReactError::from(AgentError::Cancelled("中文🙂 stop".to_string())),
                ExtensionErrorCode::Cancelled,
            ),
            (
                ReactError::from(AgentError::Timeout("deadline".to_string())),
                ExtensionErrorCode::FrameworkError,
            ),
            (
                ReactError::from(LlmError::ApiError {
                    status: 429,
                    message: "rate limited".to_string(),
                }),
                ExtensionErrorCode::FrameworkError,
            ),
            (
                ReactError::from(LlmError::ApiError {
                    status: 403,
                    message: "forbidden".to_string(),
                }),
                ExtensionErrorCode::FrameworkError,
            ),
            (
                ReactError::from(MemoryError::DeadlineExceeded("store deadline".to_string())),
                ExtensionErrorCode::FrameworkError,
            ),
            (
                ReactError::Other("cancelled timeout 429 are display text".to_string()),
                ExtensionErrorCode::FrameworkError,
            ),
        ];
        for (source, expected_code) in cases {
            let expected =
                failure_wire(&AgentFailure::try_from(&source).map_err(|error| error.to_string())?);
            let bound_handle = handle("subagent-owner", HandleKind::Subagent, 9);
            let wire = framework_error(&source, "_echo_agent/subagent/await")
                .with_handle(bound_handle.clone());
            wire.validate().map_err(str::to_string)?;
            assert_eq!(wire.code, expected_code);
            assert_ne!(wire.code, ExtensionErrorCode::ExtensionTimeout);
            assert_eq!(wire.retryable, Retryability::Never);
            let encoded = wire.into_jsonrpc_error();
            let restored = EchoSdkError::from_jsonrpc_data(encoded.data.as_ref())?;
            assert_eq!(
                restored.operation.as_deref(),
                Some("_echo_agent/subagent/await")
            );
            assert_eq!(restored.handle, Some(bound_handle));
            let fields = restored
                .details
                .and_then(|details| details.fields)
                .ok_or("missing cause fields")?;
            let find = |key| {
                fields
                    .iter()
                    .find(|field| field.key == key)
                    .map(|field| field.value.as_str())
            };
            assert_eq!(
                find("framework_failure_category"),
                Some(expected.category.as_str())
            );
            assert_eq!(
                find("framework_terminal_kind"),
                Some(expected.terminal_kind.as_str())
            );
            assert_eq!(find("framework_failure_code"), Some(expected.code.as_str()));
            assert_eq!(
                find("framework_retryable"),
                Some(if expected.retryable { "true" } else { "false" })
            );
            assert_eq!(
                find("framework_http_status").map(str::to_string),
                expected.http_status.map(|status| status.to_string())
            );
        }
        Ok(())
    }
    use echo_agent::agent::{AgentEvent, EventIdentity};

    #[test]
    fn terminal_projects_cancelled() -> Result<(), ReactError> {
        let cancelled = TurnReceipt::cancelled("turn-wire-test")
            .map_err(|error| ReactError::Other(error.to_string()))?;
        assert!(matches!(
            terminal_of(&cancelled),
            Ok(RunTerminal::Cancelled)
        ));
        assert_eq!(status_of(Some(&cancelled)), RunStatus::Cancelled);
        assert_eq!(status_of(None), RunStatus::Running);
        let receipt = receipt_wire(&cancelled).map_err(ReactError::Other)?;
        assert_eq!(receipt.outcome, "cancelled");
        assert_eq!(receipt.delivery.as_deref(), Some("not_attempted"));
        Ok(())
    }

    #[tokio::test]
    async fn wire_envelope_preserves_identity() -> Result<(), ReactError> {
        let identity = EventIdentity::new("stream-wire", "turn-wire")?;
        let envelope = EventEnvelope::new(&identity, 1, None, AgentEvent::Token("你好".into()))?;
        let wire = wire_envelope(&envelope).map_err(|error| ReactError::Other(error.message))?;
        assert_eq!(wire.sequence.to_u64(), Some(1));
        assert_eq!(wire.stream_id, "stream-wire");
        assert_eq!(wire.payload.event_type, "token");
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn unix_non_utf8_paths_round_trip_without_loss() -> Result<(), ReactError> {
        use std::os::unix::ffi::{OsStrExt as _, OsStringExt as _};
        let path =
            std::path::PathBuf::from(std::ffi::OsString::from_vec(b"/tmp/sdk-\xff".to_vec()));
        let wire = path_to_wire(&path).map_err(ReactError::Other)?;
        let restored = path_from_wire(&wire).map_err(ReactError::Other)?;
        assert_eq!(restored.as_os_str().as_bytes(), path.as_os_str().as_bytes());
        Ok(())
    }

    #[cfg(feature = "sdk-facade-adapters")]
    #[test]
    fn managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds()
    -> Result<(), String> {
        use super::managed::Projection as _;
        let mut checkpoint = echo_agent::state::AgentCheckpoint::new("中文🙂-generation");
        checkpoint.messages_json =
            r#"{"schema_version":2,"messages":[],"transcript_projection":null}"#.to_string();
        checkpoint.current_plan = Some("artifact-only".to_string());
        checkpoint.active_skills = vec!["技能".to_string()];
        checkpoint.blocked_reason = Some("pending receipt".to_string());
        checkpoint.timestamp =
            chrono::DateTime::from_timestamp(-1, 123_456_789).ok_or("invalid fixture timestamp")?;
        let wire = checkpoint.clone().project()?;
        assert_eq!(wire.timestamp.unix_seconds.to_i64(), Some(-1));
        assert_eq!(wire.timestamp.nanos, 123_456_789);
        let restored = echo_agent::state::AgentCheckpoint::restore(wire)?;
        assert_eq!(
            serde_json::to_value(restored).map_err(|error| error.to_string())?,
            serde_json::to_value(checkpoint).map_err(|error| error.to_string())?
        );
        let receipt = echo_agent::state::RuntimeCheckpointCasReceipt {
            scope: echo_agent::state::RuntimeScopeAuthority {
                scope_id: "中文🙂".to_string(),
                conversation_epoch: Some(u64::MAX),
                revision: u64::MAX,
                lifecycle: echo_agent::state::RuntimeScopeLifecycle::Active,
            },
            runtime_state_id: "generation".to_string(),
            version: echo_agent::state::RuntimeStateVersion::Managed { revision: u64::MAX },
            status: echo_agent::state::RuntimeCheckpointCasStatus::RevisionConflict,
        };
        let projected = receipt.clone().project()?;
        assert_eq!(projected.scope.revision.as_str(), "18446744073709551615");
        let restored = echo_agent::state::RuntimeCheckpointCasReceipt::restore(projected)?;
        assert_eq!(
            serde_json::to_value(restored).map_err(|error| error.to_string())?,
            serde_json::to_value(receipt).map_err(|error| error.to_string())?
        );
        Ok(())
    }

    #[test]
    fn tool_effects_and_model_content_survive_shared_projection() -> Result<(), String> {
        use echo_agent::tools::{ToolEffect, ToolResultContent};
        let mut result = ToolResult::success("中文🙂");
        result.effects = vec![
            ToolEffect::FileRead {
                path: "文件.rs".to_string(),
            },
            ToolEffect::FileEdit {
                path: "文件.rs".to_string(),
            },
            ToolEffect::TestRun {
                command: "cargo test".to_string(),
                passed: false,
                failure_count: Some(42),
            },
            ToolEffect::SubagentRun {
                agent_name: "探索".to_string(),
                task: "工作".to_string(),
                outcome: "done".to_string(),
            },
        ];
        result.model_content = vec![ToolResultContent::ImageUrl {
            url: "https://example.invalid/image".to_string(),
            detail: Some("high".to_string()),
        }];
        let restored = tool_result_from_wire(tool_result_to_wire(&result)?)?;
        assert_eq!(
            serde_json::to_value(restored).map_err(|error| error.to_string())?,
            serde_json::to_value(result).map_err(|error| error.to_string())?
        );
        Ok(())
    }

    #[cfg(feature = "sdk-facade-adapters")]
    #[test]
    fn task_store_projection_keeps_large_revision_null_result_and_interrupt() -> Result<(), String>
    {
        use super::managed::Projection as _;
        let execution = echo_agent::tasks::TaskExecution {
            task_id: "任务🙂".to_string(),
            status: echo_agent::tasks::TaskStatus::Completed,
            retry_count: 3,
            failure_fingerprint: Some("fingerprint".to_string()),
            claim: Some(echo_agent::tasks::TaskClaim {
                claim_id: "claim".to_string(),
                revision: u64::MAX,
                attempt: 3,
                spec_hash: "hash".to_string(),
            }),
            result: Some(serde_json::Value::Null),
            interruption_requested: Some(
                echo_agent::tasks::RuntimeInterruptionDisposition::Paused {
                    reason: "暂停".to_string(),
                },
            ),
        };
        let projected = execution.clone().project()?;
        assert_eq!(projected.result, Some(WireValue::Null));
        assert_eq!(
            projected
                .claim
                .as_ref()
                .map(|claim| claim.revision.as_str()),
            Some("18446744073709551615")
        );
        let restored = echo_agent::tasks::TaskExecution::restore(projected)?;
        assert_eq!(restored, execution);
        let mut absent = execution;
        absent.result = None;
        assert_eq!(absent.project()?.result, None);
        Ok(())
    }
}

/// Explicit field projections for the framework's managed persistence facts.
/// The adapter transports requests and receipts; it never computes CAS,
/// fencing, retries or receipt settlement itself.
#[cfg(all(
    feature = "sdk-facade-adapters",
    any(feature = "sdk-extension-bridge", test)
))]
pub(crate) mod managed {
    use echo_agent::tasks;
    use echo_agent::{memory, state};
    use echo_sdk_protocol::managed::*;
    use echo_sdk_protocol::methods::WireTaskStatus;
    use echo_sdk_protocol::scalar::{WireI64, WirePath, WireTimestamp, WireU64};
    use echo_sdk_protocol::task_store::*;

    pub(crate) trait Projection: Sized {
        type Wire;
        fn project(self) -> Result<Self::Wire, String>;
        fn restore(wire: Self::Wire) -> Result<Self, String>;
    }
    macro_rules! identity {
        ($($ty:ty),*) => {$(
            impl Projection for $ty {
                type Wire = Self;
                fn project(self) -> Result<Self, String> { Ok(self) }
                fn restore(wire: Self) -> Result<Self, String> { Ok(wire) }
            }
        )*};
    }
    identity!(String, u16, u32, bool);
    impl Projection for u64 {
        type Wire = WireU64;
        fn project(self) -> Result<Self::Wire, String> {
            Ok(WireU64::from_u64(self))
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            wire.to_u64()
                .ok_or_else(|| "invalid managed u64".to_string())
        }
    }
    impl Projection for i64 {
        type Wire = WireI64;
        fn project(self) -> Result<Self::Wire, String> {
            Ok(WireI64::from_i64(self))
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            wire.to_i64()
                .ok_or_else(|| "invalid managed i64".to_string())
        }
    }
    impl<T: Projection> Projection for Option<T> {
        type Wire = Option<T::Wire>;
        fn project(self) -> Result<Self::Wire, String> {
            self.map(T::project).transpose()
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            wire.map(T::restore).transpose()
        }
    }
    impl<T: Projection> Projection for Vec<T> {
        type Wire = Vec<T::Wire>;
        fn project(self) -> Result<Self::Wire, String> {
            self.into_iter().map(T::project).collect()
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            wire.into_iter().map(T::restore).collect()
        }
    }
    impl<T: Projection> Projection for std::collections::BTreeMap<String, T> {
        type Wire = std::collections::BTreeMap<String, T::Wire>;
        fn project(self) -> Result<Self::Wire, String> {
            self.into_iter()
                .map(|(key, value)| Ok((key, value.project()?)))
                .collect()
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            wire.into_iter()
                .map(|(key, value)| Ok((key, T::restore(value)?)))
                .collect()
        }
    }
    impl Projection for serde_json::Value {
        type Wire = echo_sdk_protocol::scalar::WireValue;
        fn project(self) -> Result<Self::Wire, String> {
            Self::Wire::from_json(self).map_err(|error| error.to_string())
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            wire.into_json().map_err(|error| error.to_string())
        }
    }
    impl Projection for tasks::TaskStatus {
        type Wire = WireTaskStatus;
        fn project(self) -> Result<Self::Wire, String> {
            Ok(match self {
                Self::Pending => WireTaskStatus::Pending,
                Self::Running => WireTaskStatus::Running,
                Self::Blocked(reason) => WireTaskStatus::Blocked { reason },
                Self::Completed => WireTaskStatus::Completed,
                Self::Failed(error) => WireTaskStatus::Failed { error },
                Self::Skipped => WireTaskStatus::Skipped,
                Self::Cancelled => WireTaskStatus::Cancelled,
                Self::TimedOut { error } => WireTaskStatus::TimedOut { error },
                Self::Retrying {
                    attempt,
                    last_error,
                } => WireTaskStatus::Retrying {
                    attempt,
                    last_error,
                },
                Self::Paused(reason) => WireTaskStatus::Paused { reason },
            })
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            Ok(match wire {
                WireTaskStatus::Pending => Self::Pending,
                WireTaskStatus::Running => Self::Running,
                WireTaskStatus::Blocked { reason } => Self::Blocked(reason),
                WireTaskStatus::Completed => Self::Completed,
                WireTaskStatus::Failed { error } => Self::Failed(error),
                WireTaskStatus::Skipped => Self::Skipped,
                WireTaskStatus::Cancelled => Self::Cancelled,
                WireTaskStatus::TimedOut { error } => Self::TimedOut { error },
                WireTaskStatus::Retrying {
                    attempt,
                    last_error,
                } => Self::Retrying {
                    attempt,
                    last_error,
                },
                WireTaskStatus::Paused { reason } => Self::Paused(reason),
            })
        }
    }
    impl Projection for std::path::PathBuf {
        type Wire = WirePath;
        fn project(self) -> Result<Self::Wire, String> {
            super::path_to_wire(&self)
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            super::path_from_wire(&wire)
        }
    }
    impl Projection for chrono::DateTime<chrono::Utc> {
        type Wire = WireTimestamp;
        fn project(self) -> Result<Self::Wire, String> {
            WireTimestamp::from_unix_parts(self.timestamp(), self.timestamp_subsec_nanos())
                .map_err(|error| error.to_string())
        }
        fn restore(wire: Self::Wire) -> Result<Self, String> {
            wire.validate().map_err(|error| error.to_string())?;
            chrono::DateTime::from_timestamp(
                wire.unix_seconds
                    .to_i64()
                    .ok_or_else(|| "invalid checkpoint timestamp".to_string())?,
                wire.nanos,
            )
            .ok_or_else(|| "checkpoint timestamp is out of range".to_string())
        }
    }
    macro_rules! structure {
        ($source:path, $target:ident, $($field:ident),+ $(,)?) => {
            impl Projection for $source {
                type Wire = $target;
                fn project(self) -> Result<Self::Wire, String> {
                    Ok($target { $($field: self.$field.project()?),+ })
                }
                fn restore(wire: Self::Wire) -> Result<Self, String> {
                    Ok(Self { $($field: Projection::restore(wire.$field)?),+ })
                }
            }
        };
    }
    macro_rules! enumeration {
        ($source:path, $target:ident, $($variant:ident $( { $($field:ident),+ } )?),+ $(,)?) => {
            impl Projection for $source {
                type Wire = $target;
                fn project(self) -> Result<Self::Wire, String> {
                    Ok(match self {
                        $(Self::$variant $( { $($field),+ } )? =>
                            $target::$variant $( { $($field: $field.project()?),+ } )?),+
                    })
                }
                fn restore(wire: Self::Wire) -> Result<Self, String> {
                    Ok(match wire {
                        $($target::$variant $( { $($field),+ } )? =>
                            Self::$variant $( { $($field: Projection::restore($field)?),+ } )?),+
                    })
                }
            }
        };
    }
    structure!(
        memory::ConversationProjectionAuthority,
        ConversationProjectionAuthorityWire,
        conversation_id,
        epoch,
        revision,
        lifecycle,
        delete_receipt_retention_floor_epoch
    );
    structure!(
        memory::NewConversation,
        NewConversationWire,
        conversation_id,
        user_id,
        agent_type,
        title
    );
    structure!(
        memory::StoredMessage,
        StoredMessageWire,
        id,
        conversation_id,
        role,
        content,
        attachments_json,
        tool_calls_json,
        tool_result_json,
        created_at
    );
    structure!(
        memory::EnsureConversationProjectionRequest,
        EnsureConversationProjectionRequestWire,
        conversation,
        expected_tombstone_epoch
    );
    structure!(
        memory::ConversationProjectionEpochReceipt,
        ConversationProjectionEpochReceiptWire,
        authority,
        status
    );
    structure!(
        memory::TranscriptProjectionItem,
        TranscriptProjectionItemWire,
        ordinal,
        digest,
        message
    );
    structure!(
        memory::TranscriptProjectionBatch,
        TranscriptProjectionBatchWire,
        schema_version,
        operation_id,
        payload_digest,
        conversation_id,
        conversation_epoch,
        generation_id,
        items
    );
    structure!(
        memory::TranscriptProjectionApplyReceipt,
        TranscriptProjectionApplyReceiptWire,
        operation_id,
        payload_digest,
        authority,
        status
    );
    structure!(
        memory::TranscriptProjectionSettlement,
        TranscriptProjectionSettlementWire,
        status,
        operation_id,
        conversation_id,
        generation_id,
        attempt,
        error_class,
        detail
    );
    structure!(
        memory::ManagedConversationDelete,
        ManagedConversationDeleteWire,
        schema_version,
        operation_id,
        payload_digest,
        conversation_id,
        expected_epoch
    );
    structure!(
        memory::ManagedConversationDeleteReceipt,
        ManagedConversationDeleteReceiptWire,
        operation_id,
        payload_digest,
        deleted_epoch,
        retention_floor_epoch,
        status
    );
    structure!(
        memory::ManagedConversationImport,
        ManagedConversationImportWire,
        schema_version,
        operation_id,
        payload_digest,
        conversation_id,
        expected_epoch,
        expected_revision,
        generation_id,
        messages
    );
    structure!(
        state::RuntimeScopeAuthority,
        RuntimeScopeAuthorityWire,
        scope_id,
        conversation_epoch,
        revision,
        lifecycle
    );
    structure!(
        state::ManagedRuntimeStateSnapshot,
        ManagedRuntimeStateSnapshotWire,
        scope,
        runtime_state_id,
        version,
        checkpoint
    );
    structure!(
        state::RuntimeCheckpointCasRequest,
        RuntimeCheckpointCasRequestWire,
        scope_id,
        runtime_state_id,
        conversation_epoch,
        expected_scope_revision,
        expected_state_version,
        checkpoint,
        managed_import
    );
    structure!(
        state::ManagedImportEpochTransition,
        ManagedImportEpochTransitionWire,
        request,
        receipt
    );
    structure!(
        state::RuntimeCheckpointCasReceipt,
        RuntimeCheckpointCasReceiptWire,
        scope,
        runtime_state_id,
        version,
        status
    );
    structure!(
        state::RuntimeTranscriptAckRequest,
        RuntimeTranscriptAckRequestWire,
        scope_id,
        runtime_state_id,
        conversation_epoch,
        expected_scope_revision,
        expected_state_version,
        projection_receipt,
        checkpoint
    );
    structure!(
        state::RuntimeGenerationRetireRequest,
        RuntimeGenerationRetireRequestWire,
        schema_version,
        operation_id,
        payload_digest,
        scope_id,
        runtime_state_id,
        expected_scope_revision,
        expected_state_version
    );
    structure!(
        state::RuntimeGenerationRetireReceipt,
        RuntimeGenerationRetireReceiptWire,
        scope,
        runtime_state_id,
        version,
        status
    );
    structure!(
        state::ScopeRetirementItem,
        ScopeRetirementItemWire,
        runtime_state_id,
        state_version,
        pending_operation_id,
        status
    );
    structure!(
        state::ScopeRetirementManifest,
        ScopeRetirementManifestWire,
        delete_operation_id,
        payload_digest,
        expected_conversation_epoch,
        items,
        conversation_delete_receipt
    );
    structure!(
        state::ScopeRetirementRequest,
        ScopeRetirementRequestWire,
        schema_version,
        delete_operation_id,
        payload_digest,
        scope_id,
        expected_scope_revision,
        expected_conversation_epoch
    );
    structure!(
        state::ScopeRetirementReceipt,
        ScopeRetirementReceiptWire,
        scope,
        manifest,
        dropped_operation_ids,
        retention_floor_epoch,
        status
    );
    structure!(
        state::AgentCheckpoint,
        AgentCheckpointWire,
        conversation_id,
        messages_json,
        current_plan,
        active_skills,
        blocked_reason,
        working_dir,
        timestamp
    );
    enumeration!(
        memory::ConversationProjectionLifecycle,
        ConversationProjectionLifecycleWire,
        Live,
        Deleted
    );
    enumeration!(
        memory::ConversationProjectionEpochStatus,
        ConversationProjectionEpochStatusWire,
        Created,
        AdoptedLegacy,
        Existing,
        Recreated,
        Tombstoned,
        EpochConflict
    );
    enumeration!(
        memory::TranscriptProjectionConflictKind,
        TranscriptProjectionConflictKindWire,
        OperationIdentity,
        OrdinalDigest,
        Revision,
        ManagedImport
    );
    enumeration!(
        memory::TranscriptProjectionApplyStatus,
        TranscriptProjectionApplyStatusWire,
        Applied,
        AlreadyApplied,
        Fenced {
            current_epoch,
            lifecycle
        },
        Conflict {
            current_epoch,
            current_revision,
            kind
        }
    );
    enumeration!(
        memory::TranscriptProjectionSettlementStatus,
        TranscriptProjectionSettlementStatusWire,
        Settled,
        Deferred,
        Blocked,
        Conflict
    );
    enumeration!(
        memory::TranscriptProjectionErrorClass,
        TranscriptProjectionErrorClassWire,
        OutcomeUnknown,
        TransientNoCommit,
        RevisionConflict,
        DeadlineExceeded,
        Unsupported,
        InvalidConfiguration,
        CorruptState,
        SemanticConflict
    );
    enumeration!(
        memory::ManagedConversationDeleteStatus,
        ManagedConversationDeleteStatusWire,
        Deleted,
        AlreadyDeleted,
        EpochConflict,
        ReceiptExpired,
        IdentityConflict
    );
    enumeration!(
        state::RuntimeStateVersion,
        RuntimeStateVersionWire,
        Absent,
        Unmanaged { digest },
        Managed { revision },
        Retired {
            revision,
            operation_id
        }
    );
    enumeration!(
        state::RuntimeStateExpectedVersion,
        RuntimeStateExpectedVersionWire,
        Absent,
        Unmanaged { digest },
        Managed { revision }
    );
    enumeration!(
        state::RuntimeScopeLifecycle,
        RuntimeScopeLifecycleWire,
        Active,
        Retiring,
        Tombstoned
    );
    enumeration!(
        state::RuntimeCheckpointCasStatus,
        RuntimeCheckpointCasStatusWire,
        Applied,
        AlreadyCurrent,
        RevisionConflict,
        GenerationRetired,
        ScopeFenced
    );
    enumeration!(
        state::RuntimeGenerationRetireStatus,
        RuntimeGenerationRetireStatusWire,
        Retired,
        AlreadyRetired,
        Conflict,
        PendingProjection,
        ScopeFenced
    );
    enumeration!(
        state::ScopeRetirementItemStatus,
        ScopeRetirementItemStatusWire,
        Pending,
        DroppedByDelete
    );
    enumeration!(
        state::ScopeRetirementAdvance,
        ScopeRetirementAdvanceWire,
        ConversationDeleted { receipt },
        GenerationDropped {
            runtime_state_id,
            pending_operation_id
        },
        Complete
    );
    enumeration!(
        state::ScopeRetirementStatus,
        ScopeRetirementStatusWire,
        Begun,
        InProgress,
        Completed,
        AlreadyCompleted,
        RevisionConflict,
        EpochConflict,
        ReceiptExpired,
        IdentityConflict
    );
    structure!(
        tasks::TaskSpec,
        TaskSpecWire,
        id,
        title,
        description,
        depends_on,
        max_retries,
        extension
    );
    structure!(
        tasks::TaskClaim,
        TaskClaimWire,
        claim_id,
        revision,
        attempt,
        spec_hash
    );
    structure!(
        tasks::TaskExecution,
        TaskExecutionWire,
        task_id,
        status,
        retry_count,
        failure_fingerprint,
        claim,
        result,
        interruption_requested
    );
    structure!(tasks::Task, TaskWire, spec, execution);
    structure!(
        tasks::RuntimePlanSnapshot,
        RuntimePlanSnapshotWire,
        revision,
        tasks
    );
    structure!(
        tasks::TaskGraphContext,
        TaskGraphContextWire,
        goal,
        assumptions,
        risks,
        execution_mode,
        metadata
    );
    structure!(
        tasks::RevisionedTaskGraph,
        RevisionedTaskGraphWire,
        snapshot,
        context
    );
    structure!(
        tasks::TaskPatchEffects,
        TaskPatchEffectsWire,
        inserted_task_ids,
        updated_task_ids,
        skipped_task_ids,
        reset_task_ids,
        progressed_task_ids,
        reordered
    );
    structure!(
        tasks::TaskGraphCommit,
        TaskGraphCommitWire,
        expected_revision,
        expected_executions,
        next,
        reason,
        effects
    );
    structure!(
        tasks::TaskExecutionCommit,
        TaskExecutionCommitWire,
        expected_revision,
        expected_executions,
        next
    );
    enumeration!(
        tasks::RuntimeInterruptionDisposition,
        RuntimeInterruptionDispositionWire,
        Cancelled,
        Paused { reason }
    );
    enumeration!(
        tasks::TaskGraphExecutionMode,
        TaskGraphExecutionModeWire,
        Sequential,
        Parallel
    );
}
