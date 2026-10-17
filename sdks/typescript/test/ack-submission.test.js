import test from 'node:test';
import assert from 'node:assert/strict';
import { client } from '@agentclientprotocol/sdk';
import { EchoAgentClient, RunHandle } from '../dist/client.js';
const deferred=()=>{let resolve,reject;const promise=new Promise((a,b)=>{resolve=a;reject=b});return {promise,resolve,reject};};
const tick=()=>new Promise(resolve=>setImmediate(resolve));

test('official serialized writer: low successful ACK cannot commit pending terminal ACK',async()=>{
 const writes=[];
 const stream={writable:new WritableStream({write(message){const done=deferred();writes.push({message,done});return done.promise;}}),readable:new ReadableStream({})};
 const cx=client({name:'ack-test'}).connect(stream);
 const sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=32;sdk.connection=cx;sdk.closed=false;
 const handle={id:'stream',generation:'1',kind:'stream'};sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);
 feed.lastEventSequence=1n;feed.lastLiveSequence=1n;feed.liveRanges=[{base:1n,end:1n}];feed.replayRanges=[{base:0n,end:10n}];feed.replayTerminalSequence=10n;
 try {
  const low=sdk.acknowledge(handle,'2');await tick();assert.equal(writes.length,1);
  const high=sdk.acknowledge(handle,'10');high.catch(()=>{});
  writes[0].done.resolve();await low;await tick();assert.equal(writes.length,2);
  writes[1].done.reject(new Error('native reject terminal ACK'));await assert.rejects(high);
  assert.equal(feed.acknowledgedSequence,2n);assert.equal(feed.queue.closed,false);
  assert.equal(feed.lastEventSequence,2n);
 }finally{cx.close();}
});

for(const first of [2,10])test(`official writer ${first} success then other ACK failure retains exact successful floor`,async()=>{
 const writes=[];const cx=client({name:'ack-test'}).connect({writable:new WritableStream({write(message){const done=deferred();writes.push({message,done});return done.promise;}}),readable:new ReadableStream({})});
 const sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=32;sdk.connection=cx;sdk.closed=false;
 const handle={id:'stream',generation:'1',kind:'stream'};sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);
 feed.lastEventSequence=1n;feed.lastLiveSequence=1n;feed.liveRanges=[{base:1n,end:1n}];feed.replayRanges=[{base:0n,end:10n}];feed.replayTerminalSequence=10n;
 try{
  const one=sdk.acknowledge(handle,String(first));await tick();
  const two=sdk.acknowledge(handle,String(first===2?10:2));two.catch(()=>{});
  assert.equal(feed.acknowledgedSequence,undefined);assert.equal(feed.lastEventSequence,1n);
  writes[0].done.resolve();await one;await tick();
  assert.equal(feed.acknowledgedSequence,BigInt(first));assert.equal(feed.queue.closed,first===10);
  writes[1].done.reject(new Error('native write outcome unknown'));
  await assert.rejects(two,error=>error.details.ack_submission==='outcome_unknown');
  assert.equal(feed.acknowledgedSequence,BigInt(first));assert.equal(feed.queue.closed,first===10);
 }finally{cx.close();}
});

test('official failed send preserves queue prefix and does not accept a future ACK',async()=>{
 const writes=[];const cx=client({name:'ack-test'}).connect({writable:new WritableStream({write(message){const done=deferred();writes.push({message,done});return done.promise;}}),readable:new ReadableStream({})});
 const sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=32;sdk.connection=cx;sdk.closed=false;
 const handle={id:'stream',generation:'1',kind:'stream'};sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);
 feed.lastEventSequence=2n;feed.lastLiveSequence=2n;feed.liveRanges=[{base:1n,end:2n}];feed.queue.push({envelope:{sequence:'1'}});feed.queue.push({envelope:{sequence:'2'}});
 try{
  const ack=sdk.acknowledge(handle,'2');ack.catch(()=>{});await tick();writes[0].done.reject(new Error('native send failed'));await assert.rejects(ack);
  assert.equal(feed.acknowledgedSequence,undefined);assert.equal(feed.lastEventSequence,2n);assert.equal(feed.queue.closed,false);
  feed.queue.end();const values=[];for await(const value of feed.queue)values.push(value.envelope.sequence);assert.deepEqual(values,['1','2']);
 }finally{cx.close();}
});

