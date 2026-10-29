import asyncio
import runpy
from pathlib import Path

import pytest

from echo_agent_sdk.client import _normalize_outcome


def test_llm_request_bridge_preserves_named_choice_and_rejects_bad_shapes():
    from echo_agent_sdk import LlmChatCall

    def payload(choice, operation="llm_chat"):
        return {"invocation": {"operation": operation, "input": {"messages": [], "tool_choice": choice}}}

    for choice in [None, "auto", "none", "required", {"type": "function", "function": {"name": " 原始工具🧪 "}},
                   {"type": "function", "function": {"name": "\ufeff"}}]:
        for operation in ["llm_chat", "llm_chat_stream"]:
            call = LlmChatCall.from_payload(payload(choice, operation))
            assert call.tool_choice == choice
            assert call.request["tool_choice"] == choice
    absent = LlmChatCall.from_payload({"invocation": {"operation": "llm_chat", "input": {"messages": []}}})
    assert "tool_choice" not in absent.request
    for choice in ["future", True, [], {"type": "tool", "function": {"name": "x"}},
                   {"type": "function", "function": {"name": ""}}, {"type": "function", "function": {"name": "\u00a0"}},
                   {"type": "function", "function": {"name": 2}}, {"type": "function", "function": {"name": "x", "extra": True}},
                   {"type": "function", "function": {"name": "x"}, "extra": True}]:
        with pytest.raises(ValueError, match="tool_choice"):
            LlmChatCall.from_payload(payload(choice))


@pytest.mark.asyncio
async def test_native_forwarding_waits_for_source_eof_and_never_mints_status():
    forward = runpy.run_path(
        str(Path(__file__).resolve().parents[1] / "examples/custom_agent_message.py")
    )["forward_native_output"]
    sent = []
    ended = False

    class Writer:
        async def chunk(self, value):
            sent.append(value)

        async def complete(self, value):
            assert ended
            sent.append(value)

    native = {
        "event": "execution_status",
        "data": {
            "execution_id": "original-execution",
            "sequence": "18446744073709551615",
            "phase": "Terminal",
            "status": "settled",
        },
    }

    async def source():
        nonlocal ended
        yield {"event": "final_answer", "data": {"text": "原确认🙂"}}
        assert len(sent) == 1
        yield native
        assert len(sent) == 1
        ended = True

    await forward(Writer(), source())
    assert sent[-1] == {"kind": "agent", "value": native}

    async def incomplete():
        yield {"event": "final_answer", "data": {"text": "only notification"}}

    with pytest.raises(ValueError, match="without its runtime terminal"):
        await forward(Writer(), incomplete())


@pytest.mark.asyncio
async def test_raw_custom_agent_example_preserves_message_history_and_outcome():
    register = runpy.run_path(
        str(Path(__file__).resolve().parents[1] / "examples/custom_agent_message.py")
    )["register_message_agent"]

    class SDK:
        async def register_custom_agent(self, _name, _descriptor, handler):
            self.handler = handler
            return "registered"

        def stream_writer(self, stream):
            class Writer:
                async def complete(inner, value):
                    completed.append((stream, value))

            return Writer()

    sdk = SDK()
    completed = []
    seen = []
    message = {
        "role": "tool",
        "tool_result_is_error": False,
        "provider_response": {
            "protocol": "responses",
            "provider": "fixture",
            "model": {"kind": "null"},
            "stop_reason": {"kind": "string", "value": "future-stop"},
            "usage": {
                "kind": "map",
                "value": [
                    {
                        "key": {"kind": "string", "value": "tokens"},
                        "value": {"kind": "u64", "value": "18446744073709551615"},
                    },
                    {
                        "key": {"kind": "string", "value": "text"},
                        "value": {"kind": "string", "value": "00123"},
                    },
                ],
            },
        },
        "name": "shell",
        "tool_call_id": "call-🧪",
        "content": {"kind": "map", "value": []},
        "approved_assistant_blocks": [],
    }

    async def consume(value, history, cancel):
        assert value is message
        assert not cancel.is_set()
        seen.append(history)
        return "处理完整输入🧪"

    await register(sdk, consume)
    stream = {"id": "original-stream", "generation": "1", "kind": "stream"}
    for operation in (
        "agent_execute",
        "agent_chat",
        "agent_execute_stream",
        "agent_chat_stream",
    ):
        for history in (None, [], [message]):
            key = "task" if operation.startswith("agent_execute") else "message"
            result = await sdk.handler(
                {
                    "invocation": {
                        "operation": operation,
                        "input": {key: message, "history": history},
                    },
                    "stream": stream,
                },
                asyncio.Event(),
            )
            assert seen[-1] is history
            assert _normalize_outcome(result) == result
            assert result == (
                {"outcome": "stream", "stream": stream}
                if operation.endswith("_stream")
                else {
                    "outcome": "result",
                    "result": {"operation": operation, "value": "处理完整输入🧪"},
                }
            )
    assert len(completed) == 6
    count = len(seen)
    result = await sdk.handler(
        {"invocation": {"operation": "agent_close", "input": None}}, asyncio.Event()
    )
    assert _normalize_outcome(result) == {
        "outcome": "result",
        "result": {"operation": "agent_close", "value": None},
    }
    assert len(seen) == count
    with pytest.raises(ValueError):
        await sdk.handler(
            {"invocation": {"operation": "not-an-agent", "input": {}}}, asyncio.Event()
        )
