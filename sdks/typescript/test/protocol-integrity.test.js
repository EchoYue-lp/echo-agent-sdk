import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fromWireValue, toWireValue } from "../dist/wire.js";
import { EchoAgentError } from "../dist/errors.js";
import { EchoAgentClient, ExtensionRegistration, SessionHandle } from "../dist/client.js";
const corpus = JSON.parse(readFileSync(new URL("../../shared/fixtures/native-projection-invalid.json", import.meta.url)));
for (const {name, wire} of corpus) test(`native projection rejects ${name}`, () => {
  assert.throws(() => fromWireValue(wire), error => error.code === "serialization_violation");
});
test("special native keys survive without prototype mutation", () => {
  const input = JSON.parse('{"__proto__":{"中文":"🙂"},"constructor":"safe"}');
  const result = fromWireValue(toWireValue(input));
  assert.deepEqual(result, input);
  assert.equal(Object.getPrototypeOf(result), Object.prototype);
});
test("SDK errors retain the opaque Host handle", () => {
  const handle = {id:"run-中文",generation:"18446744073709551615",kind:"run"};
  assert.deepEqual(EchoAgentError.fromUnknown({data:{code:"stale_handle",message:"stale",handle}}).handle, handle);
});
test("facade responses require their value envelope", async () => {
  const client = Object.create(EchoAgentClient.prototype);
  client.catalog = {signature:()=>"digest",familySignature:()=>"digest",resolve:()=>({family:"memory",method:"_echo_agent/memory/op"})};
  let calls = 0;
  client.request = async () => { calls++; return {}; };
  await assert.rejects(client.invoke("operation"), error => error.code === "serialization_violation");
  await assert.rejects(client.family("memory","operation"), error => error.code === "serialization_violation");
  assert.equal(calls, 2);
});
test("extension close remains retryable after unregister failure", async () => {
  let calls=0;
  const registration = new ExtensionRegistration({unregisterExtension:async()=>{if (++calls === 1) throw new Error("retry");}}, {id:"e",generation:"1",kind:"extension"});
  await assert.rejects(registration.close(), /retry/);
  await registration.close();
  assert.equal(calls,2);
});
test("execute uses the accepted task input field", async () => {
  let input;
  const client = {request:async (_method,params)=>{input=params.input;return {run:{id:"r",generation:"1",kind:"run"},stream:{id:"s",generation:"1",kind:"stream"}};}};
  const session = new SessionHandle(client,{id:"a",generation:"1",kind:"agent"},{id:"ss",generation:"1",kind:"session"},"acp");
  await session.startRun({mode:"execute",text:"任务🙂"});
  assert.deepEqual(input,{kind:"execute",task:"任务🙂"});
});

import { decodeReplayResponse } from "../dist/client.js";
test("canonical retention gap remains plain EventGap on replay wire", () => {
 const fixture=JSON.parse(readFileSync(new URL("../../../contracts/sdk/fixtures/extension/v1/event-replay-gap-valid.json",import.meta.url)));
 const stream={id:"stream-1",generation:"1",kind:"stream"};
 const result=decodeReplayResponse(fixture.payload,stream,"4");
 assert.equal(result.gap.gap.snapshot_watermark,"9");
 assert.deepEqual(result.next_cursor,fixture.payload.next_cursor);
 assert.throws(()=>decodeReplayResponse({...fixture.payload,next_cursor:{stream_id:"wrong",last_processed_sequence:"9"}},stream),e=>e.code==="serialization_violation");
 assert.throws(()=>decodeReplayResponse({...fixture.payload,next_cursor:{stream_id:stream.id,last_processed_sequence:"09"}},stream),e=>e.code==="serialization_violation");
});