test('official writer sends the same immutable stream identity that it commits', async()=>{
 const writes=[];const cx=client({name:'ack-snapshot'}).connect({writable:new WritableStream({write(message){const done=deferred();writes.push({message,done});return done.promise;}}),readable:new ReadableStream({})});
 const sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=32;sdk.connection=cx;sdk.closed=false;
 const handle={id:'stream',generation:'1',kind:'stream'};sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);
 feed.lastEventSequence=1n;feed.lastLiveSequence=1n;feed.liveRanges=[{base:1n,end:1n}];feed.replayRanges=[{base:0n,end:10n}];feed.replayTerminalSequence=10n;
 try{
  const pending=sdk.acknowledge(handle,'10');
  handle.id='different-stream';handle.generation='2';
  await tick();assert.equal(writes.length,1);
  const actual=writes[0].message.params.ack;
  writes[0].done.resolve();await pending;
  assert.equal(actual.stream.id,'stream');assert.equal(actual.stream.generation,'1');
  assert.equal(feed.acknowledgedSequence,BigInt(actual.last_processed_sequence));
 }finally{cx.close();}
});


test('official replay retention bridge commits only a successful returned cursor', async()=>{
 const handle={id:'stream',generation:'1',kind:'stream'};
 const envelope={stream_id:'stream',sequence:'6',event_id:'event-6',turn_id:'turn',content_hash:'sha256:'+'0'.repeat(64),schema_version:1,timestamp:{unix_seconds:'0',nanos:0},payload:{event_type:'delta',data:{kind:'null'}}};
 let response={requested_after_sequence:'1',events:[envelope],gap:{from_sequence:'2',to_sequence:'5',snapshot_watermark:'5',reason:'retention floor'},next_cursor:{stream_id:'stream',last_processed_sequence:'6'}};
 const writes=[];let reader;
 const cx=client({name:'retention-replay'}).connect({readable:new ReadableStream({start(controller){reader=controller;}}),writable:new WritableStream({write(message){if(message.id!==undefined){reader.enqueue({jsonrpc:'2.0',id:message.id,result:response});return;}const done=deferred();writes.push({message,done});return done.promise;}})});
 const sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=32;sdk.connection=cx;sdk.closed=false;
 sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);feed.lastEventSequence=1n;feed.lastLiveSequence=1n;feed.liveRanges=[{base:1n,end:1n}];feed.queue.push({envelope:{sequence:'1'}});
 try{
  const run=new RunHandle(sdk,{id:'run',generation:'1',kind:'run'},handle);
  const retention=response;response={requested_after_sequence:'5',events:[],next_cursor:{stream_id:'stream',last_processed_sequence:'5'}};await run.replay('5',8);response=retention;
  await run.replay('1',8);assert.equal(feed.lastEventSequence,1n);assert.equal(feed.acknowledgedSequence,undefined);
  const interior=sdk.acknowledge(handle,'3');await tick();writes[0].done.resolve();await interior;assert.equal(feed.acknowledgedSequence,undefined);
  const ack=sdk.acknowledge(handle,'6');await tick();assert.equal(feed.lastEventSequence,1n);assert.equal(feed.acknowledgedSequence,undefined);
  writes[1].done.resolve();await ack;assert.equal(feed.acknowledgedSequence,6n);assert.equal(feed.lastEventSequence,6n);
  feed.queue.end();const values=[];for await(const value of feed.queue)values.push(value);assert.deepEqual(values,[]);
 }finally{cx.close();}
});


test('gap plus empty after-gap page never grants an absent ACK cursor',async()=>{
 const writes=[];const cx=client({name:'gap-page'}).connect({writable:new WritableStream({write(message){const done=deferred();writes.push({message,done});return done.promise;}}),readable:new ReadableStream({})});
 const sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=32;sdk.connection=cx;sdk.closed=false;
 const handle={id:'stream',generation:'1',kind:'stream'};sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);feed.lastEventSequence=12n;feed.lastGapWatermark=12n;feed.lastLiveSequence=1n;feed.liveRanges=[{base:1n,end:1n}];
 sdk.observeReplay(handle,{requested_after_sequence:'10',events:[],next_cursor:{stream_id:'stream',last_processed_sequence:'10'}});
 try{const ack=sdk.acknowledge(handle,'10');await tick();writes[0].done.resolve();await ack;assert.equal(feed.acknowledgedSequence,undefined);assert.equal(feed.lastEventSequence,12n);}finally{cx.close();}
});

