import test from "node:test";
import assert from "node:assert/strict";
import { registerMessageAgent, forwardNativeOutput } from "../dist-examples/examples/custom-agent-message.js";

test("native forwarding waits for real source EOF and never mints a terminal", async () => {
  const sent = [];
  let sourceEnded = false;
  const writer = {async chunk(value) { sent.push(value); }, async complete(value) {
    assert.equal(sourceEnded, true); sent.push(value);
  }};
  const native = {event:"execution_status",data:{execution_id:"original-execution",sequence:"18446744073709551615",phase:"Terminal",status:"settled"}};
  async function* source() {
    yield {event:"final_answer",data:{text:"原确认🙂"}};
    assert.equal(sent.length, 1);
    yield native;
    assert.equal(sent.length, 1, "runtime terminal waits for producer EOF");
    sourceEnded = true;
  }
  await forwardNativeOutput(writer, source());
  assert.deepEqual(sent.at(-1), {kind:"agent",value:native});
  await assert.rejects(forwardNativeOutput(writer, (async function*() {
    yield {event:"final_answer",data:{text:"only notification"}};
  })()), /without its runtime terminal/);
});

test("raw CustomAgent example keeps full Message/history and returns raw outcomes", async () => {
  let handler;
  const completed = [];
  const sdk = {
    async registerCustomAgent(_id, _descriptor, selected) { handler = selected; return "registered"; },
    streamWriter(stream) { return { async complete(value) { completed.push({ stream, value }); } }; },
  };
  const message = { role: "tool", name: "shell", tool_call_id: "call-🧪", tool_result_is_error: false,
    content: { kind: "map", value: [] }, approved_assistant_blocks: [],
    provider_response: {protocol:"responses",provider:"fixture",model:{kind:"null"},
      stop_reason:{kind:"string",value:"future-stop"},usage:{kind:"map",value:[
        {key:{kind:"string",value:"tokens"},value:{kind:"u64",value:"18446744073709551615"}},
        {key:{kind:"string",value:"string"},value:{kind:"string",value:"00123"}},
      ]}},
  };
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
