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

test("Workflow extension streams preserve node error owner cancel and close", { skip: !host, timeout: 60000 }, async () => {
  const { work, spec, sdk } = await setup("workflow"), facts = { calls: [], producer_completed: 0, producer_errors: [], callbacks_completed: 0 }, producers = [], started = {}, releases = {};
  try {
    const registration = await sdk.registerAgentComponent("proof-workflow", { kind: "agent_component", descriptor_version: 1, component: "workflow", name: "proof-workflow", capabilities: { supports_streaming: true } }, async call => {
      try {
      const request = call.invocation.input.call, mode = request.input.input;
      facts.calls.push({ operation: call.invocation.operation, component_operation: request.operation, input: mode, context: call.context });
      if (request.operation === "workflow_run") return { operation: "agent_component_call", value: { component: "workflow", result: { operation: "workflow_run", value: { output: spec.workflow_output } } } };
      assert.ok(call.stream); const writer = sdk.streamWriter(call.stream); started[mode].release();
      if (releases[mode]) await releases[mode].promise;
      else producers.push((async () => {
        try { await sleep(); for (const value of mode === "failure" ? [spec.workflow_error_chunk] : spec.workflow_chunks) await writer.chunk(value); if (mode === "failure") await writer.failed(spec.failure); else await writer.complete(spec.workflow_complete); }
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
      if (mode === "cancel") { facts.cancel = await a.invoke("workflow.stream.cancel", [stream]); releases[mode].release(); facts.cancel_terminal = await a.invoke("workflow.stream.next", [stream]); }
      else { const closing = a.invoke("workflow.stream.close", [stream]); await sleep(); releases[mode].release(); facts.close = await closing; facts.close_again = await a.invoke("workflow.stream.close", [stream]); facts.after_close = await observedError(a.invoke("workflow.stream.next", [stream])); }
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
