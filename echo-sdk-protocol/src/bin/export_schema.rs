//! Deterministic export entry for the SDK contract artifacts.
//!
//! Default mode is a read-only `check`: every artifact is regenerated into
//! memory and compared against the committed copy; any drift exits non-zero
//! without touching the worktree. `--update` regenerates every artifact.
//! `--wire-candidate-update` combines committed inventory obligations with
//! current wire definitions for staged validation. It changes wire artifacts,
//! facade catalogs and their source digest, without rediscovering Rust scope.
//! It cannot establish the full contract gate or final-snapshot provenance.
//!
//! Artifacts governed here:
//! - complete inventory/Host catalog and `inventory-telemetry.json` — non-blocking drift evidence
//! - accepted external contract/catalog and `source-contract.json` — blocking compatibility inputs
//! - extension schema + fixtures — blocking wire-contract evidence
//!
//! The rustdoc toolchain is pinned in `contracts/sdk/toolchain.json`; if it
//! is missing locally the tool fails with the exact install prerequisite
//! instead of auto-installing anything (design §16/§20.5).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use echo_sdk_protocol::EXTENSION_PROTOCOL_VERSION;
use echo_sdk_protocol::inventory::{
    self as inv, FeatureProfile, InventoryError, PublicItem, RUSTDOC_FORMAT_VERSION,
};

const TOOLCHAIN_JSON: &str = "contracts/sdk/toolchain.json";
const PUBLIC_API_TXT: &str = "contracts/sdk/public-api.txt";
const PARITY_MANIFEST_SCHEMA_JSON: &str = "contracts/sdk/parity-manifest.schema.json";
const PARITY_MANIFEST_JSON: &str = "contracts/sdk/parity-manifest.json";
const ACCEPTED_EXTERNAL_CONTRACT_JSON: &str = "contracts/sdk/accepted-external-contract.json";
const ACCEPTED_EXTERNAL_CONTRACT_SCHEMA_JSON: &str =
    "contracts/sdk/accepted-external-contract.schema.json";
