//! 通过原 ACP reverse callback 核验真实 dyn Agent override；不另造执行器或 owner。
use super::*;
use agent_client_protocol::{Agent as AcpAgent, ByteStreams, Error as RpcError, Responder};
use echo_agent::agent::AgentInvocationContext;
use echo_agent::llm::types::{ContentPart, LinkedResource, Message, MessageContent};
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
fn custom_agent_message_roundtrip_preserves_parts_roles_tool_identity_and_empty_values()
-> Result<(), String> {
    for message in [
        Message::default(),
        Message::user("便利文本🧪".into()),
        text_tool_message(),
        rich_message(Some(false))?,
        rich_message(Some(true))?,
        rich_message(None)?,
    ] {
        let expected = serde_json::to_value(&message).map_err(|error| error.to_string())?;
        let wire = message_wire(&message)?;
        let restored = message_from_wire(wire)?;
        assert_eq!(
            serde_json::to_value(restored).map_err(|error| error.to_string())?,
            expected
        );
    }
    let mut wire = message_wire(&Message::user("invalid role".into()))?;
    wire.role = "not-a-native-role".into();
    assert!(
        message_from_wire(wire).is_err(),
        "未知 role 不能降级为 user"
    );
    Ok(())
}

#[tokio::test]
async fn custom_agent_real_message_overrides_preserve_payload_and_original_invocation()
-> Result<(), String> {
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
                let record = sdk_state
                    .handles
                    .extension(&call.extension)
                    .map_err(|error| RpcError::internal_error().data(error.message))?;
                let owner = record
                    .callback_owners
                    .lock()
                    .map_err(|error| RpcError::internal_error().data(error.to_string()))?
                    .iter()
                    .find(|owner| !owner.finished.is_cancelled())
                    .cloned()
                    .ok_or_else(|| {
                        RpcError::internal_error().data("original callback owner missing")
                    })?;
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
                    assert_eq!(events.len(), 1);
                    assert!(matches!(events.first(), Some(Ok(AgentEvent::FinalAnswer(value))) if value == "callback original result"));
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
