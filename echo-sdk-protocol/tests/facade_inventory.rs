//! Artifact-level facade inventory and parity-manifest checks.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use echo_sdk_protocol::facade::{FACADE_FAMILIES, validate_facade_route_table};
use echo_sdk_protocol::inventory::{
    AcceptedExternalContract, AcpRelationship, FeatureSemantics, ParityManifest, SdkScope,
    SemanticClass,
};
use sha2::{Digest, Sha256};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read(path: &str) -> TestResult<String> {
    Ok(std::fs::read_to_string(repo_root().join(path))?)
}

fn parse_snapshot() -> TestResult<Vec<(String, String, String)>> {
    Ok(read("contracts/sdk/public-api.txt")?
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some((
                parts.next()?.to_string(),
                parts.next()?.to_string(),
                parts.next()?.to_string(),
            ))
        })
        .collect())
}

fn manifest() -> TestResult<ParityManifest> {
    Ok(serde_json::from_str(&read(
        "contracts/sdk/parity-manifest.json",
    )?)?)
}

#[test]
fn manifest_entries_match_every_inventory_signature() -> TestResult {
    let snapshot: BTreeSet<(String, String)> = parse_snapshot()?
        .into_iter()
        .map(|(_, path, digest)| (path, digest))
        .collect();
    assert!(!snapshot.is_empty(), "snapshot must not be empty");
    let manifest: BTreeSet<(String, String)> = manifest()?
        .entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .signatures
                .into_iter()
                .map(move |signature| (entry.path.clone(), signature.digest))
        })
        .collect();
    assert_eq!(
        snapshot, manifest,
        "manifest and inventory signatures drifted"
    );
    Ok(())
}

#[test]
fn manifest_schema_compiles_and_validates_document() -> TestResult {
    let schema: serde_json::Value =
        serde_json::from_str(&read("contracts/sdk/parity-manifest.schema.json")?)?;
    let document: serde_json::Value =
        serde_json::from_str(&read("contracts/sdk/parity-manifest.json")?)?;
    let validator = jsonschema::validator_for(&schema)?;
    assert!(
        validator.validate(&document).is_ok(),
        "manifest does not satisfy its schema"
    );
    Ok(())
}

#[test]
fn sdk_scope_is_complete_deterministic_and_alias_safe() -> TestResult {
    let manifest = manifest()?;
    assert_eq!(manifest.schema_version, 2);

    let mut counts = BTreeMap::new();
    let scope_by_path: BTreeMap<&str, SdkScope> = manifest
        .entries
        .iter()
        .map(|entry| (entry.path.as_str(), entry.sdk_scope))
        .collect();
    for entry in &manifest.entries {
        if entry.canonical {
            let count = counts.entry(entry.sdk_scope).or_insert(0usize);
            *count = count.saturating_add(1);
        }
        if let Some(alias_of) = &entry.alias_of {
            let canonical_scope = scope_by_path
                .get(alias_of.as_str())
                .ok_or_else(|| format!("missing canonical scope for alias {}", entry.path))?;
            assert_eq!(
                entry.sdk_scope, *canonical_scope,
                "alias {} and canonical {alias_of} must share one SDK scope",
                entry.path
            );
        }
        if entry.sdk_scope == SdkScope::ExternalContract {
            assert!(
                entry.languages.values().all(|mapping| {
                    mapping.status
                        == echo_sdk_protocol::inventory::LanguageImplementationStatus::Done
                        && !mapping.contract_test.is_empty()
                }),
                "external SDK contract is incomplete: {}",
                entry.path
            );
        }
    }

    assert_eq!(
        counts.values().sum::<usize>(),
        manifest
            .entries
            .iter()
            .filter(|entry| entry.canonical)
            .count(),
        "every canonical inventory identity must have one scope"
    );
    eprintln!("SDK scope telemetry: {counts:?}");

    // These examples exercise classification when present. Their membership
    // is inventory telemetry, not an additional compatibility promise.
    for (path, scope) in [
        (
            "echo_agent::llm::types::ResponseFormat",
            SdkScope::ExternalContract,
        ),
        ("echo_agent::testing::MockAgent", SdkScope::InternalHelper),
        ("echo_agent::Tool", SdkScope::LanguageIntrinsic),
        ("echo_agent::a2a::A2AServer", SdkScope::HostOrRustOnly),
        ("echo_agent::a2a::A2AClient::new", SdkScope::Deferred),
    ] {
        if let Some(entry) = manifest.entries.iter().find(|entry| entry.path == path) {
            assert_eq!(entry.sdk_scope, scope, "scope classification: {path}");
        }
    }
    Ok(())
}

#[test]
fn accepted_external_contract_is_separate_from_inventory_telemetry() -> TestResult {
    let full = manifest()?;
    let accepted: AcceptedExternalContract =
        serde_json::from_str(&read("contracts/sdk/accepted-external-contract.json")?)?;
    assert_eq!(accepted.schema_version, 1);
    assert_eq!(
        accepted.extension_protocol_version,
        full.extension_protocol_version
    );
    let expected = full
        .entries
        .iter()
        .filter(|entry| entry.sdk_scope == SdkScope::ExternalContract && entry.canonical)
        .count();
    assert_eq!(accepted.entries.len(), expected);
    assert!(accepted.entries.iter().all(|entry| {
        entry.sdk_scope == SdkScope::ExternalContract
            && entry.languages.values().all(|language| {
                language.status == echo_sdk_protocol::inventory::LanguageImplementationStatus::Done
            })
    }));

    let source_contract: serde_json::Value =
        serde_json::from_str(&read("contracts/sdk/source-contract.json")?)?;
    let input_paths: Vec<&str> = source_contract
        .get("inputs")
        .and_then(serde_json::Value::as_array)
        .ok_or("source contract inputs are missing")?
        .iter()
        .filter_map(|input| input.get("path").and_then(serde_json::Value::as_str))
        .collect();
    assert_eq!(
        input_paths,
        vec![
            "contracts/sdk/accepted-external-contract.json",
            "contracts/sdk/accepted-facade-operation-catalog.json"
        ]
    );
    assert!(!input_paths.iter().any(|path| {
        path.contains("Cargo.lock")
            || path.contains("public-api")
            || path.contains("parity-manifest")
    }));
    Ok(())
}

