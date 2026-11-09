import test from "node:test";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, EchoAgentError, fromWireValue, toWireValue } from "../dist/index.js";
import { fileSha256 } from "../../shared/testing/file-sha256.mjs";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw new Error("real SearchProvider Host required");
const fixture = fileURLToPath(new URL("../../shared/testing/tool_value_effect_fixture.py", import.meta.url));

// 同一原生工具调用由框架中央机制重试；模型只发出一次 web_search。
for (const [name, code, retryable, attempts] of [
  ["permanent", "extension_failed", "never", 1],
  ["timeout-never", "extension_timeout", "never", 1],
  ["timeout", "extension_timeout", "after_delay", 3],
  ["transient", "extension_failed", "after_delay", 3],
  ["always", "extension_failed", "always", 3],
]) {
  test(`SearchProvider ${name} keeps native failure and central retry policy`, { skip: !host, timeout: 45000 }, async () => {
    const root = process.env.ECHO_AGENT_SDK_TOOL_EFFECT_ROOT ?? tmpdir();
    mkdirSync(root, { recursive: true });
    const work = mkdtempSync(join(root, `typescript-retry-${name}-`));
    const model = spawn(process.env.PYTHON ?? "python3", ["-B", fixture, "--work", work], { stdio: ["ignore", "ignore", "inherit"] });
    const calls = [];
    let sdk, observed;
    try {
      const deadline = Date.now() + 10000;
      while (!existsSync(join(work, "ready"))) {
        assert.equal(model.exitCode, null);
        assert.ok(Date.now() < deadline, "original model fixture readiness");
        await new Promise(resolve => setTimeout(resolve, 10));
      }
      sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", join(work, "host.json")] });
      const registration = await sdk.registerAgentComponent(`search-retry-${name}`, {
        kind: "agent_component", descriptor_version: 1, component: "search_provider", name: `controlled-${name}`,
      }, async (call, signal) => {
        calls.push({ invocation_id: call.invocation_id, call: call.invocation.input.call, cancelled: signal.aborted });
        // 两次可观察故障后才恢复，验证原工具机制确实跨越了失败窗口。
        if (retryable === "never" || calls.length < 3) throw new EchoAgentError(code, `provider ${name} 中文🙂`, retryable);
        const result = { title: "重试恢复中文🙂", url: "https://example.invalid/recovered", snippet: "真实中央重试" };
        return { operation: "agent_component_call", value: { component: "search_provider", result: {
          operation: "search_provider_search", value: { results: [toWireValue(result)] },
        } } };
      });
      const agent = await sdk.createAgent();
      const session = await agent.createSession(work);
      await session.invoke("echo_agent::agent::react::ReactAgent::set_permission_mode", ["full-auto"]);
      const run = await session.startRun({ text: "search-error" });
      const settled = await run.wait({ timeoutMs: 15000 });
      const replay = await run.replay("0", 512);
      observed = { host, host_sha256: fileSha256(host), name, code, retryable, calls, settled, replay };
      await run.close();
      await session.close();
      await agent.close();
      await registration.close();
      writeFileSync(join(work, "retry-results.json"), JSON.stringify(observed, (_, value) => typeof value === "bigint" ? value.toString() : value, 2));
      const frames = readFileSync(join(work, "model-http.jsonl"), "utf8").trim().split("\n").map(line => JSON.parse(line));
      assert.equal(frames.length, 2, "one model tool request and one result-consumption request");
      const modelCalls = frames.flatMap(frame => {
        const raw = Buffer.from(frame.response_base64, "base64").toString("utf8");
        const responses = frame.content_type === "text/event-stream"
          ? raw.split("\n").filter(line => line.startsWith("data: ") && line !== "data: [DONE]").map(line => JSON.parse(line.slice(6)))
          : [JSON.parse(raw)];
        return responses.flatMap(response => response.choices.flatMap(choice => (choice.delta ?? choice.message).tool_calls ?? []));
      });
      assert.equal(modelCalls.length, 1, "callback attempts must come from central retry, not model reissuance");
      assert.equal(modelCalls[0].function.name, "web_search");
      assert.equal(modelCalls[0].id, "search-proof");
      const consumption = JSON.parse(Buffer.from(frames[1].request_base64, "base64").toString("utf8"));
      assert.equal(consumption.messages.at(-1).tool_call_id, modelCalls[0].id);
      assert.equal(calls.length, attempts, "one original tool call must obey the explicit failure policy");
      assert.equal(new Set(calls.map(call => call.invocation_id)).size, attempts);
      assert.ok(calls.every(call => call.call.input.query === "failure 中文🙂" && !call.cancelled));
      assert.equal(settled.terminal.status, "completed");
      const toolEvents = replay.events.filter(event => event.envelope.payload.event_type === "tool_result")
        .map(event => fromWireValue(event.envelope.payload.data));
      assert.equal(toolEvents.length, 1);
      assert.equal(toolEvents[0].call_id, modelCalls[0].id);
      const results = toolEvents.map(event => event.result);
      if (retryable === "never") {
        assert.equal(results[0].success, false);
        assert.deepEqual(results[0].failure, { category: "permanent", recovery: "stop", side_effect: "none" });
      } else {
        assert.equal(results[0].success, true);
        assert.ok(results[0].output.includes("重试恢复中文🙂"));
      }
    } finally {
      console.log("SearchProvider retry evidence:", work);
      try { await sdk?.close(); }
      finally {
        if (model.exitCode === null && model.signalCode === null) model.kill();
        await new Promise(resolve => model.exitCode !== null || model.signalCode !== null ? resolve() : model.once("exit", resolve));
      }
    }
  });
}
