import json
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import AsyncMock

import pytest

from echo_agent_sdk import EchoAgentError, WireHandle, from_wire
from echo_agent_sdk.client import EchoAgentClient, ExtensionRegistration

CORPUS = json.loads(
    (
        Path(__file__).resolve().parents[2]
        / "shared/fixtures/native-projection-invalid.json"
    ).read_text()
)


@pytest.mark.parametrize("case", CORPUS, ids=lambda case: case["name"])
def test_native_projection_rejects_invalid(case):
    with pytest.raises(EchoAgentError) as error:
        from_wire(case["wire"])
    assert error.value.code == "serialization_violation"


def test_host_error_retains_handle():
    handle = {"id": "run-中文", "generation": "18446744073709551615", "kind": "run"}
    error = RuntimeError("failure")
    error.data = {"code": "stale_handle", "message": "stale", "handle": handle}
    assert EchoAgentError.from_exception(error).handle == WireHandle.from_dict(handle)


@pytest.mark.asyncio
async def test_facade_requires_value_envelope():
    client = object.__new__(EchoAgentClient)
    client.catalog = SimpleNamespace(
        signature=lambda _: "digest",
        family_signature=lambda *_: "digest",
        resolve=lambda _: SimpleNamespace(
            family="memory", method="_echo_agent/memory/op"
        ),
    )
    client.request = AsyncMock(return_value={})
    with pytest.raises(EchoAgentError) as error:
        await client.invoke("operation", None)
    assert error.value.code == "serialization_violation"
    with pytest.raises(EchoAgentError) as error:
        await client.family("memory", "operation", None)
    assert error.value.code == "serialization_violation"
    assert client.request.await_count == 2


@pytest.mark.asyncio
async def test_extension_close_can_retry():
    request = AsyncMock(side_effect=[RuntimeError("retry"), None])
    registration = ExtensionRegistration(
        SimpleNamespace(unregister_extension=request), WireHandle("e", "1", "extension")
    )
    with pytest.raises(RuntimeError):
        await registration.close()
    await registration.close()
    assert request.await_count == 2


@pytest.mark.asyncio
async def test_factory_origin_routes_only_valid_owned_child():
    from echo_agent_sdk.client import _Callbacks

    callbacks = _Callbacks({}, 8)
    factory = WireHandle("factory", "3", "extension")
    child = WireHandle("child", "3", "extension")
    handler = AsyncMock(
        return_value={
            "outcome": "result",
            "result": {"operation": "agent_execute", "value": "ok"},
        }
    )
    callbacks.extensions[factory.id] = handler
    callbacks.extension_registrations[factory.id] = (factory, "agent_factory")
    callbacks.factory_children[child] = factory
    payload = {
        "extension": child.to_dict(),
        "factory_origin": factory.to_dict(),
        "invocation_id": "i",
        "invocation": {"operation": "agent_execute", "input": {"task": "任务"}},
    }
    result = await callbacks.ext_method("echo_agent/extension/invoke", payload)
    assert result["outcome"] == "result"
    assert handler.await_count == 1
    for origin in [
        WireHandle("missing", "3", "extension"),
        WireHandle("factory", "4", "extension"),
    ]:
        result = await callbacks.ext_method(
            "echo_agent/extension/invoke",
            {**payload, "factory_origin": origin.to_dict()},
        )
        assert result["outcome"] == "error"
    assert handler.await_count == 1


