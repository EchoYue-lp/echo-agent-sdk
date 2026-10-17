import test from "node:test";
import assert from "node:assert/strict";
import {spawn,spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,writeFileSync,existsSync} from "node:fs";
import {join,dirname} from "node:path";
import {fileURLToPath} from "node:url";
import {tmpdir} from "node:os";
import {EchoAgentClient,fromWireValue,toWireValue} from "../dist/index.js";
const host=process.env.ECHO_AGENT_SDK_HOST;
if(process.env.ECHO_AGENT_SDK_REQUIRE_HOST_E2E==="1"&&!host)throw new Error("required MCP/channel Host missing");
const dir=dirname(fileURLToPath(new URL("../../shared/testing/mcp_channel_stdio_fixture.py",import.meta.url)));
function jsonWire(value){if(value===null)return{kind:"null"};if(typeof value==="boolean")return{kind:"bool",value};if(typeof value==="string")return{kind:"string",value};if(typeof value==="number")return{kind:"u64",value:String(value)};if(Array.isArray(value))return{kind:"list",value:value.map(jsonWire)};return{kind:"map",value:Object.entries(value).map(([key,v])=>({key:{kind:"string",value:key},value:jsonWire(v)}))};}
const clientOp=name=>`echo_agent::mcp::McpClient::${name}`;
const fromTransport="echo_integration::mcp::client::McpClient::from_transport";
function setup(name){const work=mkdtempSync(join(tmpdir(),`sdk-ts-${name}-`));assert.equal(spawnSync("python3",[join(dir,"mcp_channel_config_fixture.py"),work]).status,0);console.log(`MCP/channel evidence ${work}`);return work;}
async function until(predicate){const deadline=Date.now()+5000;while(!(await predicate())){assert.ok(Date.now()<deadline,"condition missing");await new Promise(r=>setTimeout(r,10));}}
function live(pid){try{process.kill(pid,0);return true;}catch(e){if(e.code==="ESRCH")return false;throw e;}}
async function save(work,observed){writeFileSync(join(work,"observed.json"),JSON.stringify(observed));}

test("MCP real stdio metadata/resources/prompts and same-name owner cleanup",{timeout:60000,skip:!host},async()=>{
 const work=setup("mcp-stdio"),sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});const observed=[];
 try{
  const agent=await sdk.createAgent(),one=await agent.createSession(),two=await agent.createSession();
  const call=async(s,operation,args)=>{const value=await sdk.call(operation,s.wire,args);observed.push({operation,value});await save(work,observed);return value;};
  const ma=(await call(one,"mcp.manager.open",[])).resource,mb=(await call(two,"mcp.manager.open",[])).resource;
  const a=(await call(one,"mcp.server.connect",[ma,"same-name","stdio","python3",[join(dir,"mcp_channel_stdio_fixture.py"),work,"a"]])).client;
  const b=(await call(two,"mcp.server.connect",[mb,"same-name","stdio","python3",[join(dir,"mcp_channel_stdio_fixture.py"),work,"b"]])).client;
  const pidA=Number(readFileSync(join(work,"a.pid"),"utf8")),pidB=Number(readFileSync(join(work,"b.pid"),"utf8"));assert.ok(live(pidA)&&live(pidB));
  assert.deepEqual(await call(one,clientOp("server_capabilities"),[a]),{prompts:{listChanged:false},resources:{subscribe:false,listChanged:false},tools:{}});
  assert.deepEqual(await call(one,clientOp("tools"),[a]),[{name:"echo",description:"echo Unicode",inputSchema:{type:"object",properties:{text:{type:"string"}},required:["text"]}}]);
  const resources=[{uri:"fixture://中文/text",name:"中文🙂",description:"text meta",mimeType:"text/plain"},{uri:"fixture://binary",name:"binary",mimeType:"application/octet-stream"}];
  assert.deepEqual(await call(one,clientOp("resources"),[a]),resources);assert.deepEqual(await call(one,clientOp("list_resources"),[a]),resources);
  assert.deepEqual(await call(one,clientOp("list_resource_templates"),[a]),[{uriTemplate:"fixture://item/{id}",name:"模板🙂",description:"template",mimeType:"text/plain"}]);
  assert.deepEqual(await call(one,clientOp("read_resource"),[a,"fixture://binary"]),{contents:[{uri:"fixture://binary",mimeType:"application/octet-stream",blob:"AP9BgA=="}]});
  assert.deepEqual(await call(one,clientOp("read_resource"),[a,"fixture://中文/text"]),{contents:[{uri:"fixture://中文/text",mimeType:"text/plain",text:"资源 中文🙂"}]});
  assert.deepEqual(await call(one,clientOp("get_prompt"),[a,"compose",{topic:"无损🧪"}]),{description:"topic prompt",messages:[{role:"user",content:{type:"text",text:"主题 无损🧪"}}]});
  assert.deepEqual(await call(one,clientOp("call_tool"),[a,"echo",{text:"工具🙂"}]),{content:[{type:"text",text:"工具🙂"}],isError:false});assert.equal(await call(one,clientOp("ping"),[a]),null);
  await assert.rejects(sdk.call(clientOp("ping"),two.wire,[a]),e=>e.code==="invalid_value");
  await call(one,"mcp.server.disconnect",[ma,"same-name"]);await until(()=>!live(pidA));assert.ok(live(pidB));
  await assert.rejects(sdk.call(clientOp("ping"),one.wire,[a]),e=>e.code==="closed_handle");assert.equal(await call(two,clientOp("ping"),[b]),null);
  await one.close();await assert.rejects(sdk.call("mcp.manager.server_names",two.wire,[ma]),e=>e.code==="closed_handle");assert.ok(live(pidB));
  await two.close();await until(()=>!live(pidB));await agent.close();observed.push({process_cleanup:{a_dead:true,b_survived_a_close:true,b_dead_after_own_close:true}});await save(work,observed);
 }finally{await sdk.close();}
});

