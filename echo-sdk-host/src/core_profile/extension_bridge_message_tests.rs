//! 通过原 ACP reverse callback 核验真实 dyn Agent override；不另造执行器或 owner。
use super::*;
use agent_client_protocol::{Agent as AcpAgent, ByteStreams, Error as RpcError, Responder};
use echo_agent::agent::AgentInvocationContext;
use echo_agent::llm::types::{ContentPart, LinkedResource, Message, MessageContent, Role};
use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};

fn rich_message(meta: Option<bool>) -> Result<Message, String> {
    let mut message = Message::tool_result("call-中文🧪".into(), "shell".into(), "output".into());
    message.content = MessageContent::Parts(vec![
        ContentPart::Text { text: "附件原文🙂".into() },
        ContentPart::ResourceLink {
            resource: Box::new(LinkedResource {
                annotations: meta.map(|_| serde_json::from_value(serde_json::json!({"priority":0.75,"audience":["user"]})).map_err(|error| error.to_string())).transpose()?,
                description: Some("原始附件".into()),
                mime_type: Some("text/plain".into()),
                name: "资料🧪.txt".into(),
                size: Some(0),
                title: None,
                uri: "resource://fixture/原文".into(),
                meta: meta.map(|empty| if empty { serde_json::json!({}) } else {
                    serde_json::json!({"binding":{"id":"original-🧪","values":[],"empty":{},"none":null}})
                }),
            }),
        },
    ]);
    message.tool_calls = Some(Vec::new());
    message.reasoning_blocks = Some(Vec::new());
    super::super::native_message::with_message_metadata(
        message,
        super::super::native_message::MessageMetadata {
            assistant_blocks: Some(Vec::new()),
            approved_assistant_blocks: Some(Vec::new()),
            execution_status: None,
        },
    )
}

fn text_tool_message() -> Message {
    let mut message = Message::tool_result(
        "text-call-🧪".into(),
        "shell".into(),
        "纯文本也保留tool角色".into(),
    );
    message.reasoning_content = Some(String::new());
    message.reasoning_blocks = Some(Vec::new());
    message
}

#[test]
fn llm_request_tool_choice_field_round_trips_current_core_values() -> Result<(), String> {
    use echo_agent::llm::{ChatRequest, ToolChoice};
    for choice in [
        None,
        Some(ToolChoice::Auto),
        Some(ToolChoice::None),
        Some(ToolChoice::Required),
        Some(ToolChoice::function(" 原始工具🧪 ")),
        Some(ToolChoice::function("\u{feff}")),
    ] {
        let mut request = ChatRequest::new(Vec::new());
        request.tool_choice = choice.clone();
        let wire = ExtensionLlmClientProxy::request_wire(&request)?;
        let restored: Option<ToolChoice> = from_wire(to_wire(&wire.tool_choice)?)?;
        assert_eq!(
            restored, choice,
            "Core → request bridge → Core不能丢具名工具或改写原名"
        );
        assert_eq!(
            serde_json::to_value(&wire.tool_choice).map_err(|e| e.to_string())?,
            serde_json::to_value(choice).map_err(|e| e.to_string())?
        );
    }
    for value in [
        serde_json::json!("unknown"),
        serde_json::json!({"type":"tool","name":"x"}),
        serde_json::json!({"type":"function","function":{"name":""}}),
        serde_json::json!({"type":"function","function":{"name":"\u{00a0}"}}),
        serde_json::json!({"type":"function","function":{"name":"x","extra":true}}),
    ] {
        assert!(
            from_wire::<ToolChoice>(
                WireValue::from_json(value.clone()).map_err(|e| e.to_string())?
            )
            .is_err()
        );
        assert!(
            serde_json::from_value::<echo_sdk_protocol::methods::LlmChatRequestWire>(
                serde_json::json!({"messages":[],"tool_choice":value})
            )
            .is_err()
        );
    }
    Ok(())
}

