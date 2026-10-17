import assert from "node:assert/strict";
import { spawn, execFileSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";
import { EchoAgentClient } from "../dist/client.js";

const host = process.env.ECHO_AGENT_SDK_HOST?.trim();
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw new Error("required Session rules Host missing");
const fixture = fileURLToPath(new URL("../../shared/testing/session_rules_prompt_fixture.py", import.meta.url));

test("Session cwd rules reach actual model and isolate new Session reload", { skip: !host, timeout: 30000 }, async () => {
  const root = resolve(process.env.ECHO_AGENT_SDK_SESSION_RULES_ROOT ?? "../../target/session-rules-prompt");
  mkdirSync(root, { recursive: true });
  const work = mkdtempSync(join(root, "typescript-"));
  const helper = (...args) => execFileSync("python3", [fixture, "--work", work, ...args], { encoding: "utf8" });
  helper();
  const model = spawn("python3", [fixture, "--work", work, "--serve"], { stdio: ["ignore", "ignore", "inherit"] });
  let sdk;
  const rows = [];
  try {
    const deadline = Date.now() + 10000;
    while (!existsSync(join(work, "ready"))) {
      assert.equal(model.exitCode, null, "model fixture exited");
      assert.ok(Date.now() < deadline, "model fixture ready timeout");
      await new Promise((resolve) => setTimeout(resolve, 10));
    }
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")] });
    const agent = await sdk.createAgent();
    for (const step of JSON.parse(readFileSync(join(work, "scenario.json"), "utf8"))) {
      if (step.id === "a2") helper("--mutate");
      const session = await agent.createSession(join(work, step.cwd));
      const run = await session.startRun({ mode: "chat", text: step.user });
      assert.equal((await run.wait({ timeoutMs: 15000 })).settled, true);
      rows.push({ id: step.id, outcome: await run.outcomeStatus(), replay: await run.replay() });
      await session.close();
    }
    await agent.close();
    writeFileSync(join(work, "native-results.json"), JSON.stringify(rows));
    const proof = JSON.parse(helper("--verify"));
    assert.equal(proof.cases, 3);
    console.log(JSON.stringify({ work, ...proof }));
  } finally {
    if (sdk) await sdk.close();
    model.kill("SIGTERM");
    await new Promise((resolve) => model.once("exit", resolve));
  }
});