for(const order of ['empty_gap_tail','reverse_gap','reverse_plain','disconnected'])test(`public replay connected coverage ${order}`,async()=>{
 const handle={id:'stream',generation:'1',kind:'stream'};
 const event=seq=>({stream_id:'stream',sequence:String(seq),event_id:`event-${seq}`,turn_id:'turn',content_hash:'sha256:'+'0'.repeat(64),schema_version:1,timestamp:{unix_seconds:'0',nanos:0},payload:{event_type:'delta',data:{kind:'null'}}});
 const page=(after,end,terminal=false)=>({requested_after_sequence:String(after),events:Array.from({length:end-after},(_,i)=>({...event(after+i+1),payload:{event_type:terminal&&after+i+1===end?'final_answer':'delta',data:{kind:'null'}}})),next_cursor:{stream_id:'stream',last_processed_sequence:String(end)}});
 const gap={...page(5,6),requested_after_sequence:'1',gap:{from_sequence:'2',to_sequence:'5',snapshot_watermark:'5',reason:'retention floor'}};
 const pages={empty_gap_tail:[page(6,6),gap,page(6,7,true)],reverse_gap:[page(6,8,true),gap],reverse_plain:[page(3,5,true),page(1,3)],disconnected:[page(8,10,true),gap]}[order];
 let response;let reader;const writes=[];const cx=client({name:'replay-closure'}).connect({readable:new ReadableStream({start(c){reader=c;}}),writable:new WritableStream({write(message){if(message.id!==undefined){reader.enqueue({jsonrpc:'2.0',id:message.id,result:response});return;}const done=deferred();writes.push(done);return done.promise;}})});
 const sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=32;sdk.connection=cx;sdk.closed=false;sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);feed.lastEventSequence=1n;feed.lastLiveSequence=1n;feed.liveRanges=[{base:1n,end:1n}];feed.queue.push({envelope:{sequence:'1'}});
 try{const run=new RunHandle(sdk,{id:'run',generation:'1',kind:'run'},handle);for(response of pages)await run.replay(response.requested_after_sequence,8);
 const target={empty_gap_tail:7n,reverse_gap:8n,reverse_plain:5n,disconnected:10n}[order];const ack=sdk.acknowledge(handle,String(target));await tick();assert.equal(feed.acknowledgedSequence,undefined);assert.equal(feed.queue.closed,false);writes[0].resolve();await ack;
 if(order==='disconnected'){assert.equal(feed.acknowledgedSequence,undefined);assert.equal(feed.queue.closed,false);}else{assert.equal(feed.acknowledgedSequence,target);assert.equal(feed.queue.closed,true);const values=[];for await(const x of feed.queue)values.push(x);assert.deepEqual(values,[]);}
 }finally{cx.close();}
});


test('successful earlier-missing ACK cannot retire prefix after failed recovery and real live continuation',async()=>{
 const handle={id:'stream',generation:'1',kind:'stream'};
 const event=seq=>({stream_id:'stream',sequence:String(seq),event_id:`event-${seq}`,turn_id:'turn',content_hash:'sha256:'+'0'.repeat(64),schema_version:1,timestamp:{unix_seconds:'0',nanos:0},payload:{event_type:'delta',data:{kind:'null'}}});
 const response={requested_after_sequence:'1',events:[event(6)],gap:{from_sequence:'2',to_sequence:'5',snapshot_watermark:'5',reason:'retention floor'},next_cursor:{stream_id:'stream',last_processed_sequence:'6'}};
 let reader;const writes=[];const cx=client({name:'live-facts'}).connect({readable:new ReadableStream({start(c){reader=c;}}),writable:new WritableStream({write(message){if(message.id!==undefined){reader.enqueue({jsonrpc:'2.0',id:message.id,result:response});return;}writes.push(message);}})});
 const sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=32;sdk.connection=cx;sdk.closed=false;sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);sdk.publishFirstEvent(handle,event(1));
 const native=sdk.notify.bind(sdk);const gate=deferred();let enteredResolve;const entered=new Promise(resolve=>{enteredResolve=resolve;});
 sdk.notify=async(method,params)=>{await native(method,params);if(params?.ack?.last_processed_sequence==='6'){enteredResolve();await gate.promise;}};
 try{await new RunHandle(sdk,{id:'run',generation:'1',kind:'run'},handle).replay('1');const pending=sdk.acknowledge(handle,'6');pending.catch(()=>{});await entered;sdk.publishFirstEvent(handle,event(7));gate.reject(new Error('unknown native completion'));await assert.rejects(pending);
 const interior=sdk.acknowledge(handle,'3');await interior;assert.equal(writes.length,2);assert.equal(feed.acknowledgedSequence,undefined);assert.equal(feed.lastEventSequence,7n);feed.queue.end();const values=[];for await(const x of feed.queue)values.push(x.envelope.sequence);assert.deepEqual(values,['1','7']);
 }finally{sdk.notify=native;cx.close();}
});

