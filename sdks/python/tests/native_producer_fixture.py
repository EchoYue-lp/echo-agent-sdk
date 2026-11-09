"""真实测试 callback 的原生观察值；不属于 SDK API 或执行完成推断。"""

from echo_agent_sdk import WireHandle


class NativeProducerFixture:
    def __init__(self, sdk, call):
        self.writer = sdk.stream_writer(WireHandle.from_dict(call["stream"]))
        context = call.get("context") or {}
        self.execution = context.get("execution_id") or call["invocation_id"]
        self.request = context.get("message_id") or call["invocation_id"]

    def status(self, kind, sequence, phase):
        return {
            "kind": "agent",
            "value": {
                "event": "execution_status",
                "data": {
                    "execution_id": self.execution,
                    "request_id": self.request,
                    "sequence": str(sequence),
                    "phase": phase,
                    "status": kind,
                },
            },
        }

    async def start(self):
        await self.writer.chunk(self.status("started", 0, "Thinking"))

    async def failed(self, error):
        await self.writer.failed(error)

    async def finish(self, text, cancelled=False):
        if not cancelled:
            await self.writer.chunk(
                {
                    "kind": "agent",
                    "value": {"event": "final_answer", "data": {"text": text}},
                }
            )
        await self.writer.complete(
            self.status("cancelled" if cancelled else "settled", 1, "Terminal")
        )