const ACCEPTED_FACADE_CATALOG_JSON: &str = "contracts/sdk/accepted-facade-operation-catalog.json";
const INVENTORY_TELEMETRY_JSON: &str = "contracts/sdk/inventory-telemetry.json";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
    {
        println!(
            "export_schema [--check | --update | --telemetry-check | --wire-candidate-update]\n\
            --check (default): read-only full accepted-contract drift check\n\
            --update: regenerate inventory using the existing accepted source scope; no implicit additions/retirements\n\
            --accept-source <kind> <identity> <docs/adr/file.md#section>: explicit reviewed acceptance in full --update\n\
            --retire-source <kind> <identity> <docs/adr/file.md#section>: explicit reviewed retirement in full --update\n\
            --telemetry-check: report complete inventory drift without making it a compatibility input\n\
            --wire-candidate-update: staged schema/fixtures/catalogs from committed inventory; full --update/--check still required"
        );
        return ExitCode::SUCCESS;
    }
    let wire_candidate = match wire_candidate_requested(&args) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };
    let update = args.iter().any(|arg| arg == "--update");
    let telemetry_check = args.iter().any(|arg| arg == "--telemetry-check");
    if update && telemetry_check {
        eprintln!("error: --update and --telemetry-check are mutually exclusive");
        return ExitCode::FAILURE;
    }
    let mode = if wire_candidate {
        "wire-candidate-update (partial wire candidate, not full acceptance)"
    } else if update {
        "update"
    } else if telemetry_check {
        "telemetry-check"
    } else {
        "check"
    };
    eprintln!("echo-sdk-protocol export_schema: mode={mode}");

    let repo_root = match locate_repo_root() {
        Some(root) => root,
        None => {
            eprintln!("error: cannot locate repository root (missing {TOOLCHAIN_JSON})");
            return ExitCode::FAILURE;
        }
    };
    let contracts_dir = repo_root.join("contracts/sdk");
    if !contracts_dir.is_dir() {
        eprintln!("error: {} not found", contracts_dir.display());
        return ExitCode::FAILURE;
    }

    // Deliberately return before any rustdoc toolchain/profile/metadata work.
    // This mode has no authority to change inventory or accepted source scope.
    if wire_candidate {
        if let Err(error) = acceptance_delta(&args, &repo_root, false) {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
        let artifacts = match wire_candidate_artifacts(&repo_root) {
            Ok(artifacts) => artifacts,
            Err(error) => {
                eprintln!("error: {error}");
                return ExitCode::FAILURE;
            }
        };
        if let Err(error) = validate_extension_candidate(&repo_root, &artifacts)
            .and_then(|()| validate_generated_file_set(&repo_root, &artifacts))
            .and_then(|()| write_artifacts(&repo_root, &artifacts))
        {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
        eprintln!(
            "partial wire candidate written: {} artifacts from committed inventory; public inventory, accepted identity scope and framework provenance unchanged; full --update/--check still required",
            artifacts.len()
        );
        return ExitCode::SUCCESS;
    }

    let acceptance_delta = match acceptance_delta(&args, &repo_root, update && !telemetry_check) {
        Ok(delta) => delta,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };
    let accepted_seed: inv::AcceptedExternalContract =
        match std::fs::read(repo_root.join(ACCEPTED_EXTERNAL_CONTRACT_JSON))
            .map_err(|error| format!("reading accepted contract seed: {error}"))
            .and_then(|bytes| {
                serde_json::from_slice(&bytes)
                    .map_err(|error| format!("parsing accepted contract seed: {error}"))
            }) {
            Ok(seed) => seed,
            Err(error) => {
                eprintln!("error: {error}");
                return ExitCode::FAILURE;
            }
        };

    let toolchain = match load_rustdoc_toolchain(&repo_root) {
        Ok(toolchain) => toolchain,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };
    eprintln!("rustdoc toolchain: {}", toolchain.name);

    let leaf_features = match leaf_features_of_root_crate(&repo_root) {
        Ok(features) => features,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };
    if leaf_features.is_empty() {
        eprintln!("error: root echo_agent crate exposes no leaf features; refusing empty matrix");
        return ExitCode::FAILURE;
    }
    eprintln!(
        "leaf features ({}): {}",
        leaf_features.len(),
        leaf_features.join(", ")
    );

    let profiles = inv::profiles_for_leaf_features(&leaf_features);
    let per_profile = match generate_all_profiles(&repo_root, &toolchain, &profiles) {
        Ok(per_profile) => per_profile,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let merged = match inv::merge_profiles(&per_profile) {
        Ok(merged) => merged,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };
    let snapshot = inv::render_public_api_snapshot(&profiles, &merged);
    let mut manifest_document =
        inv::manifest_document(EXTENSION_PROTOCOL_VERSION, &profiles, &merged);
    if let Err(error) = inv::apply_accepted_source_scope(
        &mut manifest_document,
        &accepted_seed,
        &acceptance_delta.0,
        &acceptance_delta.1,
    ) {
        eprintln!("error: {error}");
        return ExitCode::FAILURE;
    }
    let manifest = inv::render_manifest_document(&manifest_document);
    let manifest_schema = inv::render_parity_manifest_schema();
    let accepted_external_contract = inv::render_accepted_external_contract(&manifest_document);
    let accepted_external_contract_schema = inv::render_accepted_external_contract_schema();

    let (facade_catalog_json, accepted_facade_catalog_json, source_contract) =
        match catalog_documents(&manifest_document, accepted_external_contract.as_bytes()) {
            Ok(documents) => documents,
            Err(error) => {
                eprintln!("error: {error}");
                return ExitCode::FAILURE;
            }
        };
    let framework_source = match framework_dependency_source(&repo_root) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };
    let inventory_telemetry =
        inv::render_inventory_telemetry(&manifest_document, &framework_source);

    let mut artifacts: Vec<(PathBuf, String)> = vec![
        (repo_root.join(PUBLIC_API_TXT), snapshot),
        (repo_root.join(PARITY_MANIFEST_SCHEMA_JSON), manifest_schema),
        (repo_root.join(PARITY_MANIFEST_JSON), manifest),
        (
            repo_root.join("contracts/sdk/facade-operation-catalog.json"),
            facade_catalog_json,
        ),
        (
            repo_root.join(ACCEPTED_EXTERNAL_CONTRACT_JSON),
            accepted_external_contract,
        ),
        (
            repo_root.join(ACCEPTED_EXTERNAL_CONTRACT_SCHEMA_JSON),
            accepted_external_contract_schema,
        ),
        (
            repo_root.join(ACCEPTED_FACADE_CATALOG_JSON),
            accepted_facade_catalog_json,
        ),
        (
            repo_root.join(INVENTORY_TELEMETRY_JSON),
            inventory_telemetry,
        ),
        (
            repo_root.join("contracts/sdk/source-contract.json"),
            echo_sdk_protocol::schema::canonical_json(&source_contract),
        ),
    ];
    // Boundary validation before writing: the extension schema must never
    // shadow official ACP concepts.
    let schema_doc = echo_sdk_protocol::schema::build_extension_schema_doc();
    let boundary_problems = echo_sdk_protocol::schema::validate_schema_boundaries(&schema_doc);
    if !boundary_problems.is_empty() {
        for problem in &boundary_problems {
            eprintln!("schema boundary violation: {problem}");
        }
        return ExitCode::FAILURE;
    }
    artifacts.extend(extension_schema_artifacts(&repo_root));
    if let Err(error) = validate_generated_schemas(&repo_root, &artifacts) {
        eprintln!("error: {error}");
        return ExitCode::FAILURE;
    }
    if let Err(error) = validate_generated_file_set(&repo_root, &artifacts) {
        eprintln!("error: {error}");
        return ExitCode::FAILURE;
    }

    let mut drifted: Vec<&PathBuf> = Vec::new();
    let mut telemetry_drifted: Vec<&PathBuf> = Vec::new();
    for (path, content) in &artifacts {
        let telemetry = matches!(
            rel(&repo_root, path).as_str(),
            PUBLIC_API_TXT
                | PARITY_MANIFEST_SCHEMA_JSON
                | PARITY_MANIFEST_JSON
                | "contracts/sdk/facade-operation-catalog.json"
                | INVENTORY_TELEMETRY_JSON
        );
        match std::fs::read_to_string(path) {
            Ok(existing) if &existing == content => {
                eprintln!(
                    "ok: {} matches ({} bytes)",
                    rel(&repo_root, path),
                    content.len()
                );
            }
            Ok(existing) => {
                if telemetry {
                    telemetry_drifted.push(path);
                } else {
                    drifted.push(path);
                }
                eprintln!(
                    "{}: {} committed {} bytes, regenerated {} bytes",
                    if telemetry {
                        "TELEMETRY DRIFT"
                    } else {
                        "DRIFT"
                    },
                    rel(&repo_root, path),
                    existing.len(),
                    content.len()
                );
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                if telemetry {
                    telemetry_drifted.push(path);
                    eprintln!("TELEMETRY MISSING: {}", rel(&repo_root, path));
                } else {
                    drifted.push(path);
                    eprintln!("MISSING: {}", rel(&repo_root, path));
                }
            }
            Err(err) => {
                eprintln!("error: reading {}: {err}", rel(&repo_root, path));
                return ExitCode::FAILURE;
            }
        }
    }

    if update {
        if let Err(error) = write_artifacts(&repo_root, &artifacts) {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
        eprintln!("update complete: {} artifacts written", artifacts.len());
        return ExitCode::SUCCESS;
    }

    if drifted.is_empty() {
        if telemetry_check && !telemetry_drifted.is_empty() {
            eprintln!(
                "telemetry drift observed: {} artifact(s); this signal is non-blocking",
                telemetry_drifted.len()
            );
        }
        eprintln!(
            "check passed: {} blocking artifacts current ({} telemetry artifact(s) drifted; {} inventory items, rustdoc format {RUSTDOC_FORMAT_VERSION})",
            artifacts.len().saturating_sub(telemetry_drifted.len()),
            telemetry_drifted.len(),
            merged.len()
        );
        ExitCode::SUCCESS
    } else {
        eprintln!(
            "check FAILED: {} artifact(s) drifted; run \
             `cargo run -p echo-sdk-protocol --bin export_schema -- --update` and review the diff",
            drifted.len()
        );
        ExitCode::FAILURE
    }
}

/// One producer shared by full target generation and staged wire candidates.
/// It never reclassifies inventory entries or infers accepted identities.
fn catalog_documents(
    manifest: &inv::ParityManifest,
    accepted_external: &[u8],
) -> Result<(String, String, serde_json::Value), String> {
    let problems = echo_sdk_protocol::facade::validate_facade_route_table();
    if !problems.is_empty() {
        return Err(format!(
            "facade route table violations: {}",
            problems.join(", ")
        ));
    }
    if manifest.extension_protocol_version != EXTENSION_PROTOCOL_VERSION {
        return Err("committed inventory protocol version differs from wire definitions; full --update required".to_string());
    }
    let obligations = manifest
        .entries
        .iter()
        .map(|entry| &entry.route)
        .collect::<Vec<_>>();
    let catalog = echo_sdk_protocol::facade::build_facade_operation_catalog(
        EXTENSION_PROTOCOL_VERSION,
        &obligations,
    );
    let full = echo_sdk_protocol::schema::canonical_json(&catalog);
    let accepted_obligations = manifest
        .entries
        .iter()
        .filter(|entry| entry.canonical && entry.sdk_scope == inv::SdkScope::ExternalContract)
        .map(|entry| &entry.route)
        .collect::<Vec<_>>();
    let accepted_catalog = echo_sdk_protocol::facade::build_facade_operation_catalog(
        EXTENSION_PROTOCOL_VERSION,
        &accepted_obligations,
    );
    let accepted = echo_sdk_protocol::schema::canonical_json(
        &echo_sdk_protocol::facade::filter_external_facade_operation_catalog(&accepted_catalog),
    );
    let source = echo_sdk_protocol::schema::build_source_contract_doc(&[
        (
            echo_sdk_protocol::schema::SOURCE_CONTRACT_INPUTS[0],
            accepted_external,
        ),
        (
            echo_sdk_protocol::schema::SOURCE_CONTRACT_INPUTS[1],
            accepted.as_bytes(),
        ),
    ]);
    Ok((full, accepted, source))
}

fn wire_candidate_artifacts(repo_root: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    use sha2::{Digest as _, Sha256};
    let inventory = std::fs::read(repo_root.join(PARITY_MANIFEST_JSON))
        .map_err(|error| format!("reading committed inventory: {error}"))?;
    let accepted_external = std::fs::read(repo_root.join(ACCEPTED_EXTERNAL_CONTRACT_JSON))
        .map_err(|error| format!("reading accepted external identities: {error}"))?;
    let manifest: inv::ParityManifest = serde_json::from_slice(&inventory)
        .map_err(|error| format!("parsing committed inventory: {error}"))?;
    let (catalog, accepted_catalog, source) = catalog_documents(&manifest, &accepted_external)?;
    let mut artifacts = extension_schema_artifacts(repo_root);
    artifacts.extend([
        (
            repo_root.join("contracts/sdk/facade-operation-catalog.json"),
            catalog,
        ),
        (
            repo_root.join(ACCEPTED_FACADE_CATALOG_JSON),
            accepted_catalog,
        ),
        (
            repo_root.join("contracts/sdk/source-contract.json"),
            echo_sdk_protocol::schema::canonical_json(&source),
        ),
    ]);
    eprintln!(
        "candidate producer input: {PARITY_MANIFEST_JSON} sha256:{:x}",
        Sha256::digest(&inventory)
    );
    eprintln!(
        "candidate producer input: {ACCEPTED_EXTERNAL_CONTRACT_JSON} sha256:{:x} (identity scope retained)",
        Sha256::digest(&accepted_external)
    );
    Ok(artifacts)
}

/// Explicit reviewed scope changes are invocation inputs, never another
/// persisted accepted registry. The generated accepted contract is the seed
/// for the next invocation. Every identity carries its own existing ADR link.
type SourceScopeDelta = (
    BTreeSet<(String, inv::ItemKind)>,
    BTreeSet<(String, inv::ItemKind)>,
);

fn markdown_section(document: &str, anchor: &str) -> Option<String> {
    let mut result = String::new();
    let mut matched = false;
    let mut level = 0usize;
    for line in document.lines() {
        let heading = line.trim_start();
        let heading_level = heading
            .chars()
            .take_while(|character| *character == '#')
            .count();
        if heading_level > 0
            && heading
                .chars()
                .nth(heading_level)
                .is_some_and(char::is_whitespace)
        {
            if matched && heading_level <= level {
                break;
            }
            let title = heading.chars().skip(heading_level).collect::<String>();
            let heading_anchor = title
                .trim()
                .to_lowercase()
                .chars()
                .filter_map(|character| {
                    if character.is_alphanumeric() || character == '-' || character == '_' {
                        Some(character)
                    } else if character.is_whitespace() {
                        Some('-')
                    } else {
                        None
                    }
                })
                .collect::<String>();
            if heading_anchor == anchor {
                matched = true;
                level = heading_level;
            }
            continue;
        }
        if matched {
            result.push_str(line);
            result.push('\n');
        }
    }
    matched.then_some(result)
}

fn decision_field<'a>(section: &'a str, name: &str) -> Vec<&'a str> {
    let prefix = format!("- {name}:");
    section
        .lines()
        .filter_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
        .collect()
}

