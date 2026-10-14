// Real existing ChannelPlugin callback reentry; no fabricated inbound message.
import test from "node:test";
import assert from "node:assert/strict";
import {mkdtempSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {EchoAgentClient} from "/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance/sdks/typescript/dist/index.js";
test("channel callback manager read reentry must finish while outer callback pending",{timeout:30000},async()=>{
 const work=mkdtempSync(join(tmpdir(),"sdk-channel-reentry-"));console.log(`Reentry evidence ${work}`);assert.equal(spawnSync("python3",["/tmp/sdk-mcp-channel-candidate-fixture/config.py",work]).status,0);
 const sdk=await EchoAgentClient.spawn({hostCommand:process.env.ECHO_AGENT_SDK_HOST,args:["--config",join(work,"host.json")]});let session,manager,reentrant,readResolvedBeforeCallbackReturn=false;const events=[];
 try{
  const handler=await sdk.registerChannelMessageHandler("handler",{kind:"channel_message_handler",descriptor_version:1,handler_id:"handler"},()=>{throw new Error("no real inbound path");});
  const plugin=await sdk.registerChannelPlugin("plugin",{kind:"channel_plugin",descriptor_version:1,channel_id:"a",label:"reentry",handler_id:"handler",capabilities:{chat_types:["direct"],supports_media:false,supports_threads:false}},async call=>{
   const operation=call.invocation.operation;events.push({operation,kind:"callback-enter"});
   if(operation==="channel_health"){
    reentrant=sdk.call("channels.manager.ids",session.wire,[manager]);
    const raced=await Promise.race([reentrant.then(value=>({read:value})),new Promise(resolve=>setTimeout(()=>resolve({deadline:true}),1500))]);
    readResolvedBeforeCallbackReturn=!!raced.read;events.push({kind:"read-before-callback-return",raced});
   }
   events.push({operation,kind:"callback-return"});return{outcome:"result",result:{operation,value:null}};
  });
  const agent=await sdk.createAgent();session=await agent.createSession();manager=(await sdk.call("channels.manager.open",session.wire,[])).resource;
  await sdk.call("channels.plugin.register",session.wire,[manager,plugin.wire,handler.wire]);await sdk.call("channels.manager.start",session.wire,[manager,handler.wire]);
  const health=await sdk.call("channels.manager.health",session.wire,[manager,"a"]);const read=await reentrant;events.push({kind:"read-after-outer-response",value:read,health});
  await session.close();await plugin.close();await handler.close();await agent.close();writeFileSync(join(work,"observed.json"),JSON.stringify({events,readResolvedBeforeCallbackReturn}));
  assert.equal(readResolvedBeforeCallbackReturn,true,"manager.ids remained blocked until ChannelHealth reverse callback returned; a callback that awaits this read has a circular wait");
 }finally{await sdk.close();}
});
