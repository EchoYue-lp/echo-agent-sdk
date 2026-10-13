import asyncio,hashlib,json,os,tempfile
from pathlib import Path
from echo_agent_sdk import EchoAgentClient,EchoAgentError,from_wire,to_wire
from portable_fixture import prepare
ROOT=Path(__file__).parent
HOST=os.environ.get('ECHO_AGENT_SDK_HOST','/tmp/echo-sdk-governance-guard-pivot-first-host')
def rebase(value,work):
    if isinstance(value,str):return value.replace('$WORK',str(work))
    if isinstance(value,list):return [rebase(item,work)for item in value]
    if isinstance(value,dict):return {k:rebase(v,work)for k,v in value.items()}
    return value
async def main():
    work=Path(tempfile.mkdtemp(prefix='sdk-computation-native-python-'));prepare(work);rows=[]
    async with await EchoAgentClient.spawn(HOST,'--config',str(work/'host.json'))as sdk:
        agent=await sdk.create_agent();session=await agent.create_session(str(work));identity={'agent':agent.wire.to_dict(),'session':session.wire.to_dict(),'capability':sdk.capability}
        for step in json.loads((ROOT/'inputs.json').read_text())['rows']:
            args=rebase(step['arguments'],work);row={**step,'arguments':args}
            try:
                if step['operation']=='structured_output.validate':
                    value=await sdk.request('echo_agent/structured_output/validate',{'operation':step['operation'],'signature_digest':'sha256:'+'0123456789abcdef'*4,'handle':session.wire.to_dict(),'arguments':[to_wire(item)for item in args]});result=from_wire(value['value'])
                else:result=await session.invoke(step['operation'],args)
                row['result']=result;assert step['expected_error']is None,step['id']
            except EchoAgentError as error:
                row['error']={'code':error.code,'message':error.message,'operation':error.operation,'retryable':error.retryable,'handle':error.handle.to_dict()if error.handle else None,'details':error.details};assert error.code==step['expected_error'],row
            rows.append(row)
        await session.close();await agent.close()
    data={'host':HOST,'host_sha256':hashlib.sha256(Path(HOST).read_bytes()).hexdigest(),'work':str(work),'rows':rows,'raw_identity':identity,'actual_exports':{'output_text':(work/'output.txt').read_text(),'output_json':json.loads((work/'output.json').read_text())},'actual_side_effects':{'sentinel_contents':(work/'sentinel.txt').read_text(),'sentinel_sha256':hashlib.sha256((work/'sentinel.txt').read_bytes()).hexdigest()}}
    (ROOT/'python-final-computation-evidence.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n');print('PY native public corpus',len(rows),'work',work)
asyncio.run(main())
