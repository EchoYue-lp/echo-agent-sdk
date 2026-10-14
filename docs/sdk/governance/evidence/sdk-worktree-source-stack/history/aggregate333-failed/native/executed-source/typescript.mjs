// A finite diagnostic consumer: published a634 + private heap partition.
// This does not accept the still-broken branch switch or the final SDK snapshot.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const sdkRoot=resolve(process.env.ECHO_AGENT_SDK_SOURCE_ROOT??process.cwd());
const {EchoAgentClient,EchoAgentError}=await import(pathToFileURL(join(sdkRoot,'sdks/typescript/dist/index.js')));
const fixture=join(sdkRoot,'sdks/shared/testing/git_tool_host_fixture.py');
const host=process.env.ECHO_AGENT_SDK_HOST,root=process.env.ECHO_AGENT_SDK_GIT_ROOT;
assert.ok(host&&root,'explicit owned Host and work root required');mkdirSync(root,{recursive:true});
const work=mkdtempSync(join(root,'typescript-')),rows=[],values={};let sdk;
const oracle=(...args)=>execFileSync('python3',[fixture,'--work',work,...args],{encoding:'utf8'});
const resolveArgs=v=>Array.isArray(v)?v.map(resolveArgs):v&&typeof v==='object'?(v.$ref?values[v.$ref]:Object.fromEntries(Object.entries(v).map(([k,x])=>[k,resolveArgs(x)]))):v;
const write=()=>writeFileSync(join(work,'results.json'),JSON.stringify({scope:'a634-private-heap-partition-source-only-diagnostic',host_sha256:createHash('sha256').update(readFileSync(host)).digest('hex'),work,rows},null,2));
try{
 oracle('--prepare');oracle('--checkpoint','prepare-managed');const scenario=JSON.parse(readFileSync(join(work,'scenario.json'),'utf8'));
 sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',join(work,'host.json')],env:scenario.host_env,onStderr:text=>writeFileSync(join(work,'host.stderr'),text,{flag:'a'})});const agent=await sdk.createAgent(),session=await agent.createSession(scenario.repo);
 const steps=scenario.rows.filter(step=>step.mode==='source');const list=steps.find(step=>step.id==='source-list');steps.splice(3,0,{...list,id:'alive-after-refusal'});
 for(const step of steps){
  if(step.checkpoint)oracle('--checkpoint',step.checkpoint);const row={id:step.id,operation:step.operation,arguments:resolveArgs(step.arguments),before:JSON.parse(oracle('--snapshot'))};
  try{row.result=await session.invoke(step.operation,row.arguments);values[step.id]=row.result;}catch(error){assert.ok(error instanceof EchoAgentError);row.error={code:error.code,message:error.message,operation:error.operation,retryable:error.retryable,handle:error.handle??null,details:error.details??null};}
  row.after=JSON.parse(oracle('--snapshot'));rows.push(row);write();
  if(step.error){assert.deepEqual(row.error,{code:'framework_error',message:'Refusing to remove a worktree with uncommitted changes',operation:step.operation,retryable:'never',handle:null,details:null});assert.deepEqual(row.before,row.after);}
  else assert.ok(!row.error,JSON.stringify(row.error));
  if(step.id==='alive-after-refusal'){assert.deepEqual(row.result,values['source-list']);assert.deepEqual(row.before,row.after);}
  if(step.id==='source-merge'){assert.equal(row.after.repo.head,row.before.source.head);assert.equal(row.after.repo.branch,'main');assert.equal(row.after.repo.files['tracked.txt'],row.before.source.files['tracked.txt']);assert.equal(row.after.repo.merge_active,false);}
  if(step.id==='source-remove'){assert.equal(row.result,null);assert.equal(row.after.source.exists,false);assert.ok(row.after.repo.refs.includes('refs/heads/source-proof'));}
  if(step.id==='source-after-list'){assert.equal(row.result.length,1);assert.deepEqual(row.before,row.after);}
 }
 await session.close();await agent.close();writeFileSync(join(work,'verified.json'),JSON.stringify({scope:'finite-source-only-diagnostic',native_calls:rows.length,operations:[...new Set(rows.map(row=>row.operation))],framework:'a6346643bda47679a96760f56f45e41fd73b61b0',final_sdk_acceptance:false},null,2));console.log(work);
}finally{write();await sdk?.close();}
