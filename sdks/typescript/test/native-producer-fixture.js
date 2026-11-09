// 仅供真实测试callback producer报告自己的观察，不是SDK运行时或完成推断器。
export function nativeProducerFixture(sdk, call) {
  const writer = sdk.streamWriter(call.stream);
  const execution = call.context?.execution_id ?? call.invocation_id;
  const request = call.context?.message_id ?? call.invocation_id;
  const status = (kind, sequence, phase) => ({kind: "agent", value: {
    event: "execution_status", data: {
      execution_id: execution, request_id: request, sequence: String(sequence),
      phase, status: kind,
    },
  }});
  return {
    async start() { await writer.chunk(status("started", 0, "Thinking")); },
    async failed(error) { await writer.failed(error); },
    async finish(text, cancelled = false) {
      if (!cancelled) await writer.chunk({kind: "agent", value: {
        event: "final_answer", data: {text},
      }});
      await writer.complete(status(cancelled ? "cancelled" : "settled", 1, "Terminal"));
    },
  };
}
