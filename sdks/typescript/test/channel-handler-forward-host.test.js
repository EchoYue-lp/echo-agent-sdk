import test from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, fromWireValue } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw new Error("Channel forward requires a real current Host");
const shared = dirname(fileURLToPath(new URL("../../shared/testing/channel_forward_scenarios.json", import.meta.url)));
const scenario = JSON.parse(readFileSync(join(shared, "channel_forward_scenarios.json"), "utf8"));
const evidence = process.env.ECHO_AGENT_SDK_CHANNEL_FORWARD_EVIDENCE;
const sha = path => createHash("sha256").update(readFileSync(path)).digest("hex");
const receipt = { language: "typescript", host_sha256: host ? sha(host) : null, source_hashes: {
  "channel-handler-forward-host.test.js": sha(fileURLToPath(import.meta.url)),
  "client.ts": sha(fileURLToPath(new URL("../src/client.ts", import.meta.url))),
  "channel_forward_scenarios.json": sha(join(shared, "channel_forward_scenarios.json")),
}, scenarios: [], boundary_notes: ["Group 7 quota/setup/publication fault injection remains Host-unit proof.", "Group 9 transport waiter-drop and handler-close failure portions of 11 require Host/framework tests; native tests do not invent a fault API."] };
const clone = value => structuredClone(value);
const result = (operation, value = null) => ({ outcome: "result", result: { operation, value } });
const failure = name => ({ outcome: "error", error: clone(scenario.callback_behaviors[name].error) });
const deferred = () => { let resolve; const promise = new Promise(done => { resolve = done; }); return { promise, resolve }; };
function businessWire(value) {
  if (value === null) return { kind: "null" };
  if (typeof value === "string") return { kind: "string", value };
  if (typeof value === "boolean") return { kind: "bool", value };
  if (typeof value === "number") return { kind: "u64", value: String(value) };
  if (Array.isArray(value)) return { kind: "list", value: value.map(businessWire) };
  return { kind: "map", value: Object.entries(value).map(([key, entry]) => ({ key: { kind: "string", value: key }, value: businessWire(entry) })) };
}
function save(row) { receipt.scenarios.push(row); if (evidence) writeFileSync(join(evidence, "typescript.json"), JSON.stringify(receipt, null, 2)); }
async function bounded(promise, label = "native Channel operation", milliseconds = 8000) {
  let timer; try { return await Promise.race([promise, new Promise((_, reject) => { timer = setTimeout(() => reject(new Error(`${label} did not settle`)), milliseconds); })]); } finally { clearTimeout(timer); }
}
async function until(predicate) { const end = Date.now() + 8000; while (!predicate()) { assert.ok(Date.now() < end, "genuine callback condition missing"); await new Promise(resolve => setTimeout(resolve, 5)); } }
async function abort(signal) { if (signal.aborted) return; await new Promise(resolve => signal.addEventListener("abort", resolve, { once: true })); }
function normalMessage(value) { return { ...value, reply_to: value.reply_to ?? null, attachments: (value.attachments ?? []).map(item => ({ ...item, filename: item.filename ?? null })) }; }
function count(env, operation) { return env.calls.filter(call => call.operation === operation).length; }
function handlerCount(env) { return env.calls.filter(call => ["channel_handle", "channel_handle_stream", "channel_reply"].includes(call.operation)).length; }
async function rejectedWithoutHandler(env, name, promise, code) {
  const before = handlerCount(env); let caught;
  await assert.rejects(bounded(promise), error => { caught = error; return error.code === code; });
  assert.equal(handlerCount(env), before);
  save({ scenario: name, error_code: caught.code, callback_delta_expected: 0, callback_delta: handlerCount(env) - before });
}
async function environment(name, start = true) {
  const work = mkdtempSync(join(tmpdir(), `sdk-ts-channel-forward-${name}-`));
  assert.equal(spawnSync("python3", [join(shared, "mcp_channel_config_fixture.py"), work]).status, 0);
  const sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")] });
  const env = { sdk, work, calls: [], observations: [], mode: "normal", startMode: "normal", stopFailures: 0,
    entered: deferred(), cancelled: deferred(), release: deferred(), startEntered: deferred(), startRelease: deferred(), finalized: 0, registrations: [] };
  env.agent = await sdk.createAgent(); env.session = await env.agent.createSession();
  env.manager = (await sdk.call("channels.manager.open", env.session.wire, [])).resource;
  env.op = (operation, args, session = env.session) => sdk.call(operation, session.wire, args);
  env.forward = (operation, message = scenario.inbound) => env.op(`channels.handler.${operation}`, [env.manager, scenario.channel_id, businessWire(message)]);
  env.handler = await sdk.registerChannelMessageHandler(scenario.handler_id, { kind: "channel_message_handler", descriptor_version: 1, handler_id: scenario.handler_id }, async (call, signal) => {
    const operation = call.invocation.operation, message = call.invocation.input.message;
    env.calls.push({ operation, message: clone(message), extension: clone(call.extension) });
    const ids = await bounded(env.op("channels.manager.ids", [env.manager]), "handler ids metadata reentry");
    const listing = await bounded(env.op("channels.manager.list", [env.manager]), "handler list metadata reentry");
    assert.deepEqual(ids, [scenario.channel_id]); assert.deepEqual(listing.channels, [scenario.channel_id]);
    if (operation === "channel_handle") { if (env.mode === "fail-handle") return failure("fail-handle"); return result(operation, clone(scenario.outbound)); }
    if (operation === "channel_reply") { if (env.mode === "fail-reply") return failure("fail-reply"); return result(operation); }
    assert.equal(operation, "channel_handle_stream"); assert.equal(call.stream.kind, "stream");
    const writer = sdk.streamWriter(call.stream);
    try {
      if (env.mode === "hold-finally") {
        env.entered.resolve(); await abort(signal); env.cancelled.resolve(); await env.release.promise;
        await writer.cancelled(); return { outcome: "stream", stream: call.stream };
      }
      const items = env.mode === "full-buffer" ? [...scenario.stream_items, ...scenario.stream_items] : scenario.stream_items;
      for (const item of items.slice(0, -1)) await writer.chunk({ kind: "channel", value: clone(item) });
      await writer.complete({ kind: "channel", value: clone(items.at(-1)) });
      return { outcome: "stream", stream: call.stream };
    } finally { env.finalized++; }
  });
  env.registrations.push(env.handler);
  env.plugin = await sdk.registerChannelPlugin(`plugin-${name}`, { kind: "channel_plugin", descriptor_version: 1, channel_id: scenario.channel_id, label: "forward 中文🌊", handler_id: scenario.handler_id, capabilities: { chat_types: ["direct", "group"], supports_media: true, supports_threads: true } }, async call => {
    const operation = call.invocation.operation; env.calls.push({ operation, input: clone(call.invocation.input) });
    if (operation === "channel_start") {
      assert.equal(call.invocation.input.handler_id, scenario.handler_id);
      if (env.startMode === "park-start") { env.startEntered.resolve(); await env.startRelease.promise; }
      if (env.startMode === "fail-start-once") { env.startMode = "normal"; return failure("fail-start-once"); }
    }
    if (operation === "channel_stop" && env.stopFailures > 0) { env.stopFailures--; return failure("fail-stop-once"); }
    return result(operation);
  });
  env.registrations.push(env.plugin);
  assert.equal((await env.op("channels.plugin.register", [env.manager, env.plugin.wire, env.handler.wire])).registered, true);
  if (start) assert.deepEqual(await env.op("channels.manager.start", [env.manager, env.handler.wire]), { channels: "1", failures: "0" });
  return env;
}
async function finish(env) {
  env.release.resolve(); env.startRelease.resolve();
  try { await bounded(env.session.close()); for (const registration of env.registrations) await bounded(registration.close()); await bounded(env.agent.close()); }
  finally {
    await bounded(env.sdk.close(), "real Host shutdown");
    save({ scenario: `raw-callbacks-${env.work.split("/").at(-1)}`, callbacks: clone(env.calls), native_finally_count: env.finalized });
  }
}
async function next(env, stream, session = env.session, family = "channels") {
  const value = await env.sdk.family(family, `${family}.stream.next`, session.wire, [stream]);
  assert.equal(value.kind, "variant"); assert.equal(value.value.type_id, "echo_sdk::FacadeStreamEvent");
  const fields = Object.fromEntries(value.value.fields.map(field => [field.name, fromWireValue(field.value)]));
  assert.deepEqual(fields.stream, stream); assert.match(fields.sequence, /^[1-9][0-9]*$/);
  return { variant: value.value.variant, ...fields };
}
async function closeManager(env) {
  const operation = "facade.resource.close", route = env.sdk.catalog.resolve(operation);
  const response = await env.sdk.request(route.method, { operation, signature_digest: route.signature, handle: env.session.wire, arguments: [{ kind: "handle", value: env.manager }] });
  return fromWireValue(response.value);
}

