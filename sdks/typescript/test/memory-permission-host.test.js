import { fileSha256 } from "../../shared/testing/file-sha256.mjs";
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { mkdtemp,readFile,writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawn } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import assert from 'node:assert/strict';
import { EchoAgentClient } from '../dist/client.js';
import { fromWireValue,toWireValue } from '../dist/wire.js';
const host=process.env.ECHO_AGENT_SDK_HOST?.trim();
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==='1'&&!host)throw new Error('required memory/permission Host binary is missing');
const fixturePath=fileURLToPath(new URL('../../shared/testing/memory_permission_fixture.py',import.meta.url));
test('live model setter changes the next actual HTTP request on the same Session',{skip:!host,timeout:60000},async()=>{
 const work=await mkdtemp(join(tmpdir(),'sdk-live-model-typescript-'));
 const fixture=spawn('python3',[fixturePath,'--work',work],{stdio:['ignore','ignore','inherit']});
 const facts={cases:[]};let sdk;
 try{
  const deadline=Date.now()+5000;for(;;){try{await readFile(join(work,'ready'));break;}catch{assert.ok(Date.now()<deadline,'loopback fixture did not become ready');await new Promise(resolve=>setTimeout(resolve,10));}}
  const config=JSON.parse(await readFile(join(work,'host.json'),'utf8'));
  const credential='private-construction-'+randomUUID();
  config.default_agent.model.auth_token=credential;
  await writeFile(join(work,'host.json'),JSON.stringify(config));
  const expectedAuth=createHash('sha256').update('Bearer '+credential).digest('hex');
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});
  let requestOffset=0,authOffset=0;
  for(const variant of ['host_default','explicit']){
   const explicit={variant:'explicit',config_version:1,model:{provider:config.default_agent.model.provider,name:config.default_agent.model.name,base_url:config.default_agent.model.base_url,api_protocol:config.default_agent.model.api_protocol,credential:{source:'inline',token:credential}},agent:{name:'live-model-explicit',system_prompt:'Original model definition 中文🙂',max_iterations:2}};
   const agent=await sdk.createAgent(variant==='host_default'?{variant:'host_default'}:explicit),session=await agent.createSession(work),peer=await agent.createSession(work);
   const row={variant,request_offset:requestOffset,auth_offset:authOffset,agent:agent.wire,session:session.wire,peer:peer.wire,settlements:[],session_model_names:[],replay:[],expected_models:[config.default_agent.model.name,'native-next-model']};facts.cases.push(row);
   const first=await session.startRun({text:'first actual model request 中文🙂'});row.settlements.push(await first.wait({timeoutMs:15000}));assert.equal(row.settlements.at(-1).terminal?.status,'completed');row.replay.push(await first.replay('0',512));await first.close();
   assert.equal(await session.invoke('echo_agent::agent::react::ReactAgent::set_model',[row.expected_models[1]]),null);
   row.session_model_names.push(await session.invoke('echo_core::agent::Agent::model_name'));
   const second=await session.startRun({text:'second actual model request 保留🌊'});row.settlements.push(await second.wait({timeoutMs:15000}));assert.equal(row.settlements.at(-1).terminal?.status,'completed');row.replay.push(await second.replay('0',512));await second.close();
   const requests=(await readFile(join(work,'model-requests.jsonl'),'utf8')).trim().split('\n').map(line=>JSON.parse(line));
   assert.equal(requests.length-requestOffset,2,'one variant emitted more or fewer than its two original model requests');
   row.http_requests=requests.slice(requestOffset,requestOffset+2);row.actual_models=row.http_requests.map(request=>request.model);
   requestOffset=requests.length;
   const auth=(await readFile(join(work,'model-auth-fingerprints.jsonl'),'utf8')).trim().split('\n').map(line=>JSON.parse(line));
   assert.equal(auth.length-authOffset,2);assert.equal(auth.length,requests.length,'each request must have its corresponding header fingerprint');
   assert.equal(authOffset,row.request_offset);row.auth_fingerprints=auth.slice(authOffset,authOffset+2);authOffset=auth.length;
   row.definition_model=await sdk.call('echo_core::agent::Agent::model_name',agent.wire,[]);
   row.peer_model=await peer.invoke('echo_core::agent::Agent::model_name');row.description=await agent.describe();
   await assert.rejects(session.invoke('echo_agent::agent::react::ReactAgent::set_model',['']),error=>{row.invalid_model_error={code:error.code,operation:error.operation,retryable:error.retryable,message:error.message,details:error.details};return error.code==='invalid_value'&&error.retryable==='never';});
   assert.deepEqual(row.actual_models,row.expected_models,'set_model returned success but the original HTTP client kept its previous model');
   assert.deepEqual(row.session_model_names,[row.expected_models[1]],'the Session accessor retained the boxed metadata snapshot');
   assert.equal(row.definition_model,row.expected_models[0],'a Session mutation changed the Agent definition');assert.equal(row.peer_model,row.expected_models[0],'a model switch mutated another Session');
   assert.equal(row.auth_fingerprints.length,2);assert.ok(row.auth_fingerprints.every(entry=>entry.authorization_sha256===expectedAuth&&entry.path==='/v1/chat/completions'));
   assert.equal(JSON.stringify(row).includes(credential),false,'private construction credentials leaked in outward observations');
   await peer.close();await session.close();await agent.close();
  }
  const allRequests=(await readFile(join(work,'model-requests.jsonl'),'utf8')).trim().split('\n').map(line=>JSON.parse(line));
  const allAuth=(await readFile(join(work,'model-auth-fingerprints.jsonl'),'utf8')).trim().split('\n').map(line=>JSON.parse(line));
  assert.equal(allRequests.length,4);assert.equal(allAuth.length,allRequests.length);assert.equal(requestOffset,4);assert.equal(authOffset,4);
 }finally{
  try{await sdk?.close();}finally{
   await writeFile(join(work,'live-model-observed.json'),JSON.stringify({host,host_sha256:fileSha256(host),...facts},null,2));console.log('Live model evidence:',work);
   if(fixture.exitCode===null){const exited=new Promise(resolve=>fixture.once('exit',resolve));fixture.kill();await exited;}
  }
 }
});
test('memory and permission retain live authorities, context and actual effects',{skip:!host},async()=>{
const work=await mkdtemp(join(tmpdir(),'sdk-memory-permission-typescript-'));
const fixture=spawn('python3',[fixturePath,'--work',work],{stdio:['ignore','ignore','inherit']});
const pause=()=>new Promise(resolve=>setTimeout(resolve,10));
async function http(url,path,value){const result=await fetch(url+path,value===undefined?{}:{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(value)});return result.json();}
const rows=[];
let sdk;
try{
 const deadline=Date.now()+5000;for(;;){try{await readFile(join(work,'ready'));break;}catch{assert.ok(Date.now()<deadline);await pause();}}
 const {url}=JSON.parse(await readFile(join(work,'backend.json')));
 sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});
 const humanCalls=[];
 const human=await sdk.registerExtension('human_loop_provider','human-fixture',{kind:'human_loop_provider',descriptor_version:1},async call=>{humanCalls.push(structuredClone(call));const parameters=call.invocation.input.args?fromWireValue(call.invocation.input.args):{};if(['deny','allow'].includes(parameters.phase))return {outcome:'result',result:{operation:'human_loop_request',value:parameters.phase==='deny'?{variant:'rejected',reason:'fixture denies real write'}:{variant:'approved_with_scope',scope:'once'}}};return {outcome:'result',result:{operation:'human_loop_request',value:{variant:'modified_args',args:toWireValue({path:join(work,'modified.txt'),中文:'🌊'}),scope:'once'}}};});
 const agent=await sdk.createAgent();const a=await agent.createSession(work),b=await agent.createSession(work);
 async function invoke(session,operation,args=[]){const value=await session.invoke(operation,args);rows.push({session:session.wire.id,operation,value});return value;}
 const store=await sdk.call('echo_state::memory::store::InMemoryStore::new',agent.wire,[a.wire]);const ns=['sdk','中文'];
 for(const key of ['one','two'])await invoke(a,'memory.resource.put',[store,ns,key,{content:'duplicate-needle中文🌊'}]);
 await sdk.call('echo_state::memory::store::InMemoryStore::put_raw',agent.wire,[a.wire,store,{namespace:ns,key:'expired',value:{content:'expired'},created_at:1,updated_at:1,score:null,importance:5,last_accessed:null,expires_at:2}]);
 assert.equal((await invoke(a,'memory.resource.get',[store,ns,'expired'])).expires_at,'2');
 assert.equal((await invoke(a,'memory.resource.search_with',[store,ns,{text:'duplicate-needle',limit:5,mode:'Keyword'}])).items.length,2);
 assert.equal(await invoke(a,'memory.resource.prune_expired',[store,ns]),'1');assert.equal(await invoke(a,'memory.resource.prune_expired',[store,ns]),'0');assert.equal(await invoke(a,'memory.resource.get',[store,ns,'expired']),null);
 assert.equal(await invoke(a,'memory.resource.dedup_by_content',[store,ns]),'1');assert.equal(await invoke(a,'memory.resource.dedup_by_content',[store,ns]),'0');const survivors=(await invoke(a,'memory.resource.list',[store,ns])).items;assert.equal(survivors.length,1);assert.ok(['one','two'].includes(survivors[0].key));assert.deepEqual(survivors[0].value,{content:'duplicate-needle中文🌊'});assert.deepEqual((await invoke(a,'memory.resource.list_namespaces',[store,['sdk']])).namespaces,[ns]);
 await assert.rejects(invoke(b,'memory.resource.get',[store,ns,'one']),error=>{rows.push({negative:'foreign-store-owner',code:error.code,operation:error.operation,handle:error.handle??null,details:error.details??null,retryable:error.retryable});return true;});
 async function register(owner){return sdk.registerExtension('store',owner,{kind:'store',descriptor_version:1,search_modes:['keyword']},async call=>{
  const operation=call.invocation.operation,input=structuredClone(call.invocation.input);if(operation==='store_put')input.value=fromWireValue(input.value);
  const outcome=await http(url,'/store',{authority:owner,operation,input});if(outcome.outcome!=='result')return outcome;
  const item=raw=>{raw.value=toWireValue(raw.value);for(const field of ['created_at','updated_at','last_accessed','expires_at'])if(raw[field]!==null)raw[field]=String(raw[field]);return raw;};
  let value=outcome.result.value;if(operation==='store_get'&&value!==null)value=item(value);else if(['store_search','store_search_with','store_list'].includes(operation))value=value.map(item);return {outcome:'result',result:{operation,value}};
 });}
 const ra=await register('A');await sdk.call('echo_agent::agent::react::ReactAgent::set_store',agent.wire,[a.wire,ra.wire]);const raw=JSON.parse('{"中文":"🌊","__proto__":{"id":"domain","generation":"literal"}}');await invoke(a,'memory.store.put',[ns,'key',raw]);assert.deepEqual((await invoke(a,'memory.store.get',[ns,'key'])).value,raw);
 await ra.close();const rb=await register('B');await sdk.call('echo_agent::agent::react::ReactAgent::set_store',agent.wire,[a.wire,rb.wire]);assert.equal(await invoke(a,'memory.store.get',[ns,'key']),null);await invoke(a,'memory.store.put',[ns,'key','B-new']);assert.equal((await invoke(a,'memory.store.search',[ns,'B-new',5])).items.length,1);assert.equal((await invoke(a,'memory.store.delete',[ns,'key'])).deleted,true);assert.equal((await invoke(a,'memory.store.delete',[ns,'key'])).deleted,false);
 await http(url,'/fault',{operation:'store_put'});await assert.rejects(invoke(a,'memory.store.put',[ns,'fault','never']),error=>{rows.push({negative:'reverse-store-prewrite-fault',code:error.code,operation:error.operation,handle:error.handle??null,details:error.details??null,retryable:error.retryable});return true;});assert.equal(await invoke(a,'memory.store.get',[ns,'fault']),null);await invoke(a,'memory.store.put',[ns,'fault','retrycommitted']);assert.equal((await invoke(a,'memory.store.get',[ns,'fault'])).value,'retrycommitted');
 await rb.close();const c=await agent.createSession(work);const corpus=JSON.parse(await readFile(join(work,'scenarios.json'))).scenarios;
 for(const step of corpus[2].steps.slice(0,3)){const args=structuredClone(step.args);args[2].meta.confidence={kind:'f64',value:1};await invoke(c,step.operation,args);}const original=await invoke(c,'memory.store.get',[['agent','memories'],'approved-active']);const run=await c.startRun({text:'SDK-Recall'});const settlement=await run.wait({seconds:'10',nanos:0});assert.equal(settlement.settled,true);
 const inspection=await http(url,'/inspect');const serialized=JSON.stringify(inspection.model_requests.at(-1));assert.ok(serialized.includes('SDK-Recall-Active 中文🌊'));assert.ok(!serialized.includes('SDK-Recall-Draft')&&!serialized.includes('SDK-Recall-Legacy'));const latest=await invoke(c,'memory.store.get',[['agent','memories'],'approved-active']);assert.equal(latest.value.content,original.value.content);assert.deepEqual(latest.value.meta.provenance,original.value.meta.provenance);const journal=await run.replay('0',512);const envelopes=journal.events.map(item=>item.envelope);const memoryEvents=envelopes.filter(item=>item.payload.event_type==='memory_recalled');assert.equal(memoryEvents.length,1);assert.deepEqual(fromWireValue(memoryEvents[0].payload.data),{count:'1'});assert.deepEqual(envelopes.map(item=>Number(item.sequence)),Array.from({length:envelopes.length},(_,index)=>index+1));assert.equal(new Set(envelopes.map(item=>item.event_id)).size,envelopes.length);assert.equal(envelopes.filter(item=>item.payload.event_type==='final_answer').length,1);rows.push({actual_model_recall:true,model_request:inspection.model_requests.at(-1),settled:settlement,journal});
 const rule=corpus[3].steps[2].args[0];await invoke(a,'permission.clear_rules');await invoke(a,'permission.clear_cache');await invoke(a,'permission.add_rule',[rule]);assert.deepEqual(await invoke(a,'permission.all_rules'),[rule]);assert.deepEqual(await invoke(b,'permission.all_rules'),[]);
 const context=structuredClone(corpus[3].steps[4].args.at(-1));context.session_id=a.acpSessionId;context.permission.working_directory=work;context.permission.affected_files=[join(work,'effect.txt')];let check=await invoke(a,'permission.check_with_permissions_result_in_mode_and_context',['candidate_write',{path:join(work,'effect.txt')},['write'],'default',context]);assert.equal(check.decision,'deny');assert.equal(await invoke(a,'permission.remove_rule',[rule]),true);assert.equal(await invoke(a,'permission.remove_rule',[rule]),false);
 const scope='same-scope';await invoke(a,'permission.record_approval',[scope,'candidate_write',{x:1},'session']);assert.equal(await invoke(a,'permission.is_approved',[scope,'candidate_write',{x:1}]),true);assert.equal(await invoke(a,'permission.is_approved',[scope,'candidate_write',{x:2}]),false);assert.equal(await invoke(b,'permission.is_approved',[scope,'candidate_write',{x:1}]),false);await invoke(b,'permission.record_approval',[scope,'candidate_write',{x:1},'session']);await invoke(a,'permission.revoke_cache',[scope,'candidate_write']);assert.equal(await invoke(a,'permission.is_approved',[scope,'candidate_write',{x:1}]),false);assert.equal(await invoke(b,'permission.is_approved',[scope,'candidate_write',{x:1}]),true);await invoke(a,'permission.record_approval',[scope,'candidate_write',{x:1},'once']);assert.equal(await invoke(a,'permission.is_approved',[scope,'candidate_write',{x:1}]),false);await invoke(a,'permission.record_approval',[scope,'candidate_write',{x:1},'session_tool']);assert.equal(await invoke(a,'permission.is_approved',[scope,'candidate_write',{x:2}]),true);await invoke(a,'permission.revoke_cache',[scope,'candidate_write']);const stats=await invoke(a,'permission.stats');rows.push({actual_ttl_ms:stats.ttl_ms,ttl_expiration_proof:false});assert.equal(await invoke(a,'permission.cleanup_expired'),'0');await invoke(a,'permission.clear_cache');
 check=await invoke(a,'permission.check_with_permissions_result_in_mode_and_context',['candidate_write',{path:join(work,'context.txt')},['write'],'strict',context]);assert.equal(check.decision,'allow');assert.deepEqual(check.updated_input,{path:join(work,'modified.txt'),中文:'🌊'});const humanRequest=humanCalls.at(-1).invocation.input;assert.equal(humanRequest.session_id,a.acpSessionId);assert.equal(humanRequest.request_id,context.request_id);assert.equal(humanRequest.agent_name,'memory-fixture');assert.deepEqual(humanRequest.timeout,{seconds:'1',nanos:0});assert.deepEqual(fromWireValue(humanRequest.args),{path:join(work,'context.txt')});assert.deepEqual(fromWireValue(humanRequest.approval_context),context.permission);rows.push({permission_reverse_context:humanCalls.at(-1),returned_check:check,approval_receipt_present:'approval_receipt'in check});await a.close();assert.equal(await invoke(b,'permission.is_approved',[scope,'candidate_write',{x:1}]),true);
 const toolCalls=[];
 const tool=await sdk.registerExtension('tool','candidate-write',{kind:'tool',descriptor_version:1,name:'candidate_write',description:'write one test fixture file',parameters:toWireValue({type:'object',properties:{path:{type:'string'},phase:{type:'string'}},required:['path','phase']}),schema_revision:'1',required_input_modalities:[],required_permissions:['write'],risk_level:'standard',supports_streaming:false,exempt_from_batch_timeout:false,allows_parallel_batch_execution:false,manages_own_timeout:false},async call=>{
  const operation=call.invocation.operation;if(operation==='tool_validate_parameters')return {outcome:'result',result:{operation,value:null}};
  const parameters=fromWireValue(call.invocation.input.parameters);assert.ok(parameters.path.startsWith(work+'/'));await writeFile(parameters.path,'accepted permission effect 中文🌊');toolCalls.push(parameters);return {outcome:'result',result:{operation,value:{kind:{kind:'text'},success:true,output:'written',truncated:false,effects:[{effect:'file_edit',path:parameters.path}]}}};
 });
 const denied=await agent.createSession(work),allowed=await agent.createSession(work);
 const deniedRun=await denied.startRun({text:'permission-effect-deny'});const deniedSettlement=await deniedRun.wait({seconds:'10',nanos:0});assert.equal(deniedSettlement.settled,true);await assert.rejects(readFile(join(work,'effect-deny.txt')),error=>error.code==='ENOENT');assert.equal(toolCalls.length,0);
 const approvedRun=await allowed.startRun({text:'permission-effect-allow'});const approvedSettlement=await approvedRun.wait({seconds:'10',nanos:0});assert.equal(approvedSettlement.settled,true);assert.equal(await readFile(join(work,'effect-allow.txt'),'utf8'),'accepted permission effect 中文🌊');assert.equal(toolCalls.length,1);assert.equal(toolCalls[0].phase,'allow');rows.push({actual_permission_effect:{denied_file_exists:false,denied_tool_calls:0,approved_tool_calls:toolCalls,denial_settlement:deniedSettlement,approval_settlement:approvedSettlement},human_requests:humanCalls.slice(-2)});rows.push({backend_inspection:await http(url,'/inspect')});await denied.close();await allowed.close();await tool.close();
 await b.close();await c.close();await agent.close();await rb.close();await human.close();
 await writeFile(join(work,'typescript-real-evidence.json'),JSON.stringify({host,host_sha256:fileSha256(host),work,results:rows,limitations:['TTL expiration not observed','approval receipt omitted by existing adapter']},null,2));console.log('TS actual memory/permission evidence rows',rows.length,'work',work);
}finally{if(sdk)await sdk.close();fixture.kill();await new Promise(resolve=>fixture.once('exit',resolve));}

});
