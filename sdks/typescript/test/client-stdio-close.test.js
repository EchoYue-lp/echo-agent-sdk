import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,mkdirSync,existsSync,readFileSync,writeFileSync,openSync,closeSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {EchoAgentClient} from '../dist/index.js';

// Transport fixture only: exact negotiated ACP initialize, then actual stdin EOF.
// No framework business behavior is simulated or claimed by these tests.
const CHILD=String.raw`
const fs=require('node:fs'),path=require('node:path');
const work=process.argv[1],mode=process.argv[2];let input='';
const limits=Object.fromEntries(["max_message_bytes", "max_stream_buffer_events", "max_replay_events", "max_structured_output_bytes", "max_outstanding_live_events", "max_stream_buffer_bytes", "max_replay_bytes", "max_open_handles", "max_registered_extensions", "max_extension_descriptor_bytes", "max_extension_payload_bytes", "max_extension_stream_bytes", "max_inflight_extension_invocations", "max_facade_resources", "max_facade_streams", "max_facade_page_items", "max_facade_operation_args"].map(name=>[name,'16']));limits.max_callback_concurrency=4;limits.callback_timeout={seconds:'30',nanos:0};

fs.writeFileSync(path.join(work,'pid'),String(process.pid));if(mode==='ignore-signal'||mode==='init-fail-ignore-signal')process.on('SIGTERM',()=>{});
process.stdin.setEncoding('utf8');process.stdin.on('data',chunk=>{input+=chunk;let n;while((n=input.indexOf('\n'))>=0){const row=JSON.parse(input.slice(0,n));input=input.slice(n+1);if(row.method==='initialize'){const hello=row.params.clientCapabilities._meta.echo_agent;process.stdout.write(JSON.stringify({jsonrpc:'2.0',id:row.id,result:{protocolVersion:1,agentCapabilities:{_meta:{echo_agent:{extension_protocol_version:1,contract_digest:hello.contract_digest,source_contract_digest:hello.source_contract_digest,features:[],capabilities:mode.startsWith('init-fail-')?[]:[{capability:'agent_lifecycle',required:false}],limits}}}}})+'\n');}}});
process.stdin.on('end',()=>{fs.writeFileSync(path.join(work,'eof'),'observed');if(mode==='exit'){process.exit(0);}const timer=setInterval(()=>{if(fs.existsSync(path.join(work,'release'))){clearInterval(timer);process.exit(0);}},10);});
`;
const ROOT=fileURLToPath(new URL('../../../target/typescript-stdio-close/',import.meta.url));
async function setup(mode){mkdirSync(ROOT,{recursive:true});const work=mkdtempSync(join(ROOT,mode+'-'));const sdk=await EchoAgentClient.spawn({hostCommand:process.execPath,args:['-e',CHILD,work,mode]});const child=sdk.process;return {work,sdk,child};}
async function until(predicate,ms=1500){const end=performance.now()+ms;while(!predicate()){assert.ok(performance.now()<end,'original child condition did not occur');await new Promise(resolve=>setTimeout(resolve,10));}}
const exited=child=>child.exitCode!==null||child.signalCode!==null;
function facts(context,extra={}){const output=join(context.work,'observed.json'),previous=existsSync(output)?JSON.parse(readFileSync(output,'utf8')):{};writeFileSync(output,JSON.stringify({...previous,pid:context.child.pid,eof:existsSync(join(context.work,'eof')),exit_code:context.child.exitCode,signal_code:context.child.signalCode,...extra},null,2));}
async function clean(context,waiters=[]){writeFileSync(join(context.work,'release'),'exit');await Promise.allSettled(waiters);if(!exited(context.child)){context.child.kill('SIGKILL');await new Promise(resolve=>context.child.once('exit',resolve));}facts(context);}