#[test]
fn provider_response_message_values_and_observation_only_empty_round_trip() -> Result<(), String> {
    let observations = [
        serde_json::json!({"protocol":"responses","provider":"fixture"}),
        serde_json::json!({"protocol":"responses","provider":"fixture","model":null,"stop_reason":null,
            "stop_sequence":null,"status":null,"incomplete_details":null,"usage":null}),
        serde_json::json!({"protocol":"anthropic","provider":"fixture","request_id":"original-request",
            "model":"future-model","stop_reason":"unknown-provider-stop","stop_sequence":"00123","status":"future-status",
            "incomplete_details":{"nested":{"none":null,"values":[]}},
            "usage":{"input":u64::MAX,"nested":{"cache":12,"text":"00123","null":null}}}),
    ];
    for observation in observations {
        for flag in [None, Some(false), Some(true)] {
            let mut message = rich_message(Some(true))?;
            message.role = Role::Custom("provider-specific-角色🧪".into());
            message.tool_result_is_error = flag;
            message.provider_response = Some(
                serde_json::from_value(observation.clone()).map_err(|error| error.to_string())?,
            );
            let expected = serde_json::to_value(&message).map_err(|error| error.to_string())?;
            let restored = message_from_wire(message_wire(&message)?)?;
            assert_eq!(
                serde_json::to_value(restored).map_err(|error| error.to_string())?,
                expected
            );
        }
        let metadata = serde_json::from_value(observation).map_err(|error| error.to_string())?;
        let message = Message::provider_response_observation(metadata);
        assert!(message.is_provider_response_observation_only());
        let wire = message_wire(&message)?;
        assert_eq!(wire.content, WireValue::Null);
        let restored = message_from_wire(wire)?;
        assert!(matches!(restored.content, MessageContent::Empty));
        assert!(
            restored.is_provider_response_observation_only(),
            "wire roundtrip cannot turn metadata-only observation into a model-visible text message"
        );
    }
    Ok(())
}

#[test]
fn provider_response_delta_snapshots_keep_null_usage_and_request_isolation() -> Result<(), String> {
    for request in ["request-A", "request-B"] {
        let expected = serde_json::json!({"protocol":"responses","provider":"fixture","request_id":request,
            "status":null,"stop_reason":"future-stop","usage":{"tokens":u64::MAX,"nested":{"text":"00123"}}});
        let metadata =
            serde_json::from_value(expected.clone()).map_err(|error| error.to_string())?;
        let wire = super::super::native_message::provider_response_wire(&metadata)?;
        for _ in 0..2 {
            let value = chat_chunk_from_wire(LlmChatChunkWire {
                provider_response: Some(wire.clone()),
                assistant_blocks: Some(Vec::new()),
                ..Default::default()
            })?;
            let observed = value
                .delta
                .provider_response
                .as_ref()
                .ok_or("delta lost observation")?;
            assert_eq!(
                serde_json::to_value(observed).map_err(|error| error.to_string())?,
                expected,
                "same request snapshot is transported, never summed or inherited from prior request"
            );
            assert_eq!(value.delta.assistant_blocks.as_ref().map(Vec::len), Some(0));
            assert_eq!(
                chat_chunk_wire(&value)?.provider_response,
                Some(wire.clone())
            );
        }
    }
    Ok(())
}

#[test]
fn partial_tool_sources_keep_unknown_or_late_native_id_and_sparse_original_positions()
-> Result<(), String> {
    for request in ["", "real-native-A", "real-native-B"] {
        let mut message = Message::assistant("partial source text".into());
        message.tool_calls = Some([5_u32, 9].into_iter().map(|index| {
            super::super::native_message::tool_call_from_wire(echo_sdk_protocol::methods::LlmToolCallWire {
                id: format!("original-call-{index}"), call_type:"function".into(), function_name:"original_tool".into(), arguments:"{\"original\":true}".into(),
                provider_source:Some(echo_sdk_protocol::methods::ProviderOutputSourceWire {
                    protocol:echo_sdk_protocol::methods::LlmApiProtocolWire::ChatCompletions,
                    request_id:request.into(), item_id:format!("original-call-{index}"), output_index:index,
                    content_index:None, content_kind:Some("tool_call".into()), phase:None,
                }),
            })
        }).collect::<Result<Vec<_>,String>>()?);
        // 未观察到的原生 request ID 只属于 D2 partial，不构造成功重放或 SDK owner。
        let error = echo_agent::error::LlmError::IncompleteResponse {
            partial: Box::new(message),
        };
        let echo_agent::error::LlmError::IncompleteResponse { partial } = error else {
            return Err("fixture lost the framework incomplete-response boundary".into());
        };
        let expected = serde_json::to_value(&partial).map_err(|error| error.to_string())?;
        let wire = message_wire(&partial)?;
        let calls = wire.tool_calls.as_ref().ok_or("missing partial tools")?;
        assert_eq!(
            calls
                .iter()
                .filter_map(|call| call
                    .provider_source
                    .as_ref()
                    .map(|source| source.output_index))
                .collect::<Vec<_>>(),
            vec![5, 9]
        );
        assert!(calls.iter().all(|call| {
            call.provider_source
                .as_ref()
                .is_some_and(|source| source.request_id == request)
        }));
        assert_eq!(
            serde_json::to_value(message_from_wire(wire)?).map_err(|error| error.to_string())?,
            expected
        );
    }
    Ok(())
}

