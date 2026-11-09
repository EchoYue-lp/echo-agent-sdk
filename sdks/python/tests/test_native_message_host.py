import asyncio
import copy
import json
import os
import pathlib

import pytest

from echo_agent_sdk import EchoAgentClient, from_wire

ROOT = pathlib.Path(__file__).resolve().parents[3]
HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required native Host missing")


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
async def test_native_messages_current_host(tmp_path):
    work = tmp_path
    spec = json.loads(
        (ROOT / "sdks/shared/testing/native_message_host_contract.json").read_text(
            encoding="utf-8"
        )
    )
    config = json.loads(
        (ROOT / "echo-sdk-host/config.sdk.example.json").read_text(encoding="utf-8")
    )
    config["sdk_profile"]["state_root"] = str(work / "state")
    config["default_agent"]["agent"]["enable_tools"] = True
    config["default_agent"]["agent"]["memory_path"] = str(work / "memory.jsonl")
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")
    host = pathlib.Path(HOST)
    requests = []
    live = []
    async with await EchoAgentClient.spawn(
        str(host), "--config", str(work / "host.json")
    ) as sdk:

        async def handler(call, cancel):
            requests.append(copy.deepcopy(call))
            assert len(requests) <= len(spec["responses"])
            return {
                "outcome": "result",
                "result": {
                    "operation": "llm_chat",
                    "value": copy.deepcopy(spec["responses"][len(requests) - 1]),
                },
            }

        registration = await sdk.register_llm_client(
            "controlled-native-python",
            {
                "kind": "llm_client",
                "descriptor_version": 1,
                "model_name": "controlled-native",
                "supports_streaming": False,
            },
            handler,
        )
        agent = await sdk.create_agent()
        session = await agent.create_session(str(work))
        run = await session.start_run("native source probe")

        async def collect():
            async for item in run.events:
                assert "envelope" in item
                live.append(item)

        collecting = asyncio.create_task(collect())
        try:
            receipt = await run.wait(timeout=10)
            await asyncio.wait_for(collecting, 10)
        finally:
            if not collecting.done():
                collecting.cancel()
            await asyncio.gather(collecting, return_exceptions=True)
        replay = await run.replay("0", 512)
        assert (
            receipt["settled"] is True and receipt["terminal"]["status"] == "completed"
        )
        assert len(requests) == spec["expected_model_calls"]
        assert [v["envelope"] for v in live] == replay["events"]
        usages = [
            from_wire(item["envelope"]["payload"]["data"])
            for item in live
            if item["envelope"]["payload"]["event_type"] == "llm_usage"
        ]
        assert len(usages) == spec["expected_model_calls"]
        for usage in usages:
            observation = usage["observation"]
            assert set(observation) == {
                "call_id",
                "attempt_id",
                "outcome",
                "request_duration_ms",
                "first_output_ms",
                "last_output_ms",
                "output_samples",
                "timing_comparable",
            }
            assert observation["call_id"] and observation["attempt_id"]
            assert observation["outcome"] == "completed"
            # 既有 u64 codec 解出规范十进制字符串；这里不改变 decoder 的公开返回类型。
            for field in ("request_duration_ms", "output_samples"):
                value = observation[field]
                assert isinstance(value, str)
                assert value == "0" or (
                    value.isascii() and value.isdecimal() and not value.startswith("0")
                )
                assert int(value) >= 0
            assert isinstance(observation["timing_comparable"], bool)
        assert live[-1]["envelope"]["payload"]["event_type"] == "execution_status"
        assert from_wire(live[-1]["envelope"]["payload"]["data"])["status"] == "settled"
        blocks = [
            from_wire(v["envelope"]["payload"]["data"])["block"]
            for v in live
            if v["envelope"]["payload"]["event_type"] == "assistant_message"
        ]
        assert [v["phase"] for v in blocks] == ["commentary", "final_answer"]
        assert blocks[-1]["delta"] == spec["final_answer"]
        statuses = [
            from_wire(v["envelope"]["payload"]["data"])
            for v in live
            if v["envelope"]["payload"]["event_type"] == "execution_status"
        ]
        assert all("adapter_generated" not in status for status in statuses)
        for request in requests:
            for message in request["invocation"]["input"]["messages"]:
                status = message.get("execution_status")
                if status is not None:
                    assert "adapter_generated" not in status
        assert sum(v["phase"] == "Terminal" for v in statuses) == 1
        assert statuses[-1]["phase"] == "Terminal"
        finals = [
            v["envelope"]["payload"]
            for v in live
            if v["envelope"]["payload"]["event_type"] == "final_answer"
        ]
        assert len(finals) == 1
        assert from_wire(finals[0]["data"]) == spec["final_answer"]
        event_types = [v["envelope"]["payload"]["event_type"] for v in live]
        final_block_index = max(
            i for i, kind in enumerate(event_types) if kind == "assistant_message"
        )
        compatibility_index = event_types.index("final_answer")
        assert final_block_index < compatibility_index < len(event_types) - 1
        assert replay["events"][-1]["payload"]["event_type"] == "execution_status"
        expected_source = spec["responses"][0]["message"]["assistant_blocks"][0][
            "provider_source"
        ]
        history_sources = [
            b.get("provider_source")
            for m in requests[1]["invocation"]["input"]["messages"]
            for b in m.get("assistant_blocks", []) or []
        ]
        assert expected_source in history_sources
        await session.close()
        await agent.close()
        await registration.close()
