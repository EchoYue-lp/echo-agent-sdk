import test from 'node:test';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {mkdtempSync,writeFileSync,readFileSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {fileURLToPath} from 'node:url';
import {EchoAgentClient,toWireValue} from '../dist/index.js';
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==='1'&&!host)throw new Error('real stream cancellation Host missing');
const deferred=()=>{let resolve;const promise=new Promise(r=>{resolve=r;});return{promise,resolve};};
const bounded=async(promise)=>{let timer;try{return await Promise.race([promise,new Promise((_,reject)=>{timer=setTimeout(()=>reject(new Error('setup ACK cancellation did not reach native producer')),2500);})]);}finally{clearTimeout(timer);}};
test('real Host cancellation after setup ACK reaches retained native stream producer', {skip:!host,timeout:15000}, async()=>{
 const work=mkdtempSync(join(tmpdir(),'sdk-ts-early-ack-cancel-'));assert.equal(spawnSync('python3',[fileURLToPath(new URL('../../shared/testing/mcp_channel_config_fixture.py',import.meta.url)),work]).status,0);
 const sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});let job;let signal;let call;const entered=deferred(),cancelled=deferred(),release=deferred(),shutdown=deferred();const facts=[];
 try{
  const reg=await sdk.registerExtension('custom_agent','route-target',{kind:'custom_agent',descriptor_version:1,name:'route-target',model_name:'fixture',system_prompt:'fixture',tool_names:[]},async(value,cancellation)=>{
   if(value.invocation.operation==='agent_close')return{outcome:'result',result:{operation:'agent_close',value:null}};
   assert.equal(value.invocation.operation,'agent_execute_stream');call=value;signal=cancellation;
   job=(async()=>{try{await Promise.race([new Promise(resolve=>{if(signal.aborted)resolve();else signal.addEventListener('abort',resolve,{once:true});}),shutdown.promise]);if(!signal.aborted)return;facts.push('signal');cancelled.resolve();await release.promise;await sdk.streamWriter(call.stream).complete({kind:'agent',value:{event:'cancelled'}});facts.push('terminal-success');}finally{facts.push('finally');}})();job.catch(()=>{});
   entered.resolve();facts.push('setup-ack');return{outcome:'stream',stream:call.stream};
  });
  const agent=await sdk.createAgent(),session=await agent.createSession();
  const dispatch=await sdk.request('_echo_agent/subagent/dispatch',{session:session.wire,request:toWireValue({agent_name:'route-target',task:'retain cancel after setup 中文🌊',mode_override:'sync'}),idempotency_id:'cancel-route'});
  await bounded(entered.promise);
  await sdk.request('_echo_agent/subagent/control',{subagent:dispatch.subagent,action:'cancel'});
  await bounded(cancelled.promise);assert.ok(signal.aborted);assert.deepEqual(facts,['setup-ack','signal']);
  release.resolve();await bounded(job);assert.deepEqual(facts,['setup-ack','signal','terminal-success','finally']);
  writeFileSync(join(work,'observed.json'),JSON.stringify({call,facts}));
  await session.close();await reg.close();await agent.close();
 }finally{release.resolve();shutdown.resolve();await Promise.allSettled(job?[job]:[]);await sdk.close();}
});
