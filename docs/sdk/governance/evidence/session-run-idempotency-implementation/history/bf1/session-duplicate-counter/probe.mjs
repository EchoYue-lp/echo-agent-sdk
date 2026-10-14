import {EchoAgentClient,RunHandle} from '/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/typescript/dist/index.js';
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
const work=fileURLToPath(new URL('./work/',import.meta.url));await mkdir(work,{recursive:true});await mkdir(work+'changed-cwd',{recursive:true});
const S='/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance';
const host='/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/binaries/echo-sdk-session-run-idempotency-host';
const fixture=S+'/sdks/shared/testing/memory_permission_fixture.py';
const child=spawn('python3',[fixture,'--work',work],{stdio:['ignore','ignore','inherit']});
const facts={host,hostSHA256:createHash('sha256').update(await readFile(host)).digest('hex'),fixture,fixtureSHA256:createHash('sha256').update(await readFile(fixture)).digest('hex'),requests:[],providerCalls:[],cases:[]};let sdk;let registration;let held=false;let release;const sessions=[];const runs=[];
function err(e){return{code:e.code??null,message:e.message,operation:e.operation??null,retryable:e.retryable??null,details:e.details??null};}
async function rpc(method,params){const record={method,params};facts.requests.push(record);try{record.response=await sdk.request(method,params);return record.response;}catch(e){record.error=err(e);throw e;}}
async function observed(method,params){try{return{response:await rpc(method,params)}}catch(e){return{error:err(e)}}}
async function settled(response){const result=await rpc('_echo_agent/run/wait',{run:response.run,timeout:{seconds:'10',nanos:0}});if(!result.settled)throw Error('real Run did not settle');return result;}
async function createSession(params){const r=await rpc('_echo_agent/session/create',params);sessions.push(r.session);return r;}
async function run(params){const r=await rpc('_echo_agent/run/start',params);runs.push(r.run);return r;}
try{
 const deadline=Date.now()+5000;for(;;){try{await readFile(work+'ready');break;}catch(e){if(Date.now()>deadline)throw e;await new Promise(r=>setTimeout(r,10));}}
 sdk=await EchoAgentClient.spawn({hostCommand:host,args:['--config',work+'host.json'],onStderr:x=>process.stderr.write(x)});
 registration=await sdk.registerLlmClient('idempotency-count-model',{kind:'llm_client',descriptor_version:1,model_name:'idempotency-count-model',supports_streaming:false},async(call)=>{
  const index=facts.providerCalls.length+1;facts.providerCalls.push({index,operation:call.invocation.operation,context:call.context,input:call.invocation.input});
  if(held)await new Promise(resolve=>{release=resolve;});
  return{operation:'llm_chat',value:{message:{role:'assistant',content:{kind:'string',value:`canonical-response-${index} 中文🧪`}},finish_reason:'stop',raw:{kind:'map',value:[]}}};
 });
 const agent=await sdk.createAgent();facts.agent=agent.wire;
 const sessionPayload={agent:agent.wire,idempotency_id:'same-session-key'};
 const occupied=await createSession({agent:agent.wire,session_id:'already-registered-session'});
 const keyed={agent:agent.wire,session_id:'already-registered-session',idempotency_id:'duplicate-identity-key'};
 const full=await observed('_echo_agent/session/create',keyed);
 await rpc('_echo_agent/session/close',{session:occupied.session});
 const retry=await observed('_echo_agent/session/create',keyed);
 facts.cases.push({id:'framework-session-admission-duplicate-identity-before-factory',occupied,full,retry});facts.behaviorAccepted=Boolean(full.error)&&Boolean(retry.response);
 facts.sessionIndex=JSON.parse(await readFile(work+'state/host/session-index.json','utf8'));
 facts.exitCode=0;
}catch(e){facts.exitCode=1;facts.fatal=err(e);process.exitCode=1;}
finally{
 held=false;release?.();
 if(sdk){for(const session of sessions)await observed('_echo_agent/session/close',{session});await registration?.close();await sdk.close();}
 child.kill();await new Promise(resolve=>child.once('exit',resolve));
 await writeFile(new URL('./facts.json',import.meta.url),JSON.stringify(facts,null,2)+'\n');
 console.log(JSON.stringify({exitCode:facts.exitCode,caseCount:facts.cases.length,cases:facts.cases,providerCallCount:facts.providerCalls.length,fatal:facts.fatal},null,2));
}
