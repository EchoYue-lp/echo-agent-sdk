"""Close real official ACP tasks while a reverse callback is pending."""

import asyncio
import hashlib
import json
import sys
from pathlib import Path
from unittest.mock import AsyncMock

import pytest
from acp.client.connection import ClientSideConnection
from acp.stdio import spawn_agent_process

from echo_agent_sdk.client import EchoAgentClient, _Callbacks


@pytest.mark.asyncio
@pytest.mark.parametrize("payload_kind", ["empty", "two_mib", "unicode_crlf"])
async def test_default_spawn_inherits_diagnostics_without_a_private_pipe(
    tmp_path, capfdbinary, payload_kind
):
    # Capture only the application's inherited fd2. No SDK child pipe is read
    # or substituted, including before initialize completes.
    payloads = {
        "empty": b"",
        "two_mib": b"B" * (2 * 1024 * 1024),
        "unicode_crlf": "诊断 中😀\nCRLF\r\n尾\n".encode(),
    }
    payload = payloads[payload_kind]
    peer = tmp_path / "official_agent.py"
    peer.write_text(
        r"""
import asyncio
import json
import os
import sys
from pathlib import Path
from acp import Agent, InitializeResponse, run_agent

sdk, work, kind = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3]
payload = {
    "empty": b"",
    "two_mib": b"B" * (2 * 1024 * 1024),
    "unicode_crlf": "诊断 中😀\nCRLF\r\n尾\n".encode(),
}[kind]
document = json.loads((sdk / "contracts/sdk/fixtures/extension/v1/capability-meta-valid.json").read_text())["payload"]
document.update(json.loads((sdk / "sdks/shared/contract-digests.json").read_text()))

def mark(name):
    temporary = work / (name + ".tmp")
    temporary.write_text(json.dumps({"pid": os.getpid()}))
    temporary.replace(work / (name + ".json"))

async def main():
    remaining = memoryview(payload)
    while remaining:
        written = os.write(sys.stderr.fileno(), remaining)
        if written <= 0:
            raise OSError("diagnostic write made no progress")
        remaining = remaining[written:]

    class OfficialPeer(Agent):
        async def initialize(self, protocol_version, client_capabilities=None, client_info=None, **kwargs):
            mark("initialized")
            return InitializeResponse.model_validate({
                "protocolVersion": protocol_version,
                "agentCapabilities": {"_meta": {"echo_agent": document}},
            })

    await run_agent(OfficialPeer())
    mark("stopped")

asyncio.run(main())
""",
        encoding="utf8",
    )
    client = process = None
    main_exit = cleanup_exit = None
    initialized = close_completed = False
    try:
        client = await asyncio.wait_for(
            EchoAgentClient.spawn(
                sys.executable,
                "-B",
                str(peer),
                str(Path(__file__).resolve().parents[3]),
                str(tmp_path),
                payload_kind,
            ),
            5,
        )
        process = client.process
        initialized = True
        await asyncio.wait_for(client.close(), 5)
        close_completed = True
        main_exit = await process.wait()
    finally:
        try:
            if process is not None and process.returncode is None:
                await client.close()
                cleanup_exit = process.returncode
        finally:
            captured = capfdbinary.readouterr()
            (tmp_path / "stderr.raw").write_bytes(captured.err)
            (tmp_path / "stdout.raw").write_bytes(captured.out)
            (tmp_path / "receipt.json").write_text(
                json.dumps(
                    {
                        "payload_kind": payload_kind,
                        "pid": None if process is None else process.pid,
                        "initialized": initialized,
                        "main_close_completed": close_completed,
                        "main_native_exit": main_exit,
                        "cleanup_native_exit": cleanup_exit,
                        "stderr_bytes": len(captured.err),
                        "stderr_sha256": hashlib.sha256(captured.err).hexdigest(),
                    }
                )
            )
    assert main_exit == 0
    assert cleanup_exit is None, "cleanup must not substitute for natural success"
    assert captured.out == b"", "ACP stdout leaked into application output"
    assert captured.err == payload
    assert json.loads((tmp_path / "initialized.json").read_text())["pid"] == process.pid
    assert json.loads((tmp_path / "stopped.json").read_text())["pid"] == process.pid


