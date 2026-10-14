// DRAFT. Intended final path sdks/typescript/test/workflow-host.test.js.
import test from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";
import { EchoAgentClient, EchoAgentError } from "/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/typescript/dist/index.js";
const host = process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required Workflow Host missing");
const fixture = "/tmp/sdk-workflow-candidate-fixture/workflow_fixture.py";
function helper(work, args) {
  const output = spawnSync("python3",[fixture,"--work",work,...args],{encoding:"utf8"});
  assert.equal(output.status,0,output.stderr);return output.stdout;
}
test("workflow routing/checkpoint/claims/branch/owner and shape boundary",{skip:!host,timeout:120000},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-ts-workflow-"));let sdk,passed=false;
 try {
  helper(work,["--prepare"]);const scenario=JSON.parse(readFileSync(join(work,"scenario.json"),"utf8"));
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")],onStderr:chunk=>process.stderr.write(chunk)});
  const agent=await sdk.createAgent();const sessions={a:await agent.createSession(),b:await agent.createSession()};
  const results={},rows=[];
  for(const step of scenario.steps){
   const session=sessions[step.session];
   if(step.action==="close"){await session.close();const row={id:step.id,outcome:"closed"};rows.push(row);results[step.id]=row;writeFileSync(join(work,"results.json"),JSON.stringify(rows));continue;}
   const resolution=join(work,"resolve.json");writeFileSync(resolution,JSON.stringify({value:step.arguments,results}));
   const args=JSON.parse(helper(work,["--resolve",resolution]));writeFileSync(join(work,`call-${step.id}.json`),JSON.stringify({operation:step.operation,arguments:args}));let row;
   try {row={id:step.id,outcome:"result",value:await sdk.call(step.operation,session.wire,args)};}
   catch(error){assert.ok(error instanceof EchoAgentError,String(error));row={id:step.id,outcome:"error",code:error.code,message:error.message,operation:error.operation,retryable:error.retryable,details:error.details};}
   rows.push(row);results[step.id]=row;writeFileSync(join(work,"results.json"),JSON.stringify(rows));
  }
  await sessions.b.close();await agent.close();await sdk.close();sdk=undefined;
  const resultPath=join(work,"results.json");writeFileSync(resultPath,JSON.stringify(rows));
  const verification=helper(work,["--verify",resultPath]);process.stdout.write(`Workflow evidence: ${verification}`);passed=true;
 }finally{await sdk?.close();if(passed&&!process.env.ECHO_AGENT_SDK_RETAIN_EVIDENCE)rmSync(work,{recursive:true,force:true});else process.stderr.write(`Workflow evidence retained: ${work}\n`);}
});
