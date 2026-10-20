"""Real public steering and concurrent wait consumers, using the original Host."""

import asyncio
import json
import os
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required steering Host is missing")
FIXTURE = Path(__file__).resolve().parents[2] / "shared/testing/run_steer_fixture.py"


async def barrier(path):
    loop = asyncio.get_running_loop()
    deadline = loop.time() + 10
    while not path.exists():
        assert loop.time() < deadline, "original model did not reach barrier"
        await asyncio.sleep(0.01)


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
async def test_accepted_steering_consumed_by_model_and_two_waits_share_receipt(
    tmp_path,
):
    model = await asyncio.create_subprocess_exec(
        "python3", str(FIXTURE), "--work", str(tmp_path)
    )
    waits = []
    sdk = None
    try:
        await barrier(tmp_path / "ready")
        scenario = json.loads((tmp_path / "scenario.json").read_text())
        sdk = await EchoAgentClient.spawn(
            HOST,
            "--config",
            str(tmp_path / "host.json"),
            env={
                **os.environ,
                "NO_PROXY": "127.0.0.1,localhost",
                "no_proxy": "127.0.0.1,localhost",
            },
        )
        agent = await sdk.create_agent()
        session = await agent.create_session(str(tmp_path))
        run = await session.start_run(scenario["initial"])
        identity = {"run": run.wire.to_dict(), "stream": run.stream.to_dict()}
        await barrier(tmp_path / "parked-1")
        waits = [asyncio.create_task(run.wait()), asyncio.create_task(run.wait())]
        accepted = await run.steer(scenario["steer"])
        (tmp_path / "accepted.json").write_text(
            json.dumps(
                {
                    "identity": identity,
                    "accepted": accepted,
                    "snapshot": await run.get(),
                }
            )
        )
        assert accepted["accepted"] is True and accepted["steer_id"]
        assert all(not task.done() for task in waits)
        (tmp_path / "release-1").touch()
        await barrier(tmp_path / "parked-2")
        second = json.loads((tmp_path / "request-2.json").read_text())
        contents = [message["content"] for message in second["messages"]]
        assert all(scenario[key] in contents for key in ["initial", "draft", "steer"])
        assert all(not task.done() for task in waits)
        assert (await run.get())["status"] == "running"
        (tmp_path / "release-2").touch()
        one, two = await asyncio.gather(*waits)
        assert one == two and one["settled"] is True
        assert one["receipt"]["turn_id"] == run.wire.id
        assert one["receipt"]["final_answer"] == scenario["answer"]
        assert one["receipt"]["llm_calls"] == "2"
        assert one["terminal"]["status"] == "completed"
        snapshot = await run.get()
        assert snapshot["receipt"] == one["receipt"]
        assert snapshot["stream"] == identity["stream"]
        page = await run.replay("0", 64)
        assert (
            sum(
                event["payload"]["event_type"] == "final_answer"
                for event in page["events"]
            )
            == 1
        )
        rejected = await run.steer("late steering")
        assert rejected["accepted"] is False and rejected.get("steer_id") is None
        with pytest.raises(EchoAgentError) as error:
            await run.steer("")
        assert error.value.code == "invalid_value"
        assert error.value.operation == "_echo_agent/run/steer"
        assert await run.wait() == one
        (tmp_path / "observed.json").write_text(
            json.dumps(
                {
                    "identity": identity,
                    "accepted": accepted,
                    "rejected": rejected,
                    "waits": [one, two],
                    "snapshot": snapshot,
                    "page": page,
                    "second": second,
                },
                ensure_ascii=False,
                indent=2,
            )
        )
        print("Run steer native evidence", tmp_path)
        await session.close()
        await agent.close()
    finally:
        for number in [1, 2]:
            (tmp_path / f"release-{number}").touch()
        if sdk is not None:
            await sdk.close()
        await asyncio.gather(*waits, return_exceptions=True)
        if model.returncode is None:
            model.terminate()
        await model.wait()