@pytest.mark.asyncio
async def test_factory_children_keep_independent_identity_and_close_one_only():
    from echo_agent_sdk.client import _Callbacks

    callbacks = _Callbacks({}, 8)
    factory = WireHandle("factory", "3", "extension")
    first = WireHandle("child-one", "3", "extension")
    second = WireHandle("child-two", "3", "extension")

    async def handler(payload, _cancel):
        return {
            "outcome": "result",
            "result": {"operation": payload["invocation"]["operation"], "value": None},
        }

    callbacks.extensions[factory.id] = handler
    callbacks.extension_registrations[factory.id] = (factory, "agent_factory")
    for child in (first, second):
        construction = await callbacks.ext_method(
            "echo_agent/extension/invoke",
            {
                "extension": factory.to_dict(),
                "invocation_id": "create-" + child.id,
                "invocation": {
                    "operation": "factory_create_agent",
                    "input": {"instance": child.to_dict()},
                },
            },
        )
        assert construction["outcome"] == "result"
        result = await callbacks.ext_method(
            "echo_agent/extension/invoke",
            {
                "extension": child.to_dict(),
                "factory_origin": factory.to_dict(),
                "invocation_id": child.id,
                "invocation": {"operation": "agent_chat", "input": {"message": "任务"}},
            },
        )
        assert result["outcome"] == "result"
    result = await callbacks.ext_method(
        "echo_agent/extension/invoke",
        {
            "extension": first.to_dict(),
            "factory_origin": factory.to_dict(),
            "invocation_id": "close",
            "invocation": {"operation": "agent_close", "input": None},
        },
    )
    assert result["outcome"] == "result"
    assert first not in callbacks.factory_children
    assert callbacks.factory_children[second] == factory
    assert factory.id in callbacks.extensions


def test_retention_replay_gap_uses_plain_event_gap():
    from echo_agent_sdk.client import validate_replay_response

    fixture = json.loads(
        (
            Path(__file__).resolve().parents[3]
            / "contracts/sdk/fixtures/extension/v1/event-replay-gap-valid.json"
        ).read_text()
    )["payload"]
    stream = WireHandle("stream-1", "1", "stream")
    assert (
        validate_replay_response(fixture, stream, "4")["gap"]["snapshot_watermark"]
        == "9"
    )
    with pytest.raises(EchoAgentError):
        validate_replay_response(
            {
                **fixture,
                "next_cursor": {"stream_id": "wrong", "last_processed_sequence": "9"},
            },
            stream,
        )
    with pytest.raises((EchoAgentError, ValueError)):
        validate_replay_response(
            {
                **fixture,
                "next_cursor": {
                    "stream_id": stream.id,
                    "last_processed_sequence": "09",
                },
            },
            stream,
        )


@pytest.mark.asyncio
async def test_cleanup_waits_for_same_pending_construction_and_retries():
    import asyncio

    from echo_agent_sdk.client import _Callbacks

    callbacks = _Callbacks({}, 8)
    factory = WireHandle("factory", "1", "extension")
    child = WireHandle("reserved", "1", "extension")
    constructing = asyncio.Event()
    release = asyncio.Event()
    closed = []

    async def handler(payload, _cancel):
        operation = payload["invocation"]["operation"]
        if operation == "factory_create_agent":
            constructing.set()
            await release.wait()
        if operation == "agent_close":
            closed.append(payload["extension"]["id"])
        return {"outcome": "result", "result": {"operation": operation, "value": None}}

    callbacks.extensions[factory.id] = handler
    callbacks.extension_registrations[factory.id] = (factory, "agent_factory")
    create = {
        "extension": factory.to_dict(),
        "invocation_id": "create",
        "invocation": {
            "operation": "factory_create_agent",
            "input": {"instance": child.to_dict()},
        },
    }
    pending = asyncio.create_task(
        callbacks.ext_method("echo_agent/extension/invoke", create)
    )
    await constructing.wait()
    close = {
        "extension": child.to_dict(),
        "factory_origin": factory.to_dict(),
        "invocation_id": "close",
        "deadline": {"seconds": "0", "nanos": 1_000_000},
        "invocation": {"operation": "agent_close", "input": None},
    }
    timed = await callbacks.ext_method("echo_agent/extension/invoke", close)
    assert timed["error"]["code"] == "extension_timeout"
    assert child in callbacks.factory_children
    assert not closed
    release.set()
    await pending
    result = await callbacks.ext_method("echo_agent/extension/invoke", close)
    assert result["outcome"] == "result"
    duplicate = await callbacks.ext_method("echo_agent/extension/invoke", close)
    assert duplicate["outcome"] == "result"
    assert closed == [child.id]


