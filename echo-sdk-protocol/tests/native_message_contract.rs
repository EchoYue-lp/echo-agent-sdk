use echo_sdk_protocol::methods::{
    AGENT_EVENT_WIRE_TYPE_ID, AgentEventWire, AgentStreamChunkWire, AgentStreamTerminalWire,
    AssistantMessageBlockWire, AssistantPhaseWire, LlmChatChunkWire, LlmMessageWire,
    LlmReasoningBlockWire, LlmStreamChunkWire, LlmStreamCompleteWire,
};
use echo_sdk_protocol::scalar::{WireField, WireU64, WireValue};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn native_message_provider_coordinates_and_unknown_original_phase_are_lossless() -> TestResult {
    use echo_sdk_protocol::methods::{LlmApiProtocolWire, ProviderOutputSourceWire};
    let mut expected = block(AssistantPhaseWire::Unknown);
    expected.provider_source = Some(ProviderOutputSourceWire {
        protocol: LlmApiProtocolWire::Responses,
        request_id: "native-request".into(),
        item_id: "item-中文🧪".into(),
        output_index: u32::MAX,
        content_index: Some(u32::MAX),
        content_kind: Some("output_text".into()),
        phase: Some("provider-future-phase".into()),
    });
    let value = WireValue::Variant {
        type_id: AGENT_EVENT_WIRE_TYPE_ID.into(),
        variant: "assistant_message".into(),
        fields: vec![WireField {
            name: "block".into(),
            value: WireValue::from_json(serde_json::to_value(&expected)?)?,
        }],
    };
    assert_eq!(
        AgentEventWire::from_wire_value(&value)?,
        AgentEventWire::AssistantMessage { block: expected }
    );
    Ok(())
}

#[test]
fn native_runtime_status_preserves_exact_identity_kind_and_sequence() -> TestResult {
    use echo_sdk_protocol::methods::{
        AgentPhaseWire, ExecutionStatusKindWire, ExecutionStatusWire,
    };
    for kind in [
        ExecutionStatusKindWire::Started,
        ExecutionStatusKindWire::Waiting,
        ExecutionStatusKindWire::ToolPending,
        ExecutionStatusKindWire::Candidate,
        ExecutionStatusKindWire::Settled,
        ExecutionStatusKindWire::Cancelled,
        ExecutionStatusKindWire::Failed,
    ] {
        let status = ExecutionStatusWire {
            execution_id: "exec-中文🧪".into(),
            request_id: Some("request-1".into()),
            sequence: WireU64::from_u64(u64::MAX),
            phase: AgentPhaseWire::Terminal,
            status: kind,
            reason: Some("原始结算原因".into()),
        };
        status.validate()?;
        let event = AgentEventWire::ExecutionStatus(status);
        assert_eq!(
            serde_json::from_value::<AgentEventWire>(serde_json::to_value(&event)?)?,
            event
        );
    }
    let value = WireValue::Variant {
        type_id: AGENT_EVENT_WIRE_TYPE_ID.into(),
        variant: "execution_status".into(),
        fields: vec![
            WireField {
                name: "execution_id".into(),
                value: WireValue::String("exec-1".into()),
            },
            WireField {
                name: "sequence".into(),
                value: WireValue::U64(WireU64::from_u64(0)),
            },
            WireField {
                name: "phase".into(),
                value: WireValue::String("Terminal".into()),
            },
            WireField {
                name: "status".into(),
                value: WireValue::String("settled".into()),
            },
        ],
    };
    let event = AgentEventWire::from_wire_value(&value)?;
    assert!(matches!(
        event,
        AgentEventWire::ExecutionStatus(ExecutionStatusWire {
            status: ExecutionStatusKindWire::Settled,
            phase: AgentPhaseWire::Terminal,
            ..
        })
    ));
    Ok(())
}

fn block(phase: AssistantPhaseWire) -> AssistantMessageBlockWire {
    AssistantMessageBlockWire {
        provider_source: None,
        block_id: "message-中文🧪".to_string(),
        request_id: "response-2".to_string(),
        sequence: WireU64::from_u64(u64::MAX),
        phase,
        delta: "验证结果🙂".to_string(),
        complete: true,
        provider: Some("responses".to_string()),
        reasoning: None,
    }
}

