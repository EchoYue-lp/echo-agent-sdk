import test from 'node:test';
import assert from 'node:assert/strict';
import {client} from '@agentclientprotocol/sdk';
import {EchoAgentClient} from '../dist/client.js';
const deferred=()=>{let resolve,reject;const promise=new Promise((a,b)=>{resolve=a;reject=b;});return{promise,resolve,reject};};
const tick=()=>new Promise(resolve=>setImmediate(resolve));
function fixture(){
 const writes=[];const cx=client({name:'terminal-route-windows'}).connect({readable:new ReadableStream({}),writable:new WritableStream({write(message){const done=deferred();writes.push({message,done});return done.promise;}})});
 const sdk=Object.create(EchoAgentClient.prototype);sdk.connection=cx;sdk.invocationControllers=new Map();sdk.extensionStreamRoutes=new Map();
 const stream={id:'stream',generation:'1',kind:'stream'},extension={id:'extension',generation:'1',kind:'extension'},controller=new AbortController();
 const route={invocationId:'original',stream,extension,controller};sdk.invocationControllers.set('original',controller);sdk.extensionStreamRoutes.set(JSON.stringify(['stream','1','stream']),route);
 return{sdk,cx,writes,stream,controller};
}
test('official writer wrong stream identity cannot retire original cancellation address',async()=>{
 const{sdk,cx,writes,stream,controller}=fixture();
 try{for(const wrong of [{...stream,generation:'2'},{...stream,id:'different'},{...stream,kind:'run'}]){const send=sdk.notify('_echo_agent/extension/stream',{event:'cancelled',stream:wrong,sequence:'1'});await tick();writes.at(-1).done.resolve();await send;assert.equal(sdk.invocationControllers.get('original'),controller);assert.equal(sdk.extensionStreamRoutes.size,1);}
 const send=sdk.notify('_echo_agent/extension/stream',{event:'cancelled',stream,sequence:'1'});await tick();assert.equal(sdk.extensionStreamRoutes.size,1);writes.at(-1).done.resolve();await send;assert.equal(sdk.extensionStreamRoutes.size,0);assert.equal(sdk.invocationControllers.size,0);
 }finally{cx.close();}
});
test('official failed terminal writer preserves cancellation address and immutable request identity',async()=>{
 const{sdk,cx,writes,stream,controller}=fixture();const params={event:'cancelled',stream,sequence:'1'};
 try{const send=sdk.notify('_echo_agent/extension/stream',params);send.catch(()=>{});params.stream.generation='2';await tick();assert.equal(writes[0].message.params.stream.generation,'1');writes[0].done.reject(new Error('native terminal failure'));await assert.rejects(send);assert.equal(sdk.invocationControllers.get('original'),controller);assert.equal(sdk.extensionStreamRoutes.size,1);
 }finally{cx.close();}
});
