import asyncio

import pytest

from echo_agent_sdk import EchoAgentError, WireHandle
from echo_agent_sdk.client import EchoAgentClient, RunHandle, SessionHandle


def handle(identity, kind):
    return WireHandle(identity, "1", kind)


def started():
    return {
        "run": handle("run", "run").to_dict(),
        "stream": handle("stream", "stream").to_dict(),
    }


def fixture(request=None):
    async def default_request(method, _params):
        return started() if method == "echo_agent/run/start" else {}

    client = object.__new__(EchoAgentClient)
    client._events = {}
    client._updates = {}
    client._event_queue_size = 32
    client._update_queue_size = 32
    client._event_cursors = {}
    client._closed = False
    client.request = request or default_request
    return client, SessionHandle(
        client,
        handle("agent", "agent"),
        handle("session", "session"),
        "acp",
        handle("task", "task_run"),
    )


@pytest.mark.asyncio
async def test_successful_parent_close_finishes_waiting_child_and_invalidates_queries():
    _, session = fixture()
    run = await session.start_run("task")
    iterator = run.events
    next_value = asyncio.create_task(anext(iterator))
    await asyncio.sleep(0)
    await session.close()
    with pytest.raises(StopAsyncIteration):
        await asyncio.wait_for(next_value, 0.5)
    with pytest.raises(EchoAgentError, match="Run handle is closed"):
        await run.get()
    await run.close()


@pytest.mark.asyncio
async def test_late_start_reply_cannot_publish_live_child_after_parent_close():
    response = asyncio.get_running_loop().create_future()

    async def request(method, _params):
        return await response if method == "echo_agent/run/start" else {}

    client, session = fixture(request)
    starting = asyncio.create_task(session.start_run("task"))
    await asyncio.sleep(0)
    await session.close()
    response.set_result(started())
    run = await starting
    with pytest.raises(EchoAgentError, match="Run handle is closed"):
        await run.get()
    assert not session._runs, "a closed parent must not retain a late live child"
    feed = client._events.get(run.stream.id)
    assert feed is None or feed.queue._closed


@pytest.mark.asyncio
async def test_failed_parent_close_retains_child_and_allows_retry():
    attempts = 0

    async def request(method, _params):
        nonlocal attempts
        if method == "echo_agent/run/start":
            return started()
        if method == "echo_agent/session/close":
            attempts += 1
            if attempts == 1:
                raise EchoAgentError("busy", "retry close")
        return {}

    client, session = fixture(request)
    run = await session.start_run("task")
    iterator = run.events
    with pytest.raises(EchoAgentError, match="retry close"):
        await session.close()
    assert not client._events[run.stream.id].queue._closed
    await run.get()
    await session.close()
    with pytest.raises(StopAsyncIteration):
        await asyncio.wait_for(anext(iterator), 0.5)
    assert attempts == 2


@pytest.mark.asyncio
async def test_concurrent_parent_close_observers_cannot_cancel_shared_owner():
    reply = asyncio.get_running_loop().create_future()
    entered = asyncio.Event()
    calls = 0

    async def request(method, _params):
        nonlocal calls
        assert method == "echo_agent/session/close"
        calls += 1
        entered.set()
        return await reply

    _, session = fixture(request)
    first = asyncio.create_task(session.close())
    await entered.wait()
    second = asyncio.create_task(session.close())
    await asyncio.sleep(0)
    first.cancel()
    with pytest.raises(asyncio.CancelledError):
        await first
    try:
        assert not reply.cancelled(), (
            "caller cancellation reached the original Host close"
        )
        assert not second.done()
        reply.set_result({})
        await asyncio.wait_for(second, 0.5)
        assert calls == 1
    finally:
        if not reply.done():
            reply.set_result({})
        if not second.done():
            second.cancel()
        await asyncio.gather(second, return_exceptions=True)


@pytest.mark.asyncio
async def test_concurrent_run_close_observers_cannot_cancel_shared_owner():
    reply = asyncio.get_running_loop().create_future()
    entered = asyncio.Event()
    cancels = 0

    async def request(method, _params):
        nonlocal cancels
        if method == "echo_agent/run/cancel":
            cancels += 1
            return {}
        assert method == "echo_agent/run/wait"
        entered.set()
        return await reply

    client, _ = fixture(request)
    run = RunHandle(client, handle("run", "run"), handle("stream", "stream"))
    first = asyncio.create_task(run.close())
    await entered.wait()
    second = asyncio.create_task(run.close())
    await asyncio.sleep(0)
    first.cancel()
    with pytest.raises(asyncio.CancelledError):
        await first
    try:
        assert not reply.cancelled(), (
            "caller cancellation reached original run settlement"
        )
        assert not second.done()
        reply.set_result({"settled": True})
        await asyncio.wait_for(second, 0.5)
        assert cancels == 1
    finally:
        if not reply.done():
            reply.set_result({"settled": True})
        if not second.done():
            second.cancel()
        await asyncio.gather(second, return_exceptions=True)