fn compatibility_document(manifest: &ParityManifest) -> serde_json::Value {
    let accepted = echo_sdk_protocol::inventory::render_accepted_external_contract(manifest);
    let obligations = manifest
        .entries
        .iter()
        // Re-export aliases are telemetry: the persisted acceptance authority
        // contains defining identities, not a second set of alias obligations.
        .filter(|entry| entry.sdk_scope == SdkScope::ExternalContract && entry.canonical)
        .map(|entry| &entry.route)
        .collect::<Vec<_>>();
    let catalog = echo_sdk_protocol::facade::build_facade_operation_catalog(
        manifest.extension_protocol_version,
        &obligations,
    );
    let accepted_catalog = echo_sdk_protocol::schema::canonical_json(
        &echo_sdk_protocol::facade::filter_external_facade_operation_catalog(&catalog),
    );
    echo_sdk_protocol::schema::build_source_contract_doc(&[
        (
            "contracts/sdk/accepted-external-contract.json",
            accepted.as_bytes(),
        ),
        (
            "contracts/sdk/accepted-facade-operation-catalog.json",
            accepted_catalog.as_bytes(),
        ),
    ])
}

#[test]
fn compatibility_digest_ignores_non_external_identity_mutation() -> TestResult {
    let original = manifest()?;
    let expected: serde_json::Value =
        serde_json::from_str(&read("contracts/sdk/source-contract.json")?)?;
    assert_eq!(compatibility_document(&original), expected);
    let mut changed = original.clone();
    let sample = changed
        .entries
        .iter_mut()
        .find(|entry| entry.canonical && entry.sdk_scope != SdkScope::ExternalContract)
        .ok_or("non-external mutation fixture is missing")?;
    sample.path.push_str("::telemetry_probe");
    sample.semantic_rule.push_str(" telemetry mutation");
    assert_ne!(changed, original);
    assert_eq!(compatibility_document(&changed), expected);
    Ok(())
}

#[test]
fn compatibility_digest_detects_accepted_identity_mutation_deterministically() -> TestResult {
    let original = manifest()?;
    let expected = compatibility_document(&original);
    let mut changed = original;
    let sample = changed
        .entries
        .iter_mut()
        .find(|entry| entry.canonical && entry.sdk_scope == SdkScope::ExternalContract)
        .ok_or("accepted mutation fixture is missing")?;
    sample.semantic_rule.push_str(" accepted mutation");
    let actual = compatibility_document(&changed);
    assert_ne!(
        actual, expected,
        "accepted changes must fail the committed digest comparison"
    );
    assert_eq!(compatibility_document(&changed), actual);
    Ok(())
}

#[test]
fn entries_have_complete_mapping_and_language_obligations() -> TestResult {
    let manifest = manifest()?;
    let expected_languages: BTreeSet<&str> = ["typescript", "python", "java"].into_iter().collect();
    let mut classes = BTreeSet::new();
    let mut relationships = BTreeSet::new();
    let paths: BTreeSet<&str> = manifest.entries.iter().map(|e| e.path.as_str()).collect();
    let route_by_path: std::collections::BTreeMap<&str, &str> = manifest
        .entries
        .iter()
        .map(|entry| (entry.path.as_str(), entry.route.route.as_str()))
        .collect();
    for entry in &manifest.entries {
        assert!(
            !entry.path.ends_with("::*"),
            "unexpanded glob: {}",
            entry.path
        );
        assert!(
            !entry.signatures.is_empty(),
            "missing signatures: {}",
            entry.path
        );
        assert!(
            !entry.route.route.is_empty(),
            "missing canonical route: {}",
            entry.path
        );
        assert!(
            !entry.route.route.contains('*'),
            "wildcard route on {}: {}",
            entry.path,
            entry.route.route
        );
        assert!(
            !entry.semantic_rule.is_empty(),
            "missing semantic rule: {}",
            entry.path
        );
        assert!(
            !entry.route.validation.is_empty(),
            "missing validation: {}",
            entry.path
        );
        if let Some(alias_of) = &entry.alias_of {
            assert!(!entry.canonical, "alias marked canonical: {}", entry.path);
            assert!(
                paths.contains(alias_of.as_str()),
                "alias {} points at missing canonical {alias_of}",
                entry.path
            );
            let canonical_route = route_by_path
                .get(alias_of.as_str())
                .copied()
                .unwrap_or_default();
            assert_eq!(
                canonical_route,
                entry.route.route.as_str(),
                "alias {} and canonical {alias_of} must share one route",
                entry.path
            );
        }
        let languages: BTreeSet<&str> = entry.languages.keys().map(String::as_str).collect();
        assert_eq!(
            languages, expected_languages,
            "language mapping: {}",
            entry.path
        );
        for language in entry.languages.values() {
            assert!(
                !language.target.is_empty(),
                "empty language target: {}",
                entry.path
            );
            assert!(
                !language.contract_test.is_empty(),
                "missing language contract test: {}",
                entry.path
            );
        }
        if entry.features.is_empty() && !entry.full_only {
            assert_eq!(entry.feature_semantics, FeatureSemantics::Default);
        }
        match entry.feature_semantics {
            FeatureSemantics::Default => assert!(entry.features.is_empty()),
            FeatureSemantics::AnyOf => assert!(!entry.features.is_empty()),
            FeatureSemantics::AllOf => {
                assert!(entry.full_only);
                assert!(
                    entry.features.len() >= 2,
                    "full-only entry lacks an AND condition: {}",
                    entry.path
                );
            }
        }
        classes.insert(entry.classification);
        relationships.insert(entry.acp_relationship);
    }
    eprintln!("classification telemetry: {classes:?}, {relationships:?}");
    Ok(())
}