// Real official ACP peer. Diagnostics precede the first initialize response;
// the maintained metadata fixture and current digests define the SDK contract.
const DIAGNOSTIC_PEER = [
  "import fs from 'node:fs'; import path from 'node:path';",
  "import {Readable,Writable} from 'node:stream';",
  "import {agent,methods,ndJsonStream} from '@agentclientprotocol/sdk';",
  "const [work,kind,sdkRoot]=process.argv.slice(1);",
  "const payload=kind==='two_mib'?Buffer.alloc(2097152,0x42):kind==='unicode_crlf'?Buffer.from('诊断 中😀\\nCRLF\\r\\n尾\\n'):Buffer.alloc(0);",
  "const mark=name=>fs.writeFileSync(path.join(work,name+'.json'),JSON.stringify({pid:process.pid}));",
  "mark('peer-started'); let offset=0;",
  "while(offset<payload.length){const count=fs.writeSync(2,payload,offset,payload.length-offset);if(count<=0)throw Error('diagnostic write made no progress');offset+=count;}",
  "const document=JSON.parse(fs.readFileSync(path.join(sdkRoot,'contracts/sdk/fixtures/extension/v1/capability-meta-valid.json'),'utf8')).payload;",
  "Object.assign(document,JSON.parse(fs.readFileSync(path.join(sdkRoot,'sdks/shared/contract-digests.json'),'utf8')));",
  "const app=agent({name:'diagnostic-peer'}).onRequest(methods.agent.initialize,({params})=>{mark('peer-initialized');return {protocolVersion:params.protocolVersion,agentCapabilities:{_meta:{echo_agent:document}}};});",
  "const connection=app.connect(ndJsonStream(Writable.toWeb(process.stdout),Readable.toWeb(process.stdin)));",
  "await connection.closed; mark('peer-stopped');",
].join('\n');

const DIAGNOSTIC_DRIVER = [
  "import fs from 'node:fs'; import path from 'node:path'; import assert from 'node:assert/strict';",
  "import childProcess from 'node:child_process'; import {syncBuiltinESMExports} from 'node:module';",
  "import {EchoAgentClient} from './dist/index.js';",
  "const [work,kind,mode,sdkRoot,peerSource]=process.argv.slice(1); const callback=mode==='callback';",
  "const realSpawn=childProcess.spawn; let child,exited,stderrEnded=Promise.resolve();",
  "// Observe only the exact original child object for failure cleanup; never read its diagnostics.",
  "childProcess.spawn=(...args)=>{child=realSpawn(...args);exited=new Promise(resolve=>child.once('exit',resolve));if(callback&&child.stderr)stderrEnded=new Promise((resolve,reject)=>{if(child.stderr.readableEnded)resolve();else{child.stderr.once('end',resolve);child.stderr.once('error',reject);}});childProcess.spawn=realSpawn;syncBuiltinESMExports();return child;};syncBuiltinESMExports();",
  "let sdk,attempt,timer,failure,mainExit=null,mainSignal=null,cleanupSignal=null;const chunks=[];",
  "try{",
  " const options={hostCommand:process.execPath,args:['--input-type=module','-e',peerSource,work,kind,sdkRoot]};",
  " if(callback)options.onStderr=chunk=>chunks.push(chunk);",
  " attempt=EchoAgentClient.spawn(options);attempt.catch(()=>{});",
  " sdk=await Promise.race([attempt,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('default spawn did not initialize')),6000);})]);clearTimeout(timer);",
  " assert.equal(sdk.process,child,'test observation must retain the original Process');",
  " await sdk.close();if(callback)await stderrEnded;mainExit=child.exitCode;mainSignal=child.signalCode;",
  " assert.equal(mainExit,0);assert.equal(mainSignal,null);",
  "}catch(error){failure=error;}finally{",
  " clearTimeout(timer);childProcess.spawn=realSpawn;syncBuiltinESMExports();",
  " if(child&&child.exitCode===null&&child.signalCode===null){cleanupSignal='SIGKILL';child.kill('SIGKILL');await exited;}",
  " if(attempt)await attempt.catch(()=>{});",
  " fs.writeFileSync(path.join(work,'callback.raw'),Buffer.from(chunks.join(''),'utf8'));",
  " fs.writeFileSync(path.join(work,'diagnostic-receipt.json'),JSON.stringify({pid:child?.pid,mode,kind,mainExit,mainSignal,cleanupSignal,failure:failure?String(failure):null},null,2));",
  "}",
  "if(failure)throw failure;",
].join('\n');