@pytest.mark.asyncio
@pytest.mark.parametrize("capacity", [128, 256])
async def test_live_window_has_one_coalesced_gap_control_slot(capacity):
    from echo_agent_sdk.client import _AsyncQueue

    queue = _AsyncQueue(capacity)
    for index in range(capacity):
        queue.push({"envelope": {"sequence": str(index + 1)}})
    gap = {"gap": {"snapshot_watermark": str(capacity + 1)}}
    queue.push(gap, control=True)
    latest = {"gap": {"snapshot_watermark": str(capacity + 2)}}
    queue.push(latest, control=True)
    assert [await queue.get() for _ in range(capacity)] == [
        {"envelope": {"sequence": str(index + 1)}} for index in range(capacity)
    ]
    assert await queue.get() == latest
    assert not queue._items
    assert not queue._closed


@pytest.mark.asyncio
@pytest.mark.parametrize("kind", ["feature", "capability"])
async def test_generator_negotiation_validates_the_exact_sent_hello(monkeypatch, kind):
    import asyncio
    from types import SimpleNamespace

    from echo_agent_sdk import client as module
    from echo_agent_sdk.catalog import FacadeCatalog

    sent = []
    catalog = FacadeCatalog()
    capability = {
        "extension_protocol_version": 1,
        "contract_digest": catalog.contract_digest,
        "source_contract_digest": catalog.source_contract_digest,
        "features": [],
        "capabilities": [],
        "limits": {},
    }

    class Connection:
        async def initialize(self, _version, client_capabilities, **_kwargs):
            sent.append(client_capabilities.field_meta["echo_agent"])
            return SimpleNamespace(
                protocol_version=1,
                agent_capabilities=SimpleNamespace(
                    field_meta={"echo_agent": capability}
                ),
            )

    class Process:
        async def wait(self):
            await asyncio.Future()

    class Context:
        async def __aenter__(self):
            return Connection(), Process()

        async def __aexit__(self, *_args):
            return None

    monkeypatch.setattr(
        module, "spawn_agent_process", lambda *_args, **_kwargs: Context()
    )
    requested = (name for name in ["missing-b", "missing-a", "missing-b"])
    options = (
        {"required_features": requested}
        if kind == "feature"
        else {"required_capabilities": requested}
    )
    with pytest.raises(EchoAgentError) as error:
        sdk = await EchoAgentClient.spawn("fixture", **options)
        await sdk.close()
    assert error.value.code == (
        "feature_unavailable" if kind == "feature" else "extension_capability_mismatch"
    )
    assert sent[0][
        "required_features" if kind == "feature" else "required_capabilities"
    ] == ["missing-a", "missing-b"]


@pytest.mark.asyncio
@pytest.mark.parametrize("operation", ["prompt", "cancel", "initialize"])
async def test_standard_acp_keeps_official_error_but_preserves_typed_extension_data(
    monkeypatch, operation
):
    from types import SimpleNamespace

    from echo_agent_sdk import client as module
    from echo_agent_sdk.client import SessionHandle

    class OfficialError(RuntimeError):
        pass

    handle = WireHandle("e", "1", "extension")
    typed = OfficialError("typed")
    typed.data = {
        "code": "framework_error",
        "message": "typed",
        "retryable": "after_delay",
        "operation": "store.apply",
        "handle": handle.to_dict(),
        "details": {"fields": [{"key": "class", "value": "outcome_unknown"}]},
    }
    official = OfficialError("official")
    official.code = -32602
    for cause in (typed, official):
        request = AsyncMock(side_effect=cause)
        if operation == "initialize":

            class Context:
                def __init__(self, initialize):
                    self.initialize = initialize

                async def __aenter__(self):
                    return SimpleNamespace(
                        initialize=self.initialize
                    ), SimpleNamespace()

                async def __aexit__(self, *_args):
                    return None

            monkeypatch.setattr(
                module,
                "spawn_agent_process",
                lambda *_args, _request=request, **_kwargs: Context(_request),
            )
            call = EchoAgentClient.spawn("fixture")
        else:
            client = SimpleNamespace(
                connection=SimpleNamespace(prompt=request, cancel=request)
            )
            session = SessionHandle(
                client,
                WireHandle("a", "1", "agent"),
                WireHandle("s", "1", "session"),
                "acp",
                None,
            )
            call = session.prompt("text") if operation == "prompt" else session.cancel()
        with pytest.raises((EchoAgentError, OfficialError)) as caught:
            await call
        if cause is official:
            assert caught.value is official
        else:
            assert isinstance(caught.value, EchoAgentError)
            assert caught.value.code == "framework_error"
            assert caught.value.retryable == "after_delay"
            assert caught.value.operation == "store.apply"
            assert caught.value.handle == handle
            assert caught.value.details == typed.data["details"]


