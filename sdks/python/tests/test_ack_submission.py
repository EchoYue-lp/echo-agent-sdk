"""Official ACP submission outcomes, separate from Host Run/journal authority.

A failed notification write has unknown peer delivery. It cannot authorize
local consumption; a later known success survives caller cancellation.
"""

import asyncio

import pytest
from acp.client.connection import ClientSideConnection

from echo_agent_sdk import EchoAgentError, WireHandle
from echo_agent_sdk.client import EchoAgentClient, _AsyncQueue, _Callbacks, _EventFeed


class Transport:
    def __init__(self):
        self.calls = []
        self.entered = asyncio.Queue()
        self.closed = False

    async def send(self, message):
        completion = asyncio.get_running_loop().create_future()
        self.calls.append((message, completion))
        self.entered.put_nowait(completion)
        await completion

    async def receive(self):
        return None

    async def close(self):
        self.closed = True


def fixture():
    stream = WireHandle("stream", "1", "stream")
    transport = Transport()
    client = object.__new__(EchoAgentClient)
    client.connection = ClientSideConnection(
        _Callbacks({}, 8), transport, listening=False
    )
    client._event_cursors = {}
    client._replay_ranges = {stream: [(0, 10)]}
    client._closed = False
    client.process = None  # This fixture owns a message transport, not a child.
    client._close_lock = asyncio.Lock()
    client._callbacks = _Callbacks({}, 8)
    client._updates = {}
    client._process_watch = None

    class Context:
        async def __aexit__(self, *_args):
            await client.connection.close()

    client._context_manager = Context()
    feed = _EventFeed(_AsyncQueue(32), stream, replay_terminal_sequence=10)
    client._events = {stream.id: feed}
    first_event = {
        "stream": stream.to_dict(),
        "envelope": {"stream_id": stream.id, "sequence": "1"},
    }
    # Populate real receive evidence through the same API on old and new sources.
    assert feed.accept_event(first_event)
    feed.queue.push(first_event)
    return client, stream, feed, transport


def ack(client, stream, seq):
    return client.notify(
        "echo_agent/event/ack",
        {"ack": {"stream": stream.to_dict(), "last_processed_sequence": str(seq)}},
    )


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "order,successful",
    [
        ((2, 10), {2}),
        ((10, 2), {2}),
        ((2, 10), {10}),
        ((10, 2), {10}),
        ((10, 2), {2, 10}),
        ((2, 10), set()),
    ],
)
async def test_success_commits_only_its_own_cursor(order, successful):
    client, stream, feed, transport = fixture()
    calls = {}
    sends = {}
    try:
        for sequence in (2, 10):
            calls[sequence] = asyncio.create_task(ack(client, stream, sequence))
            sends[sequence] = await asyncio.wait_for(transport.entered.get(), 2)
        assert stream not in client._event_cursors
        assert feed.last_sequence == 1
        assert len(feed.queue._items) == 1
        committed = 0
        for sequence in order:
            if sequence in successful:
                sends[sequence].set_result(None)
                await calls[sequence]
                committed = max(committed, sequence)
            else:
                sends[sequence].set_exception(ConnectionError("native write rejected"))
                with pytest.raises(EchoAgentError) as error:
                    await calls[sequence]
                assert error.value.details["ack_submission"] == "outcome_unknown"
            assert client._event_cursors.get(stream, 0) == committed
            assert feed.queue._closed is (committed >= 10)
        assert feed.last_sequence == max(1, committed)
        assert not feed.pending_ack_sequences
        assert not client._ack_submissions
        if not successful:
            assert (await feed.queue.get())["envelope"]["sequence"] == "1"
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_duplicate_ack_late_failure_cannot_retract_success():
    client, stream, feed, transport = fixture()
    first = asyncio.create_task(ack(client, stream, 10))
    first_send = await transport.entered.get()
    second = asyncio.create_task(ack(client, stream, 10))
    second_send = await transport.entered.get()
    second_send.set_result(None)
    await second
    first_send.set_exception(ConnectionError("older duplicate write rejected"))
    with pytest.raises(EchoAgentError):
        await first
    assert client._event_cursors[stream] == 10
    assert feed.last_sequence == 10
    assert feed.queue._closed
    await client.close()