#[test]
fn native_message_blocks_keep_phase_identity_order_and_opaque_reasoning() -> TestResult {
    for phase in [
        AssistantPhaseWire::Reasoning,
        AssistantPhaseWire::Commentary,
        AssistantPhaseWire::PartialAnswer,
        AssistantPhaseWire::FinalAnswer,
        AssistantPhaseWire::Unknown,
    ] {
        for reasoning in [
            None,
            Some(LlmReasoningBlockWire::Signed {
                thinking: "原生思考".to_string(),
                signature: "signature-exact".to_string(),
            }),
            Some(LlmReasoningBlockWire::Redacted {
                data: "encrypted==".to_string(),
            }),
            Some(LlmReasoningBlockWire::Opaque {
                provider: "responses".to_string(),
                id: "reasoning-1".to_string(),
                data: "opaque-exact==".to_string(),
                summary: vec!["公开摘要".to_string()],
            }),
        ] {
            let mut value = block(phase);
            value.reasoning = reasoning;
            value.validate()?;
            let event = AgentEventWire::AssistantMessage { block: value };
            let json = serde_json::to_value(&event)?;
            assert_eq!(
                serde_json::from_value::<AgentEventWire>(json.clone())?,
                event
            );
            // FinalAnswer phase仍是普通块；不允许把它塞入CustomAgent终态通道。
            assert!(serde_json::from_value::<AgentStreamChunkWire>(json.clone()).is_ok());
            assert!(serde_json::from_value::<AgentStreamTerminalWire>(json).is_err());
        }
    }
    Ok(())
}

#[test]
fn native_message_missing_phase_is_unknown_and_numbers_are_lossless() -> TestResult {
    let mut json = serde_json::to_value(block(AssistantPhaseWire::Commentary))?;
    let object = json.as_object_mut().ok_or("block must be an object")?;
    object.remove("phase");
    let parsed: AssistantMessageBlockWire = serde_json::from_value(json.clone())?;
    assert_eq!(parsed.phase, AssistantPhaseWire::Unknown);
    assert_eq!(parsed.sequence.to_u64(), Some(u64::MAX));
    let object = json.as_object_mut().ok_or("block must be an object")?;
    object.insert("sequence".to_string(), serde_json::json!(u64::MAX));
    assert!(serde_json::from_value::<AssistantMessageBlockWire>(json).is_err());
    Ok(())
}

#[test]
fn native_message_closed_wire_decoder_rejects_bad_identity_and_extra_fields() -> TestResult {
    let make = |identity: &str| WireValue::Variant {
        type_id: AGENT_EVENT_WIRE_TYPE_ID.to_string(),
        variant: "assistant_message".to_string(),
        fields: vec![WireField {
            name: "block".to_string(),
            value: WireValue::Record {
                type_id: "echo_sdk_protocol::methods::AssistantMessageBlockWire".to_string(),
                fields: vec![
                    WireField {
                        name: "block_id".to_string(),
                        value: WireValue::String(identity.to_string()),
                    },
                    WireField {
                        name: "request_id".to_string(),
                        value: WireValue::String("request-1".to_string()),
                    },
                    WireField {
                        name: "sequence".to_string(),
                        value: WireValue::U64(WireU64::from_u64(0)),
                    },
                    WireField {
                        name: "delta".to_string(),
                        value: WireValue::String("正文".to_string()),
                    },
                    WireField {
                        name: "complete".to_string(),
                        value: WireValue::Bool(false),
                    },
                ],
            },
        }],
    };
    assert!(matches!(
        AgentEventWire::from_wire_value(&make("block-1"))?,
        AgentEventWire::AssistantMessage { .. }
    ));
    assert!(AgentEventWire::from_wire_value(&make(" ")).is_err());
    let mut json = serde_json::to_value(block(AssistantPhaseWire::Commentary))?;
    json.as_object_mut()
        .ok_or("block must be an object")?
        .insert("settled".to_string(), true.into());
    assert!(serde_json::from_value::<AssistantMessageBlockWire>(json).is_err());
    Ok(())
}

