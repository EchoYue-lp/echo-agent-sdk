import test from "node:test";
import assert from "node:assert/strict";
import {mkdtempSync,rmSync,readFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {EchoAgentClient,wireUtf8Path} from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required Factory Host E2E binary missing");
test("factory preallocated children preserve two distinct constructed objects through Host evaluation",{skip:!host},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-factory-"));let sdk;
 const instances=new Map(),created=[],executed=[],closed=[];
 let firstConcurrentClosed; const concurrentClose=new Promise(resolve=>{firstConcurrentClosed=resolve;});
 try{
  sdk=await EchoAgentClient.spawn({onStderr:chunk=>process.stderr.write(chunk),hostCommand:"python3",args:[fileURLToPath(new URL("../../python/tests/protocol_host_proxy.py",import.meta.url)),"--host",host,"--work",work,"--mode","normal"]});
  let factory;
  factory=await sdk.registerAgentFactory("factory-owned",{kind:"agent_factory",descriptor_version:1},async call=>{
   const operation=call.invocation.operation;
   if(operation==="factory_create_agent"){
    assert.deepEqual(call.extension,factory.wire);
    const instance=call.invocation.input.instance;
    assert.equal(instance.generation,factory.wire.generation);
    const state=`SDK-object-${created.length}`;
    assert.ok(!instances.has(instance.id));instances.set(instance.id,state);created.push(instance);
    return{outcome:"result",result:{operation,value:{name:"same",model_name:"same",system_prompt:"same",tool_names:[]}}};
   }
   assert.deepEqual(call.factory_origin,factory.wire);
   const instance=call.extension;
   assert.ok(instances.has(instance.id));
   if(operation==="agent_close"){
    closed.push(instance.id);instances.delete(instance.id);
    if(created[2]?.id===instance.id)firstConcurrentClosed();
    return{outcome:"result",result:{operation,value:null}};
   }
   assert.ok(["agent_execute","agent_chat","agent_execute_stream","agent_chat_stream"].includes(operation));executed.push(instance.id);
   if(operation.endsWith("_stream")) {
    const state=instances.get(instance.id);
    setImmediate(()=>{void (async()=>{
      if(created[3]?.id===instance.id){await concurrentClose;assert.ok(instances.has(instance.id));}
      await sdk.streamWriter(call.stream).complete({kind:"agent",value:{event:"final_answer",data:{text:state}}});
    })();});
    return {outcome:"stream",stream:call.stream};
   }
   return{outcome:"result",result:{operation,value:instances.get(instance.id)}};
  });
  const agent=await sdk.createAgent();const session=await agent.createSession();
  const runner=await sdk.call("echo_agent::eval::runner::EvalRunner::new",agent.wire,[session.wire,wireUtf8Path(join(work,"eval-one"))]);
  const cases=["one","two"].map(id=>({id,name:id,description:"same descriptor distinct state",domain:null,task:"return SDK",project_fixture:null,success_criteria:{type:"output_contains",substring:"SDK"},constraints:{}}));
  const firstReport=await sdk.call("echo_agent::eval::runner::EvalRunner::run_all",agent.wire,[session.wire,runner,cases,factory.wire]);
  assert.equal(firstReport.results.length,2,JSON.stringify(firstReport));assert.ok(firstReport.results.every(item=>item.success),JSON.stringify(firstReport));
  const other=await agent.createSession();
  const secondRunner=await sdk.call("echo_agent::eval::runner::EvalRunner::new",agent.wire,[other.wire,wireUtf8Path(join(work,"eval-two"))]);
  const parallelReports=await Promise.all([
    sdk.call("echo_agent::eval::runner::EvalRunner::run_all",agent.wire,[session.wire,runner,[{...cases[0],id:"parallel-one"}],factory.wire]),
    sdk.call("echo_agent::eval::runner::EvalRunner::run_all",agent.wire,[other.wire,secondRunner,[{...cases[1],id:"parallel-two"}],factory.wire]),
  ]);
  assert.ok(parallelReports.every(report=>report.results.length===1&&report.results[0].success),JSON.stringify(parallelReports));
  const lastReport=await sdk.call("echo_agent::eval::runner::EvalRunner::run_all",agent.wire,[session.wire,runner,[{...cases[0],id:"parent-still-live"}],factory.wire]);
  assert.ok(lastReport.results.length===1&&lastReport.results[0].success,JSON.stringify(lastReport));
  assert.equal(created.length,5);assert.equal(new Set(created.map(item=>item.id)).size,5);
  assert.deepEqual(new Set(executed),new Set(created.map(item=>item.id)));
  assert.deepEqual(new Set(closed),new Set(created.map(item=>item.id)));
  assert.equal(instances.size,0);
  await factory.close();await other.close();await session.close();await agent.close();
 }finally{await sdk?.close();rmSync(work,{recursive:true,force:true});}
});