@pytest.mark.asyncio
async def test_cancel_before_completion_preserves_unknown_until_actual_failure():
    client, stream, feed, transport = fixture()
    loop = asyncio.get_running_loop()
    previous = loop.get_exception_handler()
    failure_report = loop.create_future()
    loop.set_exception_handler(lambda _loop, value: failure_report.set_result(value))
    try:
        task = asyncio.create_task(ack(client, stream, 10))
        send = await transport.entered.get()
        task.cancel()
        with pytest.raises(asyncio.CancelledError):
            await task
        assert not send.cancelled()
        assert client._event_cursors.get(stream, 0) == 0
        assert feed.last_sequence == 1
        assert not feed.queue._closed
        assert client._ack_submissions
        send.set_exception(ConnectionError("native reject after caller cancellation"))
        report = await asyncio.wait_for(failure_report, 2)
        assert report["exception"].details["ack_submission"] == "outcome_unknown"
        assert not client._ack_submissions
        assert not feed.pending_ack_sequences
        assert client._event_cursors.get(stream, 0) == 0
        assert (await feed.queue.get())["envelope"]["sequence"] == "1"
    finally:
        loop.set_exception_handler(previous)
        await client.close()


@pytest.mark.asyncio
async def test_caller_cancel_after_send_success_cannot_erase_submission_fact():
    client, stream, feed, transport = fixture()
    task = asyncio.create_task(ack(client, stream, 10))
    send = await transport.entered.get()
    jobs = tuple(client._ack_submissions)
    send.set_result(None)
    task.cancel()
    with pytest.raises(asyncio.CancelledError):
        await task
    await asyncio.gather(*jobs)
    assert client._event_cursors[stream] == 10
    assert feed.queue._closed
    assert not feed.pending_ack_sequences
    await client.close()


@pytest.mark.asyncio
async def test_pending_coverage_allows_live_continuation_without_commit():
    client, stream, feed, transport = fixture()
    task = asyncio.create_task(ack(client, stream, 10))
    send = await transport.entered.get()
    event = lambda seq: {
        "stream": stream.to_dict(),
        "envelope": {"stream_id": stream.id, "sequence": str(seq)},
    }
    assert feed.accept_event(event(11))
    feed.queue.push(event(11))
    send.set_exception(ConnectionError("unknown completion after peer continued"))
    with pytest.raises(EchoAgentError):
        await task
    assert feed.last_sequence == 11
    assert stream not in client._event_cursors
    assert not feed.queue._closed
    assert [await feed.queue.get(), await feed.queue.get()] == [event(1), event(11)]
    assert feed.accept_event(event(12))
    await client.close()


@pytest.mark.asyncio
async def test_sdk_close_settles_pending_submission_jobs():
    client, stream, feed, transport = fixture()
    caller = asyncio.create_task(ack(client, stream, 10))
    send = await transport.entered.get()
    await client.close()
    with pytest.raises(asyncio.CancelledError):
        await caller
    assert transport.closed
    assert send.cancelled()
    assert not feed.pending_ack_sequences
    assert not client._ack_submissions
    assert stream not in client._event_cursors


@pytest.mark.asyncio
async def test_lower_pending_ack_cannot_authorize_backwards_live_fact():
    client, stream, feed, transport = fixture()
    low = asyncio.create_task(ack(client, stream, 2))
    low_send = await transport.entered.get()
    high = asyncio.create_task(ack(client, stream, 10))
    high_send = await transport.entered.get()
    event = lambda seq: {
        "stream": stream.to_dict(),
        "envelope": {"stream_id": stream.id, "sequence": str(seq)},
    }
    assert feed.accept_event(event(11))
    with pytest.raises(EchoAgentError) as error:
        feed.accept_event(event(3))
    assert error.value.code == "event_gap"
    assert feed.last_sequence == 11
    low_send.set_result(None)
    high_send.set_result(None)
    await asyncio.gather(low, high)
    assert feed.last_sequence == 11
    assert client._event_cursors[stream] == 10
    await client.close()


