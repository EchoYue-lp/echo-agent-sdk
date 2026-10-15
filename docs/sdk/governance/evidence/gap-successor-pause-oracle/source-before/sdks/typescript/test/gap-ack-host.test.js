import test from "node:test";
import assert from "node:assert/strict";
import { spawn, execFile } from "node:child_process";
import { promisify } from "node:util";
import { mkdtemp, access, readFile, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { EchoAgentClient } from "../dist/index.js";

const fixture = fileURLToPath(new URL("../../shared/testing/gap_ack_fixture.py", import.meta.url));
const proxy = fileURLToPath(new URL("../../shared/testing/gap_ack_host_proxy.py", import.meta.url));
const realHost = process.env.ECHO_AGENT_SDK_HOST;

async function waitFor(predicate) {
  const deadline = Date.now() + 10000;
  while (!(await predicate())) {
    assert.ok(Date.now() < deadline, "real Host scenario did not reach its barrier");
    await new Promise(resolve => setTimeout(resolve, 10));
  }
}
async function exists(path) { try { await access(path); return true; } catch { return false; } }
async function notifications(work) {
  const text = await readFile(join(work, "notifications.jsonl"), "utf8");
  return text.slice(0, text.lastIndexOf("\n") + 1).split("\n").filter(Boolean).map(line => JSON.parse(line))
    .filter(value => ["_echo_agent/event", "_echo_agent/gap"].includes(value.method));
}

for (const mode of ["window", "oversized"]) {
 for (const consume of [false, true]) {
  test(`real Host ${mode} ${consume ? "consumer" : "raw"} gap ACK resumes strictly after snapshot watermark`, {
    skip: !realHost, timeout: 30000,
  }, async () => {
    const work = await mkdtemp(join(tmpdir(), "echo-sdk-gap-ts-"));
    const python = process.env.PYTHON ?? "python3";
    const model = spawn(python, [fixture, "--work", work, "--mode", mode]);
    let sdk;
    try {
      await waitFor(() => exists(join(work, "ready")));
      sdk = await EchoAgentClient.spawn({ hostCommand: python, args: [proxy, realHost, join(work, "host.json")],
        env: { ...process.env, NO_PROXY: "127.0.0.1,localhost", no_proxy: "127.0.0.1,localhost", RUST_LOG: "error" } });
      const agent = await sdk.createAgent();
      const session = await agent.createSession();
      const run = await session.startRun({ text: "gap ACK test 中文🌊" });
      await waitFor(() => exists(join(work, "parked")));
      const before = await notifications(work);
      const gap = before.find(value => value.method === "_echo_agent/gap").params.gap;
      const watermark = BigInt(gap.snapshot_watermark);
      assert.equal(before.length, mode === "window" ? 2 : 1);
      let iterator;
      if (consume) {
        iterator = run.events[Symbol.asyncIterator]();
        let item = (await iterator.next()).value;
        if (mode === "window") {
          assert.equal(item.envelope.sequence, "1");
          item = (await iterator.next()).value;
        }
        assert.equal(item.gap.snapshot_watermark, watermark.toString());
      }
      await sdk.acknowledge(run.stream, (watermark + 1n).toString());
      const snapshot = await run.get();
      assert.equal((await notifications(work)).length, before.length, "future ACK must not resume delivery");
      assert.ok(BigInt(snapshot.last_sequence) >= watermark);
      const replay = await run.replay(watermark.toString(), 64);
      assert.ok(replay.events.length > 0);
      assert.ok(replay.events.every(event => BigInt(event.envelope.sequence) > watermark));
      await sdk.acknowledge(run.stream, watermark.toString());
      if (iterator) {
        const resumed = (await iterator.next()).value;
        assert.ok(BigInt(resumed.gap?.from_sequence ?? resumed.envelope.sequence) > watermark);
      }
      await waitFor(async () => (await notifications(work)).length > before.length);
      const after = (await notifications(work)).slice(before.length);
      assert.ok(after.every(value => BigInt(value.params.gap?.from_sequence ?? value.params.envelope.sequence) > watermark),
        "Host must not re-send snapshot-covered events or gaps, even if client deduplicates them");
      // Reordered/late ACK must neither roll back the cursor nor re-publish covered events.
      const announced = (await notifications(work)).length;
      await sdk.acknowledge(run.stream, watermark.toString());
      await run.get();
      assert.equal((await notifications(work)).length, announced, "late ACK cannot retract the next gap");
      await writeFile(join(work, "release"), "");
      assert.equal((await run.wait()).settled, true);
      assert.equal((await run.get()).status, "completed");
      const continuation = await run.replay(replay.next_cursor.last_processed_sequence, 64);
      assert.ok(continuation.events.length > 0);
      assert.ok(continuation.events.every(event => BigInt(event.envelope.sequence) > watermark));
      assert.ok((await notifications(work)).slice(before.length).every(value =>
        BigInt(value.params.gap?.from_sequence ?? value.params.envelope.sequence) > watermark));
      if (iterator) await iterator.return();
      await session.close();
      await agent.close();
    } finally {
      if (sdk) await sdk.close();
      model.kill();
      await new Promise(resolve => model.exitCode === null ? model.once("exit", resolve) : resolve());
      await rm(work, { recursive: true, force: true });
    }
  });
}
}


for (const pageSize of [1, 64]) {
  test(`real Host replay-consumed cursor ACK page size ${pageSize}`, {skip: !realHost, timeout: 30000}, async () => {
    const work = await mkdtemp(join(tmpdir(), "echo-sdk-page-ts-"));
    const python = process.env.PYTHON ?? "python3";
    const model = spawn(python, [fixture, "--work", work, "--mode", "window"]);
    let sdk;
    try {
      await waitFor(() => exists(join(work, "ready")));
      sdk = await EchoAgentClient.spawn({hostCommand: python, args: [proxy, realHost, join(work, "host.json")],
        env: {...process.env, NO_PROXY: "127.0.0.1,localhost", no_proxy: "127.0.0.1,localhost", RUST_LOG: "error"}});
      const agent = await sdk.createAgent();
      const session = await agent.createSession();
      const run = await session.startRun({text: "replay consumed cursor 中文🌊"});
      await waitFor(() => exists(join(work, "parked")));
      const before = await notifications(work);
      const gap = before.find(value => value.method === "_echo_agent/gap").params.gap;
      const snapshot = await run.get();
      const parkedTail = BigInt(snapshot.last_sequence);
      let cursor = "0";
      const consumed = new Map();
      let pages = 0;
      while (BigInt(cursor) < parkedTail) {
        const page = await run.replay(cursor, pageSize);
        assert.ok(page.events.length > 0);
        for (const event of page.events) {
          assert.equal(BigInt(event.envelope.sequence), BigInt(cursor) + 1n);
          consumed.set(event.envelope.sequence, event.envelope.event_id);
          cursor = event.envelope.sequence;
        }
        assert.equal(page.next_cursor.last_processed_sequence, cursor);
        pages++;
      }
      assert.ok(BigInt(cursor) > BigInt(gap.snapshot_watermark));
      assert.equal(pages > 1, pageSize === 1);
      // A future empty page must not grant future ACK coverage.
      const empty = await run.replay((parkedTail + 100n).toString(), pageSize);
      assert.equal(empty.events.length, 0);
      await sdk.acknowledge(run.stream, empty.next_cursor.last_processed_sequence);
      await run.get();
      assert.equal((await notifications(work)).length, before.length);
      await sdk.acknowledge(run.stream, cursor);
      await run.get();
      assert.equal((await notifications(work)).length, before.length, "already consumed pages must not replay live");
      await writeFile(join(work, "release"), "");
      await waitFor(async () => (await notifications(work)).length > before.length);
      assert.ok((await notifications(work)).slice(before.length).every(value =>
        BigInt(value.params.gap?.from_sequence ?? value.params.envelope.sequence) > BigInt(cursor)));
      assert.equal((await run.wait()).settled, true);
      const settled = await run.get();
      const terminalTail = BigInt(settled.last_sequence);
      while (BigInt(cursor) < terminalTail) {
        const page = await run.replay(cursor, pageSize);
        assert.ok(page.events.length > 0);
        for (const event of page.events) {
          assert.equal(BigInt(event.envelope.sequence), BigInt(cursor) + 1n);
          assert.ok(!consumed.has(event.envelope.sequence));
          consumed.set(event.envelope.sequence, event.envelope.event_id);
          cursor = event.envelope.sequence;
        }
      }
      await sdk.acknowledge(run.stream, cursor);
      assert.equal(consumed.size, Number(terminalTail));
      const all = await run.replay("0", 64);
      assert.equal(all.events.filter(event => event.envelope.payload.event_type === "final_answer").length, 1);
      assert.deepEqual(all.events.map(event => event.envelope.event_id), [...consumed.values()]);
      await session.close();
      await agent.close();
    } finally {
      if (sdk) await sdk.close();
      model.kill();
      await new Promise(resolve => model.exitCode === null ? model.once("exit", resolve) : resolve());
      await rm(work, {recursive: true, force: true});
    }
  });
}

for(const window of [1,2])test(`paused consumer window ${window} external replay ACK merges queued gap without covered re-delivery`,{skip:!realHost,timeout:30000},async()=>{
 const work=await mkdtemp(join(tmpdir(),"echo-sdk-coalesced-ts-"));const python=process.env.PYTHON??"python3";
 const model=spawn(python,[fixture,"--work",work,"--mode","window"]);let sdk;
 try{
  await waitFor(()=>exists(join(work,"ready")));
  const configPath=join(work,"host.json");const config=JSON.parse(await readFile(configPath,"utf8"));config.sdk_profile.limits.max_outstanding_live_events=window;await writeFile(configPath,JSON.stringify(config));
  sdk=await EchoAgentClient.spawn({hostCommand:python,args:[proxy,realHost,configPath]});
  const agent=await sdk.createAgent();const session=await agent.createSession();const run=await session.startRun({text:"coalesced external recovery"});
  await waitFor(()=>exists(join(work,"parked")));
  const first=(await notifications(work)).find(frame=>frame.method==="_echo_agent/gap").params.gap;
  const covered=new Map();let cursor="0";
  const firstPage=await run.replay(cursor,64);
  for(const event of firstPage.events){if(BigInt(event.envelope.sequence)<=BigInt(first.snapshot_watermark)){covered.set(event.envelope.sequence,event.envelope.event_id);cursor=event.envelope.sequence;}}
  assert.equal(cursor,first.snapshot_watermark);
  await sdk.acknowledge(run.stream,cursor);
  await writeFile(join(work,"release"),"");
  await waitFor(async()=> (await notifications(work)).filter(frame=>frame.method==="_echo_agent/gap").length>=2);
  const iterator=run.events[Symbol.asyncIterator]();const item=(await iterator.next()).value;
  assert.ok("gap" in item);assert.equal(item.gap.from_sequence,(BigInt(cursor)+1n).toString());assert.ok(BigInt(item.gap.snapshot_watermark)>BigInt(cursor));
  await iterator.return();
  const parked=await run.get();const parkedPage=await run.replay(cursor,64);
  assert.ok(parkedPage.events.length>0);
  let pageCursor=BigInt(cursor);
  for(const event of parkedPage.events){assert.equal(BigInt(event.envelope.sequence),pageCursor+1n);pageCursor=BigInt(event.envelope.sequence);assert.ok(!covered.has(event.envelope.sequence));covered.set(event.envelope.sequence,event.envelope.event_id);}
  assert.equal(parkedPage.next_cursor.last_processed_sequence,pageCursor.toString());
  cursor=parkedPage.next_cursor.last_processed_sequence;await sdk.acknowledge(run.stream,cursor);
  // GET samples before replay; the released model may append between RPCs.
  assert.ok(BigInt(cursor)>=BigInt(parked.last_sequence));
  await writeFile(join(work,"release"),"");assert.equal((await run.wait({timeoutMs:10000})).settled,true);
  const terminal=await run.get();assert.equal(terminal.status,"completed");assert.deepEqual(terminal.stream,run.stream);assert.ok(BigInt(cursor)<=BigInt(terminal.last_sequence));
  const continuation=await run.replay(cursor,64);for(const event of continuation.events){assert.ok(!covered.has(event.envelope.sequence));covered.set(event.envelope.sequence,event.envelope.event_id);}
  const final=await run.replay("0",64);
  assert.ok(final.events.length>0 && final.events.length<=64);
  assert.equal(final.next_cursor.stream_id,run.stream.id);
  let finalCursor=0n;const finalIds=new Set();
  for(const event of final.events){assert.equal(BigInt(event.envelope.sequence),finalCursor+1n);finalCursor=BigInt(event.envelope.sequence);assert.equal(event.stream.id,run.stream.id);assert.equal(event.stream.generation,run.stream.generation);assert.equal(event.stream.kind,run.stream.kind);assert.ok(!finalIds.has(event.envelope.event_id));finalIds.add(event.envelope.event_id);}
  assert.equal(final.next_cursor.last_processed_sequence,finalCursor.toString());assert.equal(finalCursor.toString(),terminal.last_sequence);
  assert.deepEqual([...covered.entries()].sort((a,b)=>Number(BigInt(a[0])-BigInt(b[0]))).map(([,id])=>id),final.events.map(event=>event.envelope.event_id));
  assert.equal(final.events.filter(event=>event.envelope.payload.event_type==="final_answer").length,1);
  await session.close();await agent.close();
 }finally{await sdk?.close();model.kill();await new Promise(resolve=>model.exitCode===null?model.once("exit",resolve):resolve());await rm(work,{recursive:true,force:true});}
});


test("replayed terminal explicit ACK ends live iterator without covered re-delivery", {skip: !realHost, timeout: 30000}, async () => {
  const work = await mkdtemp(join(tmpdir(), "echo-sdk-terminal-ts-"));
  const python = process.env.PYTHON ?? "python3";
  const model = spawn(python, [fixture, "--work", work, "--mode", "oversized"]);
  let sdk;
  try {
    await waitFor(() => exists(join(work, "ready")));
    sdk = await EchoAgentClient.spawn({hostCommand: python, args: [proxy, realHost, join(work, "host.json")]});
    const agent = await sdk.createAgent();
    const session = await agent.createSession();
    const run = await session.startRun({text: "replayed terminal EOF 中文🌊"});
    await waitFor(() => exists(join(work, "parked")));
    const iterator = run.events[Symbol.asyncIterator]();
    assert.ok((await iterator.next()).value.gap);
    await writeFile(join(work, "release"), "");
    assert.equal((await run.wait({timeoutMs: 10000})).settled, true);
    const replay = await run.replay("0", 64);
    assert.equal(replay.events.filter(event => event.envelope.payload.event_type === "final_answer").length, 1);
    const tail = replay.next_cursor.last_processed_sequence;
    assert.equal(tail, replay.events.at(-1).envelope.sequence);
    const announced = (await notifications(work)).length;
    let observedEof = false;
    const eof = iterator.next().then(value => { observedEof = true; return value; });
    await run.get();
    assert.equal(observedEof, false, "replay fetch alone is not terminal consumption");
    await sdk.acknowledge(run.stream, (BigInt(tail) + 1n).toString());
    await run.get();
    assert.equal(observedEof, false, "uncovered future ACK must not end the feed");
    await sdk.acknowledge(run.stream, tail);
    await run.get();
    assert.equal((await notifications(work)).length, announced, "terminal recovery must not re-send covered data");
    let timer;
    try {
      const ended = await Promise.race([eof, new Promise((_, reject) => {
        timer = setTimeout(() => reject(new Error("replayed terminal ACK did not release natural EOF")), 2000);
      })]);
      assert.equal(ended.done, true);
    } finally { clearTimeout(timer); }
    assert.equal((await iterator.next()).done, true);
    await sdk.acknowledge(run.stream, tail);
    await run.get();
    assert.equal((await notifications(work)).length, announced);
    await session.close();
    await agent.close();
  } finally {
    await sdk?.close();
    model.kill();
    await new Promise(resolve => model.exitCode === null ? model.once("exit", resolve) : resolve());
    await rm(work, {recursive: true, force: true});
  }
});

for (const success of [false, true]) test(`real Host gap before official ACK completion success=${success}`, {skip: !realHost, timeout:30000}, async()=>{
 const work=await mkdtemp(join(tmpdir(),'echo-sdk-pending-gap-ts-'));
 const python=process.env.PYTHON ?? 'python3';const model=spawn(python,[fixture,'--work',work,'--mode','oversized']);let sdk;
 try{
  await waitFor(()=>exists(join(work,'ready')));
  sdk=await EchoAgentClient.spawn({hostCommand:python,args:[proxy,realHost,join(work,'host.json')]});
  const agent=await sdk.createAgent();const session=await agent.createSession();const run=await session.startRun({text:'pending ACK real continuation 中文🌊'});
  await waitFor(()=>exists(join(work,'parked')));
  const page=await run.replay('0',64);const cursor=BigInt(page.next_cursor.last_processed_sequence);const feed=sdk.events.get(run.stream.id);
  assert.ok(cursor>feed.lastEventSequence);const before=(await notifications(work)).length;
  const native=sdk.notify.bind(sdk);let resolve,reject;const gate=new Promise((a,b)=>{resolve=a;reject=b;});let enteredResolve;const entered=new Promise(a=>{enteredResolve=a;});
  sdk.notify=async(method,params)=>{await native(method,params);if(method==='_echo_agent/event/ack'){enteredResolve();await gate;}};
  const pending=sdk.acknowledge(run.stream,cursor.toString());pending.catch(()=>{});
  try{
   await entered;assert.equal(feed.acknowledgedSequence,undefined);await writeFile(join(work,'release'),'');
   await waitFor(async()=>(await notifications(work)).length>before);
   const gap=(await notifications(work))[before].params.gap;assert.equal(BigInt(gap.from_sequence),cursor+1n);
   await waitFor(()=>feed.lastEventSequence===BigInt(gap.snapshot_watermark));assert.equal(feed.acknowledgedSequence,undefined);assert.equal(feed.queue.closed,false);
   if(success){resolve();await pending;assert.equal(feed.acknowledgedSequence,cursor);}else{reject(new Error('write outcome unknown after real peer gap'));await assert.rejects(pending);assert.equal(feed.acknowledgedSequence,undefined);}
   sdk.notify=native;const iterator=run.events[Symbol.asyncIterator]();const item=(await iterator.next()).value;
   assert.equal(BigInt(item.gap.from_sequence),success?cursor+1n:1n);assert.ok(BigInt(item.gap.snapshot_watermark)>cursor);assert.equal(feed.queue.closed,false);await iterator.return();
   assert.equal((await run.wait()).settled,true);
   if(process.env.ECHO_AGENT_SDK_RECOVERY_EVIDENCE)await writeFile(join(process.env.ECHO_AGENT_SDK_RECOVERY_EVIDENCE,`typescript-pending-gap-${success}.json`),JSON.stringify({success,replay:page,ack_cursor:cursor.toString(),continued_gap:gap,delivered:item,notifications:await notifications(work)},null,2));
  }finally{sdk.notify=native;resolve();await pending.catch(()=>{});}
  await session.close();await agent.close();
 }finally{if(sdk)await sdk.close();model.kill();await new Promise(resolve=>model.exitCode===null?model.once('exit',resolve):resolve());await rm(work,{recursive:true,force:true});}
});


test("live journal directory replacement returns typed error and isolates the other Session", {skip: !realHost, timeout: 30000}, async () => {
  const python = process.env.PYTHON ?? "python3";
  const work = await mkdtemp(join(process.env.ECHO_AGENT_SDK_JOURNAL_EVIDENCE ?? tmpdir(), "journal-ts-"));
  const model = spawn(python, [fixture, "--work", work, "--mode", "window"]);
  let sdk, run, replaced = false;
  const command = async args => JSON.parse((await promisify(execFile)(python, [fixture, "--work", work, "--mode", "window", ...args], {timeout:5000})).stdout);
  const action = kind => command(["--journal-action", kind, "--stream-id", run.stream.id]);
  const samePage = async (before, after, name) => {
    const paths = [join(work, `${name}-before.json`), join(work, `${name}-after.json`)];
    await writeFile(paths[0], JSON.stringify(before)); await writeFile(paths[1], JSON.stringify(after));
    assert.equal((await command(["--compare-pages", ...paths])).same_complete_page, true);
  };
  try {
    await waitFor(() => exists(join(work, "ready")));
    sdk = await EchoAgentClient.spawn({hostCommand:python,args:[proxy,realHost,join(work,"host.json")],env:{...process.env,NO_PROXY:"127.0.0.1,localhost",no_proxy:"127.0.0.1,localhost"}});
    const agent = await sdk.createAgent(), session = await agent.createSession();
    run = await session.startRun({text:"original journal 中文🌊"});
    await waitFor(() => exists(join(work,"parked"))); await writeFile(join(work,"release"),"");
    const settled = await run.wait(); assert.equal(settled.settled,true); assert.equal(settled.terminal.status,"completed");
    const before = await run.replay("0",64), snapshot = await run.get(); assert.ok(before.events.length>0); assert.ok(snapshot.receipt);
    const otherAgent = await sdk.createAgent(), otherSession = await otherAgent.createSession();
    const other = await otherSession.startRun({text:"independent journal 中文🌊"}); assert.equal((await other.wait()).settled,true);
    const otherBefore = await other.replay("0",64); assert.notEqual(other.stream.id,run.stream.id); assert.ok(otherBefore.events.length>0);
    const fact = await action("replace"); replaced = true; const errors = []; let restored;
    try {
      for (let count=0;count<2;count++) await assert.rejects(run.replay("0",64), error => {
        assert.equal(error.code,"replay_unavailable"); assert.equal(error.retryable,"after_delay"); assert.equal(error.operation,"_echo_agent/run/replay"); assert.ok(error.message.includes("directory identity changed")); assert.equal(error.details??null,null); assert.equal(error.handle??null,null);
        errors.push({code:error.code,retryable:error.retryable,operation:error.operation,details:error.details??null,handle:error.handle??null,message:error.message}); return true;
      });
      assert.deepEqual(await run.get(),snapshot,"failed replay cannot replace a terminal or receipt");
      await samePage(otherBefore,await other.replay("0",64),"other");
    } finally { restored=await action("restore"); replaced=false; }
    await samePage(before,await run.replay("0",64),"restored");
    if(process.env.ECHO_AGENT_SDK_JOURNAL_EVIDENCE) await writeFile(join(process.env.ECHO_AGENT_SDK_JOURNAL_EVIDENCE,"typescript.json"),JSON.stringify({stream:run.stream,other_stream:other.stream,errors,directory:fact,restored,receipt_unchanged:true,other_journal_unchanged:true,replay_restored:true,page:before},null,2));
    await otherSession.close(); await otherAgent.close(); await session.close(); await agent.close();
  } finally {
    if(replaced) await action("restore");
    if(sdk) await sdk.close(); model.kill(); await new Promise(resolve=>model.exitCode===null?model.once("exit",resolve):resolve());
    await rm(work,{recursive:true,force:true});
  }
});
