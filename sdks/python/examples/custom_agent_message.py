"""CustomAgent 示例：原样向实现方交付完整 Message，不解析旧字符串载体。"""

from collections.abc import AsyncIterable, Awaitable, Callable, Mapping, Sequence
from copy import deepcopy
from typing import Any

from echo_agent_sdk import EchoAgentClient, WireHandle


async def forward_native_output(
    writer, source: AsyncIterable[Mapping[str, Any]]
) -> None:
    """原事实逐项转运；原 source EOF 才提交原终态，不从正文或 EOF 产生状态。"""
    terminal = None
    async for event in source:
        if terminal is not None:
            raise ValueError("native producer emitted data after its runtime terminal")
        data = event.get("data")
        if (
            event.get("event") == "execution_status"
            and isinstance(data, Mapping)
            and data.get("status") in {"settled", "failed", "cancelled"}
        ):
            terminal = deepcopy(event)
        else:
            await writer.chunk({"kind": "agent", "value": event})
    if terminal is None:
        raise ValueError("native producer EOF without its runtime terminal")
    await writer.complete({"kind": "agent", "value": terminal})


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
