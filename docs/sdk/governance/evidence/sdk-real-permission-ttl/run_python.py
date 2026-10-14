import asyncio,hashlib,json,os,time,traceback
from pathlib import Path
from echo_agent_sdk import EchoAgentClient
ROOT=Path(__file__).parent;WORK=ROOT/'python';SPEC=json.loads((ROOT/'scenario.json').read_text());HOST=json.loads((ROOT/'source-provenance.json').read_text())['host']
START=time.monotonic();rows=[]
def save(phase,**extra):
 (WORK/'status.json').write_text(json.dumps({'language':'python','runner_pid':os.getpid(),'phase':phase,'elapsed_ms':(time.monotonic()-START)*1000,'wall_time':time.time(),'rows':rows,**extra},ensure_ascii=False,indent=2))
async def main():
 sdk=None
 try:
  sdk=await EchoAgentClient.spawn(HOST,'--config',str(WORK/'host.json'));agent=await sdk.create_agent();a=await agent.create_session(str(WORK));b=await agent.create_session(str(WORK));save('connected',host_pid=sdk.process.pid,agent=agent.wire.to_dict(),session=a.wire.to_dict(),peer=b.wire.to_dict())
  async def call(session,op,args=[]):
   before=time.monotonic();v=await session.invoke(op,args);rows.append({'operation':op,'session':session.wire.to_dict(),'arguments':args,'value':v,'elapsed_ms':(time.monotonic()-START)*1000,'call_duration_ms':(time.monotonic()-before)*1000});return v
  stats=await call(a,'permission.stats');assert int(stats['ttl_ms'])==SPEC['ttl_expected_ms'];assert int(stats['per_tool_entries'])==0 and int(stats['global_entries'])==0
  for entry in SPEC['entries']:await call(a,'permission.record_approval',entry)
  recorded=time.monotonic()
  for scope,tool,args,kind in SPEC['entries']:assert await call(a,'permission.is_approved',[scope,tool,args]) is True
  assert await call(a,'permission.is_approved',['scope 中文🙂','parameter-tool',{'x':99}]) is False
  assert await call(b,'permission.is_approved',SPEC['entries'][0][:3]) is False
  assert await call(a,'permission.is_approved',['scope-β','global-tool',{'different':True}]) is True
  await call(a,'permission.record_approval',SPEC['once']);assert await call(a,'permission.is_approved',SPEC['once'][:3]) is False
  assert int(await call(a,'permission.cleanup_expired'))==0
  stats=await call(a,'permission.stats');assert int(stats['per_tool_entries'])==2 and int(stats['global_entries'])==2
  deadline=recorded+(SPEC['ttl_expected_ms']+SPEC['real_wait_safety_ms'])/1000
  while time.monotonic()<deadline:
   save('waiting_real_default_ttl',host_pid=sdk.process.pid,ttl_ms=stats['ttl_ms'],remaining_ms=max(0,(deadline-time.monotonic())*1000),recorded_monotonic_s=recorded)
   await asyncio.sleep(min(5,max(.001,deadline-time.monotonic())))
  elapsed=(time.monotonic()-recorded)*1000;assert elapsed>=SPEC['ttl_expected_ms']+SPEC['real_wait_safety_ms']
  for scope,tool,args,kind in SPEC['entries']:assert await call(a,'permission.is_approved',[scope,tool,args]) is False
  stats=await call(a,'permission.stats');assert int(stats['per_tool_entries'])==2 and int(stats['global_entries'])==2
  assert int(await call(a,'permission.cleanup_expired'))==4;assert int(await call(a,'permission.cleanup_expired'))==0
  stats=await call(a,'permission.stats');assert int(stats['per_tool_entries'])==int(stats['global_entries'])==0
  await call(a,'permission.record_approval',SPEC['entries'][0]);assert await call(a,'permission.is_approved',SPEC['entries'][0][:3]) is True
  await b.close();await a.close();await agent.close();hostpid=sdk.process.pid;await sdk.close();save('passed',host_pid=hostpid,host_exit_code=sdk.process.returncode,actual_wait_ms=elapsed,ttl_ms=SPEC['ttl_expected_ms'],host_sha256=hashlib.sha256(Path(HOST).read_bytes()).hexdigest())
 except BaseException as error:save('failed',error=repr(error),traceback=traceback.format_exc());raise
 finally:
  if sdk and sdk.process.returncode is None:await sdk.close()
asyncio.run(main())