const options = { skip: !host, timeout: 60000 };
test("Channel forward native single/stream/reply, typed fidelity, explicit failures and metadata", options, async () => {
  const env = await environment("core");
  try {
    const single = await env.forward("handle"); assert.deepEqual(single, scenario.outbound);
    assert.deepEqual(env.calls.find(call => call.operation === "channel_handle").message, scenario.inbound);
    const stream = await env.forward("handle_stream"); const events = [];
    for (let index = 0; index < 3; index++) events.push(await bounded(next(env, stream)));
    assert.deepEqual(events.map(event => event.variant), ["item", "item", "complete"]);
    assert.deepEqual(events.slice(0, 2).map(event => event.value), scenario.stream_items);
    assert.equal(count(env, "channel_reply"), 0, "stream must not implicitly reply");
    const reply = await env.forward("reply", scenario.outbound); assert.equal(reply, null);
    const observed = { single, items: events.slice(0, 2).map(event => event.value), terminal: events[2].variant,
      sequences: events.map(event => event.sequence), reply, callback_counts: Object.fromEntries(["channel_handle", "channel_handle_stream", "channel_reply"].map(operation => [operation, count(env, operation)])) };
    assert.deepEqual(observed, scenario.normalized_core_expected); save({ scenario: "single-stream-reply", observed });
    for (const invalid of scenario.invalid_inbound) {
      const input = clone(scenario.inbound); if (invalid.remove) delete input[invalid.remove]; else input[invalid.field] = clone(invalid.value);
      await rejectedWithoutHandler(env, invalid.id, env.forward("handle", input), "invalid_value");
    }
    for (const filename of [undefined, null, ""]) {
      const input = clone(scenario.inbound); for (const attachment of input.attachments) { if (filename === undefined) delete attachment.filename; else attachment.filename = filename; }
      await env.forward("handle", input); const captured = env.calls.filter(call => call.operation === "channel_handle").at(-1).message;
      assert.deepEqual(captured.attachments.map(item => item.filename ?? null), Array(4).fill(filename ?? null));
    }
    const bare = clone(scenario.inbound); delete bare.attachments; await env.forward("handle", bare);
    assert.deepEqual(env.calls.filter(call => call.operation === "channel_handle").at(-1).message.attachments ?? [], []);
    for (const replyTo of [undefined, null, ""]) {
      const output = clone(scenario.outbound); if (replyTo === undefined) delete output.reply_to; else output.reply_to = replyTo;
      assert.equal(await env.forward("reply", output), null);
      assert.equal(env.calls.filter(call => call.operation === "channel_reply").at(-1).message.reply_to ?? null, replyTo ?? null);
    }
    for (const mode of ["fail-handle", "fail-reply"]) { env.mode = mode; await assert.rejects(env.forward(mode === "fail-handle" ? "handle" : "reply", mode === "fail-handle" ? scenario.inbound : scenario.outbound), error => error.code === "framework_error" && error.message.includes(scenario.callback_behaviors[mode].error.message)); save({ scenario: mode, error_code: "framework_error", callback_delta: 1 }); }
  } finally { env.mode = "normal"; await finish(env); }
});