@pytest.mark.asyncio
async def test_close_settles_pending_callback_without_sending_after_writer_close():
    entered = asyncio.Event()

    async def pending(_payload, _cancellation):
        entered.set()
        await asyncio.Event().wait()

    class Transport(asyncio.Transport):
        def __init__(self):
            self.closed = False

        def is_closing(self):
            return self.closed

        def write(self, _payload):
            pass

        def close(self):
            self.closed = True

    reader = asyncio.StreamReader()
    transport = Transport()
    writer = asyncio.StreamWriter(
        transport,
        asyncio.StreamReaderProtocol(reader),
        reader,
        asyncio.get_running_loop(),
    )
    callbacks = _Callbacks({}, 8)
    callbacks.extensions["managed"] = pending
    connection = ClientSideConnection(callbacks, writer, reader)
    client = object.__new__(EchoAgentClient)
    client.connection = connection
    client.process = None  # This test owns only an in-memory ACP connection.
    client._close_lock = asyncio.Lock()
    client._closed = False
    client._callbacks = callbacks
    client._events = {}
    client._updates = {}
    client._process_watch = None

    class Context:
        async def __aexit__(self, *_args):
            await connection.close()

    client._context_manager = Context()
    reader.feed_data(
        (
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "_echo_agent/extension/invoke",
                    "params": {
                        "extension": {
                            "id": "managed",
                            "generation": "1",
                            "kind": "extension",
                        },
                        "invocation_id": "held-call",
                        "invocation": {
                            "operation": "agent_component_call",
                            "input": {},
                        },
                    },
                }
            )
            + "\n"
        ).encode()
    )
    close = None
    try:
        await asyncio.wait_for(entered.wait(), 1)
        close = asyncio.create_task(client.close())
        done, _ = await asyncio.wait({close}, timeout=0.25)
        assert close in done, (
            "official close waits on callback response after sender exited"
        )
        await close
        assert not callbacks.invocation_cancel
    finally:
        if close is not None and not close.done():
            close.cancel()
            await asyncio.gather(close, return_exceptions=True)
        reader.feed_eof()
        writer.close()


def client_with_context(context):
    client = object.__new__(EchoAgentClient)
    client._close_lock = asyncio.Lock()
    client._closed = False
    client._callbacks = _Callbacks({}, 8)
    client._events = {}
    client._updates = {}
    client._process_watch = None
    client._context_manager = context
    client.process = None  # Controlled context tests have no native child.
    client.connection = type("Connection", (), {"close": AsyncMock()})()
    return client


@pytest.mark.asyncio
async def test_cancelled_close_waiter_retains_same_cleanup_owner():
    entered = asyncio.Event()
    release = asyncio.Event()
    calls = []

    class Context:
        async def __aexit__(self, *_args):
            calls.append("started")
            entered.set()
            await release.wait()
            calls.append("completed")

    client = client_with_context(Context())
    first = asyncio.create_task(client.close())
    second = None
    try:
        await asyncio.wait_for(entered.wait(), 1)
        first.cancel()
        with pytest.raises(asyncio.CancelledError):
            await first
        assert not client._closed, "cleanup not completed yet"
        second = asyncio.create_task(client.close())
        await asyncio.sleep(0)
        assert not second.done(), "retry must await the retained close operation"
        release.set()
        await asyncio.wait_for(second, 1)
        assert calls == ["started", "completed"]
        assert client._closed
    finally:
        release.set()
        if not first.done():
            first.cancel()
        if second is not None and not second.done():
            second.cancel()
        await asyncio.gather(
            first, *([second] if second is not None else []), return_exceptions=True
        )


@pytest.mark.asyncio
@pytest.mark.parametrize("fault_count", [1, 2])
async def test_retry_after_official_context_failure_requires_real_process_exit(
    fault_count,
):
    callbacks = _Callbacks({}, 8)
    context = spawn_agent_process(
        callbacks,
        sys.executable,
        "-c",
        "import time; time.sleep(60)",
        transport_kwargs={"shutdown_timeout": 0.05},
    )
    connection, process = await context.__aenter__()
    original_exit = context.__aexit__
    exit_calls = []

    async def observed_exit(*args):
        exit_calls.append("exit")
        return await original_exit(*args)

    context.__aexit__ = observed_exit
    client = object.__new__(EchoAgentClient)
    client.connection = connection
    client.process = process
    client._context_manager = context
    client._close_lock = asyncio.Lock()
    client._closed = False
    client._callbacks = callbacks
    client._events = {}
    client._updates = {}
    client._process_watch = None
    terminate = process.terminate
    calls = []

    def fail_once():
        calls.append("terminate")
        if len(calls) <= fault_count:
            raise OSError("injected native termination fault")
        terminate()

    process.terminate = fail_once
    try:
        with pytest.raises(OSError, match="native termination fault"):
            await client.close()
        assert not client._closed
        assert process.returncode is None
        for _ in range(fault_count - 1):
            with pytest.raises(OSError, match="native termination fault"):
                await client.close()
            assert not client._closed
            assert process.returncode is None
        await client.close()
        assert process.returncode is not None, (
            "exhausted async context must not prove native process exit"
        )
        assert calls == ["terminate"] * (fault_count + 1)
        assert client._closed
        assert exit_calls == ["exit"], (
            "official async generator can be exited only once"
        )
    finally:
        process.terminate = terminate
        if process.returncode is None:
            process.kill()
        await process.wait()