@pytest.mark.asyncio
async def test_shutdown_in_progress_preserves_child_close_failure_and_stream():
    async def request(_method, _params):
        raise EchoAgentError("host_shutting_down", "not disposed yet")

    client, _ = fixture(request)
    run = RunHandle(client, handle("run", "run"), handle("stream", "stream"))
    iterator = run.events
    with pytest.raises(EchoAgentError, match="not disposed yet"):
        await run.close()
    assert not client._events[run.stream.id].queue._closed
    await iterator.aclose()
    client._events[run.stream.id].queue.close()


@pytest.mark.asyncio
async def test_disposal_preserves_buffered_fact_without_fabricating_terminal_or_new_ack():
    client, session = fixture()
    run = await session.start_run("task")
    iterator = run.events
    feed = client._events[run.stream.id]
    fact = {"envelope": {"sequence": "1", "payload": {"event_type": "delta"}}}
    feed.queue.push(fact)
    acks = []

    async def acknowledge(*args):
        acks.append(args)
        raise AssertionError("disposed Host cannot ACK")

    client._ack_event = acknowledge
    await session.close()
    assert await asyncio.wait_for(anext(iterator), 0.5) == fact
    with pytest.raises(StopAsyncIteration):
        await asyncio.wait_for(anext(iterator), 0.5)
    assert acks == []


@pytest.mark.asyncio
async def test_disposal_releases_ack_observer_without_cancelling_original_submission():
    client, session = fixture()
    run = await session.start_run("task")
    iterator = run.events
    feed = client._events[run.stream.id]
    feed.queue.push({"envelope": {"sequence": "1"}})
    reply = asyncio.get_running_loop().create_future()
    entered = asyncio.Event()
    completed = False

    async def acknowledge(*_args):
        nonlocal completed
        entered.set()
        await reply
        completed = True

    client._ack_event = acknowledge
    await anext(iterator)
    next_value = asyncio.create_task(anext(iterator))
    await entered.wait()
    await session.close()
    with pytest.raises(StopAsyncIteration):
        await asyncio.wait_for(next_value, 0.5)
    assert not reply.cancelled()
    assert not completed
    reply.set_result(None)
    await asyncio.sleep(0)
    assert completed


@pytest.mark.asyncio
async def test_parent_disposal_releases_child_close_observer_and_keeps_original_rpc():
    reply = asyncio.get_running_loop().create_future()
    entered = asyncio.Event()

    async def request(method, _params):
        if method == "echo_agent/run/start":
            return started()
        if method == "echo_agent/run/wait":
            entered.set()
            return await reply
        return {}

    _, session = fixture(request)
    run = await session.start_run("task")
    closing = asyncio.create_task(run.close())
    await entered.wait()
    await session.close()
    await asyncio.wait_for(closing, 0.5)
    assert not reply.done()
    reply.set_result({"settled": True})
    await asyncio.sleep(0)


@pytest.mark.asyncio
async def test_failed_run_settlement_retains_stream_for_close_retry():
    waits = 0

    async def request(method, _params):
        nonlocal waits
        if method == "echo_agent/run/wait":
            waits += 1
            return {"settled": waits > 1}
        return {}

    client, _ = fixture(request)
    run = RunHandle(client, handle("run", "run"), handle("stream", "stream"))
    iterator = run.events
    with pytest.raises(EchoAgentError, match="run did not settle"):
        await run.close()
    assert not client._events[run.stream.id].queue._closed
    await run.close()
    with pytest.raises(StopAsyncIteration):
        await asyncio.wait_for(anext(iterator), 0.5)
    assert waits == 2


@pytest.mark.asyncio
async def test_parent_disposal_includes_all_local_aliases_of_an_idempotent_run():
    reply = asyncio.get_running_loop().create_future()
    entered = asyncio.Event()

    async def request(method, params):
        if method == "echo_agent/run/start":
            assert params["idempotency_id"] == "same-key"
            return started()
        if method == "echo_agent/run/wait":
            entered.set()
            return await reply
        return {}

    _, session = fixture(request)
    first = await session.start_run("task", idempotency_id="same-key")
    second = await session.start_run("task", idempotency_id="same-key")
    assert first.wire == second.wire
    closing = asyncio.create_task(first.close())
    await entered.wait()
    try:
        await session.close()
        await asyncio.wait_for(closing, 0.5)
        assert not reply.done(), "local scope closure must not cancel original RPC"
        with pytest.raises(EchoAgentError, match="Run handle is closed"):
            await first.get()
        with pytest.raises(EchoAgentError, match="Run handle is closed"):
            await second.get()
    finally:
        if not reply.done():
            reply.set_result({"settled": True})
        await asyncio.gather(closing, return_exceptions=True)
