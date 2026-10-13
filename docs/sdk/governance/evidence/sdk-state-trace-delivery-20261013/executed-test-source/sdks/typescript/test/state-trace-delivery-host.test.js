import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync, writeFileSync, readdirSync, unlinkSync, mkdtempSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";
import { execFileSync } from "node:child_process";
import { EchoAgentClient, wireU64 } from "../dist/index.js";

const host = process.env.ECHO_AGENT_SDK_HOST;
if (process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E === "1" && !host) throw new Error("required State/Trace/Delivery Host binary missing");
const fixture = fileURLToPath(new URL("../../shared/testing/state_trace_delivery_fixture.py", import.meta.url));

async function stateTrace(host, work, scene) {
const f=JSON.parse(readFileSync(join(work,'fixture.json'),'utf8').replaceAll('9007199254740993','"9007199254740993"'));
const canon=v=>v===null||typeof v==='boolean'?v:typeof v==='number'?String(v):Array.isArray(v)?v.map(canon):typeof v==='object'?Object.fromEntries(Object.entries(v).map(([k,x])=>[k,canon(x)])):v;
async function start(){const sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});const agent=await sdk.createAgent();return {sdk,session:await agent.createSession()};}
async function err(p){try{await p;throw Error('failure expected');}catch(e){assert.equal(e.code,'framework_error');assert.ok(e.operation);return {code:e.code,operation:e.operation,message:e.message};}}
const fault=a=>execFileSync('python3',[fixture,'--work',work,a]);
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

}

async function delivery(host, work) {
const f=JSON.parse(readFileSync(join(work,'fixture.json'),'utf8'));const sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});const agent=await sdk.createAgent();const session=await agent.createSession();const proof={errors:[],durability:'in-memory same-instance only'};
const enc=c=>{assert.deepEqual(Object.keys(c).sort(),['message_id','route','payload','attempt_id','attempt','claimed_at'].sort());assert.match(c.attempt,/^(0|[1-9][0-9]*)$/u);assert.ok(BigInt(c.attempt)<=4294967295n);return {...c,attempt:wireU64(c.attempt)};};
async function bad(p){await assert.rejects(p,e=>{assert.equal(e.code,'framework_error');assert.ok(e.operation);assert.match(e.message,/stale delivery claim/u);proof.errors.push({code:e.code,operation:e.operation,message:e.message});return true;});}
try{const ledger=(await session.invoke('delivery.ledger.open')).resource;const m=f.delivery;await session.invoke('delivery.enqueue',[ledger,m.message_id,m.route,m.payload]);const both=await Promise.all([session.invoke('delivery.claim_next',[ledger]),session.invoke('delivery.claim_next',[ledger])]);const claims=both.map(x=>x.claim).sort((a,b)=>Number(BigInt(a.attempt)-BigInt(b.attempt)));assert.equal(claims.length,2);assert.deepEqual(claims.map(c=>c.attempt),['1','2']);const [first,latest]=claims;for(const c of claims){assert.equal(c.message_id,m.message_id);assert.equal(c.route,m.route);assert.deepEqual(c.payload,m.payload);assert.ok(c.claimed_at);}assert.notEqual(first.attempt_id,latest.attempt_id);
 const before=await session.invoke('delivery.snapshot',[ledger]);assert.equal(before.records[0].attempt_id,latest.attempt_id);const seq=(await session.invoke('delivery.recover',[ledger])).last_applied_sequence;await bad(session.invoke('delivery.transition',[ledger,enc(first),'effect_started','turn-superseded']));assert.deepEqual(await session.invoke('delivery.snapshot',[ledger]),before);assert.equal((await session.invoke('delivery.recover',[ledger])).last_applied_sequence,seq);
 await session.invoke('delivery.transition',[ledger,enc(latest),'effect_started','turn-before-defer']);const no=await Promise.all([session.invoke('delivery.claim_next',[ledger]),session.invoke('delivery.claim_next',[ledger])]);assert.ok(no.every(x=>x.claim===null));await session.invoke('delivery.defer',[ledger,enc(latest),'延迟重试🙂','2000-01-01T00:00:00Z']);const next=(await session.invoke('delivery.claim_next',[ledger])).claim;assert.equal(next.attempt,'3');assert.notEqual(next.attempt_id,latest.attempt_id);
 const snap=await session.invoke('delivery.snapshot',[ledger]);const cursor=(await session.invoke('delivery.recover',[ledger])).last_applied_sequence;for(const [op,args] of [['delivery.transition',[ledger,enc(first),'effect_started','turn-旧']],['delivery.defer',[ledger,enc(first),'旧请求','2000-01-01T00:00:00Z']],['delivery.settle',[ledger,enc(first),'completed',null,'turn-旧']]]){await bad(session.invoke(op,args));assert.deepEqual(await session.invoke('delivery.snapshot',[ledger]),snap);assert.equal((await session.invoke('delivery.recover',[ledger])).last_applied_sequence,cursor);}
 await session.invoke('delivery.transition',[ledger,enc(next),'effect_started','turn-新']);await session.invoke('delivery.settle',[ledger,enc(next),'completed',null,'turn-新']);const settled=await session.invoke('delivery.snapshot',[ledger]);assert.equal(settled.records.length,1);assert.equal(settled.records[0].outcome,'completed');assert.equal(settled.records[0].attempt_id,next.attempt_id);const recovery=await session.invoke('delivery.recover',[ledger]);assert.deepEqual(await session.invoke('delivery.snapshot',[ledger]),settled);assert.deepEqual(await session.invoke('delivery.recover',[ledger]),recovery);Object.assign(proof,{concurrent_claims:claims,effect_started_no_reclaim:no,claim1:first,claim2:next,snapshot:settled,recovery});
}finally{await sdk.close();}writeFileSync(join(work,'delivery-observed.json'),JSON.stringify(proof,null,2));

}

for (const scene of ["state", "trace", "delivery"]) {
  test(`real Host ${scene} full fields, failure and authority`, { skip: !host, timeout: 60000 }, async () => {
    const work = mkdtempSync(join(tmpdir(), `sdk-typescript-${scene}-`));
    console.log(`State/Trace/Delivery evidence ${scene}: ${work}`);
    execFileSync("python3", [fixture, "--work", work, "prepare"]);
    if (scene === "delivery") await delivery(host, work);
    else await stateTrace(host, work, scene);
    assert.equal(readFileSync(join(work, "state/host/generation"), "utf8").trim(), scene === "delivery" ? "1" : "2");
  });
}
