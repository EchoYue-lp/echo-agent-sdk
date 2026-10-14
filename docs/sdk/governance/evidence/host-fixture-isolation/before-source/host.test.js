import { fileURLToPath } from "node:url";
import test from "node:test";
import assert from "node:assert/strict";
import { EchoAgentClient } from "../dist/index.js";

test("source SDK reaches a real Host when configured", { skip: !process.env.ECHO_AGENT_SDK_HOST }, async () => {
  let sdk;
  try {
    sdk = await EchoAgentClient.spawn({
      hostCommand: process.env.ECHO_AGENT_SDK_HOST,
      args: ["--config", process.env.ECHO_AGENT_SDK_CONFIG],
      onStderr: (chunk) => process.stderr.write(chunk),
    });
    const registration = await sdk.registerLlmClient(
      "typescript-smoke-llm",
      {
        kind: "llm_client",
        descriptor_version: 1,
        model_name: "typescript-smoke-model",
        supports_streaming: false,
      },
      async (call) => {
        assert.equal(call.invocation.operation, "llm_chat");
        assert.ok(Array.isArray(call.invocation.input.messages));
        assert.ok(call.invocation.input.messages.length > 0);
        return {
          operation: "llm_chat",
          value: {
            message: {
              role: "assistant",
              content: { kind: "string", value: "typescript-smoke" },
            },
            finish_reason: "stop",
            raw: { kind: "map", value: [] },
          },
        };
      },
    );
    const agent = await sdk.createAgent();
    const session = await agent.createSession();
    assert.equal(await session.invoke("echo_core::agent::Agent::name"), "echo-agent");
    await session.invoke("memory.store.put", [["sdk"], "typescript", "ok"]);
    const stored = await session.invoke("memory.store.get", [["sdk"], "typescript"]);
    assert.equal(stored.value, "ok");
    const telemetry = await sdk.call("telemetry.status", undefined);
    assert.equal(typeof telemetry.initialized, "boolean");
    const classifiedOutcome = await sdk.classifyTurnOutcome({
      kind: "variant",
      value: {
        type_id: "echo_sdk_protocol::methods::AgentEventWire",
        variant: "final_answer",
        fields: [{ name: "text", value: { kind: "string", value: "classified" } }],
      },
    });
    assert.deepEqual(classifiedOutcome, {
      kind: "variant",
      value: {
        type_id: "echo_orchestration::runtime::turn_driver::TurnOutcome",
        variant: "completed",
        fields: [],
      },
    });
    const nonTerminalOutcome = await sdk.classifyTurnOutcome({
      kind: "variant",
      value: {
        type_id: "echo_sdk_protocol::methods::AgentEventWire",
        variant: "token",
        fields: [{ name: "text", value: { kind: "string", value: "partial" } }],
      },
    });
    assert.equal(nonTerminalOutcome, null);
    const updates = session.updates();
    const firstUpdate = updates[Symbol.asyncIterator]().next();
    const prompt = await session.prompt("hello");
    assert.equal(prompt.stopReason, "end_turn");
    const update = await firstUpdate;
    assert.equal(update.done, false);
    assert.equal(update.value.sessionId, session.acpSessionId);
    const run = await session.startRun({ mode: "chat", text: "run smoke" });
    const observedEvents = (async () => {
      const items = [];
      for await (const item of run.events) {
        items.push(item);
      }
      return items;
    })();
    const waited = await run.wait();
    assert.equal(waited.settled, true);
    const events = await observedEvents;
    assert.ok(events.length > 0);
    assert.equal(events.at(-1).envelope.payload.event_type, "final_answer");
    assert.ok(events.every((item) => item.stream.id === run.stream.id));
    const runState = await run.get();
    assert.equal(runState.status, "completed");
    assert.equal(await run.status(), "completed");
    assert.equal(await run.outcomeStatus(), "completed");
    const usage = await run.usage();
    assert.equal(typeof usage.duration_ms, "string");
    assert.match(usage.duration_ms, /^\d+$/u);
    assert.ok(usage.tokens_used === null || typeof usage.tokens_used === "string");
    assert.ok(usage.iterations === null || typeof usage.iterations === "string");
    await registration.close();
    await session.close();
    await agent.close();
  } finally {
    await sdk?.close();
  }
});

