import test from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { randomUUID } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, fromWireValue, wireU64 } from "../dist/index.js";
const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw Error("required LLM/Workflow Host missing");
const fixture = fileURLToPath(new URL("../../shared/testing/llm_workflow_stream_fixture.py", import.meta.url));
const root = process.env.ECHO_AGENT_SDK_LLM_WORKFLOW_ROOT ?? fileURLToPath(new URL("../../../target/llm-workflow-stream", import.meta.url));
const sleep = () => new Promise(resolve => setTimeout(resolve, 0));
function gate() { let release; const promise = new Promise(resolve => { release = resolve; }); return { promise, release }; }
function helper(work, ...args) { const result = spawnSync("python3", [fixture, "--work", work, ...args], { encoding: "utf8" }); assert.equal(result.status, 0, result.stderr); return result.stdout; }
async function setup(group) { mkdirSync(root, { recursive: true }); const work = mkdtempSync(join(root, `typescript-${group}-`)); helper(work, "--prepare"); const spec = JSON.parse(readFileSync(join(work, "scenario.json"), "utf8")); return { work, spec, sdk: await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")] }) }; }
const errorFact = error => ({ code: error.code, message: error.message, operation: error.operation, details: error.details });
async function observedError(call) { try { await call; assert.fail("expected SDK failure"); } catch (error) { assert.ok(error.code, error); return errorFact(error); } }
async function collect(session, stream) { const events = []; for (let i = 0; i < 12; i++) { const event = await session.invoke("workflow.stream.next", [stream]); events.push(event); if ((event.value ?? event).variant !== "item") return events; } assert.fail("workflow did not reach bounded terminal"); }
function finish(work, group, facts) { writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2)); console.log(`LLM/Workflow evidence: ${work}`); console.log(helper(work, "--verify", group)); }

test("registered LLM requests usage terminal error and cancellation", { skip: !host, timeout: 60000 }, async () => {
  const { work, spec, sdk } = await setup("llm"), rows = [];
  try {
    for (const mode of ["nonstream", "stream", "missing_finish", "error", "cancel"]) {
      const row = { mode, calls: [], finally: 0, cancel_observed: false, sent: [] }; rows.push(row);
      const started = gate(), producers = [];
      const registration = await sdk.registerLlmClient("proof-" + mode, { kind: "llm_client", descriptor_version: 1, model_name: "fixture-model", supports_streaming: mode === "stream" }, async (call, signal) => {
        row.calls.push({ operation: call.invocation.operation, request: call.invocation.input, invocation_id: call.invocation_id, deadline: call.deadline }); started.release();
        try {
          if (mode === "error") throw Error(spec.failure.message);
          if (mode === "cancel") { await new Promise(resolve => signal.aborted ? resolve() : signal.addEventListener("abort", resolve, { once: true })); row.cancel_observed = signal.aborted; }
          if (mode === "stream") {
            assert.ok(call.stream); const writer = sdk.streamWriter(call.stream);
            producers.push((async () => { await sleep(); for (const [index, value] of spec.llm_chunks.entries()) { await writer.chunk(value); row.sent.push({ event: "chunk", sequence: String(index + 1), value }); } await writer.complete(spec.llm_complete); row.sent.push({ event: "complete", sequence: "3", value: spec.llm_complete }); })());
            return { outcome: "stream", stream: call.stream };
          }
          const value = structuredClone(spec.response); if (mode === "missing_finish") delete value.finish_reason;
          return { operation: "llm_chat", value };
        } finally { row.finally++; }
      });
      const agent = await sdk.createAgent(), session = await agent.createSession();
      await session.invoke("echo_agent::agent::react::ReactAgent::set_temperature", [spec.temperature]);
      await session.invoke("echo_agent::agent::react::ReactAgent::set_max_tokens", [wireU64(String(spec.max_tokens))]);
      const run = await session.startRun({ text: spec.prompt }); if (mode === "cancel") { await started.promise; await run.cancel(); }
      row.settled = await run.wait({ timeoutMs: 15000 }); row.usage = await run.usage(); row.replay = await run.replay("0", 512);
      await Promise.all(producers);
      if (mode === "nonstream") { const history = await session.startRun({ text: spec.history_prompt }); row.history_settled = await history.wait({ timeoutMs: 15000 }); await history.close(); }
      await run.close(); await session.close(); await agent.close(); await registration.close();
    }
  } finally { await sdk.close(); writeFileSync(join(work, "observed.json"), JSON.stringify(rows, null, 2)); }
  finish(work, "llm", rows);
});

