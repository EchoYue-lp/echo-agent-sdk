import test from "node:test";
import assert from "node:assert/strict";
import { spawn, execFileSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient } from "../dist/index.js";
import { fileSha256 } from "../../shared/testing/file-sha256.mjs";
const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw Error("required telemetry Host missing");
const root = process.env.ECHO_AGENT_SDK_TELEMETRY_ROOT ?? fileURLToPath(new URL("../../../target/telemetry-host", import.meta.url));
const fixture = fileURLToPath(new URL("../../shared/testing/telemetry_host_fixture.py", import.meta.url));

async function bounded(action, label) {
  let timer;
  try {
    return await Promise.race([Promise.resolve().then(action), new Promise((_, reject) => {
      timer = setTimeout(() => reject(Error(`${label} did not settle within fixture cleanup deadline`)), 5000);
    })]);
  } finally { clearTimeout(timer); }
}

async function fixtureScope(work, body, { close = async () => {}, evidence = () => {} } = {}) {
  let service, exited, primary, result;
  const failures = [];
  try {
    service = spawn("python3", [fixture, "--work", work, "--serve"], { stdio: ["ignore", "ignore", "inherit"] });
    exited = new Promise(resolve => {
      service.once("exit", (code, signal) => resolve({ code, signal }));
      service.once("error", error => resolve({ error }));
    });
    result = await body(service);
  } catch (error) { primary = error; }
  // Keep the genuine collector available until SDK close has been attempted.
  // No evidence/close failure is allowed to skip stop or original-child wait.
  for (const cleanup of [
    () => writeFileSync(join(work, "release-export"), "cleanup release"),
    () => bounded(close, "SDK close"),
    () => execFileSync("python3", [fixture, "--work", work, "--cleanup-host"], { stdio: "pipe" }),
    () => writeFileSync(join(work, "stop"), "stop"),
    async () => {
      if (!service) return;
      let outcome;
      try { outcome = await bounded(() => exited, "fixture stop"); }
      catch (error) {
        failures.push(error);
        service.kill("SIGTERM");
        try { outcome = await bounded(() => exited, "fixture TERM"); }
        catch (termError) {
          failures.push(termError);
          service.kill("SIGKILL");
          outcome = await bounded(() => exited, "fixture KILL");
        }
      }
      if (outcome.error) throw outcome.error;
      assert.equal(outcome.code, 0, `fixture exited by ${outcome.signal ?? outcome.code}`);
    },
    evidence,
  ]) {
    try { await cleanup(); } catch (error) { failures.push(error); }
  }
  if (primary && failures.length) throw new AggregateError([primary, ...failures], "telemetry scenario and cleanup failed", { cause: primary });
  if (primary) throw primary;
  if (failures.length) throw new AggregateError(failures, "telemetry cleanup failed");
  return result;
}

for (const consoleEnabled of [false, true]) test(`accepted telemetry exports real business records and flushes (console=${consoleEnabled})`, { skip: !host, timeout: 30000 }, async () => {
  mkdirSync(root, { recursive: true });
  const work = mkdtempSync(join(root, "typescript-"));
  const helper = (...args) => execFileSync("python3", [fixture, "--work", work, ...args], { encoding: "utf8" });
  helper(...(consoleEnabled ? ["--console"] : []));
  const facts = { stderr: [], errors: {}, work, host, host_sha256: undefined };
  let sdk;
  await fixtureScope(work, async service => {
    facts.host_sha256 = fileSha256(host);
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
    await failure("invalid_endpoint", "telemetry.init", ["", facts.scenario.service_name, false]);
    await failure("invalid_service", "telemetry.init", [facts.scenario.endpoint, " ", false]);
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
  }, {
    close: () => sdk?.close(),
    evidence: () => {
      writeFileSync(join(work, "observed.json"), JSON.stringify(facts, null, 2));
      console.log(`Telemetry evidence: ${work}`);
    },
  });
});

