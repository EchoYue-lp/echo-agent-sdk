import { fileSha256 } from "../../shared/testing/file-sha256.mjs";
import test from 'node:test';
import assert from 'node:assert/strict';
import {spawn,execFileSync} from 'node:child_process';
import {mkdtempSync,mkdirSync,existsSync,readFileSync,writeFileSync,renameSync} from 'node:fs';
import {join,dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {EchoAgentClient,EchoAgentError,toWireValue,fromWireValue} from '../dist/index.js';
const host=process.env.ECHO_AGENT_SDK_HOST?.trim();
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==='1'&&!host)throw new Error('required Tool value/effect Host missing');
const fixture=fileURLToPath(new URL('../../shared/testing/tool_value_effect_fixture.py',import.meta.url));
const root=process.env.ECHO_AGENT_SDK_TOOL_EFFECT_ROOT??join(dirname(fixture),'../../../target/tool-value-effects');
const unrecord=value=>value?.kind==='record'?Object.fromEntries(value.value.fields.map(f=>[f.name,f.value.value??null])):value;
function resolve(value,results){if(Array.isArray(value))return value.map(v=>resolve(v,results));if(value&&typeof value==='object'){if(value.$artifact)return results[value.$artifact].result;const key=value.$ref?'$ref':value.$jsonref?'$jsonref':null;if(key){const [name,...path]=value[key].split('.');let v=results[name].result;if(key==='$jsonref')v=decode(v.data);v=unrecord(v);for(const p of path)v=v[p];return v;}return Object.fromEntries(Object.entries(value).map(([k,v])=>[k,resolve(v,results)]));}return value;}
function decode(v){if(v.kind==='null')return null;if(v.kind==='list')return v.value.map(decode);if(v.kind==='map')return Object.fromEntries(v.value.map(e=>[decode(e.key),decode(e.value)]));return v.value;}
const sha=file=>createHash('sha256').update(readFileSync(file)).digest('hex');
const snapshot=work=>Object.fromEntries(['sentinel.txt','fixture.db','src/main.py'].map(n=>[n,sha(join(work,n))]));
const files=work=>Object.fromEntries(['中文🙂/note.txt','中文🙂/moved.txt'].map(n=>[n,existsSync(join(work,n))?readFileSync(join(work,n),'utf8'):null]));
const processes=work=>execFileSync('ps',['-axo','pid=,ppid=,command='],{encoding:'utf8'}).split('\n').map(line=>line.trim().match(/^(\d+)\s+(\d+)\s+(.*)$/)).filter(m=>m&&m[3].startsWith('tail -f '+join(work,'tail.txt'))).map(m=>({pid:Number(m[1]),ppid:Number(m[2]),command:m[3]}));
function artifactPath(value){const path=unrecord(value).path;if(path.encoding==='utf8')return path.path;if(path.encoding==='unix')return Buffer.from(path.bytes_base64,'base64').toString('utf8');return Buffer.from(path.utf16_base64,'base64').toString('utf16le');}
test('accepted tool exact values, errors and actual effects', {skip:!host,timeout:240000}, async()=>{
 mkdirSync(root,{recursive:true});const work=mkdtempSync(join(root,'typescript-'));const fixtureProcess=spawn('python3',[fixture,'--work',work],{stdio:['ignore','ignore','inherit']});let sdk;const rows=[],results={};
 try{
  const deadline=Date.now()+10000;while(!existsSync(join(work,'ready'))){assert.ok(Date.now()<deadline&&fixtureProcess.exitCode===null,'fixture unavailable');await new Promise(r=>setTimeout(r,10));}
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')],env:{...process.env,TMPDIR:join(work,'artifact-temp')}});const agent=await sdk.createAgent(),session=await agent.createSession(work);const identity={agent:agent.wire,session:session.wire,capability:sdk.capability};
  for(const step of JSON.parse(readFileSync(join(work,'scenario.json'),'utf8')).rows){const row={...step,protected_before:snapshot(work),files_before:files(work)};
   if(step.action==='mutate_artifact')writeFileSync(artifactPath(results[step.ref].result),'mutated 中文🙂\n');
   else if(step.action==='rename_fixture_directory'){
    assert.equal(step.source,'fs-owner');assert.equal(step.target,'fs-owner-backup');
    const source=join(work,step.source),target=join(work,step.target);
    row.stimulus_before={source:existsSync(source),target:existsSync(target)};
    renameSync(source,target);row.stimulus_after={source:existsSync(source),target:existsSync(target)};
   }
   else{const arguments_=resolve(step.arguments,results);row.arguments=arguments_;try{
    if(step.watch_process){let settled=false;const call=session.invoke(step.operation,arguments_).finally(()=>{settled=true;});let pids=[];for(let n=0;n<50;n++){pids=processes(work);if(pids.length||settled)break;await new Promise(r=>setTimeout(r,10));}row.process_before=pids;if(step.terminate_observed){assert.equal(pids.length,1);assert.deepEqual(processes(work),pids,'own tail identity changed before signal');process.kill(pids[0].pid,'SIGTERM');}row.result=await call;row.process_after=processes(work);}
    else row.result=await session.invoke(step.operation,arguments_);
   }catch(error){assert.ok(error instanceof EchoAgentError,String(error));row.error={code:error.code,message:error.message,operation:error.operation,retryable:error.retryable,handle:error.handle??null,details:error.details??null};}}
   row.protected_after=snapshot(work);row.files_after=files(work);rows.push(row);results[row.id]=row;
  }
  await session.close();await agent.close();const data={work,host,host_sha256:fileSha256(host),raw_identity:identity,rows};writeFileSync(join(work,'results.json'),JSON.stringify(data,(_,v)=>typeof v==='bigint'?v.toString():v,2));
  const summary=execFileSync('python3',[fixture,'--work',work,'--verify',join(work,'results.json')],{encoding:'utf8'});process.stdout.write('Tool value/effects: '+summary.trim()+' evidence: '+work+'\n');
 }finally{if(rows.length&&!existsSync(join(work,'results.json')))writeFileSync(join(work,'partial-results.json'),JSON.stringify(rows,(_,v)=>typeof v==='bigint'?v.toString():v,2));await sdk?.close();fixtureProcess.kill();await new Promise(r=>fixtureProcess.exitCode!==null||fixtureProcess.signalCode!==null?r():fixtureProcess.once('exit',r));}
});

