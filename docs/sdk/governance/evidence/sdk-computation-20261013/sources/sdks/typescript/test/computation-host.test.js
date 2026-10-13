import { mkdtemp,readFile,writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
import { EchoAgentClient } from '../dist/client.js';
import { toWireValue,fromWireValue } from '../dist/wire.js';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
const fixture=fileURLToPath(new URL('../../shared/testing/computation_fixture.py',import.meta.url));
const host=process.env.ECHO_AGENT_SDK_HOST?.trim();
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==='1'&&!host)throw new Error('required computation Host binary is missing');
const snapshot=async(work)=>Object.fromEntries(await Promise.all(['stats.csv','input.csv','lookup.csv','quality.csv','input.txt','sentinel.txt','null-index.csv','collision.csv'].map(async(name)=>[name,createHash('sha256').update(await readFile(join(work,name))).digest('hex')])));
test('computation exact values, errors and protected effects', {skip:!host,timeout:120000},async()=>{
const work=await mkdtemp(join(tmpdir(),'sdk-computation-native-typescript-'));
execFileSync('python3',[fixture,'--work',work]);
const rows=[];let sdk;let identity;
function rebase(value){if(typeof value==='string')return value.replaceAll('$WORK',work);if(Array.isArray(value))return value.map(rebase);if(value&&typeof value==='object')return Object.fromEntries(Object.entries(value).map(([k,v])=>[k,rebase(v)]));return value;}
try{
 sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')]});const agent=await sdk.createAgent(),session=await agent.createSession(work);identity={agent:agent.wire,session:session.wire,capability:sdk.capability};
 for(const step of JSON.parse(await readFile(join(work,'scenario.json'))).rows){const args=rebase(step.arguments),row={...step,arguments:args,protected_before:await snapshot(work)};
  try{let result;if(step.operation==='structured_output.validate'){const value=await sdk.request('_echo_agent/structured_output/validate',{operation:step.operation,signature_digest:'sha256:'+'0123456789abcdef'.repeat(4),handle:session.wire,arguments:args.map(toWireValue)});result=fromWireValue(value.value);}else result=await session.invoke(step.operation,args);row.result=result;assert.equal(step.expected_error,null,step.id);}
  catch(error){row.error={code:error.code,message:error.message,operation:error.operation,retryable:error.retryable,handle:error.handle??null,details:error.details??null};assert.equal(error.code,step.expected_error,JSON.stringify(row));}
  row.protected_after=await snapshot(work);rows.push(row);
 }
 await session.close();await agent.close();
}finally{if(sdk)await sdk.close();}
await writeFile(join(work,'results.json'),JSON.stringify({host,host_sha256:createHash('sha256').update(await readFile(host)).digest('hex'),work,rows,raw_identity:identity,actual_exports:{output_text:await readFile(join(work,'output.txt'),'utf8'),output_json:JSON.parse(await readFile(join(work,'output.json'),'utf8'))},actual_side_effects:{sentinel_contents:await readFile(join(work,'sentinel.txt'),'utf8'),sentinel_sha256:createHash('sha256').update(await readFile(join(work,'sentinel.txt'))).digest('hex')}},(_,value)=>typeof value==='bigint'?value.toString():value,2));
console.log('TS native public corpus',rows.length,'work',work);

const summary=execFileSync('python3',[fixture,'--work',work,'--verify',join(work,'results.json')],{encoding:'utf8'});console.log('Computation verified:',summary.trim(),'evidence:',work);
});
