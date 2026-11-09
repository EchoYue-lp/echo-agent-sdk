//! 无Framework依赖的消费者合同回归；既有accepted字段采用正式owner的固定值投影。
use echo_sdk_protocol::methods::{LlmChatRequestWire, LlmMessageWire, LlmToolDefinitionWire};
use echo_sdk_protocol::scalar::WireValue;
use serde_json::{Value, json};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn consumer_compatibility_typed_choice_and_function_strict_roundtrip() -> TestResult {
    for choice in [
        json!("auto"),
        json!("none"),
        json!("required"),
        json!({"type":"function","function":{"name":"selected_function"}}),
    ] {
        for strict in [None, Some(false), Some(true)] {
            let mut tool = json!({"tool_type":"function","name":"selected_function",
                "description":"fixture", "parameters":WireValue::from_json(json!({"type":"object"}))?});
            if let Some(strict) = strict {
                tool.as_object_mut()
                    .ok_or("tool must be an object")?
                    .insert("strict".into(), json!(strict));
            }
            let original = json!({"messages":[], "tools":[tool], "tool_choice":choice.clone(),
                "response_format":WireValue::from_json(json!({"type":"json_schema","json_schema":{
                    "name":"final_schema","schema":{"type":"object"},"strict":true}}))?});
            let decoded: LlmChatRequestWire = serde_json::from_value(original.clone())?;
            assert_eq!(serde_json::to_value(decoded)?, original);
        }
    }
    Ok(())
}

#[test]
fn consumer_compatibility_choice_rejects_unknown_or_malformed_shapes() -> TestResult {
    for choice in [
        json!("selected_function"),
        json!("{\"type\":\"function\",\"function\":{\"name\":\"selected_function\"}}"),
        json!({"type":"function","name":"selected_function"}),
        json!({"type":"tool","name":"selected_function"}),
        json!({"type":"function","function":{"name":"selected_function","extra":true}}),
        json!({"type":"function","function":{"name":"selected_function"},"settled":true}),
    ] {
        assert!(
            serde_json::from_value::<LlmChatRequestWire>(
                json!({"messages":[],"tool_choice":choice})
            )
            .is_err()
        );
    }
    Ok(())
}

#[test]
fn consumer_compatibility_parts_and_original_error_presence_survive() -> TestResult {
    let parts = json!([
        {"type":"text","text":"before"},
        {"type":"image_url","image_url":{"url":"data:image/png;base64,AQID"}},
        {"type":"text","text":"after"}
    ]);
    for error in [None, Some(false), Some(true)] {
        let mut original = json!({"role":"tool","content":WireValue::from_json(parts.clone())?,
            "tool_call_id":"original-call","name":"selected_function"});
        if let Some(error) = error {
            original
                .as_object_mut()
                .ok_or("message must be an object")?
                .insert("tool_result_is_error".into(), json!(error));
        }
        let decoded: LlmMessageWire = serde_json::from_value(original.clone())?;
        assert_eq!(decoded.content.clone().into_json()?, parts);
        assert_eq!(serde_json::to_value(decoded)?, original);
    }
    Ok(())
}

#[test]
fn consumer_compatibility_schema_exposes_typed_choice_and_sibling_fields() -> TestResult {
    let tool = serde_json::to_value(schemars::schema_for!(LlmToolDefinitionWire))?;
    assert!(tool.pointer("/properties/strict").is_some());
    let message = serde_json::to_value(schemars::schema_for!(LlmMessageWire))?;
    assert!(
        message
            .pointer("/properties/tool_result_is_error")
            .is_some()
    );
    let request = serde_json::to_value(schemars::schema_for!(LlmChatRequestWire))?;
    let choice = request
        .pointer("/properties/tool_choice")
        .ok_or("missing tool_choice")?;
    assert_ne!(choice.get("type"), Some(&Value::String("string".into())));
    assert!(choice.to_string().contains("LlmToolChoiceWire"));
    Ok(())
}

#[test]
fn consumer_compatibility_choice_none_null_and_original_name_are_not_rewritten() -> TestResult {
    for input in [
        json!({"messages":[]}),
        json!({"messages":[],"tool_choice":null}),
    ] {
        let request: LlmChatRequestWire = serde_json::from_value(input)?;
        assert!(request.tool_choice.is_none());
    }
    let value =
        json!({"messages":[],"tool_choice":{"type":"function","function":{"name":" 原始工具🧪 "}}});
    let request: LlmChatRequestWire = serde_json::from_value(value.clone())?;
    assert_eq!(serde_json::to_value(request)?, value);
    for invalid in [
        json!({}),
        json!([]),
        json!(true),
        json!(17),
        json!({"type":"function","function":{"name":""}}),
        json!({"type":"function","function":{"name":"  "}}),
        json!({"type":"function","function":{"name":42}}),
    ] {
        assert!(
            serde_json::from_value::<LlmChatRequestWire>(
                json!({"messages":[],"tool_choice":invalid})
            )
            .is_err()
        );
    }
    Ok(())
}
