"""CustomAgent 示例：原样向实现方交付完整 Message，不解析旧字符串载体。"""

from collections.abc import Awaitable, Callable, Mapping, Sequence
from typing import Any

from echo_agent_sdk import EchoAgentClient, WireHandle


async def register_message_agent(
    sdk: EchoAgentClient,
    consume: Callable[
        [Mapping[str, Any], Sequence[Mapping[str, Any]] | None, Any], Awaitable[str]
    ],
):
    async def handler(call, cancel):
        invocation = call["invocation"]
        operation = invocation["operation"]
        if operation == "agent_close":
            return {
                "outcome": "result",
                "result": {"operation": operation, "value": None},
            }
        if operation in {"agent_execute", "agent_execute_stream"}:
            message = invocation["input"]["task"]
        elif operation in {"agent_chat", "agent_chat_stream"}:
            message = invocation["input"]["message"]
        else:
            raise ValueError("unexpected message Agent operation")
        answer = await consume(message, invocation["input"].get("history"), cancel)
        if not operation.endswith("_stream"):
            return {
                "outcome": "result",
                "result": {"operation": operation, "value": answer},
            }
        writer = sdk.stream_writer(WireHandle.from_dict(call["stream"]))
        await writer.complete(
            {
                "kind": "agent",
                "value": {"event": "final_answer", "data": {"text": answer}},
            }
        )
        return {"outcome": "stream", "stream": call["stream"]}

    return await sdk.register_custom_agent(
        "message-example",
        {
            "kind": "custom_agent",
            "descriptor_version": 1,
            "name": "message-example",
            "model_name": "example",
            "system_prompt": "保留完整输入",
            "tool_names": [],
        },
        handler,
    )
