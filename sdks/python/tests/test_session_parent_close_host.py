"""Real ACK-paused delivery closes locally without inventing terminal facts."""

import asyncio
import copy
import json
import os
import runpy
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required parent-close Host is missing")
FIXTURE = (
    Path(__file__).resolve().parents[2]
    / "shared/testing/session_run_idempotency_fixture.py"
)


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
@pytest.mark.parametrize("window", [1, None], ids=["paused1", "default128"])
async def test_real_host_parent_close_ends_paused_child_and_preserves_other_session(
    tmp_path, window
):
    runpy.run_path(str(FIXTURE))["prepare"](tmp_path)
    config = json.loads((tmp_path / "host.json").read_text())
    if window is not None:
        config["sdk_profile"]["limits"] = {"max_outstanding_live_events": window}
    (tmp_path / "host.json").write_text(json.dumps(config))
    spec = json.loads((tmp_path / "scenario.json").read_text())
    sdk = await EchoAgentClient.spawn(HOST, "--config", str(tmp_path / "host.json"))
    try:

        async def provider(_call, _cancel):
            return {
                "outcome": "result",
                "result": {
                    "operation": "llm_chat",
                    "value": copy.deepcopy(spec["response"]),
                },
            }

        registration = await sdk.register_llm_client(
            "parent-close-native", spec["descriptor"], provider
        )
        agent = await sdk.create_agent()
        parent = await agent.create_session()
        independent = await agent.create_session()
        run = await parent.start_run("ACK-paused child 中文🌊")
        iterator = run.events
        settled = await asyncio.wait_for(run.wait(), 5)
        assert settled["settled"] and settled["terminal"]["status"] == "completed"
        await asyncio.wait_for(parent.close(), 5)
        events = await asyncio.wait_for(_collect(iterator), 5)
        assert events, "parent disposal must retain buffered facts"
        if window == 1:
            assert any("gap" in event for event in events)
        assert (
            sum(
                event.get("envelope", {}).get("payload", {}).get("event_type")
                in {"final_answer", "cancelled", "error"}
                for event in events
            )
            <= 1
        )
        with pytest.raises(EchoAgentError, match="Run handle is closed"):
            await run.get()
        with pytest.raises(EchoAgentError) as error:
            await sdk.request("echo_agent/run/get", {"run": run.wire.to_dict()})
        assert error.value.code == "closed_handle"
        await run.close()
        other = await independent.start_run("another Session still works")
        other_settled = await asyncio.wait_for(other.wait(), 5)
        assert (
            other_settled["settled"]
            and other_settled["terminal"]["status"] == "completed"
        )
        await independent.close()
        await agent.close()
        await registration.close()
        (tmp_path / "observed.json").write_text(
            json.dumps(
                {
                    "window": window or 128,
                    "events": events,
                    "independent": other_settled,
                }
            )
        )
        print("parent-close real Host evidence:", tmp_path)
    finally:
        await sdk.close()


async def _collect(iterator):
    return [event async for event in iterator]