for (const scenario of [
  {kind:'empty',mode:'default'},
  {kind:'two_mib',mode:'default'},
  {kind:'unicode_crlf',mode:'default'},
  {kind:'two_mib',mode:'callback'},
]) test('native diagnostic ownership '+scenario.mode+'/'+scenario.kind,{timeout:20000},()=>{
  mkdirSync(ROOT,{recursive:true});
  const work=mkdtempSync(join(ROOT,'diagnostics-'+scenario.mode+'-'+scenario.kind+'-'));
  const sdkRoot=fileURLToPath(new URL('../../../',import.meta.url));
  const cwd=fileURLToPath(new URL('../',import.meta.url));
  const stderrFile=join(work,'application-stderr.raw');
  const stderrDescriptor=openSync(stderrFile,'wx');
  let result;
  try{
    result=spawnSync(process.execPath,['--input-type=module','-e',DIAGNOSTIC_DRIVER,work,scenario.kind,scenario.mode,sdkRoot,DIAGNOSTIC_PEER],{
      cwd,stdio:['ignore','pipe',stderrDescriptor],timeout:15000,
    });
  }finally{closeSync(stderrDescriptor);}
  writeFileSync(join(work,'driver-stdout.raw'),result.stdout??Buffer.alloc(0));
  assert.equal(result.error,undefined);assert.equal(result.signal,null);assert.equal(result.status,0,readFileSync(stderrFile,'utf8'));
  const receipt=JSON.parse(readFileSync(join(work,'diagnostic-receipt.json'),'utf8'));
  assert.equal(receipt.mainExit,0);assert.equal(receipt.mainSignal,null);assert.equal(receipt.cleanupSignal,null);assert.equal(receipt.failure,null);
  assert.equal(JSON.parse(readFileSync(join(work,'peer-initialized.json'),'utf8')).pid,receipt.pid);
  assert.equal(JSON.parse(readFileSync(join(work,'peer-stopped.json'),'utf8')).pid,receipt.pid);
  const expected=scenario.kind==='two_mib'?Buffer.alloc(2097152,0x42):scenario.kind==='unicode_crlf'?Buffer.from('诊断 中😀\nCRLF\r\n尾\n'):Buffer.alloc(0);
  assert.deepEqual(result.stdout,Buffer.alloc(0),'ACP stdout must not leak to application stdout');
  assert.deepEqual(readFileSync(stderrFile),scenario.mode==='default'?expected:Buffer.alloc(0));
  assert.deepEqual(readFileSync(join(work,'callback.raw')),scenario.mode==='callback'?expected:Buffer.alloc(0));
  console.log('stdio diagnostic evidence:',work);
});

test('public close sends native stdin EOF and awaits normal original child exit',{timeout:10000},async()=>{
 const c=await setup('exit');let closing;try{closing=c.sdk.close();await until(()=>existsSync(join(c.work,'eof')));await closing;assert.equal(c.child.exitCode,0);assert.equal(c.child.signalCode,null);facts(c);console.log('stdio EOF evidence:',c.work);}finally{await clean(c,closing?[closing]:[]);}
});

test('concurrent and repeated close wait for the same child held after EOF',{timeout:10000},async()=>{
 const c=await setup('held');let one,two,first=false,second=false;try{one=c.sdk.close().then(()=>{first=true;});two=c.sdk.close().then(()=>{second=true;});await new Promise(resolve=>setImmediate(resolve));assert.equal(second,false,'a second close must not settle before original child exits');await until(()=>existsSync(join(c.work,'eof')));assert.equal(first,false);assert.equal(second,false);assert.equal(exited(c.child),false);writeFileSync(join(c.work,'release'),'exit');await Promise.all([one,two]);assert.equal(c.child.exitCode,0);await c.sdk.close();facts(c,{first,second});console.log('stdio coalesced close evidence:',c.work);}finally{await clean(c,[one,two].filter(Boolean));}
});

