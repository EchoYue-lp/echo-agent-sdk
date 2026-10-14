import test from "node:test";
import assert from "node:assert/strict";
import {spawn} from "node:child_process";
import {mkdtempSync,existsSync,readFileSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {EchoAgentClient,toWireValue,fromWireValue} from "/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/task-pause-diagnostic/sdk-client/dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required real Task/Subagent Host binary missing");
const fixture="/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/shared/testing/task_subagent_fixture.py";
const mark=(label,detail={})=>console.log("TRACE "+JSON.stringify({at:Date.now(),label,...detail}));
async function until(check){const deadline=Date.now()+15000;while(!(await check())){assert.ok(Date.now()<deadline,"observed framework condition did not occur");await new Promise(r=>setTimeout(r,10));}}
async function scenario(action){
 const work=mkdtempSync(join(tmpdir(),`sdk-ts-task-${action}-`));console.log(`Task evidence ${work}`);
 const backend=spawn("python3",[fixture,"--work",work]);let sdk;
 try{
  await until(()=>existsSync(join(work,"ready")));const{url}=JSON.parse(readFileSync(join(work,"fixture.json"),"utf8"));
  const http=async(path,payload)=>{const response=await fetch(url+path,{method:payload===undefined?"GET":"POST",headers:{"Content-Type":"application/json"},...(payload===undefined?{}:{body:JSON.stringify(payload)})});assert.equal(response.status,200);return response.json();};
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});
  const registrations=[];
  registrations.push(await sdk.registerAgentComponent("task-store",{kind:"agent_component",descriptor_version:1,component:"revisioned_task_store",name:"task-store",capabilities:{execution_cas_v1:true}},async call=>{
   const reply=await http("/store",call.invocation.input.call);
   return reply.kind==="error"?{outcome:"error",error:reply.error}:{operation:"agent_component_call",value:{component:"revisioned_task_store",result:reply.result}};
  }));
  const counts={a:0,b:0,c:0,d:0};const prompts={};const completions=[];const callbackJobs=[];
  for(const lane of ["a","b","c","d"]){
   registrations.push(await sdk.registerExtension("custom_agent",`task-${lane}`,{kind:"custom_agent",descriptor_version:1,name:`task-${lane}`,model_name:"fixture",system_prompt:"fixture",tool_names:[]},async(call,signal)=>{
    const operation=call.invocation.operation;mark("reverse_entry",{lane,operation,invocation_id:call.invocation_id});
    if(operation==="agent_close")return{outcome:"result",result:{operation,value:null}};
    assert.equal(operation,"agent_execute_stream");const attempt=++counts[lane];prompts[`${lane}/${attempt}`]=call.invocation.input.task;
    await http("/agent",{lane,attempt,task:call.invocation.input.task,extension:call.extension});
    const job=Promise.resolve().then(async()=>{
     const interrupted=new Promise(resolve=>{if(signal.aborted){mark("signal_already",{lane,attempt});resolve();}else signal.addEventListener("abort",()=>{mark("signal",{lane,attempt});resolve();},{once:true});});
     if((lane==="a"||lane==="b")&&attempt===1)await Promise.race([http("/gate",{lane,attempt}),interrupted]);
     const writer=sdk.streamWriter(call.stream);
     if(signal.aborted){mark("send_cancelled",{lane,attempt});await writer.complete({kind:"agent",value:{event:"cancelled"}});mark("sent_cancelled",{lane,attempt});}
     else{const output=`${lane.toUpperCase()}-canonical-result 中文🧪`;completions.push({lane,attempt,output});await writer.complete({kind:"agent",value:{event:"final_answer",data:{text:output}}});}
    });job.catch(()=>{});callbackJobs.push(job);
    mark("return_setup_ack",{lane,attempt,stream:call.stream});return{outcome:"stream",stream:call.stream};
   }));
  }
  const agent=await sdk.createAgent();const session=await agent.createSession();const task_run=session.taskRun;
  const tasks=[{id:"a",title:"A",description:"controlled A",extension:{subagent:"task-a"}},{id:"b",title:"B",description:"independent B",extension:{subagent:"task-b"}},{id:"c",title:"C",description:"after B",depends_on:["b"],extension:{subagent:"task-c"}}];
  if(action==="pause")tasks.push({id:"d",title:"D",description:"after resume A and B",depends_on:["a","b"],extension:{subagent:"task-d"}});
  const created=await sdk.request("_echo_agent/task/create",{task_run,spec:toWireValue({execution_mode:"parallel",tasks})});
  const handles=Object.fromEntries(tasks.map((task,index)=>[task.id,created.tasks[index]]));
  const renamed=await sdk.request("_echo_agent/task/update",{task_run,patch:toWireValue({base_revision:BigInt(created.revision),reason:"fixture rename",operations:[{op:"update",task_id:"a",patch:{title:"A canonical v2"}}]})});
  const beforeStale=await http("/inspect");
  await assert.rejects(sdk.request("_echo_agent/task/update",{task_run,patch:toWireValue({base_revision:BigInt(created.revision),reason:"stale writer",operations:[{op:"update",task_id:"a",patch:{title:"must not commit"}}]})}),error=>error.code==="framework_error");
  assert.deepEqual(await http("/inspect"),beforeStale);
  await sdk.request("_echo_agent/task/execute",{task_run});await until(()=>counts.a===1&&counts.b===1);
  await assert.rejects(sdk.request("_echo_agent/task/execute",{task_run}),error=>error.code==="invalid_value"&&/already executing/.test(error.message));
  mark("before_control");const controlled=await sdk.request("_echo_agent/task/control",{task_run,task:handles.a,action});assert.equal(controlled.accepted,true);mark("after_control");
  const snapshot=()=>sdk.request("_echo_agent/task/list",{task_run});let listed;
  await until(async()=>{listed=await snapshot();return listed.tasks.some(item=>item.task.id===handles.a.id&&item.status.status===(action==="pause"?"paused":"cancelled"));});
  mark("observed_paused");assert.equal(listed.tasks.find(item=>item.task.id===handles.b.id).status.status,"running");assert.equal(listed.tasks.find(item=>item.task.id===handles.c.id).status.status,"pending");
  await http("/release",{lane:"b",attempt:1});
  await until(async()=>{listed=await snapshot();return [handles.b,handles.c].every(handle=>listed.tasks.some(item=>item.task.id===handle.id&&item.status.status==="completed"));});
  mark("observed_bc_completed");assert.ok(prompts["c/1"].includes("Completed dependency b")&&prompts["c/1"].includes("B-canonical-result"));
  if(action==="pause"){
   mark("before_resume");assert.equal((await sdk.request("_echo_agent/task/control",{task_run,task:handles.a,action:"resume"})).accepted,true);mark("after_resume");
   await until(async()=>{try{await sdk.request("_echo_agent/task/execute",{task_run});return true;}catch(error){if(error.code!=="invalid_value"||!/already executing/.test(error.message))throw error;return false;}});
   await until(async()=>{listed=await snapshot();return listed.tasks.every(item=>item.status.status==="completed");});
   assert.equal(counts.a,2);assert.equal(counts.b,1);assert.equal(counts.c,1);assert.equal(counts.d,1);
   assert.ok(prompts["d/1"].includes("Completed dependency b")&&prompts["d/1"].includes("B-canonical-result"));
  }else{assert.equal(counts.a,1);assert.equal(counts.b,1);assert.equal(counts.c,1);}
  const persisted=await http("/inspect");const graph=Object.values(persisted.graphs)[0];assert.equal(graph.snapshot.revision,renamed.revision);
  assert.ok(graph.snapshot.tasks.filter(task=>["b","c"].includes(task.spec.id)).every(task=>task.execution.status.status==="completed"&&task.execution.result));
  const executionCommits=persisted.commits.filter(commit=>commit.operation==="revisioned_task_compare_and_commit_execution");
  const running=executionCommits.flatMap(commit=>commit.after.snapshot.tasks.filter(task=>task.execution.status.status==="running").map(task=>({task:task.spec.id,claim:task.execution.claim})));
  assert.ok(running.every(item=>item.claim&&item.claim.revision===renamed.revision&&item.claim.claim_id&&item.claim.spec_hash));
  const completedB=executionCommits.filter(commit=>commit.after.snapshot.tasks.some(task=>task.spec.id==="b"&&task.execution.status.status==="completed"));
  assert.ok(completedB.length);const bExecution=completedB[0].after.snapshot.tasks.find(task=>task.spec.id==="b").execution;
  assert.ok(completedB.every(commit=>JSON.stringify(commit.after.snapshot.tasks.find(task=>task.spec.id==="b").execution)===JSON.stringify(bExecution)),"resume must reuse canonical B result without rewriting its execution facts");
  assert.equal(fromWireValue(bExecution.result).output,"B-canonical-result 中文🧪");
  writeFileSync(join(work,"observed.json"),JSON.stringify({action,counts,statuses:listed.tasks.map(item=>item.status),graph,persisted}));
  await http("/release",{lane:"a",attempt:1});await session.close();for(const registration of registrations)await registration.close();await agent.close();await Promise.all(callbackJobs);
 }finally{await sdk?.close();backend.kill();await new Promise(resolve=>backend.exitCode!==null?resolve():backend.once("exit",resolve));}
}
for(const action of ["pause","cancel"])test(`exact Task ${action} preserves independent B/C`,{timeout:60000,skip:!host},()=>scenario(action));
async function singleScenario(kind){
 const work=mkdtempSync(join(tmpdir(),`sdk-ts-task-${kind}-`));console.log(`Task evidence ${work}`);const backend=spawn("python3",[fixture,"--work",work]);let sdk;
 try{
  await until(()=>existsSync(join(work,"ready")));const{url}=JSON.parse(readFileSync(join(work,"fixture.json"),"utf8"));
  const http=async(path,payload)=>{const response=await fetch(url+path,{method:payload===undefined?"GET":"POST",headers:{"Content-Type":"application/json"},...(payload===undefined?{}:{body:JSON.stringify(payload)})});assert.equal(response.status,200);return response.json();};
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});const registrations=[];let invocations=0;let rawCall;const callbackJobs=[];
  if(kind==="crud-only")registrations.push(await sdk.registerAgentComponent("crud-store",{kind:"agent_component",descriptor_version:1,component:"revisioned_task_store",name:"crud-store",capabilities:{execution_cas_v1:false}},async call=>{const reply=await http("/store",call.invocation.input.call);return reply.kind==="error"?{outcome:"error",error:reply.error}:{operation:"agent_component_call",value:{component:"revisioned_task_store",result:reply.result}};}));
  registrations.push(await sdk.registerExtension("custom_agent","await-target",{kind:"custom_agent",descriptor_version:1,name:"await-target",model_name:"fixture",system_prompt:"fixture",tool_names:[]},async(call,signal)=>{
   const operation=call.invocation.operation;if(operation==="agent_close")return{outcome:"result",result:{operation,value:null}};
   assert.ok(["agent_execute","agent_execute_stream"].includes(operation),operation);invocations++;rawCall=call;
   await http("/agent",{lane:"await",attempt:1,task:call.invocation.input.task,context:call.context??null,extension:call.extension});
   if(operation.endsWith("_stream")){
    const job=Promise.resolve().then(async()=>{await http("/gate",{lane:"await",attempt:1});await sdk.streamWriter(call.stream).complete({kind:"agent",value:{event:"final_answer",data:{text:"full retained result 中文🧪"}}});});job.catch(()=>{});callbackJobs.push(job);
    return{outcome:"stream",stream:call.stream};
   }
   await http("/gate",{lane:"await",attempt:1});return{outcome:"result",result:{operation,value:"full retained result 中文🧪"}};
  }));
  const agent=await sdk.createAgent(),session=await agent.createSession();
  if(kind==="crud-only"){
   await sdk.request("_echo_agent/task/create",{task_run:session.taskRun,spec:toWireValue({tasks:[{id:"pending",title:"pending",description:"no user call",extension:{subagent:"await-target"}}]})});
   const before=await http("/inspect");await assert.rejects(sdk.request("_echo_agent/task/execute",{task_run:session.taskRun}),error=>{assert.equal(error.code,"feature_unavailable");assert.deepEqual(error.handle,session.taskRun);return true;});
   assert.deepEqual(await http("/inspect"),before);assert.equal(invocations,0);
   assert.ok((await sdk.request("_echo_agent/task/list",{task_run:session.taskRun})).tasks.every(task=>task.status.status==="pending"));
   writeFileSync(join(work,"observed.json"),JSON.stringify({kind,invocations,execution_commits:0,error:"feature_unavailable"}));
  }else{
   const payload={session:session.wire,request:toWireValue({agent_name:"await-target",task:"await retained 中文🧪",mode_override:"sync",constraints:["constraint retained 🧪"],prompt_context:{acceptance_criteria:["field preserved"]}}),idempotency_id:"one-receipt"};
   const [one,two]=await Promise.all([sdk.request("_echo_agent/subagent/dispatch",payload),sdk.request("_echo_agent/subagent/dispatch",payload)]);assert.deepEqual(one.subagent,two.subagent);
   await until(()=>invocations===1);
   const short=await sdk.request("_echo_agent/subagent/await",{subagent:one.subagent,timeout:{seconds:"0",nanos:5000000}});assert.equal(short.settled,false);assert.ok(!Object.hasOwn(short,"result"));
   await assert.rejects(sdk.request("_echo_agent/subagent/dispatch",{...payload,request:toWireValue({agent_name:"await-target",task:"conflicting payload"})}),error=>error.code==="invalid_value");
   await http("/release",{lane:"await",attempt:1});
   const full=await sdk.request("_echo_agent/subagent/await",{subagent:one.subagent}),repeat=await sdk.request("_echo_agent/subagent/await",{subagent:one.subagent});assert.deepEqual(full,repeat);assert.equal(full.settled,true);
   const result=fromWireValue(full.result);assert.equal(result.output,"full retained result 中文🧪");assert.ok(rawCall.invocation.input.task.includes("constraint retained 🧪"));
   assert.equal((await sdk.request("_echo_agent/subagent/control",{subagent:one.subagent,action:"guidance",payload:"late"})).accepted,false);assert.equal(invocations,1);
   writeFileSync(join(work,"observed.json"),JSON.stringify({kind,invocations,short,full,rawCall}));
  }
  await session.close();for(const registration of registrations)await registration.close();await agent.close();await Promise.all(callbackJobs);
 }finally{await sdk?.close();backend.kill();await new Promise(resolve=>backend.exitCode!==null?resolve():backend.once("exit",resolve));}
}
for(const kind of ["subagent-await","crud-only"])test(`real Task/Subagent ${kind}`,{timeout:60000,skip:!host},()=>singleScenario(kind));
