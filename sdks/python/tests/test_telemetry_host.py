"""Accepted telemetry: real Agent records, local OTLP receipt and lifecycle."""

import asyncio
import base64
import inspect
import json
import os
import subprocess
import sys
import tempfile
import threading
import time
from contextlib import AsyncExitStack, asynccontextmanager
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required telemetry Host missing")
ROOT = Path(
    os.environ.get(
        "ECHO_AGENT_SDK_TELEMETRY_ROOT",
        str(Path(__file__).resolve().parents[3] / "target/telemetry-host"),
    )
)
FIXTURE = (
    Path(__file__).resolve().parents[2] / "shared/testing/telemetry_host_fixture.py"
)

SHARED = Path(__file__).resolve().parents[2] / "shared/testing"
sys.path.insert(0, str(SHARED))
from file_sha256 import file_sha256


def test_host_exit_publication_is_complete_before_visible(tmp_path, monkeypatch):
    # This pure fixture regression runs without a Host or OTLP collector. Pause
    # the actual writer after half the JSON, not after an arbitrary sleep.
    from telemetry_host_fixture import publish_host_exit

    partial = threading.Event()
    release = threading.Event()
    failures = []
    original_write_text = Path.write_text
    expected = {"pid": 123, "returncode": 0, "observed_by_original_parent": True}

    def paused_write_text(path, text, *args, **kwargs):
        if path.parent != tmp_path:
            return original_write_text(path, text, *args, **kwargs)
        with path.open("w", encoding="utf8") as output:
            middle = len(text) // 2
            output.write(text[:middle])
            output.flush()
            partial.set()
            if not release.wait(5):
                raise TimeoutError("test did not release the controlled JSON writer")
            output.write(text[middle:])
        return len(text)

    def write_exit():
        try:
            publish_host_exit(tmp_path, expected["pid"], expected["returncode"])
        except OSError as error:
            failures.append(error)

    monkeypatch.setattr(Path, "write_text", paused_write_text)
    writer = threading.Thread(target=write_exit, daemon=False)
    writer.start()
    try:
        assert partial.wait(5), "actual publisher did not reach the write barrier"
        assert not (tmp_path / "host-exit.json").exists(), (
            "final exit receipt became visible before its JSON was complete"
        )
    finally:
        release.set()
        writer.join(timeout=5)
    assert not writer.is_alive(), "controlled publisher thread did not terminate"
    assert not failures, failures
    receipt = tmp_path / "host-exit.json"
    assert receipt.exists()
    assert json.loads(receipt.read_text()) == expected


def test_proxy_partial_writes_preserve_utf8_bytes(monkeypatch):
    from telemetry_host_fixture import write_all

    data = "中😀\r\n完整字节\n".encode()
    received = bytearray()

    def partial_write(_descriptor, block):
        count = min(3, len(block))
        received.extend(block[:count])
        return count

    monkeypatch.setattr(os, "write", partial_write)
    write_all(123, data)
    assert received == data

    monkeypatch.setattr(os, "write", lambda _descriptor, _block: 0)
    with pytest.raises(OSError, match="no progress"):
        write_all(123, data)


