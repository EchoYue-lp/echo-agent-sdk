import test from 'node:test';
import assert from 'node:assert/strict';
import {client} from '@agentclientprotocol/sdk';
import {EchoAgentClient,ExtensionStreamWriter} from '../dist/client.js';
import {readFileSync} from 'node:fs';
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

const observationCases=JSON.parse(readFileSync(new URL('../../shared/testing/cancellation-observation-cases.json',import.meta.url),'utf8')).cases;
test('public writer keeps only complete legitimate observations open for the real terminal',async()=>{
 for(const item of observationCases){
  const frames=[];const sdk={async notify(method,params){frames.push({method,params:structuredClone(params)});}};
  const writer=new ExtensionStreamWriter(sdk,{id:'original-stream',generation:'1',kind:'stream'});
  await writer.failed(structuredClone(item.error));
  if(item.nonterminal){await writer.complete({kind:'agent',value:{event:'final_answer',data:{text:'actual final value'}}});assert.deepEqual(frames.map(frame=>frame.params.sequence),['1','2'],item.name);}
  else await assert.rejects(writer.complete({kind:'agent',value:{event:'cancelled'}}),/already closed/,item.name);
  assert.deepEqual(frames[0].params.error,item.error,item.name);
 }
});
test('official observation send retains the exact original cancellation route until real terminal',async()=>{
 const{sdk,cx,writes,stream,controller}=fixture();
 try{
  const observation=sdk.notify('_echo_agent/extension/stream',{event:'failed',stream,sequence:'1',error:structuredClone(observationCases[0].error)});
  await tick();writes.at(-1).done.resolve();await observation;
  assert.equal(sdk.invocationControllers.get('original'),controller);assert.equal(sdk.extensionStreamRoutes.size,1);
  const actual=sdk.notify('_echo_agent/extension/stream',{event:'complete',stream,sequence:'2',value:{kind:'agent',value:{event:'final_answer',data:{text:'actual'}}}});
  await tick();writes.at(-1).done.resolve();await actual;
  assert.equal(sdk.extensionStreamRoutes.size,0);assert.equal(sdk.invocationControllers.size,0);
 }finally{cx.close();}
});

test('late observation submission cannot reopen the same writer after its real terminal',async()=>{
 // The official scheduler serializes native writes. Isolate the public writer's
 // caller continuation with deferred notify completions, not a fake ACP reorder.
 const release=deferred(),entered=deferred(),frames=[];
 const sdk={async notify(method,params){frames.push(structuredClone(params));if(params.event==='failed'){entered.resolve();await release.promise;}}};
 const writer=new ExtensionStreamWriter(sdk,{id:'original-stream',generation:'1',kind:'stream'});
 const first=writer.failed(structuredClone(observationCases[0].error));
 try{
  await entered.promise;
  await writer.complete({kind:'agent',value:{event:'final_answer',data:{text:'original final'}}});
  release.resolve();await first;
  await assert.rejects(writer.chunk({kind:'agent',value:{event:'token',text:'must not revive'}}),/already closed/);
  assert.deepEqual(frames.map(item=>item.sequence),['1','2']);
 }finally{release.resolve();await first;}
});
