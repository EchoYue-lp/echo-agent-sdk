import test from "node:test";
import assert from "node:assert/strict";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, EchoAgentError } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw new Error("required tool behavior Host binary missing");
const fixturePath = fileURLToPath(new URL("../../shared/testing/tool_behavior_fixture.py", import.meta.url));
test("six tool families preserve native results, failures and real local effects", { skip: !host, timeout: 120000 }, async () => {
  const work = mkdtempSync(join(tmpdir(), "sdk-ts-tool-behaviors-"));
  const fixture = spawn("python3", [fixturePath, "--work", work], { stdio: ["ignore", "ignore", "inherit"] });
  let sdk, passed = false;
  try {
    const deadline = Date.now() + 10000;
    while (!existsSync(join(work, "ready"))) {
      assert.ok(Date.now() < deadline && fixture.exitCode === null, "fixture preparation failed");
      await new Promise(resolve => setTimeout(resolve, 10));
    }
    const scenario = JSON.parse(readFileSync(join(work, "scenario.json"), "utf8"));
    sdk = await EchoAgentClient.spawn({ hostCommand: host, env: { ...process.env, TMPDIR: join(work, "artifact-temp") }, args: ["--config", join(work, "host.json")], onStderr: chunk => process.stderr.write(chunk) });
    const agent = await sdk.createAgent();
    const sessions = { a: await agent.createSession(scenario.sessions.a), b: await agent.createSession(scenario.sessions.b) };
    const results = [];
    for (const step of scenario.steps) {
      const session = sessions[step.session];
      if (step.action === "close") {
        await session.close(); results.push({ id: step.id, outcome: "closed" }); continue;
      }
      try {
        const value = await sdk.call(step.operation, session.wire, step.arguments);
        results.push({ id: step.id, outcome: "result", value });
      } catch (error) {
        assert.ok(error instanceof EchoAgentError, String(error));
        results.push({ id: step.id, outcome: "error", code: error.code, message: error.message, operation: error.operation, retryable: error.retryable, details: error.details });
      }
    }
    await sessions.b.close(); await agent.close(); await sdk.close(); sdk = undefined;
    writeFileSync(join(work, "results.json"), JSON.stringify(results, (_, value) => typeof value === "bigint" ? value.toString() : value));
    const verified = spawnSync("python3", [fixturePath, "--work", work, "--verify", join(work, "results.json")], { encoding: "utf8" });
    assert.equal(verified.status, 0, `${verified.stderr}\n${verified.stdout}\nEvidence: ${work}`);
    const evidence = JSON.parse(verified.stdout);
    assert.equal(evidence.capability_families, 6); assert.equal(evidence.native_calls, scenario.steps.length);
    process.stdout.write(`Tool behavior verified: ${JSON.stringify(evidence)}\n`);
    passed = true;
  } finally {
    await sdk?.close(); fixture.kill();
    await new Promise(resolve => fixture.exitCode !== null || fixture.signalCode !== null ? resolve() : fixture.once("exit", resolve));
    if (passed) rmSync(work, { recursive: true, force: true }); else process.stderr.write(`Tool behavior evidence retained: ${work}\n`);
  }
});
