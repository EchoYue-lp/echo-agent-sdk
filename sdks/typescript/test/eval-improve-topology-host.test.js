import test from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, readdirSync, renameSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, wireUtf8Path } from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw Error("required analysis Host missing");
const fixture=fileURLToPath(new URL("../../shared/testing/eval_improve_topology_fixture.py",import.meta.url));
function helper(work,...args){const output=spawnSync("python3",[fixture,"--work",work,...args],{encoding:"utf8"});assert.equal(output.status,0,output.stderr);return output.stdout;}
const outcome=(operation,value=null)=>({outcome:"result",result:{operation,value}});
for(const group of ["eval","improve","topology"])test(`native ${group} accepted consumers`,{skip:!host,timeout:60000},async()=>{
 const root=process.env.ECHO_AGENT_SDK_ANALYSIS_ROOT??fileURLToPath(new URL("../../../target/analysis-native",import.meta.url));mkdirSync(root,{recursive:true});const work=mkdtempSync(join(root,`typescript-${group}-`));helper(work,"--prepare",group);const spec=JSON.parse(readFileSync(join(work,"scenario.json"),"utf8"));
 const facts={language:"typescript",group,created:[],closed:[],executed:[],callbacks:[],factory_finally:0,cancel_observed:0,rows:[]},instances=new Map(),values={};let mode="plain",factory, releaseDeadline, noticeDeadline; const deadlineRelease=new Promise(resolve=>{releaseDeadline=resolve;}); const deadlineCancelled=new Promise(resolve=>{noticeDeadline=resolve;});
 const sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")],env:{...process.env,TMPDIR:join(work,"runtime-temp")}});
 try{
  factory=await sdk.registerAgentFactory("analysis-factory",{kind:"agent_factory",descriptor_version:1},async(call,signal)=>{
   const op=call.invocation.operation;facts.callbacks.push(call);
   if(op==="factory_create_agent"){const instance=call.invocation.input.instance;assert.ok(!instances.has(instance.id));instances.set(instance.id,instance);facts.created.push(instance.id);return outcome(op,{name:"native-case",model_name:"native-case",system_prompt:"native-case",tool_names:[]});}
   assert.deepEqual(call.factory_origin,factory.wire);const identity=call.extension.id;assert.ok(instances.has(identity));
   if(op==="agent_close"){facts.closed.push(identity);instances.delete(identity);return outcome(op);}
   facts.executed.push(identity);
   try{const task=call.invocation.input.task??call.invocation.input.message??"";
    if(task==="hang"){await new Promise(resolve=>signal.aborted?resolve():signal.addEventListener("abort",resolve,{once:true}));facts.cancel_observed+=Number(signal.aborted);noticeDeadline();await deadlineRelease;return outcome(op,"cancelled");}
    const answer=task==="bad"?"wrong response":spec.factory_answer;
    if(op.endsWith("_stream")){await sdk.streamWriter(call.stream).complete({kind:"agent",value:{event:"final_answer",data:{text:answer}}});return {outcome:"stream",stream:call.stream};}return outcome(op,answer);
   }finally{facts.factory_finally++;}
  });
  const custom=await sdk.registerCustomAgent("analysis-custom",{kind:"custom_agent",descriptor_version:1,name:"analysis-custom",model_name:"analysis-custom",system_prompt:"analysis-custom",tool_names:[]},async call=>{
   const op=call.invocation.operation;facts.callbacks.push(call);if(op==="agent_close")return outcome(op);
   const answer=mode==="plain"?spec.factory_answer:mode==="grade"?JSON.stringify(spec.grade_good):"not valid JSON🙂";
   if(op.endsWith("_stream")){await sdk.streamWriter(call.stream).complete({kind:"agent",value:{event:"final_answer",data:{text:answer}}});return {outcome:"stream",stream:call.stream};}return outcome(op,answer);
  });
  const agent=await sdk.createAgent(),sessions={a:await agent.createSession(),b:await agent.createSession()};
  const resolve=value=>{if(Array.isArray(value))return value.map(resolve);if(value===null||typeof value!=="object")return value;if("$ref"in value){let selected=values[value.$ref];for(const key of value.path??[])selected=selected[key];return selected;}if("$path"in value)return wireUtf8Path(join(work,value.$path));if("$string_path"in value)return join(work,value.$string_path);if("$u64"in value)return {kind:"u64",value:value.$u64};if("$f64"in value)return {kind:"f64",value:value.$f64};if("$factory"in value)return factory.wire;if("$custom"in value)return custom.wire;return Object.fromEntries(Object.entries(value).map(([key,item])=>[key,resolve(item)]));};
  for(const step of spec.steps){const row={id:step.id};
   if(step.action==="custom_mode"){mode=step.mode;Object.assign(row,{outcome:"result",value:mode});}
   else if(["block_directory","restore_directory"].includes(step.action)){const directory=join(work,step.directory),backup=directory+".retained";if(step.action==="block_directory"){facts.trajectory_bytes_before=readdirSync(directory).filter(name=>name.endsWith(".jsonl")).sort().map(name=>readFileSync(join(directory,name)).toString("base64"));renameSync(directory,backup);writeFileSync(directory,"blocks write");}else{unlinkSync(directory);renameSync(backup,directory);facts.trajectory_bytes_after=readdirSync(directory).filter(name=>name.endsWith(".jsonl")).sort().map(name=>readFileSync(join(directory,name)).toString("base64"));}Object.assign(row,{outcome:"result",value:true});}
   else{const session=sessions[step.session??"a"],args=resolve(step.arguments);try{let value; const request=step.operation==="facade.resource.close"?await sdk.request("_echo_agent/facade/invoke",{operation:step.operation,signature_digest:sdk.catalog.resolve(step.operation).signature,handle:session.wire,arguments:[{kind:"handle",value:args[0]}]}):session.invoke(step.operation,args);if(step.id==="timeout"){let settled=false;request.finally(()=>{settled=true;}).catch(()=>{});await Promise.race([request,deadlineCancelled,new Promise(resolve=>setTimeout(resolve,16000))]);facts.deadline_waiter_pending_during_cleanup=facts.cancel_observed===1&&!settled;facts.workspace_exists_during_cleanup=readdirSync(join(work,"eval-configured")).some(name=>name.startsWith("eval-"));releaseDeadline();}value=await request;Object.assign(row,{outcome:"result",value});values[step.id]=value;}catch(error){Object.assign(row,{outcome:"error",code:error.code,message:error.message,details:error.details,operation:error.operation});}}
   facts.rows.push(row);writeFileSync(join(work,"observed.json"),JSON.stringify(facts,null,2));
  }
  assert.equal(instances.size,0);await sessions.a.close();await sessions.b.close();await agent.close();await custom.close();await factory.close();
 }finally{releaseDeadline();await sdk.close();writeFileSync(join(work,"observed.json"),JSON.stringify(facts,null,2));console.log(`Analysis native evidence: ${work}`);}
 console.log(helper(work,"--verify"));
});