@pytest.mark.asyncio
async def test_official_connection_close_fault_retries_original_connection_owner():
    callbacks = _Callbacks({}, 8)
    context = spawn_agent_process(
        callbacks,
        sys.executable,
        "-c",
        "import time; time.sleep(60)",
        transport_kwargs={"shutdown_timeout": 0.05},
    )
    connection, process = await context.__aenter__()
    client = client_with_context(context)
    client.connection, client.process, client._callbacks = (
        connection,
        process,
        callbacks,
    )
    real_close = connection.close
    calls = []

    async def failing_close():
        calls.append("connection close")
        if len(calls) == 1:
            raise OSError("injected connection close fault")
        await real_close()

    connection.close = failing_close
    callbacks.extensions["retained"] = object()
    try:
        with pytest.raises(OSError, match="connection close fault"):
            await client.close()
        assert not client._closed
        assert "retained" in callbacks.extensions
        assert process.returncode is not None, (
            "outer official transport still terminates its child"
        )
        await client.close()
        assert client._closed
        assert calls == ["connection close", "connection close"]
        assert not callbacks.extensions
        assert connection._conn._closed
    finally:
        connection.close = real_close
        await real_close()
        if process.returncode is None:
            process.kill()
        await process.wait()


@pytest.mark.asyncio
async def test_cancelled_retry_waiter_retains_original_live_process_cleanup_task():
    callbacks = _Callbacks({}, 8)
    context = spawn_agent_process(
        callbacks,
        sys.executable,
        "-c",
        "import time; time.sleep(60)",
        transport_kwargs={"shutdown_timeout": 0.05},
    )
    connection, process = await context.__aenter__()
    client = client_with_context(context)
    client.connection, client.process, client._callbacks = (
        connection,
        process,
        callbacks,
    )
    terminate = process.terminate
    real_wait = process.wait
    calls = []

    def fail_once():
        calls.append("terminate")
        if len(calls) == 1:
            raise OSError("injected termination fault")
        terminate()

    process.terminate = fail_once
    entered, release = asyncio.Event(), asyncio.Event()
    waiters = []

    async def held_wait():
        entered.set()
        await release.wait()
        return await real_wait()

    try:
        with pytest.raises(OSError, match="termination fault"):
            await client.close()
        process.wait = held_wait
        waiter = asyncio.create_task(client.close())
        waiters.append(waiter)
        await asyncio.wait_for(entered.wait(), 1)
        owner = client._close_task
        waiter.cancel()
        with pytest.raises(asyncio.CancelledError):
            await waiter
        assert not owner.done()
        assert process.returncode is None
        assert not client._closed
        retry = asyncio.create_task(client.close())
        waiters.append(retry)
        await asyncio.sleep(0)
        assert client._close_task is owner
        assert not retry.done()
        release.set()
        await asyncio.wait_for(retry, 4)
        assert process.returncode is not None
        assert calls == ["terminate", "terminate"]
        assert client._closed
    finally:
        release.set()
        process.terminate, process.wait = terminate, real_wait
        if process.returncode is None:
            process.kill()
        await real_wait()
        await asyncio.gather(*waiters, return_exceptions=True)


@pytest.mark.asyncio
async def test_kill_failure_retains_live_owner_and_retry_observes_actual_exit(tmp_path):
    ready = tmp_path / "ready"
    callbacks = _Callbacks({}, 8)
    context = spawn_agent_process(
        callbacks,
        sys.executable,
        "-c",
        "import signal,time;from pathlib import Path;"
        "signal.signal(signal.SIGTERM,signal.SIG_IGN);"
        f"Path({str(ready)!r}).touch();time.sleep(60)",
        transport_kwargs={"shutdown_timeout": 0.05},
    )
    connection, process = await context.__aenter__()
    client = client_with_context(context)
    client.connection, client.process, client._callbacks = (
        connection,
        process,
        callbacks,
    )
    terminate, kill = process.terminate, process.kill
    terminate_calls, kill_calls = [], []

    def initial_termination_fault():
        terminate_calls.append("terminate")
        if len(terminate_calls) == 1:
            raise OSError("initial termination failure")
        terminate()

    def first_kill_fault():
        kill_calls.append("kill")
        if len(kill_calls) == 1:
            raise OSError("injected kill failure")
        kill()

    process.terminate, process.kill = initial_termination_fault, first_kill_fault
    try:

        async def wait_for_condition_1():
            while not ready.exists():
                await asyncio.sleep(0.01)

        await asyncio.wait_for(wait_for_condition_1(), 2)
        with pytest.raises(OSError, match="initial termination failure"):
            await client.close()
        with pytest.raises(OSError, match="kill failure"):
            await asyncio.wait_for(client.close(), 6)
        assert not client._closed
        assert process.returncode is None
        await asyncio.wait_for(client.close(), 6)
        assert client._closed
        assert process.returncode is not None
        assert kill_calls == ["kill", "kill"]
        assert terminate_calls == ["terminate", "terminate", "terminate"]
    finally:
        process.terminate, process.kill = terminate, kill
        if process.returncode is None:
            kill()
        await process.wait()