test("Channel forward exact Session/manager/stream generation and batch binding before effects", options, async () => {
  const env = await environment("owners"); let other;
  try {
    other = await env.agent.createSession(); const manager = (await env.op("channels.manager.open", [], other)).resource;
    // Extension registration is connection-scoped; each manager owns a distinct entry.
    // Reuse the one valid global registration instead of duplicate channel identities.
    await env.op("channels.plugin.register", [manager, env.plugin.wire, env.handler.wire], other);
    await env.op("channels.manager.start", [manager, env.handler.wire], other);
    const otherInput = { ...scenario.inbound, message_id: "other-session-message" };
    assert.deepEqual(await env.op("channels.handler.handle", [manager, scenario.channel_id, businessWire(otherInput)], other), scenario.outbound);
    assert.equal(env.calls.filter(call => call.operation === "channel_handle").at(-1).message.message_id, "other-session-message");
    assert.deepEqual(await env.forward("handle"), scenario.outbound);
    assert.equal(env.calls.filter(call => call.operation === "channel_handle").at(-1).message.message_id, scenario.inbound.message_id);
    assert.notEqual(manager.id, env.manager.id);
    save({ scenario: "same-global-extension-distinct-session-entries", first_manager: env.manager, second_manager: manager, first_session: env.session.wire, second_session: other.wire, handler_received_message_ids: ["other-session-message", scenario.inbound.message_id] });
    await rejectedWithoutHandler(env, "cross-session-manager", env.op("channels.handler.handle", [env.manager, scenario.channel_id, businessWire(scenario.inbound)], other), "invalid_value");
    const stale = { ...env.manager, generation: (BigInt(env.manager.generation) - 1n).toString() };
    await rejectedWithoutHandler(env, "stale-manager", env.op("channels.handler.handle", [stale, scenario.channel_id, businessWire(scenario.inbound)]), "stale_handle");
    const stream = await env.forward("handle_stream");
    await rejectedWithoutHandler(env, "cross-session-stream", env.op("channels.stream.next", [stream], other), "invalid_value");
    await rejectedWithoutHandler(env, "stale-stream", env.op("channels.stream.next", [{ ...stream, generation: (BigInt(stream.generation) - 1n).toString() }]), "stale_handle");
    for (let index = 0; index < 3; index++) await next(env, stream);
    const wrongHandler = await env.sdk.registerChannelMessageHandler("wrong-handler", { kind: "channel_message_handler", descriptor_version: 1, handler_id: "wrong-handler" }, async call => result(call.invocation.operation, scenario.outbound)); env.registrations.push(wrongHandler);
    const starts = count(env, "channel_start");
    await rejectedWithoutHandler(env, "wrong-handler-original-binding", env.op("channels.manager.start", [env.manager, wrongHandler.wire]), "invalid_value");
    assert.equal(count(env, "channel_start"), starts);
    await rejectedWithoutHandler(env, "wrong-handler-generation", env.op("channels.manager.start", [env.manager, { ...env.handler.wire, generation: (BigInt(env.handler.wire.generation) + 1n).toString() }]), "invalid_value");
    assert.equal(count(env, "channel_start"), starts);
    const batch = (await env.op("channels.manager.open", [])).resource; let batchStarts = 0;
    for (const [id, handler] of [["batch-a", env.handler], ["batch-b", wrongHandler]]) {
      const plugin = await env.sdk.registerChannelPlugin(`plugin-${id}`, { kind: "channel_plugin", descriptor_version: 1, channel_id: id, label: id, handler_id: handler === env.handler ? scenario.handler_id : "wrong-handler", capabilities: { chat_types: ["direct"], supports_media: false, supports_threads: false } }, async call => { if (call.invocation.operation === "channel_start") batchStarts++; return result(call.invocation.operation); }); env.registrations.push(plugin);
      await env.op("channels.plugin.register", [batch, plugin.wire, handler.wire]);
    }
    await rejectedWithoutHandler(env, "batch-mismatch-all-before-effects", env.op("channels.manager.start", [batch, env.handler.wire]), "invalid_value"); assert.equal(batchStarts, 0);
    await other.close(); await env.forward("handle"); save({ scenario: "independent-owner-survives", other_session_closed: true, batch_start_effects: batchStarts });
  } finally { await other?.close(); await finish(env); }
});

