import test from "node:test";
import assert from "node:assert/strict";
import {execFile} from "node:child_process";
import {promisify} from "node:util";
import {mkdtemp,readFile,writeFile,mkdir} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {EchoAgentClient,RunHandle} from "../dist/index.js";

const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required idempotency Host is missing");
const fixture=fileURLToPath(new URL("../../shared/testing/session_run_idempotency_fixture.py",import.meta.url));
const proxy=fileURLToPath(new URL("../../shared/testing/gap_ack_host_proxy.py",import.meta.url));
const gate=()=>{let release;return{promise:new Promise(resolve=>{release=resolve;}),release:()=>release()};};
async function until(predicate){const end=Date.now()+5000;while(!predicate()){assert.ok(Date.now()<end,"native provider barrier missing");await new Promise(resolve=>setTimeout(resolve,5));}}
function errorFact(error){return{code:error.code,retryable:error.retryable,operation:error.operation,details:error.details??null,handle:error.handle??null};}

test("Session and Run keys replay one real publication and release pre-effect busy admission",{skip:!host,timeout:30000},async()=>{
 const root=process.env.ECHO_AGENT_SDK_IDEMPOTENCY_ROOT??tmpdir();await mkdir(root,{recursive:true});const work=await mkdtemp(join(root,"typescript-"));
 const helper=async(...args)=>(await promisify(execFile)(process.env.PYTHON??"python3",[fixture,"--work",work,...args],{timeout:5000})).stdout;
 await helper();const spec=JSON.parse(await readFile(join(work,"scenario.json"),"utf8"));const facts={groups:{},calls:[],requests:[],provider_finally:0};let sdk,registration,held;const sessions=[];
 try{
  sdk=await EchoAgentClient.spawn({hostCommand:process.env.PYTHON??"python3",args:[proxy,host,join(work,"host.json")],env:{...process.env,NO_PROXY:"127.0.0.1,localhost",no_proxy:"127.0.0.1,localhost"}});
  registration=await sdk.registerLlmClient("idempotency-native",spec.descriptor,async call=>{
   facts.calls.push({operation:call.invocation.operation,invocation_id:call.invocation_id,context:call.context,input:call.invocation.input});const wait=held;
   try{if(wait)await wait.promise;return{operation:"llm_chat",value:structuredClone(spec.response)};}finally{facts.provider_finally++;}
  });
  const rpc=async(method,params)=>{const row={method,params:structuredClone(params)};facts.requests.push(row);try{return row.response=await sdk.request(method,params);}catch(error){row.error=errorFact(error);throw error;}};
  const failure=async(method,params)=>{try{await rpc(method,params);assert.fail("key misuse unexpectedly succeeded");}catch(error){assert.ok(error.code);return errorFact(error);}};
  const wait=response=>rpc("_echo_agent/run/wait",{run:response.run,timeout:{seconds:"5",nanos:0}});
  const notices=async stream=> (await readFile(join(work,"notifications.jsonl"),"utf8")).split("\n").filter(Boolean).map(line=>JSON.parse(line)).filter(row=>["_echo_agent/event","_echo_agent/gap"].includes(row.method)&&row.params.stream.id===stream.id).length;
  const agent=await sdk.createAgent();const sessionParams={agent:agent.wire,idempotency_id:spec.keys.session};
  const first=await rpc("_echo_agent/session/create",sessionParams);sessions.push(first.session);
  const [duplicate,concurrent]=await Promise.all([rpc("_echo_agent/session/create",sessionParams),rpc("_echo_agent/session/create",sessionParams)]);assert.deepEqual(duplicate,first);assert.deepEqual(concurrent,first);
  const changed=await failure("_echo_agent/session/create",{...sessionParams,working_dir:{encoding:"utf8",path:join(work,"changed-cwd")}});
  facts.groups.session={first,duplicate,concurrent,changed,creations:JSON.parse(await readFile(join(work,"state/host/session-index.json"),"utf8")).length};
  held=gate();const runParams={session:first.session,input:{kind:"chat",text:spec.prompt},idempotency_id:spec.keys.run};
  const runFirst=await rpc("_echo_agent/run/start",runParams);await until(()=>facts.calls.length===1);
  const noticeBefore=await notices(runFirst.stream);const runConcurrent=await rpc("_echo_agent/run/start",runParams);assert.deepEqual(runConcurrent,runFirst);assert.equal(facts.calls.length,1);assert.equal(await notices(runFirst.stream),noticeBefore);
  const changedRun=await failure("_echo_agent/run/start",{...runParams,input:{kind:"chat",text:spec.changed_prompt}});held.release();held=undefined;const settlement=await wait(runFirst);
  await rpc("_echo_agent/run/get",{run:runFirst.run});const completedNotices=await notices(runFirst.stream);
  const completed=await rpc("_echo_agent/run/start",runParams);await new RunHandle(sdk,runFirst.run,runFirst.stream).close();const afterLocalClose=await rpc("_echo_agent/run/start",runParams);
  facts.groups.run={first:runFirst,concurrent:runConcurrent,completed,after_local_close:afterLocalClose,changed:changedRun,settlement,provider_calls:facts.calls.length,notifications_before_retry:completedNotices,notifications_after_retry:await notices(runFirst.stream)};
  // A second Agent owns an independent key scope without changing either payload.
  const otherAgent=await sdk.createAgent();const otherSession=await rpc("_echo_agent/session/create",{agent:otherAgent.wire,idempotency_id:spec.keys.session});sessions.push(otherSession.session);
  const prior=facts.calls.length;const otherRun=await rpc("_echo_agent/run/start",{...runParams,session:otherSession.session});await wait(otherRun);
  facts.groups.owner={distinct_session:otherSession.session.id!==first.session.id,distinct_run:otherRun.run.id!==runFirst.run.id,provider_calls:facts.calls.length-prior,session:otherSession,run:otherRun};
  held=gate();const busyBase=facts.calls.length;const original=await rpc("_echo_agent/run/start",{session:otherSession.session,input:{kind:"chat",text:spec.busy_prompt}});await until(()=>facts.calls.length===busyBase+1);
  const keyed={session:otherSession.session,input:{kind:"chat",text:spec.retry_prompt},idempotency_id:spec.keys.busy};const rejection=await failure("_echo_agent/run/start",keyed);const rejectedCount=facts.calls.length-busyBase;held.release();held=undefined;await wait(original);
  const retry=await rpc("_echo_agent/run/start",keyed);await wait(retry);facts.groups.busy={original,rejection,retry,provider_calls_while_rejected:rejectedCount,provider_calls_after_retry:facts.calls.length-busyBase};
  await rpc("_echo_agent/session/close",{session:first.session});const closedSession=await failure("_echo_agent/session/create",sessionParams);const closedRun=await failure("_echo_agent/run/start",runParams);facts.groups.closed={session:closedSession,run:closedRun};
  facts.provider_calls=facts.calls.length;facts.session_index_count=JSON.parse(await readFile(join(work,"state/host/session-index.json"),"utf8")).length;
  await rpc("_echo_agent/session/close",{session:otherSession.session});await otherAgent.close();await agent.close();await registration.close();registration=undefined;
  await writeFile(join(work,"observed.json"),JSON.stringify(facts,null,2));console.log("idempotency evidence:",work,await helper("--verify"));
 }finally{held?.release();if(sdk){for(const session of sessions)await sdk.request("_echo_agent/session/close",{session}).catch(()=>{});await registration?.close();await sdk.close();}}
});