// The peer sees the original official ACK frame before completing its write.
const liveEnvelope = sequence => ({stream_id:'stream',sequence:String(sequence),event_id:`live-${sequence}`,turn_id:'turn',content_hash:'sha256:'+'0'.repeat(64),schema_version:1,timestamp:{unix_seconds:'0',nanos:0},payload:{event_type:'delta',data:{kind:'null'}}});
async function admissionFixture(capacity=1) {
 const writes=[];let peer;let sdk;
 const app=client({name:'ack-admission'}).onNotification('_echo_agent/event',p=>p,async({params})=>sdk.publishFirstEvent(params.stream,params.envelope));
 const cx=app.connect({readable:new ReadableStream({start(c){peer=c;}}),writable:new WritableStream({write(message){if(message.id!==undefined){peer.enqueue({jsonrpc:'2.0',id:message.id,result:{independent:true}});return;}const done=deferred();writes.push({message,done});return done.promise;}})});
 sdk=Object.create(EchoAgentClient.prototype);sdk.events=new Map();sdk.queueCapacity=capacity;sdk.connection=cx;sdk.closed=false;
 const handle={id:'stream',generation:'7',kind:'stream'};sdk.eventsFor(handle.id,handle);const feed=sdk.events.get(handle.id);
 sdk.publishFirstEvent(handle,liveEnvelope(1));await tick();
 const gap={stream:handle,gap:{from_sequence:'2',to_sequence:'2',snapshot_watermark:'2',reason:'window pause'}};
 feed.lastEventSequence=2n;feed.lastGapWatermark=2n;feed.queue.push(gap,true);
 return {sdk,cx,handle,feed,writes,peer,gap,async close(){for(const w of writes)w.done.resolve();await tick();feed.queue.end();cx.close();}};
}
async function untilAdmission(condition){for(let i=0;i<1000;i++){if(condition())return;await tick();}assert.fail('official notification did not reach the expected admission state');}
test('peer-visible ACK frees capacity only on native success without losing resumed event',async()=>{
 const f=await admissionFixture();let ack;
 try{
  ack=f.sdk.acknowledge(f.handle,'2');await untilAdmission(()=>f.writes.length===1);
  assert.deepEqual(f.writes[0].message.params.ack,{stream:f.handle,last_processed_sequence:'2'});
  f.peer.enqueue({jsonrpc:'2.0',method:'_echo_agent/event',params:{stream:f.handle,envelope:liveEnvelope(3)}});
  await untilAdmission(()=>f.feed.lastEventSequence===3n);await tick();
  assert.equal(f.feed.queue.closed,false,'pending visible ACK must not permanently overflow its resumed event');
  assert.equal(f.feed.acknowledgedSequence,undefined);
  f.writes[0].done.resolve();await ack;await tick();
  assert.equal(f.feed.queue.closed,false);assert.equal(f.feed.acknowledgedSequence,2n);
  const iterator=f.feed.queue[Symbol.asyncIterator]();
  assert.deepEqual((await iterator.next()).value,f.gap);
  assert.equal((await iterator.next()).value.envelope.sequence,'3');
 }finally{await f.close();if(ack)await ack.catch(()=>{});}
});

for(const outcome of ['failure','consumer','close'])test(`pending ACK admission wakes on ${outcome} without borrowing its IO owner`,async()=>{
 const f=await admissionFixture();const ack=f.sdk.acknowledge(f.handle,'2');ack.catch(()=>{});
 try{
  await untilAdmission(()=>f.writes.length===1);
  f.peer.enqueue({jsonrpc:'2.0',method:'_echo_agent/event',params:{stream:f.handle,envelope:liveEnvelope(3)}});
  await untilAdmission(()=>f.feed.lastEventSequence===3n);await tick();
  assert.equal(f.feed.queue.closed,false);assert.equal(f.feed.acknowledgedSequence,undefined);
  if(outcome==='failure'){
   f.writes[0].done.reject(new Error('native ACK unknown'));
   await assert.rejects(ack,error=>error.details.ack_submission==='outcome_unknown');
   await f.feed.deliveryTail;
   assert.equal(f.feed.acknowledgedSequence,undefined);
   assert.equal(f.feed.queue.closed,true);
   assert.equal(f.feed.queue.items[0].value.envelope.sequence,'1');
   assert.equal(f.feed.queue.items.at(-1).error.code,'backpressure');
  }else if(outcome==='consumer'){
   const iterator=f.feed.queue[Symbol.asyncIterator]();
   assert.equal((await iterator.next()).value.envelope.sequence,'1');
   await f.feed.deliveryTail;
   assert.equal(f.feed.queue.closed,false);assert.equal(f.feed.acknowledgedSequence,undefined);
   assert.deepEqual((await iterator.next()).value,f.gap);
   assert.equal((await iterator.next()).value.envelope.sequence,'3');
   f.writes[0].done.resolve();await ack;
  }else{
   f.feed.queue.end();await f.feed.deliveryTail;
   assert.equal(f.feed.acknowledgedSequence,undefined);
   assert.equal(f.feed.queue.items.some(x=>x.value?.envelope?.sequence==='3'),false);
   f.writes[0].done.resolve();await ack;
  }
 }finally{await f.close();await ack.catch(()=>{});}
});

