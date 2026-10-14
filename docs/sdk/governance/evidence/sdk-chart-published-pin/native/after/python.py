import asyncio,hashlib,json,os,subprocess,sys
from pathlib import Path
S=Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance');sys.path.insert(0,str(S/'sdks/python/src'))
from echo_agent_sdk import EchoAgentClient,EchoAgentError,to_wire
D=Path(__file__).parent;work=D/'python-runtime';host=os.environ['ECHO_AGENT_SDK_HOST'];subprocess.run(['python3',str(D/'prepare.py'),str(work)],check=True)
def snapshot():return {n:hashlib.sha256((work/n).read_bytes()).hexdigest() for n in ['stats.csv','input.csv','lookup.csv','quality.csv','input.txt','sentinel.txt','null-index.csv','collision.csv']}
async def main():
 before=snapshot();rows=[]
 async with await EchoAgentClient.spawn(host,'--config',str(work/'host.json')) as sdk:
  agent=await sdk.create_agent();session=await agent.create_session(str(work));identity={'agent':agent.wire.to_dict(),'session':session.wire.to_dict(),'capability':sdk.capability};process=[l for l in subprocess.check_output(['ps','-axo','pid,ppid,command'],text=True).splitlines() if host in l and 'ps -axo' not in l]
  for step in json.loads((work/'bounds.json').read_text()):
   args=step['arguments'];row={'id':step['id'],'operation':step['operation'],'arguments_wire':[to_wire(v) for v in args]}
   try:row['result']=await session.invoke(step['operation'],args)
   except EchoAgentError as error:row['error']={'code':error.code,'message':error.message,'operation':error.operation,'retryable':error.retryable,'handle':error.handle.to_dict() if error.handle else None,'details':error.details}
   rows.append(row)
  await session.close();await agent.close()
 (D/'python.json').write_text(json.dumps({'language':'python','host':host,'host_sha256':hashlib.sha256(Path(host).read_bytes()).hexdigest(),'pid':os.getpid(),'hostProcess':process,'work':str(work),'identity':identity,'protected_before':before,'protected_after':snapshot(),'rows':rows},ensure_ascii=False,indent=2)+'\n')
 print('observed',len(rows),'native Chart calls')
asyncio.run(main())
