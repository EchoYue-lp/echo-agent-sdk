"""Prepare only tiny real input files, never execute Host commands."""
import json
from pathlib import Path
REPO=Path('/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance')
def prepare(work):
    values={'stats.csv':'value,label\n1,一\n2,二\n3,三\n4,四\n','input.csv':'id,group,quarter,value\n1,a,q1,1\n2,a,q2,2\n3,b,q1,3\n4,b,q2,4\n','lookup.csv':'id,label\n1,中文\n3,emoji😀\n','quality.csv':'id,value,label\n1,10,中文\n2,,emoji😀\n3,-1,alpha\n4,100,beta\n','input.txt':'中文😀\nalpha\n中文😀\n','sentinel.txt':'unchanged\n'}
    for name,value in values.items():(work/name).write_text(value)
    config=json.loads((REPO/'echo-sdk-host/config.sdk.example.json').read_text());config['sdk_profile']['state_root']=str(work/'state');config['default_agent']['agent']['memory_path']=str(work/'memory.jsonl');(work/'host.json').write_text(json.dumps(config))
if __name__=='__main__':
    import sys
    work=Path(sys.argv[1]);work.mkdir(parents=True,exist_ok=True);prepare(work)