@pytest.mark.parametrize("close_stdout", [False, True])
def test_proxy_reaps_child_without_buffered_stdio_finalization(tmp_path, close_stdout):
    # Use the same proxy implementation with an explicit Python argv, including
    # on Windows. No Rust Host, shell executable or platform skip is involved.
    child = tmp_path / "synthetic_child.py"
    child.write_text(
        r"""
from pathlib import Path
import base64
import json
import os
import sys
import time

work = Path(sys.argv[2]).parent
expected = base64.b64decode((work / "input.b64").read_bytes())
received = sys.stdin.buffer.read(len(expected)) if expected else sys.stdin.buffer.read()
temporary = work / "received.tmp"
temporary.write_text(json.dumps({"pid": os.getpid(), "input": base64.b64encode(received).decode()}))
temporary.replace(work / "received.json")
deadline = time.monotonic() + 5
while not (work / "release-child").exists():
    if time.monotonic() > deadline:
        raise TimeoutError("test did not release synthetic child")
    time.sleep(0.005)
sys.stdout.buffer.write(("out 中😀\r\n" * 16384).encode())
sys.stdout.buffer.flush()
sys.stderr.buffer.write("synthetic stderr 中😀\n".encode())
sys.stderr.buffer.flush()
""",
        encoding="utf8",
    )
    payload = b"" if close_stdout else ("in 中😀\r\n" * 1024).encode()
    (tmp_path / "input.b64").write_bytes(base64.b64encode(payload))
    (tmp_path / "host.json").write_text("{}")
    program = (
        "import sys; from pathlib import Path; "
        "sys.path.insert(0, sys.argv[1]); "
        "from telemetry_host_fixture import proxy; "
        "proxy(Path(sys.argv[2]), sys.executable, host_args=(sys.argv[3],))"
    )
    process = None
    with (
        (tmp_path / "stdout.raw").open("wb") as stdout,
        (tmp_path / "stderr.raw").open("wb") as stderr,
    ):
        process = subprocess.Popen(
            [
                sys.executable,
                "-B",
                "-c",
                program,
                str(SHARED),
                str(tmp_path),
                str(child),
            ],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE if close_stdout else stdout,
            stderr=stderr,
            env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
        )
        try:
            if close_stdout:
                process.stdin.close()
                process.stdout.close()
            else:
                # Deliberately split a multibyte character across parent writes;
                # keep this stdin open until after the proxy naturally exits.
                process.stdin.write(payload[:4])
                process.stdin.flush()
                process.stdin.write(payload[4:])
                process.stdin.flush()
            deadline = time.monotonic() + 5
            while not (tmp_path / "received.json").exists():
                assert process.poll() is None, "proxy exited before forwarding input"
                assert time.monotonic() < deadline, (
                    "synthetic child did not receive input"
                )
                time.sleep(0.005)
            received = json.loads((tmp_path / "received.json").read_text())
            assert base64.b64decode(received["input"]) == payload
            (tmp_path / "release-child").write_text("release original synthetic child")
            assert process.wait(timeout=8) == 0
        finally:
            (tmp_path / "release-child").write_text(
                "release during failed test cleanup"
            )
            if not process.stdin.closed:
                process.stdin.close()
            if process.poll() is None:
                process.terminate()
                process.wait(timeout=5)
    exit_receipt = json.loads((tmp_path / "host-exit.json").read_text())
    assert exit_receipt == {
        "pid": received["pid"],
        "returncode": 0,
        "observed_by_original_parent": True,
    }
    assert (tmp_path / "stderr.raw").read_bytes() == "synthetic stderr 中😀\n".encode()
    if not close_stdout:
        assert (tmp_path / "stdout.raw").read_bytes() == (
            "out 中😀\r\n" * 16384
        ).encode()


async def helper(work, *args):
    process = await asyncio.create_subprocess_exec(
        sys.executable,
        str(FIXTURE),
        "--work",
        str(work),
        *args,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.STDOUT,
    )
    output, _ = await process.communicate()
    assert process.returncode == 0, output.decode()
    return output.decode()