@pytest.mark.asyncio
async def test_ack_submission_keeps_the_validated_request_snapshot():
    client, stream, feed, transport = fixture()
    params = {"ack": {"stream": stream.to_dict(), "last_processed_sequence": "10"}}
    try:
        caller = asyncio.create_task(client.notify("echo_agent/event/ack", params))
        # Let notify register the independently owned official submission.
        await asyncio.sleep(0)
        params["ack"]["last_processed_sequence"] = "2"
        params["ack"]["stream"]["id"] = "different-stream"
        sent = await asyncio.wait_for(transport.entered.get(), 2)
        request = transport.calls[0][0]["params"]["ack"]
        sent.set_result(None)
        await caller
        assert request["last_processed_sequence"] == "10"
        assert request["stream"] == stream.to_dict()
        assert client._event_cursors[stream] == int(request["last_processed_sequence"])
        assert feed.queue._closed
    finally:
        await client.close()


def recovery_page(stream, after="1", watermark="5", end="6"):
    return {
        "requested_after_sequence": after,
        "gap": {
            "from_sequence": str(int(after) + 1),
            "to_sequence": watermark,
            "snapshot_watermark": watermark,
            "reason": "retention floor",
        },
        "events": [
            {
                "stream_id": stream.id,
                "sequence": end,
                "payload": {"event_type": "delta"},
            }
        ],
        "next_cursor": {"stream_id": stream.id, "last_processed_sequence": end},
    }


class ReplayTransport(Transport):
    def __init__(self, response):
        super().__init__()
        self.response = response
        self.incoming = asyncio.Queue()

    async def send(self, message):
        if "id" in message:
            await self.incoming.put(
                {"jsonrpc": "2.0", "id": message["id"], "result": self.response}
            )
            return
        await super().send(message)

    async def receive(self):
        return await self.incoming.get()


@pytest.mark.asyncio
async def test_retention_replay_bridge_requires_successful_official_write():
    from echo_agent_sdk.client import RunHandle

    client, stream, feed, _ = fixture()
    await client.connection.close()
    transport = ReplayTransport(recovery_page(stream))
    client.connection = ClientSideConnection(_Callbacks({}, 8), transport)
    client._replay_ranges.clear()
    feed.replay_terminal_sequence = None
    try:
        run = RunHandle(client, WireHandle("run", "1", "run"), stream)
        transport.response = {
            "requested_after_sequence": "5",
            "events": [],
            "next_cursor": {"stream_id": stream.id, "last_processed_sequence": "5"},
        }
        await run.replay("5")
        transport.response = recovery_page(stream)
        assert (await run.replay("1"))["gap"]["snapshot_watermark"] == "5"
        assert feed.last_sequence == 1
        # The retention gap connects to snapshot 5, not any absent interior fact.
        unknown = asyncio.create_task(ack(client, stream, 3))
        (await transport.entered.get()).set_result(None)
        await unknown
        assert stream not in client._event_cursors
        pending = asyncio.create_task(ack(client, stream, 6))
        send = await transport.entered.get()
        assert feed.last_sequence == 1
        assert stream not in client._event_cursors
        send.set_result(None)
        await pending
        assert client._event_cursors[stream] == 6
        assert feed.last_sequence == 6
        assert len(feed.queue._items) == 0
        assert feed.accept_event(
            {
                "stream": stream.to_dict(),
                "envelope": {"stream_id": stream.id, "sequence": "7"},
            }
        )
    finally:
        await client.close()


