import test from "node:test";
import assert from "node:assert/strict";
import {spawn, execFile} from "node:child_process";
import {promisify} from "node:util";
import {mkdir, mkdtemp, readFile, writeFile, access} from "node:fs/promises";
import {join, resolve, relative, isAbsolute} from "node:path";
import {fileURLToPath} from "node:url";
import {once} from "node:events";
import {EchoAgentClient, fromWireValue} from "../dist/index.js";

const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host) throw Error("required Host missing");
const sdkRoot=fileURLToPath(new URL("../../..",import.meta.url));
const helper=join(sdkRoot,"sdks/shared/testing/consumer_provider_http.py");
async function closeHttpFixture(sdk, collecting, server, exited, recordExit) {
  let closeFailure;
  try { await sdk?.close(); }
  catch (error) { closeFailure = {error}; }
  finally {
    let fixtureFailure;
    try {
      // SDK拥有Host进程；此夹具仅收口自己创建的HTTP子进程，真实exit另存。
      if (server.exitCode === null && server.signalCode === null) server.kill("SIGTERM");
      await recordExit(await exited);
    } catch (error) { fixtureFailure = {error}; }
    if (collecting) await Promise.allSettled([collecting]);
    if (closeFailure && fixtureFailure) throw new AggregateError([closeFailure.error, fixtureFailure.error], "SDK close and HTTP fixture cleanup failed");
    if (fixtureFailure) throw fixtureFailure.error;
    if (closeFailure) throw closeFailure.error;
  }
}

test("HTTP fixture exits and records facts when SDK close rejects", {timeout:10000}, async () => {
  const server = spawn(process.execPath, ["-e", "process.stdout.write('ready\\n');setInterval(()=>{},1000)"], {stdio:["ignore","pipe","pipe"]});
  const exited = new Promise((done, fail) => { server.once("error", fail); server.once("exit", (code, signal) => done({code,signal})); });
  const failure = Error("forced original SDK close failure");
  let recorded;
  try {
    await once(server.stdout, "data");
    await assert.rejects(closeHttpFixture({close:async()=>{throw failure;}}, undefined, server, exited, async value=>{recorded=value;}), error=>error === failure);
    assert.equal(recorded.signal, "SIGTERM");
    assert.equal(server.signalCode, "SIGTERM");
  } finally {
    if (server.exitCode === null && server.signalCode === null) server.kill("SIGTERM");
    await exited;
  }
});

async function barrier(file,child) {
  const deadline=Date.now()+10000;
  while(Date.now()<deadline) {
    if(child.exitCode!==null) throw Error(`HTTP fixture exited ${child.exitCode}`);
    try {await access(file);return;} catch {await new Promise(done=>setTimeout(done,10));}
  }
  throw Error(`fixture barrier missing: ${file}`);
}
for(const protocol of ["responses","anthropic","chat_completions"]) for(const mode of ["success","stop_continue","cancel","eof"]) {
  test(`actual Host/provider HTTP ${protocol}/${mode}`,{skip:!host,timeout:60000},async()=>{
    const root=process.env.ECHO_AGENT_SDK_CONSUMER_ROOT;
    assert.ok(root&&isAbsolute(root));
    const under=relative(resolve(sdkRoot,".."),resolve(root));assert.ok(under&&!under.startsWith("..")&&!isAbsolute(under));
    await mkdir(root,{recursive:true});const work=await mkdtemp(join(root,`http-ts-${protocol}-${mode}-`));
    const args=[helper,"--work",work,"--protocol",protocol,"--mode",mode];
    if(process.env.ECHO_AGENT_SDK_CONSUMER_IMAGE==="1") args.push("--image");
    const server=spawn(process.env.PYTHON??"python3",args,{stdio:["ignore","pipe","pipe"]});
    let serverError="";server.stderr.on("data",value=>{serverError+=value;});
    const exited=new Promise((done,fail)=>{server.on("error",fail);server.on("exit",(code,signal)=>done({code,signal}));});
    const facts={tool_calls:[],stop_calls:0,public:[],live:[],replay:[]};let sdk;let collecting;
    try {
      await barrier(join(work,"ready"),server);const spec=JSON.parse(await readFile(join(work,"spec.json"),"utf8"));
      sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});
      const registrations=[];
      for(const name of ["consumer_image","consumer_error"]) {
        registrations.push(await sdk.registerExtension("tool",name,{...spec.tool_descriptor,name},async call=>{
          const operation=call.invocation.operation;
          if(operation==="tool_validate_parameters") return {outcome:"result",result:{operation,value:null}};
          assert.equal(operation,"tool_execute");facts.tool_calls.push(name);
          await writeFile(join(work,`${name}-effect`),String(facts.tool_calls.filter(item=>item===name).length));
          return {outcome:"result",result:{operation,value:structuredClone(spec.results[name])}};
        }));
      }
      if(mode==="stop_continue") registrations.push(await sdk.registerHook("continue-once",{kind:"hook",descriptor_version:1,events:["Stop"]},async call=>{
        const result=structuredClone(spec.hook_allow);
        if(facts.stop_calls++===0) result.continue_reason="continue original execution";
        return {outcome:"result",result:{operation:call.invocation.operation,value:result}};
      }));
      const agent=await sdk.createAgent(),session=await agent.createSession(work);
      const run=await session.startRun({text:"two actual tool results and final validation"});
      collecting=(async()=>{for await(const item of run.events) {
        assert.ok(item.envelope,"unexpected gap");facts.live.push(item.envelope);
        const payload=item.envelope.payload,value=fromWireValue(payload.data);
        if(payload.event_type==="assistant_message") facts.public.push({kind:"assistant_message",text:value.block.delta});
        if(payload.event_type==="final_answer") facts.public.push({kind:"final_answer",text:value});
        if(payload.event_type==="execution_status") facts.public.push({kind:"execution_status",status:value.status,phase:value.phase,execution_id:value.execution_id});
      }})();collecting.catch(()=>{});
      if(mode==="cancel") {await barrier(join(work,"cancel-ready"),server);await run.cancel();}
      facts.receipt=await run.wait({timeoutMs:20000});await collecting;
      const replay=await run.replay("0",512);facts.replay=replay.events.map(item=>item.envelope);
      await writeFile(join(work,"facts.json"),JSON.stringify(facts));
      await promisify(execFile)(process.env.PYTHON??"python3",[...args,"--verify"],{timeout:10000});
      await run.close();await session.close();await agent.close();for(const item of registrations.reverse()) await item.close();
    } finally {
      await closeHttpFixture(sdk, collecting, server, exited,
        closed=>writeFile(join(work,"fixture-exit.json"),JSON.stringify({...closed,stderr:serverError})));
    }
  });
}
