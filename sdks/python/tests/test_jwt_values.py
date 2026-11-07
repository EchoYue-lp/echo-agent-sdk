import json
from pathlib import Path

import echo_agent_sdk as sdk


def test_a2a_jwt_exports_are_retired():
    assert not hasattr(sdk, "JwtConfig")
    assert not hasattr(sdk, "JwtClaims")
    assert sdk.EchoAgentClient


def test_retired_a2a_jwt_mappings_stay_outside_external_contract() -> None:
    root = Path(__file__).resolve().parents[3]
    manifest = json.loads((root / "contracts/sdk/parity-manifest.json").read_text())
    accepted = json.loads(
        (root / "contracts/sdk/accepted-external-contract.json").read_text()
    )
    adr = (root / "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md").read_text()
    # 承接九项已批准退出约束；83的JWT导出退出断言继续保留，不恢复旧构造。
    retired = {
        "echo_agent::a2a::auth::JwtClaims::subject": "method",
        "echo_agent::a2a::auth::JwtConfig": "struct",
        "echo_agent::a2a::auth::JwtConfig::disabled": "method",
        "echo_agent::a2a::auth::JwtConfig::hs256": "method",
        "echo_agent::a2a::auth::JwtConfig::impl<Debug>": "trait_impl",
        "echo_agent::a2a::auth::JwtConfig::is_enabled": "method",
        "echo_agent::a2a::auth::JwtConfig::rs256": "method",
        "echo_agent::a2a::auth::JwtConfig::with_audience": "method",
        "echo_agent::a2a::auth::JwtConfig::with_issuer": "method",
    }
    assert len(retired) == 9
    for source, kind in retired.items():
        assert f"- Kind: {kind}\n- Source: {source}\n" in adr, source

        def matches(entry, source=source, kind=kind):
            return entry["kind"] == kind and source in entry["source_paths"]

        assert not any(matches(entry) for entry in accepted["entries"]), source
        for entry in filter(matches, manifest["entries"]):
            assert entry["sdk_scope"] == "host_or_rust_only", entry["path"]
            mapping = entry["languages"]["python"]
            assert mapping["status"] == "not_implemented", entry["path"]
            assert mapping["contract_test"] == "sdk-parity/python/retired_a2a_lsp", (
                entry["path"]
            )
