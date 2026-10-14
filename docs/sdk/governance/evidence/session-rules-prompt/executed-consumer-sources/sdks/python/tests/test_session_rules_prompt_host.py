"""Default Session cwd rules must reach the real model request, without resolve calls."""

import asyncio
import json
import os
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient

HOST = os.environ.get("ECHO_AGENT_SDK_HOST", "").strip()
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Session rules Host missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]
FIXTURE = (
    Path(__file__).resolve().parents[2]
    / "shared/testing/session_rules_prompt_fixture.py"
)


async def helper(work, *args):
    process = await asyncio.create_subprocess_exec(
        "python3",
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


async def test_session_cwd_rules_reach_actual_model_and_new_session(tmp_path):
    work = tmp_path.resolve()
    await helper(work)
    model = await asyncio.create_subprocess_exec(
        "python3", str(FIXTURE), "--work", str(work), "--serve"
    )
    rows = []
    try:
        async with asyncio.timeout(10):
            while not (work / "ready").exists():
                assert model.returncode is None
                await asyncio.sleep(0.01)
        async with await EchoAgentClient.spawn(
            HOST, "--config", str(work / "host.json")
        ) as sdk:
            agent = await sdk.create_agent()
            for step in json.loads((work / "scenario.json").read_text()):
                if step["id"] == "a2":
                    await helper(work, "--mutate")
                session = await agent.create_session(str(work / step["cwd"]))
                run = await session.start_run(step["user"])
                assert (await run.wait(timeout=15))["settled"]
                rows.append(
                    {
                        "id": step["id"],
                        "outcome": await run.outcome_status(),
                        "replay": await run.replay(),
                    }
                )
                await session.close()
            await agent.close()
        (work / "native-results.json").write_text(json.dumps(rows, ensure_ascii=False))
        proof = json.loads(await helper(work, "--verify"))
        assert proof["cases"] == 3
        print(json.dumps({"work": str(work), **proof}, ensure_ascii=False))
    finally:
        if model.returncode is None:
            model.terminate()
        await asyncio.wait_for(model.wait(), 5)
