//! Product withdrawal must not be undone by inventory or family aggregation.
use std::error::Error;
use std::path::PathBuf;

type TestResult = Result<(), Box<dyn Error>>;

fn artifact(name: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    Ok(serde_json::from_slice(&std::fs::read(root.join(name))?)?)
}

#[test]
fn retired_source_scope_and_routes_are_absent() -> TestResult {
    let accepted = artifact("contracts/sdk/accepted-external-contract.json")?;
    let entries = accepted
        .get("entries")
        .and_then(serde_json::Value::as_array)
        .ok_or("entries absent")?;
    // 原 Plan04 之上，具名 11 退役 + 3 with_id 接入；依据 P3 严格 caller 与生成原件。
    assert_eq!(entries.len(), 5344);
    for entry in entries {
        let text = entry
            .get("source_paths")
            .ok_or("source identity absent")?
            .to_string();
        assert!(!text.contains("::a2a::") && !text.contains("::lsp::"));
        assert!(!text.contains("PluginCapability::LspServer"));
        assert!(!text.contains("PluginWiringResult::lsp_discovered"));
        assert!(!text.contains("ResolvedComponents::lsp_config_file"));
    }
    let catalog = artifact("contracts/sdk/accepted-facade-operation-catalog.json")?;
    let routes = catalog
        .get("routes")
        .and_then(serde_json::Value::as_array)
        .ok_or("routes absent")?;
    // 依据 P3 严格 caller 与生成原件，精确退出一条 source route，原 431 条保留为 430。
    assert_eq!(routes.len(), 430);
    for route in routes {
        assert!(!matches!(
            route.get("family").and_then(serde_json::Value::as_str),
            Some("a2a" | "lsp")
        ));
    }
    Ok(())
}

#[test]
fn retired_family_methods_are_not_protocol_methods() {
    for method in echo_sdk_protocol::catalog::METHOD_CATALOG {
        assert!(!matches!(
            method.name,
            "_echo_agent/a2a/op" | "_echo_agent/lsp/op"
        ));
    }
}

#[test]
fn obsolete_plugin_fields_and_variant_fail_without_poisoning_shared_values() {
    use echo_sdk_protocol::scalar::{WireField, WireValue};
    for (type_id, field) in [
        (
            "echo_core::plugin::registry::ResolvedComponents",
            "lsp_config_file",
        ),
        (
            "echo_agent::plugin::prepared::PluginWiringResult",
            "lsp_discovered",
        ),
    ] {
        let value = WireValue::Record {
            type_id: type_id.to_owned(),
            fields: vec![WireField {
                name: field.to_owned(),
                value: WireValue::Null,
            }],
        };
        assert!(value.validate().is_err());
        let retained = WireValue::Record {
            type_id: type_id.to_owned(),
            fields: vec![WireField {
                name: "diagnostics".to_owned(),
                value: WireValue::List(vec![]),
            }],
        };
        assert!(retained.validate().is_ok());
    }
    for variant in ["LspServer", "lsp_server"] {
        let value = WireValue::Variant {
            type_id: "echo_core::plugin::capability::PluginCapability".to_owned(),
            variant: variant.to_owned(),
            fields: vec![],
        };
        assert!(value.validate().is_err());
    }
    let retained = WireValue::Variant {
        type_id: "echo_core::plugin::capability::PluginCapability".to_owned(),
        variant: "McpServer".to_owned(),
        fields: vec![],
    };
    assert!(retained.validate().is_ok());
}
