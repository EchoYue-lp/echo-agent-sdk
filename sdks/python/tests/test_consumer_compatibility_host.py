import asyncio
import copy
import json
import os
import pathlib
import sys
import tempfile

import pytest

from echo_agent_sdk import EchoAgentClient, WireHandle, from_wire

ROOT = pathlib.Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sdks/shared/testing"))
from gap_ack_fixture import canonical_replay

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required native Host missing")


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
@pytest.mark.parametrize("protocol", ["responses", "anthropic", "chat_completions"])
@pytest.mark.parametrize("streaming", [False, True])
async def test_consumer_compatibility_whole_loop(protocol, streaming):
    runtime_root = pathlib.Path(os.environ["ECHO_AGENT_SDK_CONSUMER_ROOT"]).resolve()
    assert runtime_root.relative_to(ROOT.parent.resolve()) != pathlib.Path(".")
    runtime_root.mkdir(parents=True, exist_ok=True)
    work = pathlib.Path(tempfile.mkdtemp(prefix="python-", dir=runtime_root))
    spec = json.loads(
        (ROOT / "sdks/shared/testing/consumer_compatibility_host.json").read_text(
            encoding="utf-8"
        )
    )
    spec = json.loads(json.dumps(spec[protocol]).replace("__WORK__", str(work)))
    (work / "fixture.txt").write_text("actual-tool-a", encoding="utf-8")
    producers = []
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
    try:
        async with await EchoAgentClient.spawn(
            str(host), "--config", str(work / "host.json")
        ) as sdk:

            async def handler(call, cancel):
                requests.append(copy.deepcopy(call))
                assert len(requests) <= len(spec["responses"])
                value = copy.deepcopy(spec["responses"][len(requests) - 1])
                if streaming:
                    writer = sdk.stream_writer(WireHandle.from_dict(call["stream"]))
                    message = value["message"]
                    chunk = {
                        "role": "assistant",
                        "content": from_wire(message["content"]),
                        "assistant_blocks": message["assistant_blocks"],
                    }
                    if "reasoning_blocks" in message:
                        chunk["reasoning_blocks"] = message["reasoning_blocks"]
                    if "tool_calls" in message:
                        chunk["tool_calls"] = [
                            {
                                "index": i,
                                "id": tool["id"],
                                "call_type": tool["call_type"],
                                "provider_source": tool["provider_source"],
                                "function": {
                                    "name": tool["function_name"],
                                    "arguments": tool["arguments"],
                                },
                            }
                            for i, tool in enumerate(message["tool_calls"])
                        ]

                    async def produce():
                        await writer.chunk({"kind": "llm", "value": chunk})
                        await writer.complete(
                            {
                                "kind": "llm",
                                "value": {"finish_reason": value["finish_reason"]},
                            }
                        )

                    producers.append(asyncio.create_task(produce()))
                    return {"outcome": "stream", "stream": call["stream"]}
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
                    "supports_streaming": streaming,
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
            await asyncio.gather(*producers)
            replay = await run.replay("0", 512)
            assert (
                receipt["settled"] is True
                and receipt["terminal"]["status"] == "completed"
            )
            assert len(requests) == spec["expected_model_calls"]
            (work / "observed.json").write_text(
                json.dumps(
                    {
                        "requests": requests,
                        "live": live,
                        "replay": replay,
                        "receipt": receipt,
                    }
                ),
                encoding="utf-8",
            )
            assert canonical_replay([v["envelope"] for v in live]) == canonical_replay(
                replay["events"]
            )
            assert live[-1]["envelope"]["payload"]["event_type"] == "execution_status"
            assert (
                from_wire(live[-1]["envelope"]["payload"]["data"])["status"]
                == "settled"
            )
            blocks = [
                from_wire(v["envelope"]["payload"]["data"])["block"]
                for v in live
                if v["envelope"]["payload"]["event_type"] == "assistant_message"
            ]
            assert [v["phase"] for v in blocks] == spec["expected_phases"]
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
            tool_results = [
                message
                for message in requests[1]["invocation"]["input"]["messages"]
                if message["role"] == "tool"
            ]
            for call_id, is_error in [("call-a", False), ("call-b", True)]:
                result = next(
                    message
                    for message in tool_results
                    if message.get("tool_call_id") == call_id
                )
                assert result["tool_result_is_error"] is is_error
            assert any(
                spec["reasoning"] in message.get("reasoning_blocks", [])
                for message in requests[2]["invocation"]["input"]["messages"]
            )
            assert (
                sum(
                    item["envelope"]["payload"]["event_type"] == "tool_result"
                    for item in live
                )
                == 2
            )
            await session.close()
            await agent.close()
            await registration.close()
    finally:
        for producer in producers:
            if not producer.done():
                producer.cancel()
        if producers:
            await asyncio.gather(*producers, return_exceptions=True)
