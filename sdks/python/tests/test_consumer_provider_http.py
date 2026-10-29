"""真实Python客户端→stdio Host→实际provider HTTP；资格按具名批次记录。"""

import asyncio
import copy
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, from_wire

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Host missing")
ROOT = Path(__file__).resolve().parents[3]
HELPER = ROOT / "sdks/shared/testing/consumer_provider_http.py"


async def barrier(path, child):
    async def observe():
        while not path.exists():
            if child.returncode is not None:
                raise RuntimeError(f"HTTP fixture exited {child.returncode}")
            await asyncio.sleep(0.01)

    await asyncio.wait_for(observe(), 10)


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="actual Host required")
@pytest.mark.parametrize("protocol", ["responses", "anthropic", "chat_completions"])
@pytest.mark.parametrize("mode", ["success", "stop_continue", "cancel", "eof"])
async def test_actual_provider_http(protocol, mode):
    runtime = Path(os.environ["ECHO_AGENT_SDK_CONSUMER_ROOT"]).resolve()
    assert runtime.relative_to(ROOT.parent) != Path(".")
    runtime.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix=f"http-py-{protocol}-{mode}-", dir=runtime))
    args = [str(HELPER), "--work", str(work), "--protocol", protocol, "--mode", mode]
    if os.environ.get("ECHO_AGENT_SDK_CONSUMER_IMAGE") == "1":
        args.append("--image")
    stderr = (work / "fixture.stderr").open("wb")
    child = await asyncio.create_subprocess_exec(
        sys.executable, *args, stdout=asyncio.subprocess.DEVNULL, stderr=stderr
    )
    facts = {"tool_calls": [], "stop_calls": 0, "public": [], "live": [], "replay": []}
    collecting = None
    try:
        await barrier(work / "ready", child)
        spec = json.loads((work / "spec.json").read_text())
        async with await EchoAgentClient.spawn(
            HOST, "--config", str(work / "host.json")
        ) as sdk:
            registrations = []
            for name in ["consumer_image", "consumer_error"]:

                async def tool(call, cancel, name=name):
                    operation = call["invocation"]["operation"]
                    if operation == "tool_validate_parameters":
                        return {
                            "outcome": "result",
                            "result": {"operation": operation, "value": None},
                        }
                    assert operation == "tool_execute"
                    facts["tool_calls"].append(name)
                    (work / f"{name}-effect").write_text(
                        str(facts["tool_calls"].count(name))
                    )
                    return {
                        "outcome": "result",
                        "result": {
                            "operation": operation,
                            "value": copy.deepcopy(spec["results"][name]),
                        },
                    }

                registrations.append(
                    await sdk.register_extension(
                        "tool", name, dict(spec["tool_descriptor"], name=name), tool
                    )
                )
            if mode == "stop_continue":

                async def hook(call, cancel):
                    result = copy.deepcopy(spec["hook_allow"])
                    if facts["stop_calls"] == 0:
                        result["continue_reason"] = "continue original execution"
                    facts["stop_calls"] += 1
                    return {
                        "outcome": "result",
                        "result": {
                            "operation": call["invocation"]["operation"],
                            "value": result,
                        },
                    }

                registrations.append(
                    await sdk.register_extension(
                        "hook",
                        "continue-once",
                        {"kind": "hook", "descriptor_version": 1, "events": ["Stop"]},
                        hook,
                    )
                )
            agent = await sdk.create_agent()
            session = await agent.create_session(str(work))
            run = await session.start_run(
                "two actual tool results and final validation"
            )

            async def collect():
                async for item in run.events:
                    assert "envelope" in item
                    envelope = item["envelope"]
                    facts["live"].append(envelope)
                    kind = envelope["payload"]["event_type"]
                    # 生命周期unit事件没有data；只解码现有四类公开正文/状态载荷。
                    if kind not in {
                        "assistant_message",
                        "final_answer",
                        "execution_status",
                    }:
                        continue
                    value = from_wire(envelope["payload"]["data"])
                    if kind == "assistant_message":
                        facts["public"].append(
                            {"kind": kind, "text": value["block"]["delta"]}
                        )
                    elif kind == "final_answer":
                        facts["public"].append({"kind": kind, "text": value})
                    elif kind == "execution_status":
                        facts["public"].append(
                            {
                                "kind": kind,
                                "status": value["status"],
                                "phase": value["phase"],
                                "execution_id": value["execution_id"],
                            }
                        )

            collecting = asyncio.create_task(collect())
            if mode == "cancel":
                await barrier(work / "cancel-ready", child)
                await run.cancel()
            facts["receipt"] = await run.wait(timeout=20)
            await asyncio.wait_for(collecting, 20)
            facts["replay"] = (await run.replay("0", 512))["events"]
            (work / "facts.json").write_text(json.dumps(facts, default=str))
            verifier = await asyncio.create_subprocess_exec(
                sys.executable,
                *args,
                "--verify",
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
            )
            out, error = await asyncio.wait_for(verifier.communicate(), 10)
            assert verifier.returncode == 0, error.decode()
            (work / "verify.stdout").write_bytes(out)
            await run.close()
            await session.close()
            await agent.close()
            for registration in reversed(registrations):
                await registration.close()
    finally:
        if collecting is not None:
            if not collecting.done():
                collecting.cancel()
            await asyncio.gather(collecting, return_exceptions=True)
        if child.returncode is None:
            child.terminate()
        await asyncio.wait_for(child.wait(), 10)
        stderr.close()
        (work / "fixture-exit.json").write_text(json.dumps({"exit": child.returncode}))