test("late factory construction remains owned until same reserved child cleanup settles",{skip:!host},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-factory-late-"));let sdk;let release;let cancellation;let child;let parent;let closed=0;let created=0;let executed=0;
 const gate=new Promise(resolve=>{release=resolve;});
 const until=async predicate=>{const end=Date.now()+15000;while(!predicate()){assert.ok(Date.now()<end,"late factory condition did not occur");await new Promise(resolve=>setTimeout(resolve,10));}};
 try{
  sdk=await EchoAgentClient.spawn({onStderr:chunk=>process.stderr.write(chunk),hostCommand:"python3",args:[fileURLToPath(new URL("../../python/tests/protocol_host_proxy.py",import.meta.url)),"--host",host,"--work",work,"--mode","normal"]});
  const factory=await sdk.registerExtension("agent_factory","late-factory",{kind:"agent_factory",descriptor_version:1},async(call,signal)=>{
   const operation=call.invocation.operation;
   if(operation==="factory_create_agent"){
    child=call.invocation.input.instance;parent=call.extension;cancellation=signal;
    await gate;created++;
    return {outcome:"result",result:{operation,value:{name:"late",model_name:"late",system_prompt:"late",tool_names:[]}}};
   }
   if(operation!=="agent_close")executed++;
   assert.equal(operation,"agent_close");assert.deepEqual(call.extension,child);assert.deepEqual(call.factory_origin,parent);assert.equal(created,1);closed++;
   return {outcome:"result",result:{operation,value:null}};
  },{seconds:"0",nanos:100_000_000});
  const agent=await sdk.createAgent();const session=await agent.createSession();
  const runner=await sdk.call("echo_agent::eval::runner::EvalRunner::new",agent.wire,[session.wire,wireUtf8Path(join(work,"eval"))]);
  const request=sdk.call("echo_agent::eval::runner::EvalRunner::run_all",agent.wire,[session.wire,runner,[{id:"late",name:"late",description:"delayed constructor",domain:null,task:"SDK",project_fixture:null,success_criteria:{type:"output_contains",substring:"SDK"},constraints:{}}],factory.wire]);
  let requestDone=false;request.then(()=>{requestDone=true;},()=>{requestDone=true;});
  await until(()=>child&&cancellation?.aborted);
  assert.equal(readFileSync(join(work,"frames.jsonl"),"utf8").split("\n").some(line=>{if(!line)return false;const frame=JSON.parse(line);return frame.method==="_echo_agent/extension/invoke"&&frame.params.invocation.operation==="agent_close"&&frame.params.extension.id===child.id;}),false);
  assert.equal(created,0);assert.equal(closed,0);assert.equal(requestDone,false);
  await assert.rejects(factory.close(),error=>error.code==="extension_conflict");
  // The same constructor must actually finish before its reserved child can
  // be cleaned. The cancelled client returns its final Cancelled response.
  release();const report=await request;await until(()=>closed===1);
  assert.equal(report.results.length,1);assert.equal(report.results[0].success,false);
  assert.equal(created,1);assert.equal(closed,1);assert.equal(executed,0);
  const trace=readFileSync(join(work,"frames.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line));
  const original=trace.find(frame=>frame.method==="_echo_agent/extension/invoke"&&frame.params.invocation.operation==="factory_create_agent"&&frame.params.invocation.input.instance.id===child.id);
  const clientTrace=readFileSync(join(work,"client-frames.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line));
  assert.ok(original);const replies=clientTrace.filter(frame=>frame.id===original.id&&(Object.hasOwn(frame,"result")||Object.hasOwn(frame,"error")));
  assert.equal(replies.length,1);const [reply]=replies;assert.equal(Object.hasOwn(reply,"error"),false);
  assert.equal(reply.result.outcome,"error");assert.equal(reply.result.error.code,"cancelled");
  await factory.close();await session.close();await agent.close();
 }catch(error){process.stderr.write(`late Factory failure evidence: ${work}\n`);throw error;
 }finally{release?.();await sdk?.close();console.log("Late Factory evidence",work);}
});
