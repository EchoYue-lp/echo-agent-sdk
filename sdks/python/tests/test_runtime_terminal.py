"""原生事实的SDK投影测试；合成输入不充作真实Framework/Host验收。"""

import asyncio
from types import SimpleNamespace

import pytest

from echo_agent_sdk import EchoAgentError, WireHandle, to_wire
from echo_agent_sdk.client import EchoAgentClient, RunHandle

STREAM = WireHandle("stream", "1", "stream")


def envelope(sequence, event_type, value):
    return {
        "schema_version": 4,
        "event_id": f"event-{sequence}",
        "content_hash": "sha256:" + "a" * 64,
        "sequence": str(sequence),
        "stream_id": STREAM.id,
        "turn_id": "turn",
        "execution_id": "outer-execution",
        "timestamp": {"unix_seconds": "0", "nanos": 0},
        "payload": {"event_type": event_type, "data": to_wire(value)},
    }


def status(kind):
    return {
        "execution_id": "physical-execution",
        "request_id": "request",
        "sequence": 2**64 - 1,
        "phase": "Terminal",
        "status": kind,
        "reason": None,
    }


def fixture():
    writes = []

    async def notify(method, params):
        writes.append((method, params))

    client = object.__new__(EchoAgentClient)
    client._events = {}
    client._event_queue_size = 32
    client._event_cursors = {}
    client._replay_ranges = {}
    client._closed = False
    client.connection = SimpleNamespace(ext_notification=notify)
    return client, writes


@pytest.mark.asyncio
@pytest.mark.parametrize("kind", ["settled", "cancelled", "failed"])
async def test_only_runtime_fact_finishes_after_final_body_and_ack(kind):
    client, writes = fixture()
    items = [
        envelope(1, "final_answer", ""),
        envelope(
            2,
            "assistant_message",
            {"block": {"phase": "final_answer", "complete": True, "delta": ""}},
        ),
        envelope(3, "error", {"message": "ExecutionNotSettled"}),
        envelope(4, "execution_status", status("candidate")),
        envelope(5, "assistant_message", {"block": {"delta": "尾部正文🙂"}}),
        envelope(6, "execution_status", status(kind)),
    ]
    for item in items:
        await client._publish_first_event(STREAM, item)

    async def collect():
        return [item["envelope"] async for item in client.events_for(STREAM.id, STREAM)]

    assert await asyncio.wait_for(collect(), 1) == items
    assert [row[1]["ack"]["last_processed_sequence"] for row in writes] == [
        str(n) for n in range(1, 7)
    ]


@pytest.mark.asyncio
async def test_replay_terminal_uses_envelope_cursor_and_preserves_prefix_ack():
    client, _ = fixture()
    client.events_for(STREAM.id, STREAM)
    run = RunHandle(client, WireHandle("run", "1", "run"), STREAM)
    response = {}

    async def request(_method, _params):
        return response

    client.request = request

    def page(after, item):
        return {
            "requested_after_sequence": str(after),
            "events": [item],
            "next_cursor": {
                "stream_id": STREAM.id,
                "last_processed_sequence": item["sequence"],
            },
        }

    response = page(0, envelope(1, "final_answer", ""))
    await run.replay()
    await client.notify(
        "echo_agent/event/ack",
        {"ack": {"stream": STREAM.to_dict(), "last_processed_sequence": "1"}},
    )
    assert not client._events[STREAM.id].queue._closed
    response = page(1, envelope(2, "execution_status", status("settled")))
    await run.replay("1")
    assert not client._events[STREAM.id].queue._closed
    await client.notify(
        "echo_agent/event/ack",
        {"ack": {"stream": STREAM.to_dict(), "last_processed_sequence": "2"}},
    )
    assert client._events[STREAM.id].queue._closed


@pytest.mark.asyncio
async def test_invalid_status_is_observation_error():
    client, _ = fixture()
    await client._publish_first_event(
        STREAM, envelope(1, "execution_status", {**status("settled"), "sequence": "01"})
    )
    with pytest.raises(EchoAgentError, match="malformed runtime"):
        await anext(client.events_for(STREAM.id, STREAM))


@pytest.mark.asyncio
@pytest.mark.parametrize("value", ["01", "18446744073709551616"])
async def test_invalid_nonterminal_wire_counter_is_typed_observation(value):
    client, _ = fixture()
    item = envelope(1, "execution_status", {**status("waiting"), "phase": "Thinking"})
    for entry in item["payload"]["data"]["value"]:
        if entry["key"]["value"] == "sequence":
            entry["value"] = {"kind": "u64", "value": value}
    await client._publish_first_event(STREAM, item)
    with pytest.raises(EchoAgentError) as failure:
        await anext(client.events_for(STREAM.id, STREAM))
    assert failure.value.code == "serialization_violation"