test("MCP registered transport failed close retains exact client/quota for retry",{timeout:60000,skip:!host},async()=>{
 const work=setup("mcp-transport-retry"),sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});const calls=[];let closeCount=0;
 try{
  const reg=await sdk.registerAgentComponent("mcp-transport",{kind:"agent_component",descriptor_version:1,component:"mcp_transport",name:"transport",capabilities:{supports_notifications:false}},async call=>{
   const inner=call.invocation.input.call,op=inner.operation;calls.push(inner);await save(work,calls);
   if(op==="mcp_transport_close"&&++closeCount===1)return{outcome:"error",error:{code:"extension_conflict",message:"controlled transport close failure",retryable:"after_delay"}};
   let result;
   if(op==="mcp_transport_send"){
    const req=fromWireValue(inner.input.request);let value={};if(req.method==="initialize")value={protocolVersion:"2024-11-05",capabilities:{},serverInfo:{name:"registered-中文🙂",version:"1"}};
    result={operation:op,value:{response:toWireValue({jsonrpc:"2.0",id:req.id,result:value})}};
   }else if(op==="mcp_transport_try_notification")result={operation:op,value:{notification:null}};
   else result={operation:op};
   return{operation:"agent_component_call",value:{component:"mcp_transport",result}};
  });
  const agent=await sdk.createAgent(),session=await agent.createSession();
  const client=await sdk.invoke(fromTransport,agent.wire,[session.wire,"registered-中文🙂",reg.wire]);
  await assert.rejects(sdk.call(clientOp("close"),session.wire,[client]),e=>e.code==="framework_error"&&e.retryable==="after_delay");
  assert.equal(closeCount,1);assert.equal(await sdk.call(clientOp("server_name"),session.wire,[client]),"registered-中文🙂");
  assert.equal(await sdk.call(clientOp("close"),session.wire,[client]),null);assert.equal(closeCount,2);
  await assert.rejects(sdk.call(clientOp("ping"),session.wire,[client]),e=>e.code==="closed_handle");
  for(let i=0;i<4;i++)await sdk.call("mcp.manager.open",session.wire,[]);await assert.rejects(sdk.call("mcp.manager.open",session.wire,[]),e=>e.code==="payload_too_large");
  await session.close();await reg.close();await agent.close();await save(work,{calls,closeCount,released_quota_refilled:4});
 }finally{await sdk.close();}
});

