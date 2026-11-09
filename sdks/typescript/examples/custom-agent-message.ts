import type { CustomAgentInvocation, EchoAgentClient, ExtensionInvokeCall, ExtensionStreamWriter, LlmMessage } from "../src/index.js";

/** 转运真实 Framework producer 的完整输出；消费到原 EOF 后才提交原生末尾。 */
export async function forwardNativeOutput(
  writer: ExtensionStreamWriter,
  source: AsyncIterable<{ readonly event: string; readonly data?: unknown }>,
): Promise<void> {
  let terminal: unknown;
  for await (const event of source) {
    if (terminal !== undefined) throw new Error("native producer emitted data after its runtime terminal");
    const status = event.data as { status?: string } | undefined;
    if (event.event === "execution_status" && ["settled", "failed", "cancelled"].includes(status?.status ?? "")) {
      // 保存原始载体，绝不从 String、FinalAnswer 或 EOF 创建一个 Settled。
      terminal = structuredClone(event);
    } else {
      await writer.chunk({ kind: "agent", value: event });
    }
  }
  if (terminal === undefined) throw new Error("native producer EOF without its runtime terminal");
  await writer.complete({ kind: "agent", value: terminal });
}

/** 普通 String 实现示例；其运输结果不冒充原生执行结算。原生实现使用上面的原 source 转运。 */
export async function registerMessageAgent(
  sdk: EchoAgentClient,
  consume: (message: LlmMessage, history: readonly LlmMessage[] | null | undefined, signal: AbortSignal) => Promise<string>,
) {
  return sdk.registerCustomAgent("message-example", {
    kind: "custom_agent", descriptor_version: 1, name: "message-example",
    model_name: "example", system_prompt: "保留完整输入", tool_names: [],
  }, async (raw, signal) => {
    // 原登记入口的 raw handler 边界；载荷合同由本次 Host/protocol 统一确定。
    const call = raw as ExtensionInvokeCall<CustomAgentInvocation>;
    const invocation = call.invocation;
    if (invocation.operation === "agent_close") return { outcome: "result", result: { operation: "agent_close", value: null } };
    let message: LlmMessage;
    switch (invocation.operation) {
      case "agent_execute": case "agent_execute_stream": message = invocation.input.task; break;
      case "agent_chat": case "agent_chat_stream": message = invocation.input.message; break;
      default: throw new Error("unexpected message Agent operation");
    }
    const answer = await consume(message, invocation.input.history, signal);
    if (!invocation.operation.endsWith("_stream")) return { outcome: "result", result: { operation: invocation.operation, value: answer } };
    if (!call.stream) throw new Error("original stream handle missing");
    await sdk.streamWriter(call.stream).complete({ kind: "agent", value: { event: "final_answer", data: { text: answer } } });
    return { outcome: "stream", stream: call.stream };
  });
}