test("live scalar configuration changes original LLM thinking and visible tools", { skip: !host, timeout: 30000 }, async () => {
  const { work, spec, sdk } = await setup("scalar"), facts = { requests: [], settlements: [], replay: [] };
  try {
    const registration = await sdk.registerLlmClient("live-scalar-model", { kind: "llm_client", descriptor_version: 1, model_name: "fixture-model", supports_streaming: false }, async call => {
      facts.requests.push(call.invocation.input);
      return { operation: "llm_chat", value: structuredClone(spec.response) };
    });
    const agent = await sdk.createAgent(), session = await agent.createSession();
    const names = "echo_core::agent::Agent::tool_names";
    facts.registered_before = await session.invoke(names); assert.ok(facts.registered_before.includes("read_file"));
    await session.invoke("echo_agent::agent::react::ReactAgent::set_thinking", ["4000"]);
    facts.configured_thinking = await session.invoke("echo_agent::agent::react::ReactAgent::thinking");
    assert.ok(JSON.stringify(facts.configured_thinking).includes("4000"));
    await session.invoke("echo_agent::agent::react::ReactAgent::set_disabled_tools", [["read_file"]]);
    facts.registered_while_hidden = await session.invoke(names); assert.ok(facts.registered_while_hidden.includes("read_file"));
    const first = await session.startRun({ text: "hidden tool and explicit thinking 中文🙂" });
    facts.settlements.push(await first.wait({ timeoutMs: 15000 })); facts.replay.push(await first.replay("0", 512)); await first.close();
    assert.equal(facts.settlements[0].terminal?.status, "completed"); assert.equal(facts.requests.length, 1);
    assert.deepEqual(fromWireValue(facts.requests[0].thinking), facts.configured_thinking);
    assert.ok(!facts.requests[0].tools.some(tool => tool.name === "read_file"));
    facts.model_before_failed_switch = await session.invoke("echo_core::agent::Agent::model_name");
    await assert.rejects(session.invoke("echo_agent::agent::react::ReactAgent::set_model", ["cannot-rebuild-opaque-client"]), error => {
      facts.opaque_model_error = { ...errorFact(error), retryable: error.retryable };
      return error.code === "framework_error" && error.retryable === "never";
    });
    assert.equal(await session.invoke("echo_core::agent::Agent::model_name"), facts.model_before_failed_switch);
    await session.invoke("echo_agent::agent::react::ReactAgent::set_thinking", [null]);
    await session.invoke("echo_agent::agent::react::ReactAgent::set_disabled_tools", [null]);
    facts.registered_after_restore = await session.invoke(names); assert.deepEqual(facts.registered_after_restore, facts.registered_before);
    const second = await session.startRun({ text: "restored model tool visibility 保留🌊" });
    facts.settlements.push(await second.wait({ timeoutMs: 15000 })); facts.replay.push(await second.replay("0", 512)); await second.close();
    assert.equal(facts.settlements[1].terminal?.status, "completed"); assert.equal(facts.requests.length, 2);
    assert.equal(facts.requests[1].thinking ?? null, null);
    assert.ok(facts.requests[1].tools.some(tool => tool.name === "read_file"));
    for (const replay of facts.replay) for (const event of replay.events) {
      const payload = event.payload ?? event.envelope?.payload;
      assert.ok(!["tool_start", "tool_end", "tool_call", "tool_result"].includes(payload?.event_type), "scalar configuration executed an unexpected tool");
    }
    await session.close(); await agent.close(); await registration.close();
  } finally { await sdk.close(); writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2)); console.log(`Live scalar evidence: ${work}`); }
});

