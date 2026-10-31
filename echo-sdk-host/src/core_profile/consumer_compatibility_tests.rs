//! 真实 Host request/message adapter 的回归，非自建转换替身。
use super::*;
use serde_json::json;

#[path = "sandbox_command_owner_tests.rs"]
mod sandbox_command_owner_tests;

/// 直接调用真实LlmClient代理并经过原ACP reverse callback；不以手工DTO往返代替bridge。
#[tokio::test]
async fn consumer_compatibility_named_choice_uses_actual_llm_chat_and_stream_bridge()
-> Result<(), String> {
    use agent_client_protocol::{Agent as AcpAgent, ByteStreams, Error as RpcError, Responder};
    use echo_agent::llm::{ChatRequest, LlmClient, ToolChoice};
    use tokio_util::compat::{TokioAsyncReadCompatExt as _, TokioAsyncWriteCompatExt as _};

    let runtime_root = std::env::var("ECHO_AGENT_SDK_CONSUMER_ROOT")
        .map_err(|error| format!("explicit task runtime directory required: {error}"))?;
    std::fs::create_dir_all(&runtime_root).map_err(|error| error.to_string())?;
    let directory = tempfile::Builder::new()
        .prefix("tool-choice-bridge-")
        .tempdir_in(runtime_root)
        .map_err(|error| error.to_string())?;
    let config: crate::SdkHostConfig = serde_json::from_value(json!({
        "schema_version":1,"default_agent":{
            "model":{"provider":"fixture","name":"fixture","base_url":"http://127.0.0.1:1/v1","api_protocol":"chat_completions","auth_token":"fixture"},
            "agent":{"name":"choice-fixture","enable_tools":true}
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
    let captures = Arc::new(Mutex::new(Vec::<(bool, Option<ToolChoice>)>::new()));
    let sdk_captures = captures.clone();
    let finish = CancellationToken::new();
    let sdk_finish = finish.clone();
    let (host_io, sdk_io) = tokio::io::duplex(65_536);
    let (host_read, host_write) = tokio::io::split(host_io);
    let (sdk_read, sdk_write) = tokio::io::split(sdk_io);
    let sdk = Client.builder().on_receive_request(
        async move |call: ExtensionInvokeCall, responder: Responder<ExtensionInvokeOutcome>, connection: ConnectionTo<AcpAgent>| {
            let (streaming, request) = match call.invocation {
                ExtensionInvocation::LlmChat(request) => (false, request),
                ExtensionInvocation::LlmChatStream(request) => (true, request),
                _ => return Err(RpcError::internal_error().data("wrong real LlmClient invocation")),
            };
            // 消费者反向恢复沿Core唯一Deserialize，不能另造Function/name解析语义。
            let observed: Option<ToolChoice> = serde_json::from_value(
                serde_json::to_value(request.tool_choice).map_err(|error| RpcError::internal_error().data(error.to_string()))?
            ).map_err(|error| RpcError::internal_error().data(error.to_string()))?;
            sdk_captures.lock().map_err(|error| RpcError::internal_error().data(error.to_string()))?
                .push((streaming, observed));
            if streaming {
                let stream = call.stream.ok_or_else(|| RpcError::internal_error().data("original stream missing"))?;
                responder.respond(ExtensionInvokeOutcome::Stream { stream: stream.clone() })?;
                connection.send_notification(ExtensionStreamEvent::Complete {
                    stream,
                    sequence: echo_sdk_protocol::scalar::WireNonZeroU64::try_from("1".to_string())
                        .map_err(|error| RpcError::internal_error().data(error.to_string()))?,
                    value: ExtensionStreamCompleteValue::Llm(serde_json::from_value(json!({
                        "role":"assistant","content":"choice delivered","finish_reason":"stop"
                    })).map_err(|error| RpcError::internal_error().data(error.to_string()))?),
                })
            } else {
                responder.respond(ExtensionInvokeOutcome::Result {
                    result: ExtensionResult::LlmChat(serde_json::from_value(json!({
                        "message":{"role":"assistant","content":{"kind":"string","value":"choice delivered"}},
                        "finish_reason":"stop","raw":{"kind":"map","value":[]}
                    })).map_err(|error| RpcError::internal_error().data(error.to_string()))?),
                })
            }
        }, agent_client_protocol::on_receive_request!(),
    ).connect_with(ByteStreams::new(sdk_write.compat_write(), sdk_read.compat()), async move |_| {
        sdk_finish.cancelled().await;
        Ok(())
    });
    let notification_shared = state.extension_shared.clone();
    let bridge = state.extension_bridge.clone();
    let host = AcpAgent.builder().on_receive_notification(
        async move |event: ExtensionStreamEvent, _connection: ConnectionTo<Client>| {
            notification_shared.deliver_stream_event(event).map_err(|error| RpcError::internal_error().data(error))
        }, agent_client_protocol::on_receive_notification!(),
    ).connect_with(ByteStreams::new(host_write.compat_write(), host_read.compat()), async move |connection| {
        let _finish_on_exit = finish.drop_guard();
        bridge.shared.bind_connection(connection);
        let descriptor = serde_json::from_value(json!({
            "kind":"llm_client","descriptor_version":1,"model_name":"choice-fixture","supports_streaming":true
        })).map_err(|error| RpcError::internal_error().data(error.to_string()))?;
        let (extension, _) = state.handles.register_extension(
            super::super::handles::ExtensionRegistrationLimits { max_extensions:4, max_descriptor_bytes:65_536 },
            ExtensionKind::LlmClient, "choice-fixture", descriptor, None, false,
        ).map_err(|error| RpcError::internal_error().data(error.message))?;
        let proxy = ExtensionLlmClientProxy::new(bridge, extension, "choice-session".into())
            .ok_or_else(|| RpcError::internal_error().data("original LlmClient proxy unavailable"))?;
        let choices = [None, Some(ToolChoice::Auto), Some(ToolChoice::None), Some(ToolChoice::Required),
            Some(ToolChoice::function(" 原始工具🧪 ")), Some(ToolChoice::function("\u{feff}"))];
        for choice in choices {
            for streaming in [false, true] {
                let request = ChatRequest { tool_choice: choice.clone(), cancel_token: Some(CancellationToken::new()), ..Default::default() };
                if streaming {
                    let chunks = proxy.chat_stream(request).await.map_err(|error| RpcError::internal_error().data(error.to_string()))?
                        .collect::<Vec<_>>().await;
                    assert_eq!(chunks.len(), 1);
                    let chunk = chunks.first().ok_or_else(|| RpcError::internal_error().data("actual stream complete missing"))?
                        .as_ref().map_err(|error| RpcError::internal_error().data(error.to_string()))?;
                    assert_eq!(chunk.finish_reason.as_deref(), Some("stop"));
                } else {
                    assert_eq!(proxy.chat(request).await.map_err(|error| RpcError::internal_error().data(error.to_string()))?.finish_reason.as_deref(), Some("stop"));
                }
                let actual = captures.lock().map_err(|error| RpcError::internal_error().data(error.to_string()))?
                    .last().cloned().ok_or_else(|| RpcError::internal_error().data("actual callback bypassed"))?;
                assert_eq!(actual, (streaming, choice.clone()));
            }
        }
        assert_eq!(captures.lock().map_err(|error| RpcError::internal_error().data(error.to_string()))?.len(), 12);
        // 公共构造器可表达空名；bridge必须拒绝，且不能发送另一次callback或默认为auto。
        for streaming in [false, true] {
            let request = ChatRequest { tool_choice: Some(ToolChoice::function("  ")), cancel_token: Some(CancellationToken::new()), ..Default::default() };
            let rejected = if streaming { proxy.chat_stream(request).await.is_err() } else { proxy.chat(request).await.is_err() };
            assert!(rejected);
        }
        assert_eq!(captures.lock().map_err(|error| RpcError::internal_error().data(error.to_string()))?.len(), 12);
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

#[test]
fn consumer_compatibility_request_preserves_choice_function_strict_and_final_schema()
-> Result<(), String> {
    for choice in [
        json!("auto"),
        json!("none"),
        json!("required"),
        json!({"type":"function","function":{"name":"selected_function"}}),
    ] {
        for strict in [None, Some(false), Some(true)] {
            let mut function = json!({"name":"selected_function", "description":"fixture",
                "parameters":{"type":"object","properties":{},"additionalProperties":false}});
            if let Some(strict) = strict {
                function
                    .as_object_mut()
                    .ok_or("function must be an object")?
                    .insert("strict".into(), json!(strict));
            }
            let request = echo_agent::llm::ChatRequest {
                messages: vec![echo_agent::llm::types::Message::user("request-body".into())],
                tools: Some(vec![
                    serde_json::from_value(json!({"type":"function","function":function}))
                        .map_err(|error| error.to_string())?,
                ]),
                tool_choice: Some(
                    serde_json::from_value(choice.clone()).map_err(|error| error.to_string())?,
                ),
                response_format: Some(echo_agent::llm::ResponseFormat::json_schema(
                    "final_schema",
                    json!({"type":"object","properties":{}}),
                )),
                ..Default::default()
            };
            let wire = ExtensionLlmClientProxy::request_wire(&request)?;
            let value = serde_json::to_value(&wire).map_err(|error| error.to_string())?;
            let encoded: echo_agent::llm::ToolChoice =
                serde_json::from_value(value.get("tool_choice").cloned().ok_or("missing choice")?)
                    .map_err(|error| error.to_string())?;
            assert_eq!(Some(&encoded), request.tool_choice.as_ref());
            assert_eq!(
                serde_json::to_value(encoded).map_err(|error| error.to_string())?,
                choice
            );
            let tool = value.pointer("/tools/0").ok_or("missing original tool")?;
            assert_eq!(
                tool.get("strict"),
                strict.map(|value| json!(value)).as_ref()
            );
            let definition = wire
                .tools
                .as_ref()
                .and_then(|tools| tools.first())
                .ok_or("missing wire definition")?;
            let parameters = definition
                .parameters
                .clone()
                .into_json()
                .map_err(|error| error.to_string())?;
            assert!(
                parameters.get("strict").is_none(),
                "strict must remain a sibling"
            );
            let final_schema = wire
                .response_format
                .ok_or("missing final schema")?
                .into_json()
                .map_err(|error| error.to_string())?;
            assert_eq!(
                final_schema.pointer("/json_schema/strict"),
                Some(&json!(true))
            );
        }
    }
    Ok(())
}

#[test]
fn consumer_compatibility_host_parts_and_original_tool_error_roundtrip() -> Result<(), String> {
    let parts = json!([
        {"type":"text","text":"before"},
        {"type":"image_url","image_url":{"url":"data:image/png;base64,AQID"}},
        {"type":"text","text":"after"}
    ]);
    for error in [None, Some(false), Some(true)] {
        let mut original = json!({"role":"tool","content":parts,"name":"selected_function",
            "tool_call_id":"original-call"});
        if let Some(error) = error {
            original
                .as_object_mut()
                .ok_or("message must be object")?
                .insert("tool_result_is_error".into(), json!(error));
        }
        let native: echo_agent::llm::types::Message =
            serde_json::from_value(original.clone()).map_err(|error| error.to_string())?;
        let wire = message_wire(&native)?;
        let wire_value = serde_json::to_value(&wire).map_err(|error| error.to_string())?;
        assert_eq!(
            wire_value.get("tool_result_is_error"),
            original.get("tool_result_is_error")
        );
        assert_eq!(
            wire.content
                .clone()
                .into_json()
                .map_err(|error| error.to_string())?,
            parts
        );
        let restored = message_from_wire(wire)?;
        assert_eq!(
            serde_json::to_value(restored).map_err(|error| error.to_string())?,
            original
        );
    }
    Ok(())
}

#[test]
fn consumer_compatibility_nonstream_response_to_stream_keeps_native_fields() -> Result<(), String> {
    for reasoning in [
        json!({"kind":"signed","thinking":"original-thought","signature":"exact-signature=="}),
        json!({"kind":"redacted","data":"exact-encrypted=="}),
        json!({"kind":"opaque","provider":"responses","id":"reasoning-1",
            "data":"exact-opaque==","summary":["public-summary"]}),
    ] {
        let block = json!({"block_id":"original-block", "request_id":"original-request",
            "sequence":"18446744073709551615", "phase":"reasoning","delta":"",
            "complete":true,"provider":"responses","reasoning":reasoning,
            "provider_source":{"protocol":"responses","request_id":"original-request",
                "item_id":"reasoning-1","output_index":0,"content_index":0,
                "content_kind":"reasoning","phase":"opaque-original"}});
        let response: LlmChatResponseWire = serde_json::from_value(json!({
            "message":{"role":"assistant", "content":to_wire("body")?,
                "assistant_blocks":[block.clone()]},
            "finish_reason":"stop", "raw":to_wire(json!({}))?
        }))
        .map_err(|error| error.to_string())?;
        let native = chat_response_from_wire(response)?;
        let chunk = chat_response_into_chunk(native)?;
        let observed = serde_json::to_value(
            chunk
                .delta
                .assistant_blocks
                .as_ref()
                .and_then(|blocks| blocks.first())
                .ok_or("nonstream adapter lost blocks")?,
        )
        .map_err(|error| error.to_string())?;
        assert_eq!(observed.get("request_id"), block.get("request_id"));
        assert_eq!(
            observed.get("provider_source"),
            block.get("provider_source")
        );
        assert_eq!(
            observed.pointer("/reasoning/signature"),
            block.pointer("/reasoning/signature")
        );
        assert_eq!(
            observed.pointer("/reasoning/data"),
            block.pointer("/reasoning/data")
        );
        assert_eq!(chunk.finish_reason.as_deref(), Some("stop"));
    }
    Ok(())
}

#[test]
fn consumer_compatibility_usage_keeps_reasoning_detail_through_host_and_nonstream_fallback()
-> Result<(), String> {
    let response: LlmChatResponseWire = serde_json::from_value(json!({
        "message":{"role":"assistant","content":to_wire("body")?},"finish_reason":"stop",
        "raw":to_wire(json!({}))?,"usage":{"value":to_wire(json!({
            "prompt_tokens":11,"completion_tokens":7,"total_tokens":18,
            "output_tokens_details":{"reasoning_tokens":5}}))?}
    }))
    .map_err(|error| error.to_string())?;
    let native = chat_response_from_wire(response)?;
    let chunk = chat_response_into_chunk(native)?;
    let usage = serde_json::to_value(chunk.usage.ok_or("fallback lost usage")?)
        .map_err(|error| error.to_string())?;
    assert_eq!(
        usage.pointer("/output_tokens_details/reasoning_tokens"),
        Some(&json!(5))
    );
    assert_eq!(chunk.finish_reason.as_deref(), Some("stop"));
    Ok(())
}

/// reverse LlmClient回调不能冒充runtime确认；外部HTTP原协议防伪是另一条边界。
#[test]
fn consumer_compatibility_reverse_llm_fallback_rejects_runtime_authority() -> Result<(), String> {
    for (field, forged) in [
        (
            "execution_status",
            json!({"execution_id":"forged-provider-owner","request_id":"forged-request", "sequence":"0","phase":"Terminal","status":"settled"}),
        ),
        ("approved_assistant_blocks", json!([])),
    ] {
        let mut message =
            json!({"role":"assistant","content":to_wire("unapproved reverse answer")?});
        message
            .as_object_mut()
            .ok_or("fixture message must be object")?
            .insert(field.into(), forged);
        let response: LlmChatResponseWire = serde_json::from_value(json!({
            "message":message,"finish_reason":"stop","raw":to_wire(json!({}))?
        }))
        .map_err(|error| error.to_string())?;
        let native = chat_response_from_wire(response)?;
        let error = chat_response_into_chunk(native).err().ok_or(
            "reverse callback cannot publish a successful chunk with runtime confirmation",
        )?;
        assert!(
            error.contains("runtime/approved confirmation"),
            "wrong rejection cause: {error}"
        );
    }
    Ok(())
}
