import test from "node:test";
import assert from "node:assert/strict";
import { EchoAgentClient, SessionHandle } from "../dist/client.js";
import { toWireValue } from "../dist/wire.js";

const stream = { id: "stream", generation: "1", kind: "stream" };
function envelope(sequence, event_type, value) {
  return { schema_version: 4, event_id: `event-${sequence}`, content_hash: `sha256:${"a".repeat(64)}`,
    sequence: String(sequence), stream_id: stream.id, turn_id: "turn", execution_id: "outer-execution",
    conversation_id: undefined, run_id: undefined, message_id: undefined, parent_event_id: undefined,
    timestamp: { unix_seconds: "0", nanos: 0 }, payload: { event_type, data: toWireValue(value) } };
}
function status(kind) {
  return { execution_id: "physical-execution", request_id: "request", sequence: (1n << 64n) - 1n,
    phase: "Terminal", status: kind, reason: null };
}
function fixture() {
  const sdk = Object.create(EchoAgentClient.prototype);
  const writes = [];
  Object.assign(sdk, { events: new Map(), queueCapacity: 32, closed: false,
    notify: async (method, value) => { writes.push({ method, value }); } });
  return { sdk, writes };
}

for (const kind of ["settled", "cancelled", "failed"]) {
  test(`only native ${kind} finishes the feed after final body and ACK`, { timeout: 1000 }, async () => {
    const { sdk, writes } = fixture();
    const items = [envelope(1, "final_answer", ""),
      envelope(2, "assistant_message", { block: { phase: "final_answer", complete: true, delta: "" } }),
      envelope(3, "error", { message: "ExecutionNotSettled", failure: { terminal_kind: "observation" } }),
      envelope(4, "execution_status", status("candidate")),
      envelope(5, "assistant_message", { block: { phase: "commentary", complete: true, delta: "尾部正文🙂" } }),
      envelope(6, "execution_status", status(kind))];
    for (const item of items) sdk.publishFirstEvent(stream, item);
    const observed = [];
    for await (const item of sdk.eventsFor(stream.id, stream)) observed.push(item.envelope);
    assert.deepEqual(observed, items);
    assert.deepEqual(writes.map(row => row.value.ack.last_processed_sequence), ["1", "2", "3", "4", "5", "6"]);
  });
}

test("replay closes only after the runtime envelope watermark is acknowledged", async () => {
  const { sdk } = fixture();
  sdk.eventsFor(stream.id, stream);
  const page = (after, item) => ({ requested_after_sequence: String(after), events: [{ stream, envelope: item }],
    next_cursor: { stream_id: stream.id, last_processed_sequence: item.sequence } });
  sdk.observeReplay(stream, page(0, envelope(1, "final_answer", "")));
  await sdk.acknowledge(stream, "1");
  assert.equal(sdk.events.get(stream.id).queue.closed, false);
  sdk.observeReplay({ ...stream, generation: "2" }, page(1, envelope(2, "execution_status", status("settled"))));
  assert.equal(sdk.events.get(stream.id).replayTerminalSequence, undefined);
  sdk.observeReplay(stream, page(1, envelope(2, "execution_status", status("settled"))));
  assert.equal(sdk.events.get(stream.id).queue.closed, false);
  await sdk.acknowledge(stream, "2");
  assert.equal(sdk.events.get(stream.id).queue.closed, true);
});

test("malformed native status fails observation instead of reporting a successful end", async () => {
  const { sdk } = fixture();
  sdk.publishFirstEvent(stream, envelope(1, "execution_status", { ...status("settled"), sequence: "01" }));
  await assert.rejects(sdk.eventsFor(stream.id, stream)[Symbol.asyncIterator]().next(), error => error.code === "serialization_violation");
});

test("an accepted Run stays parent-owned when its first observation is malformed", async () => {
  const { sdk } = fixture();
  sdk.request = async () => ({ run: { id: "run", generation: "1", kind: "run" }, stream,
    first_event: { ...envelope(1, "execution_status", status("settled")), event_id: undefined } });
  const session = new SessionHandle(sdk, { id: "agent", generation: "1", kind: "agent" },
    { id: "session", generation: "1", kind: "session" }, "acp", { id: "task", generation: "1", kind: "task_run" });
  const run = await session.startRun({ text: "task" });
  assert.ok(session.runs.has(run));
  await assert.rejects(run.events[Symbol.asyncIterator]().next(), error => error.code === "serialization_violation");
});

for (const value of ["01", "18446744073709551616"]) {
  test(`invalid waiting wire counter ${value} is a typed observation failure`, async () => {
    const { sdk } = fixture();
    const item = envelope(1, "execution_status", { ...status("waiting"), phase: "Thinking" });
    item.payload.data.value.find(entry => entry.key.value === "sequence").value = { kind: "u64", value };
    sdk.publishFirstEvent(stream, item);
    await assert.rejects(sdk.eventsFor(stream.id, stream)[Symbol.asyncIterator]().next(), error => error.code === "serialization_violation");
  });
}