@asynccontextmanager
async def fixture_scope(work, *, close=lambda: None, evidence=lambda: None):
    service = primary = None
    failures = []

    async def close_sdk():
        outcome = close()
        if inspect.isawaitable(outcome):
            await asyncio.wait_for(outcome, 5)

    async def stop_service():
        if service is None:
            return
        try:
            await asyncio.wait_for(service.wait(), 5)
        except asyncio.TimeoutError as error:
            failures.append(error)
            if service.returncode is None:
                service.terminate()
            try:
                await asyncio.wait_for(service.wait(), 5)
            except asyncio.TimeoutError as term_error:
                failures.append(term_error)
                if service.returncode is None:
                    service.kill()
                await asyncio.wait_for(service.wait(), 5)
        assert service.returncode == 0, service.returncode

    async def run_cleanup(cleanup):
        try:
            outcome = cleanup()
            if inspect.isawaitable(outcome):
                await outcome
        except BaseException as error:
            failures.append(error)
            raise

    try:
        # The standard stack attempts every callback even if an earlier one
        # raises, including cancellation. No exception is suppressed.
        async with AsyncExitStack() as stack:
            for cleanup in reversed(
                (
                    lambda: (work / "release-export").write_text(
                        "cleanup actual response"
                    ),
                    close_sdk,
                    lambda: helper(work, "--cleanup-host"),
                    lambda: (work / "stop").write_text("stop"),
                    stop_service,
                    evidence,
                )
            ):
                stack.push_async_callback(run_cleanup, cleanup)
            try:
                service = await asyncio.create_subprocess_exec(
                    sys.executable, str(FIXTURE), "--work", str(work), "--serve"
                )
                yield service
            finally:
                primary = sys.exc_info()[1]
    finally:
        # Project after the stack has unwound, retaining the original exception
        # object without creating a cycle in ExitStack's context repair.
        if primary is not None and failures:
            primary.cleanup_errors = tuple(failures)
            raise primary from RuntimeError(f"telemetry cleanup failures: {failures!r}")
        if primary is None and failures:
            raise RuntimeError(
                f"telemetry cleanup failures: {failures!r}"
            ) from failures[0]


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
@pytest.mark.parametrize("console_enabled", [False, True])
async def test_accepted_telemetry_exports_and_flushes(console_enabled):
    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-", dir=ROOT)).resolve()
    await helper(work, *(["--console"] if console_enabled else []))
    facts = {
        "work": str(work),
        "host": HOST,
        "errors": {},
    }
    sdk = None

    async def close_sdk():
        if sdk is not None:
            await sdk.close()

    def evidence():
        (work / "observed.json").write_text(
            json.dumps(facts, ensure_ascii=False, indent=2)
        )
        print(f"Telemetry evidence: {work}")

    async with fixture_scope(work, close=close_sdk, evidence=evidence) as service:
        facts["host_sha256"] = file_sha256(HOST)

        async def ready():
            while not (work / "ready").exists():
                assert service.returncode is None
                await asyncio.sleep(0.01)

        await asyncio.wait_for(ready(), 10)
        facts["scenario"] = json.loads((work / "scenario.json").read_text())
        async with await EchoAgentClient.spawn(
            HOST,
            "--config",
            str(work / "host.json"),
            env={
                "RUST_LOG": "info",
                "OTEL_METRIC_EXPORT_INTERVAL": "60000",
                "OTEL_BSP_SCHEDULE_DELAY": "60000",
            },
        ) as sdk:
            agent = await sdk.create_agent()
            session = await agent.create_session(str(work))

            async def failure(name, op, args):
                with pytest.raises(EchoAgentError) as caught:
                    await session.invoke(op, args)
                facts["errors"][name] = caught.value.code

            facts["before"] = await session.invoke("telemetry.status", [])
            await failure("shutdown_before", "telemetry.shutdown", [])
            scenario = facts["scenario"]
            args = [scenario["endpoint"], scenario["service_name"], console_enabled]
            await failure("invalid_console", "telemetry.init", [*args[:2], "false"])
            await failure(
                "invalid_endpoint",
                "telemetry.init",
                ["", scenario["service_name"], False],
            )
            await failure(
                "invalid_service", "telemetry.init", [scenario["endpoint"], " ", False]
            )
            facts["init"] = await session.invoke("telemetry.init", args)
            facts["after"] = await session.invoke("telemetry.status", [])
            await failure("repeat_init", "telemetry.init", args)
            run = await session.start_run(
                "Read probe.txt using read_file and consume its full content."
            )
            assert (await run.wait(timeout=15))["settled"]
            facts["run_outcome"] = await run.outcome_status()
            facts["replay"] = await run.replay()
            facts["shutdown"] = await session.invoke("telemetry.shutdown", [])
            facts["collector_at_shutdown"] = (work / "otlp.jsonl").read_text()
            facts["after_shutdown"] = await session.invoke("telemetry.status", [])
            await failure("repeat_shutdown", "telemetry.shutdown", [])
            await session.close()
            await agent.close()
        (work / "observed.json").write_text(
            json.dumps(facts, ensure_ascii=False, indent=2)
        )
        proof = json.loads(await helper(work, "--verify"))
        assert proof["shutdown_flush"] and proof["business_file_content_consumed"]


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
async def test_public_close_waits_for_real_slow_telemetry_flush():
    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-public-close-", dir=ROOT)).resolve()
    await helper(work)
    facts = {
        "work": str(work),
        "host": HOST,
        "mode": "Python public sdk.close only; no fixture EOF",
    }
    sdk = shutdown = closing = None

    async def barrier(name):
        async def ready():
            while not (work / name).exists():
                await asyncio.sleep(0.005)

        await asyncio.wait_for(ready(), 10)

    async def close_sdk():
        if sdk is not None:
            await sdk.close()
        if shutdown is not None:
            await asyncio.wait_for(asyncio.gather(shutdown, return_exceptions=True), 5)

    def evidence():
        (work / "wire-observed.json").write_text(
            json.dumps(facts, ensure_ascii=False, indent=2)
        )
        print(f"Telemetry Python publicclose evidence: {work}")

    async with fixture_scope(work, close=close_sdk, evidence=evidence):
        facts["host_sha256"] = file_sha256(HOST)
        await barrier("ready")
        scenario = json.loads((work / "scenario.json").read_text())
        sdk = await EchoAgentClient.spawn(
            sys.executable,
            str(FIXTURE),
            "--work",
            str(work),
            "--proxy-host",
            HOST,
            env={
                "RUST_LOG": "info",
                "OTEL_METRIC_EXPORT_INTERVAL": "60000",
                "OTEL_BSP_SCHEDULE_DELAY": "60000",
            },
        )
        agent = await sdk.create_agent()
        session = await agent.create_session(str(work))
        await session.invoke(
            "telemetry.init", [scenario["endpoint"], scenario["service_name"], False]
        )
        run = await session.start_run(
            "Read probe.txt using read_file and consume its full content."
        )
        assert (await run.wait(timeout=15))["settled"]
        (work / "hold-export").write_text("hold actual export response")
        shutdown = asyncio.create_task(session.invoke("telemetry.shutdown", []))
        await barrier("export-held")
        facts["status_while_export_held"] = await session.invoke("telemetry.status", [])
        assert not shutdown.done()
        closing = asyncio.create_task(sdk.close())
        await asyncio.sleep(0.1)
        facts["before_release"] = {
            "public_close_done": closing.done(),
            "real_host_exited": (work / "host-exit.json").exists(),
        }
        (work / "wire-observed.json").write_text(
            json.dumps(facts, ensure_ascii=False, indent=2)
        )
        assert not closing.done(), (
            "public close finished while actual exporter work was held"
        )
        assert not (work / "host-exit.json").exists()
        (work / "release-export").write_text("release real response")
        await asyncio.wait_for(closing, 10)
        await barrier("host-exit.json")
        facts["exit"] = json.loads((work / "host-exit.json").read_text())
        assert facts["exit"]["returncode"] == 0
        facts["collector_after_exit"] = (work / "otlp.jsonl").read_text()
        assert "llm.calls" in facts["collector_after_exit"]