fn acceptance_delta(
    args: &[String],
    repo_root: &Path,
    update: bool,
) -> Result<SourceScopeDelta, String> {
    let mut additions = BTreeSet::new();
    let mut retirements = BTreeSet::new();
    let mut args = args.iter().skip(1);
    while let Some(arg) = args.next() {
        let kind = match arg.as_str() {
            "--accept-source" => "accept",
            "--retire-source" => "retire",
            _ => continue,
        };
        if !update {
            return Err("acceptance/retirement deltas require full --update".to_string());
        }
        let item_kind = args
            .next()
            .filter(|value| !value.starts_with("--"))
            .ok_or_else(|| format!("{arg} requires an exact item kind"))?;
        let item_kind: inv::ItemKind =
            serde_json::from_value(serde_json::Value::String(item_kind.clone()))
                .map_err(|error| format!("invalid item kind for {arg}: {error}"))?;
        let identity = args
            .next()
            .filter(|value| !value.starts_with("--"))
            .ok_or_else(|| format!("{arg} requires an exact source identity"))?;
        let reference = args
            .next()
            .filter(|value| !value.starts_with("--"))
            .ok_or_else(|| format!("{arg} {identity} requires an ADR reference"))?;
        let (relative, anchor) = reference
            .split_once('#')
            .ok_or_else(|| format!("ADR reference needs a specific section: {reference}"))?;
        let relative_path = Path::new(relative);
        if anchor.is_empty()
            || !relative_path.starts_with("docs/adr")
            || relative_path
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
            || relative_path.extension().and_then(|value| value.to_str()) != Some("md")
            || !repo_root.join(relative_path).is_file()
        {
            return Err(format!(
                "acceptance reference must name an existing repository ADR section: {reference}"
            ));
        }
        let decision = std::fs::read_to_string(repo_root.join(relative_path))
            .map_err(|error| format!("reading source disposition ADR: {error}"))?;
        let status = markdown_section(&decision, "status")
            .or_else(|| markdown_section(&decision, "状态"))
            .ok_or_else(|| {
                format!("source disposition ADR requires an approval Status: {reference}")
            })?;
        if !matches!(
            status.trim(),
            "accepted" | "approved" | "已批准" | "已采纳" | "已确认"
        ) {
            return Err(format!(
                "source disposition ADR is not approved: {reference}"
            ));
        }
        let section = markdown_section(&decision, anchor)
            .ok_or_else(|| format!("source disposition ADR section is absent: {reference}"))?;
        let fields = [
            ("Action", kind.to_string()),
            (
                "Kind",
                serde_json::to_value(item_kind)
                    .map_err(|error| format!("serializing item kind: {error}"))?
                    .as_str()
                    .ok_or("item kind is not a string")?
                    .to_string(),
            ),
            ("Source", identity.to_string()),
        ];
        for (field, expected) in fields {
            let values = decision_field(&section, field);
            if values.len() != 1 || values.first().copied() != Some(expected.as_str()) {
                return Err(format!(
                    "ADR disposition must state exact {field}: {expected}: {reference}"
                ));
            }
        }
        for field in ["Reason", "Compatibility", "Rollback"] {
            let values = decision_field(&section, field);
            if values.len() != 1 || values.first().is_none_or(|value| value.is_empty()) {
                return Err(format!(
                    "ADR disposition requires one nonempty {field}: {reference}"
                ));
            }
        }
        if kind == "retire" {
            let replacements = decision_field(&section, "Replacement");
            if replacements.len() != 1 || replacements.first().is_none_or(|value| value.is_empty())
            {
                return Err(format!(
                    "retirement requires a replacement or explicit unsupported disposition: {reference}"
                ));
            }
        }
        let target = if kind == "accept" {
            &mut additions
        } else {
            &mut retirements
        };
        if !target.insert((identity.to_string(), item_kind)) {
            return Err(format!("duplicate {kind} delta: {identity}"));
        }
        use std::io::Write as _;
        writeln!(
            std::io::stderr(),
            "explicit source {kind}: {identity} ({item_kind:?}); decision {reference}"
        )
        .map_err(|error| format!("recording disposition decision: {error}"))?;
    }
    Ok((additions, retirements))
}

fn wire_candidate_requested(args: &[String]) -> Result<bool, &'static str> {
    let partial = args.iter().any(|arg| arg == "--wire-candidate-update");
    if partial
        && args
            .iter()
            .any(|arg| matches!(arg.as_str(), "--update" | "--check" | "--telemetry-check"))
    {
        return Err("--wire-candidate-update cannot be combined with full generation/check modes");
    }
    Ok(partial)
}

fn write_artifacts(repo_root: &Path, artifacts: &[(PathBuf, String)]) -> Result<(), String> {
    for (path, content) in artifacts {
        let parent = path.parent().unwrap_or(Path::new("."));
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("creating {}: {error}", parent.display()))?;
        std::fs::write(path, content)
            .map_err(|error| format!("writing {}: {error}", rel(repo_root, path)))?;
        eprintln!("updated: {}", rel(repo_root, path));
    }
    Ok(())
}

/// Validate the exact staged wire artifacts using the same protocol/schema
/// source as full generation. Semantic invalid fixtures remain covered by the
/// DTO contract tests; schema validation alone cannot prove their behavior.
fn validate_extension_candidate(
    repo_root: &Path,
    artifacts: &[(PathBuf, String)],
) -> Result<(), String> {
    let path = repo_root.join("contracts/sdk/schema/echo-agent-extension-v1.schema.json");
    let text = artifacts
        .iter()
        .find(|(candidate, _)| candidate == &path)
        .map(|(_, content)| content)
        .ok_or("generated extension schema is missing")?;
    let document: serde_json::Value =
        serde_json::from_str(text).map_err(|error| error.to_string())?;
    let problems = echo_sdk_protocol::schema::validate_schema_boundaries(&document);
    if !problems.is_empty() {
        return Err(format!(
            "schema boundary violations: {}",
            problems.join(", ")
        ));
    }
    echo_sdk_protocol::schema::build_extension_validator(&document)
        .map_err(|error| error.to_string())?;
    let definitions = document
        .get("definitions")
        .ok_or("schema definitions missing")?;
    for fixture in echo_sdk_protocol::schema::all_fixtures() {
        // MethodCatalog fixtures exercise namespace admission rather than a
        // DTO definition. Validate that exact boundary instead of inventing
        // a schema or skipping unknown fixture targets.
        if fixture.target == "MethodCatalog" {
            let method = fixture
                .payload
                .get("method")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| format!("catalog fixture {} has no method", fixture.name))?;
            let accepted = echo_sdk_protocol::catalog::METHOD_CATALOG
                .iter()
                .any(|entry| entry.name == method);
            if accepted != (fixture.kind == echo_sdk_protocol::schema::FixtureKind::Valid)
                || (fixture.kind == echo_sdk_protocol::schema::FixtureKind::Invalid
                    && fixture.expect_error.as_deref() != Some("invalid_request"))
            {
                return Err(format!(
                    "catalog fixture {} violates namespace admission",
                    fixture.name
                ));
            }
            continue;
        }
        if definitions.get(&fixture.target).is_none() {
            return Err(format!(
                "fixture {} targets missing schema {}",
                fixture.name, fixture.target
            ));
        }
        if fixture.kind == echo_sdk_protocol::schema::FixtureKind::Valid {
            let target = serde_json::json!({"$schema":"http://json-schema.org/draft-07/schema#",
                "$ref":format!("#/definitions/{}",fixture.target), "definitions":definitions});
            let validator = echo_sdk_protocol::schema::build_extension_validator(&target)
                .map_err(|error| error.to_string())?;
            validator.validate(&fixture.payload).map_err(|error| {
                format!("valid fixture {} violates schema: {error}", fixture.name)
            })?;
        } else if fixture
            .expect_error
            .as_deref()
            .is_none_or(|code| !matches!(code, "invalid_value" | "invalid_request"))
        {
            return Err(format!(
                "invalid fixture {} has no accepted failure code",
                fixture.name
            ));
        }
    }
    Ok(())
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

