import { fileSha256 } from "../../shared/testing/file-sha256.mjs";
import test from 'node:test';
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {EchoAgentClient,EchoAgentError} from '../dist/index.js';
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==='1'&&!host)throw Error('required Git Host missing');
const fixture=fileURLToPath(new URL('../../shared/testing/git_tool_host_fixture.py',import.meta.url));
function resolved(value,results){if(Array.isArray(value))return value.map(v=>resolved(v,results));if(value&&typeof value==='object'){if(value.$ref)return results[value.$ref];return Object.fromEntries(Object.entries(value).map(([key,v])=>[key,resolved(v,results)]));}return value;}
test('accepted Git tools and worktree source operations preserve exact local effects',{skip:!host,timeout:120000},async()=>{
 const root=resolve(process.env.ECHO_AGENT_SDK_GIT_ROOT??'../../target/git-native');mkdirSync(root,{recursive:true});const work=mkdtempSync(join(root,'typescript-'));
 const oracle=(...args)=>execFileSync('python3',[fixture,'--work',work,...args],{encoding:'utf8'});
 oracle('--prepare');const scenario=JSON.parse(readFileSync(join(work,'scenario.json'),'utf8')),rows=[],results={};let sdk;
 try{
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')],env:scenario.host_env,onStderr:text=>writeFileSync(join(work,'host.stderr'),text,{flag:'a'})});const agent=await sdk.createAgent(),session=await agent.createSession(scenario.repo);
  for(const step of scenario.rows){
   if(step.checkpoint)oracle('--checkpoint',step.checkpoint);
   const row={id:step.id,operation:step.operation,arguments:resolved(step.arguments,results),before:JSON.parse(oracle('--snapshot'))};
   try{row.result=await session.invoke(step.operation,row.arguments);results[step.id]=row.result;}catch(error){assert.ok(error instanceof EchoAgentError,String(error));row.error={code:error.code,message:error.message,operation:error.operation,retryable:error.retryable,handle:error.handle??null,details:error.details??null};}
   row.after=JSON.parse(oracle('--snapshot'));rows.push(row);writeFileSync(join(work,'results.json'),JSON.stringify({work,host,host_sha256:fileSha256(host),rows},(_,v)=>typeof v==='bigint'?v.toString():v,2));
  }
  await session.close();await agent.close();const proof=JSON.parse(oracle('--verify'));assert.equal(proof.tool_operations.length,9);assert.equal(proof.source_operations.length,4);assert.equal(proof.groups.length,3);console.log('Git evidence',work);
 }finally{await sdk?.close();console.log('Git evidence retained',work);}
});