#[test]
fn language_mapping_status_matches_the_route_boundary() -> TestResult {
    let manifest = manifest()?;
    assert!(
        !manifest.entries.is_empty(),
        "facade manifest must not be empty"
    );
    for entry in manifest
        .entries
        .iter()
        .filter(|entry| entry.canonical && entry.sdk_scope == SdkScope::ExternalContract)
    {
        for (language, mapping) in &entry.languages {
            if entry.route.surface != "intrinsic" {
                assert_eq!(
                    mapping.status,
                    echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                    "{language}: {}",
                    entry.path
                );
            }
            assert!(
                mapping.contract_test.starts_with("sdk-parity/"),
                "{} has no language parity contract test",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn local_tool_value_intrinsics_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.sdk_scope == SdkScope::ExternalContract
                && entry.route.route == "intrinsic:language-local-wire-helper"
        })
        .collect();
    assert_eq!(entries.len(), 28, "local tool-value intrinsic set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
            assert!(
                mapping.contract_test.ends_with("/local_tool_values"),
                "{language}: {} has the wrong contract test",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn a2a_task_state_intrinsics_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/a2a_task_state"))
        })
        .collect();
    assert_eq!(entries.len(), 10, "A2A TaskState intrinsic set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn a2a_value_intrinsics_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/a2a_values"))
        })
        .collect();
    assert_eq!(entries.len(), 14, "A2A value intrinsic set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn a2a_agent_card_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/a2a_agent_card"))
        })
        .collect();
    assert_eq!(entries.len(), 14, "A2A Agent Card value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn a2a_wire_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/a2a_wire_values"))
        })
        .collect();
    assert_eq!(entries.len(), 8, "A2A wire value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn a2a_stream_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/a2a_stream_values"))
        })
        .collect();
    assert_eq!(entries.len(), 18, "A2A stream value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn a2a_task_envelopes_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/a2a_task_envelopes"))
        })
        .collect();
    assert_eq!(entries.len(), 15, "A2A task envelope set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn thinking_level_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/thinking_level"))
        })
        .collect();
    assert_eq!(entries.len(), 9, "ThinkingLevel value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn steering_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/steering_values"))
        })
        .collect();
    assert_eq!(entries.len(), 13, "steering value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn subagent_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/subagent_values"))
        })
        .collect();
    assert_eq!(entries.len(), 15, "subagent value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn content_guard_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/content_guard_values"))
        })
        .collect();
    assert_eq!(entries.len(), 6, "content guard value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn guard_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/guard_values"))
        })
        .collect();
    assert_eq!(entries.len(), 6, "guard value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn delivery_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/delivery_values"))
        })
        .collect();
    assert_eq!(entries.len(), 16, "delivery value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn subagent_stop_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/subagent_stop_values"))
        })
        .collect();
    assert_eq!(entries.len(), 6, "subagent stop value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn task_terminal_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/task_terminal_values"))
        })
        .collect();
    assert_eq!(entries.len(), 7, "task terminal value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn permission_rule_sources_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/permission_rule_values"))
        })
        .collect();
    assert_eq!(entries.len(), 10, "permission rule source set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn permission_rule_behavior_has_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/permission_rule_behavior"))
        })
        .collect();
    assert_eq!(entries.len(), 6, "permission rule behavior set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn permission_mode_helpers_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/permission_mode_values"))
        })
        .collect();
    assert_eq!(entries.len(), 6, "permission mode helper set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn permission_rule_matcher_has_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/permission_rule_matcher"))
        })
        .collect();
    assert_eq!(entries.len(), 9, "permission rule matcher set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn command_cell_phases_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/command_cell_values"))
        })
        .collect();
    assert_eq!(entries.len(), 10, "command cell phase set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn command_cell_status_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/command_cell_status_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 15, "command cell status set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn team_strategy_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/team_strategy_values"))
        })
        .collect();
    assert_eq!(entries.len(), 7, "team strategy value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn acp_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/acp_values"))
        })
        .collect();
    assert_eq!(entries.len(), 13, "ACP value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn acp_config_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/acp_config_values"))
        })
        .collect();
    assert_eq!(entries.len(), 13, "ACP adapter config value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn acp_lease_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/acp_lease_values"))
        })
        .collect();
    assert_eq!(entries.len(), 5, "ACP lease error value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn jwt_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/jwt_values"))
        })
        .collect();
    assert_eq!(entries.len(), 9, "JWT value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn dependency_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/dependency_values"))
        })
        .collect();
    assert_eq!(entries.len(), 7, "dependency value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn context_inheritance_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/context_inheritance_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 11, "context inheritance set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn observed_isolation_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/observed_isolation_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 4, "observed isolation set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn segment_range_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/segment_range_values"))
        })
        .collect();
    assert_eq!(entries.len(), 3, "segment range set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn prompt_diagnostics_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/prompt_diagnostics_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 3, "prompt diagnostics set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn subagent_command_identity_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/subagent_command_identity_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 6, "Subagent command identity set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn subagent_usage_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/subagent_usage_values"))
        })
        .collect();
    assert_eq!(entries.len(), 3, "Subagent usage set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn tool_output_artifact_config_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/tool_output_artifact_config_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 7, "artifact config set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn skill_validation_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/skill_validation_values"))
        })
        .collect();
    assert_eq!(entries.len(), 2, "skill validation set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn skill_content_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/skill_content_values"))
        })
        .collect();
    assert_eq!(entries.len(), 2, "skill content set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn jsonrpc_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/jsonrpc_values"))
        })
        .collect();
    assert_eq!(entries.len(), 4, "JSON-RPC value set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn subagent_context_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/subagent_context_values"))
        })
        .collect();
    assert_eq!(entries.len(), 3, "Subagent context set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn usage_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/usage_values"))
        })
        .collect();
    assert_eq!(entries.len(), 6, "usage set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn hook_action_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/hook_action_values"))
        })
        .collect();
    assert_eq!(entries.len(), 30, "hook action set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn page_info_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/page_info_values"))
        })
        .collect();
    assert_eq!(entries.len(), 2, "page info set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn hook_event_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/hook_event_values"))
        })
        .collect();
    assert_eq!(entries.len(), 45, "hook event set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn event_identity_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/event_identity_values"))
        })
        .collect();
    assert_eq!(entries.len(), 29, "event identity set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn intervention_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/intervention_values"))
        })
        .collect();
    assert_eq!(entries.len(), 7, "intervention result set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn token_budget_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/token_budget_values"))
        })
        .collect();
    assert_eq!(entries.len(), 35, "token budget set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn execution_usage_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/execution_usage_values"))
        })
        .collect();
    assert_eq!(entries.len(), 1, "execution usage set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn turn_mode_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/turn_mode_values"))
        })
        .collect();
    assert_eq!(entries.len(), 3, "turn mode set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn retry_policy_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/retry_policy_values"))
        })
        .collect();
    assert_eq!(entries.len(), 9, "retry policy set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn thinking_config_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/thinking_config_values"))
        })
        .collect();
    assert_eq!(entries.len(), 14, "thinking config set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn time_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/time_values"))
        })
        .collect();
    assert_eq!(entries.len(), 8, "time helper set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn thinking_protocol_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/thinking_protocol_values"))
        })
        .collect();
    assert_eq!(entries.len(), 13, "thinking protocol set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn sandbox_resource_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/sandbox_resource_values"))
        })
        .collect();
    assert_eq!(entries.len(), 4, "sandbox resource set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn provider_capabilities_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/provider_capabilities_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 4, "provider capabilities set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn thinking_profile_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/thinking_profile_values"))
        })
        .collect();
    assert_eq!(entries.len(), 5, "thinking profile set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn model_profile_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/model_profile_values"))
        })
        .collect();
    assert_eq!(entries.len(), 32, "model profile set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn llm_api_protocol_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/llm_api_protocol_values"))
        })
        .collect();
    assert_eq!(entries.len(), 7, "LLM API protocol set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn model_input_modality_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/model_input_modality_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 7, "model input modality set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn response_format_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/response_format_values"))
        })
        .collect();
    assert_eq!(entries.len(), 6, "response format set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn prompt_cache_layout_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry.languages.values().all(|mapping| {
                    mapping
                        .contract_test
                        .ends_with("/prompt_cache_layout_values")
                })
        })
        .collect();
    assert_eq!(entries.len(), 3, "prompt cache layout set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn memory_scope_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/memory_scope_values"))
        })
        .collect();
    assert_eq!(entries.len(), 12, "memory scope set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn memory_policy_values_have_language_behavior_evidence() -> TestResult {
    let manifest = manifest()?;
    let entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.canonical
                && entry
                    .languages
                    .values()
                    .all(|mapping| mapping.contract_test.ends_with("/memory_policy_values"))
        })
        .collect();
    assert_eq!(entries.len(), 23, "memory policy set drifted");
    for entry in entries {
        for (language, mapping) in &entry.languages {
            assert_eq!(
                mapping.status,
                echo_sdk_protocol::inventory::LanguageImplementationStatus::Done,
                "{language}: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn known_facade_semantics_are_classified_correctly() -> TestResult {
    let manifest = manifest()?;
    // Presence belongs to accepted-external-contract.json. Validate any
    // surviving inventory examples without freezing Host/Rust-only members.
    for (path, classification) in [
        ("echo_agent::agent::Agent", SemanticClass::Extension),
        ("echo_agent::agent::AgentHandle", SemanticClass::Handle),
        ("echo_agent::llm::LlmClient", SemanticClass::Extension),
        (
            "echo_agent::agent::ReactAgentBuilder",
            SemanticClass::LanguageIntrinsic,
        ),
        (
            "echo_agent::agent::CancellationToken",
            SemanticClass::Handle,
        ),
        (
            "echo_agent::agent::AgentRunSnapshot::llm_client",
            SemanticClass::LanguageIntrinsic,
        ),
        ("echo_agent::agent::AgentRunSnapshot", SemanticClass::Handle),
        (
            "echo_agent::agent::subagent::SubagentExecutor",
            SemanticClass::Handle,
        ),
        ("echo_agent::intent::IntentRouter", SemanticClass::Handle),
        (
            "echo_agent::agent::react::run::pipeline::ToolExecutionPipeline",
            SemanticClass::Handle,
        ),
        (
            "echo_agent::agent::subagent::SharedIsolationProvider",
            SemanticClass::Extension,
        ),
        (
            "echo_agent::tools::SubagentUplinkFn",
            SemanticClass::LanguageIntrinsic,
        ),
        (
            "echo_agent::scheduler::FireFn",
            SemanticClass::LanguageIntrinsic,
        ),
    ] {
        if let Some(entry) = manifest.entries.iter().find(|entry| entry.path == path) {
            assert_eq!(
                entry.classification, classification,
                "classification: {path}"
            );
        }
    }
    for (path, relationship) in [
        (
            "echo_agent::evolution::PromptInjectionDetector",
            AcpRelationship::EchoExtension,
        ),
        (
            "echo_agent::agent::AgentEvent",
            AcpRelationship::StandardProjection,
        ),
        (
            "echo_agent::llm::types::LinkedResource",
            AcpRelationship::StandardProjection,
        ),
        (
            "echo_agent::prelude::LinkedResource",
            AcpRelationship::StandardProjection,
        ),
        (
            "echo_agent::acp::AcpAgentAdapter",
            AcpRelationship::LanguageIntrinsic,
        ),
        (
            "echo_agent::acp::AcpAdapterConfig",
            AcpRelationship::LanguageIntrinsic,
        ),
        (
            "echo_agent::acp::AcpSessionFactory",
            AcpRelationship::LanguageIntrinsic,
        ),
        (
            "echo_agent::acp::AcpSessionContext",
            AcpRelationship::StandardProjection,
        ),
    ] {
        if let Some(entry) = manifest.entries.iter().find(|entry| entry.path == path) {
            assert_eq!(
                entry.acp_relationship, relationship,
                "ACP relationship: {path}"
            );
        }
    }
    // Alias validity is checked for every existing entry above, including
    // route equality. No particular prelude alias is required to survive.
    Ok(())
}

#[test]
fn manifest_and_snapshot_agree_on_profiles() -> TestResult {
    let snapshot = read("contracts/sdk/public-api.txt")?;
    let snapshot_profiles = snapshot
        .lines()
        .find_map(|line| line.strip_prefix("# profiles: "))
        .ok_or("snapshot profiles header missing")?;
    assert_eq!(
        snapshot_profiles,
        manifest()?.generated.profiles.join(", "),
        "profile lists diverged"
    );
    Ok(())
}

#[test]
fn facade_route_table_is_mechanically_closed() -> TestResult {
    assert!(
        validate_facade_route_table().is_empty(),
        "route table violations: {:?}",
        validate_facade_route_table()
    );
    // Every family actually referenced by a canonical manifest item must
    // have a registered descriptor. A descriptor may legitimately have no
    // current root item: protocol-native tool families still own closed
    // operations after their Rust construction types become language-local.
    let manifest = manifest()?;
    let mut items_by_family: std::collections::BTreeMap<&str, usize> =
        std::collections::BTreeMap::new();
    // One operation identity may legitimately carry several signature
    // variants (cfg-shaped re-exports); the exact (operation, signature)
    // pair is what a request must match, so that pair must be unique.
    let mut invoke_signatures: BTreeSet<(&str, &str)> = BTreeSet::new();
    for entry in &manifest.entries {
        if let Some(family) = entry.route.family.as_deref()
            && entry.canonical
        {
            *items_by_family.entry(family).or_insert(0) += 1;
        }
        if let Some(operation) = entry.route.operation.as_deref()
            && entry.alias_of.is_none()
        {
            for signature in &entry.signatures {
                assert!(
                    invoke_signatures.insert((operation, signature.digest.as_str())),
                    "duplicate invoke operation signature {operation} {}",
                    signature.digest
                );
            }
        }
    }
    let registered_families: BTreeSet<&str> = FACADE_FAMILIES
        .iter()
        .map(|descriptor| descriptor.family.as_str())
        .collect();
    for family in items_by_family.keys() {
        assert!(
            registered_families.contains(family),
            "canonical manifest family {family} has no route descriptor"
        );
    }
    Ok(())
}

#[test]
fn intrinsic_routes_have_unique_identities_and_report_telemetry() -> TestResult {
    let mut routes = manifest()?
        .entries
        .into_iter()
        .filter(|entry| entry.canonical && entry.route.route.starts_with("intrinsic:"))
        .map(|entry| {
            format!(
                "{}\t{}\t{}\t{}",
                entry.kind.as_str(),
                entry.path,
                entry.source_paths.into_iter().collect::<Vec<_>>().join(","),
                entry.route.route
            )
        })
        .collect::<Vec<_>>();
    routes.sort();
    assert_eq!(
        routes.iter().collect::<BTreeSet<_>>().len(),
        routes.len(),
        "canonical intrinsic identities must remain unique"
    );
    let mut frozen = routes.join("\n");
    frozen.push('\n');
    let digest = format!("{:x}", Sha256::digest(frozen.as_bytes()));
    eprintln!("intrinsic membership telemetry digest: {digest}");
    Ok(())
}

#[test]
fn facade_operation_catalog_artifact_matches_manifest() -> TestResult {
    let manifest = manifest()?;
    let catalog: serde_json::Value =
        serde_json::from_str(&read("contracts/sdk/facade-operation-catalog.json")?)?;
    let total_items = catalog
        .get("total_items")
        .and_then(|v| v.as_u64())
        .ok_or("facade catalog missing total_items")?;
    assert_eq!(
        total_items,
        manifest.entries.len() as u64,
        "facade catalog item total drifted from the parity manifest"
    );
    let routes = catalog
        .get("routes")
        .and_then(|v| v.as_array())
        .ok_or("facade catalog missing routes")?;
    for descriptor in FACADE_FAMILIES
        .iter()
        .filter(|descriptor| !descriptor.family.operations().is_empty())
    {
        let route_id = format!("family:{}", descriptor.family.as_str());
        assert!(
            routes.iter().any(|route| {
                route.get("route").and_then(serde_json::Value::as_str) == Some(route_id.as_str())
            }),
            "closed family operations have no executable catalog route: {}",
            descriptor.family.as_str()
        );
    }
    let mut route_ids: Vec<&str> = routes
        .iter()
        .filter_map(|route| route.get("route").and_then(|v| v.as_str()))
        .collect();
    assert!(
        route_ids.iter().all(|id| !id.contains('*')),
        "wildcard route in the generated catalog"
    );
    route_ids.sort_unstable();
    route_ids.dedup();
    let manifest_routes: BTreeSet<&str> = manifest
        .entries
        .iter()
        .map(|e| e.route.route.as_str())
        .collect();
    for id in route_ids {
        if !manifest_routes.contains(id)
            && let Some(family) = id.strip_prefix("family:")
        {
            let route = routes
                .iter()
                .find(|route| route.get("route").and_then(|value| value.as_str()) == Some(id))
                .ok_or("generated catalog is missing a synthetic family route")?;
            let descriptor = FACADE_FAMILIES
                .iter()
                .find(|descriptor| descriptor.family.as_str() == family)
                .ok_or("synthetic family route has no descriptor")?;
            let operations: BTreeSet<&str> = route
                .get("operation_signatures")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|entry| entry.get("operation").and_then(serde_json::Value::as_str))
                .collect();
            let expected: BTreeSet<&str> = descriptor.family.operations().iter().copied().collect();
            assert_eq!(
                operations, expected,
                "protocol-owned family operations drifted for {family}"
            );
            assert_eq!(
                route.get("method").and_then(serde_json::Value::as_str),
                descriptor.methods.first().copied(),
                "synthetic family route method drifted for {family}"
            );
            assert_eq!(
                route.get("items").and_then(serde_json::Value::as_u64),
                Some(0),
                "synthetic family route must not fabricate manifest items"
            );
            continue;
        }
        assert!(
            manifest_routes.contains(id),
            "generated catalog route {id} is absent from the manifest"
        );
    }
    for route in routes {
        if matches!(
            route.get("surface").and_then(|value| value.as_str()),
            Some("invoke") | Some("family")
        ) {
            assert_eq!(
                route
                    .get("input")
                    .and_then(|value| value.get("encoding"))
                    .and_then(|value| value.as_str()),
                Some("wire_value_array"),
                "operation route is missing its wire input description"
            );
            assert_eq!(
                route
                    .get("result")
                    .and_then(|value| value.get("encoding"))
                    .and_then(|value| value.as_str()),
                Some("wire_value"),
                "operation route is missing its wire result description"
            );
        }
    }
    Ok(())
}

/// Plan 08: the extension obligation set is closed. Every
/// bridge-routed manifest item carries a typed kind the Host actually has
/// a proxy for, every typed kind is reachable from at least one canonical
/// trait identity, and the pre-closure `bridge:pending` middle state no
/// longer exists anywhere in the contract.
#[test]
fn extension_obligations_form_a_closed_bridge_set() -> TestResult {
    use echo_sdk_protocol::facade::typed_bridge_kinds;
    use echo_sdk_protocol::methods::ExtensionKind;

    let manifest = manifest()?;
    let typed: BTreeSet<String> = typed_bridge_kinds()
        .into_iter()
        .map(|kind| kind.as_str().to_string())
        .collect();
    // Hook registers by event descriptor, not by trait identity; it is the
    // only descriptor-routed kind. Everything else must come from the table.
    let mut expected = typed.clone();
    expected.insert(ExtensionKind::Hook.as_str().to_string());
    let all_kinds: BTreeSet<String> = [
        ExtensionKind::Tool,
        ExtensionKind::LlmClient,
        ExtensionKind::Store,
        ExtensionKind::HumanLoopProvider,
        ExtensionKind::Hook,
        ExtensionKind::AgentCallback,
        ExtensionKind::InterventionCallback,
        ExtensionKind::AgentFactory,
        ExtensionKind::CustomAgent,
        ExtensionKind::Critic,
        ExtensionKind::ChannelPlugin,
        ExtensionKind::ChannelMessageHandler,
        ExtensionKind::ContextCompressor,
        ExtensionKind::AgentComponent,
    ]
    .into_iter()
    .map(|kind| kind.as_str().to_string())
    .collect();
    assert_eq!(
        expected, all_kinds,
        "typed trait kinds + descriptor-routed Hook must cover every wire kind"
    );

    let mut routed_kinds: BTreeSet<String> = BTreeSet::new();
    for entry in &manifest.entries {
        let route = entry.route.route.as_str();
        assert!(
            route != "bridge:pending",
            "pending bridge route reopened on {}: {}",
            entry.path,
            route
        );
        if let Some(kind) = route.strip_prefix("bridge:")
            && entry.canonical
        {
            assert!(
                all_kinds.contains(kind),
                "unknown bridge kind '{kind}' on {}",
                entry.path
            );
            assert!(
                typed.contains(kind),
                "kind {} has no typed trait identity and is not Hook",
                kind
            );
            routed_kinds.insert(kind.to_string());
        }
    }
    assert_eq!(
        routed_kinds, typed,
        "every typed trait kind must be routed by at least one canonical item"
    );
    Ok(())
}

#[test]
fn canonical_consumer_traits_and_streams_have_executable_routes() -> TestResult {
    let manifest = manifest()?;
    // A newly discovered deferred Rust trait is telemetry, not an accepted
    // reverse bridge or a language API. Keep checking all supported routes.
    for entry in manifest
        .entries
        .iter()
        .filter(|entry| entry.canonical && entry.sdk_scope != SdkScope::Deferred)
    {
        if entry.classification == SemanticClass::Extension {
            assert!(
                matches!(entry.route.surface.as_str(), "bridge" | "intrinsic"),
                "consumer trait {} is mislabeled as an executable family/core route: {}",
                entry.path,
                entry.route.route
            );
            if entry.route.surface == "intrinsic" {
                assert!(
                    entry.route.route.starts_with("intrinsic:process-local-"),
                    "consumer trait {} lacks explicit process-local evidence: {}",
                    entry.path,
                    entry.route.route
                );
            }
        }
        if entry.classification == SemanticClass::Stream {
            assert!(
                matches!(
                    entry.route.surface.as_str(),
                    "bridge" | "core" | "family" | "intrinsic" | "invoke"
                ),
                "stream {} has no executable or evidenced route",
                entry.path
            );
        }
    }
    let workflow = echo_sdk_protocol::facade::FacadeFamily::Workflow.operations();
    for operation in [
        "workflow.graph.run_stream",
        "workflow.stream.next",
        "workflow.stream.cancel",
        "workflow.stream.close",
    ] {
        assert!(
            workflow.contains(&operation),
            "missing workflow stream operation {operation}"
        );
    }
    let a2a = echo_sdk_protocol::facade::FacadeFamily::A2a.operations();
    for operation in [
        "a2a.task.stream.open",
        "a2a.stream.next",
        "a2a.stream.cancel",
        "a2a.stream.close",
    ] {
        assert!(
            a2a.contains(&operation),
            "missing A2A stream operation {operation}"
        );
    }
    Ok(())
}

#[test]
fn source_family_operations_are_exact_and_type_scoped() -> TestResult {
    let manifest = manifest()?;
    let mut mapped = 0_usize;
    for entry in manifest.entries.iter().filter(|entry| entry.canonical) {
        let Some(handler_operation) = entry.route.handler_operation.as_deref() else {
            continue;
        };
        mapped = mapped.saturating_add(1);
        assert_eq!(
            entry.route.surface, "invoke",
            "mapped source route must use invoke"
        );
        assert_eq!(
            entry.route.method.as_deref(),
            Some("_echo_agent/facade/invoke"),
            "mapped source route must retain the source-signature admission method"
        );
        let canonical_source = entry
            .source_paths
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| entry.path.clone());
        assert_eq!(
            entry.route.operation.as_deref(),
            Some(canonical_source.as_str()),
            "mapped source route must preserve its exact canonical identity"
        );
        let family = entry
            .route
            .family
            .as_deref()
            .ok_or("mapped route has no family")?;
        let descriptor = FACADE_FAMILIES
            .iter()
            .find(|descriptor| descriptor.family.as_str() == family)
            .ok_or("mapped route has no family descriptor")?;
        assert!(
            descriptor.family.operations().contains(&handler_operation),
            "mapped source route {} targets an operation absent from family {family}: {handler_operation}",
            entry.path
        );
        assert!(
            !entry.path.contains("McpServerBuilder")
                && !entry.path.contains("PermissionServiceBuilder")
                && !entry.path.contains("WsClient"),
            "same-named construction or transport method was mapped as a runtime family operation: {}",
            entry.path
        );
    }
    assert!(
        mapped > 0,
        "the catalog must contain source-to-family operation mappings"
    );
    Ok(())
}

