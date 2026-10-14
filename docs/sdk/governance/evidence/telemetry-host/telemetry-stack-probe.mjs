import test from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient } from "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/typescript/dist/index.js";
const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw Error("required telemetry Host missing");
const root = process.env.ECHO_AGENT_SDK_TELEMETRY_ROOT ?? fileURLToPath(new URL("../../../target/telemetry-host", import.meta.url));
const fixture = "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/shared/testing/telemetry_host_fixture.py";

test("accepted telemetry initializes through the real Host", { skip: !host, timeout: 30000 }, async () => {
  mkdirSync(root, { recursive: true });
  const work = mkdtempSync(join(root, "typescript-"));
  const prepared = spawnSync("python3", [fixture, "--work", work], { encoding: "utf8" });
  assert.equal(prepared.status, 0, prepared.stderr);
  const scenario = JSON.parse(readFileSync(join(work, "scenario.json"), "utf8"));
  const facts = { stderr: [], work, host, host_sha256: createHash("sha256").update(readFileSync(host)).digest("hex"), scenario };
  let sdk;
  try {
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")], onStderr: text => facts.stderr.push(text) });
    const agent = await sdk.createAgent(), session = await agent.createSession();
    facts.before = await session.invoke("telemetry.status", []);
    writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2));
    let timer;
    try { facts.init = await Promise.race([
      session.invoke("telemetry.init", [scenario.endpoint, scenario.service_name, scenario.enable_console]),
      new Promise((_, reject) => { timer = setTimeout(() => reject(Object.assign(new Error("accepted telemetry.init did not finish within observation window"), { code: "observation_timeout" })), 10000); }),
    ]); }
    catch (error) { facts.error = { code: error.code, message: error.message, retryable: error.retryable, operation: error.operation, details: error.details, handle: error.handle }; }
    finally { clearTimeout(timer); }
    if (facts.error?.code !== "observation_timeout") facts.after = await session.invoke("telemetry.status", []);
    else facts.after_unavailable = "initialization request still pending; status was not queried through the blocked dispatcher";
    writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2));
    if (!facts.error) { await session.close(); await agent.close(); }
    assert.equal(facts.error, undefined, `accepted initialization failed: ${JSON.stringify(facts.error)}`);
    assert.equal(facts.init?.initialized, true);
    assert.equal(facts.after.initialized, true);
  } finally {
    await sdk?.close();
    writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2));
    console.log(`Telemetry evidence: ${work}`);
  }
});
