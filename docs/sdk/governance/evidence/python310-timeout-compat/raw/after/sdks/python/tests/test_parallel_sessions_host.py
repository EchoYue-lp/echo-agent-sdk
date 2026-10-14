import asyncio
import os
from contextlib import asynccontextmanager
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Host E2E binary is missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]
TESTING = Path(__file__).resolve().parents[2] / "shared" / "testing"


async def barrier(predicate):
    async def wait_for_condition_1():
        while not predicate():
            await asyncio.sleep(0.01)

    await asyncio.wait_for(wait_for_condition_1(), 10)


@asynccontextmanager
async def fixture(tmp_path):
    model = await asyncio.create_subprocess_exec(
        "python3", str(TESTING / "parallel_session_fixture.py"), "--work", str(tmp_path)
    )
    try:
        await barrier(lambda: (tmp_path / "ready").exists())
        async with await EchoAgentClient.spawn(
            HOST, "--config", str(tmp_path / "host.json")
        ) as sdk:
            yield sdk
    finally:
        if model.returncode is None:
            model.terminate()
        await asyncio.wait_for(model.wait(), 5)


async def collect(run, started):
    values = []
    async for item in run.events:
        assert "gap" not in item
        assert item["stream"] == run.stream.to_dict()
        assert item["envelope"]["stream_id"] == run.stream.id
        values.append(item["envelope"])
        started.set()
    return values


async def verify_journal(run, observed):
    replay = (await run.replay("0", 512))["events"]
    assert [int(item["sequence"]) for item in observed] == list(
        range(1, len(replay) + 1)
    )
    assert [item["event_id"] for item in observed] == [
        item["event_id"] for item in replay
    ]
    assert len({item["event_id"] for item in observed}) == len(observed)
    assert (
        sum(
            item["payload"]["event_type"] in {"final_answer", "cancelled", "error"}
            for item in observed
        )
        == 1
    )


async def test_normal_live_terminal_natural_eof(tmp_path):
    async with fixture(tmp_path) as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session()
        run = await session.start_run("lane-normal 中文🌊")
        started = asyncio.Event()
        consumer = asyncio.create_task(collect(run, started))
        await asyncio.wait_for(started.wait(), 5)
        await barrier(lambda: (tmp_path / "parked-normal").exists())
        (tmp_path / "release-normal").touch()
        assert (await run.wait(timeout=10))["settled"]
        observed = await asyncio.wait_for(consumer, 5)
        assert observed[-1]["payload"]["event_type"] == "final_answer"
        await verify_journal(run, observed)
        assert (tmp_path / "completed-normal").exists()
        await session.close()
        await agent.close()


async def test_parallel_session_cancel_close_does_not_end_other_feed(tmp_path):
    async with fixture(tmp_path) as sdk:
        agent_a, agent_b = await sdk.create_agent(), await sdk.create_agent()
        session_a, session_b = (
            await agent_a.create_session(),
            await agent_b.create_session(),
        )
        run_a = await session_a.start_run("lane-a 中文🌊")
        run_b = await session_b.start_run("lane-b 中文🌊")
        assert session_a.wire != session_b.wire
        assert run_a.stream != run_b.stream
        started_a, started_b = asyncio.Event(), asyncio.Event()
        consumer_a = asyncio.create_task(collect(run_a, started_a))
        consumer_b = asyncio.create_task(collect(run_b, started_b))
        await asyncio.wait_for(asyncio.gather(started_a.wait(), started_b.wait()), 5)
        await barrier(
            lambda: (
                (tmp_path / "parked-a").exists() and (tmp_path / "parked-b").exists()
            )
        )
        await run_a.cancel()
        assert (await run_a.wait(timeout=10))["terminal"]["status"] == "cancelled"
        observed_a = await asyncio.wait_for(consumer_a, 5)
        await verify_journal(run_a, observed_a)
        await session_a.close()
        await agent_a.close()
        assert not consumer_b.done(), (
            "closing A must not complete or fail B's event feed"
        )
        assert (await run_b.wait(timeout=0))["settled"] is False
        assert not (tmp_path / "completed-b").exists()
        (tmp_path / "release-b").touch()
        assert (await run_b.wait(timeout=10))["terminal"]["status"] == "completed"
        observed_b = await asyncio.wait_for(consumer_b, 5)
        assert observed_b[-1]["payload"]["event_type"] == "final_answer"
        await verify_journal(run_b, observed_b)
        assert {item["event_id"] for item in observed_a}.isdisjoint(
            item["event_id"] for item in observed_b
        )
        assert (tmp_path / "completed-b").exists()
        await session_b.close()
        await agent_b.close()
