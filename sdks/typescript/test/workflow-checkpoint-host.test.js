import test from "node:test";
import assert from "node:assert/strict";
import { spawn, spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync, rmSync, mkdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, EchoAgentError } from "../dist/index.js";
const host = process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required Workflow Host missing");
const fixture = fileURLToPath(new URL("../../shared/testing/workflow_checkpoint_fixture.py", import.meta.url));
function helper(work, args) {
  const output = spawnSync("python3",[fixture,"--work",work,...args],{encoding:"utf8"});
  assert.equal(output.status,0,output.stderr);return output.stdout;
}
test("workflow routing/checkpoint/claims/branch/owner and shape boundary",{skip:!host,timeout:120000},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-workflow-"));let sdk,passed=false;
 try {
  helper(work,["--prepare"]);const scenario=JSON.parse(readFileSync(join(work,"scenario.json"),"utf8"));
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")],onStderr:chunk=>process.stderr.write(chunk)});
  const agent=await sdk.createAgent();const sessions={a:await agent.createSession(),b:await agent.createSession()};
  const results={},rows=[];
  for(const step of scenario.steps){
   const session=sessions[step.session];
   if(step.action==="close"){await session.close();const row={id:step.id,outcome:"closed"};rows.push(row);results[step.id]=row;writeFileSync(join(work,"results.json"),JSON.stringify(rows));continue;}
   const resolution=join(work,"resolve.json");writeFileSync(resolution,JSON.stringify({value:step.arguments,results}));
   const args=JSON.parse(helper(work,["--resolve",resolution]));writeFileSync(join(work,`call-${step.id}.json`),JSON.stringify({operation:step.operation,arguments:args}));let row;
   try {row={id:step.id,outcome:"result",value:await sdk.call(step.operation,session.wire,args)};}
   catch(error){assert.ok(error instanceof EchoAgentError,String(error));row={id:step.id,outcome:"error",code:error.code,message:error.message,operation:error.operation,retryable:error.retryable,details:error.details};}
   rows.push(row);results[step.id]=row;writeFileSync(join(work,"results.json"),JSON.stringify(rows));
  }
  await sessions.b.close();await agent.close();await sdk.close();sdk=undefined;
  const resultPath=join(work,"results.json");writeFileSync(resultPath,JSON.stringify(rows));
  const verification=helper(work,["--verify",resultPath]);process.stdout.write(`Workflow evidence: ${verification}`);passed=true;
 }finally{await sdk?.close();if(passed&&!process.env.ECHO_AGENT_SDK_RETAIN_EVIDENCE)rmSync(work,{recursive:true,force:true});else process.stderr.write(`Workflow evidence retained: ${work}\n`);}
});