test('unobserved future ACK cannot postpone the original overflow policy',async()=>{
 const f=await admissionFixture();const ack=f.sdk.acknowledge(f.handle,'3');
 try{
  await untilAdmission(()=>f.writes.length===1);
  f.peer.enqueue({jsonrpc:'2.0',method:'_echo_agent/event',params:{stream:f.handle,envelope:liveEnvelope(3)}});
  await untilAdmission(()=>f.feed.queue.closed);
  f.writes[0].done.resolve();await ack;
  assert.equal(f.feed.acknowledgedSequence,undefined);
  assert.equal(f.feed.queue.items[0].value.envelope.sequence,'1');
 }finally{await f.close();await ack.catch(()=>{});}
});

test('later real successful ACK covers held data before its delayed delivery resumes',async()=>{
 const f=await admissionFixture();const gate=deferred();const visible=deferred();
 const native=f.sdk.notify.bind(f.sdk);
 f.sdk.notify=async(method,params)=>{await native(method,params);if(params.ack.last_processed_sequence==='2'){visible.resolve();await gate.promise;}};
 let low,high;
 try{
  low=f.sdk.acknowledge(f.handle,'2');await untilAdmission(()=>f.writes.length===1);
  f.writes[0].done.resolve();await visible.promise;
  f.peer.enqueue({jsonrpc:'2.0',method:'_echo_agent/event',params:{stream:f.handle,envelope:liveEnvelope(3)}});
  await untilAdmission(()=>f.feed.lastEventSequence===3n);await tick();
  high=f.sdk.acknowledge(f.handle,'3');await untilAdmission(()=>f.writes.length===2);
  f.writes[1].done.resolve();await high;await f.feed.deliveryTail;
  assert.equal(f.feed.acknowledgedSequence,3n);
  assert.equal(f.feed.queue.items.some(x=>x.value?.envelope),false);
  gate.resolve();await low;assert.equal(f.feed.acknowledgedSequence,3n);
 }finally{gate.resolve();await f.close();await Promise.allSettled([low,high].filter(Boolean));}
});


test('pending feed admission does not block another feed or a duplicate notification',async()=>{
 const f=await admissionFixture();const ack=f.sdk.acknowledge(f.handle,'2');
 try{
  await untilAdmission(()=>f.writes.length===1);
  f.peer.enqueue({jsonrpc:'2.0',method:'_echo_agent/event',params:{stream:f.handle,envelope:liveEnvelope(3)}});
  await untilAdmission(()=>f.feed.lastEventSequence===3n);await tick();const tail=f.feed.deliveryTail;
  f.peer.enqueue({jsonrpc:'2.0',method:'_echo_agent/event',params:{stream:f.handle,envelope:liveEnvelope(3)}});
  const other={id:'other',generation:'8',kind:'stream'};
  f.peer.enqueue({jsonrpc:'2.0',method:'_echo_agent/event',params:{stream:other,envelope:{...liveEnvelope(1),stream_id:'other'}}});
  await untilAdmission(()=>f.sdk.events.has('other'));
  assert.equal(f.sdk.events.get('other').queue.items[0].value.envelope.sequence,'1');
  assert.equal(f.feed.deliveryTail,tail);assert.equal(f.feed.acknowledgedSequence,undefined);
  f.writes[0].done.resolve();await ack;await f.feed.deliveryTail;
  assert.equal(f.feed.queue.closed,false);
 }finally{await f.close();await ack.catch(()=>{});}
});
