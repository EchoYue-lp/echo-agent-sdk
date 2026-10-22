import test from 'node:test';
import assert from 'node:assert/strict';
import {spawn,execFileSync} from 'node:child_process';
import {existsSync,mkdirSync,mkdtempSync,readFileSync,writeFileSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {EchoAgentClient} from '../dist/index.js';
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==='1'&&!host)throw Error('required A2A Host missing');
const fixture=fileURLToPath(new URL('../../shared/testing/a2a_host_fixture.py',import.meta.url));
for(const quota of [false,true]) test(quota?'A2A quota rejection drops the source and releases the facade slot':'accepted A2A operations reach real HTTP/SSE with typed owner and error behavior',{skip:!host,timeout:30000},async()=>{
 const root=resolve(process.env.ECHO_AGENT_SDK_A2A_ROOT??'../../target/a2a-native');mkdirSync(root,{recursive:true});const work=mkdtempSync(join(root,'typescript-'));
 if(quota)writeFileSync(join(work,'quota'),'1');
 const peer=spawn('python3',[fixture,'--work',work],{stdio:['ignore','ignore','inherit']});const exited=new Promise(r=>peer.once('exit',r));let sdk;const rows=[],values={};
 try{
  const deadline=Date.now()+10000;while(!existsSync(join(work,'ready'))){assert.equal(peer.exitCode,null);assert.ok(Date.now()<deadline);await new Promise(r=>setTimeout(r,5));}
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')],env:{NO_PROXY:'127.0.0.1,localhost',no_proxy:'127.0.0.1,localhost'},onStderr:text=>writeFileSync(join(work,'host.stderr'),text,{flag:'a'})});const agent=await sdk.createAgent(),sessions={a:await agent.createSession(work),b:await agent.createSession(work)};
  for(const step of JSON.parse(readFileSync(join(work,'scenario.json'),'utf8'))){
   if(step.action==='close_session'){await sessions.a.close();rows.push({id:step.id,value:true});continue;}
   if(step.action==='await_source_closed'){const source=JSON.parse(execFileSync('python3',[fixture,'--work',work,'--await-source',step.source],{encoding:'utf8',timeout:20000}));rows.push({id:step.id,value:true,source});writeFileSync(join(work,'results.json'),JSON.stringify(rows,null,2));continue;}
   const args=step.args.map(arg=>{if(!arg||typeof arg!=='object'||!arg.ref)return arg;let value=values[arg.ref];if(arg.field)value=value[arg.field];if(arg.generation_delta)value={...value,generation:(BigInt(value.generation)+BigInt(arg.generation_delta)).toString()};return value;});
   const row={id:step.id};try{row.value=await sessions[step.session??'a'].invoke(step.op,args);assert.equal(step.error,undefined,`${step.id} unexpectedly succeeded`);values[step.id]=row.value;}catch(error){if(!step.error)throw error;row.error={code:error.code,message:error.message,operation:error.operation,retryable:error.retryable,details:error.details,handle:error.handle};writeFileSync(join(work,"last-error.json"),JSON.stringify({id:step.id,...row.error},null,2));assert.equal(error.code,step.error);}
   rows.push(row);writeFileSync(join(work,'results.json'),JSON.stringify(rows,null,2));
  }
  writeFileSync(join(work,'results.json'),JSON.stringify(rows,null,2));
  const verify=()=>JSON.parse(execFileSync('python3',[fixture,'--work',work,'--verify'],{encoding:'utf8',timeout:20000}));
  if(quota){assert.equal(verify().quota_slot_recovered,true);await sessions.a.close();}
  await sessions.b.close();await agent.close();
  if(!quota)assert.equal(verify().operations.length,9);
  console.log('A2A evidence',work);

 }finally{writeFileSync(join(work,'results.json'),JSON.stringify(rows,null,2));await sdk?.close();writeFileSync(join(work,'stop'),'stop');await exited;}
});