test("Channel forward rejects unstarted/starting/start rollback before native handler", options, async () => {
  const env = await environment("start-states", false);
  try {
    await rejectedWithoutHandler(env, "unstarted", env.forward("handle"), "invalid_value");
    env.startMode = "park-start"; const starting = env.op("channels.manager.start", [env.manager, env.handler.wire]);
    await bounded(env.startEntered.promise); await rejectedWithoutHandler(env, "starting", env.forward("handle"), "invalid_value");
    env.startRelease.resolve(); assert.deepEqual(await bounded(starting), { channels: "1", failures: "0" });
    await env.op("channels.manager.stop_exact", [env.manager, scenario.channel_id]);
    env.startMode = "fail-start-once"; assert.deepEqual(await env.op("channels.manager.start", [env.manager, env.handler.wire]), { channels: "1", failures: "1" });
    await rejectedWithoutHandler(env, "start-failure-rollback", env.forward("handle"), "invalid_value");
    assert.deepEqual(await env.op("channels.manager.start", [env.manager, env.handler.wire]), { channels: "1", failures: "0" });
    assert.deepEqual(await env.forward("handle"), scenario.outbound); save({ scenario: "restart-after-success-only", failures: "1", restarted: true });
  } finally { await finish(env); }
});

test("Channel stream cancellation waits actual callback finally while independent work survives", options, async () => {
  const env = await environment("cancel");
  try {
    env.mode = "hold-finally"; const stream = await env.forward("handle_stream"); await bounded(env.entered.promise);
    env.mode = "normal"; assert.deepEqual(await env.forward("handle"), scenario.outbound);
    assert.equal(await env.op("channels.stream.cancel", [stream]), true); await bounded(env.cancelled.promise);
    let settled = false; const closing = env.op("channels.stream.close", [stream]).finally(() => { settled = true; }); closing.catch(() => {});
    await env.op("channels.manager.ids", [env.manager]); assert.equal(settled, false); assert.equal(env.finalized, 0);
    assert.deepEqual(await env.forward("handle"), scenario.outbound, "cancelling A must not cancel an independent B");
    env.release.resolve(); assert.equal(await bounded(closing), true); assert.equal(env.finalized, 1);
    assert.equal(await env.op("channels.stream.close", [stream]), false);
    save({ scenario: "cancel-A-keeps-B", finally_before_close: true, finally_count: env.finalized, repeated_close: false });
  } finally { await finish(env); }
});