/// The workspace root is the ancestor of this crate that carries
/// `contracts/sdk/toolchain.json`.
fn locate_repo_root() -> Option<PathBuf> {
    let mut current: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    loop {
        if current.join(TOOLCHAIN_JSON).is_file() {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
}

struct RustdocToolchain {
    name: String,
}

fn load_rustdoc_toolchain(repo_root: &Path) -> Result<RustdocToolchain, String> {
    let path = repo_root.join(TOOLCHAIN_JSON);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let doc: serde_json::Value =
        serde_json::from_str(&raw).map_err(|e| format!("parsing {}: {e}", path.display()))?;
    let name = doc
        .pointer("/rustdoc/toolchain")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("{} missing rustdoc.toolchain", path.display()))?;
    if name.trim().is_empty() {
        return Err(format!("{} has empty rustdoc.toolchain", path.display()));
    }
    let format_version = doc
        .pointer("/rustdoc/rustdoc_json_format_version")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| {
            format!(
                "{} missing rustdoc.rustdoc_json_format_version",
                path.display()
            )
        })?;
    if format_version != RUSTDOC_FORMAT_VERSION {
        return Err(format!(
            "{} declares rustdoc format {format_version}, but the parser requires {RUSTDOC_FORMAT_VERSION}",
            path.display()
        ));
    }
    Ok(RustdocToolchain {
        name: name.to_string(),
    })
}

/// Leaf features of the root `echo_agent` package via `cargo metadata`
/// (everything except `default` and `full`), sorted deterministically.
fn leaf_features_of_root_crate(repo_root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--locked")
        .current_dir(repo_root)
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("running cargo metadata: {e}"))?;
    if !output.status.success() {
        return Err("cargo metadata failed".to_string());
    }
    let doc: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("parsing cargo metadata output: {e}"))?;
    let packages = doc
        .get("packages")
        .and_then(|v| v.as_array())
        .ok_or("cargo metadata output missing packages")?;
    for package in packages {
        if package.get("name").and_then(|v| v.as_str()) == Some("echo_agent") {
            let features = package
                .get("features")
                .and_then(|v| v.as_object())
                .ok_or("echo_agent package missing features table")?;
            let mut leaves: Vec<String> = features
                .keys()
                .filter(|name| name.as_str() != "default" && name.as_str() != "full")
                .map(|name| name.to_string())
                .collect();
            leaves.sort();
            return Ok(leaves);
        }
    }
    Err("echo_agent package not found in cargo metadata".to_string())
}

/// Read the exact framework package provenance from Cargo's resolved graph so
/// the telemetry report cannot silently claim a different revision than the
/// Host actually builds.
fn framework_dependency_source(repo_root: &Path) -> Result<String, String> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--locked")
        .current_dir(repo_root)
        .output()
        .map_err(|error| format!("running cargo metadata for framework provenance: {error}"))?;
    if !output.status.success() {
        return Err("cargo metadata failed while reading framework provenance".to_string());
    }
    let document: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("parsing framework provenance metadata: {error}"))?;
    document
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .find(|package| {
            package.get("name").and_then(serde_json::Value::as_str) == Some("echo_agent")
        })
        .and_then(|package| package.get("source"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| "cargo metadata did not expose a Git source for echo_agent".to_string())
}

fn framework_manifest_path(repo_root: &Path) -> Result<PathBuf, String> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--locked")
        .current_dir(repo_root)
        .output()
        .map_err(|error| format!("running cargo metadata for framework manifest: {error}"))?;
    if !output.status.success() {
        return Err("cargo metadata failed while locating framework manifest".to_string());
    }
    let document: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("parsing framework manifest metadata: {error}"))?;
    document
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .find(|package| {
            package.get("name").and_then(serde_json::Value::as_str) == Some("echo_agent")
        })
        .and_then(|package| package.get("manifest_path"))
        .and_then(serde_json::Value::as_str)
        .map(PathBuf::from)
        .ok_or_else(|| "cargo metadata did not expose echo_agent manifest_path".to_string())
}

#[derive(Debug, Clone)]
struct WorkspaceRustdocPackage {
    package_name: String,
    target_name: String,
    features: Vec<String>,
    manifest_path: PathBuf,
}

struct RustdocRequest<'a> {
    package_name: &'a str,
    target_name: &'a str,
    features: &'a [String],
    profile_name: &'a str,
    external_manifest: Option<&'a Path>,
}

fn generate_profile(
    repo_root: &Path,
    toolchain: &RustdocToolchain,
    profile: &FeatureProfile,
    run_identity: &str,
    dependency_cache: &mut BTreeMap<(String, Vec<String>), String>,
) -> Result<Vec<PublicItem>, String> {
    let framework_manifest = framework_manifest_path(repo_root)?;
    let framework_root = framework_manifest
        .parent()
        .ok_or("framework manifest has no parent directory")?;
    let root_json = generate_rustdoc_json(
        repo_root,
        toolchain,
        run_identity,
        RustdocRequest {
            package_name: "echo_agent",
            target_name: "echo_agent",
            features: &profile.features,
            profile_name: &profile.name,
            external_manifest: Some(framework_manifest.as_path()),
        },
    )?;
    let root_json = attach_macro_behavior_digests(framework_root, "echo_agent", &root_json)?;
    let packages = resolved_workspace_dependencies(repo_root, profile)?;
    let mut dependencies = BTreeMap::new();
    for package in packages {
        let cache_key = (package.package_name.clone(), package.features.clone());
        let json = match dependency_cache.get(&cache_key) {
            Some(json) => json.clone(),
            None => {
                let generated = generate_rustdoc_json(
                    repo_root,
                    toolchain,
                    run_identity,
                    RustdocRequest {
                        package_name: &package.package_name,
                        target_name: &package.target_name,
                        features: &package.features,
                        profile_name: &profile.name,
                        external_manifest: Some(package.manifest_path.as_path()),
                    },
                )?;
                let package_root = package.manifest_path.parent().ok_or_else(|| {
                    format!("package {} manifest has no parent", package.package_name)
                })?;
                let generated =
                    attach_macro_behavior_digests(package_root, &package.target_name, &generated)?;
                dependency_cache.insert(cache_key, generated.clone());
                generated
            }
        };
        dependencies.insert(package.target_name, json);
    }
    inv::extract_public_items_with_dependencies(&root_json, &dependencies)
        .map_err(|error: InventoryError| error.to_string())
}