test("parallel Agent feeds isolate cancellation and preserve B natural terminal", { skip: !process.env.ECHO_AGENT_SDK_HOST }, async () => {
  const { mkdtemp, access, writeFile, rm } = await import("node:fs/promises");
  const { tmpdir } = await import("node:os");
  const { join } = await import("node:path");
  const { spawn } = await import("node:child_process");
  const work = await mkdtemp(join(tmpdir(), "sdk-ts-parallel-"));
  const fixture = fileURLToPath(new URL("../../shared/testing/parallel_session_fixture.py", import.meta.url));
  const model = spawn(process.env.PYTHON ?? "python3", [fixture, "--work", work]);
  const exists = async path => { try { await access(path); return true; } catch { return false; } };
  const waitFor = async predicate => { const end = Date.now() + 10000; while (!(await predicate())) { assert.ok(Date.now() < end); await new Promise(resolve => setTimeout(resolve, 10)); } };
  let sdk;
  try {
    await waitFor(() => exists(join(work, "ready")));
    sdk = await EchoAgentClient.spawn({ hostCommand: process.env.ECHO_AGENT_SDK_HOST, args: ["--config", join(work, "host.json")] });
    const agentA = await sdk.createAgent(); const agentB = await sdk.createAgent();
    const sessionA = await agentA.createSession(); const sessionB = await agentB.createSession();
    const runA = await sessionA.startRun({ text: "lane-a 中文🌊" }); const runB = await sessionB.startRun({ text: "lane-b 中文🌊" });
    assert.notEqual(sessionA.wire.id, sessionB.wire.id); assert.notEqual(runA.stream.id, runB.stream.id);
    const collect = async run => { const events=[]; for await (const item of run.events) { assert.ok(!("gap" in item)); assert.deepEqual(item.stream, run.stream); assert.equal(item.envelope.stream_id, run.stream.id); events.push(item); } return events; };
    const bounded = async promise => { let timer; try { return await Promise.race([promise, new Promise((_, reject) => { timer=setTimeout(() => reject(new Error("feed did not naturally EOF")),5000); })]); } finally { clearTimeout(timer); } };
    const verifyJournal = async (run, events) => { const page=await run.replay("0",512); assert.deepEqual(events.map(item=>item.envelope.sequence),page.events.map(item=>item.envelope.sequence)); assert.deepEqual(events.map(item=>item.envelope.event_id),page.events.map(item=>item.envelope.event_id)); assert.deepEqual(events.map(item=>item.envelope.sequence),Array.from({length:events.length},(_,i)=>String(i+1))); assert.equal(events.filter(item=>["final_answer","cancelled","error"].includes(item.envelope.payload.event_type)).length,1); };
    const eventsA = collect(runA); let endedB=false; const eventsB = collect(runB).then(values=>{endedB=true;return values;});
    await waitFor(() => exists(join(work, "parked-a")) && exists(join(work, "parked-b")));
    await runA.cancel(); assert.equal((await runA.wait()).terminal.status, "cancelled");
    const observedA = await bounded(eventsA);
    assert.equal(observedA.at(-1).envelope.payload.event_type, "cancelled");
    await verifyJournal(runA,observedA);
    await sessionA.close(); await agentA.close();
    assert.equal(endedB,false,"closing A cannot end B feed");
    assert.equal((await runB.wait({ timeoutMs: 0 })).settled, false);
    await writeFile(join(work, "release-b"), "");
    assert.equal((await runB.wait()).terminal.status, "completed");
    const observedB = await bounded(eventsB);
    assert.equal(observedB.at(-1).envelope.payload.event_type, "final_answer");
    const idsA = new Set(observedA.map(item => item.envelope.event_id));
    assert.ok(observedB.every(item => !idsA.has(item.envelope.event_id)));
    await verifyJournal(runB,observedB);
    await sessionB.close(); await agentB.close();
  } finally { await sdk?.close(); model.kill(); await new Promise(resolve => model.exitCode === null ? model.once("exit", resolve) : resolve()); await rm(work, { recursive: true, force: true }); }
});