async function channels(mode){
 const work=setup(`channels-${mode}`),sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});const calls=[],stops={a:0,b:0},running={a:false,b:false},processes={};let manager,session;
 try{
  const handler=await sdk.registerChannelMessageHandler("handler",{kind:"channel_message_handler",descriptor_version:1,handler_id:"handler"},()=>{throw new Error("No real inbound path exists: this callback must not be faked");});const regs=[];
  for(const id of ["a","b"])regs.push(await sdk.registerChannelPlugin("plugin-"+id,{kind:"channel_plugin",descriptor_version:1,channel_id:id,label:"通道 "+id,handler_id:"handler",capabilities:{chat_types:["direct","group"],supports_media:true,supports_threads:true}},async call=>{
   const op=call.invocation.operation;calls.push({id,operation:op,input:call.invocation.input});await save(work,{calls,stops,running});
   if(op==="channel_start"){running[id]=true;processes[id]=spawn("python3",["-c","import time; time.sleep(60)"]);writeFileSync(join(work,id+".pid"),String(processes[id].pid));}
   if(op==="channel_stop"){
    stops[id]++;if(mode==="retry"&&id==="a"&&stops.a===1)return{outcome:"error",error:{code:"extension_conflict",message:"stop once fails",retryable:"after_delay"}};const child=processes[id];child.kill();await new Promise(resolve=>child.exitCode!==null||child.signalCode!==null?resolve():child.once("exit",resolve));running[id]=false;
   }
   return{outcome:"result",result:{operation:op,value:null}};
  }));
  const agent=await sdk.createAgent();session=await agent.createSession();manager=(await sdk.call("channels.manager.open",session.wire,[])).resource;
  for(const reg of regs)assert.equal((await sdk.call("channels.plugin.register",session.wire,[manager,reg.wire,handler.wire])).registered,true);
  assert.deepEqual(await sdk.call("channels.manager.start",session.wire,[manager,handler.wire]),{channels:"2",failures:"0"});assert.deepEqual((await sdk.call("channels.manager.ids",session.wire,[manager])).sort(),["a","b"]);
  const attachments=["image","file","audio","video"].map((kind,i)=>({kind,data_base64:Buffer.from([0,255,i,128]).toString("base64"),filename:`文件-${kind}-🙂.bin`}));
  const message={channel_id:"a",to:"收件人🙂",chat_type:"direct",text:"hello 中文🧪",reply_to:"原消息🙂",attachments};
  assert.equal((await sdk.call("channels.plugin.send",session.wire,[manager,"a",jsonWire(message)])).sent,true);assert.deepEqual(calls.find(x=>x.operation==="channel_send").input,{message});
  assert.deepEqual(calls.filter(x=>x.operation==="channel_start").map(x=>x.input),[{handler_id:"handler"},{handler_id:"handler"}]);
  if(mode==="retry"){await assert.rejects(sdk.call("channels.manager.stop_exact",session.wire,[manager,"a"]),e=>e.code==="framework_error");assert.ok(live(processes.a.pid)&&live(processes.b.pid));}
  assert.equal(running.b,true);assert.equal(stops.b,0);assert.equal(await sdk.call("channels.manager.stop_exact",session.wire,[manager,"a"]),null);assert.equal(running.a,false);assert.equal(running.b,true);assert.equal(stops.b,0);assert.ok(!live(processes.a.pid)&&live(processes.b.pid));assert.equal(stops.a,mode==="retry"?2:1);
  assert.equal((await sdk.call("channels.manager.health",session.wire,[manager,"b"])).healthy,true);
  await session.close();assert.equal(running.b,false);assert.equal(stops.b,1);assert.ok(!live(processes.b.pid));for(const reg of regs)await reg.close();await handler.close();await agent.close();await save(work,{calls,stops,running,inbound_handler_calls:0});
 }finally{await sdk.close();for(const child of Object.values(processes))if(child.exitCode===null&&child.signalCode===null)child.kill();}
}
for(const mode of ["normal","retry"])test(`Channels attachments/exact stop ${mode}`,{timeout:60000,skip:!host},()=>channels(mode));