#[test]
fn native_message_history_survives_message_and_stream_dtos() -> TestResult {
    let mut value = block(AssistantPhaseWire::FinalAnswer);
    value.reasoning = Some(LlmReasoningBlockWire::Opaque {
        provider: "responses".into(),
        id: "opaque-id".into(),
        data: "encrypted==".into(),
        summary: vec!["摘要🙂".into()],
    });
    let message = LlmMessageWire {
        role: "assistant".into(),
        content: WireValue::String("正文".into()),
        tool_calls: None,
        name: None,
        tool_call_id: None,
        reasoning_content: None,
        reasoning_blocks: None,
        assistant_blocks: Some(vec![value.clone()]),
        approved_assistant_blocks: Some(Vec::new()),
        execution_status: Some(echo_sdk_protocol::methods::ExecutionStatusWire {
            execution_id: "history-execution".into(),
            request_id: Some("response-2".into()),
            sequence: WireU64::from_u64(u64::MAX),
            phase: echo_sdk_protocol::methods::AgentPhaseWire::Terminal,
            status: echo_sdk_protocol::methods::ExecutionStatusKindWire::Candidate,
            reason: None,
        }),
    };
    assert_eq!(
        serde_json::from_value::<LlmMessageWire>(serde_json::to_value(&message)?)?,
        message
    );
    let chunk = LlmStreamChunkWire {
        assistant_blocks: Some(vec![value.clone()]),
        ..Default::default()
    };
    let restored: LlmChatChunkWire = serde_json::from_value(serde_json::to_value(chunk)?)?;
    assert_eq!(restored.assistant_blocks, Some(vec![value.clone()]));
    let complete = LlmStreamCompleteWire {
        role: None,
        content: None,
        reasoning_content: None,
        reasoning_blocks: None,
        assistant_blocks: Some(vec![value.clone()]),
        tool_calls: None,
        finish_reason: "stop".into(),
        usage: None,
    };
    let restored: LlmChatChunkWire = serde_json::from_value(serde_json::to_value(complete)?)?;
    assert_eq!(restored.assistant_blocks, Some(vec![value]));
    Ok(())
}

#[test]
fn native_message_wire_record_keeps_signed_redacted_and_opaque_fields() -> TestResult {
    for (kind, fields, expected) in [
        (
            "signed",
            vec![
                WireField {
                    name: "thinking".into(),
                    value: WireValue::String("签名思考".into()),
                },
                WireField {
                    name: "signature".into(),
                    value: WireValue::String("exact-signature==".into()),
                },
            ],
            LlmReasoningBlockWire::Signed {
                thinking: "签名思考".into(),
                signature: "exact-signature==".into(),
            },
        ),
        (
            "redacted",
            vec![WireField {
                name: "data".into(),
                value: WireValue::String("encrypted==".into()),
            }],
            LlmReasoningBlockWire::Redacted {
                data: "encrypted==".into(),
            },
        ),
        (
            "opaque",
            vec![
                WireField {
                    name: "provider".into(),
                    value: WireValue::String("responses".into()),
                },
                WireField {
                    name: "id".into(),
                    value: WireValue::String("opaque-1".into()),
                },
                WireField {
                    name: "data".into(),
                    value: WireValue::String("opaque==".into()),
                },
                WireField {
                    name: "summary".into(),
                    value: WireValue::List(vec![WireValue::String("摘要🙂".into())]),
                },
            ],
            LlmReasoningBlockWire::Opaque {
                provider: "responses".into(),
                id: "opaque-1".into(),
                data: "opaque==".into(),
                summary: vec!["摘要🙂".into()],
            },
        ),
    ] {
        let mut reasoning = vec![WireField {
            name: "kind".into(),
            value: WireValue::String(kind.into()),
        }];
        reasoning.extend(fields);
        let value = WireValue::Variant {
            type_id: AGENT_EVENT_WIRE_TYPE_ID.into(),
            variant: "assistant_message".into(),
            fields: vec![WireField {
                name: "block".into(),
                value: WireValue::Record {
                    type_id: "echo_sdk_protocol::methods::AssistantMessageBlockWire".into(),
                    fields: vec![
                        WireField {
                            name: "block_id".into(),
                            value: WireValue::String("block".into()),
                        },
                        WireField {
                            name: "request_id".into(),
                            value: WireValue::String("request".into()),
                        },
                        WireField {
                            name: "sequence".into(),
                            value: WireValue::U64(WireU64::from_u64(u64::MAX)),
                        },
                        WireField {
                            name: "phase".into(),
                            value: WireValue::String("reasoning".into()),
                        },
                        WireField {
                            name: "delta".into(),
                            value: WireValue::String("".into()),
                        },
                        WireField {
                            name: "complete".into(),
                            value: WireValue::Bool(true),
                        },
                        WireField {
                            name: "reasoning".into(),
                            value: WireValue::Record {
                                type_id: "echo_sdk_protocol::methods::LlmReasoningBlockWire".into(),
                                fields: reasoning,
                            },
                        },
                    ],
                },
            }],
        };
        let AgentEventWire::AssistantMessage { block } = AgentEventWire::from_wire_value(&value)?
        else {
            return Err("native assistant event changed variant".into());
        };
        assert_eq!(block.reasoning, Some(expected));
        assert_eq!(block.sequence.to_u64(), Some(u64::MAX));
        assert_eq!(block.phase, AssistantPhaseWire::Reasoning);
    }
    Ok(())
}