test("managed callback requests and receipts cross the accepted typed adapter", async()=>{
 const {decodeExtensionInvokeCall}=await import("../dist/client.js");
 const input={scope_id:"scope",context:{absolute_deadline_unix_ms:"9223372036854775807"}};
 const payload={extension:{id:"ext",generation:"1",kind:"extension"},invocation_id:"invoke",deadline:{seconds:"1",nanos:0},invocation:{operation:"agent_component_call",input:{component:"runtime_state_store",call:{operation:"runtime_load_scope_authority_with_context",input}}}};
 const parsed=decodeExtensionInvokeCall(payload,"agent_component");
 assert.deepEqual(parsed.invocation.input.call.input,input);
 const client=Object.create(EchoAgentClient.prototype);
 let handler;
 client.registerExtension=async (_kind,_id,_descriptor,value)=>{handler=value;return {};};
 await client.registerAgentComponent("managed",{kind:"agent_component",descriptor_version:1,name:"managed",component:"runtime_state_store",capabilities:{runtime_state:"revisioned_v1",persistence_call:"absolute_deadline_v1"}},async()=>({operation:"agent_component_call",value:{component:"runtime_state_store",result:{operation:"runtime_load_scope_authority_with_context",value:{authority:{scope_id:"scope",conversation_epoch:null,revision:"9007199254740993",lifecycle:"active"}}}}}));
 const result=await handler(payload,new AbortController().signal);
 assert.equal(result.result.value.result.value.authority.revision,"9007199254740993");
 assert.throws(()=>decodeExtensionInvokeCall({...payload,invocation:{...payload.invocation,input:{...payload.invocation.input,call:{...payload.invocation.input.call,input:{...input,context:{absolute_deadline_unix_ms:"01"}}}}}},"agent_component"));
});

test("tool effect facts and canonical callback IDs survive native APIs", async()=>{
 const {ToolResultValue}=await import("../dist/helpers.js");
 const {decodeAgentCallback}=await import("../dist/client.js");
 const effects=[{effect:"file_read",path:"/中文🙂"},{effect:"file_edit",path:"/x"},{effect:"test_run",command:"测试",passed:false,failure_count:"18446744073709551615"},{effect:"subagent_run",agent_name:"a",task:"任务",outcome:"cancelled"}];
 assert.deepEqual(ToolResultValue.success("ok").withEffects(effects).toJSON().effects,effects);
 assert.throws(()=>ToolResultValue.success("ok").withEffects([{effect:"test_run",command:"test",passed:false,failure_count:"01"}]));
 const input={agent:"a",call_id:"canonical-call",tool:"t",input:{kind:"null"},error:"cancelled"};
 const raw={extension:{id:"e",generation:"1",kind:"extension"},invocation_id:"transport-invoke",invocation:{operation:"callback_on_tool_interrupted",input}};
 assert.deepEqual(decodeAgentCallback(raw).invocation.input,input);
 assert.throws(()=>decodeAgentCallback({...raw,invocation:{...raw.invocation,input:{...input,call_id:null}}}));
});

test("all eleven managed context operations preserve the same shared typed corpus",async()=>{
 const {decodeExtensionInvokeCall}=await import("../dist/client.js");
 const corpus=JSON.parse(readFileSync(new URL("../../shared/fixtures/managed-context-corpus.json",import.meta.url)));
 assert.equal(corpus.length,11);
 for(const item of corpus){
  const payload={extension:{id:"e",generation:"1",kind:"extension"},invocation_id:item.scenario_id,deadline:{seconds:"1",nanos:0},invocation:{operation:"agent_component_call",input:{component:item.component,call:{operation:item.operation,input:item.input}}}};
  assert.deepEqual(decodeExtensionInvokeCall(payload,"agent_component").invocation.input.call.input,item.input);
  const client=Object.create(EchoAgentClient.prototype);let adapter;
  client.registerExtension=async(_kind,_id,_descriptor,callback)=>{adapter=callback;return {};};
  await client.registerAgentComponent(item.scenario_id,{kind:"agent_component",descriptor_version:1,name:"corpus",component:item.component},async()=>({operation:"agent_component_call",value:{component:item.component,result:{operation:item.operation,value:item.result}}}));
  const result=await adapter(payload,new AbortController().signal);
  assert.deepEqual(result.result.value.result.value,item.result,item.scenario_id);
 }
});