fn attach_macro_behavior_digests(
    package_root: &Path,
    target_name: &str,
    json: &str,
) -> Result<String, String> {
    let proc_macro_digest = if target_name == "echo_macros" {
        let mut files = Vec::new();
        collect_rust_sources(&package_root.join("src"), &mut files)?;
        files.push(package_root.join("Cargo.toml"));
        Some(source_files_digest(package_root, &files)?)
    } else {
        None
    };
    let mut document: serde_json::Value = serde_json::from_str(json)
        .map_err(|error| format!("parsing {target_name} rustdoc JSON: {error}"))?;
    let existing_ids: std::collections::BTreeSet<String> = {
        let index = document
            .get_mut("index")
            .and_then(serde_json::Value::as_object_mut)
            .ok_or_else(|| format!("{target_name} rustdoc JSON missing index"))?;
        for item in index.values_mut() {
            let is_proc_macro = item.pointer("/inner/proc_macro").is_some();
            let is_declarative_macro = item.pointer("/inner/macro").is_some();
            let digest = if is_proc_macro {
                proc_macro_digest.clone()
            } else if is_declarative_macro {
                item.pointer("/span/filename")
                    .and_then(serde_json::Value::as_str)
                    .map(|filename| {
                        let path = Path::new(filename);
                        if path.is_absolute() {
                            path.to_path_buf()
                        } else {
                            package_root.join(path)
                        }
                    })
                    .map(|path| source_files_digest(package_root, &[path]))
                    .transpose()?
            } else {
                None
            };
            if let Some(digest) = digest
                && let Some(attrs) = item
                    .get_mut("attrs")
                    .and_then(serde_json::Value::as_array_mut)
            {
                attrs.push(serde_json::json!({
                    "other": format!("#[echo_sdk_behavior_digest = \"{digest}\"]")
                }));
            }
        }
        index.keys().cloned().collect()
    };
    if let Some(digest) = proc_macro_digest {
        let missing_derives: Vec<(String, String)> = document
            .get("paths")
            .and_then(serde_json::Value::as_object)
            .into_iter()
            .flatten()
            .filter_map(|(id, summary)| {
                let is_derive =
                    summary.get("kind").and_then(serde_json::Value::as_str) == Some("proc_derive");
                let name = summary
                    .get("path")
                    .and_then(serde_json::Value::as_array)
                    .and_then(|path| path.last())
                    .and_then(serde_json::Value::as_str);
                (is_derive && !existing_ids.contains(id))
                    .then(|| name.map(|name| (id.clone(), name.to_string())))
                    .flatten()
            })
            .collect();
        let index = document
            .get_mut("index")
            .and_then(serde_json::Value::as_object_mut)
            .ok_or("echo_macros rustdoc JSON missing index")?;
        for (id, name) in missing_derives {
            let numeric_id = id.parse::<u64>().unwrap_or_default();
            index.insert(
                id,
                serde_json::json!({
                    "id": numeric_id,
                    "crate_id": 0,
                    "name": name,
                    "visibility": "public",
                    "attrs": [{
                        "other": format!("#[echo_sdk_behavior_digest = \"{digest}\"]")
                    }],
                    "inner": {"proc_macro": {"kind": "derive", "helpers": []}}
                }),
            );
        }
    }
    serde_json::to_string(&document)
        .map_err(|error| format!("serializing augmented {target_name} rustdoc JSON: {error}"))
}

fn collect_rust_sources(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(directory)
        .map_err(|error| format!("reading {}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("reading source entry: {error}"))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("reading type for {}: {error}", path.display()))?;
        if file_type.is_dir() {
            collect_rust_sources(&path, files)?;
        } else if file_type.is_file() && path.extension().is_some_and(|extension| extension == "rs")
        {
            files.push(path);
        }
    }
    Ok(())
}

fn source_files_digest(root: &Path, files: &[PathBuf]) -> Result<String, String> {
    let mut sorted = files.to_vec();
    sorted.sort();
    let mut sources = Vec::new();
    for path in sorted {
        let content = std::fs::read_to_string(&path)
            .map_err(|error| format!("reading {}: {error}", path.display()))?;
        sources.push(serde_json::json!({
            "path": rel(root, &path),
            "content": content,
        }));
    }
    Ok(echo_sdk_protocol::schema::digest_of(
        &serde_json::Value::Array(sources),
    ))
}

fn cargo_target_dir(repo_root: &Path, override_directory: Option<&std::ffi::OsStr>) -> PathBuf {
    match override_directory.map(Path::new) {
        Some(directory) if directory.is_absolute() => directory.to_path_buf(),
        Some(directory) => repo_root.join(directory),
        None => repo_root.join("target"),
    }
}

fn rustdoc_command(
    repo_root: &Path,
    toolchain: &RustdocToolchain,
    run_identity: &str,
    request: &RustdocRequest<'_>,
    target_dir: &Path,
) -> Command {
    let working_dir = request
        .external_manifest
        .map(workspace_root_for_manifest)
        .unwrap_or_else(|| repo_root.to_path_buf());
    let mut command = Command::new("cargo");
    command
        .env("RUSTUP_TOOLCHAIN", &toolchain.name)
        .env("CARGO_INCREMENTAL", "0")
        .arg("rustdoc")
        .arg("--lib")
        .arg("--locked")
        .arg("--no-default-features");
    command.arg("--target-dir").arg(target_dir);
    if let Some(manifest_path) = request.external_manifest {
        command.arg("--manifest-path").arg(manifest_path);
    } else {
        command.arg("-p").arg(request.package_name);
    }
    if !request.features.is_empty() {
        command.arg("--features").arg(request.features.join(","));
    }
    let profile_digest = echo_sdk_protocol::schema::digest_of(&serde_json::json!({
        "package": request.package_name,
        "profile": request.profile_name,
        "features": request.features,
    }));
    let profile_cfg: String = profile_digest
        .chars()
        .filter(|character| character.is_ascii_hexdigit())
        .take(16)
        .collect();
    command
        .arg("--")
        .arg("-Zunstable-options")
        .arg("--output-format")
        .arg("json")
        .arg("--cfg")
        .arg(format!("echo_sdk_contract_run_{run_identity}"))
        .arg("--cfg")
        .arg(format!("echo_sdk_contract_profile_{profile_cfg}"))
        .current_dir(&working_dir);
    command
}

fn generate_rustdoc_json(
    repo_root: &Path,
    toolchain: &RustdocToolchain,
    run_identity: &str,
    request: RustdocRequest<'_>,
) -> Result<String, String> {
    // Resolve relative Cargo overrides against this SDK root before changing
    // cwd to an external framework workspace. Build and read share this path.
    let target_dir = cargo_target_dir(repo_root, std::env::var_os("CARGO_TARGET_DIR").as_deref());
    let mut command = rustdoc_command(repo_root, toolchain, run_identity, &request, &target_dir);
    let output = command.output().map_err(|error| {
        format!(
            "spawning cargo rustdoc for {}: {error}",
            request.package_name
        )
    })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "cargo rustdoc failed for {} profile {} (toolchain {}).\n\
             Install it with: rustup toolchain install {}\n\
             stderr tail: {}",
            request.package_name,
            request.profile_name,
            toolchain.name,
            toolchain.name,
            tail_chars(&stderr, 2000)
        ));
    }
    read_rustdoc_json(&target_dir, request.target_name)
}

fn read_rustdoc_json(target_dir: &Path, target_name: &str) -> Result<String, String> {
    let path = target_dir.join("doc").join(format!("{target_name}.json"));
    std::fs::read_to_string(&path).map_err(|error| format!("reading {}: {error}", path.display()))
}

fn workspace_root_for_manifest(manifest: &Path) -> PathBuf {
    let mut current = manifest.parent().unwrap_or(Path::new("."));
    loop {
        let cargo_toml = current.join("Cargo.toml");
        let has_workspace = std::fs::read_to_string(&cargo_toml)
            .ok()
            .is_some_and(|contents| contents.lines().any(|line| line.trim() == "[workspace]"));
        if has_workspace {
            return current.to_path_buf();
        }
        let Some(parent) = current.parent() else {
            return current.to_path_buf();
        };
        current = parent;
    }
}

