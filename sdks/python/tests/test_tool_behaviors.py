import asyncio
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required tool behavior Host binary missing")
FIXTURE = (
    Path(__file__).resolve().parents[2] / "shared/testing/tool_behavior_fixture.py"
)


async def _run_tool_story(media=False):
    work = Path(tempfile.mkdtemp(prefix="sdk-py-tool-behaviors-"))
    fixture = await asyncio.create_subprocess_exec(
        sys.executable,
        str(FIXTURE),
        "--work",
        str(work),
        *(["--media"] if media else []),
    )
    sdk = None
    passed = False
    try:

        async def prepared():
            while not (work / "ready").exists():
                assert fixture.returncode is None, "fixture preparation failed"
                await asyncio.sleep(0.01)

        await asyncio.wait_for(prepared(), timeout=10)
        scenario = json.loads((work / "scenario.json").read_text())
        sdk = await EchoAgentClient.spawn(
            HOST,
            "--config",
            str(work / "host.json"),
            env={**os.environ, "TMPDIR": str(work / "artifact-temp")},
        )
        agent = await sdk.create_agent()
        sessions = {
            name: await agent.create_session(cwd)
            for name, cwd in scenario["sessions"].items()
        }
        results = []
        for step in scenario["steps"]:
            session = sessions[step["session"]]
            if step.get("action") == "close":
                await session.close()
                results.append({"id": step["id"], "outcome": "closed"})
                continue
            try:
                value = await sdk.call(
                    step["operation"], session.wire, step["arguments"]
                )
                results.append({"id": step["id"], "outcome": "result", "value": value})
            except EchoAgentError as error:
                results.append(
                    {
                        "id": step["id"],
                        "outcome": "error",
                        "code": error.code,
                        "message": str(error),
                        "operation": error.operation,
                        "retryable": error.retryable,
                        "details": error.details,
                    }
                )
        await sessions["b"].close()
        await agent.close()
        await sdk.close()
        sdk = None
        (work / "results.json").write_text(json.dumps(results, ensure_ascii=False))
        verified = await asyncio.create_subprocess_exec(
            sys.executable,
            str(FIXTURE),
            "--work",
            str(work),
            "--verify",
            str(work / "results.json"),
            *(["--media"] if media else []),
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )
        output, diagnostics = await verified.communicate()
        assert verified.returncode == 0, (
            output.decode() + diagnostics.decode() + f"\nEvidence: {work}"
        )
        evidence = json.loads(output)
        if media:
            assert evidence["successful_operations"] == 12
        else:
            assert evidence["capability_families"] == 6
        assert evidence["native_calls"] == len(scenario["steps"])
        print("Tool behavior verified: " + json.dumps(evidence))
        if media:
            print(f"Accepted media evidence: {work}")
        passed = True
    finally:
        if sdk is not None:
            await sdk.close()
        fixture.terminate()
        await asyncio.wait_for(fixture.wait(), timeout=5)
        if passed and not media:
            import shutil

            shutil.rmtree(work)
        elif not passed:
            print(f"Tool behavior evidence retained: {work}", file=sys.stderr)


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host binary required")
async def test_six_tool_families_preserve_results_failures_and_local_effects():
    await _run_tool_story()


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host binary required")
async def test_accepted_non_text_media_preserves_parsed_files_pixels_and_failures():
    await _run_tool_story(media=True)
