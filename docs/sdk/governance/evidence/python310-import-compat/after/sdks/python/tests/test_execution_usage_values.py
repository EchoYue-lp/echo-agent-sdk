import json
from pathlib import Path
from typing import get_args, get_origin, get_type_hints

from typing_extensions import NotRequired

from echo_agent_sdk import ExecutionUsageValue, execution_usage_duration_millis


def test_execution_usage_duration_defaults_to_zero() -> None:
    assert execution_usage_duration_millis({}) == 0
    assert execution_usage_duration_millis({"duration_ms": "42"}) == 42
    assert execution_usage_duration_millis({"duration_ms": True}) == 0


def test_execution_usage_mapping_is_complete() -> None:
    root = Path(__file__).resolve().parents[3]
    manifest = json.loads((root / "contracts/sdk/parity-manifest.json").read_text())
    entries = [
        entry
        for entry in manifest["entries"]
        if entry["canonical"]
        and entry["languages"]["python"]["contract_test"].endswith(
            "/execution_usage_values"
        )
    ]
    assert len(entries) == 1
    assert entries[0]["languages"]["python"]["status"] == "done"


def test_execution_usage_type_hints_preserve_not_required_fields() -> None:
    hints = get_type_hints(ExecutionUsageValue, include_extras=True)
    assert set(hints) == {"duration_ms", "tokens_used", "iterations"}
    for annotation in hints.values():
        assert get_origin(annotation) is NotRequired
        assert get_args(annotation) == (int | None,)
    missing: ExecutionUsageValue = {}
    explicit_null: ExecutionUsageValue = {"duration_ms": None}
    assert missing == {}
    assert explicit_null == {"duration_ms": None}
