//! Framework canonical assistant block 的薄边界；不判断候选或执行完成。

#[cfg(feature = "sdk-facade-adapters")]
use echo_agent::agent::AgentEvent;
#[cfg(feature = "sdk-extension-bridge")]
use echo_agent::llm::types::DeltaMessage;
#[cfg(feature = "sdk-extension-bridge")]
use echo_agent::llm::types::DeltaToolCall;
use echo_agent::llm::types::ToolCall;
use echo_agent::llm::types::{Message, ReasoningBlock};
#[cfg(feature = "sdk-extension-bridge")]
use echo_sdk_protocol::methods::AgentEventWire;
use echo_sdk_protocol::methods::{AgentPhaseWire, ExecutionStatusKindWire, ExecutionStatusWire};
use echo_sdk_protocol::methods::{AssistantMessageBlockWire, AssistantPhaseWire};
use echo_sdk_protocol::methods::{LlmToolCallWire, ProviderOutputSourceWire};
#[cfg(feature = "sdk-extension-bridge")]
use echo_sdk_protocol::scalar::WireU64;
use serde::{Deserialize, Serialize};

use super::wire::reasoning_block_from_wire;
#[cfg(feature = "sdk-extension-bridge")]
use super::wire::reasoning_block_wire;

#[cfg(test)]
pub(super) fn test_commentary_event(text: &str) -> Result<echo_agent::agent::AgentEvent, String> {
    serde_json::from_value(
        serde_json::json!({"type":"assistant_message","data":{"block":{
            "block_id":"fixture-block","request_id":"fixture-request","sequence":0,
            "phase":"commentary","delta":text,"complete":false,"provider":null,"reasoning":null
        }}}),
    )
    .map_err(|error| error.to_string())
}

/// 仅转换原生 u64 与 wire 十进制、reasoning 的 type/kind 标签。
/// 显式闭合的 DTO 允许旧 build pin 明确拒绝新事件，同时在后继来源中逐字段解码；
/// 不把任意 JSON 或未知字段默认为已支持的 Framework 事件。
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeAssistantBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    provider_source: Option<ProviderOutputSourceWire>,
    block_id: String,
    request_id: String,
    sequence: u64,
    #[serde(default)]
    phase: AssistantPhaseWire,
    delta: String,
    complete: bool,
    provider: Option<String>,
    reasoning: Option<ReasoningBlock>,
}

#[cfg(feature = "sdk-extension-bridge")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "data",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum NativeEvent {
    AssistantMessage { block: NativeAssistantBlock },
    ExecutionStatus(NativeExecutionStatus),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeExecutionStatus {
    execution_id: String,
    request_id: Option<String>,
    sequence: u64,
    phase: AgentPhaseWire,
    status: ExecutionStatusKindWire,
    reason: Option<String>,
}

impl NativeExecutionStatus {
    fn from_wire(status: ExecutionStatusWire) -> Result<Self, String> {
        status.validate()?;
        Ok(Self {
            execution_id: status.execution_id,
            request_id: status.request_id,
            sequence: status.sequence.to_u64().ok_or("invalid runtime sequence")?,
            phase: status.phase,
            status: status.status,
            reason: status.reason,
        })
    }

    #[cfg(feature = "sdk-extension-bridge")]
    fn into_wire(self) -> Result<ExecutionStatusWire, String> {
        let status = ExecutionStatusWire {
            execution_id: self.execution_id,
            request_id: self.request_id,
            sequence: WireU64::from_u64(self.sequence),
            phase: self.phase,
            status: self.status,
            reason: self.reason,
        };
        status.validate()?;
        Ok(status)
    }
}

impl NativeAssistantBlock {
    #[cfg(feature = "sdk-extension-bridge")]
    fn into_wire(self) -> Result<AssistantMessageBlockWire, String> {
        let block = AssistantMessageBlockWire {
            provider_source: self.provider_source,
            block_id: self.block_id,
            request_id: self.request_id,
            sequence: WireU64::from_u64(self.sequence),
            phase: self.phase,
            delta: self.delta,
            complete: self.complete,
            provider: self.provider,
            reasoning: self.reasoning.as_ref().map(reasoning_block_wire),
        };
        block.validate()?;
        Ok(block)
    }