test("live compressor retains actual context and releases its original Host tokenizer", { skip: !host, timeout: 60000 }, async () => {
  const { work, spec, sdk } = await setup("compressor"), c = spec.compression;
  const facts = { model_requests: [], compressions: [], compression_results: [], released_tokenizers: [], settlements: [], before_messages: [], after_messages: [], decoy_calls: 0 };
  const tokenizers = [];
  try {
    const model = await sdk.registerLlmClient("compression-model", { kind: "llm_client", descriptor_version: 1, model_name: "fixture-model", supports_streaming: false }, async call => {
      const request = call.invocation.input; facts.model_requests.push(request);
      const response = structuredClone(spec.response);
      response.message.content = { kind: "string", value: "answer: " + request.messages.at(-1).content.value };
      return { operation: "llm_chat", value: response };
    });
    const agent = await sdk.createAgent(), session = await agent.createSession();
    // Bind after creation: latest registered defaults cannot prove that a
    // source setter changed the live Session's ContextManager.
    const original = await sdk.registerContextCompressor("original-compressor", { kind: "context_compressor", descriptor_version: 1, name: "original-compressor" }, async call => {
      const input = call.invocation.input;
      const row = { input, extension: call.extension, invocation_id: call.invocation_id, deadline: call.deadline };
      facts.compressions.push(row); tokenizers.push(input.tokenizer);
      row.token_count = String(await sdk.countTokens(input.tokenizer, "hello"));
      row.wrong_owner = await observedError(sdk.countTokens({ ...input.tokenizer, owner_session_id: "foreign-owner" }, "hello"));
      const previous = facts.before_messages.at(-1);
      row.protected_messages = previous.filter(message => !input.messages.some(candidate => candidate.role === message.role && candidate.content.value === message.content));
      let protectedTokens = 0n;
      for (const message of row.protected_messages) protectedTokens += await sdk.countTokens(input.tokenizer, message.content);
      row.protected_tokens = String(protectedTokens);
      const evicted = input.messages.filter(message => [c.evict, "answer: " + c.evict].includes(message.content.value));
      const messages = input.messages.filter(message => !evicted.includes(message));
      row.output = { messages, evicted, checkpoint: null };
      return { operation: "compressor_compress", value: row.output };
    });
    const decoy = await sdk.registerContextCompressor("decoy-compressor", { kind: "context_compressor", descriptor_version: 1, name: "decoy-compressor" }, async () => { facts.decoy_calls++; throw Error("wrong compressor selected"); });
    facts.original_extension = original.wire;
    await session.invoke("echo_agent::agent::react::ReactAgent::set_token_limit", [wireU64(c.token_limit)]);
    await session.invoke("echo_agent::agent::react::ReactAgent::set_compressor", [original.wire]); facts.binding = original.wire;
    facts.before_messages.push(await session.invoke("echo_agent::agent::react::ReactAgent::get_messages"));
    facts.compression_results.push(await session.invoke("echo_agent::agent::react::ReactAgent::force_compress_context"));
    facts.after_messages.push(await session.invoke("echo_agent::agent::react::ReactAgent::get_messages"));
    facts.released_tokenizers.push(await observedError(sdk.countTokens(tokenizers[0], "hello")));
    for (const text of [c.evict, c.retain]) { const run = await session.startRun({ text }); facts.settlements.push(await run.wait({ timeoutMs: 15000 })); await run.close(); }
    facts.before_messages.push(await session.invoke("echo_agent::agent::react::ReactAgent::get_messages"));
    facts.compression_results.push(await session.invoke("echo_agent::agent::react::ReactAgent::force_compress_with_focus_and_hooks", [c.focus, wireU64("64"), "manual"]));
    facts.after_messages.push(await session.invoke("echo_agent::agent::react::ReactAgent::get_messages"));
    facts.released_tokenizers.push(await observedError(sdk.countTokens(tokenizers[1], "hello")));
    facts.before_messages.push(await session.invoke("echo_agent::agent::react::ReactAgent::get_messages"));
    facts.compression_results.push(await session.invoke("echo_agent::agent::react::ReactAgent::force_compress_context"));
    facts.after_messages.push(await session.invoke("echo_agent::agent::react::ReactAgent::get_messages"));
    facts.released_tokenizers.push(await observedError(sdk.countTokens(tokenizers[2], "hello")));
    const next = await session.startRun({ text: c.next }); facts.settlements.push(await next.wait({ timeoutMs: 15000 })); await next.close();
    await session.close(); await agent.close(); await original.close(); await decoy.close(); await model.close();
  } finally { await sdk.close(); writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2)); }
  finish(work, "compressor", facts);
});

