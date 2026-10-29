use echo_sdk_protocol::methods::{LlmMessageWire, ProviderResponseMetadataWire};
use echo_sdk_protocol::scalar::WireValue;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn accepted_request_field_keeps_canonical_named_tool_choice() -> TestResult {
    let input = serde_json::json!({"messages":[],"tool_choice":{
        "type":"function","function":{"name":" 原始工具🧪 "}
    }});
    let request: echo_sdk_protocol::methods::LlmChatRequestWire =
        serde_json::from_value(input.clone())?;
    assert_eq!(serde_json::to_value(request)?, input);
    Ok(())
}

#[test]
fn tool_choice_field_preserves_modes_none_and_rejects_noncanonical_shapes() -> TestResult {
    use echo_sdk_protocol::methods::LlmChatRequestWire;
    for choice in [
        serde_json::Value::Null,
        serde_json::json!("auto"),
        serde_json::json!("none"),
        serde_json::json!("required"),
        serde_json::json!({"type":"function","function":{"name":"\u{feff}"}}),
    ] {
        let request: LlmChatRequestWire =
            serde_json::from_value(serde_json::json!({"messages":[],"tool_choice":choice}))?;
        let actual = serde_json::to_value(request)?;
        assert_eq!(
            actual
                .get("tool_choice")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            choice
        );
    }
    let absent: LlmChatRequestWire = serde_json::from_value(serde_json::json!({"messages":[]}))?;
    assert!(absent.tool_choice.is_none());
    for choice in [
        serde_json::json!("future"),
        serde_json::json!(true),
        serde_json::json!([]),
        serde_json::json!({"type":"tool","function":{"name":"x"}}),
        serde_json::json!({"type":"function","function":{"name":""}}),
        serde_json::json!({"type":"function","function":{"name":"\u{00a0}"}}),
        serde_json::json!({"type":"function","function":{"name":2}}),
        serde_json::json!({"type":"function","function":{"name":"x","extra":true}}),
        serde_json::json!({"type":"function","function":{"name":"x"},"extra":true}),
    ] {
        assert!(
            serde_json::from_value::<LlmChatRequestWire>(
                serde_json::json!({"messages":[],"tool_choice":choice})
            )
            .is_err()
        );
    }
    Ok(())
}

#[test]
fn provider_response_wire_keeps_missing_null_unknown_and_nested_usage() -> TestResult {
    for (field, value) in [
        ("model", serde_json::json!({"kind":"null"})),
        (
            "stop_reason",
            serde_json::json!({"kind":"string","value":"provider-future-stop"}),
        ),
        ("stop_sequence", serde_json::json!({"kind":"null"})),
        (
            "status",
            serde_json::json!({"kind":"string","value":"provider-future-status"}),
        ),
        ("incomplete_details", serde_json::json!({"kind":"null"})),
        (
            "usage",
            serde_json::json!({"kind":"map","value":[{
                "key":{"kind":"string","value":"nested"},
                "value":{"kind":"map","value":[
                    {"key":{"kind":"string","value":"tokens"},"value":{"kind":"u64","value":"18446744073709551615"}},
                    {"key":{"kind":"string","value":"text"},"value":{"kind":"string","value":"00123"}}
                ]}
            }]}),
        ),
    ] {
        let mut metadata =
            serde_json::json!({"protocol":"responses","provider":"fixture-provider"});
        metadata
            .as_object_mut()
            .ok_or("metadata must be object")?
            .insert(field.into(), value);
        let input = serde_json::json!({"role":"provider-specific-角色🧪",
            "content":{"kind":"string","value":"完整消息"},
            "provider_response":metadata});
        let decoded: LlmMessageWire = serde_json::from_value(input.clone())?;
        let observed = serde_json::to_value(decoded)?;
        assert_eq!(observed, input);
    }
    let no_observation =
        serde_json::json!({"role":"assistant","content":{"kind":"string","value":"body"}});
    let value: LlmMessageWire = serde_json::from_value(no_observation.clone())?;
    assert_eq!(serde_json::to_value(value)?, no_observation);
    Ok(())
}

#[test]
fn current_tool_error_observation_keeps_absent_false_and_true() -> TestResult {
    for flag in [false, true] {
        let input = serde_json::json!({"role":"tool","content":{"kind":"string","value":"原工具输出"},
            "tool_result_is_error":flag});
        let decoded: LlmMessageWire = serde_json::from_value(input.clone())?;
        assert_eq!(serde_json::to_value(decoded)?, input);
    }
    Ok(())
}

#[test]
fn provider_response_read_and_write_reject_wrong_shapes_without_coercion() -> TestResult {
    for (field, bad) in [
        ("model", serde_json::json!({"kind":"bool","value":true})),
        ("status", serde_json::json!({"kind":"i64","value":"1"})),
        ("stop_reason", serde_json::Value::Null),
        ("usage", serde_json::json!({"kind":"string","value":"{}"})),
        (
            "incomplete_details",
            serde_json::json!({"kind":"list","value":[]}),
        ),
        ("future", serde_json::json!({"kind":"null"})),
    ] {
        let mut value = serde_json::json!({"protocol":"responses","provider":"fixture"});
        value
            .as_object_mut()
            .ok_or("object required")?
            .insert(field.into(), bad);
        assert!(serde_json::from_value::<ProviderResponseMetadataWire>(value).is_err());
    }
    let mut value: ProviderResponseMetadataWire =
        serde_json::from_value(serde_json::json!({"protocol":"responses","provider":"fixture"}))?;
    value.status = Some(WireValue::Bool(false));
    assert!(serde_json::to_value(&value).is_err());
    value.status = None;
    value.usage = Some(WireValue::String("not an object".into()));
    assert!(serde_json::to_value(&value).is_err());
    Ok(())
}
