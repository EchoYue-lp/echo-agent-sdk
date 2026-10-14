"""Official ACP receive/send windows around the existing reverse-call route."""

import asyncio
from copy import deepcopy

import pytest
from acp.client.connection import ClientSideConnection

from echo_agent_sdk import EchoAgentError, WireHandle
from echo_agent_sdk.client import EchoAgentClient, _Callbacks


class Transport:
    def __init__(self):
        self.incoming = asyncio.Queue()
        self.responses = asyncio.Queue()
        self.terminals = asyncio.Queue()
        self.hold_terminal = False

    async def receive(self):
        return await self.incoming.get()

    async def send(self, message):
        if "result" in message:
            self.responses.put_nowait(message)
        elif self.hold_terminal:
            done = asyncio.get_running_loop().create_future()
            self.terminals.put_nowait((deepcopy(message), done))
            await done

    async def close(self):
        return None


async def fixture(handler):
    callbacks = _Callbacks({}, 8)
    callbacks.extensions["extension"] = handler
    transport = Transport()
    connection = ClientSideConnection(callbacks, transport)
    sdk = object.__new__(EchoAgentClient)
    sdk._closed = False
    sdk._callbacks = callbacks
    sdk.connection = connection
    stream = WireHandle("stream", "1", "stream")
    call = {
        "extension": {"id": "extension", "generation": "1", "kind": "extension"},
        "invocation_id": "original-invocation",
        "stream": stream.to_dict(),
        "invocation": {
            "operation": "agent_execute_stream",
            "input": {"task": "中文🌊"},
        },
    }
    await transport.incoming.put(
        {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "_echo_agent/extension/invoke",
            "params": call,
        }
    )
    return sdk, callbacks, stream, transport, call


async def cancel(transport):
    await transport.incoming.put(
        {
            "jsonrpc": "2.0",
            "method": "_echo_agent/extension/cancel",
            "params": {"invocation_id": "original-invocation"},
        }
    )


@pytest.mark.asyncio
async def test_early_ack_failed_or_wrong_identity_terminal_keeps_real_cancel_address():
    signals = []

    async def handler(call, signal):
        signals.append(signal)
        return {"outcome": "stream", "stream": call["stream"]}

    sdk, callbacks, stream, transport, _ = await fixture(handler)
    try:
        response = await asyncio.wait_for(transport.responses.get(), 2)
        assert response["result"]["outcome"] == "stream"
        assert callbacks.invocation_cancel["original-invocation"] is signals[0]
        wrong = WireHandle(stream.id, "2", "stream")
        await sdk.notify(
            "echo_agent/extension/stream",
            {"event": "cancelled", "stream": wrong.to_dict(), "sequence": "1"},
        )
        assert "original-invocation" in callbacks.invocation_cancel
        transport.hold_terminal = True
        send = asyncio.create_task(sdk.stream_writer(stream).cancelled())
        _, completed = await asyncio.wait_for(transport.terminals.get(), 2)
        completed.set_exception(ConnectionError("unknown native terminal completion"))
        with pytest.raises(EchoAgentError):
            await send
        assert "original-invocation" in callbacks.invocation_cancel
        await cancel(transport)
        await asyncio.wait_for(signals[0].wait(), 2)
        send = asyncio.create_task(sdk.stream_writer(stream).cancelled())
        _, completed = await asyncio.wait_for(transport.terminals.get(), 2)
        assert "original-invocation" in callbacks.invocation_cancel
        completed.set_result(None)
        await send
        assert not callbacks.invocation_cancel
        assert not callbacks.stream_cancel_routes
    finally:
        await sdk.connection.close()


@pytest.mark.asyncio
async def test_cancel_before_setup_response_preserves_declared_live_stream():
    entered, release = asyncio.Event(), asyncio.Event()
    signals = []

    async def handler(call, signal):
        signals.append(signal)
        entered.set()
        await release.wait()
        return {"outcome": "stream", "stream": call["stream"]}

    sdk, callbacks, stream, transport, _ = await fixture(handler)
    try:
        await asyncio.wait_for(entered.wait(), 2)
        await cancel(transport)
        await asyncio.wait_for(signals[0].wait(), 2)
        release.set()
        response = await asyncio.wait_for(transport.responses.get(), 2)
        assert response["result"]["error"]["code"] == "cancelled"
        assert "original-invocation" in callbacks.invocation_cancel
        await sdk.stream_writer(stream).cancelled()
        assert not callbacks.invocation_cancel
    finally:
        release.set()
        await sdk.connection.close()


@pytest.mark.asyncio
async def test_terminal_before_setup_response_and_setup_failure_do_not_retain_routes():
    entered, release = asyncio.Event(), asyncio.Event()

    async def handler(call, signal):
        entered.set()
        await release.wait()
        return {"outcome": "stream", "stream": call["stream"]}

    sdk, callbacks, stream, transport, _ = await fixture(handler)
    try:
        await asyncio.wait_for(entered.wait(), 2)
        await sdk.stream_writer(stream).cancelled()
        release.set()
        await asyncio.wait_for(transport.responses.get(), 2)
        assert not callbacks.invocation_cancel
        assert not callbacks.stream_cancel_routes
    finally:
        release.set()
        await sdk.connection.close()

    async def failure(call, signal):
        return {
            "outcome": "error",
            "error": {
                "code": "extension_failed",
                "message": "setup failed",
                "retryable": "never",
            },
        }

    sdk, callbacks, stream, transport, _ = await fixture(failure)
    try:
        await asyncio.wait_for(transport.responses.get(), 2)
        assert not callbacks.invocation_cancel
        assert not callbacks.stream_cancel_routes
    finally:
        await sdk.connection.close()


@pytest.mark.asyncio
async def test_failed_client_close_retains_the_original_cancel_route_for_retry():
    async def handler(call, signal):
        return {"outcome": "stream", "stream": call["stream"]}

    sdk, callbacks, stream, transport, _ = await fixture(handler)
    await asyncio.wait_for(transport.responses.get(), 2)
    sdk._close_lock = asyncio.Lock()
    sdk.process = None
    sdk._events = {}
    sdk._updates = {}
    sdk._process_watch = None

    class FailedContextExit:
        async def __aexit__(self, *_args):
            raise RuntimeError("official context teardown failed")

    sdk._context_manager = FailedContextExit()
    with pytest.raises(RuntimeError, match="teardown failed"):
        await sdk.close()
    assert callbacks.invocation_cancel["original-invocation"].is_set()
    assert stream in callbacks.stream_cancel_routes
    await sdk.close()
    assert not callbacks.invocation_cancel
    assert not callbacks.stream_cancel_routes
