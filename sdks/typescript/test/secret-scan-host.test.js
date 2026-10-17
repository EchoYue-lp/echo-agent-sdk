import test from "node:test";
import assert from "node:assert/strict";
import {execFile} from "node:child_process";
import {promisify} from "node:util";
import {mkdtemp,readFile,writeFile,mkdir} from "node:fs/promises";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {EchoAgentClient,toWireValue} from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required Security Host missing");
const fixture=fileURLToPath(new URL("../../shared/testing/secret_scan_host_fixture.py",import.meta.url));
test("accepted Security root aliases preserve Unicode secret span and byte offset without model effects",{skip:!host,timeout:30000},async()=>{
 const root=process.env.ECHO_AGENT_SDK_SECRET_ROOT??tmpdir();await mkdir(root,{recursive:true});const work=await mkdtemp(join(root,"typescript-"));
 const helper=async(...args)=>JSON.parse((await promisify(execFile)(process.env.PYTHON??"python3",[fixture,"--work",work,...args],{timeout:5000})).stdout||"null");await helper();
 const spec=JSON.parse(await readFile(join(work,"scenario.json"),"utf8"));let sdk;let provider=0;const facts={rows:[]};
 try{
  sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});
  const registration=await sdk.registerLlmClient("security-unused-model",{kind:"llm_client",descriptor_version:1,model_name:"security-unused-model",supports_streaming:false},async()=>{provider++;throw new Error("Security helper must not invoke model");});
  const agent=await sdk.createAgent(),session=await agent.createSession(work);facts.identity={agent:agent.wire,session:session.wire};facts.before=await helper("--snapshot");
  for(const operation of spec.operations){
   for(const item of spec.texts)facts.rows.push({operation,mode:"value",text_id:item.id,result:await session.invoke(operation,[item.text])});
   for(const mode of ["wrong-signature","missing-text"]){
    const resolved=sdk.catalog.resolve(operation);const request={operation,signature_digest:mode==="wrong-signature"?"sha256:"+"0".repeat(64):resolved.signature,handle:agent.wire,arguments:[toWireValue(session.wire),...(mode==="wrong-signature"?[toWireValue(spec.texts[0].text)]:[])]};
    let caught;try{await sdk.request(resolved.method,request);assert.fail("invalid source invocation succeeded");}catch(error){assert.ok(error.code);caught={code:error.code,retryable:error.retryable,operation:error.operation,handle:error.handle??null,details:error.details??null,message:error.message};}
    facts.rows.push({operation,mode,request,error:caught});
   }
  }
  const scan=spec.operations.find(value=>value.endsWith("scan_secrets")),resolved=sdk.catalog.resolve(scan);
  facts.raw_scan=await sdk.request(resolved.method,{operation:scan,signature_digest:resolved.signature,handle:agent.wire,arguments:[toWireValue(session.wire),toWireValue(spec.texts[0].text)]});facts.provider_calls=provider;facts.after=await helper("--snapshot");
  await session.close();await agent.close();await registration.close();await writeFile(join(work,"observed.json"),JSON.stringify(facts,null,2));console.log("Security native evidence:",work,await helper("--verify"));
 }finally{if(sdk)await sdk.close();}
});
