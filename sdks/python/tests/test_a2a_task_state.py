import json
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, ThinkingLevel
from echo_agent_sdk.catalog import FacadeCatalog


def test_retired_a2a_surface_is_absent():
    import echo_agent_sdk as sdk

    for name in ("A2AMessage", "A2ATask", "AgentCard", "TaskState"):
        assert not hasattr(sdk, name), name
    for operation in ("a2a.client.open", "a2a.task.send", "lsp.manager.open"):
        with pytest.raises(KeyError, match="absent"):
            FacadeCatalog().resolve(operation)
    assert EchoAgentClient


def test_thinking_level_parses_rust_aliases() -> None:
    assert ThinkingLevel.parse(" OFF ") is ThinkingLevel.NONE
    assert ThinkingLevel.parse("normal") is ThinkingLevel.MEDIUM
    assert ThinkingLevel.parse("xhigh") is ThinkingLevel.XHIGH
    assert ThinkingLevel.parse("unknown") is None


def test_thinking_level_mappings_are_ready() -> None:
    root = Path(__file__).resolve().parents[3]
    manifest = json.loads((root / "contracts/sdk/parity-manifest.json").read_text())
    entries = [
        entry
        for entry in manifest["entries"]
        if entry["canonical"]
        and entry["languages"]["python"]["contract_test"].endswith("/thinking_level")
    ]
    assert len(entries) == 9
    assert all(entry["languages"]["python"]["status"] == "done" for entry in entries)
