import test from "node:test";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, SessionHandle, validateManaged } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw Error("required retirement Host missing");
const CLEAR = "echo_agent::state::clear_persisted_runtime_incarnation";
const DELETE = "echo_agent::state::delete_persisted_conversation";
const RETIRE = "runtime_retire_generation_with_context";
const root = process.env.ECHO_AGENT_SDK_RETIREMENT_RUNTIME_ROOT ?? fileURLToPath(new URL("../../../target/managed-retirement", import.meta.url));
const durable = ({ observations, supported_operations, calls, ...state }) => state;
async function http(url, path, payload) {
  const response = await fetch(url + path, { method: payload === undefined ? "GET" : "POST", headers: { "Content-Type": "application/json" }, ...(payload === undefined ? {} : { body: JSON.stringify(payload) }) });
  assert.equal(response.status, 200);
  return response.json();
}
const fact = error => ({ code: error.code, operation: error.operation, message: error.message, details: error.details });
for (const enabled of [false, true]) test(`accepted managed exact clear retirement enabled=${enabled}`, { skip: !host, timeout: 60000 }, async () => {
  mkdirSync(root, { recursive: true });
  const work = mkdtempSync(join(root, `typescript-retirement-${enabled}-`));
  const backend = spawn("python3", [fileURLToPath(new URL("../../shared/testing/managed_persistence_backend.py", import.meta.url)), "--work", work, ...(enabled ? ["--enable-retirement"] : [])]);
  const proof = { enabled, errors: [] }; let sdk;
  try {
    const end = Date.now() + 5000;
    while (!existsSync(join(work, "ready"))) { assert.ok(Date.now() < end, "backend ready timeout"); await new Promise(resolve => setTimeout(resolve, 10)); }
    const { url } = JSON.parse(readFileSync(join(work, "backend.json"), "utf8"));
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")] });
    const register = async client => {
      const registrations = [];
      for (const component of ["conversation_store", "runtime_state_store"]) registrations.push(await client.registerAgentComponent("retirement-" + component, {
      kind: "agent_component", descriptor_version: 1, component, name: "retirement-" + component,
      capabilities: { conversation_projection: component === "conversation_store" ? "atomic_v1" : "unsupported", runtime_state: component === "runtime_state_store" ? "revisioned_v1" : "unsupported", persistence_call: "absolute_deadline_v1" }
    }, async call => {
      const response = await http(url, "/call", call.invocation.input.call);
      if (response.kind === "error") return { outcome: "error", error: response.error };
      // The public SDK's closed managed DTO converter validates the complete
      // runtime retirement receipt before the normal extension response path.
      if (response.result.operation === RETIRE) validateManaged("RuntimeGenerationRetireReceiptWire", response.result.value.receipt);
      return { operation: "agent_component_call", value: { component, result: response.result } };
      }));
      return registrations;
    };
    const registrations = await register(sdk);
    const agent = await sdk.createAgent(), session = await agent.createSession();
    const run = await session.startRun({ text: "managed retirement 中文🙂" });
    assert.equal((await run.wait({ timeoutMs: 40000 })).terminal.status, "completed");
    const before = await http(url, "/inspect");
    assert.equal(Object.keys(before.runtime).length, 1);
    const [identity, snapshot] = Object.entries(before.runtime)[0], scope = snapshot.scope.scope_id;
    assert.ok(snapshot.checkpoint && Object.keys(before.receipts).length && before.counters.projected_items > 0);
    assert.ok(!JSON.parse(snapshot.checkpoint.messages_json).pending_transcript_projection);
    proof.before = before;
    await assert.rejects(session.invoke(DELETE, [scope]), error => {
      assert.equal(error.code, "framework_error"); assert.equal(error.operation, DELETE);
      assert.match(error.message, /managed conversation deletion requires/u);
      assert.ok(error.details.fields.some(field => field.key === "framework_failure_code" && field.value === "runtime_state"));
      proof.errors.push(fact(error)); return true;
    });
    const afterDelete = await http(url, "/inspect");
    assert.deepEqual(durable(afterDelete), durable(before)); assert.deepEqual(afterDelete.calls, before.calls);
    let after;
    if (!enabled) {
      await assert.rejects(session.invoke(CLEAR, [scope, identity]), error => {
        assert.equal(error.operation, CLEAR); assert.match(error.message, /does not implement runtime_retire_generation_with_context/u);
        proof.errors.push(fact(error)); return true;
      });
      after = await http(url, "/inspect"); assert.deepEqual(durable(after), durable(before)); assert.ok(!after.supported_operations.includes(RETIRE));
    } else {
      const cleared = await session.invoke(CLEAR, [scope, identity]);
      assert.deepEqual(cleared, { scope_id: scope, runtime_state_id: identity, checkpoint_removed: true });
      after = await http(url, "/inspect"); const retired = after.runtime[identity];
      assert.equal(retired.checkpoint, null); assert.equal(retired.runtime_state_id, identity);
      assert.deepEqual(retired.scope, after.scopes[scope]);
      assert.deepEqual(retired.scope, { ...snapshot.scope, revision: String(BigInt(snapshot.scope.revision) + 1n) });
      assert.equal(retired.scope.lifecycle, "active");
      for (const key of ["conversations", "receipts", "batches"]) assert.deepEqual(after[key], before[key]);
      assert.deepEqual(after.counters, { ...before.counters, mutations: before.counters.mutations + 1 });
      const request = after.cas_proofs[identity].request;
      assert.equal(request.scope_id, scope); assert.equal(request.runtime_state_id, identity);
      assert.equal(request.expected_scope_revision, snapshot.scope.revision); assert.deepEqual(request.expected_state_version, snapshot.version);
      assert.deepEqual(retired.version, { retired: { revision: String(BigInt(snapshot.version.managed.revision) + 1n), operation_id: request.operation_id } });
      proof.clear_receipt = cleared;
      const replayed = await session.invoke(CLEAR, [scope, identity]);
      assert.deepEqual(replayed, { ...cleared, checkpoint_removed: false });
      assert.deepEqual(durable(await http(url, "/inspect")), durable(after)); proof.replayed_clear_receipt = replayed;
    }
    const calls = after.calls.filter(call => call.operation === RETIRE); assert.equal(calls.length, 1);
    assert.equal(calls[0].input.request.runtime_state_id, identity);
    assert.equal(typeof calls[0].input.context.absolute_deadline_unix_ms, "string"); assert.ok(BigInt(calls[0].input.context.absolute_deadline_unix_ms) > 0n);
    proof.after = after;
    const previousGeneration = session.wire.generation;
    await run.close(); await session.close(); for (const registration of registrations) await registration.close(); await agent.close();
    if (enabled) {
      await sdk.close();
      sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")] });
      const stores = await register(sdk), agent = await sdk.createAgent();
      const response = await sdk.request("_echo_agent/session/load", { agent: agent.wire, session_id: scope, working_dir: null });
      const restored = new SessionHandle(sdk, agent.wire, response.session, response.acp_session_id, response.task_run);
      assert.notEqual(restored.wire.generation, previousGeneration);
      assert.deepEqual(await restored.invoke(CLEAR, [scope, identity]), { ...proof.clear_receipt, checkpoint_removed: false });
      const recovered = await http(url, "/inspect"); assert.deepEqual(durable(recovered), durable(after));
      assert.equal(recovered.calls.filter(call => call.operation === RETIRE).length, 1);
      proof.after_host_restart = recovered; proof.host_generations = [previousGeneration, restored.wire.generation];
      await restored.close(); for (const registration of stores) await registration.close(); await agent.close();
    }
  } finally {
    await sdk?.close(); backend.kill(); await new Promise(resolve => backend.exitCode !== null ? resolve() : backend.once("exit", resolve));
    writeFileSync(join(work, "typescript-observed.json"), JSON.stringify(proof, null, 2)); console.log(`Managed retirement evidence: ${work}`);
  }
});