#[test]
fn canonical_intrinsic_routes_do_not_use_generic_fallback_reasons() -> TestResult {
    let manifest = manifest()?;
    let forbidden = [
        "intrinsic:callable-operation",
        "intrinsic:consumer-implemented-trait",
        "intrinsic:language-local-process-helper",
        "intrinsic:language-local-pure-helper",
        "intrinsic:language-local-react-agent-helper",
        "intrinsic:process-local-explicit-type-seam",
    ];
    for entry in manifest.entries.iter().filter(|entry| entry.canonical) {
        assert!(
            !forbidden.contains(&entry.route.route.as_str()),
            "canonical item {} uses a generic intrinsic fallback: {}",
            entry.path,
            entry.route.route
        );
    }
    Ok(())
}

#[test]
fn new_upstream_managed_and_team_seams_have_explicit_sdk_scope() {
    use echo_sdk_protocol::facade::governance_source_disposition;
    for source in [
        "echo_core::memory::conversation::ConversationStore::import_managed_messages",
        "echo_core::memory::conversation::ConversationStore::get_latest_managed_import_with_context",
        "echo_core::memory::conversation::ConversationStore::update_managed_conversation",
        "echo_core::memory::conversation::ManagedConversationMetadataUpdate::prepare",
    ] {
        assert_eq!(
            governance_source_disposition(source),
            Some("deferred-managed-import-and-metadata-without-host-caller")
        );
    }
    for source in [
        "echo_agent::agent::subagent::team::TeamRuntimeHandle::snapshot",
        "echo_agent::agent::subagent::team::TeamDispatchController::dispatch",
        "echo_agent::agent::subagent::team::execute_team_on_runtime_service",
    ] {
        assert_eq!(
            governance_source_disposition(source),
            Some("process-local-team-runtime-and-dispatch-authority")
        );
    }
    for production_caller in [
        "echo_core::memory::conversation::ConversationStore::apply_transcript_projection_with_context",
        "echo_core::memory::conversation::ConversationStore::delete_managed_conversation_with_context",
        "echo_agent::state::RuntimeStateStore::compare_and_save_checkpoint_with_context",
    ] {
        assert!(
            governance_source_disposition(production_caller).is_none(),
            "existing managed caller bridge must remain accepted"
        );
    }
}