test('SearchProvider exact title/url/snippet and real Agent tool failure',{skip:!host,timeout:45000},async()=>{
 mkdirSync(root,{recursive:true});const work=mkdtempSync(join(root,'typescript-search-'));const fixtureProcess=spawn('python3',[fixture,'--work',work],{stdio:['ignore','ignore','inherit']});let sdk;const calls=[],rows=[];
 try{const deadline=Date.now()+10000;while(!existsSync(join(work,'ready'))){assert.ok(Date.now()<deadline&&fixtureProcess.exitCode===null);await new Promise(r=>setTimeout(r,10));}
 sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});
 const registration=await sdk.registerAgentComponent('tool-search-proof',{kind:'agent_component',descriptor_version:1,component:'search_provider',name:'controlled-search'},async(call,signal)=>{
  const request=call.invocation.input.call;const fact={operation:call.invocation.operation,request:{operation:request.operation,...request.input},invocation_id:call.invocation_id,deadline:call.deadline,cancelled:signal.aborted};calls.push(fact);
  if(request.input.query.startsWith('failure'))throw new Error('provider failed 中文🙂');const sent=[{title:'标题中文🙂',url:'https://example.invalid/paper',snippet:'摘要 β🙂'},{title:'Second',url:'https://example.invalid/other',snippet:''}];fact.sent=sent;
  return {operation:'agent_component_call',value:{component:'search_provider',result:{operation:'search_provider_search',value:{results:sent.map(toWireValue)}}}};
 });
 const agent=await sdk.createAgent(),session=await agent.createSession(work);await session.invoke('echo_agent::agent::react::ReactAgent::set_permission_mode',['full-auto']);
 for(const prompt of ['search-success','search-error']){const before=snapshot(work),run=await session.startRun({text:prompt});const settled=await run.wait({timeoutMs:15000}),replay=await run.replay('0',512);rows.push({prompt,settled,replay,protected_before:before,protected_after:snapshot(work)});await run.close();}
 await session.close();await agent.close();await registration.close();writeFileSync(join(work,'search-results.json'),JSON.stringify({work,host,host_sha256:fileSha256(host),calls,rows},(_,v)=>typeof v==='bigint'?v.toString():v,2));
 const summary=execFileSync('python3',[fixture,'--work',work,'--kind','search','--verify',join(work,'search-results.json')],{encoding:'utf8'});console.log('SearchProvider verified:',summary.trim(),'evidence:',work);
 }finally{if(sdk)await sdk.close();fixtureProcess.kill();await new Promise(r=>fixtureProcess.exitCode!==null||fixtureProcess.signalCode!==null?r():fixtureProcess.once('exit',r));}
});

