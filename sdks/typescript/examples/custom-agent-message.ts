import type { CustomAgentInvocation, EchoAgentClient, ExtensionInvokeCall, LlmMessage } from "../src/index.js";

/** 示例：实现方接收完整 Message，不能先取文本而丢掉附件、meta 或工具身份。 */
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
