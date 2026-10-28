import asyncio
import runpy
from pathlib import Path

import pytest
from echo_agent_sdk.client import _normalize_outcome


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