#[test]
fn runtime_capability_getter_is_host_preflight_without_new_remote_api() {
    assert_eq!(
        echo_sdk_protocol::facade::governance_source_disposition(
            "echo_orchestration::tasks::revisioned::TaskRevisionService::supports_runtime_execution"
        ),
        Some("process-local-task-runtime-capability-preflight")
    );
    assert!(
        echo_sdk_protocol::facade::governance_source_disposition(
            "echo_orchestration::tasks::revisioned::RevisionedTaskStore::supports_runtime_execution"
        )
        .is_none(),
        "the accepted registered-store bridge projects its capability descriptor"
    );
}

fn accepted_seed() -> TestResult<AcceptedExternalContract> {
    Ok(serde_json::from_str(&read(
        "contracts/sdk/accepted-external-contract.json",
    )?)?)
}

fn source_key(
    entry: &echo_sdk_protocol::inventory::ManifestEntry,
) -> (String, echo_sdk_protocol::inventory::ItemKind) {
    (
        entry
            .source_paths
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| entry.path.clone()),
        entry.kind,
    )
}

#[test]
fn acceptance_scope_new_routable_public_identity_never_implies_acceptance() -> TestResult {
    let seed = accepted_seed()?;
    let mut current = manifest()?;
    let mut new = seed
        .entries
        .iter()
        .find(|entry| entry.route.surface == "invoke")
        .cloned()
        .ok_or("accepted source operation missing")?;
    new.path = "echo_agent::NewRuntimeCapability::execute".to_string();
    new.source_paths = BTreeSet::from(["echo_core::NewRuntimeCapability::execute".to_string()]);
    new.route.route = "source:echo_core::NewRuntimeCapability::execute".to_string();
    new.route.operation = Some("echo_core::NewRuntimeCapability::execute".to_string());
    let new_key = source_key(&new);
    current.entries.push(new);
    echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::new(),
        &BTreeSet::new(),
    )?;
    assert_eq!(
        current
            .entries
            .iter()
            .find(|entry| source_key(entry) == new_key)
            .ok_or("new identity disappeared from telemetry")?
            .sdk_scope,
        SdkScope::Deferred
    );
    assert!(
        !echo_sdk_protocol::inventory::accepted_external_contract_document(&current)
            .entries
            .iter()
            .any(|entry| source_key(entry) == new_key)
    );
    Ok(())
}