for (const closeKind of ["stop", "resource", "session"]) test(`Channel unconsumed full buffer ${closeKind} cancels before write permit`, options, async () => {
  const env = await environment(`full-${closeKind}`);
  try {
    env.mode = "full-buffer"; const stream = await env.forward("handle_stream"); await until(() => env.finalized === 1);
    if (closeKind === "stop") { assert.equal(await bounded(env.op("channels.manager.stop_exact", [env.manager, scenario.channel_id])), null); await rejectedWithoutHandler(env, "stopped-forward", env.forward("handle"), "invalid_value"); await env.op("channels.stream.close", [stream]); await env.op("channels.manager.start", [env.manager, env.handler.wire]); assert.deepEqual(await env.forward("handle"), scenario.outbound); }
    else if (closeKind === "resource") { await bounded(closeManager(env)); await rejectedWithoutHandler(env, "closed-manager-forward", env.forward("handle"), "closed_handle"); }
    else { await bounded(env.session.close()); await rejectedWithoutHandler(env, "closed-session-stream", env.op("channels.stream.next", [stream]), "closed_handle"); }
    save({ scenario: `unconsumed-full-${closeKind}`, native_finally_count: env.finalized, settled: true });
  } finally { await finish(env); }
});

test("Channel resource close retains real cancelled callback and failed stop retry owner", options, async () => {
  const env = await environment("close-retry");
  try {
    env.mode = "hold-finally"; await env.forward("handle_stream"); await bounded(env.entered.promise); env.stopFailures = 1;
    let settled = false; const closing = closeManager(env).finally(() => { settled = true; }); closing.catch(() => {});
    await bounded(env.cancelled.promise); assert.equal(settled, false); assert.equal(env.finalized, 0);
    await rejectedWithoutHandler(env, "closing-forward-admission", env.forward("handle"), "closed_handle");
    env.release.resolve(); await assert.rejects(bounded(closing), error => error.code === "framework_error" && error.message.includes("controlled channel stop failure"));
    assert.equal(env.finalized, 1); assert.equal(count(env, "channel_stop"), 1);
    await bounded(closeManager(env)); assert.equal(count(env, "channel_stop"), 2);
    save({ scenario: "resource-close-late-finally-retry", stop_attempts: 2, native_finally_count: 1, early_settled: false });
  } finally { await finish(env); }
});

