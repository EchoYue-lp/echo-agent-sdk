import test from "node:test";
import assert from "node:assert/strict";
import { registerMessageAgent } from "../dist-examples/examples/custom-agent-message.js";

test("raw CustomAgent example keeps full Message/history and returns raw outcomes", async () => {
  let handler;
  const completed = [];
  const sdk = {
    async registerCustomAgent(_id, _descriptor, selected) { handler = selected; return "registered"; },
    streamWriter(stream) { return { async complete(value) { completed.push({ stream, value }); } }; },
  };
  const message = { role: "tool", name: "shell", tool_call_id: "call-🧪", content: { kind: "map", value: [] }, approved_assistant_blocks: [] };
  const seen = [];
  await registerMessageAgent(sdk, async (value, history, signal) => {
    assert.equal(value, message);
    assert.equal(signal.aborted, false);
    seen.push(history);
    return "处理完整输入🧪";
  });
  const stream = { id: "original-stream", generation: "1", kind: "stream" };
  for (const operation of ["agent_execute", "agent_chat", "agent_execute_stream", "agent_chat_stream"]) {
    for (const history of [undefined, [], [message]]) {
      const input = { [operation.startsWith("agent_execute") ? "task" : "message"]: message, history };
      const result = await handler({ invocation: { operation, input }, stream }, new AbortController().signal);
      assert.equal(seen.at(-1), history);
      assert.deepEqual(result, operation.endsWith("_stream")
        ? { outcome: "stream", stream }
        : { outcome: "result", result: { operation, value: "处理完整输入🧪" } });
    }
  }
  assert.equal(completed.length, 6);
  const calls = seen.length;
  assert.deepEqual(await handler({ invocation: { operation: "agent_close", input: null } }, new AbortController().signal),
    { outcome: "result", result: { operation: "agent_close", value: null } });
  assert.equal(seen.length, calls, "close不能重新执行正文处理");
  await assert.rejects(handler({ invocation: { operation: "not-an-agent", input: {} } }, new AbortController().signal));
});
