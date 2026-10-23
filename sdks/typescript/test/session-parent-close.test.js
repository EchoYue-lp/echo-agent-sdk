import test from "node:test";
import assert from "node:assert/strict";
import { EchoAgentClient, RunHandle, SessionHandle } from "../dist/client.js";
import { EchoAgentError } from "../dist/errors.js";

const handle = (id, kind) => ({ id, generation: "1", kind });
const started = () => ({ run: handle("run", "run"), stream: handle("stream", "stream") });
const deferred = () => {
  let resolve, reject;
  const promise = new Promise((done, fail) => { resolve = done; reject = fail; });
  return { promise, resolve, reject };
};

function fixture(request = async method => method === "_echo_agent/run/start" ? started() : {}) {
  // Keep real SDK mailboxes and iteration; only the Host RPC reply is controlled.
  const client = Object.create(EchoAgentClient.prototype);
  client.events = new Map();
  client.updates = new Map();
  client.queueCapacity = 32;
  client.closed = false;
  client.request = request;
  const session = new SessionHandle(client, handle("agent", "agent"), handle("session", "session"), "acp");
  return { client, session };
}

async function bounded(promise) {
  let timer;
  try {
    return await Promise.race([
      promise,
      new Promise((_, reject) => { timer = setTimeout(() => reject(new Error("local stream did not end")), 200); }),
    ]);
  } finally {
    clearTimeout(timer);
  }
}

test("successful parent close finishes an already waiting child stream", async () => {
  const { client, session } = fixture();
  const run = await session.startRun({ text: "task" });
  const next = run.events[Symbol.asyncIterator]().next();
  try {
    await session.close();
    assert.deepEqual(await bounded(next), { value: undefined, done: true });
    await run.close();
    assert.throws(() => run.get(), error => error.code === "closed_handle");
  } finally {
    client.endEvents(run.stream.id);
  }
});

test("first child and update subscriptions after parent close are already finite", async () => {
  const { client, session } = fixture();
  const run = await session.startRun({ text: "task" });
  await session.close();
  try {
    assert.deepEqual(await bounded(run.events[Symbol.asyncIterator]().next()), { value: undefined, done: true });
    assert.throws(() => session.updates(), error => error.code === "closed_handle");
    await run.close();
  } finally {
    client.endEvents(run.stream.id);
  }
});

test("late start response cannot reopen a child after successful parent close", async () => {
  const response = deferred();
  const { client, session } = fixture(async method => method === "_echo_agent/run/start" ? response.promise : {});
  const starting = session.startRun({ text: "task" });
  await session.close();
  response.resolve(started());
  const run = await starting;
  try {
    assert.deepEqual(await bounded(run.events[Symbol.asyncIterator]().next()), { value: undefined, done: true });
    assert.throws(() => run.cancel(), error => error.code === "closed_handle");
  } finally {
    client.endEvents(run.stream.id);
  }
});

test("failed parent close keeps child streams open and permits a successful retry", async () => {
  let attempts = 0;
  const { client, session } = fixture(async method => {
    if (method === "_echo_agent/run/start") return started();
    if (method === "_echo_agent/session/close" && ++attempts === 1) throw new EchoAgentError("busy", "retry close");
    return {};
  });
  const run = await session.startRun({ text: "task" });
  run.events;
  await assert.rejects(session.close(), error => error.code === "busy");
  assert.equal(client.events.get(run.stream.id).queue.ended, false);
  await run.get();
  await session.close();
  assert.deepEqual(await bounded(run.events[Symbol.asyncIterator]().next()), { value: undefined, done: true });
  assert.equal(attempts, 2);
});

test("concurrent parent close callers share one original Host request", async () => {
  const reply = deferred();
  let closes = 0;
  const { session } = fixture(async method => {
    if (method === "_echo_agent/session/close") { closes++; return reply.promise; }
    return {};
  });
  const one = session.close();
  const two = session.close();
  reply.resolve({});
  await Promise.all([one, two]);
  assert.equal(closes, 1);
});

for (const code of ["closed_handle", "stale_handle", "host_exited"]) {
  test(`child local close can release its stream after ${code}`, async () => {
    const { client } = fixture(async () => { throw new EchoAgentError(code, "owner already gone"); });
    const value = started();
    const run = new RunHandle(client, value.run, value.stream);
    const next = run.events[Symbol.asyncIterator]().next();
    try {
      await run.close();
      assert.deepEqual(await bounded(next), { value: undefined, done: true });
      await run.close();
    } finally {
      client.endEvents(run.stream.id);
    }
  });
}