fn resolved_workspace_dependencies(
    repo_root: &Path,
    profile: &FeatureProfile,
) -> Result<Vec<WorkspaceRustdocPackage>, String> {
    let framework_manifest = framework_manifest_path(repo_root)?;
    let mut command = Command::new("cargo");
    command
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--locked")
        .arg("--no-default-features")
        .arg("--manifest-path")
        .arg(&framework_manifest)
        .current_dir(repo_root);
    if !profile.features.is_empty() {
        command.arg("--features").arg(profile.features.join(","));
    }
    let output = command
        .output()
        .map_err(|error| format!("running cargo metadata for {}: {error}", profile.name))?;
    if !output.status.success() {
        return Err(format!("cargo metadata failed for {}", profile.name));
    }
    let document: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("parsing cargo metadata for {}: {error}", profile.name))?;
    let packages = document
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or("cargo metadata packages is missing")?;
    let root = document
        .pointer("/resolve/root")
        .and_then(serde_json::Value::as_str)
        .or_else(|| {
            packages
                .iter()
                .find(|package| {
                    package.get("name").and_then(serde_json::Value::as_str) == Some("echo_agent")
                })
                .and_then(|package| package.get("id"))
                .and_then(serde_json::Value::as_str)
        })
        .ok_or("cargo metadata has neither a workspace root nor echo_agent package")?;
    let nodes = document
        .pointer("/resolve/nodes")
        .and_then(serde_json::Value::as_array)
        .ok_or("cargo metadata resolve.nodes is missing")?;
    let node_by_id: BTreeMap<&str, &serde_json::Value> = nodes
        .iter()
        .filter_map(|node| {
            node.get("id")
                .and_then(serde_json::Value::as_str)
                .map(|id| (id, node))
        })
        .collect();
    let mut reachable = BTreeMap::<String, Vec<String>>::new();
    let mut pending = vec![root.to_string()];
    while let Some(id) = pending.pop() {
        if reachable.contains_key(&id) {
            continue;
        }
        let Some(node) = node_by_id.get(id.as_str()) else {
            continue;
        };
        let mut features: Vec<String> = node
            .get("features")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_str)
            .map(str::to_string)
            .collect();
        features.sort();
        reachable.insert(id.clone(), features);
        for dependency in node
            .get("deps")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(package_id) = dependency.get("pkg").and_then(serde_json::Value::as_str) {
                pending.push(package_id.to_string());
            }
        }
    }

    let mut result = Vec::new();
    for package in packages {
        let Some(id) = package.get("id").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let Some(features) = reachable.get(id) else {
            continue;
        };
        let Some(package_name) = package.get("name").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let registry_source = package
            .get("source")
            .is_some_and(|source| !source.is_null());
        if registry_source && package_name != "tokio-util" {
            continue;
        }
        if matches!(
            package_name,
            "echo_agent" | "echo-sdk-protocol" | "echo-agent-learning"
        ) {
            continue;
        }
        let target_name = package
            .get("targets")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .find(|target| {
                target
                    .get("kind")
                    .and_then(serde_json::Value::as_array)
                    .is_some_and(|kinds| {
                        kinds
                            .iter()
                            .any(|kind| matches!(kind.as_str(), Some("lib" | "proc-macro")))
                    })
            })
            .and_then(|target| target.get("name"))
            .and_then(serde_json::Value::as_str);
        let Some(target_name) = target_name else {
            continue;
        };
        let manifest_path = package
            .get("manifest_path")
            .and_then(serde_json::Value::as_str)
            .map(PathBuf::from)
            .ok_or_else(|| format!("package {package_name} missing manifest_path"))?;
        result.push(WorkspaceRustdocPackage {
            package_name: package_name.to_string(),
            target_name: target_name.to_string(),
            features: features.clone(),
            manifest_path,
        });
    }
    result.sort_by(|left, right| left.package_name.cmp(&right.package_name));
    Ok(result)
}

fn generate_all_profiles(
    repo_root: &Path,
    toolchain: &RustdocToolchain,
    profiles: &[FeatureProfile],
) -> Result<BTreeMap<String, Vec<PublicItem>>, String> {
    let mut per_profile = BTreeMap::new();
    let run_identity = format!(
        "{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    );
    let mut dependency_cache = BTreeMap::new();
    for profile in profiles {
        eprintln!("rustdoc profile {} ...", profile.name);
        let items = generate_profile(
            repo_root,
            toolchain,
            profile,
            &run_identity,
            &mut dependency_cache,
        )?;
        eprintln!(
            "rustdoc profile {}: {} public items",
            profile.name,
            items.len()
        );
        per_profile.insert(profile.name.clone(), items);
    }
    verify_profile_invariants(&per_profile)?;
    Ok(per_profile)
}

/// Deterministic-output guard: mathematically, `default` (no features) is a
/// subset of every profile and `full` (every feature) is a superset of all of
/// them. Violations mean the rustdoc toolchain returned stale or degraded
/// JSON for some profile (observed once as a caching anomaly during rapid
/// feature flips) — fail closed instead of freezing a broken inventory.
fn verify_profile_invariants(
    per_profile: &BTreeMap<String, Vec<PublicItem>>,
) -> Result<(), String> {
    let set_of = |name: &str| -> Option<std::collections::BTreeSet<String>> {
        per_profile
            .get(name)
            .map(|items| items.iter().map(|i| i.path.clone()).collect())
    };
    let default_set = set_of("default").ok_or("default profile missing")?;
    let full_set = set_of("full").ok_or("full profile missing")?;
    for (profile, items) in per_profile {
        let paths: std::collections::BTreeSet<String> =
            items.iter().map(|i| i.path.clone()).collect();
        let missing_default: Vec<&String> = default_set.difference(&paths).take(5).collect();
        if !missing_default.is_empty() {
            return Err(format!(
                "profile {profile} is missing {}/{} items present in the default \
                 profile (e.g. {missing_default:?}); rustdoc returned degraded output",
                default_set.difference(&paths).count(),
                default_set.len()
            ));
        }
        let missing_from_full: Vec<&String> = paths.difference(&full_set).take(5).collect();
        if !missing_from_full.is_empty() {
            return Err(format!(
                "profile {profile} has {}/{} items absent from the full profile \
                 (e.g. {missing_from_full:?}); rustdoc returned inconsistent output",
                paths.difference(&full_set).count(),
                paths.len()
            ));
        }
    }
    Ok(())
}

/// Extension schema + golden fixture artifacts, generated from the extension
/// DTOs and the method catalog (deterministic; see `schema.rs`).
fn extension_schema_artifacts(repo_root: &Path) -> Vec<(PathBuf, String)> {
    let mut artifacts = Vec::new();
    let doc = echo_sdk_protocol::schema::build_extension_schema_doc();
    artifacts.push((
        repo_root.join("contracts/sdk/schema/echo-agent-extension-v1.schema.json"),
        echo_sdk_protocol::schema::canonical_json(&doc),
    ));
    for (relative, content) in echo_sdk_protocol::schema::build_fixtures() {
        artifacts.push((repo_root.join(relative), content));
    }
    artifacts
}

fn validate_generated_schemas(
    repo_root: &Path,
    artifacts: &[(PathBuf, String)],
) -> Result<(), String> {
    let content = |relative: &str| -> Result<&str, String> {
        let expected = repo_root.join(relative);
        artifacts
            .iter()
            .find(|(path, _)| path == &expected)
            .map(|(_, content)| content.as_str())
            .ok_or_else(|| format!("generated artifact {relative} is missing"))
    };
    let manifest_schema: serde_json::Value =
        serde_json::from_str(content(PARITY_MANIFEST_SCHEMA_JSON)?)
            .map_err(|error| format!("parsing generated parity manifest schema: {error}"))?;
    let manifest: serde_json::Value = serde_json::from_str(content(PARITY_MANIFEST_JSON)?)
        .map_err(|error| format!("parsing generated parity manifest: {error}"))?;
    let validator = jsonschema::validator_for(&manifest_schema)
        .map_err(|error| format!("compiling parity manifest schema: {error}"))?;
    if let Err(error) = validator.validate(&manifest) {
        return Err(format!(
            "generated parity manifest violates its schema: {error}"
        ));
    }

    let accepted_schema: serde_json::Value =
        serde_json::from_str(content(ACCEPTED_EXTERNAL_CONTRACT_SCHEMA_JSON)?)
            .map_err(|error| format!("parsing generated accepted contract schema: {error}"))?;
    let accepted: serde_json::Value =
        serde_json::from_str(content(ACCEPTED_EXTERNAL_CONTRACT_JSON)?)
            .map_err(|error| format!("parsing generated accepted contract: {error}"))?;
    let accepted_validator = jsonschema::validator_for(&accepted_schema)
        .map_err(|error| format!("compiling accepted contract schema: {error}"))?;
    if let Err(error) = accepted_validator.validate(&accepted) {
        return Err(format!(
            "generated accepted external contract violates its schema: {error}"
        ));
    }

    let extension_relative = "contracts/sdk/schema/echo-agent-extension-v1.schema.json";
    let extension_schema: serde_json::Value = serde_json::from_str(content(extension_relative)?)
        .map_err(|error| format!("parsing generated extension schema: {error}"))?;
    echo_sdk_protocol::schema::build_extension_validator(&extension_schema)
        .map_err(|error| format!("compiling generated extension schema: {error}"))?;
    Ok(())
}

fn validate_generated_file_set(
    repo_root: &Path,
    artifacts: &[(PathBuf, String)],
) -> Result<(), String> {
    let fixture_root = repo_root.join("contracts/sdk/fixtures/extension/v1");
    let expected: std::collections::BTreeSet<PathBuf> = artifacts
        .iter()
        .map(|(path, _)| path)
        .filter(|path| path.starts_with(&fixture_root))
        .cloned()
        .collect();
    let entries = std::fs::read_dir(&fixture_root)
        .map_err(|error| format!("reading {}: {error}", fixture_root.display()))?;
    let mut actual = std::collections::BTreeSet::new();
    for entry in entries {
        let entry = entry.map_err(|error| {
            format!(
                "reading fixture entry in {}: {error}",
                fixture_root.display()
            )
        })?;
        if entry
            .file_type()
            .map_err(|error| format!("reading fixture type: {error}"))?
            .is_file()
        {
            actual.insert(entry.path());
        }
    }
    let stale: Vec<String> = actual
        .difference(&expected)
        .map(|path| rel(repo_root, path))
        .collect();
    if stale.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "stale generated extension fixtures are not in the canonical set: {}",
            stale.join(", ")
        ))
    }
}

