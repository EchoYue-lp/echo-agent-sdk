import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import test from "node:test";
import { EchoAgentClient, fromWireValue } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) {
  throw new Error("required native Host missing");
}

test("native messages preserve provider history and the runtime terminal", {
  skip: !host,
  timeout: 30000,
}, async () => {
  const root = fileURLToPath(new URL("../../..", import.meta.url));
  const work = await mkdtemp(join(tmpdir(), "sdk-native-messages-typescript-"));
  const spec = JSON.parse(await readFile(join(root, "sdks/shared/testing/native_message_host_contract.json"), "utf8"));
  const config = JSON.parse(await readFile(join(root, "echo-sdk-host/config.sdk.example.json"), "utf8"));
  config.sdk_profile.state_root = join(work, "state");
  config.default_agent.agent.enable_tools = true;
  config.default_agent.agent.memory_path = join(work, "memory.jsonl");
  const configPath = join(work, "host.json");
  await writeFile(configPath, JSON.stringify(config));
  const requests = [];
  const live = [];
  let sdk;
  let timer;
  let collecting;
  try {
    sdk = await EchoAgentClient.spawn({ hostCommand: host, args: ["--config", configPath] });
    const registration = await sdk.registerLlmClient("native-typescript", {
      kind: "llm_client",
      descriptor_version: 1,
      model_name: "controlled-native",
      supports_streaming: false,
    }, async (call) => {
      requests.push(structuredClone(call));
      assert.ok(requests.length <= spec.responses.length, "unexpected extra model request");
      return { operation: "llm_chat", value: structuredClone(spec.responses[requests.length - 1]) };
    });
    const agent = await sdk.createAgent();
    const session = await agent.createSession(work);
    const run = await session.startRun({ mode: "chat", text: "native source probe" });
    collecting = (async () => {
      for await (const item of run.events) {
        assert.ok("envelope" in item, "unexpected retention gap");
        live.push(item);
      }
    })();
    // wait() 先失败时，finally 仍须回收并读取收集任务的结果。
    collecting.catch(() => {});
    const receipt = await run.wait({ timeoutMs: 10000 });
    await Promise.race([collecting, new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error("native stream did not finish")), 10000);
    })]);
    clearTimeout(timer);
    const replay = await run.replay("0", 512);
    assert.equal(receipt.settled, true);
    assert.equal(receipt.terminal.status, "completed");
    assert.equal(requests.length, spec.expected_model_calls);
    assert.deepEqual(live.map((item) => item.envelope), replay.events.map((item) => item.envelope));
    const payloads = live.map((item) => item.envelope.payload);
    const usages = payloads.filter((item) => item.event_type === "llm_usage").map((item) => fromWireValue(item.data));
    assert.equal(usages.length, spec.expected_model_calls);
    for (const usage of usages) {
      const observation = usage.observation;
      assert.ok(observation, "actual Some observation missing from live/replay consumer");
      assert.deepEqual(Object.keys(observation).sort(), ["call_id", "attempt_id", "outcome", "request_duration_ms", "first_output_ms", "last_output_ms", "output_samples", "timing_comparable"].sort());
      assert.ok(observation.call_id.length > 0 && observation.attempt_id.length > 0);
      assert.equal(observation.outcome, "completed");
      assert.ok(BigInt(observation.request_duration_ms) >= 0n);
      assert.ok(BigInt(observation.output_samples) >= 0n);
      assert.equal(typeof observation.timing_comparable, "boolean");
      // 原完整 live==replay 断言同时核对这些真实时刻，不发明固定耗时或正速率。
    }
    const statuses = payloads.filter((item) => item.event_type === "execution_status").map((item) => fromWireValue(item.data));
    assert.ok(statuses.every((status) => !Object.hasOwn(status, "adapter_generated")));
    for (const request of requests) {
      for (const message of request.invocation.input.messages) {
        if (message.execution_status != null) assert.equal(Object.hasOwn(message.execution_status, "adapter_generated"), false);
      }
    }
    const tail = payloads.at(-1);
    assert.equal(tail.event_type, "execution_status");
    const terminal = fromWireValue(tail.data);
    assert.equal(terminal.phase, "Terminal");
    assert.equal(terminal.status, "settled");
    assert.equal(statuses.filter((status) => status.phase === "Terminal").length, 1);
    const blocks = payloads.filter((item) => item.event_type === "assistant_message").map((item) => fromWireValue(item.data).block);
    assert.deepEqual(blocks.map((block) => block.phase), ["commentary", "final_answer"]);
    assert.equal(blocks.at(-1).delta, spec.final_answer);
    const finals = payloads.filter((item) => item.event_type === "final_answer");
    assert.equal(finals.length, 1);
    assert.equal(fromWireValue(finals[0].data), spec.final_answer);
    const finalBlockIndex = payloads.findLastIndex((item) => item.event_type === "assistant_message");
    const compatibilityIndex = payloads.findIndex((item) => item.event_type === "final_answer");
    assert.ok(finalBlockIndex < compatibilityIndex && compatibilityIndex < payloads.length - 1);
    const expectedSource = spec.responses[0].message.assistant_blocks[0].provider_source;
    const historySources = requests[1].invocation.input.messages.flatMap((message) => message.assistant_blocks ?? []).map((block) => block.provider_source);
    assert.ok(historySources.some((source) => source?.item_id === expectedSource.item_id));
    assert.deepEqual(historySources.find((source) => source?.item_id === expectedSource.item_id), expectedSource);
    await session.close();
    await agent.close();
    await registration.close();
  } finally {
    clearTimeout(timer);
    await sdk?.close();
    if (collecting) await Promise.allSettled([collecting]);
    await rm(work, { recursive: true, force: true });
  }
});