test('closing an already signalled child observes existing exit without a new waiter',{timeout:10000},async()=>{
 const c=await setup('held');try{const done=new Promise(resolve=>c.child.once('exit',resolve));c.child.kill('SIGTERM');await done;assert.equal(c.child.signalCode,'SIGTERM');await Promise.race([c.sdk.close(),new Promise((_,reject)=>{const t=setTimeout(()=>reject(new Error('close waited for an already exited child')),1000);t.unref();})]);facts(c);}finally{await clean(c);}
});

test('five second fallback terminates a child that observes EOF but never exits',{timeout:10000},async()=>{
 const c=await setup('held');try{const start=performance.now();await c.sdk.close();const elapsed=performance.now()-start;assert.ok(elapsed>=4900&&elapsed<8000,`fallback deadline changed: ${elapsed}`);assert.ok(existsSync(join(c.work,'eof')));assert.equal(c.child.signalCode,'SIGTERM');facts(c,{elapsed_ms:elapsed});console.log('stdio fallback evidence:',c.work);}finally{await clean(c);}
});

test('a failed connection close remains observable and retries the same original child',{timeout:10000},async()=>{
 const c=await setup('exit');const original=c.sdk.connection.close.bind(c.sdk.connection);let attempts=0;try{c.sdk.connection.close=()=>{if(++attempts===1)throw new Error('injected original connection close failure');return original();};await assert.rejects(c.sdk.close(),/injected original connection close failure/);assert.equal(exited(c.child),false);assert.equal(existsSync(join(c.work,'eof')),false);await c.sdk.close();assert.equal(c.child.exitCode,0);assert.equal(existsSync(join(c.work,'eof')),true);assert.equal(attempts,2);facts(c,{attempts});console.log('stdio same-owner retry evidence:',c.work);}finally{c.sdk.connection.close=original;await clean(c);}
});


test('synchronous cancellation callback reentry shares the already published close owner',{timeout:10000},async()=>{
 const c=await setup('held');const original=c.sdk.connection.close.bind(c.sdk.connection);let calls=0,outer,reentrant,outerDone=false,reentrantDone=false;const controller=new AbortController();try{c.sdk.connection.close=()=>{calls++;return original();};controller.signal.addEventListener('abort',()=>{reentrant=c.sdk.close().then(()=>{reentrantDone=true;});},{once:true});c.sdk.invocationControllers.set('reentrant-native-fixture',controller);outer=c.sdk.close().then(()=>{outerDone=true;});await until(()=>existsSync(join(c.work,'eof')));assert.equal(calls,1,'reentry must not start a second transport close');assert.ok(reentrant);assert.equal(outerDone,false);assert.equal(reentrantDone,false);writeFileSync(join(c.work,'release'),'exit');await Promise.all([outer,reentrant]);assert.equal(c.child.exitCode,0);facts(c,{calls,outerDone,reentrantDone});console.log('stdio synchronous reentry evidence:',c.work);}finally{c.sdk.connection.close=original;await clean(c,[outer,reentrant].filter(Boolean));}
});


test('close synchronously rejects same-tick public request admission',{timeout:10000},async()=>{
 const c=await setup('held');let closing;try{closing=c.sdk.close();assert.equal(c.sdk.connection.signal.aborted,true,'the original connection must close synchronously');await assert.rejects(c.sdk.request('fixture/must-not-enter',{}),/closed/);await until(()=>existsSync(join(c.work,'eof')));writeFileSync(join(c.work,'release'),'exit');await closing;assert.equal(c.child.exitCode,0);facts(c,{same_tick_request_rejected:true});}finally{await clean(c,closing?[closing]:[]);}
});