/// UTF-8 safe suffix of at most `n` characters for error reporting.
fn tail_chars(s: &str, n: usize) -> String {
    s.chars()
        .rev()
        .take(n)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rustdoc_target_directory_resolves_repo_relative_overrides()
    -> Result<(), Box<dyn std::error::Error>> {
        let temporary = tempfile::tempdir()?;
        let root = temporary.path().join("sdk/root");
        let shared = temporary.path().join("shared/cache");
        assert_eq!(cargo_target_dir(&root, None), root.join("target"));
        assert_eq!(cargo_target_dir(&root, Some(shared.as_os_str())), shared);
        assert_eq!(
            cargo_target_dir(&root, Some(std::ffi::OsStr::new("cache/中文😀"))),
            root.join("cache/中文😀")
        );
        Ok(())
    }

    #[test]
    fn rustdoc_command_and_artifact_read_share_target_across_external_workspaces()
    -> Result<(), Box<dyn std::error::Error>> {
        let temporary = tempfile::tempdir()?;
        let repo = temporary.path().join("sdk");
        let external = temporary.path().join("framework");
        std::fs::create_dir_all(external.join("crate"))?;
        std::fs::write(
            external.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crate\"]\n",
        )?;
        let manifest = external.join("crate/Cargo.toml");
        std::fs::write(
            &manifest,
            "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\n",
        )?;
        let toolchain = RustdocToolchain {
            name: "nightly-fixture".to_string(),
        };
        let features = vec!["lsp".to_string()];
        let shared = temporary.path().join("absolute-cache");
        for directory in [
            None,
            Some(shared.as_os_str()),
            Some(std::ffi::OsStr::new("relative-cache/中文😀")),
        ] {
            let target = cargo_target_dir(&repo, directory);
            std::fs::create_dir_all(target.join("doc"))?;
            std::fs::create_dir_all(repo.join("target/doc"))?;
            std::fs::write(repo.join("target/doc/fixture.json"), "stale-default")?;
            std::fs::write(target.join("doc/fixture.json"), "selected-current")?;
            for external_manifest in [None, Some(manifest.as_path())] {
                let request = RustdocRequest {
                    package_name: "fixture",
                    target_name: "fixture",
                    features: &features,
                    profile_name: "core",
                    external_manifest,
                };
                let command = rustdoc_command(&repo, &toolchain, "fixed-run", &request, &target);
                let arguments = command.get_args().collect::<Vec<_>>();
                let target_argument = arguments
                    .windows(2)
                    .find(|pair| {
                        pair.first()
                            .is_some_and(|value| *value == std::ffi::OsStr::new("--target-dir"))
                    })
                    .and_then(|pair| pair.get(1));
                assert_eq!(target_argument.copied(), Some(target.as_os_str()));
                assert!(target.is_absolute());
                assert_eq!(
                    command.get_current_dir(),
                    Some(if external_manifest.is_some() {
                        external.as_path()
                    } else {
                        repo.as_path()
                    })
                );
                assert_eq!(
                    read_rustdoc_json(&target, request.target_name)?,
                    "selected-current"
                );
                assert!(arguments.contains(&std::ffi::OsStr::new("--locked")));
                assert!(arguments.contains(&std::ffi::OsStr::new("-Zunstable-options")));
            }
        }
        Ok(())
    }

    #[test]
    fn partial_generation_is_separate_from_full_contract_gate() -> Result<(), String> {
        let args = |values: &[&str]| {
            values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
        };
        assert!(wire_candidate_requested(&args(&[
            "export_schema",
            "--wire-candidate-update"
        ]))?);
        assert!(!wire_candidate_requested(&args(&[
            "export_schema",
            "--check"
        ]))?);
        for full in ["--update", "--check", "--telemetry-check"] {
            assert!(
                wire_candidate_requested(&args(&[
                    "export_schema",
                    "--wire-candidate-update",
                    full
                ]))
                .is_err()
            );
        }
        let root = Path::new("/protocol-wire-candidate");
        let artifacts = extension_schema_artifacts(root);
        assert!(artifacts.iter().all(|(path, _)| path
            == &root.join("contracts/sdk/schema/echo-agent-extension-v1.schema.json")
            || path.starts_with(root.join("contracts/sdk/fixtures/extension/v1"))));
        validate_extension_candidate(root, &artifacts)?;
        let repo = locate_repo_root().ok_or("repository missing")?;
        let inventory: inv::ParityManifest = serde_json::from_slice(
            &std::fs::read(repo.join(PARITY_MANIFEST_JSON)).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let accepted =
            std::fs::read(repo.join(ACCEPTED_EXTERNAL_CONTRACT_JSON)).map_err(|e| e.to_string())?;
        let (_, before_catalog, before_source) = catalog_documents(&inventory, &accepted)?;
        let mut changed = inventory;
        let intrinsic = changed
            .entries
            .iter_mut()
            .find(|entry| entry.route.surface == "intrinsic")
            .ok_or("intrinsic sample missing")?;
        intrinsic.path.push_str("_telemetry_mutation");
        intrinsic.semantic_rule.push_str(" telemetry mutation");
        let (_, after_catalog, after_source) = catalog_documents(&changed, &accepted)?;
        assert_eq!(before_catalog, after_catalog);
        assert_eq!(before_source, after_source);
        let candidate = wire_candidate_artifacts(&repo)?;
        assert!(
            candidate
                .iter()
                .all(|(path, _)| path != &repo.join(PARITY_MANIFEST_JSON)
                    && path != &repo.join(ACCEPTED_EXTERNAL_CONTRACT_JSON)
                    && path != &repo.join(INVENTORY_TELEMETRY_JSON))
        );
        Ok(())
    }

    #[test]
    fn new_source_inventory_route_does_not_change_accepted_catalog_or_digest() -> Result<(), String>
    {
        let repo = locate_repo_root().ok_or("repository missing")?;
        let mut inventory: inv::ParityManifest = serde_json::from_slice(
            &std::fs::read(repo.join(PARITY_MANIFEST_JSON)).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let accepted =
            std::fs::read(repo.join(ACCEPTED_EXTERNAL_CONTRACT_JSON)).map_err(|e| e.to_string())?;
        let (_, before_catalog, before_source) = catalog_documents(&inventory, &accepted)?;
        let mut new = inventory
            .entries
            .iter()
            .find(|entry| entry.route.surface == "invoke")
            .cloned()
            .ok_or("invoke fixture missing")?;
        new.path = "echo_agent::FuturePublicApi::execute".to_string();
        new.source_paths = BTreeSet::from([new.path.clone()]);
        new.sdk_scope = inv::SdkScope::Deferred;
        new.route.route = "source:echo_agent::FuturePublicApi::execute".to_string();
        new.route.operation = Some(new.path.clone());
        inventory.entries.push(new);
        let (full, after_catalog, after_source) = catalog_documents(&inventory, &accepted)?;
        assert!(full.contains("FuturePublicApi"));
        assert_eq!(before_catalog, after_catalog);
        assert_eq!(before_source, after_source);
        Ok(())
    }

    #[test]
    fn acceptance_dispositions_reject_draft_wrong_action_and_identity_substrings()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = tempfile::tempdir()?;
        std::fs::create_dir_all(root.path().join("docs/adr"))?;
        let identity = "echo_orchestration::tasks::runtime::TaskExecution::result";
        let args = [
            "export_schema",
            "--update",
            "--accept-source",
            "struct_field",
            identity,
            "docs/adr/decision.md#task-result",
        ]
        .map(str::to_string);
        let draft = format!(
            "# Decision\n\n## Status\n\ndraft\n\n## Task result\n\n- Action: accept\n- Kind: struct_field\n- Source: {identity}\n- Reason: preserve canonical output\n- Compatibility: existing Task result field\n- Rollback: restore prior contract with prior framework revision\n"
        );
        std::fs::write(root.path().join("docs/adr/decision.md"), &draft)?;
        assert!(
            acceptance_delta(&args, root.path(), true).is_err(),
            "draft is not an acceptance decision"
        );
        let approved = draft.replace("\ndraft\n", "\naccepted\n");
        std::fs::write(root.path().join("docs/adr/decision.md"), &approved)?;
        assert!(acceptance_delta(&args, root.path(), true).is_ok());
        let mut retirement = args.clone();
        *retirement.get_mut(2).ok_or("argument missing")? = "--retire-source".to_string();
        assert!(
            acceptance_delta(&retirement, root.path(), true).is_err(),
            "accept decision cannot retire a source"
        );
        std::fs::write(
            root.path().join("docs/adr/decision.md"),
            approved.replace(identity, &format!("{identity}_cache")),
        )?;
        assert!(
            acceptance_delta(&args, root.path(), true).is_err(),
            "source identities must match exactly"
        );
        Ok(())
    }

    #[test]
    fn accepted_catalog_ignores_reexport_alias_inventory_growth()
    -> Result<(), Box<dyn std::error::Error>> {
        let repo = locate_repo_root().ok_or("repository missing")?;
        let mut inventory: inv::ParityManifest =
            serde_json::from_slice(&std::fs::read(repo.join(PARITY_MANIFEST_JSON))?)?;
        let accepted = std::fs::read(repo.join(ACCEPTED_EXTERNAL_CONTRACT_JSON))?;
        let (_, before_catalog, before_source) = catalog_documents(&inventory, &accepted)?;
        let mut alias = inventory
            .entries
            .iter()
            .find(|entry| {
                entry.canonical
                    && entry.sdk_scope == inv::SdkScope::ExternalContract
                    && entry.route.surface == "invoke"
            })
            .cloned()
            .ok_or("accepted invoke missing")?;
        let canonical = alias.path.clone();
        alias.path = format!("echo_agent::governance_alias::{canonical}");
        alias.canonical = false;
        alias.alias_of = Some(canonical);
        inventory.entries.push(alias);
        let (full, after_catalog, after_source) = catalog_documents(&inventory, &accepted)?;
        assert!(full.contains("total_items"));
        assert_eq!(
            before_catalog, after_catalog,
            "reexport growth is telemetry only"
        );
        assert_eq!(before_source, after_source);
        Ok(())
    }

    #[test]
    fn acceptance_deltas_require_full_update_precise_kind_and_existing_adr()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = tempfile::tempdir()?;
        std::fs::create_dir_all(root.path().join("docs/adr"))?;
        std::fs::write(
            root.path().join("docs/adr/decision.md"),
            "# Decision\n\n## Status\n\naccepted\n\n## Task result\n\n- Action: accept\n- Kind: struct_field\n- Source: echo_orchestration::tasks::runtime::TaskExecution::result\n- Reason: preserve the authoritative result in Task CAS\n- Compatibility: explicit accepted result field\n- Rollback: restore prior revision and contract\n",
        )?;
        let args = [
            "export_schema",
            "--update",
            "--accept-source",
            "struct_field",
            "echo_orchestration::tasks::runtime::TaskExecution::result",
            "docs/adr/decision.md#task-result",
        ]
        .map(str::to_string);
        let (added, retired) = acceptance_delta(&args, root.path(), true)?;
        assert_eq!(added.len(), 1);
        assert!(retired.is_empty());
        assert!(acceptance_delta(&args, root.path(), false).is_err());
        let mut invalid = args.clone();
        *invalid.last_mut().ok_or("argument missing")? = "../foreign.md#task-result".to_string();
        assert!(acceptance_delta(&invalid, root.path(), true).is_err());
        let mut missing_section = args.clone();
        *missing_section.last_mut().ok_or("argument missing")? =
            "docs/adr/decision.md#absent".to_string();
        assert!(acceptance_delta(&missing_section, root.path(), true).is_err());
        let mut unrelated = args.clone();
        *unrelated.get_mut(4).ok_or("identity missing")? =
            "echo_core::UnreviewedCapability::invoke".to_string();
        assert!(acceptance_delta(&unrelated, root.path(), true).is_err());
        let mut invalid_kind = args.clone();
        *invalid_kind.get_mut(3).ok_or("kind missing")? = "not_an_item_kind".to_string();
        assert!(acceptance_delta(&invalid_kind, root.path(), true).is_err());
        let mut duplicate = args.to_vec();
        duplicate.extend(args.iter().skip(2).cloned());
        assert!(acceptance_delta(&duplicate, root.path(), true).is_err());
        let mut retire = args.clone();
        *retire.get_mut(2).ok_or("operation missing")? = "--retire-source".to_string();
        assert!(acceptance_delta(&retire, root.path(), true).is_err());
        let decision_path = root.path().join("docs/adr/decision.md");
        let retirement_decision = std::fs::read_to_string(&decision_path)?
            .replace("- Action: accept", "- Action: retire")
            + "- Replacement: explicit FeatureUnavailable for the retired operation\n";
        std::fs::write(decision_path, retirement_decision)?;
        let (added, retired) = acceptance_delta(&retire, root.path(), true)?;
        assert!(added.is_empty());
        assert_eq!(retired.len(), 1);
        Ok(())
    }

    #[test]
    fn macro_source_digest_covers_helper_modules() -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let root = directory.path();
        let entry = root.join("lib.rs");
        let helper = root.join("helper.rs");
        std::fs::write(&entry, "mod helper;\n")?;
        std::fs::write(&helper, "fn expand() -> bool { true }\n")?;
        let files = vec![entry, helper.clone()];
        let before = source_files_digest(root, &files)?;
        std::fs::write(&helper, "fn expand() -> bool { false }\n")?;
        let after = source_files_digest(root, &files)?;
        assert_ne!(before, after);
        Ok(())
    }
}
