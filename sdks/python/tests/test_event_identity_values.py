import json
from pathlib import Path
from types import SimpleNamespace

import pytest

from echo_agent_sdk import EventId, EventIdentity, StreamId


def test_event_identities_preserve_validation_and_immutable_updates() -> None:
    assert EventId.new("evt-1").as_str() == "evt-1"
    assert str(StreamId.new("stream-1")) == "stream-1"
    with pytest.raises(ValueError):
        EventId.new("  ")
    identity = (
        EventIdentity.new("stream-1", "turn-1")
        .with_run_id("run-1")
        .with_message_id("message-1")
        .with_execution_id("exec-1")
        .with_conversation_id("conversation-1")
        .with_parent_event_id("event-0")
    )
    assert identity.stream_id.as_str() == "stream-1"
    assert identity.turn_id == "turn-1"
    assert identity.parent_event_id == "event-0"
    assert EventIdentity.for_run("run-2").execution_id == "run-2"
    assert (
        EventIdentity.for_chat(
            "conversation-2", "turn-2", "message-2", "run-2"
        ).message_id
        == "message-2"
    )
    assert (
        EventIdentity.from_runtime_context(
            {"run_id": "run-3", "execution_id": "exec-3"}
        ).turn_id
        == "exec-3"
    )
    assert (
        EventIdentity.from_invocation(
            {"runtime": {"conversation_id": "conversation-3"}}
        ).conversation_id
        == "conversation-3"
    )


def test_runtime_lineage_parent_survives_mapping_object_and_invocation_projection() -> (
    None
):
    runtime = {
        "turn_id": "turn-中文🙂",
        "run_id": "run-1",
        "conversation_id": "conversation-1",
        "message_id": "message-1",
        "execution_id": "execution-1",
        "subagent_lineage": {"parent_event_id": "parent-中文🙂"},
    }
    original = {**runtime, "subagent_lineage": dict(runtime["subagent_lineage"])}
    for value in (
        runtime,
        SimpleNamespace(
            **{
                **runtime,
                "subagent_lineage": SimpleNamespace(parent_event_id="parent-中文🙂"),
            }
        ),
    ):
        identity = EventIdentity.from_runtime_context(value)
        assert identity.parent_event_id == "parent-中文🙂"
        assert (
            identity.turn_id,
            identity.run_id,
            identity.conversation_id,
            identity.message_id,
            identity.execution_id,
        ) == ("turn-中文🙂", "run-1", "conversation-1", "message-1", "execution-1")
        assert identity.stream_id.as_str()
        assert identity.with_parent_event_id("changed").parent_event_id == "changed"
        assert identity.parent_event_id == "parent-中文🙂"
        assert (
            EventIdentity.from_invocation({"runtime": value}).parent_event_id
            == "parent-中文🙂"
        )
    assert runtime == original
    for lineage in (None, {}, SimpleNamespace()):
        assert (
            EventIdentity.from_runtime_context(
                {"turn_id": "t", "subagent_lineage": lineage}
            ).parent_event_id
            is None
        )
    assert EventIdentity.from_runtime_context({"turn_id": "t"}).parent_event_id is None
    assert (
        EventIdentity.from_runtime_context(
            {**runtime, "parent_event_id": "flat"}
        ).parent_event_id
        == "flat"
    )
    assert (
        EventIdentity.from_runtime_context(
            {**runtime, "parent_event_id": None}
        ).parent_event_id
        == "parent-中文🙂"
    )
    # Preserve Python's existing empty-flat fallback and scalar-leaf conversion.
    assert (
        EventIdentity.from_runtime_context(
            {**runtime, "parent_event_id": ""}
        ).parent_event_id
        == "parent-中文🙂"
    )
    assert (
        EventIdentity.from_runtime_context(
            {"turn_id": "t", "subagent_lineage": {"parent_event_id": 7}}
        ).parent_event_id
        == "7"
    )
    for parent in ("", " "):
        with pytest.raises(ValueError, match="event_id must not be empty"):
            EventIdentity.from_runtime_context(
                {"turn_id": "t", "subagent_lineage": {"parent_event_id": parent}}
            )


def test_event_identity_mappings_are_complete() -> None:
    root = Path(__file__).resolve().parents[3]
    manifest = json.loads((root / "contracts/sdk/parity-manifest.json").read_text())
    entries = [
        entry
        for entry in manifest["entries"]
        if entry["canonical"]
        and entry["languages"]["python"]["contract_test"].endswith(
            "/event_identity_values"
        )
    ]
    assert len(entries) == 29
    assert all(entry["languages"]["python"]["status"] == "done" for entry in entries)