test('Sandbox stream real process completion, pending cancel and repeated close',{skip:!host,timeout:45000},async()=>{
 mkdirSync(root,{recursive:true});const work=mkdtempSync(join(root,'typescript-sandbox-')),fixtureProcess=spawn('python3',[fixture,'--work',work],{stdio:['ignore','ignore','inherit']});let sdk;const facts=[];
 try{const deadline=Date.now()+10000;while(!existsSync(join(work,'ready'))){assert.ok(Date.now()<deadline&&fixtureProcess.exitCode===null);await new Promise(r=>setTimeout(r,10));}sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});
 for(const mode of ['success','cancel','close']){const fact={mode,started:false,cancel_observed:false,finally:0,pid:null,exit_code:null,sent:[]};facts.push(fact);let startedResolve,settledResolve;const started=new Promise(r=>{startedResolve=r;}),settled=new Promise(r=>{settledResolve=r;});
 const registration=await sdk.registerExtension('agent_component','sandbox-'+mode,{kind:'agent_component',descriptor_version:1,component:'sandbox_executor',name:'native-sandbox',capabilities:{isolation_level:'process',supports_streaming:true}},async(payload,signal)=>{
  const request=payload.invocation.input.call;if(request.operation==='sandbox_cleanup'){assert.equal(fact.finally,1);assert.equal(fact.pid_alive,false);fact.cleanup_calls=(fact.cleanup_calls??0)+1;return {outcome:'result',result:{operation:'agent_component_call',value:{component:'sandbox_executor',result:{operation:'sandbox_cleanup'}}}};}if(request.operation==='sandbox_is_available')return {outcome:'result',result:{operation:'agent_component_call',value:{component:'sandbox_executor',result:{operation:'sandbox_is_available',value:{available:true}}}}};assert.equal(request.operation,'sandbox_execute_stream');const command=fromWireValue(request.input.command);fact.command=command;fact.invocation_id=payload.invocation_id;const writer=sdk.streamWriter(payload.stream);let child;
  try{const argv=mode==='success'?['printf','sandbox 中文🙂\n']:['tail','-f',join(work,'tail.txt')];assert.equal(command.kind.Shell,mode==='success'?"printf 'sandbox 中文🙂\\n'":'tail -f '+join(work,'tail.txt'));child=spawn(argv[0],argv.slice(1),{stdio:['ignore','pipe','pipe']});fact.pid=child.pid;fact.started=true;startedResolve();let stdout=Buffer.alloc(0),stderr=Buffer.alloc(0);child.stdout.on('data',b=>{stdout=Buffer.concat([stdout,b]);});child.stderr.on('data',b=>{stderr=Buffer.concat([stderr,b]);});const exited=new Promise((resolve,reject)=>{child.once('error',reject);child.once('exit',(code,signal)=>{fact.exit_code=code??(signal==='SIGTERM'?-15:-1);fact.exit_signal=signal;resolve();});});
   if(mode==='success'){await exited;for(const [channel,data] of [['stdout',stdout],['stderr',stderr]])if(data.length){const chunk=data.toString('utf8');fact.sent.push({channel,chunk});await writer.agentComponentChunk({component:'sandbox',event:{event:'output',channel,chunk}});}
    const result={exit_code:fact.exit_code,stdout:stdout.toString('utf8'),stderr:stderr.toString('utf8'),duration:{secs:0,nanos:1},sandbox_type:'owned-native-fixture',timed_out:false,cancelled:false,output_truncated:false,stdout_bytes:stdout.length,stderr_bytes:stderr.length};await writer.agentComponentComplete({component:'sandbox',terminal:{terminal:'complete',result:toWireValue(result)}});return {outcome:'stream',stream:payload.stream};}
   await new Promise((resolve,reject)=>{const t=setTimeout(()=>reject(new Error('callback cancellation not observed')),5000);if(signal.aborted){clearTimeout(t);resolve();}else signal.addEventListener('abort',()=>{clearTimeout(t);resolve();},{once:true});});fact.cancel_observed=signal.aborted;child.kill('SIGTERM');await exited;return {outcome:'cancelled'};
  }finally{if(child&&child.exitCode===null&&child.signalCode===null){child.kill('SIGTERM');await new Promise(r=>child.once('exit',r));}fact.finally++;settledResolve();}
 });
 const agent=await sdk.createAgent(),session=await agent.createSession(work);const command={kind:{Shell:mode==='success'?"printf 'sandbox 中文🙂\\n'":'tail -f '+join(work,'tail.txt')},minimum_isolation:null,working_dir:null,env:{},timeout:{secs:30,nanos:0},stdin:null};const stream=await session.invoke('sandbox.extension.run_stream',[registration.wire,command]);fact.stream=stream;await started;
 if(mode==='success'){fact.events=[];for(let n=0;n<5;n++){const event=await session.invoke('sandbox.stream.next',[stream]);fact.events.push(event);assert.notEqual(event.value.variant,'failed',JSON.stringify(event));if(event.value.variant==='complete'){fact.terminal_returned_after_cleanup=fact.finally===1;break;}}}
 else {const before=performance.now();fact.close_result=await session.invoke(mode==='cancel'?'sandbox.stream.cancel':'sandbox.stream.close',[stream]);fact.control_elapsed_ms=performance.now()-before;fact.control_returned_after_cleanup=fact.finally===1;}await settled;try{process.kill(fact.pid,0);fact.pid_alive=true;}catch(error){if(error.code!=='ESRCH')throw error;fact.pid_alive=false;}fact.repeated_close=await session.invoke('sandbox.stream.close',[stream]);fact.protected=snapshot(work);await session.close();await agent.close();await registration.close();
 }
 writeFileSync(join(work,'sandbox-results.json'),JSON.stringify({work,host,host_sha256:fileSha256(host),facts},(_,v)=>typeof v==='bigint'?v.toString():v,2));const summary=execFileSync('python3',[fixture,'--work',work,'--kind','sandbox','--verify',join(work,'sandbox-results.json')],{encoding:'utf8'});console.log('Sandbox verified:',summary.trim(),'evidence:',work);
 }finally{writeFileSync(join(work,'sandbox-partial.json'),JSON.stringify(facts,(_,v)=>typeof v==='bigint'?v.toString():v,2));if(sdk)await sdk.close();fixtureProcess.kill();await new Promise(r=>fixtureProcess.exitCode!==null||fixtureProcess.signalCode!==null?r():fixtureProcess.once('exit',r));}
});