@pytest.mark.asyncio
@pytest.mark.parametrize("success", [False, True])
async def test_pending_replay_ack_allows_contiguous_gap_without_consumption(success):
    client, stream, feed, transport = fixture()
    feed.replay_terminal_sequence = None
    pending = asyncio.create_task(ack(client, stream, 10))
    send = await transport.entered.get()
    gap = {
        "stream": stream.to_dict(),
        "gap": {
            "from_sequence": "11",
            "to_sequence": "12",
            "snapshot_watermark": "12",
            "reason": "oversized event",
        },
    }
    try:
        assert feed.accept_gap(gap)
        feed.queue.push(gap, control=True)
        assert feed.last_sequence == 12
        assert stream not in client._event_cursors
        assert not feed.queue._closed
        if success:
            send.set_result(None)
            await pending
            assert client._event_cursors[stream] == 10
        else:
            send.set_exception(ConnectionError("outcome unknown after peer gap"))
            with pytest.raises(EchoAgentError):
                await pending
            assert stream not in client._event_cursors
            assert (await feed.queue.get())["envelope"]["sequence"] == "1"
        assert not feed.queue._closed
        assert (await feed.queue.get())["gap"]["snapshot_watermark"] == "12"
        assert feed.accept_event(
            {
                "stream": stream.to_dict(),
                "envelope": {"stream_id": stream.id, "sequence": "13"},
            }
        )
    finally:
        if not send.done():
            send.set_result(None)
        await asyncio.gather(pending, return_exceptions=True)
        await client.close()


@pytest.mark.asyncio
async def test_gap_plus_empty_after_gap_page_cannot_authorize_missing_cursor():
    from echo_agent_sdk.client import RunHandle

    client, stream, feed, _ = fixture()
    await client.connection.close()
    response = {
        "requested_after_sequence": "10",
        "events": [],
        "next_cursor": {"stream_id": stream.id, "last_processed_sequence": "10"},
    }
    transport = ReplayTransport(response)
    client.connection = ClientSideConnection(_Callbacks({}, 8), transport)
    client._replay_ranges.clear()
    feed.replay_terminal_sequence = None
    feed.accept_gap(
        {
            "stream": stream.to_dict(),
            "gap": {
                "from_sequence": "2",
                "to_sequence": "12",
                "snapshot_watermark": "12",
                "reason": "pause",
            },
        }
    )
    try:
        await RunHandle(client, WireHandle("run", "1", "run"), stream).replay("10")
        unknown = asyncio.create_task(ack(client, stream, 10))
        (await transport.entered.get()).set_result(None)
        await unknown
        assert stream not in client._event_cursors
        assert feed.last_sequence == 12
        assert (await feed.queue.get())["envelope"]["sequence"] == "1"
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_pending_ack_cannot_authorize_noncontiguous_or_backwards_gap():
    client, stream, feed, transport = fixture()
    feed.replay_terminal_sequence = None
    task = asyncio.create_task(ack(client, stream, 10))
    send = await transport.entered.get()

    def gap(start, end):
        return {
            "stream": stream.to_dict(),
            "gap": {
                "from_sequence": str(start),
                "to_sequence": str(end),
                "snapshot_watermark": str(end),
                "reason": "oversized",
            },
        }

    try:
        with pytest.raises(EchoAgentError):
            feed.accept_gap(gap(12, 13))
        assert feed.last_sequence == 1
        assert feed.accept_gap(gap(11, 12))
        # A still-pending predecessor cannot reconnect a later overlapping gap.
        with pytest.raises(EchoAgentError):
            feed.accept_gap(gap(11, 13))
        with pytest.raises(EchoAgentError):
            feed.accept_gap(gap(2, 3))
        assert feed.last_sequence == 12
        send.set_exception(ConnectionError("unknown write completion"))
        with pytest.raises(EchoAgentError):
            await task
        assert stream not in client._event_cursors
        assert not feed.pending_ack_sequences
    finally:
        if not send.done():
            send.set_result(None)
        await asyncio.gather(task, return_exceptions=True)
        await client.close()


