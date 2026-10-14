import test from "node:test";
import assert from "node:assert/strict";
import {mkdtempSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {EchoAgentClient, EchoAgentError} from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw new Error("required Host E2E binary is missing");
const proxy=fileURLToPath(new URL("../../python/tests/protocol_host_proxy.py",import.meta.url));
async function scenario(mode, run) {
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-protocol-"));
 let sdk;let passed=false;
 try {
  sdk=await EchoAgentClient.spawn({hostCommand:"python3",args:[proxy,"--host",host,"--work",work,"--mode",mode]});
  await run(sdk);passed=true;
 } catch(error){process.stderr.write(`Protocol Host failure evidence: ${work}\n`);throw error;
 } finally {await sdk?.close();if(passed)rmSync(work,{recursive:true,force:true});}
}
for(const mode of ["bad-version","bad-digest","bad-acp-version","overflow-acp-version","overflow-extension-version","nontext-feature","null-capability","empty-capabilities","missing-limits","duplicate-feature", "unsorted-features", "duplicate-capability", "zero-limit", "overflow-limit", "numeric-limit", "invalid-timeout"]) test(`Host ${mode} negotiation fails`,{skip:!host},async()=>{
 await assert.rejects(scenario(mode,async()=>assert.fail("must reject")),error=>error.code==="extension_capability_mismatch"&&error.operation==="initialize"&&error.retryable==="never");
});
test("real Host errors remain typed with opaque handles and independent Sessions",{skip:!host},async()=>scenario("normal",async sdk=>{
 const agent=await sdk.createAgent();
 const one=await agent.createSession(),two=await agent.createSession();
 assert.notEqual(one.wire.id,two.wire.id);
 const stale={...one.wire,generation:"0"};
 await assert.rejects(sdk.request("_echo_agent/session/close",{session:stale}),error=>{
  assert.ok(error instanceof EchoAgentError);assert.equal(error.code,"stale_handle");assert.deepEqual(error.handle,stale);return true;
 });
 const special = JSON.parse('{"__proto__":{"中文":"🙂"},"constructor":"safe"}');
 await two.invoke("memory.store.put", [["protocol"], "特殊键", special]);
 const stored = await two.invoke("memory.store.get", [["protocol"], "特殊键"]);
 assert.deepEqual(stored.value, special);
 await one.close();
 assert.equal(await two.invoke("echo_core::agent::Agent::name"),"echo-agent");
 await two.close();await agent.close();
}));
test("real Host facade missing envelope is an explicit failure",{skip:!host},async()=>scenario("no-value",async sdk=>{
 const agent=await sdk.createAgent();const session=await agent.createSession();
 await assert.rejects(session.invoke("echo_core::agent::Agent::name"),error=>error.code==="serialization_violation");
 await session.close();await agent.close();
}));
test("Session close retries after an ACP failure",{skip:!host},async()=>scenario("close-retry",async sdk=>{
 const agent=await sdk.createAgent();const session=await agent.createSession();
 await assert.rejects(session.close(),error=>error.code==="extension_timeout");
 await session.close();await agent.close();
}));
for(const mode of ["unregister-retry","run-close-retry"]) test(`real Host ${mode} retains usable lifecycle`,{skip:!host},async()=>scenario(mode,async sdk=>{
 let calls=0;
 const registration=await sdk.registerLlmClient("protocol-model",{kind:"llm_client",descriptor_version:1,model_name:"protocol-model",supports_streaming:false},async (_call, signal)=>{
  if (mode === "run-close-retry") {
    await new Promise((resolve) => { if (signal.aborted) resolve(); else signal.addEventListener("abort", resolve, {once:true}); });
  }
  ++calls;return {operation:"llm_chat",value:{message:{role:"assistant",content:{kind:"string",value:"中文🙂"}},finish_reason:"stop",raw:{kind:"map",value:[]}}};
 });
 if(mode==="unregister-retry") await assert.rejects(registration.close(),error=>error.code==="extension_timeout");
 const agent=await sdk.createAgent();const session=await agent.createSession();
 const run=await session.startRun({text:"中文🙂"});
 if(mode==="run-close-retry") {
  await assert.rejects(run.close(),error=>error.code==="timeout");
  await run.close();
 } else {assert.equal((await run.wait({timeoutMs:5000})).settled,true);assert.ok(calls>0);await run.close();}
 await registration.close();await session.close();await agent.close();
}));

test("Agent close retries after an ACP failure",{skip:!host},async()=>scenario("agent-close-retry",async sdk=>{
 const agent=await sdk.createAgent();
 await assert.rejects(agent.close(),error=>error.code==="extension_timeout");
 await agent.close();
}));

test("async callback exception settles through the real Host",{skip:!host},async()=>scenario("normal",async sdk=>{
 const registration = await sdk.registerExtension("llm_client", "failing-model", {kind:"llm_client",descriptor_version:1,model_name:"failing-model",supports_streaming:false}, async()=>{throw new Error("callback-中文🙂");});
 const agent = await sdk.createAgent(); const session = await agent.createSession();
 const run = await session.startRun({text:"trigger"});
 const settled = await run.wait({timeoutMs:5000});
 assert.equal(settled.settled,true); assert.equal(settled.terminal.status,"failed"); assert.equal(settled.receipt.outcome,"failed");
 await run.close();await session.close();await registration.close();await agent.close();
}));

test("cancelled tool waits for connection-scoped interrupted callback side effect",{skip:!host},async()=>scenario("normal",async sdk=>{
 let toolStarted;const parked=new Promise(resolve=>{toolStarted=resolve;});
 let canonical;let interrupted;let effect=false;let count=0;
 let releaseTool;const toolReleased=new Promise(resolve=>{releaseTool=resolve;});
 const callback=await sdk.registerAgentCallback("terminal-observer",{kind:"agent_callback",descriptor_version:1},async call=>{
  if(call.invocation.operation==="callback_on_tool_start")canonical=call.invocation.input.call_id;
  if(call.invocation.operation==="callback_on_tool_interrupted"){
   interrupted=call.invocation.input;
   await new Promise(resolve=>setTimeout(resolve,250));effect=true;
  }
  return {operation:call.invocation.operation,value:null};
 });
 const tool=await sdk.registerExtension("tool","hung-tool",{kind:"tool",descriptor_version:1,name:"hung_fixture",description:"park",parameters:{kind:"map",value:[]},schema_revision:"1",required_input_modalities:[],required_permissions:[],risk_level:"standard",supports_streaming:false,exempt_from_batch_timeout:false,allows_parallel_batch_execution:true,manages_own_timeout:false},async(call,signal)=>{
  if(call.invocation.operation==="tool_validate_parameters")return {outcome:"result",result:{operation:"tool_validate_parameters",value:null}};
  toolStarted();await toolReleased;
  return {outcome:"error",error:{code:"cancelled",message:"cancelled",retryable:"never"}};
 });
 const model=await sdk.registerLlmClient("tool-model",{kind:"llm_client",descriptor_version:1,model_name:"tool-model",supports_streaming:false},async()=>{
  const message=count++===0?{role:"assistant",content:{kind:"string",value:""},tool_calls:[{id:"model-call",call_type:"function",function_name:"hung_fixture",arguments:'{"key":"中文🙂"}'}]}:{role:"assistant",content:{kind:"string",value:"done"}};
  return{operation:"llm_chat",value:{message,finish_reason:count===1?"tool_calls":"stop",raw:{kind:"map",value:[]}}};
 });
 const agent=await sdk.createAgent();const session=await agent.createSession();
 await session.invoke("echo_agent::agent::react::ReactAgent::set_permission_mode",["full-auto"]);
 const run=await session.startRun({text:"cancel tool"});
 await Promise.race([parked,new Promise((_resolve,reject)=>setTimeout(()=>reject(new Error("tool never entered")),5000))]);
 await run.cancel();const settled=await run.wait({timeoutMs:5000});
 releaseTool();
 assert.equal(settled.terminal.status,"cancelled");assert.equal(effect,true);
 assert.ok(canonical);assert.equal(interrupted.call_id,canonical);assert.equal(interrupted.tool,"hung_fixture");
 assert.deepEqual(interrupted.input,{kind:"map",value:[{key:{kind:"string",value:"key"},value:{kind:"string",value:"中文🙂"}}]});
 await run.close();await session.close();await tool.close();await model.close();await callback.close();await agent.close();
}));

test("Host Rust Unicode feature order is accepted", {skip:!host}, async()=>scenario("unicode-features",async sdk=>{
 assert.deepEqual(sdk.capability.features,["\ue000","\u{10000}"]);
}));