#[test]
fn custom_agent_message_roundtrip_preserves_parts_roles_tool_identity_and_empty_values()
-> Result<(), String> {
    let custom_role = "provider-specific-角色🧪";
    let mut custom_message = Message::user("自定义角色原文".into());
    custom_message.role = Role::Custom(custom_role.into());
    for message in [
        Message::default(),
        Message::user("便利文本🧪".into()),
        text_tool_message(),
        rich_message(Some(false))?,
        rich_message(Some(true))?,
        rich_message(None)?,
        custom_message.clone(),
    ] {
        let expected = serde_json::to_value(&message).map_err(|error| error.to_string())?;
        let wire = message_wire(&message)?;
        let restored = message_from_wire(wire)?;
        assert_eq!(
            serde_json::to_value(restored).map_err(|error| error.to_string())?,
            expected
        );
    }
    // Custom是框架当前公开能力；未知provider角色必须原样保留，不能降成User或拒绝。
    let wire = message_wire(&custom_message)?;
    let restored = message_from_wire(wire.clone())?;
    assert_eq!(restored.role, Role::Custom(custom_role.into()));
    assert_ne!(restored.role, Role::User);

    let valid_wire = serde_json::to_value(wire).map_err(|error| error.to_string())?;
    for invalid_role in [
        serde_json::Value::Null,
        serde_json::json!(7),
        serde_json::json!({ "custom": custom_role }),
    ] {
        let mut invalid_wire = valid_wire.clone();
        invalid_wire
            .as_object_mut()
            .ok_or_else(|| "Message wire must be an object".to_string())?
            .insert("role".into(), invalid_role);
        assert!(serde_json::from_value::<LlmMessageWire>(invalid_wire).is_err());
    }
    let mut missing_role = valid_wire;
    missing_role
        .as_object_mut()
        .ok_or_else(|| "Message wire must be an object".to_string())?
        .remove("role");
    assert!(serde_json::from_value::<LlmMessageWire>(missing_role).is_err());
    Ok(())
}

#[tokio::test]
async fn custom_agent_real_message_overrides_preserve_payload_and_original_invocation()
-> Result<(), String> {
    custom_agent_message_and_output_case(false).await
}

#[tokio::test]
async fn custom_agent_native_output_keeps_original_status_reasoning_and_final_notice_through_eof()
-> Result<(), String> {
    custom_agent_message_and_output_case(true).await
}