def ordinary_page(stream, after, end, terminal=False):
    return {
        "requested_after_sequence": str(after),
        "events": [
            {
                "stream_id": stream.id,
                "sequence": str(seq),
                "payload": {
                    "event_type": "final_answer" if terminal and seq == end else "delta"
                },
            }
            for seq in range(after + 1, end + 1)
        ],
        "next_cursor": {"stream_id": stream.id, "last_processed_sequence": str(end)},
    }


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "order,expected",
    [
        ("empty_gap_tail", True),
        ("reverse_gap", True),
        ("reverse_plain", True),
        ("disconnected", False),
    ],
)
async def test_public_replay_coverage_closure_is_order_independent(order, expected):
    from echo_agent_sdk.client import RunHandle

    client, stream, feed, _ = fixture()
    await client.connection.close()
    transport = ReplayTransport({})
    client.connection = ClientSideConnection(_Callbacks({}, 8), transport)
    client._replay_ranges.clear()
    feed.replay_terminal_sequence = None
    run = RunHandle(client, WireHandle("run", "1", "run"), stream)
    pages = {
        "empty_gap_tail": [
            ordinary_page(stream, 6, 6),
            recovery_page(stream),
            ordinary_page(stream, 6, 7, True),
        ],
        "reverse_gap": [ordinary_page(stream, 6, 8, True), recovery_page(stream)],
        "reverse_plain": [
            ordinary_page(stream, 3, 5, True),
            ordinary_page(stream, 1, 3),
        ],
        "disconnected": [ordinary_page(stream, 8, 10, True), recovery_page(stream)],
    }[order]
    target = {
        "empty_gap_tail": 7,
        "reverse_gap": 8,
        "reverse_plain": 5,
        "disconnected": 10,
    }[order]
    try:
        for page in pages:
            transport.response = page
            await run.replay(page["requested_after_sequence"])
        assert feed.last_sequence == 1
        pending = asyncio.create_task(ack(client, stream, target))
        send = await transport.entered.get()
        assert stream not in client._event_cursors
        assert not feed.queue._closed
        send.set_result(None)
        await pending
        if expected:
            assert client._event_cursors[stream] == target
            assert feed.queue._closed
            assert len(feed.queue._items) == 0
        else:
            assert stream not in client._event_cursors
            assert not feed.queue._closed
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_pending_recovery_live_continuation_cannot_authorize_absent_prefix():
    from echo_agent_sdk.client import RunHandle

    client, stream, feed, _ = fixture()
    await client.connection.close()
    transport = ReplayTransport(recovery_page(stream))
    client.connection = ClientSideConnection(_Callbacks({}, 8), transport)
    client._replay_ranges.clear()
    feed.replay_terminal_sequence = None
    try:
        await RunHandle(client, WireHandle("run", "1", "run"), stream).replay("1")
        pending = asyncio.create_task(ack(client, stream, 6))
        send = await transport.entered.get()
        assert feed.accept_event(
            {
                "stream": stream.to_dict(),
                "envelope": {"stream_id": stream.id, "sequence": "7"},
            }
        )
        send.set_exception(ConnectionError("unknown outcome after peer continued"))
        with pytest.raises(EchoAgentError):
            await pending
        interior = asyncio.create_task(ack(client, stream, 3))
        (await transport.entered.get()).set_result(None)
        await interior
        assert stream not in client._event_cursors
        assert not feed.queue._closed
        assert (await feed.queue.get())["envelope"]["sequence"] == "1"
        assert feed.last_sequence == 7
    finally:
        await client.close()


# Production spawn and these controlled official-transport fixtures share the
# same private observer factory; no second validation or delivery handler.
def admission_observers(events, capacity):
    from echo_agent_sdk.client import _event_observers

    return _event_observers(events, capacity)


async def admission_fixture(capacity=1):
    client, stream, _, _ = fixture()
    await client.connection.close()
    stream = WireHandle("stream", "7", "stream")
    feed = _EventFeed(_AsyncQueue(capacity), stream)
    client._events = {stream.id: feed}
    client._replay_ranges = {}
    transport = ReplayTransport({})
    client.connection = ClientSideConnection(
        _Callbacks({}, 8),
        transport,
        observers=admission_observers(client._events, capacity),
    )
    return client, stream, feed, transport


def live_event(stream, sequence):
    return {
        "stream": stream.to_dict(),
        "envelope": {
            "stream_id": stream.id,
            "sequence": str(sequence),
            "payload": {"event_type": "delta"},
        },
    }


