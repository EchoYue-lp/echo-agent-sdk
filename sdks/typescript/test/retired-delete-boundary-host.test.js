import test from "node:test";
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, EchoAgentError } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw new Error("required retirement Host is missing");
const DELETE = "echo_agent::state::delete_persisted_conversation";
const METHOD = "_echo_agent/facade/invoke";
// 原accepted DELETE摘要只作为直接拒绝探针的历史输入，不生成新可执行签名。
const OLD_SIGNATURE = "sha256:97b7ff491c492fa6e3b26e6130c5f51285ff1f1608ae64d447de744152c501e2";
const proxy = fileURLToPath(new URL("../../python/tests/protocol_host_proxy.py", import.meta.url));
const runtimeRoot = process.env.ECHO_AGENT_SDK_RETIRED_DELETE_RUNTIME_ROOT
  ?? fileURLToPath(new URL("../../../target/retired-delete-boundary", import.meta.url));

function requests(work) {
  return readFileSync(join(work, "client-frames.jsonl"), "utf8").split("\n").filter(Boolean)
    .map(line => JSON.parse(line)).filter(frame => frame.id !== undefined && typeof frame.method === "string")
    .map(frame => ({ id: frame.id, method: frame.method, operation: frame.params?.operation }));
}
function unavailable(error) {
  assert.ok(error instanceof EchoAgentError);
  assert.equal(error.code, "feature_unavailable");
  assert.equal(error.retryable, "never");
  assert.equal(error.operation, DELETE);
  assert.match(error.message, /unmanaged conversation deletion is retired/u);
  return true;
}
async function scenario(label, run) {
  mkdirSync(runtimeRoot, { recursive: true });
  const work = mkdtempSync(join(runtimeRoot, `typescript-${label}-`));
  const proof = { label, epoch: process.env.ECHO_AGENT_SDK_TEST_EPOCH ?? "unspecified", work };
  let sdk;
  try {
    sdk = await EchoAgentClient.spawn({ hostCommand: "python3", args: [proxy, "--host", host, "--work", work, "--mode", "normal"] });
    // 必须先有原完整生成和export的真实退休后catalog；本测试从不改catalog/seed。
    assert.equal(sdk.catalog.operations().some(item => item.operation === DELETE), false, "requires actual retired catalog");
    const agent = await sdk.createAgent();
    const session = await agent.createSession();
    await agent.describe();
    await run({ sdk, agent, session, work, proof });
    proof.passed = true;
  } catch (error) {
    proof.failure = { name: error.name, code: error.code, message: error.message };
    throw error;
  } finally {
    try { await sdk?.close(); }
    finally { writeFileSync(join(work, "boundary-proof.json"), JSON.stringify(proof, null, 2) + "\n"); }
  }
}

test("retired DELETE rejects all public local entry points without submitted request IDs", { skip: !host }, async () => scenario("local", async ({ sdk, agent, session, work, proof }) => {
  const before = requests(work);
  await assert.rejects(sdk.invoke(DELETE, agent.wire, [session.wire, "scope"]), unavailable);
  await assert.rejects(sdk.call(DELETE, agent.wire, [session.wire, "scope"]), unavailable);
  await assert.rejects(session.invoke(DELETE, ["scope"]), unavailable);
  await agent.describe(); // 真正RPC屏障，确保原proxy已处理此前所有提交。
  const delta = requests(work).slice(before.length);
  proof.submitted = delta;
  assert.equal(delta.length, 1, "local refusal must submit only the explicit observation barrier");
  assert.equal(delta[0]?.method, "_echo_agent/agent/describe");
}));

test("retired DELETE matching stays exact and closed Session keeps its priority", { skip: !host }, async () => scenario("negative", async ({ sdk, agent, session, work, proof }) => {
  const before = requests(work);
  for (const unknown of [`${DELETE}_unknown`, `prefix::${DELETE}`, DELETE.toUpperCase()]) {
    assert.throws(() => sdk.catalog.signature(unknown), error => !(error instanceof EchoAgentError) && error.message.includes(unknown));
    await assert.rejects(sdk.invoke(unknown, agent.wire), error => !(error instanceof EchoAgentError) && error.message.includes(unknown));
    await assert.rejects(sdk.call(unknown, agent.wire), error => !(error instanceof EchoAgentError) && error.message.includes(unknown));
    await assert.rejects(session.invoke(unknown), error => !(error instanceof EchoAgentError) && error.message.includes(unknown));
  }
  await agent.describe();
  const delta = requests(work).slice(before.length);
  assert.equal(delta.length, 1);
  assert.equal(delta[0]?.method, "_echo_agent/agent/describe");
  proof.unknownSubmissions = delta;
  await session.close();
  const closedBefore = requests(work);
  for (const operation of [DELETE, `${DELETE}_unknown`]) {
    await assert.rejects(session.invoke(operation), error => error instanceof EchoAgentError && error.code === "closed_handle");
  }
  await agent.describe();
  const closedDelta = requests(work).slice(closedBefore.length);
  assert.equal(closedDelta.length, 1);
  assert.equal(closedDelta[0]?.method, "_echo_agent/agent/describe");
  proof.closedSessionSubmissions = closedDelta;
}));

test("direct Host retired DELETE is typed while other unknown and malformed operations retain errors", { skip: !host }, async () => scenario("host", async ({ sdk, proof }) => {
  const params = { operation: DELETE, signature_digest: OLD_SIGNATURE, handle: null, arguments: [] };
  await assert.rejects(sdk.request(METHOD, params), unavailable);
  await assert.rejects(sdk.request(METHOD, { ...params, operation: `${DELETE}_unknown` }), error => {
    assert.ok(error instanceof EchoAgentError); assert.equal(error.code, "invalid_value"); assert.equal(error.retryable, "never"); return true;
  });
  await assert.rejects(sdk.request(METHOD, { ...params, arguments: "not-an-array" }), error => {
    assert.ok(error instanceof EchoAgentError); assert.equal(error.code, "invalid_request"); return true;
  });
  proof.directHostControls = ["retired", "unknown", "malformed"];
}));
