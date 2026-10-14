import test from "node:test";
import assert from "node:assert/strict";
import {mkdtempSync,readFileSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {execFileSync} from "node:child_process";
import {EchoAgentClient,EchoAgentError} from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required LSP diagnostics Host missing");
const fixture=fileURLToPath(new URL("../../shared/testing/lsp_diagnostic_server.py",import.meta.url));
test("real Host diagnostic notification replacement, error isolation and runtime settlement",{skip:!host,timeout:60000},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-lsp-diagnostics-"));console.log(`LSP diagnostics evidence ${work}`);
 execFileSync("python3",[fixture,"--prepare","--work",work]);
 const sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});
 try{
  const agent=await sdk.createAgent(),session=await agent.createSession();
  const open=async()=>{
   const manager=await session.invoke("lsp.manager.open_exact");
   await session.invoke("lsp.manager.load_config",[manager,JSON.parse(readFileSync(join(work,"lsp.json"),"utf8"))]);
   await session.invoke("lsp.manager.start_server",[manager,"test"]);
   const option=await session.invoke("lsp.manager.get_client",[manager,"test"]);
   assert.equal(option.kind,"variant");assert.equal(option.value.variant,"some");
   return{manager,client:option.value.fields.find(field=>field.name==="client").value.value};
  };
  const {manager,client}=await open();const observed=[];
  assert.deepEqual(JSON.parse(readFileSync(join(work,"advertisement.json"),"utf8")),{relatedInformation:false});
  const corpus=JSON.parse(readFileSync(join(work,"diagnostic-corpus.json"),"utf8"));
  const other=corpus[0].expected;
  for(const item of corpus){
   await session.invoke("lsp.client.hover",[client,`file:///${item.case}`,{line:0,character:3}]);
   if(item.error){
    await assert.rejects(session.invoke("lsp.client.diagnostics",[client,item.uri]),error=>{assert.ok(error instanceof EchoAgentError);assert.equal(error.code,item.error);assert.match(error.message,/publishDiagnostics/);observed.push({case:item.case,error:error.code,message:error.message,retryable:error.retryable,error_operation:error.operation,details:error.details??null});return true;});
    assert.deepEqual(await session.invoke("lsp.client.diagnostics",[client,"file:///other.test"]),other);
   }else{const value=await session.invoke("lsp.client.diagnostics",[client,item.uri]);assert.deepEqual(value,item.expected);observed.push({case:item.case,value});}
  }
  let pid=Number(readFileSync(join(work,"server.pid"),"utf8"));await session.invoke("lsp.manager.shutdown_all",[manager]);assert.throws(()=>process.kill(pid,0),error=>error.code==="ESRCH");
  for(const fatal of ["missing-uri","wrong-uri","missing-params"]){
   const next=await open();pid=Number(readFileSync(join(work,"server.pid"),"utf8"));
   await assert.rejects(session.invoke("lsp.client.hover",[next.client,`file:///${fatal}`,{line:0,character:3}]),error=>error.code==="framework_error");
   const deadline=Date.now()+3000;let status;
   do{status=await session.invoke("lsp.client.status",[next.client]);if(!status.running)break;await new Promise(resolve=>setTimeout(resolve,10));}while(Date.now()<deadline);
   assert.equal(status.running,false);assert.equal(status.initialized,false);assert.equal(status.pid,null);assert.match(status.last_error,/publishDiagnostics/);
   assert.throws(()=>process.kill(pid,0),error=>error.code==="ESRCH");
   await assert.rejects(session.invoke("lsp.client.diagnostics",[next.client,"file:///中文😀.test"]),error=>error.code==="framework_error"&&/publishDiagnostics/.test(error.message));
   await session.invoke("lsp.manager.shutdown_all",[next.manager]);
   observed.push({case:fatal,pending_error:"framework_error",runtime_running:false,process_alive:false,diagnostics_error:"framework_error"});
  }
  writeFileSync(join(work,"observed-diagnostics.json"),JSON.stringify(observed));
  await session.close();await agent.close();
 }finally{await sdk.close();}
});