fn native_output_fixture() -> Vec<AgentEventWire> {
    use echo_sdk_protocol::methods::{
        AgentPhaseWire, AssistantMessageBlockWire, AssistantPhaseWire, ExecutionStatusKindWire,
        ExecutionStatusWire, LlmReasoningBlockWire,
    };
    let status = |sequence, kind, phase| {
        AgentEventWire::ExecutionStatus(ExecutionStatusWire {
            execution_id: "native-original-execution".into(),
            request_id: Some("native-original-request".into()),
            sequence: WireU64::from_u64(sequence),
            phase,
            status: kind,
            reason: Some("原生原因🧪".into()),
        })
    };
    vec![
        status(
            0,
            ExecutionStatusKindWire::Started,
            AgentPhaseWire::Thinking,
        ),
        AgentEventWire::AssistantMessage {
            block: AssistantMessageBlockWire {
                block_id: "原块🧪".into(),
                request_id: "native-original-request".into(),
                sequence: WireU64::from_u64(u64::MAX),
                phase: AssistantPhaseWire::Reasoning,
                delta: String::new(),
                complete: true,
                provider: Some("responses".into()),
                provider_source: None,
                reasoning: Some(LlmReasoningBlockWire::Opaque {
                    provider: "responses".into(),
                    id: "original-item".into(),
                    data: "encrypted-original==".into(),
                    summary: vec!["原摘要".into()],
                }),
            },
        },
        status(
            1,
            ExecutionStatusKindWire::Waiting,
            AgentPhaseWire::Thinking,
        ),
        status(
            2,
            ExecutionStatusKindWire::Candidate,
            AgentPhaseWire::Thinking,
        ),
        AgentEventWire::FinalAnswer {
            text: "callback original result".into(),
        },
        status(
            u64::MAX,
            ExecutionStatusKindWire::Settled,
            AgentPhaseWire::Terminal,
        ),
    ]
}

