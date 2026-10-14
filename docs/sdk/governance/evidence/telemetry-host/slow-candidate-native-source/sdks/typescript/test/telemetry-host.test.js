import test from "node:test";
import assert from "node:assert/strict";
import { spawn, execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient } from "../dist/index.js";
const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw Error("required telemetry Host missing");
const root = process.env.ECHO_AGENT_SDK_TELEMETRY_ROOT ?? fileURLToPath(new URL("../../../target/telemetry-host", import.meta.url));
const fixture = fileURLToPath(new URL("../../shared/testing/telemetry_host_fixture.py", import.meta.url));

for (const consoleEnabled of [false, true]) test(`accepted telemetry exports real business records and flushes (console=${consoleEnabled})`, { skip: !host, timeout: 30000 }, async () => {
  mkdirSync(root, { recursive: true });
  const work = mkdtempSync(join(root, "typescript-"));
  const helper = (...args) => execFileSync("python3", [fixture, "--work", work, ...args], { encoding: "utf8" });
  helper(...(consoleEnabled ? ["--console"] : []));
  const service = spawn("python3", [fixture, "--work", work, "--serve"], { stdio: ["ignore", "ignore", "inherit"] });
  const serviceExit = new Promise(resolve => service.once("exit", resolve));
  const facts = { stderr: [], errors: {}, work, host, host_sha256: createHash("sha256").update(readFileSync(host)).digest("hex") };
  let sdk;
  try {
    const deadline = Date.now() + 10000;
    while (!existsSync(join(work, "ready"))) {
      assert.equal(service.exitCode, null, "collector/model fixture exited");
      assert.ok(Date.now() < deadline, "collector/model ready timeout");
      await new Promise(resolve => setTimeout(resolve, 10));
    }
    facts.scenario = JSON.parse(readFileSync(join(work, "scenario.json"), "utf8"));
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")], env: { RUST_LOG: "info", OTEL_METRIC_EXPORT_INTERVAL: "60000", OTEL_BSP_SCHEDULE_DELAY: "60000" }, onStderr: text => facts.stderr.push(text) });
    const agent = await sdk.createAgent(), session = await agent.createSession(work);
    const failure = async (name, op, args) => {
      try { await session.invoke(op, args); assert.fail(`${op} must fail`); }
      catch (error) { assert.ok(error.code, String(error)); facts.errors[name] = error.code; }
    };
    facts.before = await session.invoke("telemetry.status", []);
    await failure("shutdown_before", "telemetry.shutdown", []);
    await failure("invalid_console", "telemetry.init", [facts.scenario.endpoint, facts.scenario.service_name, "false"]);
    facts.init = await session.invoke("telemetry.init", [facts.scenario.endpoint, facts.scenario.service_name, consoleEnabled]);
    facts.after = await session.invoke("telemetry.status", []);
    await failure("repeat_init", "telemetry.init", [facts.scenario.endpoint, facts.scenario.service_name, consoleEnabled]);
    const run = await session.startRun({ mode: "chat", text: "Read probe.txt using read_file and consume its full content." });
    assert.equal((await run.wait({ timeoutMs: 15000 })).settled, true);
    facts.run_outcome = await run.outcomeStatus(); facts.replay = await run.replay();
    facts.shutdown = await session.invoke("telemetry.shutdown", []);
    // Reading the collector file immediately after return proves flush, rather
    // than a later periodic export or asynchronous client cleanup.
    facts.collector_at_shutdown = readFileSync(join(work, "otlp.jsonl"), "utf8");
    facts.after_shutdown = await session.invoke("telemetry.status", []);
    await failure("repeat_shutdown", "telemetry.shutdown", []);
    await session.close(); await agent.close();
    writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2));
    const proof = JSON.parse(helper("--verify"));
    assert.equal(proof.shutdown_flush, true); assert.equal(proof.business_file_content_consumed, true);
  } finally {
    writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2));
    await sdk?.close();
    writeFileSync(join(work, "stop"), "stop");
    const deadline = Date.now() + 5000;
    while (service.exitCode === null && Date.now() < deadline) await new Promise(resolve => setTimeout(resolve, 10));
    if (service.exitCode === null) service.kill("SIGTERM");
    await serviceExit;
    console.log(`Telemetry evidence: ${work}`);
  }
});
