import test from "node:test";
import assert from "node:assert/strict";
import {spawn,spawnSync} from "node:child_process";
import {mkdtempSync,mkdirSync,existsSync,readFileSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {EchoAgentClient,toWireValue,fromWireValue} from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required real Task/Subagent Host binary missing");
const fixture=fileURLToPath(new URL("../../shared/testing/task_subagent_fixture.py",import.meta.url));
async function until(check){const deadline=Date.now()+15000;while(!(await check())){assert.ok(Date.now()<deadline,"observed framework condition did not occur");await new Promise(r=>setTimeout(r,10));}}
async function scenario(action){
 const work=mkdtempSync(join(tmpdir(),`sdk-ts-task-${action}-`));console.log(`Task evidence ${work}`);
 const backend=spawn("python3",[fixture,"--work",work,...(action==='observation'?['--observation']:[])]);let sdk,http;const formalEvidence={action};
 try{
  await until(()=>existsSync(join(work,"ready")));const{url}=JSON.parse(readFileSync(join(work,"fixture.json"),"utf8"));
  http=async(path,payload)=>{const response=await fetch(url+path,{method:payload===undefined?"GET":"POST",headers:{"Content-Type":"application/json"},...(payload===undefined?{}:{body:JSON.stringify(payload)})});assert.equal(response.status,200);return response.json();};
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")],...(action==='observation'?{onStderr:chunk=>{formalEvidence.host_stderr=(formalEvidence.host_stderr??'')+chunk;}}:{})});
  if(action==='observation'){formalEvidence.host_pid=sdk.process.pid;sdk.process.once('exit',(code,signal)=>{formalEvidence.host_exit={code,signal};});}
  const registrations=[];
  registrations.push(await sdk.registerAgentComponent("task-store",{kind:"agent_component",descriptor_version:1,component:"revisioned_task_store",name:"task-store",capabilities:{execution_cas_v1:true}},async call=>{
   const reply=await http("/store",call.invocation.input.call);
   return reply.kind==="error"?{outcome:"error",error:reply.error}:{operation:"agent_component_call",value:{component:"revisioned_task_store",result:reply.result}};
  }));
  const counts={a:0,b:0,c:0,d:0};const prompts={};const completions=[];const callbackJobs=[];const observations=[];
  let canonicalTaskRunScope;
  for(const lane of ["a","b","c","d"]){
   registrations.push(await sdk.registerExtension("custom_agent",`task-${lane}`,{kind:"custom_agent",descriptor_version:1,name:`task-${lane}`,model_name:"fixture",system_prompt:"fixture",tool_names:[]},async(call,signal)=>{
    const operation=call.invocation.operation;
    if(operation==="agent_close")return{outcome:"result",result:{operation,value:null}};
    assert.equal(operation,"agent_execute_stream");
    assert.equal(call.context?.run_id,canonicalTaskRunScope,"Task callback must retain its active canonical TaskRun scope");
    const attempt=++counts[lane];prompts[`${lane}/${attempt}`]=call.invocation.input.task;
    await http("/agent",{lane,attempt,task:call.invocation.input.task,extension:call.extension});
    if(action==='observation'&&lane==='a'){
     const binding=await http('/physical/admit',call);assert.deepEqual(binding.context,call.context);assert.equal(binding.factory_origin,null);
     const error={code:'framework_error',retryable:'never',message:'same Task attempt remains physically held 中文🙂',operation:'_echo_agent/extension/invoke',details:{fields:[{key:'observation',value:'not_settled'},{key:'run_id',value:binding.run_id},{key:'execution_id',value:binding.execution_id}]}};
     const writer=sdk.streamWriter(call.stream);
     const job=(async()=>{
      await writer.failed(error);observations.push({call,binding,error});formalEvidence.original=observations[0];
      const joined=await http('/physical/join',{});assert.deepEqual(joined,{admitted:true,exit_code:0,stopped:binding});formalEvidence.joined=joined;
      await writer.complete({kind:'agent',value:{event:'final_answer',data:{text:'A-after-physical-close 中文🧪'}}});
     })();job.catch(()=>{});callbackJobs.push(job);return{outcome:'stream',stream:call.stream};
    }
    const job=Promise.resolve().then(async()=>{
     const interrupted=new Promise(resolve=>{if(signal.aborted)resolve();else signal.addEventListener("abort",resolve,{once:true});});
     if((lane==="a"||lane==="b")&&attempt===1)await Promise.race([http("/gate",{lane,attempt}),interrupted]);
     const writer=sdk.streamWriter(call.stream);
     if(signal.aborted){await writer.complete({kind:"agent",value:{event:"cancelled"}});}
     else{const output=`${lane.toUpperCase()}-canonical-result 中文🧪`;completions.push({lane,attempt,output});await writer.complete({kind:"agent",value:{event:"final_answer",data:{text:output}}});}
    });job.catch(()=>{});callbackJobs.push(job);
    return{outcome:"stream",stream:call.stream};
   }));
  }
  const agent=await sdk.createAgent();const session=await agent.createSession();const task_run=session.taskRun;
  canonicalTaskRunScope=session.acpSessionId;
  const tasks=[{id:"a",title:"A",description:"controlled A",extension:{subagent:"task-a"}},{id:"b",title:"B",description:"independent B",extension:{subagent:"task-b"}},{id:"c",title:"C",description:"after B",depends_on:["b"],extension:{subagent:"task-c"}}];
  if(action==="pause"||action==='observation')tasks.push({id:"d",title:"D",description:action==='observation'?'after A and B':'after resume A and B',depends_on:["a","b"],extension:{subagent:"task-d"}});
  const created=await sdk.request("_echo_agent/task/create",{task_run,spec:toWireValue({execution_mode:"parallel",tasks})});
  const handles=Object.fromEntries(tasks.map((task,index)=>[task.id,created.tasks[index]]));
  const renamed=await sdk.request("_echo_agent/task/update",{task_run,patch:toWireValue({base_revision:BigInt(created.revision),reason:"fixture rename",operations:[{op:"update",task_id:"a",patch:{title:"A canonical v2"}}]})});
  const beforeStale=await http("/inspect");
  await assert.rejects(sdk.request("_echo_agent/task/update",{task_run,patch:toWireValue({base_revision:BigInt(created.revision),reason:"stale writer",operations:[{op:"update",task_id:"a",patch:{title:"must not commit"}}]})}),error=>error.code==="framework_error");
  assert.deepEqual(await http("/inspect"),beforeStale);
  await sdk.request("_echo_agent/task/execute",{task_run});await until(()=>counts.a===1&&counts.b===1);
  await assert.rejects(sdk.request("_echo_agent/task/execute",{task_run}),error=>error.code==="invalid_value"&&/already executing/.test(error.message));
  if(action==='observation'){
   await until(()=>observations.length===1);await http('/release',{lane:'b',attempt:1});
   let listed;await until(async()=>{listed=await sdk.request('_echo_agent/task/list',{task_run});return listed.tasks.find(item=>item.task.id===handles.b.id).status.status==='completed';});
   const held=await http('/inspect'),original=observations[0];formalEvidence.held=held;formalEvidence.held_list=listed;formalEvidence.held_counts={...counts};const graph=held.graphs[canonicalTaskRunScope];
   const execution=graph.snapshot.tasks.find(task=>task.spec.id==='a').execution,claim=execution.claim;
   const heldB=graph.snapshot.tasks.find(task=>task.spec.id==='b').execution;
   assert.equal(fromWireValue(heldB.result).output,'B-canonical-result 中文🧪');
   // The original executor resolves the entire ready batch before admitting C.
   assert.equal(listed.tasks.find(item=>item.task.id===handles.c.id).status.status,'pending');
   assert.equal(execution.status.status,'running');assert.ok(execution.result===undefined||execution.result===null);assert.equal(execution.retry_count,0);assert.ok(claim);
   assert.equal(original.binding.run_id,canonicalTaskRunScope);assert.equal(original.binding.execution_id,`${canonicalTaskRunScope}:a:${claim.revision}:${claim.attempt}:${claim.claim_id}`);
   assert.deepEqual(original.binding.child,original.call.extension);assert.deepEqual(original.binding.stream,original.call.stream);assert.equal(original.binding.invocation_id,original.call.invocation_id);
   assert.equal(held.physical.alive,true);assert.equal(held.physical.stopped,false);assert.deepEqual(counts,{a:1,b:1,c:0,d:0});assert.equal(held.model_requests.length,0);
   await http('/physical/release',{});await Promise.all(callbackJobs);
   await until(async()=>{listed=await sdk.request('_echo_agent/task/list',{task_run});return listed.tasks.every(item=>item.status.status==='completed');});
   const listedAgain=await sdk.request('_echo_agent/task/list',{task_run});assert.deepEqual(listedAgain,listed);
   const after=await http('/inspect');assert.equal(after.physical.exit_code,0);assert.equal(after.physical.stopped,true);assert.deepEqual(after.physical.binding,original.binding);
   const transitions=after.commits.filter(commit=>commit.operation==='revisioned_task_compare_and_commit_execution'&&commit.before?.snapshot.tasks.find(task=>task.spec.id==='a').execution.status.status==='running'&&commit.after.snapshot.tasks.find(task=>task.spec.id==='a').execution.status.status==='completed');
   assert.equal(transitions.length,1);const settlement=transitions[0];assert.equal(settlement.commit.expected_revision,renamed.revision);assert.deepEqual(settlement.commit.expected_executions.a.claim,claim);
   const settled=settlement.after.snapshot.tasks.find(task=>task.spec.id==='a').execution;assert.equal(settled.claim,null);assert.equal(fromWireValue(settled.result).output,'A-after-physical-close 中文🧪');assert.equal(settled.retry_count,0);
   const finalTasks=after.graphs[canonicalTaskRunScope].snapshot.tasks;
   assert.deepEqual(finalTasks.find(task=>task.spec.id==='b').execution,heldB);
   assert.equal(fromWireValue(finalTasks.find(task=>task.spec.id==='c').execution.result).output,'C-canonical-result 中文🧪');
   assert.ok(prompts['c/1'].includes('Completed dependency b')&&prompts['c/1'].includes('B-canonical-result'));
   assert.equal(fromWireValue(finalTasks.find(task=>task.spec.id==='d').execution.result).output,'D-canonical-result 中文🧪');
   assert.ok(prompts['d/1'].includes('Completed dependency a')&&prompts['d/1'].includes('A-after-physical-close')&&prompts['d/1'].includes('Completed dependency b')&&prompts['d/1'].includes('B-canonical-result'));
   for(const commit of after.commits){const b=commit.after.snapshot.tasks.find(task=>task.spec.id==='b').execution;if(b.status.status==='completed')assert.deepEqual(b,heldB);}
   assert.deepEqual(counts,{a:1,b:1,c:1,d:1});assert.equal(after.graphs[canonicalTaskRunScope].snapshot.revision,renamed.revision);
   Object.assign(formalEvidence,{counts,observations,held,after,listed,settlement,prompts,scope:canonicalTaskRunScope,revision:renamed.revision,listed_again:listedAgain});writeFileSync(join(work,'observed.json'),JSON.stringify(formalEvidence));
   const verified=spawnSync('python3',[fixture,'--work',work,'--verify-formal',join(work,'observed.json')],{encoding:'utf8'});assert.equal(verified.status,0,verified.stderr);console.log(verified.stdout);
   await session.close();for(const registration of registrations)await registration.close();await agent.close();return;
  }
  const controlled=await sdk.request("_echo_agent/task/control",{task_run,task:handles.a,action});assert.equal(controlled.accepted,true);
  const snapshot=()=>sdk.request("_echo_agent/task/list",{task_run});let listed;
  await until(async()=>{listed=await snapshot();return listed.tasks.some(item=>item.task.id===handles.a.id&&item.status.status===(action==="pause"?"paused":"cancelled"));});
  assert.equal(listed.tasks.find(item=>item.task.id===handles.b.id).status.status,"running");assert.equal(listed.tasks.find(item=>item.task.id===handles.c.id).status.status,"pending");
  await http("/release",{lane:"b",attempt:1});
  await until(async()=>{listed=await snapshot();return [handles.b,handles.c].every(handle=>listed.tasks.some(item=>item.task.id===handle.id&&item.status.status==="completed"));});
  assert.ok(prompts["c/1"].includes("Completed dependency b")&&prompts["c/1"].includes("B-canonical-result"));
  if(action==="pause"){
   assert.equal((await sdk.request("_echo_agent/task/control",{task_run,task:handles.a,action:"resume"})).accepted,true);
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
 }catch(error){if(action==='observation')formalEvidence.failure={name:error.name,code:error.code,message:error.message,stack:error.stack};throw error;}
 finally{
  try{if(action==='observation'&&http){await http('/physical/release',{});formalEvidence.cleanup_join=await http('/physical/join',{});}await sdk?.close();}
  finally{
   try{if(action==='observation'&&http)await http('/shutdown',{});else backend.kill();await new Promise(resolve=>backend.exitCode!==null?resolve():backend.once("exit",resolve));if(action==='observation'){formalEvidence.backend_exit={code:backend.exitCode,signal:backend.signalCode};assert.deepEqual(formalEvidence.backend_exit,{code:0,signal:null});}}
   finally{
    if(action==='observation'){
     if(backend.exitCode===null&&backend.signalCode===null){backend.kill('SIGKILL');formalEvidence.forced_backend_cleanup=true;await new Promise(resolve=>backend.once('exit',resolve));}
     writeFileSync(join(work,'observed.json'),JSON.stringify(formalEvidence));
    }
   }
  }
 }
}
for(const action of ["pause","cancel"])test(`exact Task ${action} preserves independent B/C`,{timeout:60000,skip:!host},()=>scenario(action));
test('B matching Task observation settles only the same real physical attempt',{timeout:60000,skip:!host},()=>scenario('observation'));
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

async function observationScenario(carrier){
 const root=fileURLToPath(new URL('../../../target/cancellation-observation/',import.meta.url));mkdirSync(root,{recursive:true});
 const work=mkdtempSync(join(root,`typescript-${carrier}-`));console.log('B observation evidence',work);
 const backend=spawn('python3',[fixture,'--work',work,'--observation']);const exited=new Promise(resolve=>backend.once('exit',(code,signal)=>resolve({code,signal})));
 let sdk,http;const jobs=[],facts={carrier,source_kind:carrier==='stream'?'issued_stream_with_late_terminal':'issued_stream_without_successor_terminal',actual_complete_sent:false,callbacks:[],closes:[]};
 try{
  await until(()=>existsSync(join(work,'ready')));const {url}=JSON.parse(readFileSync(join(work,'fixture.json'),'utf8'));
  http=async(path,payload)=>{const response=await fetch(url+path,{method:payload===undefined?'GET':'POST',headers:{'Content-Type':'application/json'},signal:AbortSignal.timeout(35000),...(payload===undefined?{}:{body:JSON.stringify(payload)})});assert.equal(response.status,200);return response.json();};
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')],onStderr:chunk=>{facts.host_stderr=(facts.host_stderr??'')+chunk;}});
  facts.host_pid=sdk.process.pid;sdk.process.once('exit',(code,signal)=>{facts.host_exit={code,signal,phase:facts.phase};});
  let factory,observed;const observationSent=new Promise(resolve=>{observed=resolve;});
  factory=await sdk.registerAgentFactory('observation-factory',{kind:'agent_factory',descriptor_version:1},async call=>{
   const operation=call.invocation.operation;facts.callbacks.push(call);await http('/agent',call);
   if(operation==='factory_create_agent')return{outcome:'result',result:{operation,value:{name:'original-physical',model_name:'fixture',system_prompt:'',tool_names:[]}}};
   assert.deepEqual(call.factory_origin,factory.wire);
   if(operation==='agent_close'){
    const state=await http('/inspect');assert.deepEqual(call.extension,state.physical.binding.child);
    assert.equal(state.physical.stopped,true);assert.equal(state.physical.exit_code,0);facts.closes.push(call);
    return{outcome:'result',result:{operation,value:null}};
   }
   assert.equal(operation,'agent_execute_stream');const binding=await http('/physical/admit',call);facts.binding=binding;
   assert.deepEqual(binding.context,call.context);assert.equal(call.context?.run_id,undefined);
   assert.equal(binding.execution_id,call.context.execution_id);assert.equal(binding.invocation_id,call.invocation_id);
   const error={code:'framework_error',retryable:'never',message:'original physical cleanup remains held 中文🙂',operation:'_echo_agent/extension/invoke',details:{fields:[{key:'observation',value:'not_settled'},{key:'run_id',value:binding.run_id},{key:'execution_id',value:binding.execution_id}]}};facts.error=error;
   const writer=sdk.streamWriter(call.stream);
   const job=(async()=>{
    if(carrier==='stream'){await writer.failed(error);observed();}
    const joined=await http('/physical/join',{});assert.deepEqual(joined,{admitted:true,exit_code:0,stopped:binding});facts.joined=joined;
    if(carrier==='stream'){await writer.complete({kind:'agent',value:{event:'final_answer',data:{text:'same physical producer actually exited'}}});facts.actual_complete_sent=true;}
   })();job.catch(()=>{});jobs.push(job);
   if(carrier==='setup'){observed();return{outcome:'error',error};}
   return{outcome:'stream',stream:call.stream};
  });
  const agent=await sdk.createAgent(),session=await agent.createSession(),run=await session.startRun({mode:'execute',text:'delegate original physical work'});
  await Promise.race([observationSent,new Promise((_,reject)=>{const timer=setTimeout(()=>reject(new Error('actual child observation missing')),15000);timer.unref();})]);
  const waited=await run.wait({timeoutMs:250}),state=await run.get(),replay=await run.replay(),held=await http('/inspect');
  Object.assign(facts,{run:run.wire,waited,state,replay,held});assert.deepEqual(waited,{settled:false});assert.equal(state.status,'running');assert.equal('terminal'in state,false);assert.equal('receipt'in state,false);
  assert.ok(replay.events.every(row=>!['final_answer','cancelled','error'].includes(row.envelope.payload.event_type)));
  assert.equal(held.model_requests.length,1);assert.equal(facts.closes.length,0);assert.equal(held.physical.alive,true);assert.equal(held.physical.stopped,false);assert.deepEqual(held.physical.binding,facts.binding);
  assert.equal(facts.callbacks.filter(call=>call.invocation.operation==='factory_create_agent').length,1);assert.equal(facts.callbacks.filter(call=>call.invocation.operation==='agent_execute_stream').length,1);
  assert.deepEqual(facts.binding.child,facts.callbacks.find(call=>call.invocation.operation==='factory_create_agent').invocation.input.instance);assert.deepEqual(facts.binding.stream,facts.callbacks.find(call=>call.invocation.operation==='agent_execute_stream').stream);
  await assert.rejects(session.startRun({mode:'execute',text:'must not admit another execution'}),error=>{facts.held_admission_error={code:error.code,message:error.message,retryable:error.retryable,operation:error.operation,details:error.details,handle:error.handle};return error.code==='invalid_request'&&error.retryable==='after_delay'&&error.operation==='_echo_agent/run/start';});
  await http('/physical/release',{});await Promise.all(jobs);facts.after=await http('/inspect');assert.equal(facts.after.physical.exit_code,0);
  if(carrier==='stream')await until(()=>facts.closes.length===1);
  // An ordinary turn has no matching TaskRun identity. Even a real foreign
  // stream terminal cannot manufacture a canonical Host execution receipt.
  assert.notEqual(facts.binding.run_id,run.wire.id);
  facts.phase='post-closure-run-wait';
  const[one,two]=await Promise.all([run.wait({timeoutMs:250}),run.wait({timeoutMs:250})]);assert.deepEqual(one,two);assert.deepEqual(one,{settled:false});facts.after_wait=one;
  facts.after_run=await run.get();assert.equal(facts.after_run.status,'running');assert.equal('terminal'in facts.after_run,false);assert.equal('receipt'in facts.after_run,false);
  facts.after_replay=await run.replay();assert.ok(facts.after_replay.events.every(row=>!['final_answer','cancelled','error'].includes(row.envelope.payload.event_type)));
  facts.after=await http('/inspect');
  assert.equal(facts.actual_complete_sent,carrier==='stream');assert.equal(facts.closes.length,carrier==='stream'?1:0);assert.equal(facts.after.model_requests.length,1);
  assert.equal(facts.callbacks.filter(call=>call.invocation.operation==='agent_execute_stream').length,1);

 }catch(error){facts.failure={name:error.name,code:error.code,message:error.message,stack:error.stack,phase:facts.phase};throw error;}
 finally{
  try{if(http){await http('/physical/release',{});facts.cleanup_join=await http('/physical/join',{});await Promise.all(jobs);}}
  finally{
   try{facts.phase='sdk-close';await sdk?.close();}
   catch(error){facts.sdk_close_error={name:error.name,code:error.code,message:error.message,stack:error.stack};throw error;}
   finally{
    try{
     if(http)await http('/shutdown',{});else backend.kill();
     const outcome=await Promise.race([exited,new Promise((_,reject)=>{const timer=setTimeout(()=>reject(new Error('owned fixture backend did not exit')),5000);timer.unref();})]);
     facts.backend_exit=outcome;assert.deepEqual(outcome,{code:0,signal:null});
    }finally{
     if(backend.exitCode===null&&backend.signalCode===null){backend.kill('SIGKILL');facts.forced_backend_cleanup=await exited;}
     writeFileSync(join(work,'observed.json'),JSON.stringify(facts,null,2));
    }
   }
  }
 }
}
for(const carrier of ['setup','stream'])test(`B ${carrier} observation preserves real Run and the same physical producer`,{timeout:60000,skip:!host},()=>observationScenario(carrier));