def live_gap(stream, sequence):
    return {
        "stream": stream.to_dict(),
        "gap": {
            "from_sequence": str(sequence),
            "to_sequence": str(sequence),
            "snapshot_watermark": str(sequence),
            "reason": "window pause",
        },
    }


async def peer_notification(transport, method, value):
    await transport.incoming.put({"jsonrpc": "2.0", "method": method, "params": value})


async def until_admission(condition):
    async def wait():
        while not condition():
            await asyncio.sleep(0)

    await asyncio.wait_for(wait(), 2)


@pytest.mark.asyncio
async def test_visible_pending_ack_does_not_lose_resumed_event():
    client, stream, feed, transport = await admission_fixture()
    caller = None
    try:
        await peer_notification(transport, "_echo_agent/event", live_event(stream, 1))
        await until_admission(lambda: feed.last_sequence == 1)
        gap = live_gap(stream, 2)
        await peer_notification(transport, "_echo_agent/gap", gap)
        await until_admission(lambda: feed.last_gap_watermark == 2)
        caller = asyncio.create_task(ack(client, stream, 2))
        completion = await transport.entered.get()
        assert transport.calls[0][0]["params"]["ack"] == {
            "stream": stream.to_dict(),
            "last_processed_sequence": "2",
        }
        assert not completion.done()
        await peer_notification(transport, "_echo_agent/event", live_event(stream, 3))
        await until_admission(lambda: feed.last_sequence == 3)
        await asyncio.sleep(0)
        assert not feed.queue._closed, (
            "pending visible ACK must not permanently overflow its resumed event"
        )
        assert stream not in client._event_cursors
        completion.set_result(None)
        await caller
        assert client._event_cursors[stream] == 2
        assert await asyncio.wait_for(feed.queue.get(), 2) == gap
        assert await asyncio.wait_for(feed.queue.get(), 2) == live_event(stream, 3)
        assert not feed.queue._closed
    finally:
        for _, completion in transport.calls:
            if not completion.done():
                completion.set_result(None)
        if caller is not None:
            await asyncio.gather(caller, return_exceptions=True)
        await client.close()


async def paused_admission(capacity=1):
    client, stream, feed, transport = await admission_fixture(capacity)
    await peer_notification(transport, "_echo_agent/event", live_event(stream, 1))
    await until_admission(lambda: feed.last_sequence == 1)
    await peer_notification(transport, "_echo_agent/gap", live_gap(stream, 2))
    await until_admission(lambda: feed.last_gap_watermark == 2)
    caller = asyncio.create_task(ack(client, stream, 2))
    completion = await transport.entered.get()
    assert transport.calls[0][0]["params"]["ack"]["last_processed_sequence"] == "2"
    assert not completion.done()
    return client, stream, feed, transport, caller, completion


