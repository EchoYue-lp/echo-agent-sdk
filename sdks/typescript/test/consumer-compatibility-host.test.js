import { mkdir, mkdtemp, readFile, writeFile } from "node:fs/promises";
import { join, resolve, relative, isAbsolute } from "node:path";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import test from "node:test";
import {execFile} from "node:child_process";
import {promisify} from "node:util";
import { EchoAgentClient, ExtensionStreamWriter, fromWireValue } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) {
  throw new Error("required native Host missing");
}

function startLlmProducer(writer, chunk, complete) {
  const producer=(async()=>{
    await writer.chunk({kind:"llm",value:chunk});
    await writer.complete({kind:"llm",value:complete});
  })();
  // 先观察拒绝，原Run可能仍等待；同一原Promise仍交成功/失败收尾并保原错误。
  producer.catch(()=>{});
  return producer;
}

test("LLM send rejection is observed while the original Run wait is pending", async () => {
  const failure=Error("original notification failure");
  let finishWait;
  const originalWait=new Promise(resolve=>{finishWait=resolve;});
  let waitSettled=false;
  originalWait.then(()=>{waitSettled=true;});
  const unhandled=[];
  const observe=error=>unhandled.push(error);
  process.on("unhandledRejection",observe);
  try {
    const writer=new ExtensionStreamWriter({notify:async()=>{throw failure;}},
      {id:"original-stream",generation:"1",kind:"stream"});
    const producer=startLlmProducer(writer,{content:"fixture"},{finish_reason:"stop"});
    await new Promise(resolve=>setImmediate(()=>setImmediate(resolve)));
    assert.equal(waitSettled,false);
    assert.deepEqual(unhandled,[]);
    await assert.rejects(producer,error=>error===failure);
  } finally {finishWait();await originalWait;process.off("unhandledRejection",observe);}
});

for (const protocol of ["responses", "anthropic", "chat_completions"]) for (const streaming of [false, true])
test(`consumer whole loop ${protocol} streaming=${streaming}`, {
  skip: !host,
  timeout: 30000,
}, async () => {
  const root = fileURLToPath(new URL("../../..", import.meta.url));
  const runtimeRoot = process.env.ECHO_AGENT_SDK_CONSUMER_ROOT;
  assert.ok(runtimeRoot && isAbsolute(runtimeRoot) && relative(resolve(root, ".."), resolve(runtimeRoot)) && !relative(resolve(root, ".."), resolve(runtimeRoot)).startsWith(".."), "consumer runtime root must be inside workspace");
  await mkdir(runtimeRoot, {recursive:true});
  const work = await mkdtemp(join(runtimeRoot, "typescript-"));
  const all = JSON.parse(await readFile(join(root, "sdks/shared/testing/consumer_compatibility_host.json"), "utf8"));
  const spec = JSON.parse(JSON.stringify(all[protocol]).replaceAll("__WORK__", work));
  await writeFile(join(work, "fixture.txt"), "actual-tool-a");
  const producers = [];
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
      supports_streaming: streaming,
    }, async (call) => {
      requests.push(structuredClone(call));
      assert.ok(requests.length <= spec.responses.length, "unexpected extra model request");
      const value = structuredClone(spec.responses[requests.length - 1]);
      if (streaming) {
        assert.ok(call.stream);
        const writer = sdk.streamWriter(call.stream);
        const {message, finish_reason} = value;
        const tool_calls = message.tool_calls?.map((tool, index) => ({index, provider_source:tool.provider_source,
          id:tool.id, call_type:tool.call_type, function:{name:tool.function_name, arguments:tool.arguments}}));
        producers.push(startLlmProducer(writer,{role:"assistant",content:fromWireValue(message.content),
          assistant_blocks:message.assistant_blocks,reasoning_blocks:message.reasoning_blocks,tool_calls},{finish_reason}));
        return {outcome:"stream",stream:call.stream};
      }
      return {operation:"llm_chat",value};
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
    await Promise.all(producers);
    const replay = await run.replay("0", 512);
    await writeFile(join(work,"observed.json"),JSON.stringify({requests,live,replay,receipt},null,2));
    console.log("consumer evidence:",work);
    assert.equal(receipt.settled, true);
    assert.equal(receipt.terminal.status, "completed");
    assert.equal(requests.length, spec.expected_model_calls);
    const livePage=join(work,"complete-live.json"),replayPage=join(work,"complete-replay.json");
    await writeFile(livePage,JSON.stringify({events:live.map(item=>item.envelope)}));
    await writeFile(replayPage,JSON.stringify({events:replay.events.map(item=>item.envelope)}));
    // 复用既有完整entry-multiset比较；事件/其它数组、重复map项与字段不丢弃。
    await promisify(execFile)(process.env.PYTHON??"python3",[
      join(root,"sdks/shared/testing/gap_ack_fixture.py"),"--work",work,"--mode","window",
      "--compare-pages",livePage,replayPage],{timeout:10000});
    const payloads = live.map((item) => item.envelope.payload);
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
    assert.deepEqual(blocks.map((block) => block.phase), spec.expected_phases);
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
    const toolResults = requests[1].invocation.input.messages.filter(message => message.role === "tool");
    for (const [id, isError] of [["call-a",false],["call-b",true]]) {
      const result = toolResults.find(message => message.tool_call_id === id);
      assert.ok(result, "original tool result missing");
      assert.equal(result.tool_result_is_error, isError);
    }
    const rawHistory = requests[2].invocation.input.messages;
    assert.ok(rawHistory.some(message => message.reasoning_blocks?.some(block =>
      JSON.stringify(block) === JSON.stringify(spec.reasoning))));
    assert.equal(payloads.filter(item => item.event_type === "tool_result").length, 2);
    await session.close();
    await agent.close();
    await registration.close();
  } finally {
    clearTimeout(timer);
    try { await sdk?.close(); }
    finally { await Promise.allSettled([...producers,...(collecting?[collecting]:[])]); }
  }
});
