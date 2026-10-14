import test from "node:test";
import assert from "node:assert/strict";
import {spawn} from "node:child_process";
import {mkdtempSync,existsSync,readFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {EchoAgentClient,SessionHandle,fromWireValue,decodeSettlementEvent} from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required Managed Host E2E binary missing");
async function http(url,path,payload){const response=await fetch(url+path,{method:payload===undefined?"GET":"POST",headers:{"Content-Type":"application/json"},...(payload===undefined?{}:{body:JSON.stringify(payload)})});assert.equal(response.status,200);return response.json();}
for(const fault of ["lose_first_apply_response","expire_before_work_once"])test(`managed ${fault} preserves backend commit authority through real Host`,{skip:!host},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-managed-"));
 const backend=spawn("python3",[fileURLToPath(new URL("../../shared/testing/managed_persistence_backend.py",import.meta.url)),"--work",work,"--fault",fault]);let sdk;let passed=false;
 try{
  const deadline=Date.now()+5000;while(!existsSync(join(work,"ready"))){assert.ok(Date.now()<deadline,"backend never became ready");await new Promise(resolve=>setTimeout(resolve,10));}
  const {url}=JSON.parse(readFileSync(join(work,"backend.json"),"utf8"));
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")],onStderr:chunk=>process.stderr.write(chunk)});
  const registrations=[];
  for(const component of ["conversation_store","runtime_state_store"]){
   registrations.push(await sdk.registerAgentComponent("managed-"+component,{kind:"agent_component",descriptor_version:1,component,name:"managed-"+component,capabilities:{conversation_projection:component==="conversation_store"?"atomic_v1":"unsupported",runtime_state:component==="runtime_state_store"?"revisioned_v1":"unsupported",persistence_call:"absolute_deadline_v1"}},async call=>{
    const response=await http(url,"/call",call.invocation.input.call);
    if(response.kind==="error")process.stderr.write(`managed actual callback ${call.invocation.input.call.operation}: ${JSON.stringify(response.error)}\n`);
    if(response.kind==="error")return {outcome:"error",error:response.error};
    return{operation:"agent_component_call",value:{component,result:response.result}};
   }));
  }
  const agent=await sdk.createAgent();const session=await agent.createSession();const run=await session.startRun({text:"first managed turn"});
  const settled=await run.wait({timeoutMs:40000});const first=await http(url,"/inspect");
  if(fault==="expire_before_work_once"){
   assert.equal(first.counters.mutations,0);assert.equal(Object.keys(first.receipts).length,0);assert.equal(settled.terminal.status,"failed");
  }else{
   assert.ok(Object.keys(first.receipts).length);assert.ok(first.counters.projected_items>0);
   const replay=await run.replay("0",512);
   assert.ok(replay.events.some(({envelope})=>envelope.payload.event_type==="transcript_projection_settlement"&&decodeSettlementEvent(envelope.payload.data).error_class==="outcome_unknown"));
   assert.ok(Object.values(first.runtime).some(snapshot=>JSON.parse(snapshot.checkpoint.messages_json).pending_transcript_projection));
   const second=await session.startRun({text:"second managed turn"});assert.equal((await second.wait({timeoutMs:40000})).settled,true);
   const final=await http(url,"/inspect");for(const [operation,receipt] of Object.entries(first.receipts))assert.deepEqual(final.receipts[operation],receipt);
   assert.ok(final.counters.duplicate_applies>0);await second.close();
  }
  await run.close();await session.close();for(const registration of registrations)await registration.close();await agent.close();passed=true;
 }catch(error){process.stderr.write(`managed backend failure evidence: ${work}\n`);throw error;
 }finally{await sdk?.close();backend.kill();await new Promise(resolve=>backend.exitCode!==null?resolve():backend.once("exit",resolve));if(passed)rmSync(work,{recursive:true,force:true});}
});

test("managed committed held response survives real Host restart with the same receipt",{skip:!host},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-managed-restart-"));
 const backend=spawn("python3",[fileURLToPath(new URL("../../shared/testing/managed_persistence_backend.py",import.meta.url)),"--work",work,"--fault","hold_first_apply_response"]);let sdk;let passed=false;
 const wait=async predicate=>{const end=Date.now()+35000;while(!(await predicate())){assert.ok(Date.now()<end,"managed restart barrier missing");await new Promise(resolve=>setTimeout(resolve,10));}};
 const start=async url=>{
  const client=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")],onStderr:chunk=>process.stderr.write(chunk)});
  const registrations=[];
  for(const component of ["conversation_store","runtime_state_store"])registrations.push(await client.registerAgentComponent("managed-"+component,{kind:"agent_component",descriptor_version:1,component,name:"managed-"+component,capabilities:{conversation_projection:component==="conversation_store"?"atomic_v1":"unsupported",runtime_state:component==="runtime_state_store"?"revisioned_v1":"unsupported",persistence_call:"absolute_deadline_v1"}},async call=>{
    const response=await http(url,"/call",call.invocation.input.call);
    return response.kind==="error"?{outcome:"error",error:response.error}:{operation:"agent_component_call",value:{component,result:response.result}};
  }));
  return {client,registrations};
 };
 const sessionFor=async(client,agent,load)=>{
  const result=await client.request(load?"_echo_agent/session/load":"_echo_agent/session/create",{agent:agent.wire,session_id:"ts-managed-restart",working_dir:null});
  return new SessionHandle(client,agent.wire,result.session,result.acp_session_id,result.task_run);
 };
 const pending=state=>Object.values(state.runtime).some(snapshot=>JSON.parse(snapshot.checkpoint.messages_json).pending_transcript_projection);
 try{
  await wait(()=>existsSync(join(work,"ready")));const {url}=JSON.parse(readFileSync(join(work,"backend.json"),"utf8"));
  let setup=await start(url);sdk=setup.client;let agent=await sdk.createAgent();let session=await sessionFor(sdk,agent,false);const previous=session.wire;
  await session.startRun({text:"commit then disconnect"});
  await wait(async()=> (await http(url,"/inspect")).observations.held_responses===1);
  const committed=await http(url,"/inspect");assert.ok(Object.keys(committed.receipts).length);assert.ok(pending(committed));
  await sdk.close();sdk=undefined;await http(url,"/release",{});
  setup=await start(url);sdk=setup.client;agent=await sdk.createAgent();session=await sessionFor(sdk,agent,true);
  assert.notEqual(session.wire.generation,previous.generation);
  const run=await session.startRun({text:"continue after restart"});assert.equal((await run.wait({timeoutMs:40000})).terminal.status,"completed");
  const final=await http(url,"/inspect");assert.ok(final.counters.duplicate_applies>0);assert.equal(pending(final),false);
  for(const [operation,receipt] of Object.entries(committed.receipts))assert.deepEqual(final.receipts[operation],receipt);
  let items=0;for(const conversation of Object.values(final.conversations)){const ordinals=new Set();for(const item of conversation.messages){assert.ok(!ordinals.has(item.ordinal));ordinals.add(item.ordinal);items++;}}
  assert.equal(items,final.counters.projected_items);
  await run.close();await session.close();for(const registration of setup.registrations)await registration.close();await agent.close();passed=true;
 }catch(error){process.stderr.write(`managed restart evidence: ${work}\n`);throw error;
 }finally{await sdk?.close();backend.kill();await new Promise(resolve=>backend.exitCode!==null?resolve():backend.once("exit",resolve));if(passed)rmSync(work,{recursive:true,force:true});}
});