// These cases prove Host wire cancellation and physical close ownership. The
// scalar TS invoke API has no AbortSignal; cancel-rpc is the official ACP frame
// injected by the transparent fixture using the observed real request ID.
for (const mode of ["rpc_cancel", "input_eof", "public_close"]) test(`telemetry slow flush retains original Host owner (${mode})`, { skip: !host, timeout: 30000 }, async () => {
  mkdirSync(root, { recursive: true });
  const work = mkdtempSync(join(root, `typescript-wire-${mode}-`));
  execFileSync("python3", [fixture, "--work", work]);
  const barrier = async file => {
    const deadline = Date.now() + 10000;
    while (!existsSync(join(work, file))) {
      assert.ok(Date.now() < deadline, `fixture barrier ${file} timed out`);
      await new Promise(resolve => setTimeout(resolve, 5));
    }
  };
  const facts = { mode, work, host, host_sha256: undefined };
  let sdk, shutdown, close;
  await fixtureScope(work, async () => {
    facts.host_sha256 = fileSha256(host);
    await barrier("ready"); const scenario = JSON.parse(readFileSync(join(work, "scenario.json"), "utf8"));
    sdk = await EchoAgentClient.spawn({ hostCommand: "python3", args: [fixture, "--work", work, "--proxy-host", host], env: { RUST_LOG: "info", OTEL_METRIC_EXPORT_INTERVAL: "60000", OTEL_BSP_SCHEDULE_DELAY: "60000" }, onStderr: text => writeFileSync(join(work, "host.stderr"), text, { flag: "a" }) });
    const agent = await sdk.createAgent(), session = await agent.createSession(work);
    await session.invoke("telemetry.init", [scenario.endpoint, scenario.service_name, false]);
    const run = await session.startRun({ text: "Read probe.txt using read_file and consume its full content." });
    assert.equal((await run.wait({ timeoutMs: 15000 })).settled, true);
    facts.business = await run.get();
    writeFileSync(join(work, "hold-export"), "hold actual response");
    let shutdownSettled = false, closeSettled = false;
    shutdown = session.invoke("telemetry.shutdown", []).then(result => { shutdownSettled = true; facts.shutdown = result; }, error => { shutdownSettled = true; facts.shutdown_error = { code: error.code, message: error.message }; });
    await barrier("export-held");
    facts.status_while_export_held = await session.invoke("telemetry.status", []);
    assert.equal(shutdownSettled, false, "real shutdown cannot finish before actual export ACK");
    if (mode === "rpc_cancel") { writeFileSync(join(work, "cancel-rpc"), "cancel actual observed request"); await barrier("cancel-sent.json"); }
    if (mode !== "public_close") {
      writeFileSync(join(work, "close-input"), "EOF actual Host stdin");
      facts.close_trigger = "owned Host input EOF";
    } else {
      // This case exercises only the original public API. The fixture must
      // not send EOF, catch termination or supply a substitute close signal.
      facts.close_trigger = "TypeScript public sdk.close only";
    }
    if (mode !== "input_eof") close = sdk.close().then(() => { closeSettled = true; });
    // This is an observation barrier, not a new production timeout.
    await new Promise(resolve => setTimeout(resolve, 100));
    assert.equal(existsSync(join(work, "host-exit.json")), false, "Host exited while original exporter work was still held");
    if (close) assert.equal(closeSettled, false);
    facts.before_release = { shutdown_settled: shutdownSettled, close_settled: closeSettled, real_host_exited: false };
    writeFileSync(join(work, "release-export"), "release actual OTLP response");
    await barrier("host-exit.json");
    facts.exit = JSON.parse(readFileSync(join(work, "host-exit.json"), "utf8"));
    assert.equal(facts.exit.returncode, 0, "original Host must finish graceful close after true exporter completion");
    if (close) await close;
    await shutdown;
    facts.collector_after_exit = readFileSync(join(work, "otlp.jsonl"), "utf8");
    assert.ok(facts.collector_after_exit.includes("llm.calls"));
    if (mode === "rpc_cancel") {
      facts.cancel = JSON.parse(readFileSync(join(work, "cancel-sent.json"), "utf8"));
      const request = JSON.parse(readFileSync(join(work, "shutdown-request.json"), "utf8"));
      assert.equal(facts.cancel.method, "$/cancel_request");
      assert.equal(facts.cancel.params.requestId, request.id);
    }
  }, {
    close: () => sdk?.close(),
    evidence: () => {
      writeFileSync(join(work, "wire-observed.json"), JSON.stringify(facts, null, 2));
      console.log(`Telemetry wire evidence: ${work}`);
    },
  });
});


test("telemetry setup and cleanup faults reap the original fixture and collector", { skip: !process.env.ECHO_AGENT_SDK_OTLP_COLLECTOR, timeout: 30000 }, async () => {
  mkdirSync(root, { recursive: true });
  const work = mkdtempSync(join(root, "typescript-cleanup-fault-"));
  execFileSync("python3", [fixture, "--work", work]);
  let service, collectorPid, primary;
  const closeFailure = Error("controlled SDK close failure");
  const evidenceFailure = Error("controlled evidence write failure");
  await assert.rejects(fixtureScope(work, async owned => {
    service = owned;
    const deadline = Date.now() + 10000;
    while (!existsSync(join(work, "ready"))) {
      assert.equal(owned.exitCode, null);
      assert.ok(Date.now() < deadline, "fixture ready timeout");
      await new Promise(resolve => setTimeout(resolve, 5));
    }
    collectorPid = JSON.parse(readFileSync(join(work, "collector.json"), "utf8")).pid;
    assert.ok(collectorPid > 0 && service.pid > 0);
    try { fileSha256(join(work, "missing-host")); } catch (error) { primary = error; throw error; }
  }, {
    close: async () => { throw closeFailure; },
    evidence: () => { throw evidenceFailure; },
  }), error => {
    assert.ok(error instanceof AggregateError);
    assert.equal(error.cause, primary);
    assert.equal(primary.code, "ENOENT");
    assert.ok(error.errors.includes(closeFailure) && error.errors.includes(evidenceFailure));
    return true;
  });
  assert.equal(service.exitCode, 0);
  for (const pid of [service.pid, collectorPid]) assert.throws(() => process.kill(pid, 0), { code: "ESRCH" });
  assert.equal(existsSync(join(work, "stop")), true);
});
