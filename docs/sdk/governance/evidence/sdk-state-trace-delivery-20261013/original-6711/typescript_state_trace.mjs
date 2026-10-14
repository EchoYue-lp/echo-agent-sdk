import assert from 'node:assert/strict';
import {readFileSync,writeFileSync,existsSync,readdirSync,renameSync,unlinkSync,mkdirSync} from 'node:fs';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
import {EchoAgentClient} from '/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/typescript/dist/index.js';
import {toWireValue} from '/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/typescript/dist/wire.js';
const [host,work,scene]=process.argv.slice(2);
// Preserve the shared fixture's known u64 timing before JSON.parse can round
// it; outgoing values use the existing explicit u64 WireValue constructor.
const f=JSON.parse(readFileSync(join(work,'fixture.json'),'utf8').replaceAll('9007199254740993','"9007199254740993"'));
const canon=v=>v===null||typeof v==='boolean'?v:typeof v==='number'?String(v):Array.isArray(v)?v.map(canon):typeof v==='object'?Object.fromEntries(Object.entries(v).map(([k,x])=>[k,canon(x)])):v;
async function start(){const sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});const agent=await sdk.createAgent();return {sdk,session:await agent.createSession()};}
async function err(p){try{await p;throw Error('failure expected');}catch(e){assert.equal(e.code,'framework_error');assert.ok(e.operation);return {code:e.code,operation:e.operation,message:e.message};}}
const fault=a=>execFileSync('python3',['/tmp/sdk-state-trace-delivery-candidate-fixture/prepare_fixture.py','--work',work,a]);
let setup=await start();let {sdk,session}=setup;const proof={errors:[]};
try{
 if(scene==='state'){
  for(const label of ['a','b']){const cp=f['checkpoint_'+label];assert.deepEqual(await session.invoke('state.checkpoint.save',[f['scope_'+label],cp]),{ok:true});assert.deepEqual(canon((await session.invoke('state.checkpoint.get',[cp.conversation_id])).checkpoint),canon(cp));assert.deepEqual((await session.invoke('state.runtime.list',[f['scope_'+label]])).runtime_state_ids,[cp.conversation_id]);}
  const dir=join(work,'state/runtime_state/_runtime_owners');const files=readdirSync(dir).filter(x=>x.endsWith('.json'));const before=Object.fromEntries(files.map(x=>[x,readFileSync(join(dir,x),'utf8')]));
  fault('block-owners');try{proof.errors.push(await err(session.invoke('state.checkpoint.save',[f.scope_a,f.checkpoint_a])));}finally{fault('unblock');}
  assert.deepEqual(Object.fromEntries(files.map(x=>[x,readFileSync(join(dir,x),'utf8')])),before);
  assert.deepEqual(await session.invoke('state.checkpoint.save',[f.scope_a,f.checkpoint_a]),{ok:true});
  const owner=join(dir,files.find(x=>JSON.parse(readFileSync(join(dir,x),'utf8')).runtime_state_id===f.checkpoint_a.conversation_id));const raw=readFileSync(owner);writeFileSync(owner,'{corrupt-owner');try{proof.errors.push(await err(session.invoke('state.checkpoint.get',[f.checkpoint_a.conversation_id])));}finally{writeFileSync(owner,raw);}
  const cleared=await session.invoke('state.scope.clear',[f.scope_a]);assert.equal(cleared.scope_id,f.scope_a);assert.deepEqual(cleared.runtime_state_ids,[f.checkpoint_a.conversation_id]);assert.equal((await session.invoke('state.checkpoint.get',[f.checkpoint_a.conversation_id])).checkpoint,null);assert.deepEqual(canon((await session.invoke('state.checkpoint.get',[f.checkpoint_b.conversation_id])).checkpoint),canon(f.checkpoint_b));
  const idx=join(work,'state/runtime_state/_scope_index');for(const name of readdirSync(idx)){const p=join(idx,name);if(name.endsWith('.json')&&JSON.parse(readFileSync(p,'utf8')).scope_id===f.scope_b)unlinkSync(p);}
  assert.deepEqual((await session.invoke('state.runtime.list',[f.scope_b])).runtime_state_ids,[f.checkpoint_b.conversation_id]);
 }else{
  const blocked=join(work,'trace-open-parent');writeFileSync(blocked,'directoryblock');proof.errors.push(await err(session.invoke('trace.store.open',['jsonl',join(blocked,'child')])));
  const store=(await session.invoke('trace.store.open',['jsonl',f.trace_dir])).resource;
  const run={...f.trace,timings:{...f.trace.timings,total_duration_ms:{kind:'u64',value:'9007199254740993'}}};
  assert.deepEqual(await session.invoke('trace.run.save',[store,run]),{ok:true});const loaded=(await session.invoke('trace.run.load',[store,f.trace.run_id])).run;assert.deepEqual(canon(loaded),canon(f.trace_expected));
  const path=join(f.trace_dir,readdirSync(f.trace_dir).find(x=>x.endsWith('.jsonl')));const disk=readFileSync(path);assert.ok(!disk.toString().includes('sk-abcdefghijklmnopqrstuvwxyz123456'));const durable=JSON.parse(disk.toString().split('\n')[0].replaceAll('9007199254740993','"9007199254740993"'));assert.deepEqual(canon(durable),canon(f.trace_expected));
  fault('block-trace');try{proof.errors.push(await err(session.invoke('trace.run.save',[store,run])));assert.deepEqual(canon((await session.invoke('trace.run.load',[store,f.trace.run_id])).run),canon(loaded));}finally{fault('unblock');}
  assert.deepEqual(readFileSync(path),disk);assert.deepEqual(await session.invoke('trace.run.save',[store,run]),{ok:true});const summaries=(await session.invoke('trace.run.list_session',[store,f.trace.session_id])).summaries;assert.equal(summaries.length,1);for(const key of ['run_id','parent_run_id','session_id','agent_name','model','provider','turn_id','execution_id','status','started_at','finished_at','token_usage'])assert.deepEqual(canon(summaries[0][key]),canon(f.trace_expected[key]));assert.equal(summaries[0].total_duration_ms,'9007199254740993');proof.loaded=loaded;
 }
}finally{await sdk.close();}
setup=await start();({sdk,session}=setup);try{if(scene==='state'){proof.after_real_host_restart=(await session.invoke('state.checkpoint.get',[f.checkpoint_b.conversation_id])).checkpoint;assert.deepEqual(canon(proof.after_real_host_restart),canon(f.checkpoint_b));}else{const store=(await session.invoke('trace.store.open',['jsonl',f.trace_dir])).resource;proof.after_real_host_restart=(await session.invoke('trace.run.load',[store,f.trace.run_id])).run;assert.deepEqual(canon(proof.after_real_host_restart),canon(f.trace_expected));}}finally{await sdk.close();}
writeFileSync(join(work,scene+'-observed.json'),JSON.stringify(proof,null,2));
