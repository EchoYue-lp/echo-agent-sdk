import json
from pathlib import Path

import pytest

from echo_agent_sdk import JwtClaims, JwtConfig


def test_jwt_config_and_claims_preserve_local_values() -> None:
    config = JwtConfig.hs256("secret").with_issuer("echo-agent").with_audience("a2a")
    assert config.is_enabled()
    assert "[redacted]" in repr(config)
    assert not JwtConfig.disabled().is_enabled()
    assert JwtClaims(sub="subject").subject() == "subject"
    with pytest.raises(ValueError, match="invalid RSA public key"):
        JwtConfig.rs256("invalid")


def test_retired_a2a_jwt_mappings_stay_outside_external_contract() -> None:
    root = Path(__file__).resolve().parents[3]
    manifest = json.loads((root / "contracts/sdk/parity-manifest.json").read_text())
    accepted = json.loads(
        (root / "contracts/sdk/accepted-external-contract.json").read_text()
    )
    adr = (root / "docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md").read_text()
    # 只迁移已批准退役的九项；通用JWT本地值正控不变。
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
