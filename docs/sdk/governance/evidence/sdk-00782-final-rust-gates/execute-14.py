import json,hashlib,os,subprocess,shutil
from pathlib import Path
s=Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance')
a=Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-00782-final-rust-gates')
commands=json.loads((a/'exact-original-commands.json').read_text())
before=json.loads((a/'inputs-before.json').read_text())
base=dict(os.environ,CARGO_TARGET_DIR='/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-final-framework-pin/target',CARGO_BUILD_JOBS='2',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0')
runner='/Users/ls/.codex/plugins/cache/supreme/supreme/0.4.12/skills/build/scripts/run-command.mjs'
manifest={'scope':'published00782 SDK Rust14 exact source cut; contract and language/main separate','environment':{k:base[k] for k in ['CARGO_TARGET_DIR','CARGO_BUILD_JOBS','CARGO_PROFILE_DEV_DEBUG','CARGO_PROFILE_TEST_DEBUG','CARGO_INCREMENTAL']},'steps':[],'complete':False}
for index,(name,args,extra) in enumerate(commands,1):
 label=f'{index:02d}-{name}';print('START',label,flush=True)
 result=subprocess.run(['node',runner,'--cwd',str(s),'--log',str(a/(label+'.log')),'--run',str(a/(label+'-run.json')),'--label',label,'--',*args],env={**base,**extra},capture_output=True,text=True)
 (a/(label+'-runner.log')).write_text(result.stdout+result.stderr)
 receipt=json.loads((a/(label+'-run.json')).read_text());step=receipt['steps'][-1] if 'steps' in receipt else receipt
 current={p:hashlib.sha256((s/p).read_bytes()).hexdigest() for p in before}
 row={'label':label,'command':args,'environmentOverrides':extra,'exitCode':result.returncode,'stepReceipt':label+'-run.json','logSha256':hashlib.sha256((a/(label+'.log')).read_bytes()).hexdigest(),'truncated':step['truncated'],'bytes':step['bytesWritten'],'diskAvailableBytes':shutil.disk_usage(s).free,'sourceChanged':[p for p in before if before[p]!=current[p]]}
 manifest['steps'].append(row);(a/'inputs-after-current.json').write_text(json.dumps(current,indent=2)+'\n');(a/'matrix.json').write_text(json.dumps(manifest,indent=2)+'\n')
 print('END',label,'exit',result.returncode,'sourceDelta',len(row['sourceChanged']),'free',row['diskAvailableBytes'],flush=True)
 if result.returncode:raise SystemExit(result.returncode)
 if row['sourceChanged']:raise SystemExit('Source changed during gate')
 if row['truncated']:raise SystemExit('Raw truncated')
manifest['complete']=True;manifest['allExitZero']=True;(a/'inputs-after.json').write_text(json.dumps(current,indent=2)+'\n');(a/'matrix.json').write_text(json.dumps(manifest,indent=2)+'\n');print('ALL_14_PASS_SOURCE61_UNCHANGED',flush=True)
