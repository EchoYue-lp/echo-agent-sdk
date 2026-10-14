"""Accepted telemetry: real Agent records, local OTLP receipt and lifecycle."""

import asyncio
import inspect
import json
import os
import sys
import tempfile
from contextlib import asynccontextmanager
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
    try:
        service = await asyncio.create_subprocess_exec(
            sys.executable, str(FIXTURE), "--work", str(work), "--serve"
        )
        yield service
    except BaseException as error:  # noqa: BLE001 - re-raised after all owner cleanup
        primary = error

    async def close_sdk():
        outcome = close()
        if inspect.isawaitable(outcome):
            await asyncio.wait_for(outcome, 5)

    async def stop_service():
        if service is None:
            return
        try:
            await asyncio.wait_for(service.wait(), 5)
        except TimeoutError as error:
            failures.append(error)
            service.terminate()
            try:
                await asyncio.wait_for(service.wait(), 5)
            except TimeoutError as term_error:
                failures.append(term_error)
                service.kill()
                await asyncio.wait_for(service.wait(), 5)
        assert service.returncode == 0, service.returncode

    # Failure of any release must not skip later releases or original-PID wait.
    for cleanup in (
        lambda: (work / "release-export").write_text("cleanup actual response"),
        close_sdk,
        lambda: helper(work, "--cleanup-host"),
        lambda: (work / "stop").write_text("stop"),
        stop_service,
        evidence,
    ):
        try:
            outcome = cleanup()
            if inspect.isawaitable(outcome):
                await outcome
        except BaseException as error:  # noqa: BLE001 - re-raised after all owner cleanup
            failures.append(error)
    if primary is not None:
        if failures:
            primary.cleanup_errors = tuple(failures)
            raise primary from RuntimeError(f"telemetry cleanup failures: {failures!r}")
        raise primary
    if failures:
        raise RuntimeError(f"telemetry cleanup failures: {failures!r}") from failures[0]


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
async def test_telemetry_setup_and_cleanup_faults_reap_original_processes():
    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-cleanup-fault-", dir=ROOT)).resolve()
    await helper(work)
    service = collector_pid = None
    close_error = RuntimeError("controlled SDK close failure")
    evidence_error = OSError("controlled evidence write failure")

    async def failed_close():
        raise close_error

    def failed_evidence():
        raise evidence_error

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
    assert service.returncode == 0
    for pid in (service.pid, collector_pid):
        with pytest.raises(ProcessLookupError):
            os.kill(pid, 0)
    assert (work / "stop").exists()