#[test]
fn acceptance_scope_exact_accepted_source_and_kind_survive_alias_and_signature_changes()
-> TestResult {
    let seed = accepted_seed()?;
    let mut current = manifest()?;
    let key = source_key(seed.entries.first().ok_or("empty accepted seed")?);
    let entry = current
        .entries
        .iter_mut()
        .find(|entry| entry.canonical && source_key(entry) == key)
        .ok_or("accepted identity missing")?;
    entry.path.push_str("_new_facade_alias");
    entry.signatures = vec![echo_sdk_protocol::inventory::ManifestSignature {
        digest: "sha256:changed".to_string(),
    }];
    entry.route.signatures = entry.signatures.clone();
    echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::new(),
        &BTreeSet::new(),
    )?;
    let accepted = echo_sdk_protocol::inventory::accepted_external_contract_document(&current);
    let entry = accepted
        .entries
        .iter()
        .find(|entry| source_key(entry) == key)
        .ok_or("silently retired accepted signature")?;
    assert!(entry.path.ends_with("_new_facade_alias"));
    assert_eq!(
        entry
            .signatures
            .first()
            .map(|signature| signature.digest.as_str()),
        Some("sha256:changed")
    );
    assert_ne!(accepted, seed);
    Ok(())
}

#[test]
fn acceptance_scope_disappearing_accepted_identity_fails_atomically_without_explicit_retirement()
-> TestResult {
    let seed = accepted_seed()?;
    let mut current = manifest()?;
    let key = source_key(seed.entries.first().ok_or("empty accepted seed")?);
    current.entries.retain(|entry| source_key(entry) != key);
    let before = current.clone();
    let error = echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::new(),
        &BTreeSet::new(),
    )
    .err()
    .ok_or("silent accepted retirement")?;
    assert!(error.contains("accepted source disappeared"));
    assert_eq!(current, before);
    echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::new(),
        &BTreeSet::from([key.clone()]),
    )?;
    assert!(
        !echo_sdk_protocol::inventory::accepted_external_contract_document(&current)
            .entries
            .iter()
            .any(|entry| source_key(entry) == key)
    );
    Ok(())
}