test('unexpected stdin end callback error after actual child exit rejects the shared close owner',{timeout:10000},async()=>{
 const c=await setup('exit');const original=c.child.stdin.end.bind(c.child.stdin);let release,closing,two,settled=false;try{c.child.stdin.end=callback=>original(error=>{assert.ok(error===undefined||error===null);release=()=>callback(Object.assign(new Error('late original stdin EIO'),{code:'EIO'}));});closing=c.sdk.close();two=c.sdk.close();closing.then(()=>{settled=true;},()=>{settled=true;});await until(()=>exited(c.child)&&!!release);assert.equal(settled,false,'child exit must not discard the original EOF completion');release();await assert.rejects(closing,/late original stdin EIO/);await assert.rejects(two,/late original stdin EIO/);c.child.stdin.end=original;await c.sdk.close();assert.equal(c.child.exitCode,0);facts(c,{unexpected_error_observed:true});}finally{c.child.stdin.end=original;if(release&&!settled)release();await clean(c,[closing,two].filter(Boolean));}
});

test('expected EPIPE completion is harmless but still waits for native exit',{timeout:10000},async()=>{
 const c=await setup('held');const original=c.child.stdin.end.bind(c.child.stdin);let closing,settled=false;try{c.child.stdin.end=callback=>original(()=>callback(Object.assign(new Error('closed child read side'),{code:'EPIPE'})));closing=c.sdk.close().then(()=>{settled=true;});await until(()=>existsSync(join(c.work,'eof')));assert.equal(settled,false);assert.equal(exited(c.child),false);writeFileSync(join(c.work,'release'),'exit');await closing;assert.equal(c.child.exitCode,0);facts(c,{expected_pipe_error_observed:true});}finally{c.child.stdin.end=original;await clean(c,closing?[closing]:[]);}
});

test('a signal-sent flag is not an exit receipt for an original child still alive',{timeout:10000},async()=>{
 const c=await setup('ignore-signal');let closing,settled=false;try{c.child.kill('SIGTERM');assert.equal(c.child.killed,true);assert.equal(exited(c.child),false);closing=c.sdk.close().then(()=>{settled=true;});await until(()=>existsSync(join(c.work,'eof')));assert.equal(settled,false);assert.equal(exited(c.child),false);writeFileSync(join(c.work,'release'),'exit');await closing;assert.equal(c.child.exitCode,0);facts(c,{signal_sent_while_still_live:true});}finally{await clean(c,closing?[closing]:[]);}
});


for(const mode of ['init-fail-held','init-fail-ignore-signal']) test(`initialization failure retains original child until EOF and native exit: ${mode}`,{timeout:10000},async()=>{
 mkdirSync(ROOT,{recursive:true});const work=mkdtempSync(join(ROOT,mode+'-'));let settled=false,error,forcedCleanup=false,pid;
 const alive=()=>{try{process.kill(pid,0);return true;}catch{return false;}};
 const pending=EchoAgentClient.spawn({hostCommand:process.execPath,args:['-e',CHILD,work,mode]}).then(()=>{settled=true;throw Error('malformed advertisement unexpectedly accepted');},failure=>{error=failure;settled=true;});
 try {
  await until(()=>existsSync(join(work,'pid')));pid=Number(readFileSync(join(work,'pid'),'utf8'));
  await until(()=>settled||existsSync(join(work,'eof')));
  assert.equal(settled,false,'initialization error must await the original child exit');
  assert.equal(existsSync(join(work,'eof')),true,'initialization cleanup must deliver actual stdin EOF');
  assert.equal(alive(),true,'held fixture must still be physically alive');
  writeFileSync(join(work,'release'),'exit');await pending;
  assert.equal(error.code,'extension_capability_mismatch');assert.equal(error.operation,'initialize');assert.equal(error.retryable,'never');
  assert.equal(alive(),false,'initialization rejection leaked the original child');
 } finally {
  writeFileSync(join(work,'release'),'cleanup');
  if(pid&&alive()){await new Promise(resolve=>setTimeout(resolve,100));if(alive()){forcedCleanup=true;process.kill(pid,'SIGKILL');}}
  await pending.catch(()=>{});
  writeFileSync(join(work,'observed.json'),JSON.stringify({mode,pid,settled,error:error?{code:error.code,operation:error.operation,retryable:error.retryable}:null,stdin_eof:existsSync(join(work,'eof')),forced_cleanup:forcedCleanup},null,2));
 }
});