test("Host shutdown in progress remains a retryable child close failure", async () => {
  const { client } = fixture(async () => { throw new EchoAgentError("host_shutting_down", "not disposed yet"); });
  const value = started();
  const run = new RunHandle(client, value.run, value.stream);
  run.events;
  await assert.rejects(run.close(), error => error.code === "host_shutting_down");
  assert.equal(client.events.get(run.stream.id).queue.ended, false);
  client.endEvents(run.stream.id);
});

test("scope disposal preserves buffered facts and does not fabricate a terminal or submit new ACKs", async () => {
  const { client, session } = fixture();
  const run = await session.startRun({ text: "task" });
  const iterator = run.events[Symbol.asyncIterator]();
  const feed = client.events.get(run.stream.id);
  const fact = { envelope: { sequence: "1", payload: { event_type: "delta" } } };
  feed.queue.push(fact);
  let acks = 0;
  client.acknowledge = async () => { acks++; throw new Error("disposed Host cannot ACK"); };
  await session.close();
  assert.deepEqual(await bounded(iterator.next()), { value: fact, done: false });
  assert.deepEqual(await bounded(iterator.next()), { value: undefined, done: true });
  assert.equal(acks, 0);
});

test("scope disposal releases an iterator waiting on ACK without cancelling that submission", async () => {
  const { client, session } = fixture();
  const run = await session.startRun({ text: "task" });
  const iterator = run.events[Symbol.asyncIterator]();
  const feed = client.events.get(run.stream.id);
  feed.queue.push({ envelope: { sequence: "1", payload: { event_type: "delta" } } });
  const ack = deferred();
  const entered = deferred();
  let completed = false;
  client.acknowledge = async () => { entered.resolve(); await ack.promise; completed = true; };
  await iterator.next();
  const next = iterator.next();
  await entered.promise;
  await session.close();
  assert.deepEqual(await bounded(next), { value: undefined, done: true });
  assert.equal(completed, false);
  ack.resolve();
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(completed, true);
});

test("concurrent child close requests share cancellation and settlement", async () => {
  const reply = deferred();
  const calls = [];
  const { client } = fixture(async method => {
    calls.push(method);
    if (method === "_echo_agent/run/get") return { status: "running" };
    if (method === "_echo_agent/run/wait") return reply.promise;
    return {};
  });
  const value = started();
  const run = new RunHandle(client, value.run, value.stream);
  const one = run.close();
  const two = run.close();
  reply.resolve({ settled: true });
  await Promise.all([one, two]);
  assert.equal(calls.filter(method => method === "_echo_agent/run/cancel").length, 1);
  assert.deepEqual(await bounded(run.events[Symbol.asyncIterator]().next()), { value: undefined, done: true });
});

test("parent disposal releases a child close observer while preserving the original RPC", async () => {
  const reply = deferred();
  const entered = deferred();
  let originalCompleted = false;
  const { session } = fixture(async method => {
    if (method === "_echo_agent/run/start") return started();
    if (method === "_echo_agent/run/get") {
      entered.resolve();
      await reply.promise;
      originalCompleted = true;
      return { status: "completed" };
    }
    return {};
  });
  const run = await session.startRun({ text: "task" });
  const closing = run.close();
  await entered.promise;
  await session.close();
  await bounded(closing);
  assert.equal(originalCompleted, false);
  reply.resolve();
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(originalCompleted, true);
});

test("failed run settlement keeps the same stream available for a later close retry", async () => {
  let waits = 0;
  const { client } = fixture(async method => {
    if (method === "_echo_agent/run/get") return { status: "running" };
    if (method === "_echo_agent/run/wait") return { settled: ++waits > 1 };
    return {};
  });
  const value = started();
  const run = new RunHandle(client, value.run, value.stream);
  run.events;
  await assert.rejects(run.close(), error => error.code === "timeout");
  assert.equal(client.events.get(run.stream.id).queue.ended, false);
  await run.close();
  assert.equal(waits, 2);
  assert.deepEqual(await bounded(run.events[Symbol.asyncIterator]().next()), { value: undefined, done: true });
});

test("parent scope retains both local handles when Host returns the same Run identity", async () => {
  const reply = deferred();
  const entered = deferred();
  const { session } = fixture(async method => {
    if (method === "_echo_agent/run/start") return started();
    if (method === "_echo_agent/run/get") { entered.resolve(); await reply.promise; return { status: "completed" }; }
    return {};
  });
  const first = await session.startRun({ text: "task" });
  const second = await session.startRun({ text: "task" });
  assert.deepEqual(first.wire, second.wire);
  const closing = first.close();
  await entered.promise;
  await session.close();
  await bounded(closing);
  assert.throws(() => first.get(), error => error.code === "closed_handle");
  assert.throws(() => second.get(), error => error.code === "closed_handle");
  reply.resolve();
});
