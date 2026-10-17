import test from "node:test";
import assert from "node:assert/strict";
import {mkdtempSync, readFileSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {execFileSync} from "node:child_process";
import {EchoAgentClient, EchoAgentError} from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required Host LSP E2E binary missing");
const fixture=fileURLToPath(new URL("../../shared/testing/lsp_fault_server.py",import.meta.url));
test("real Host LSP common values, failure and manager-owned lifecycle",{skip:!host,timeout:60000},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-lsp-"));console.log(`LSP evidence ${work}`);
 execFileSync("python3",[fixture,"--prepare","--work",work]);
 const sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});
 try{
  const agent=await sdk.createAgent();const session=await agent.createSession();
  const manager=await session.invoke("lsp.manager.open_exact");
  await session.invoke("lsp.manager.load_config",[manager,JSON.parse(readFileSync(join(work,"lsp.json"),"utf8"))]);
  await session.invoke("lsp.manager.start_server",[manager,"test"]);
  const option=await session.invoke("lsp.manager.get_client",[manager,"test"]);
  assert.equal(option.kind,"variant");assert.equal(option.value.variant,"some");const client=option.value.fields.find(field=>field.name==="client").value.value;
  const corpus=JSON.parse(readFileSync(join(work,"corpus.json"),"utf8"));const observed=[];
  for(const item of corpus){
   const operation=`lsp.client.${item.operation}`;const args=[client,`file:///${item.case}`,{line:0,character:3}];
   if(item.error){
    const start=Date.now();await assert.rejects(session.invoke(operation,args),error=>{assert.ok(error instanceof EchoAgentError);assert.equal(error.code,item.error);assert.match(error.message,/Server returned error/);assert.equal(error.retryable,"never");observed.push({operation:item.operation,case:item.case,error:error.code,message:error.message,retryable:error.retryable,error_operation:error.operation,details:error.details??null});return true;});
    assert.ok(Date.now()-start<3000,"malformed response must settle immediately");
    assert.deepEqual(await session.invoke("lsp.client.hover",[client,"file:///hover-string",{line:0,character:3}]),{contents:"中文😀",range:null});
   }else{const value=await session.invoke(operation,args);assert.deepEqual(value,item.expected);observed.push({operation:item.operation,case:item.case,value});}
  }
  const pid=Number(readFileSync(join(work,"server.pid"),"utf8"));
  process.kill(pid,0);
  await session.invoke("lsp.manager.shutdown_all",[manager]);
  assert.throws(()=>process.kill(pid,0),error=>error.code==="ESRCH");
  await assert.rejects(session.invoke("lsp.client.initialize",[client,"file:///cannot-revive"]),error=>error.code==="framework_error"&&/not initialized/i.test(error.message));
  assert.equal(Number(readFileSync(join(work,"server.pid"),"utf8")),pid);
  observed.push({case:"manager-close",process_alive_before:true,process_alive_after:false,stale_error:"framework_error",resurrected:false});
  writeFileSync(join(work,"observed.json"),JSON.stringify(observed));
  await session.close();await agent.close();
 }finally{await sdk.close();}
});