@pytest.mark.asyncio
@pytest.mark.parametrize("outcome", ["failure", "consumer", "close", "caller_cancel"])
async def test_pending_admission_lifecycle(outcome):
    client, stream, feed, transport, caller, completion = await paused_admission()
    try:
        await peer_notification(transport, "_echo_agent/event", live_event(stream, 3))
        await until_admission(lambda: feed.last_sequence == 3)
        await asyncio.sleep(0)
        assert not feed.queue._closed
        if outcome == "failure":
            completion.set_exception(ConnectionError("native ACK unknown"))
            with pytest.raises(EchoAgentError) as error:
                await caller
            assert error.value.details["ack_submission"] == "outcome_unknown"
            await until_admission(lambda: feed.queue._closed)
            assert feed.queue._failure.details["reason"] == "client_queue_full"
            assert stream not in client._event_cursors
            assert next(iter(feed.queue._items)) == live_event(stream, 1)
        elif outcome == "consumer":
            # Capacity is signalled before the consumer's own callback completes.
            consumed = asyncio.Event()
            release = asyncio.Event()

            async def on_consume(_value):
                consumed.set()
                await release.wait()

            feed.queue.set_on_consume(on_consume)
            consumer = asyncio.create_task(feed.queue.get())
            await consumed.wait()
            await asyncio.wait_for(asyncio.shield(feed.delivery_tail), 2)
            assert not completion.done()
            assert list(feed.queue._items)[-1] == live_event(stream, 3)
            release.set()
            assert await consumer == live_event(stream, 1)
            feed.queue.set_on_consume(None)
            completion.set_result(None)
            await caller
            assert await feed.queue.get() == live_gap(stream, 2)
            assert await feed.queue.get() == live_event(stream, 3)
        elif outcome == "close":
            tail = feed.delivery_tail
            await client.close()
            with pytest.raises(asyncio.CancelledError):
                await caller
            await asyncio.wait_for(asyncio.shield(tail), 2)
            assert not any(
                "envelope" in x and x["envelope"]["sequence"] == "3"
                for x in feed.queue._items
            )
        else:
            caller.cancel()
            with pytest.raises(asyncio.CancelledError):
                await caller
            assert not completion.done() and not completion.cancelled()
            completion.set_result(None)
            await until_admission(lambda: feed.delivery_tail is None)
            assert client._event_cursors[stream] == 2
            assert not feed.queue._closed
            assert list(feed.queue._items)[-1] == live_event(stream, 3)
    finally:
        if not completion.done():
            completion.set_result(None)
        await asyncio.gather(caller, return_exceptions=True)
        await client.close()


@pytest.mark.asyncio
async def test_ordered_event_and_gap_delivery_survives_pending_write():
    client, stream, feed, transport, caller, completion = await paused_admission(2)
    try:
        for seq in (3, 4):
            await peer_notification(
                transport, "_echo_agent/event", live_event(stream, seq)
            )
        await peer_notification(transport, "_echo_agent/gap", live_gap(stream, 5))
        await until_admission(lambda: feed.last_gap_watermark == 5)
        completion.set_result(None)
        await caller
        await until_admission(lambda: feed.delivery_tail is None)
        assert not feed.queue._closed
        # Existing one-control-slot coalescing retains the old gap start and
        # removes data covered by its later watermark, without revalidation.
        merged = live_gap(stream, 5)
        merged["gap"]["from_sequence"] = "2"
        assert list(feed.queue._items) == [merged]
    finally:
        if not completion.done():
            completion.set_result(None)
        await asyncio.gather(caller, return_exceptions=True)
        await client.close()


@pytest.mark.asyncio
async def test_later_success_covers_held_event_without_reinsertion():
    client, stream, feed, transport, caller, completion = await paused_admission()
    high = None
    try:
        await peer_notification(transport, "_echo_agent/event", live_event(stream, 3))
        await until_admission(lambda: feed.last_sequence == 3)
        high = asyncio.create_task(ack(client, stream, 3))
        high_completion = await transport.entered.get()
        high_completion.set_result(None)
        await high
        await until_admission(lambda: feed.delivery_tail is None)
        assert client._event_cursors[stream] == 3
        assert list(feed.queue._items) == [live_gap(stream, 2)]
        completion.set_result(None)
        await caller
        assert client._event_cursors[stream] == 3
    finally:
        if not completion.done():
            completion.set_result(None)
        await asyncio.gather(caller, *([high] if high else []), return_exceptions=True)
        await client.close()


@pytest.mark.asyncio
async def test_unobserved_ack_cannot_borrow_capacity():
    client, stream, feed, transport = await admission_fixture()
    caller = None
    try:
        for seq in (1,):
            await peer_notification(
                transport, "_echo_agent/event", live_event(stream, seq)
            )
        await peer_notification(transport, "_echo_agent/gap", live_gap(stream, 2))
        await until_admission(lambda: feed.last_gap_watermark == 2)
        caller = asyncio.create_task(ack(client, stream, 3))
        completion = await transport.entered.get()
        await peer_notification(transport, "_echo_agent/event", live_event(stream, 3))
        await until_admission(lambda: feed.queue._closed)
        completion.set_result(None)
        await caller
        assert stream not in client._event_cursors
    finally:
        for _, completion in transport.calls:
            if not completion.done():
                completion.set_result(None)
        if caller is not None:
            await asyncio.gather(caller, return_exceptions=True)
        await client.close()