test("managed checkpoints reject unknown path encoding and non-absolute native paths",async()=>{
 const {validateManaged}=await import("../dist/managed.js");
 assert.throws(()=>validateManaged("WirePath",{encoding:"unix_bytes",base64:"L3RtcC//"}));
 assert.throws(()=>validateManaged("WirePath",{encoding:"unix",bytes_base64:"cmVsYXRpdmU"}));
 assert.throws(()=>validateManaged("WirePath",{encoding:"utf8",path:"relative"}));
 validateManaged("WirePath",{encoding:"unix",bytes_base64:"L3RtcC//"});
});

test("managed error outcome preserves unknown commit facts through the typed adapter",async()=>{
 const client=Object.create(EchoAgentClient.prototype);let adapter;
 client.registerExtension=async(_kind,_id,_descriptor,callback)=>{adapter=callback;return {};};
 const outcome={outcome:"error",error:{code:"framework_error",message:"commit acknowledgement lost 中文🙂",retryable:"after_delay",operation:"conversation_apply_transcript_projection_with_context",handle:{id:"store",generation:"18446744073709551615",kind:"extension"},details:{fields:[{key:"class",value:"outcome_unknown"},{key:"operation_id",value:"commit-one"},{key:"payload_digest",value:"sha256:"+"a".repeat(64)}]}}};
 await client.registerAgentComponent("unknown",{kind:"agent_component",descriptor_version:1,name:"unknown",component:"runtime_state_store",capabilities:{runtime_state:"revisioned_v1",persistence_call:"absolute_deadline_v1"}},async()=>outcome);
 const payload={extension:{id:"e",generation:"1",kind:"extension"},invocation_id:"i",deadline:{seconds:"1",nanos:0},invocation:{operation:"agent_component_call",input:{component:"runtime_state_store",call:{operation:"runtime_load_scope_authority_with_context",input:{scope_id:"scope",context:{absolute_deadline_unix_ms:"9223372036854775807"}}}}}};
 assert.deepEqual(await adapter(payload,new AbortController().signal),outcome);
 for(const error of [{...outcome.error,retryable:undefined},{...outcome.error,handle:{...outcome.error.handle,generation:"01"}},{...outcome.error,details:{fields:[{key:"class",value:7}]}}]){
  await client.registerAgentComponent("unknown",{kind:"agent_component",descriptor_version:1,name:"unknown",component:"runtime_state_store"},async()=>({outcome:"error",error}));
  await assert.rejects(adapter(payload,new AbortController().signal),candidate=>candidate.code==="serialization_violation");
 }
});

for(const capacity of [128,256])test(`live window ${capacity} reserves one coalesced gap control`,async()=>{
 const client=Object.create(EchoAgentClient.prototype);client.events=new Map();client.queueCapacity=capacity;
 client.eventsFor("stream",{id:"stream",generation:"1",kind:"stream"});
 const queue=client.events.get("stream").queue;
 for(let index=1;index<=capacity;index++)queue.push({envelope:{sequence:String(index)}});
 queue.push({gap:{snapshot_watermark:String(capacity+1)}},true);
 const latest={gap:{snapshot_watermark:String(capacity+2)}};queue.push(latest,true);queue.end();
 const collected=[];for await(const item of queue)collected.push(item);
 assert.equal(collected.length,capacity+1);assert.deepEqual(collected.at(-1),latest);
});

