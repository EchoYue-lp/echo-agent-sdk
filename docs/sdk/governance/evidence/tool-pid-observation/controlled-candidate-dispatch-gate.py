from __future__ import annotations

import asyncio
import ast
import hashlib
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

SDK = Path("/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance")
ADMIN = Path("/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-linux-tool-pid-investigation")
HOST = Path("/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/binaries/echo-sdk-00782-git-branch-rustls046-host")
sys.path.insert(0, str(SDK / "sdks/python/src"))
sys.path.insert(0, str(SDK / "sdks/shared/testing"))
from echo_agent_sdk import EchoAgentClient
import tool_value_effect_fixture as fixture
from file_sha256 import file_sha256

source = SDK / "sdks/python/tests/test_tool_value_effect_host.py"
parsed = ast.parse(source.read_text())
original_processes = next(node for node in parsed.body if isinstance(node, ast.FunctionDef) and node.name == "processes")
candidate_observer = next(node for node in ast.walk(parsed) if isinstance(node, ast.AsyncFunctionDef) and node.name == "observe_owned_producer")
namespace = {"subprocess": subprocess}
exec(compile(ast.Module(body=[original_processes], type_ignores=[]), str(source), "exec"), namespace)
processes = namespace["processes"]
namespace.update({"asyncio": asyncio, "processes": processes})
exec(compile(ast.Module(body=[candidate_observer], type_ignores=[]), str(source), "exec"), namespace)
observe_owned_producer = namespace["observe_owned_producer"]


def raw_owned_ps(work, wide=False):
    args = ["ps", *(["-ww"] if wide else []), "-axo", "pid=,ppid=,command="]
    result = subprocess.run(args, capture_output=True, text=True)
    assert result.returncode == 0, result.stderr
    return {"argv": args, "rows": [line for line in result.stdout.splitlines() if str(work / "tail.txt") in line], "stderr": result.stderr}


async def main():
    work = ADMIN / "controlled-candidate-real-host-work"
    assert not work.exists()
    fixture.prepare(work, "http://127.0.0.1:1")
    assert file_sha256(HOST) == "0f1ecf23a2875621caf5128327046cbf6592ce61665c6c6872aed33fdf1931df"
    step = next(row for row in json.loads((work / "scenario.json").read_text())["rows"] if row["id"] == "shell-signal")
    assert step["arguments"] == ["tail -f " + str(work / "tail.txt"), 5]
    gate = asyncio.Event()
    entered = asyncio.Event()
    facts = {"platform": sys.platform, "scriptRole": "controlled request-dispatch queue counterexample, not reproduction of Linux CI root cause", "host": str(HOST), "hostSHA256": file_sha256(HOST), "sourceTestSHA256": hashlib.sha256(source.read_bytes()).hexdigest(), "step": step, "timestamps": {}, "oldObservations": []}
    sdk = None
    invocation = None
    observer = None
    started = time.monotonic_ns()
    stamp = lambda: time.monotonic_ns() - started
    try:
        sdk = await EchoAgentClient.spawn(str(HOST), "--config", str(work / "host.json"))
        agent = await sdk.create_agent()
        session = await agent.create_session(str(work))
        facts["protectedBefore"] = fixture.snapshot(work)

        async def publicly_dispatched_invocation():
            facts["timestamps"]["queuedBeforePublicInvokeNs"] = stamp()
            entered.set()
            await gate.wait()
            facts["timestamps"]["actualSessionInvokeAdmissionNs"] = stamp()
            return await session.invoke(step["operation"], step["arguments"])

        invocation = asyncio.create_task(publicly_dispatched_invocation())
        await entered.wait()
        # Direct actual source coroutine under its unchanged test-only 10s watchdog.
        namespace["work"] = work
        observer = asyncio.create_task(asyncio.wait_for(observe_owned_producer(invocation), 10))
        pids = []
        observation_started = stamp()
        for iteration in range(50):
            pids = processes(work)
            facts["oldObservations"].append({"iteration": iteration, "elapsedNs": stamp(), "taskDone": invocation.done(), "pids": pids})
            if pids or invocation.done():
                break
            await asyncio.sleep(0.01)
        facts["timestamps"]["oldObservationCompletedNs"] = stamp()
        facts["oldNominalSleepBudgetNs"] = 500000000
        facts["oldActualObservationElapsedNs"] = stamp() - observation_started
        facts["oldTaskPending"] = not invocation.done()
        facts["oldDefaultPS"] = raw_owned_ps(work)
        facts["oldWidePS"] = raw_owned_ps(work, True)
        try:
            assert len(pids) == 1, pids
        except AssertionError as error:
            facts["oldAssertion"] = {"failed": True, "exception": type(error).__name__, "value": str(error)}
        else:
            raise AssertionError("old PID assertion did not reject the queued invocation")
        assert not invocation.done() and not pids and not facts["oldDefaultPS"]["rows"]
        assert not observer.done(), "candidate observer ended while original invocation still queued"
        facts["candidateObserverPendingAtOldFailure"] = True
        gate.set()
        facts["timestamps"]["requestGateReleasedNs"] = stamp()
        pids = await observer
        facts["timestamps"]["ownedProducerObservedNs"] = stamp()
        facts["observedPIDs"] = pids
        facts["beforeSignalDefaultPS"] = raw_owned_ps(work)
        facts["beforeSignalWidePS"] = raw_owned_ps(work, True)
        assert len(pids) == 1, pids
        assert processes(work) == pids, "owned tail changed before SIGTERM"
        facts["signal"] = {"name": "SIGTERM", "pid": pids[0]["pid"]}
        os.kill(pids[0]["pid"], signal.SIGTERM)
        result = await asyncio.wait_for(asyncio.shield(invocation), 5)
        facts["timestamps"]["originalInvocationSettledNs"] = stamp()
        facts["result"] = result
        facts["afterPIDs"] = processes(work)
        facts["protectedAfter"] = fixture.snapshot(work)
        assert facts["protectedBefore"] == facts["protectedAfter"]
        assert facts["afterPIDs"] == []
        assert result["kind"] == {"kind": "command_output", "exit_code": "-1"}, result
        assert result["failure"]["category"] == "permanent" and "timeout" not in result.get("error", "").lower(), result
        facts["conditionObservationPreservedStrictPIDAndEffects"] = True
        await session.close()
        await agent.close()
    finally:
        gate.set()
        if observer is not None and not observer.done():
            observer.cancel()
            await asyncio.gather(observer, return_exceptions=True)
        if invocation is not None and not invocation.done():
            invocation.cancel()
            await asyncio.gather(invocation, return_exceptions=True)
        if sdk is not None:
            await sdk.close()
        facts["timestamps"]["clientClosedNs"] = stamp()
        facts["finalPIDs"] = processes(work)
        (ADMIN / "controlled-candidate-dispatch-results.json").write_text(json.dumps(facts, ensure_ascii=False, indent=2) + "\n")
    assert facts["finalPIDs"] == []
    print(json.dumps({"oldAssertionFailed": facts["oldAssertion"]["failed"], "oldTaskPending": facts["oldTaskPending"], "oldObservationElapsedNs": facts["oldActualObservationElapsedNs"], "actualProducerCount": len(facts["observedPIDs"]), "actualToolResult": facts["result"], "physicalPIDsAfter": facts["afterPIDs"], "protectedBytesUnchanged": facts["protectedBefore"] == facts["protectedAfter"], "LinuxNativeExecuted": False, "candidateObserverPendingAtOldFailure": facts["candidateObserverPendingAtOldFailure"]}, ensure_ascii=False))


asyncio.run(main())