test("Channel owned callbacks reenter metadata and preserve exact close owner", {timeout:60000,skip:!host}, async()=>{
 const work=setup("channels-owned-reentry"),sdk=await EchoAgentClient.spawn({hostCommand:host,args:["--config",join(work,"host.json")]});
 const events=[],owners={},phases={},stops={a:0,b:0,other:0};let parkHealth=false,failCloseOnce=true,parkedResolve,releaseResolve;
 const parked=new Promise(resolve=>parkedResolve=resolve),release=new Promise(resolve=>releaseResolve=resolve);
 const closeManager=(session,manager)=>sdk.request("_echo_agent/facade/invoke",{operation:"facade.resource.close",signature_digest:sdk.catalog.resolve("facade.resource.close").signature,handle:session.wire,arguments:[{kind:"handle",value:manager}]});
 const bounded=async(promise)=>{let timer;try{return await Promise.race([promise,new Promise((_,reject)=>timer=setTimeout(()=>reject(new Error("metadata/independent operation blocked")),2000))]);}finally{clearTimeout(timer);}};
 try{
  const handler=await sdk.registerChannelMessageHandler("owned-handler",{kind:"channel_message_handler",descriptor_version:1,handler_id:"owned-handler"},()=>{throw new Error("no accepted inbound consumer; do not fabricate one");});
  const agent=await sdk.createAgent(),one=await agent.createSession(),two=await agent.createSession();
  const ma=(await sdk.call("channels.manager.open",one.wire,[])).resource,mb=(await sdk.call("channels.manager.open",two.wire,[])).resource,regs=[];
  for(const [key,channel,session,manager,ids] of [["a","a",one,ma,["a","b"]],["b","b",one,ma,["a","b"]],["other","c",two,mb,["c"]]]){
   owners[key]={session,manager,ids};phases[key]="active";
   const reg=await sdk.registerChannelPlugin("owned-"+key,{kind:"channel_plugin",descriptor_version:1,channel_id:channel,label:"owned 中文🙂 "+key,handler_id:"owned-handler",capabilities:{chat_types:["direct"],supports_media:false,supports_threads:false}},async call=>{
    const operation=call.invocation.operation,{session,manager,ids}=owners[key],phase=phases[key];
    for(const metadata of ["ids","list"]){
     try{const value=await bounded(sdk.call("channels.manager."+metadata,session.wire,[manager]));assert.equal(phase,"active");assert.deepEqual((metadata==="ids"?value:value.channels).sort(),ids);events.push({id:key,operation,phase,metadata,result:"read-before-return"});}
     catch(error){assert.notEqual(phase,"active");assert.equal(error.code,"closed_handle");events.push({id:key,operation,phase,metadata,result:error.code});}
    }
    await save(work,events);
    if(key==="a"&&operation==="channel_health"&&parkHealth){parkedResolve();await release;}
    if(operation==="channel_stop"){stops[key]++;if(key==="a"&&phase==="resource-close"&&failCloseOnce){failCloseOnce=false;return{outcome:"error",error:{code:"extension_conflict",message:"owned stop once fails",retryable:"after_delay"}};}}
    return{outcome:"result",result:{operation,value:null}};
   });regs.push(reg);await sdk.call("channels.plugin.register",session.wire,[manager,reg.wire,handler.wire]);
  }
  assert.deepEqual(await sdk.call("channels.manager.start",one.wire,[ma,handler.wire]),{channels:"2",failures:"0"});
  assert.deepEqual(await sdk.call("channels.manager.start",two.wire,[mb,handler.wire]),{channels:"1",failures:"0"});
  const message={channel_id:"a",to:"用户🙂",chat_type:"direct",text:"重入中文"};assert.equal((await sdk.call("channels.plugin.send",one.wire,[ma,"a",jsonWire(message)])).sent,true);
  parkHealth=true;let healthDone=false;const health=sdk.call("channels.manager.health",one.wire,[ma,"a"]).finally(()=>healthDone=true);
  try{await bounded(parked);assert.equal(healthDone,false);assert.deepEqual((await bounded(sdk.call("channels.manager.ids",one.wire,[ma]))).sort(),["a","b"]);assert.equal((await bounded(sdk.call("channels.manager.health",one.wire,[ma,"b"]))).healthy,true);assert.equal((await bounded(sdk.call("channels.manager.health",two.wire,[mb,"c"]))).healthy,true);assert.equal(healthDone,false);}finally{releaseResolve();}
  assert.equal((await health).healthy,true);
  assert.equal((await sdk.call("channels.manager.stop",one.wire,[ma,"b"])).stopped,true);assert.equal(await sdk.call("channels.manager.stop_exact",one.wire,[ma,"a"]),null);assert.deepEqual(stops,{a:1,b:1,other:0});
  await sdk.call("channels.manager.start",one.wire,[ma,handler.wire]);assert.equal(await sdk.call("channels.manager.stop_all",one.wire,[ma]),null);assert.deepEqual(stops,{a:2,b:2,other:0});
  await sdk.call("channels.manager.start",one.wire,[ma,handler.wire]);phases.a=phases.b="resource-close";
  await assert.rejects(closeManager(one,ma),error=>error.code==="framework_error");assert.deepEqual(stops,{a:3,b:3,other:0});assert.equal((await sdk.call("channels.manager.health",two.wire,[mb,"c"])).healthy,true);
  await closeManager(one,ma);assert.deepEqual(stops,{a:4,b:3,other:0});await assert.rejects(sdk.call("channels.manager.ids",one.wire,[ma]),error=>error.code==="closed_handle");
  for(let i=0;i<3;i++)await sdk.call("channels.manager.open",one.wire,[]);await assert.rejects(sdk.call("channels.manager.open",one.wire,[]),error=>error.code==="payload_too_large");
  await one.close();phases.other="session-close";await two.close();assert.deepEqual(stops,{a:4,b:3,other:1});for(const reg of regs)await reg.close();await handler.close();await agent.close();
  await save(work,{events,stops,parked_other_access:true,quota_refilled:3,inbound_handler_calls:0});
 }finally{releaseResolve();await sdk.close();}
});
