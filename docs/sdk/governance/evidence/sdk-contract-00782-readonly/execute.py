from pathlib import Path
import subprocess,os,json,hashlib,shutil,time
s=Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance')
a=Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-contract-00782-readonly')
t=Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-final-framework-pin/target')
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def inputs():
 source=json.loads(Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-language-00782-reviewed-final/source-before.json').read_text())
 return {p:digest(s/p) for p in sorted(source) if (s/p).is_file()}
def artifacts():
 files=[p for p in (s/'contracts/sdk').rglob('*') if p.is_file()]
 files += [s/'sdks/shared/facade-operation-catalog.json',s/'sdks/shared/contract-digests.json']
 return {str(p.relative_to(s)): {'sha256':digest(p),'bytes':p.stat().st_size} for p in sorted(files)}
env={**os.environ,'CARGO_TARGET_DIR':str(t),'CARGO_BUILD_JOBS':'2','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_INCREMENTAL':'0'}
before=inputs();generated=artifacts();head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=s,text=True).strip();lock=digest(s/'Cargo.lock')
assert len(before)==793 and lock=='88d99f3880f75315f60c8eca48035eb6151a1a055aeda7f621e689b354d411b3'
assert 'rev = "00782b8aeddefe2ea8c566922cbdee6aedc80d70"' in (s/'echo-sdk-host/Cargo.toml').read_text()
(a/'source-before.json').write_text(json.dumps(before,indent=2)+'\n');(a/'artifacts-before.json').write_text(json.dumps(generated,indent=2)+'\n')
start=time.time_ns();print('START full read-only check793inputs / diskGiB',shutil.disk_usage(a).free/2**30,flush=True)
r=subprocess.run(['node','/Users/ls/.codex/plugins/cache/supreme/supreme/0.4.12/skills/build/scripts/run-command.mjs','--cwd',str(s),'--log',str(a/'full-check.log'),'--run',str(a/'full-check-run.json'),'--label','sdk-contract-00782-full-readonly','--','bash','scripts/check-sdk-contracts.sh'],cwd=s,env=env,capture_output=True,text=True)
(a/'runner.log').write_text(r.stdout+r.stderr);after=inputs();ga=artifacts();(a/'source-after.json').write_text(json.dumps(after,indent=2)+'\n');(a/'artifacts-after.json').write_text(json.dumps(ga,indent=2)+'\n')
rec={'command':['bash','scripts/check-sdk-contracts.sh'],'observedHead':head,'frameworkGit':'00782b8aeddefe2ea8c566922cbdee6aedc80d70','toolExit':r.returncode,'startedNs':start,'finishedNs':time.time_ns(),'sourceInputs':len(before),'sourceUnchanged':before==after,'artifactSeedUnchanged':generated==ga,'lockUnchanged':lock==digest(s/'Cargo.lock'),'logSHA':digest(a/'full-check.log'),'diskFreeGiB':shutil.disk_usage(a).free/2**30,'scope':'complete check only; no update/accept/retire/seed edits; retain failure and actual profile count; short-circuited commands not proved'}
(a/'receipt.json').write_text(json.dumps(rec,indent=2)+'\n');print(json.dumps(rec,indent=2),flush=True)
if not (rec['sourceUnchanged'] and rec['artifactSeedUnchanged'] and rec['lockUnchanged']):raise SystemExit(2)
raise SystemExit(r.returncode)