    fn from_wire(block: AssistantMessageBlockWire) -> Result<Self, String> {
        block.validate()?;
        Ok(Self {
            provider_source: block.provider_source,
            block_id: block.block_id,
            request_id: block.request_id,
            sequence: block
                .sequence
                .to_u64()
                .ok_or("invalid assistant sequence")?,
            phase: block.phase,
            delta: block.delta,
            complete: block.complete,
            provider: block.provider,
            reasoning: block.reasoning.map(reasoning_block_from_wire),
        })
    }
}

#[cfg(feature = "sdk-extension-bridge")]
pub(super) fn from_framework(event: &AgentEvent) -> Result<AgentEventWire, String> {
    let serialized = serde_json::to_value(event).map_err(|error| error.to_string())?;
    let native: NativeEvent = serde_json::from_value(serialized)
        .map_err(|error| format!("unsupported framework AgentEvent variant: {error}"))?;
    Ok(match native {
        NativeEvent::AssistantMessage { block } => AgentEventWire::AssistantMessage {
            block: block.into_wire()?,
        },
        NativeEvent::ExecutionStatus(status) => {
            AgentEventWire::ExecutionStatus(status.into_wire()?)
        }
    })
}

#[cfg(feature = "sdk-facade-adapters")]
pub(super) fn status_to_framework(status: ExecutionStatusWire) -> Result<AgentEvent, String> {
    let status = NativeExecutionStatus::from_wire(status)?;
    serde_json::from_value(serde_json::json!({"type":"execution_status", "data": status})).map_err(
        |error| format!("framework build pin does not support native runtime status: {error}"),
    )
}

#[cfg(feature = "sdk-extension-bridge")]
pub(super) fn to_framework(block: AssistantMessageBlockWire) -> Result<AgentEvent, String> {
    let native = NativeEvent::AssistantMessage {
        block: NativeAssistantBlock::from_wire(block)?,
    };
    let serialized = serde_json::to_value(native).map_err(|error| error.to_string())?;
    serde_json::from_value(serialized).map_err(|error| {
        format!("framework build pin does not support the native assistant event: {error}")
    })
}

#[derive(Default)]
pub(super) struct MessageMetadata {
    pub assistant_blocks: Option<Vec<AssistantMessageBlockWire>>,
    pub approved_assistant_blocks: Option<Vec<AssistantMessageBlockWire>>,
    pub execution_status: Option<ExecutionStatusWire>,
}