def test_native_domain_ids_are_maps_and_explicit_handle_fields_are_exact():
    from echo_agent_sdk import to_wire

    for value in [
        {"id": "case", "name": "case"},
        {"generation": "domain"},
        {"id": "domain", "kind": "session", "generation": "1", "title": "ordinary"},
    ]:
        assert to_wire(value)["kind"] == "map"
        assert from_wire(to_wire(value)) == value
    with pytest.raises(TypeError):
        to_wire(
            {
                "kind": "handle",
                "value": {
                    "id": "e",
                    "generation": "1",
                    "kind": "extension",
                    "title": "lost",
                },
            }
        )


@pytest.mark.asyncio
async def test_coalesced_gap_retains_original_range_and_only_covers_tail():
    from echo_agent_sdk.client import _AsyncQueue

    queue = _AsyncQueue(8)
    event = lambda sequence: {"envelope": {"sequence": str(sequence)}}
    gap = lambda sequence: {
        "gap": {
            "from_sequence": str(sequence),
            "to_sequence": str(sequence),
            "snapshot_watermark": str(sequence),
            "reason": "bounded",
        }
    }
    queue.push(event(1))
    queue.push(gap(2), control=True)
    queue.push(event(3))
    queue.push(event(5))
    queue.push(gap(4), control=True)
    queue.close()
    assert [value async for value in queue] == [
        event(1),
        {
            "gap": {
                "from_sequence": "2",
                "to_sequence": "4",
                "snapshot_watermark": "4",
                "reason": "bounded",
            }
        },
        event(5),
    ]


@pytest.mark.asyncio
async def test_observed_snapshot_ack_filters_covered_queue_without_future_ack():
    from echo_agent_sdk.client import _AsyncQueue, _EventFeed

    client = object.__new__(EchoAgentClient)
    stream = WireHandle("s", "1", "stream")
    queue = _AsyncQueue(8)
    feed = _EventFeed(queue, stream, 2, 1, 2)
    client._events = {stream.id: feed}
    client._replay_ranges = {}
    client._event_cursors = {}
    client._closed = False
    client.connection = SimpleNamespace(ext_notification=AsyncMock())
    event = lambda seq: {"stream": stream.to_dict(), "envelope": {"sequence": str(seq)}}
    gap = lambda seq: {
        "stream": stream.to_dict(),
        "gap": {
            "from_sequence": str(seq),
            "to_sequence": str(seq),
            "snapshot_watermark": str(seq),
            "reason": "bounded",
        },
    }
    queue.push(event(1))
    queue.push(gap(2), control=True)
    await client.notify(
        "echo_agent/event/ack",
        {"ack": {"stream": stream.to_dict(), "last_processed_sequence": "100"}},
    )
    assert stream not in client._event_cursors
    await client.notify(
        "echo_agent/event/ack",
        {"ack": {"stream": stream.to_dict(), "last_processed_sequence": "2"}},
    )
    queue.push(event(3))
    queue.push(gap(4), control=True)
    feed.last_sequence = 4
    feed.last_live_sequence = 3
    feed.last_gap_watermark = 4
    queue.close()
    assert [value async for value in client._iterate_events(queue, stream)] == [
        {
            "stream": stream.to_dict(),
            "gap": {
                "from_sequence": "3",
                "to_sequence": "4",
                "snapshot_watermark": "4",
                "reason": "bounded",
            },
        }
    ]


def test_explicit_record_wrapper_can_carry_domain_identity_fields():
    from echo_agent_sdk import to_wire

    value = {
        "kind": "record",
        "id": "record-id",
        "generation": "1",
        "value": {"type_id": "Record", "fields": []},
    }
    assert to_wire(value) == value
    assert from_wire(to_wire(value)) == value
