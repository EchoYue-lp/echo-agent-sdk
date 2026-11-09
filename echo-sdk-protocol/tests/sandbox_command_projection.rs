//! Before控制只使用现manifest/route与reverse-call验证入口，不依赖后继投影helper。
use echo_sdk_protocol::handle::{HandleKind, WireHandle};
use echo_sdk_protocol::inventory::{
    InventoryEntry, InventorySignature, ItemKind, SemanticClass, manifest_entries,
};
use echo_sdk_protocol::methods::{
    AgentComponentCallInputWire, AgentComponentCallWire, AgentComponentKindWire,
    ExtensionInvocation, ExtensionInvokeCall,
};
use echo_sdk_protocol::scalar::{WireDuration, WireU64, WireValue};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn actual_command_inventory() -> Result<Vec<InventoryEntry>, Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let snapshot = std::fs::read_to_string(root.join("contracts/sdk/public-api.txt"))?;
    let mut items = Vec::new();
    for line in snapshot.lines() {
        let mut words = line.split_whitespace();
        let Some(kind) = words.next() else {
            continue;
        };
        let Some(path) = words.next() else {
            continue;
        };
        if path != "echo_agent::sandbox::SandboxCommand"
            && !path.starts_with("echo_agent::sandbox::SandboxCommand::")
        {
            continue;
        }
        let digest = words.next().ok_or("actual digest missing")?;
        let start = line.find('{').ok_or("actual shape missing")?;
        let shape = line
            .get(start..)
            .ok_or("actual shape boundary missing")?
            .to_string();
        let source = path.replacen("echo_agent::sandbox::", "echo_core::sandbox::", 1);
        items.push(InventoryEntry {
            path: path.to_string(),
            kind: serde_json::from_value(json!(kind))?,
            source_paths: BTreeSet::from([source]),
            signatures: BTreeMap::from([(
                digest.to_string(),
                InventorySignature {
                    digest: digest.to_string(),
                    shape,
                    profiles: BTreeSet::from(["default".into()]),
                },
            )]),
            profiles: BTreeSet::from(["default".into()]),
            declared_feature_requirements: BTreeSet::new(),
            automatically_derived: false,
        });
    }
    if items.is_empty() {
        return Err("real command inventory absent".into());
    }
    Ok(items)
}

#[test]
fn sandbox_projection_keeps_raw_local_type_but_value_route_and_language_target() -> TestResult {
    let entries = manifest_entries(&actual_command_inventory()?);
    let command = entries
        .iter()
        .find(|e| e.path == "echo_agent::sandbox::SandboxCommand")
        .ok_or("command row absent")?;
    assert_eq!(
        command.classification,
        SemanticClass::Handle,
        "raw Rust local ownership must remain visible"
    );
    assert_eq!(command.semantic_rule, "contains-process-local-state");
    assert_eq!(
        command.route.surface, "value",
        "serialized command value must not advertise an unimplemented opaque authority"
    );
    assert!(command.route.operation.is_none());
    for language in command.languages.values() {
        assert_eq!(language.target, "generated_or_lossless_value");
        assert!(!language.contract_test.ends_with("/facade_handle"));
    }
    for field in [
        "kind",
        "minimum_isolation",
        "working_dir",
        "env",
        "timeout",
        "stdin",
    ] {
        let field_path = format!("echo_agent::sandbox::SandboxCommand::{field}");
        assert!(
            entries
                .iter()
                .any(|e| e.path == field_path && e.route.surface == "value")
        );
    }
    Ok(())
}

#[test]
fn sandbox_projection_does_not_expand_to_other_sources_or_kinds() -> TestResult {
    let source = actual_command_inventory()?;
    let unrelated: Vec<_> = source
        .iter()
        .map(|entry| InventoryEntry {
            path: entry.path.replace("SandboxCommand", "OtherCommand"),
            source_paths: entry
                .source_paths
                .iter()
                .map(|p| p.replace("SandboxCommand", "OtherCommand"))
                .collect(),
            ..entry.clone()
        })
        .collect();
    let other = manifest_entries(&unrelated)
        .into_iter()
        .find(|e| e.path == "echo_agent::sandbox::OtherCommand")
        .ok_or("negative type absent")?;
    assert_eq!(other.classification, SemanticClass::Handle);
    assert_ne!(
        other.route.surface, "value",
        "another skip/Arc owner cannot inherit this projection"
    );
    let mut function = source
        .iter()
        .find(|e| e.path == "echo_agent::sandbox::SandboxCommand")
        .ok_or("source absent")?
        .clone();
    function.kind = ItemKind::Function;
    let entry = manifest_entries(&[function])
        .into_iter()
        .next()
        .ok_or("negative kind absent")?;
    assert_ne!(entry.route.surface, "value");
    Ok(())
}

fn command_call(
    command: serde_json::Value,
) -> Result<ExtensionInvokeCall, Box<dyn std::error::Error>> {
    Ok(ExtensionInvokeCall {
        factory_origin: None,
        extension: WireHandle {
            id: "sandbox-projection-before".into(),
            generation: WireU64::from_u64(1),
            kind: HandleKind::Extension,
        },
        invocation_id: "sandbox-command-original-call".into(),
        context: None,
        invocation: ExtensionInvocation::AgentComponentCall(AgentComponentCallWire {
            component: AgentComponentKindWire::SandboxExecutor,
            call: AgentComponentCallInputWire::SandboxExecute {
                command: WireValue::from_json(command)?,
            },
        }),
        deadline: WireDuration {
            seconds: WireU64::from_u64(1),
            nanos: 0,
        },
        stream: None,
    })
}

#[test]
fn sandbox_projection_preserves_three_command_kinds_and_optional_serde_values() -> TestResult {
    for kind in [
        json!({"Shell":"printf 中文🙂"}),
        json!({"Program":{"program":"printf","args":["中文🙂"]}}),
        json!({"Code":{"language":"python","code":"print('中文🙂')"}}),
    ] {
        let mut value = json!({"kind":kind,"minimum_isolation":null,"working_dir":null,"env":{"中文":"🙂"},"timeout":{"secs":1,"nanos":0},"stdin":null});
        command_call(value.clone())?.validate()?;
        let object = value.as_object_mut().ok_or("command must be object")?;
        for name in ["minimum_isolation", "working_dir", "stdin"] {
            object.remove(name);
        }
        command_call(value)?.validate()?;
    }
    Ok(())
}

#[test]
fn sandbox_projection_rejects_runtime_only_fields_and_bad_value_shapes() -> TestResult {
    let base = json!({"kind":{"Shell":"fixture"},"env":{},"timeout":{"secs":1,"nanos":0}});
    for (name, value) in [
        ("resource_guards", json!([])),
        ("cancel", json!(null)),
        ("unknown_command_field", json!(true)),
    ] {
        let mut command = base.clone();
        command
            .as_object_mut()
            .ok_or("command must be object")?
            .insert(name.into(), value);
        assert!(
            command_call(command)?.validate().is_err(),
            "runtime/unknown field {name} cannot become a constructible wire field"
        );
    }
    for command in [
        json!([]),
        json!("shell string"),
        json!({"kind":{"Shell":1},"env":{},"timeout":{"secs":1,"nanos":0}}),
    ] {
        assert!(
            command_call(command)?.validate().is_err(),
            "bad command cannot be admitted by generic WireValue validation"
        );
    }
    Ok(())
}