test("Channel forward wrong family/digest/argument shape reject before callback", options, async () => {
  const env = await environment("contracts");
  try {
    const operation = "channels.handler.handle", route = env.sdk.catalog.resolve(operation);
    const params = { operation, signature_digest: route.signature, handle: env.session.wire, arguments: [businessWire(env.manager), businessWire(scenario.channel_id), businessWire(scenario.inbound)] };
    // Handles are explicit on the wire; message maps preserve their business kind fields.
    params.arguments[0] = { kind: "handle", value: env.manager };
    await rejectedWithoutHandler(env, "wrong-family", env.sdk.request("_echo_agent/workflow/op", params), "invalid_value");
    await rejectedWithoutHandler(env, "wrong-digest", env.sdk.request(route.method, { ...params, signature_digest: `sha256:${"0".repeat(64)}` }), "invalid_value");
    await rejectedWithoutHandler(env, "extra-argument", env.sdk.request(route.method, { ...params, arguments: [...params.arguments, { kind: "null" }] }), "invalid_value");
    const stream = await env.forward("handle_stream");
    await rejectedWithoutHandler(env, "wrong-family-stream-control", env.sdk.call("workflow.stream.next", env.session.wire, [stream]), "invalid_value");
    for (let index = 0; index < 3; index++) await next(env, stream);
  } finally { await finish(env); }
});

test("Channel genuine transport disconnect cancels callback and waits actual local finally", options, async () => {
  const env = await environment("disconnect"); const pid = env.sdk.process.pid; let disconnected;
  try {
    env.mode = "hold-finally"; await env.forward("handle_stream"); await bounded(env.entered.promise);
    // The official ACP connection owns this transport. No fixture process acts as Host.
    env.sdk.connection.close(); disconnected = true;
    await bounded(env.cancelled.promise); assert.equal(env.finalized, 0);
    env.release.resolve(); await until(() => env.finalized === 1);
    await bounded(env.sdk.close(), "Host exit after native callback settlement");
    await until(() => { try { process.kill(pid, 0); return false; } catch (error) { if (error.code === "ESRCH") return true; throw error; } });
    save({ scenario: "genuine-transport-disconnect", callback_cancel_observed: true, native_finally_count: 1, actual_host_pid: pid, actual_host_exited: true });
  } finally {
    env.release.resolve(); if (!disconnected) await finish(env); else await bounded(env.sdk.close());
  }
});