test("Workflow extension streams preserve node error owner cancel and close", { skip: !host, timeout: 60000 }, async () => {
  const { work, spec, sdk } = await setup("workflow"), facts = { calls: [], producer_completed: 0, producer_errors: [], callbacks_completed: 0, control_terminals: {cancel:0,close:0}, control_items: {} }, producers = [], started = {}, releases = {};
  try {
    const registration = await sdk.registerAgentComponent("proof-workflow", { kind: "agent_component", descriptor_version: 1, component: "workflow", name: "proof-workflow", capabilities: { supports_streaming: true } }, async call => {
      try {
      const request = call.invocation.input.call, mode = request.input.input;
      facts.calls.push({ operation: call.invocation.operation, component_operation: request.operation, input: mode, context: call.context });
      if (request.operation === "workflow_run") return { operation: "agent_component_call", value: { component: "workflow", result: { operation: "workflow_run", value: { output: spec.workflow_output } } } };
      assert.ok(call.stream); const writer = sdk.streamWriter(call.stream); started[mode].release();
      producers.push((async () => {
        try {
          await sleep();
          if(releases[mode]){
            await writer.chunk(spec.workflow_chunks[0]);
            await releases[mode].promise;
            await writer.complete(spec.workflow_complete);facts.control_terminals[mode]++;
          }else{
            for (const value of mode === "failure" ? [spec.workflow_error_chunk] : spec.workflow_chunks) await writer.chunk(value); if (mode === "failure") await writer.failed(spec.failure); else await writer.complete(spec.workflow_complete);
          }
        }
        catch (error) { facts.producer_errors.push(error.message); throw error; } finally { facts.producer_completed++; }
      })());
      return { outcome: "stream", stream: call.stream };
      } finally { facts.callbacks_completed++; }
    });
    const agent = await sdk.createAgent(), a = await agent.createSession(), b = await agent.createSession();
    facts.run = await a.invoke("workflow.extension.run", [registration.wire, "input🙂"]);
    for (const mode of ["success", "failure"]) {
      started[mode] = gate(); const stream = await a.invoke("workflow.extension.run_stream", [registration.wire, mode]);
      if (mode === "success") { await started[mode].promise; facts.callback_count_before_wrong_owner = facts.calls.length; facts.wrong_owner = await observedError(b.invoke("workflow.stream.next", [stream])); facts.callback_count_after_wrong_owner = facts.calls.length; }
      facts[mode] = await collect(a, stream);
    }
    for (const mode of ["cancel", "close"]) {
      started[mode] = gate(); releases[mode] = gate(); const stream = await a.invoke("workflow.extension.run_stream", [registration.wire, mode]); await started[mode].promise;
      // A real item proves the original source is open; its producer still
      // owns the same writer while the explicit release gate is held.
      facts.control_items[mode]=await a.invoke("workflow.stream.next",[stream]);
      if (mode === "cancel") { facts.cancel = await a.invoke("workflow.stream.cancel", [stream]); releases[mode].release(); facts.cancel_terminal = await a.invoke("workflow.stream.next", [stream]); }
      else { const closing = a.invoke("workflow.stream.close", [stream]); await sleep(); assert.equal(facts.control_terminals[mode],0); releases[mode].release(); facts.close = await closing; facts.close_again = await a.invoke("workflow.stream.close", [stream]); facts.after_close = await observedError(a.invoke("workflow.stream.next", [stream])); }
    }
    await Promise.all(producers); await a.close(); await b.close(); await agent.close(); await registration.close();
  } finally { for (const release of Object.values(releases)) release.release(); await sdk.close(); writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2)); }
  finish(work, "workflow", facts);
});

test("pure Workflow graph streams owner fence and canonical cancellation", { skip: !host, timeout: 30000 }, async () => {
  const { work, spec, sdk } = await setup("graph"), facts = { callback_count: 0 };
  try {
    const guard = await sdk.registerLlmClient("graph-model-guard", { kind: "llm_client", descriptor_version: 1, model_name: "fixture-model", supports_streaming: false }, async () => { facts.callback_count++; throw Error("pure router graph must not invoke model"); });
    const agent = await sdk.createAgent(), a = await agent.createSession(), b = await agent.createSession();
    const { resource: graph } = await a.invoke("workflow.graph.build", [JSON.stringify(spec.graph)]);
    facts.wrong_owner = await observedError(b.invoke("workflow.graph.run_stream", [graph, spec.state])); facts.wrong_cancel = await observedError(b.invoke("workflow.graph.cancel", [graph]));
    facts.events = await collect(a, await a.invoke("workflow.graph.run_stream", [graph, spec.state]));
    facts.cancel = await a.invoke("workflow.graph.cancel", [graph]); facts.cancelled_events = await collect(a, await a.invoke("workflow.graph.run_stream", [graph, spec.state]));
    await a.close(); await b.close(); await agent.close(); await guard.close();
  } finally { await sdk.close(); writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2)); }
  finish(work, "graph", facts);
});

