import test from "node:test";
import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { mkdtemp, readFile, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw new Error("required parent-close Host is missing");
const fixture = fileURLToPath(new URL("../../shared/testing/session_run_idempotency_fixture.py", import.meta.url));
const deferred = () => { let resolve; return { promise: new Promise(done => { resolve = done; }), resolve: () => resolve() }; };
async function bounded(promise) {
  let timer;
  try {
    return await Promise.race([promise, new Promise((_, reject) => { timer = setTimeout(() => reject(new Error("parent-close Host barrier timed out")), 5000); })]);
  } finally { clearTimeout(timer); }
}

for (const window of [1, undefined]) test(`real Host ${window === 1 ? "settled ACK-paused" : "active default128"} parent close ends child scope and preserves another Session`, {
  skip: !host, timeout: 20000,
}, async () => {
  const work = await mkdtemp(join(tmpdir(), "echo-sdk-parent-ts-"));
  await promisify(execFile)(process.env.PYTHON ?? "python3", [fixture, "--work", work], { timeout: 5000 });
  const config = JSON.parse(await readFile(join(work, "host.json"), "utf8"));
  if (window !== undefined) config.sdk_profile.limits = { max_outstanding_live_events: window };
  await writeFile(join(work, "host.json"), JSON.stringify(config));
  const spec = JSON.parse(await readFile(join(work, "scenario.json"), "utf8"));
  const entered = deferred();
  const release = deferred();
  let calls = 0;
  let cancelled = false;
  let finalized = 0;
  let sdk;
  try {
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")] });
    const registration = await sdk.registerLlmClient("parent-close-native", spec.descriptor, async (_call, signal) => {
      if (++calls === 1 && window !== 1) {
        const onAbort = () => { cancelled = true; release.resolve(); };
        signal.addEventListener("abort", onAbort, { once: true });
        if (signal.aborted) onAbort();
        entered.resolve();
        try { await release.promise; }
        finally { signal.removeEventListener("abort", onAbort); finalized++; }
      }
      return { operation: "llm_chat", value: structuredClone(spec.response) };
    });
    const agent = await sdk.createAgent();
    const parent = await agent.createSession();
    const independent = await agent.createSession();
    const run = await parent.startRun({ text: "active parent scope 中文🌊" });
    if (window === 1) {
      assert.equal((await bounded(run.wait())).settled, true);
    } else {
      await bounded(entered.promise);
      assert.equal((await run.get()).status, "running");
    }
    await bounded(parent.close());
    if (window !== 1) {
      assert.equal(cancelled, true, "the original native callback must observe cancellation");
      assert.equal(finalized, 1, "parent close must wait for original provider cleanup");
    }
    const events = await bounded((async () => { const values = []; for await (const event of run.events) values.push(event); return values; })());
    assert.ok(events.length > 0, "buffered live facts must remain visible");
    if (window === 1) assert.ok(events.some(event => "gap" in event), "ACK-paused delivery must reach its real gap");
    assert.ok(events.filter(event => "envelope" in event && ["final_answer", "cancelled", "error"].includes(event.envelope.payload.event_type)).length <= 1);
    assert.throws(() => run.get(), error => error.code === "closed_handle");
    await assert.rejects(sdk.request("_echo_agent/run/get", { run: run.wire }), error => error.code === "closed_handle");
    await run.close();
    const other = await independent.startRun({ text: "independent Session still works" });
    const settled = await bounded(other.wait());
    assert.equal(settled.settled, true);
    assert.equal(settled.terminal.status, "completed");
    await independent.close();
    await agent.close();
    await registration.close();
    await writeFile(join(work, "observed.json"), JSON.stringify({ window: window ?? 128, cancelled, finalized, events, independent: settled }, null, 2));
    console.log("parent-close real Host evidence:", work);
  } finally {
    release.resolve();
    await sdk?.close();
  }
});