#[test]
fn acceptance_scope_new_task_result_field_requires_exact_accepted_delta_without_accepting_its_siblings()
-> TestResult {
    let mut seed = accepted_seed()?;
    let mut current = manifest()?;
    // Build the pre-acceptance fixture explicitly: the real current seed now
    // includes result, so re-accepting it must correctly fail as a duplicate.
    let result_key = (
        "echo_orchestration::tasks::runtime::TaskExecution::result".to_string(),
        echo_sdk_protocol::inventory::ItemKind::StructField,
    );
    seed.entries.retain(|entry| source_key(entry) != result_key);
    current
        .entries
        .retain(|entry| source_key(entry) != result_key);
    let field = seed
        .entries
        .iter()
        .find(|entry| entry.path.ends_with("TaskExecution::status"))
        .cloned()
        .ok_or("accepted Task execution field missing")?;
    let mut keys = Vec::new();
    for name in ["result", "unreviewed_future_field"] {
        let mut new = field.clone();
        new.path = format!("echo_agent::tasks::TaskExecution::{name}");
        new.source_paths = BTreeSet::from([format!(
            "echo_orchestration::tasks::runtime::TaskExecution::{name}"
        )]);
        keys.push(source_key(&new));
        current.entries.push(new);
    }
    let accepted_key = keys.first().cloned().ok_or("result field missing")?;
    echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::from([accepted_key.clone()]),
        &BTreeSet::new(),
    )?;
    for key in keys {
        let entry = current
            .entries
            .iter()
            .find(|entry| source_key(entry) == key)
            .ok_or("new field missing")?;
        assert_eq!(
            entry.sdk_scope,
            if key == accepted_key {
                SdkScope::ExternalContract
            } else {
                SdkScope::Deferred
            }
        );
    }
    Ok(())
}