test("native domain IDs remain maps while explicit handle wrappers stay strict",()=>{
 for(const value of [{id:"case-one",name:"case"},{generation:"domain-revision"},{id:"domain",kind:"session",generation:"1",title:"ordinary-domain"},{id:"partial-domain",generation:"1"},{id:"case",kind:"evaluation",generation:"1"}]){
  assert.equal(toWireValue(value).kind,"map");assert.deepEqual(fromWireValue(toWireValue(value)),value);
 }
 assert.throws(()=>toWireValue({kind:"handle",value:{id:"partial-handle",kind:"session"}}));
 assert.throws(()=>toWireValue({kind:"handle",value:{id:"handle",kind:"session",generation:"1",title:"discarded"}}));
});

test("coalesced gap keeps original range and replaces only snapshot-covered tail",async()=>{
 const client=Object.create(EchoAgentClient.prototype);client.events=new Map();client.queueCapacity=8;
 client.eventsFor("stream",{id:"stream",generation:"1",kind:"stream"});
 const queue=client.events.get("stream").queue;
 const event=sequence=>({envelope:{sequence:String(sequence)}});
 const gap=sequence=>({gap:{from_sequence:String(sequence),to_sequence:String(sequence),snapshot_watermark:String(sequence),reason:"bounded"}});
 queue.push(event(1));queue.push(gap(2),true);queue.push(event(3));queue.push(event(5));queue.push(gap(4),true);queue.end();
 const values=[];for await(const value of queue)values.push(value);
 assert.deepEqual(values,[event(1),{gap:{from_sequence:"2",to_sequence:"4",snapshot_watermark:"4",reason:"bounded"}},event(5)]);
});

test("explicit observed snapshot ACK filters covered queue values without accepting a future ACK",async()=>{
 const client=Object.create(EchoAgentClient.prototype);client.events=new Map();client.queueCapacity=8;client.closed=false;client.notify=async()=>{};
 const stream={id:"s",generation:"1",kind:"stream"};const iterator=client.eventsFor(stream.id,stream)[Symbol.asyncIterator]();const feed=client.events.get(stream.id);
 const event=sequence=>({envelope:{sequence:String(sequence)}});const gap=sequence=>({stream,gap:{from_sequence:String(sequence),to_sequence:String(sequence),snapshot_watermark:String(sequence),reason:"bounded"}});
 feed.lastEventSequence=2n;feed.lastLiveSequence=1n;feed.lastGapWatermark=2n;feed.queue.push(event(1));feed.queue.push(gap(2),true);
 await client.acknowledge(stream,"100");assert.equal(feed.acknowledgedSequence,undefined);
 await client.acknowledge(stream,"2");feed.queue.push(event(3));feed.queue.push(gap(4),true);feed.lastEventSequence=4n;feed.lastLiveSequence=3n;feed.lastGapWatermark=4n;feed.queue.end();
 const values=[];for(;;){const next=await iterator.next();if(next.done)break;values.push(next.value);}
 assert.deepEqual(values,[{stream,gap:{from_sequence:"3",to_sequence:"4",snapshot_watermark:"4",reason:"bounded"}}]);
});

test("typed settlement event preserves raw u64 contract while checking its u32 attempt",async()=>{
 const {decodeSettlementEvent}=await import("../dist/managed.js");
 const payload={status:"deferred",operation_id:"commit",conversation_id:"session",generation_id:"generation",attempt:4294967295,error_class:"outcome_unknown",detail:"lost 中文🙂"};
 const wire=toWireValue(payload);assert.equal(fromWireValue(wire).attempt,"4294967295");
 assert.deepEqual(decodeSettlementEvent(wire),payload);
 const wrong=structuredClone(wire);wrong.value.find(entry=>entry.key.value==="attempt").value.value="4294967296";assert.throws(()=>decodeSettlementEvent(wrong));
 const duplicate=structuredClone(wire);duplicate.value.push(duplicate.value[0]);assert.throws(()=>decodeSettlementEvent(duplicate));
 const missing=structuredClone(wire);missing.value.pop();assert.throws(()=>decodeSettlementEvent(missing));
});