@pytest.mark.asyncio
async def test_cancelled_middle_observer_cannot_release_its_successor_early():
    from acp.connection import StreamDirection, StreamEvent

    client, stream, feed, transport, caller, completion = await paused_admission()
    observe = admission_observers(client._events, 1)[0]
    middle = None
    try:
        await peer_notification(transport, "_echo_agent/event", live_event(stream, 3))
        await until_admission(lambda: feed.last_sequence == 3)
        predecessor = feed.delivery_tail
        middle = asyncio.create_task(
            observe(
                StreamEvent(
                    StreamDirection.INCOMING,
                    {
                        "method": "_echo_agent/event",
                        "params": live_event(stream, 4),
                    },
                )
            )
        )
        await until_admission(lambda: feed.last_sequence == 4)
        cancelled_tail = feed.delivery_tail
        middle.cancel()
        with pytest.raises(asyncio.CancelledError):
            await middle
        assert not predecessor.done() and not cancelled_tail.done()
        # The cancelled context remains ordered after its original predecessor.
        await peer_notification(transport, "_echo_agent/gap", live_gap(stream, 5))
        await until_admission(lambda: feed.last_gap_watermark == 5)
        assert list(feed.queue._items)[-1] == live_gap(stream, 2)
        completion.set_result(None)
        await caller
        await until_admission(lambda: feed.delivery_tail is None)
        merged = live_gap(stream, 5)
        merged["gap"]["from_sequence"] = "2"
        assert list(feed.queue._items) == [merged]
        assert not feed.queue._closed
    finally:
        if not completion.done():
            completion.set_result(None)
        if middle is not None:
            await asyncio.gather(middle, return_exceptions=True)
        await asyncio.gather(caller, return_exceptions=True)
        await client.close()


@pytest.mark.asyncio
async def test_pending_feed_does_not_block_other_feed_response_or_duplicate():
    client, stream, feed, transport, caller, completion = await paused_admission()
    try:
        await peer_notification(transport, "_echo_agent/event", live_event(stream, 3))
        await until_admission(lambda: feed.last_sequence == 3)
        tail = feed.delivery_tail
        await peer_notification(transport, "_echo_agent/event", live_event(stream, 3))
        other = WireHandle("other", "8", "stream")
        await peer_notification(transport, "_echo_agent/event", live_event(other, 1))
        await until_admission(lambda: other.id in client._events)
        assert await asyncio.wait_for(
            client._events[other.id].queue.get(), 2
        ) == live_event(other, 1)
        assert (
            await asyncio.wait_for(
                client.connection.ext_method("test/independent", {}), 2
            )
            == {}
        )
        assert feed.delivery_tail is tail and not tail.done()
        assert not completion.done()
        completion.set_result(None)
        await caller
        await until_admission(lambda: feed.delivery_tail is None)
        assert not feed.queue._closed
    finally:
        if not completion.done():
            completion.set_result(None)
        await asyncio.gather(caller, return_exceptions=True)
        await client.close()


@pytest.mark.asyncio
async def test_cleanup_cancels_not_yet_started_ack_without_orphan_projection():
    client, stream, feed, transport = fixture()
    # Both real owners are ready before the loop advances. ACK registration
    # appends submit behind cleanup, so cleanup cancels it before its first step.
    caller = asyncio.create_task(ack(client, stream, 10))
    closer = asyncio.create_task(client._close_owned())
    try:
        await asyncio.wait_for(closer, 2)
        with pytest.raises(asyncio.CancelledError):
            await caller
        assert transport.calls == []
        assert stream not in client._event_cursors
        assert not client._ack_submissions
        assert not feed.pending_acknowledgements
        assert not feed.pending_ack_sequences
    finally:
        await asyncio.gather(caller, closer, return_exceptions=True)
        await client.close()