#[test]
fn acceptance_scope_recovered_turn_reservation_is_host_authority_without_new_language_contract()
-> TestResult {
    let seed = accepted_seed()?;
    let mut current = manifest()?;
    let mut new = seed
        .entries
        .iter()
        .find(|entry| entry.kind == echo_sdk_protocol::inventory::ItemKind::Method)
        .cloned()
        .ok_or("method fixture missing")?;
    new.path = "echo_agent::acp::AcpSession::reserve_recovered_turn_ids".to_string();
    new.source_paths = BTreeSet::from([new.path.clone()]);
    let key = source_key(&new);
    current.entries.push(new);
    echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::new(),
        &BTreeSet::new(),
    )?;
    assert_eq!(
        current
            .entries
            .iter()
            .find(|entry| source_key(entry) == key)
            .ok_or("recovery method missing")?
            .sdk_scope,
        SdkScope::HostOrRustOnly
    );
    assert!(
        echo_sdk_protocol::inventory::apply_accepted_source_scope(
            &mut current,
            &seed,
            &BTreeSet::from([key]),
            &BTreeSet::new()
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn acceptance_scope_current_seed_reproduces_exact_committed_membership() -> TestResult {
    let seed = accepted_seed()?;
    let mut current = manifest()?;
    let empty = BTreeSet::new();
    echo_sdk_protocol::inventory::apply_accepted_source_scope(&mut current, &seed, &empty, &empty)?;
    assert_eq!(
        echo_sdk_protocol::inventory::accepted_external_contract_document(&current),
        seed
    );
    Ok(())
}

#[test]
fn acceptance_scope_invalid_deltas_fail_atomically() -> TestResult {
    use echo_sdk_protocol::inventory::{ItemKind, apply_accepted_source_scope};
    let seed = accepted_seed()?;
    let current = manifest()?;
    let key = source_key(seed.entries.first().ok_or("accepted seed empty")?);
    let empty = BTreeSet::new();
    for (additions, retirements) in [
        (BTreeSet::from([key.clone()]), empty.clone()),
        (BTreeSet::from([key.clone()]), BTreeSet::from([key.clone()])),
        (
            BTreeSet::from([(
                "echo_core::UnknownCapability::invoke".to_string(),
                ItemKind::Method,
            )]),
            empty.clone(),
        ),
        (
            empty.clone(),
            BTreeSet::from([(
                "echo_core::UnknownCapability::invoke".to_string(),
                ItemKind::Method,
            )]),
        ),
    ] {
        let mut candidate = current.clone();
        assert!(
            apply_accepted_source_scope(&mut candidate, &seed, &additions, &retirements).is_err()
        );
        assert_eq!(candidate, current);
    }
    let mut changed_kind = current.clone();
    for entry in changed_kind
        .entries
        .iter_mut()
        .filter(|entry| source_key(entry) == key)
    {
        entry.kind = if entry.kind == ItemKind::Method {
            ItemKind::Function
        } else {
            ItemKind::Method
        };
    }
    let before = changed_kind.clone();
    assert!(apply_accepted_source_scope(&mut changed_kind, &seed, &empty, &empty).is_err());
    assert_eq!(changed_kind, before);
    Ok(())
}

#[test]
fn acceptance_scope_reports_all_missing_sources_before_mutation_and_excludes_retired() -> TestResult
{
    let seed = accepted_seed()?;
    let keys: Vec<_> = seed.entries.iter().take(3).map(source_key).collect();
    assert_eq!(
        keys.len(),
        3,
        "the accepted seed needs three distinct source identities"
    );
    let mut current = manifest()?;
    current
        .entries
        .retain(|entry| !keys.contains(&source_key(entry)));
    let before = current.clone();
    let error = echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::new(),
        &BTreeSet::new(),
    )
    .err()
    .ok_or("multiple disappeared sources were silently retired")?;
    for (identity, kind) in &keys {
        assert!(
            error.contains(&format!("{identity} ({kind:?})")),
            "missing closure omitted {identity}: {error}"
        );
    }
    assert_eq!(
        current, before,
        "diagnostic collection must not alter the manifest"
    );
    let retired = keys.first().ok_or("first source is absent")?.clone();
    let error = echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::new(),
        &BTreeSet::from([retired.clone()]),
    )
    .err()
    .ok_or("partial retirement hid the remaining missing sources")?;
    assert!(
        !error.contains(&format!("{} ({:?})", retired.0, retired.1)),
        "an explicitly retired source must not remain in the missing report"
    );
    for (identity, kind) in keys.iter().skip(1) {
        assert!(error.contains(&format!("{identity} ({kind:?})")));
    }
    assert_eq!(current, before);
    echo_sdk_protocol::inventory::apply_accepted_source_scope(
        &mut current,
        &seed,
        &BTreeSet::new(),
        &keys.into_iter().collect(),
    )?;
    Ok(())
}

#[test]
fn security_defining_source_migration_preserves_existing_operations() -> TestResult {
    use echo_sdk_protocol::facade::{CanonicalRoute, resolve_route};
    let inventory = manifest()?;
    for name in ["contains_secrets", "redact_secrets", "scan_secrets"] {
        let old = format!("echo_agent::security::{name}");
        let new = format!("echo_core::utils::retention::{name}");
        let original = inventory
            .entries
            .iter()
            .find(|entry| entry.path == old || entry.path == new)
            .ok_or_else(|| format!("missing Security operation {name}"))?;
        let entry = echo_sdk_protocol::inventory::InventoryEntry {
            path: new.clone(),
            kind: original.kind,
            source_paths: BTreeSet::from([new]),
            signatures: BTreeMap::new(),
            profiles: BTreeSet::new(),
            declared_feature_requirements: BTreeSet::new(),
            automatically_derived: false,
        };
        let route = resolve_route(
            &entry,
            original.classification,
            original.acp_relationship,
            "callable-operation",
        );
        assert_eq!(route, CanonicalRoute::SourceOperation { operation: old });
    }
    Ok(())
}

#[test]
fn task_context_getters_remain_native_without_language_acceptance() -> TestResult {
    use echo_sdk_protocol::inventory::ItemKind;
    let accepted: AcceptedExternalContract =
        serde_json::from_str(&read("contracts/sdk/accepted-external-contract.json")?)?;
    let inventory = manifest()?;
    for name in ["run_id", "delegation_policy", "waived_dependency_ids"] {
        let identity = format!("echo_orchestration::tasks::runtime::TaskSubagentContext::{name}");
        assert!(
            !accepted
                .entries
                .iter()
                .any(|entry| entry.source_paths.contains(&identity))
        );
        let entries: Vec<_> = inventory
            .entries
            .iter()
            .filter(|entry| entry.source_paths.contains(&identity))
            .collect();
        assert!(!entries.is_empty(), "native getter missing: {identity}");
        for entry in entries {
            assert_eq!(entry.kind, ItemKind::Method);
            assert_eq!(
                entry.sdk_scope,
                SdkScope::HostOrRustOnly,
                "getter scope: {}",
                entry.path
            );
        }
    }
    Ok(())
}

#[test]
fn new_deferred_rust_items_do_not_create_compatibility_obligations() -> TestResult {
    let original = manifest()?;
    let expected = compatibility_document(&original);
    let deferred = [
        "echo_core::tools::ToolResult::with_effect",
        "echo_state::audit::DiagnosticDeliveryObserver",
    ];
    let accepted: AcceptedExternalContract =
        serde_json::from_str(&read("contracts/sdk/accepted-external-contract.json")?)?;
    let catalog: serde_json::Value = serde_json::from_str(&read(
        "contracts/sdk/accepted-facade-operation-catalog.json",
    )?)?;
    let routes = catalog
        .get("routes")
        .and_then(serde_json::Value::as_array)
        .ok_or("accepted routes missing")?;
    for identity in deferred {
        let entry = original
            .entries
            .iter()
            .find(|entry| entry.canonical && entry.source_paths.contains(identity))
            .ok_or("deferred fixture missing")?;
        assert_eq!(
            entry.sdk_scope,
            SdkScope::Deferred,
            "unreviewed source must stay deferred: {identity}"
        );
        assert!(
            !accepted
                .entries
                .iter()
                .any(|entry| entry.source_paths.contains(identity))
        );
        assert!(!routes.iter().any(|route| {
            route.get("operation").and_then(serde_json::Value::as_str) == Some(identity)
        }));
    }
    let mut without = original.clone();
    without.entries.retain(|entry| {
        !deferred
            .iter()
            .any(|identity| entry.source_paths.contains(*identity))
    });
    assert_ne!(without, original);
    assert_eq!(
        compatibility_document(&without),
        expected,
        "deferred discovery must not change the blocking contract"
    );
    Ok(())
}