test("Workflow observation retains the real late Completed payload", { skip: !host, timeout: 30000 }, async () => {
  const { work, spec, sdk } = await setup("observation"), release = gate(), observed = gate(), producers = [];
  const facts = { calls: [], physical_completed: 0, actual_complete_sent: 0 };
  const variant = value => value.value ?? value;
  const fields = value => {
    assert.ok(Array.isArray(variant(value).fields), "canonical variant fields missing");
    return Object.fromEntries(variant(value).fields.map(field => [field.name, fromWireValue(field.value)]));
  };
  try {
    const registration = await sdk.registerAgentComponent("observed-workflow", { kind: "agent_component", descriptor_version: 1, component: "workflow", name: "observed-workflow", capabilities: { supports_streaming: true } }, async call => {
      facts.calls.push(call);
      assert.equal(call.invocation.input.call.operation, "workflow_run_stream");
      assert.ok(call.stream);
      // These identities belong to the actual producer admitted in this
      // callback, independently of the RPC id and the façade stream handle.
      facts.binding = { run_id: `workflow-original-${randomUUID()}-中文🙂`, execution_id: `workflow-attempt-${randomUUID()}-🧪`, stream: call.stream, extension: call.extension };
      facts.error = { code: "framework_error", retryable: "never", message: "original Workflow producer remains held", operation: "_echo_agent/extension/invoke", details: { fields: [{ key: "observation", value: "not_settled" }, { key: "run_id", value: facts.binding.run_id }, { key: "execution_id", value: facts.binding.execution_id }] } };
      const writer = sdk.streamWriter(call.stream);
      const original = (async () => {
        await writer.failed(facts.error);
        observed.release();
        await release.promise;
        facts.physical_completed++;
        await writer.complete(spec.workflow_complete);
        facts.actual_complete_sent++;
      })();
      original.catch(() => {}); producers.push(original);
      return { outcome: "stream", stream: call.stream };
    });
    const agent = await sdk.createAgent(), session = await agent.createSession();
    const stream = await session.invoke("workflow.extension.run_stream", [registration.wire, "original held work"]);
    facts.facade_stream = stream;
    await observed.promise;
    await assert.rejects(session.invoke("workflow.stream.next", [stream]), error => {
      facts.observed = { ...errorFact(error), handle: error.handle, retryable: error.retryable };
      assert.equal(error.code, "framework_error"); assert.equal(error.retryable, "never");
      assert.equal(error.operation, "workflow.stream.next"); assert.deepEqual(error.handle, stream);
      assert.deepEqual(error.details?.fields, facts.error.details.fields);
      return true;
    });
    assert.equal(facts.physical_completed, 0); assert.equal(facts.actual_complete_sent, 0); assert.equal(facts.calls.length, 1);
    facts.held = { physical_completed: facts.physical_completed, actual_complete_sent: facts.actual_complete_sent };
    release.release(); await Promise.all(producers);
    assert.equal(facts.physical_completed, 1); assert.equal(facts.actual_complete_sent, 1);
    const item = await session.invoke("workflow.stream.next", [stream]); facts.item = item;
    assert.equal(variant(item).variant, "item", "real Completed payload was replaced by a generic terminal");
    const itemFields = fields(item); assert.equal(itemFields.sequence, "1"); assert.deepEqual(itemFields.stream, stream);
    const completed = itemFields.value;
    assert.equal(variant(completed).type_id, "echo_orchestration::workflow::WorkflowEvent");
    assert.equal(variant(completed).variant, "completed");
    // The shared verifier checks every field, including the canonical
    // duration wrapper preserved by this SDK's public wire decoder.
    const terminal = await session.invoke("workflow.stream.next", [stream]); facts.terminal = terminal;
    assert.equal(variant(terminal).variant, "complete"); assert.equal(fields(terminal).sequence, "2");
    facts.close = await session.invoke("workflow.stream.close", [stream]); assert.equal(facts.close, false);
    assert.equal(facts.calls.length, 1);
    await session.close(); await agent.close(); await registration.close();
  } finally {
    release.release();
    try { await Promise.all(producers); } finally { await sdk.close(); writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2)); console.log(`Workflow late-payload evidence: ${work}`); }
  }
  finish(work, "observation", facts);
});