async fn custom_agent_message_and_output_case(native_output: bool) -> Result<(), String> {
    let directory = tempfile::tempdir().map_err(|error| error.to_string())?;
    let config: crate::SdkHostConfig = serde_json::from_value(serde_json::json!({
        "schema_version":1,"default_agent":{
            "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
            "agent":{"name":"fixture","enable_tools":true}
        }
    })).map_err(|error| error.to_string())?;
    let prepared = config.prepare().map_err(|error| error.to_string())?;
    let state = CoreProfileState::install(
        directory.path(),
        crate::config::SdkProfileLimits::default(),
        prepared.framework,
        prepared.llm_client,
    )
    .map_err(|error| error.to_string())?;
    let sessions = Arc::new(echo_agent::acp::SessionRegistry::new(
        Arc::new(state.session_factory.clone()),
        4,
    ));
    state.bind_services(Arc::new(echo_agent::acp::AcpConnectionServices::new(
        sessions,
        Arc::new(echo_agent::acp::AcpAdapterConfig::default()),
    )));
    let bridge = state.extension_bridge.clone();
    let captures = Arc::new(Mutex::new(Vec::<(
        ExtensionInvokeCall,
        Option<AgentInvocationContext>,
    )>::new()));
    let sdk_captures = captures.clone();
    let sdk_state = state.clone();
    let finish = CancellationToken::new();
    let sdk_finish = finish.clone();
    let (host_io, sdk_io) = tokio::io::duplex(65_536);
    let (host_read, host_write) = tokio::io::split(host_io);
    let (sdk_read, sdk_write) = tokio::io::split(sdk_io);
    let sdk = Client
        .builder()
        .on_receive_request(
            async move |call: ExtensionInvokeCall,
                        responder: Responder<ExtensionInvokeOutcome>,
                        connection: ConnectionTo<AcpAgent>| {
                // callback_owners只保留Unknown，正常请求必须沿原Host签发stream找同一个owner。
                let owner = {
                    let stream = call.stream.as_ref().ok_or_else(|| {
                        RpcError::internal_error().data("original stream missing")
                    })?;
                    let sink = sdk_state
                        .extension_shared
                        .stream_sink(&stream.id)
                        .ok_or_else(|| {
                            RpcError::internal_error().data("original stream sink missing")
                        })?;
                    let owner = sink
                        .owner
                        .lock()
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?
                        .upgrade()
                        .ok_or_else(|| {
                            RpcError::internal_error().data("original stream owner missing")
                        })?;
                    let record = sdk_state
                        .handles
                        .extension(&call.extension)
                        .map_err(|error| RpcError::internal_error().data(error.message))?;
                    let original_record = owner
                        .registration
                        .lock()
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?
                        .upgrade()
                        .ok_or_else(|| {
                            RpcError::internal_error().data("original owner registration missing")
                        })?;
                    assert!(
                        Arc::ptr_eq(&record, &original_record),
                        "stream owner must be the original invocation registration"
                    );
                    owner
                };
                let invocation = owner
                    ._agent_invocation
                    .lock()
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?
                    .clone();
                sdk_captures
                    .lock()
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?
                    .push((call.clone(), invocation));
                let stream = call
                    .stream
                    .ok_or_else(|| RpcError::internal_error().data("real stream handle missing"))?;
                responder.respond(ExtensionInvokeOutcome::Stream {
                    stream: stream.clone(),
                })?;
                if native_output {
                    let original = native_output_fixture();
                    let count = original.len();
                    for (index, event) in original.into_iter().enumerate() {
                        let sequence = u64::try_from(index)
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?
                            .checked_add(1)
                            .ok_or_else(|| {
                                RpcError::internal_error().data("fixture sequence overflow")
                            })?;
                        let sequence = echo_sdk_protocol::scalar::WireNonZeroU64::try_from(
                            sequence.to_string(),
                        )
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                        let wire = serde_json::to_value(event)
                            .map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                        let event = if index.checked_add(1) == Some(count) {
                            ExtensionStreamEvent::Complete {
                                stream: stream.clone(),
                                sequence,
                                value: ExtensionStreamCompleteValue::Agent(
                                    serde_json::from_value(wire).map_err(|error| {
                                        RpcError::internal_error().data(error.to_string())
                                    })?,
                                ),
                            }
                        } else {
                            ExtensionStreamEvent::Chunk {
                                stream: stream.clone(),
                                sequence,
                                value: ExtensionStreamChunkValue::Agent(
                                    serde_json::from_value(wire).map_err(|error| {
                                        RpcError::internal_error().data(error.to_string())
                                    })?,
                                ),
                            }
                        };
                        connection.send_notification(event)?;
                    }
                    return Ok(());
                }
                connection.send_notification(ExtensionStreamEvent::Complete {
                    stream,
                    sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string())
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    value: ExtensionStreamCompleteValue::Agent(
                        AgentStreamTerminalWire::FinalAnswer {
                            text: "callback original result".into(),
                        },
                    ),
                })
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_with(
            ByteStreams::new(sdk_write.compat_write(), sdk_read.compat()),
            async move |_| {
                sdk_finish.cancelled().await;
                Ok(())
            },
        );
    let notification_shared = state.extension_shared.clone();
    let host = AcpAgent.builder().on_receive_notification(
        async move |event: ExtensionStreamEvent, _connection: ConnectionTo<Client>| {
            notification_shared.deliver_stream_event(event).map_err(|error| RpcError::internal_error().data(error))
        }, agent_client_protocol::on_receive_notification!()
    ).connect_with(ByteStreams::new(host_write.compat_write(), host_read.compat()), async move |connection| {
        let _finish_on_exit = finish.drop_guard();
        bridge.shared.bind_connection(connection);
        let descriptor = serde_json::from_value(serde_json::json!({
            "kind":"custom_agent","descriptor_version":1,"name":"rich-message","model_name":"fixture","system_prompt":"typed forwarding","tool_names":[]
        })).map_err(|error| RpcError::internal_error().data(error.to_string()))?;
        let (extension, _) = state.handles.register_extension(
            super::super::handles::ExtensionRegistrationLimits { max_extensions:4, max_descriptor_bytes:65_536 },
            ExtensionKind::CustomAgent, "rich-message", descriptor, None, false,
        ).map_err(|error| RpcError::internal_error().data(error.message))?;
        let proxy: Box<dyn Agent> = Box::new(ExtensionCustomAgentProxy::for_registration(bridge, extension, "rich-session".into())
            .ok_or_else(|| RpcError::internal_error().data("registered proxy unavailable"))?);
        for (index, message) in [Message::default(), text_tool_message(), rich_message(Some(false)).map_err(|error| RpcError::internal_error().data(error))?, rich_message(Some(true)).map_err(|error| RpcError::internal_error().data(error))?, rich_message(None).map_err(|error| RpcError::internal_error().data(error))?].into_iter().enumerate() {
            for chat in [false, true] {
                for explicit in [false, true] {
                    let expected = serde_json::to_value(&message).map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                    let invocation = AgentInvocationContext {
                        working_dir: Some(directory.path().to_path_buf()),
                        tool_access: Some(echo_agent::tools::ToolAccess::ReadOnly),
                        allowed_tools: Some(Default::default()),
                        history: Some(if index == 2 { Vec::new() } else { vec![message.clone()] }),
                        runtime: Some(echo_agent::tools::ExternalRunContext {
                            run_id: Some(format!("scope-{index}")), execution_id: Some(format!("attempt-{index}")),
                            ..Default::default()
                        }),
                        ..Default::default()
                    };
                    let cancel = CancellationToken::new();
                    let stream = match (chat, explicit) {
                        (false, false) => proxy.as_ref().execute_stream_message_with_cancel(message.clone(), cancel).await,
                        (true, false) => proxy.as_ref().chat_stream_message_with_cancel(message.clone(), cancel).await,
                        (false, true) => proxy.as_ref().execute_stream_message_with_invocation_context(message.clone(), cancel, invocation).await,
                        (true, true) => proxy.as_ref().chat_stream_message_with_invocation_context(message.clone(), cancel, invocation).await,
                    }.map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                    let events = stream.collect::<Vec<_>>().await;
                    if native_output {
                        let actual = events.iter().map(|event| {
                            let event = event.as_ref().map_err(|error| error.to_string())?;
                            agent_event_wire_from_framework(event)
                        }).collect::<Result<Vec<_>, String>>().map_err(|error| RpcError::internal_error().data(error))?;
                        assert_eq!(actual, native_output_fixture(), "FinalAnswer cannot hide original status, EOF, identity, u64 or encrypted reasoning");
                    } else {
                        assert_eq!(events.len(), 1);
                        assert!(matches!(events.first(), Some(Ok(AgentEvent::FinalAnswer(value))) if value == "callback original result"));
                    }
                    let (call, retained) = captures.lock().map_err(|error| RpcError::internal_error().data(error.to_string()))?
                        .last().cloned().ok_or_else(|| RpcError::internal_error().data("callback was bypassed by a text/default entry"))?;
                    let (delivered, delivered_history) = match (chat, call.invocation) {
                        (false, ExtensionInvocation::AgentExecuteStream(input)) => (input.task, input.history),
                        (true, ExtensionInvocation::AgentChatStream(input)) => (input.message, input.history),
                        _ => return Err(RpcError::internal_error().data("wrong invocation override/operation")),
                    };
                    let restored = message_from_wire(delivered).map_err(|error| RpcError::internal_error().data(error))?;
                    assert_eq!(serde_json::to_value(restored).map_err(|error| RpcError::internal_error().data(error.to_string()))?, expected);
                    let expected_history = explicit.then(|| if index == 2 { Vec::new() } else { vec![expected.clone()] });
                    let delivered_history = delivered_history.map(|history| history.into_iter().map(|item| {
                        let native = message_from_wire(item)?;
                        serde_json::to_value(native).map_err(|error| error.to_string())
                    }).collect::<Result<Vec<_>, String>>()).transpose().map_err(|error| RpcError::internal_error().data(error))?;
                    assert_eq!(delivered_history, expected_history, "remote history cannot be retained only in local owner");
                    let retained = retained.ok_or_else(|| RpcError::internal_error().data("original invocation was not retained"))?;
                    if explicit {
                        assert_eq!(retained.working_dir.as_deref(), Some(directory.path()));
                        assert_eq!(retained.tool_access, Some(echo_agent::tools::ToolAccess::ReadOnly));
                        assert_eq!(retained.allowed_tools.as_ref().map(|tools| tools.len()), Some(0));
                        let history = retained.history.ok_or_else(|| RpcError::internal_error().data("original history missing"))?;
                        let retained_history = history.iter().map(serde_json::to_value).collect::<Result<Vec<_>, _>>().map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                        assert_eq!(Some(retained_history), expected_history);
                        assert_eq!(call.context.and_then(|value| value.run_id), Some(format!("scope-{index}")));
                    } else {
                        assert!(retained.working_dir.is_none() && retained.history.is_none() && retained.tool_access.is_none());
                    }
                }
            }
        }
        assert_eq!(captures.lock().map_err(|error| RpcError::internal_error().data(error.to_string()))?.len(), 20);
        Ok(())
    });
    let (host, sdk) =
        tokio::time::timeout(Duration::from_secs(10), async { tokio::join!(host, sdk) })
            .await
            .map_err(|error| error.to_string())?;
    host.map_err(|error| error.to_string())?;
    sdk.map_err(|error| error.to_string())?;
    Ok(())
}