@pytest.mark.asyncio
@pytest.mark.skipif(
    not os.environ.get("ECHO_AGENT_SDK_OTLP_COLLECTOR"),
    reason="real collector required",
)
async def test_telemetry_setup_and_cleanup_faults_reap_original_processes(monkeypatch):
    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-cleanup-fault-", dir=ROOT)).resolve()
    await helper(work)
    service = collector_pid = None
    close_error = RuntimeError("controlled SDK close failure")
    evidence_error = OSError("controlled evidence write failure")
    stop_error = OSError("controlled stop publication failure")

    async def failed_close():
        raise close_error

    def failed_evidence():
        raise evidence_error

    original_write = Path.write_text

    def failed_stop(path, *args, **kwargs):
        if path == work / "stop":
            raise stop_error
        return original_write(path, *args, **kwargs)

    # A real absent stop keeps the original service alive until wait_for's
    # actual timeout. This exercises the Python 3.10 TimeoutError branch and
    # serve-only TERM cleanup, rather than faking a timeout receipt.
    with monkeypatch.context() as faults:
        faults.setattr(Path, "write_text", failed_stop)
        with pytest.raises(FileNotFoundError) as caught:
            async with fixture_scope(
                work, close=failed_close, evidence=failed_evidence
            ) as owned:
                service = owned

                async def ready():
                    while not (work / "ready").exists():
                        assert owned.returncode is None
                        await asyncio.sleep(0.005)

                await asyncio.wait_for(ready(), 10)
                collector_pid = json.loads((work / "collector.json").read_text())["pid"]
                assert collector_pid > 0 and service.pid > 0
                file_sha256(work / "missing-host")
    assert close_error in caught.value.cleanup_errors
    assert evidence_error in caught.value.cleanup_errors
    assert stop_error in caught.value.cleanup_errors
    assert any(
        isinstance(error, asyncio.TimeoutError) for error in caught.value.cleanup_errors
    )
    assert service.returncode == 128 + 15
    for pid in (service.pid, collector_pid):
        with pytest.raises(ProcessLookupError):
            os.kill(pid, 0)
    assert not (work / "stop").exists()

    # Fail Thread.start after the real collector/model socket exists. A
    # never-started model must not be joined or passed to server.shutdown().
    startup_work = Path(
        tempfile.mkdtemp(prefix="python-startup-fault-", dir=ROOT)
    ).resolve()
    await helper(startup_work)
    startup_code = """
import runpy, sys
from pathlib import Path
module = runpy.run_path(sys.argv[1])
original_read = Path.read_text
collector_reads = 0
def publication_read(path, *args, **kwargs):
    global collector_reads
    if path == Path(sys.argv[2]) / "collector.json":
        collector_reads += 1
        if collector_reads == 1:
            # The real collector publishes with create/truncate then write.
            # Reproduce the actual empty-read window without replacing it.
            return ""
    return original_read(path, *args, **kwargs)
Path.read_text = publication_read
def fail_start(_thread):
    assert collector_reads >= 2, "collector publication was not observed complete"
    raise RuntimeError("controlled model thread startup failure")
module["threading"].Thread.start = fail_start
module["serve"](Path(sys.argv[2]))
"""
    startup = await asyncio.create_subprocess_exec(
        sys.executable,
        "-c",
        startup_code,
        str(FIXTURE),
        str(startup_work),
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )
    try:
        _, stderr = await asyncio.wait_for(startup.communicate(), 5)
    finally:
        if startup.returncode is None:
            startup.terminate()
            try:
                await asyncio.wait_for(startup.wait(), 5)
            except asyncio.TimeoutError:
                startup.kill()
                await asyncio.wait_for(startup.wait(), 5)
    assert startup.returncode == 1
    assert b"controlled model thread startup failure" in stderr
    startup_collector = json.loads((startup_work / "collector.json").read_text())["pid"]
    for pid in (startup.pid, startup_collector):
        with pytest.raises(ProcessLookupError):
            os.kill(pid, 0)