#[cfg(feature = "sdk-extension-bridge")]
pub(super) fn message_metadata(message: &Message) -> Result<MessageMetadata, String> {
    let value = serde_json::to_value(message).map_err(|error| error.to_string())?;
    let read_blocks = |name: &str| -> Result<Option<Vec<AssistantMessageBlockWire>>, String> {
        value
            .get(name)
            .filter(|value| !value.is_null())
            .map(|value| {
                serde_json::from_value::<Vec<NativeAssistantBlock>>(value.clone())
                    .map_err(|error| format!("invalid native {name}: {error}"))
            })
            .transpose()?
            .map(|blocks| {
                blocks
                    .into_iter()
                    .map(NativeAssistantBlock::into_wire)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()
    };
    let execution_status = value
        .get("execution_status")
        .filter(|value| !value.is_null())
        .map(|value| {
            serde_json::from_value::<NativeExecutionStatus>(value.clone())
                .map_err(|error| format!("invalid native execution status: {error}"))
        })
        .transpose()?
        .map(NativeExecutionStatus::into_wire)
        .transpose()?;
    Ok(MessageMetadata {
        assistant_blocks: read_blocks("assistant_blocks")?,
        approved_assistant_blocks: read_blocks("approved_assistant_blocks")?,
        execution_status,
    })
}

/// Keep raw history and approved presentation distinct, including Some(empty).
pub(super) fn with_message_metadata(
    message: Message,
    metadata: MessageMetadata,
) -> Result<Message, String> {
    let mut fields = Vec::new();
    for (name, blocks) in [
        ("assistant_blocks", metadata.assistant_blocks),
        (
            "approved_assistant_blocks",
            metadata.approved_assistant_blocks,
        ),
    ] {
        if let Some(blocks) = blocks {
            let blocks = blocks
                .into_iter()
                .map(NativeAssistantBlock::from_wire)
                .collect::<Result<Vec<_>, _>>()?;
            fields.push((
                name,
                serde_json::to_value(blocks).map_err(|error| error.to_string())?,
            ));
        }
    }
    if let Some(status) = metadata.execution_status {
        let status = NativeExecutionStatus::from_wire(status)?;
        fields.push((
            "execution_status",
            serde_json::to_value(status).map_err(|error| error.to_string())?,
        ));
    }
    if fields.is_empty() {
        return Ok(message);
    }
    let mut value = serde_json::to_value(message).map_err(|error| error.to_string())?;
    let object = value
        .as_object_mut()
        .ok_or("native Message must be an object")?;
    for (name, value) in &fields {
        object.insert((*name).to_string(), value.clone());
    }
    let restored: Message = serde_json::from_value(value).map_err(|error| error.to_string())?;
    let observed = serde_json::to_value(&restored).map_err(|error| error.to_string())?;
    for (name, expected) in fields {
        if observed.get(name) != Some(&expected) {
            return Err(format!(
                "framework build pin cannot preserve native assistant history field {name}"
            ));
        }
    }
    Ok(restored)
}

#[cfg(feature = "sdk-extension-bridge")]
pub(super) fn with_delta_blocks(
    delta: DeltaMessage,
    blocks: Option<Vec<AssistantMessageBlockWire>>,
) -> Result<DeltaMessage, String> {
    let Some(blocks) = blocks else {
        return Ok(delta);
    };
    // 同一精确Framework来源必须先具备Message持久化合同，不能在旧pin吞掉新delta元数据。
    let checked = with_message_metadata(
        Message::default(),
        MessageMetadata {
            assistant_blocks: Some(blocks),
            ..Default::default()
        },
    )?;
    let value = serde_json::to_value(checked).map_err(|error| error.to_string())?;
    let blocks = value
        .get("assistant_blocks")
        .ok_or("missing checked assistant history")?;
    let mut restored: DeltaMessage =
        serde_json::from_value(serde_json::json!({"assistant_blocks": blocks}))
            .map_err(|error| error.to_string())?;
    restored.role = delta.role;
    restored.content = delta.content;
    restored.reasoning_content = delta.reasoning_content;
    restored.reasoning_blocks = delta.reasoning_blocks;
    restored.tool_calls = delta.tool_calls;
    Ok(restored)
}

pub(super) fn tool_call_from_wire(call: LlmToolCallWire) -> Result<ToolCall, String> {
    let source = call.provider_source;
    let mut value = serde_json::json!({
        "id":call.id, "type":call.call_type,
        "function":{"name":call.function_name,"arguments":call.arguments},
    });
    if let Some(source) = &source {
        value
            .as_object_mut()
            .ok_or("tool call must be an object")?
            .insert(
                "provider_source".into(),
                serde_json::to_value(source).map_err(|error| error.to_string())?,
            );
    }
    let native: ToolCall = serde_json::from_value(value).map_err(|error| {
        if source.is_some() {
            format!("framework build pin cannot preserve provider source: {error}")
        } else {
            error.to_string()
        }
    })?;
    if let Some(source) = source {
        let expected = serde_json::to_value(source).map_err(|error| error.to_string())?;
        let observed = serde_json::to_value(&native).map_err(|error| error.to_string())?;
        if observed.get("provider_source") != Some(&expected) {
            return Err("framework build pin cannot preserve provider source".to_string());
        }
    }
    Ok(native)
}

#[cfg(feature = "sdk-extension-bridge")]
pub(super) fn tool_call_wire(call: &ToolCall) -> Result<LlmToolCallWire, String> {
    let value = serde_json::to_value(call).map_err(|error| error.to_string())?;
    let provider_source = value
        .get("provider_source")
        .filter(|value| !value.is_null())
        .map(|value| serde_json::from_value(value.clone()).map_err(|error| error.to_string()))
        .transpose()?;
    Ok(LlmToolCallWire {
        provider_source,
        id: call.id.clone(),
        call_type: call.call_type.clone(),
        function_name: call.function.name.clone(),
        arguments: call.function.arguments.clone(),
    })
}

#[cfg(feature = "sdk-extension-bridge")]
pub(super) fn delta_tool_call_from_wire(
    call: echo_sdk_protocol::methods::LlmDeltaToolCallWire,
) -> Result<DeltaToolCall, String> {
    if let Some(source) = &call.provider_source {
        // 仅构造值检查原serde合同；此probe不注册、不派发、更不执行工具。
        tool_call_from_wire(LlmToolCallWire {
            provider_source: Some(source.clone()),
            id: "sdk-source-probe".into(),
            call_type: "function".into(),
            function_name: "sdk_source_probe".into(),
            arguments: "{}".into(),
        })?;
    }
    let mut value = serde_json::json!({"index":call.index,"id":call.id,"type":call.call_type,"function":call.function});
    if let Some(source) = call.provider_source {
        value
            .as_object_mut()
            .ok_or("tool delta must be an object")?
            .insert(
                "provider_source".into(),
                serde_json::to_value(source).map_err(|error| error.to_string())?,
            );
    }
    serde_json::from_value(value).map_err(|error| error.to_string())
}

#[cfg(all(test, feature = "sdk-extension-bridge"))]
mod tests {
    use super::*;

    fn source() -> ProviderOutputSourceWire {
        ProviderOutputSourceWire {
            protocol: echo_sdk_protocol::methods::LlmApiProtocolWire::Responses,
            request_id: "native-request".into(),
            item_id: "item-中文🧪".into(),
            output_index: u32::MAX,
            content_index: Some(u32::MAX),
            content_kind: Some("function_call".into()),
            phase: Some("unknown-original-phase".into()),
        }
    }

    #[test]
    fn native_message_adapter_tool_provenance_round_trips_all_fields() -> Result<(), String> {
        let expected = LlmToolCallWire {
            provider_source: Some(source()),
            id: "call-1".into(),
            call_type: "function".into(),
            function_name: "fixture".into(),
            arguments: "{}".into(),
        };
        let native = tool_call_from_wire(expected.clone())?;
        assert_eq!(tool_call_wire(&native)?, expected);
        Ok(())
    }

    #[test]
    fn native_message_adapter_delta_provenance_round_trips_all_fields() -> Result<(), String> {
        use echo_sdk_protocol::methods::{LlmDeltaFunctionWire, LlmDeltaToolCallWire};

        for expected in [
            LlmDeltaToolCallWire {
                provider_source: Some(source()),
                index: 0,
                id: None,
                call_type: None,
                function: None,
            },
            LlmDeltaToolCallWire {
                provider_source: Some(source()),
                index: u32::MAX,
                id: Some("delta-call-中文🧪".into()),
                call_type: Some("function".into()),
                function: Some(LlmDeltaFunctionWire {
                    name: Some("fixture".into()),
                    arguments: Some("{\"query\":\"完整原文🙂\"}".into()),
                }),
            },
        ] {
            let expected_source =
                serde_json::to_value(source()).map_err(|error| error.to_string())?;
            let native = delta_tool_call_from_wire(expected.clone())?;
            assert_eq!(native.index, expected.index);
            assert_eq!(native.id, expected.id);
            assert_eq!(native.call_type, expected.call_type);
            match (&native.function, &expected.function) {
                (None, None) => {}
                (Some(actual), Some(expected)) => {
                    assert_eq!(actual.name, expected.name);
                    assert_eq!(actual.arguments, expected.arguments);
                }
                _ => return Err("delta function presence changed".into()),
            }
            let actual_source = serde_json::to_value(
                native
                    .provider_source
                    .as_ref()
                    .ok_or("delta lost provider source")?,
            )
            .map_err(|error| error.to_string())?;
            assert_eq!(actual_source, expected_source);
        }
        Ok(())
    }

    #[test]
    fn native_message_adapter_preserves_some_empty_approved_projection() -> Result<(), String> {
        let restored = with_message_metadata(
            Message::assistant("raw".to_string()),
            MessageMetadata {
                approved_assistant_blocks: Some(Vec::new()),
                ..Default::default()
            },
        )?;
        let metadata = message_metadata(&restored)?;
        assert_eq!(metadata.approved_assistant_blocks, Some(Vec::new()));
        assert!(metadata.assistant_blocks.is_none());
        assert!(metadata.execution_status.is_none());
        let actual = serde_json::to_value(restored).map_err(|error| error.to_string())?;
        assert_eq!(
            actual.get("approved_assistant_blocks"),
            Some(&serde_json::json!([]))
        );
        Ok(())
    }

    #[test]
    fn native_message_adapter_runtime_status_framework_round_trips_all_fields() -> Result<(), String>
    {
        let original = ExecutionStatusWire {
            execution_id: "exec-1".into(),
            request_id: None,
            sequence: WireU64::from_u64(0),
            phase: AgentPhaseWire::Terminal,
            status: ExecutionStatusKindWire::Settled,
            reason: None,
        };
        for expected in [
            original.clone(),
            ExecutionStatusWire {
                execution_id: "exec-中文🧪".into(),
                request_id: Some("request-1".into()),
                sequence: WireU64::from_u64(u64::MAX),
                phase: AgentPhaseWire::Terminal,
                status: ExecutionStatusKindWire::Failed,
                reason: Some("原始结算原因".into()),
            },
        ] {
            expected.validate()?;
            assert_eq!(
                from_framework(&status_to_framework(expected.clone())?)?,
                AgentEventWire::ExecutionStatus(expected)
            );
        }
        let mut invalid = original;
        invalid.execution_id = " ".into();
        assert!(status_to_framework(invalid).is_err());
        Ok(())
    }

    #[test]
    fn native_message_adapter_runtime_status_preserves_all_fields() -> Result<(), String> {
        let status = ExecutionStatusWire {
            execution_id: "exec-中文🧪".into(),
            request_id: Some("request-1".into()),
            sequence: WireU64::from_u64(u64::MAX),
            phase: AgentPhaseWire::Terminal,
            status: ExecutionStatusKindWire::Failed,
            reason: Some("原始结算原因".into()),
        };
        assert_eq!(
            NativeExecutionStatus::from_wire(status.clone())?.into_wire()?,
            status
        );
        Ok(())
    }

    #[test]
    fn native_message_adapter_status_history_round_trips_all_fields() -> Result<(), String> {
        let expected = ExecutionStatusWire {
            execution_id: "old-history-execution".into(),
            request_id: None,
            sequence: WireU64::from_u64(0),
            phase: AgentPhaseWire::Terminal,
            status: ExecutionStatusKindWire::Settled,
            reason: None,
        };
        expected.validate()?;
        let restored = with_message_metadata(
            Message::assistant("历史".to_string()),
            MessageMetadata {
                execution_status: Some(expected.clone()),
                ..Default::default()
            },
        )?;
        let metadata = message_metadata(&restored)?;
        assert_eq!(metadata.execution_status, Some(expected.clone()));
        assert!(metadata.assistant_blocks.is_none());
        assert!(metadata.approved_assistant_blocks.is_none());
        let mut invalid = expected;
        invalid.request_id = Some(" ".into());
        assert!(
            with_message_metadata(
                Message::assistant("历史".to_string()),
                MessageMetadata {
                    execution_status: Some(invalid),
                    ..Default::default()
                },
            )
            .is_err()
        );
        Ok(())
    }

    // 当前已确认native合同必须完整往返；原f7负控二进制和日志独立保留其来源资格。
    #[test]
    fn native_message_adapter_native_event_round_trips_fields_and_rejects_invalid_input()
    -> Result<(), String> {
        let block = AssistantMessageBlockWire {
            provider_source: None,
            block_id: "old-pin-block".into(),
            request_id: "old-pin-request".into(),
            sequence: WireU64::from_u64(0),
            phase: AssistantPhaseWire::FinalAnswer,
            delta: "".into(),
            complete: true,
            provider: None,
            reasoning: None,
        };
        block.validate()?;
        assert_eq!(
            from_framework(&to_framework(block.clone())?)?,
            AgentEventWire::AssistantMessage {
                block: block.clone()
            }
        );
        for reasoning in [
            echo_sdk_protocol::methods::LlmReasoningBlockWire::Signed {
                thinking: "签名思考🙂".into(),
                signature: "sig-exact==".into(),
            },
            echo_sdk_protocol::methods::LlmReasoningBlockWire::Redacted {
                data: "encrypted-exact==".into(),
            },
            echo_sdk_protocol::methods::LlmReasoningBlockWire::Opaque {
                provider: "responses".into(),
                id: "reasoning-id".into(),
                data: "opaque==".into(),
                summary: vec!["公开摘要".into()],
            },
        ] {
            let mut expected = block.clone();
            let mut provenance = source();
            provenance.content_kind = Some("reasoning".into());
            expected.provider_source = Some(provenance);
            expected.sequence = WireU64::from_u64(u64::MAX);
            expected.phase = AssistantPhaseWire::Reasoning;
            expected.delta = "原生思考🙂".into();
            expected.provider = Some("responses".into());
            expected.reasoning = Some(reasoning);
            expected.validate()?;
            assert_eq!(
                from_framework(&to_framework(expected.clone())?)?,
                AgentEventWire::AssistantMessage { block: expected }
            );
        }
        let mut invalid = block;
        invalid.block_id = " ".into();
        assert!(to_framework(invalid).is_err());
        assert!(from_framework(&AgentEvent::FinalAnswer("ordinary notification".into())).is_err());
        assert!(
            serde_json::from_value::<NativeEvent>(serde_json::json!({
                "type": "unknown_native_event", "data": {}
            }))
            .is_err()
        );
        Ok(())
    }

    #[test]
    fn native_message_adapter_block_history_keeps_raw_and_approved_separate() -> Result<(), String>
    {
        let block = AssistantMessageBlockWire {
            provider_source: None,
            block_id: "history-block".into(),
            request_id: "history-request".into(),
            sequence: WireU64::from_u64(u64::MAX),
            phase: AssistantPhaseWire::Commentary,
            delta: "完整原文🙂".into(),
            complete: true,
            provider: None,
            reasoning: None,
        };
        block.validate()?;
        for approved in [None, Some(Vec::new())] {
            let restored = with_message_metadata(
                Message::assistant("原文".to_string()),
                MessageMetadata {
                    assistant_blocks: Some(vec![block.clone()]),
                    approved_assistant_blocks: approved.clone(),
                    ..Default::default()
                },
            )?;
            let metadata = message_metadata(&restored)?;
            assert_eq!(metadata.assistant_blocks, Some(vec![block.clone()]));
            assert_eq!(metadata.approved_assistant_blocks, approved);
            assert!(metadata.execution_status.is_none());
        }
        let mut invalid = block;
        invalid.request_id = " ".into();
        assert!(
            with_message_metadata(
                Message::assistant("原文".to_string()),
                MessageMetadata {
                    assistant_blocks: Some(vec![invalid]),
                    ..Default::default()
                },
            )
            .is_err()
        );
        Ok(())
    }

    #[test]
    fn native_message_adapter_preserves_u64_phase_signature_and_provider_payload()
    -> Result<(), String> {
        for reasoning in [
            None,
            Some(ReasoningBlock::Signed {
                thinking: "签名思考🙂".into(),
                signature: "sig-exact==".into(),
            }),
            Some(ReasoningBlock::Redacted {
                data: "encrypted-exact==".into(),
            }),
            Some(ReasoningBlock::Opaque {
                provider: "responses".into(),
                id: "reasoning-id".into(),
                data: "opaque==".into(),
                summary: vec!["公开摘要".into()],
            }),
        ] {
            let native = NativeAssistantBlock {
                provider_source: None,
                block_id: "provider-block".into(),
                request_id: "request-中文🧪".into(),
                sequence: u64::MAX,
                phase: AssistantPhaseWire::PartialAnswer,
                delta: "普通回答".into(),
                complete: true,
                provider: Some("responses".into()),
                reasoning,
            };
            let expected = serde_json::to_value(&native).map_err(|error| error.to_string())?;
            let restored = NativeAssistantBlock::from_wire(native.into_wire()?)?;
            let actual = serde_json::to_value(restored).map_err(|error| error.to_string())?;
            assert_eq!(actual, expected);
        }
        Ok(())
    }
}