test("protected Workflow claim keeps bound Store and original LLM physical owner", {skip:!host, timeout:120000}, async()=>{
 const root=fileURLToPath(new URL("../../../target/workflow-protected-claims/",import.meta.url));mkdirSync(root,{recursive:true});
 const work=mkdtempSync(join(root,"typescript-")),rows=[],results={},flights=new Map();
 const backend=spawn("python3",[fixture,"--work",work,"--serve-protected"]);
 const exited=new Promise((resolve,reject)=>{backend.once("error",reject);backend.once("exit",(code,signal)=>resolve({code,signal}));});
 let sdk,http,verified=false;
 try{
  const ready=join(work,"protected-ready.json"),deadline=Date.now()+10000;
  while(!existsSync(ready)){assert.ok(Date.now()<deadline,"owned Workflow fixture not ready");await new Promise(resolve=>setTimeout(resolve,10));}
  const {url}=JSON.parse(readFileSync(ready,"utf8"));
  http=async(path,value)=>{const response=await fetch(url+path,{method:value===undefined?"GET":"POST",headers:{"Content-Type":"application/json"},signal:AbortSignal.timeout(15000),...(value===undefined?{}:{body:JSON.stringify(value)})});assert.equal(response.status,200,await response.clone().text());return response.json();};
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});
  await sdk.registerLlmClient("protected-llm",{kind:"llm_client",descriptor_version:1,model_name:"protected-fixture",supports_streaming:false},async call=>{
   const value=await http("/llm",call);return value.error?{outcome:"error",error:value.error}:{operation:"llm_chat",value:value.response};
  });
  const agent=await sdk.createAgent(),session=await agent.createSession();
  const scenario=JSON.parse(readFileSync(join(work,"protected-scenario.json"),"utf8"));
  const run=async(step,args)=>{try{return{id:step.id,outcome:"result",value:await sdk.call(step.operation,session.wire,args)};}catch(error){assert.ok(error instanceof EchoAgentError,String(error));return{id:step.id,outcome:"error",code:error.code,message:error.message,retryable:error.retryable,operation:error.operation??null,handle:error.handle??null,details:error.details??null};}};
  for(const step of scenario.steps){
   let row;
   if(step.action==="bind_store"){
    const registration=await sdk.registerAgentComponent("protected-"+step.store,{kind:"agent_component",descriptor_version:1,component:"workflow_checkpoint_store",name:step.store,capabilities:{claim_heartbeat_interval_ms:"1000",protected_claim_v1:step.supported}},async call=>{
     const reply=await http("/store/"+step.store,call);return reply.error?{outcome:"error",error:reply.error}:{operation:"agent_component_call",value:{component:"workflow_checkpoint_store",result:reply}};
    });row={id:step.id,outcome:"registered",wire:registration.wire};
   }else if(step.action==="inspect")row={id:step.id,outcome:"observed",value:await http("/inspect")};
   else if(step.action==="wait_protect")row={id:step.id,outcome:"observed",value:await http("/wait_protect",{})};
   else if(step.action==="release_protect")row={id:step.id,outcome:"released",value:await http("/release_protect",{})};
   else if(step.action==="release_join")row={id:step.id,outcome:"joined",value:await http("/release_join",{})};
   else if(step.action==="await_call")row={...await flights.get(step.flight),id:step.id};
   else{
    const resolution=join(work,"protected-resolve.json");writeFileSync(resolution,JSON.stringify({value:step.arguments,results}));
    const args=JSON.parse(helper(work,["--resolve",resolution]));
    if(step.action==="start_call"){flights.set(step.id,run(step,args));row={id:step.id,outcome:"started"};}
    else row=await run(step,args);
   }
   rows.push(row);results[step.id]=row;writeFileSync(join(work,"protected-results.json"),JSON.stringify(rows,null,2));
  }
  console.log(helper(work,["--verify-protected",join(work,"protected-results.json")]));
  verified=true;
 }finally{
  try{if(http){await http("/release_join",{});await Promise.all(flights.values());}}
  finally{try{
   if(sdk&&verified){
    const child=sdk.process,binding=results['while-physical-held'].value.physical.binding;
    const address=sdk.invocationControllers.get(binding.invocation_id);assert.ok(address);assert.equal(address.signal.aborted,false);
    await sdk.close();assert.equal(sdk.process,child);assert.ok(child.exitCode!==null||child.signalCode!==null);
    const after=await http('/inspect'),before=results['after-external-join'].value;
    for(const key of ['stores','llm_calls','calls'])assert.deepEqual(after[key],before[key],`transport cleanup changed ${key}`);
    writeFileSync(join(work,'typescript-close-facts.json'),JSON.stringify({original_invocation_id:binding.invocation_id,original_cancel_address_retained_before_close:true,host_pid:child.pid,host_exit_code:child.exitCode,host_exit_signal:child.signalCode,host_signal_requested:child.killed,local_transport_cleanup_returned:true,framework_receipt_proved:false,protected_attempt_unchanged:true}));
    writeFileSync(join(work,'typescript-after-close-backend.json'),JSON.stringify(after));
   }else await sdk?.close();
  }finally{
   try{if(http)await http("/shutdown",{});else backend.kill();
    const end=await Promise.race([exited,new Promise((_,reject)=>{const timer=setTimeout(()=>reject(new Error("Workflow backend exit missing")),5000);timer.unref();})]);assert.deepEqual(end,{code:0,signal:null});
   }finally{if(backend.exitCode===null&&backend.signalCode===null)backend.kill("SIGKILL");console.log("Protected Workflow evidence",work);}
  }}
 }
});
