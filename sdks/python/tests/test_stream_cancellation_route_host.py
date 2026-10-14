"""Actual Host cancel after the original public custom-agent stream setup ACK."""

import asyncio
import os
import sys
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, WireHandle, to_wire

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
CONFIG = (
    Path(__file__).resolve().parents[2] / "shared/testing/mcp_channel_config_fixture.py"
)
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("real stream cancellation Host missing")


@pytest.mark.skipif(not HOST, reason="real Host required")
@pytest.mark.asyncio
async def test_setup_ack_retains_real_host_cancel_route(tmp_path):
    config = await asyncio.create_subprocess_exec(
        sys.executable, str(CONFIG), str(tmp_path)
    )
    assert await config.wait() == 0
    sdk = await EchoAgentClient.spawn(HOST, "--config", str(tmp_path / "host.json"))
    entered, cancelled, release, shutdown = (asyncio.Event() for _ in range(4))
    jobs, facts = [], []

    async def handler(call, cancellation):
        operation = call["invocation"]["operation"]
        if operation == "agent_close":
            return {
                "outcome": "result",
                "result": {"operation": operation, "value": None},
            }
        assert operation == "agent_execute_stream"

        async def produce():
            waits = [
                asyncio.create_task(cancellation.wait()),
                asyncio.create_task(shutdown.wait()),
            ]
            try:
                await asyncio.wait(waits, return_when=asyncio.FIRST_COMPLETED)
                if not cancellation.is_set():
                    return
                facts.append("signal")
                cancelled.set()
                await release.wait()
                await sdk.stream_writer(WireHandle.from_dict(call["stream"])).complete(
                    {"kind": "agent", "value": {"event": "cancelled"}}
                )
                facts.append("terminal-success")
            finally:
                for task in waits:
                    task.cancel()
                await asyncio.gather(*waits, return_exceptions=True)
                facts.append("finally")

        jobs.append(asyncio.create_task(produce()))
        entered.set()
        facts.append("setup-ack")
        return {"outcome": "stream", "stream": call["stream"]}

    try:
        reg = await sdk.register_extension(
            "custom_agent",
            "route-target",
            {
                "kind": "custom_agent",
                "descriptor_version": 1,
                "name": "route-target",
                "model_name": "fixture",
                "system_prompt": "fixture",
                "tool_names": [],
            },
            handler,
        )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        dispatch = await sdk.request(
            "echo_agent/subagent/dispatch",
            {
                "session": session.wire.to_dict(),
                "request": to_wire(
                    {
                        "agent_name": "route-target",
                        "task": "retain cancel after setup 中文🌊",
                        "mode_override": "sync",
                    }
                ),
                "idempotency_id": "cancel-route",
            },
        )
        await asyncio.wait_for(entered.wait(), 2)
        await sdk.request(
            "echo_agent/subagent/control",
            {"subagent": dispatch["subagent"], "action": "cancel"},
        )
        await asyncio.wait_for(cancelled.wait(), 2)
        assert facts == ["setup-ack", "signal"]
        release.set()
        await asyncio.wait_for(asyncio.gather(*jobs), 2)
        assert facts == ["setup-ack", "signal", "terminal-success", "finally"]
        await session.close()
        await reg.close()
        await agent.close()
    finally:
        release.set()
        shutdown.set()
        await asyncio.gather(*jobs, return_exceptions=True)
        await sdk.close()
