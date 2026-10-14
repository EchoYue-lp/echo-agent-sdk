from pathlib import Path
import subprocess, os, json, hashlib, time, shutil, xml.etree.ElementTree as ET
s=Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance')
a=Path(__file__).resolve().parent
runtime=a/'runtime';runtime.mkdir(exist_ok=True)
old=json.loads((s/'docs/sdk/governance/evidence/sdk-language-00782-reviewed-final/source-before.json').read_text())
skip={'node_modules','target','.venv','__pycache__','.ruff_cache','.pytest_cache','dist','dist-examples','build','.git'}
def snapshot():
 paths={s/p for p in old if '/' not in p}
 for root in {p.split('/')[0] for p in old if '/' in p}:
  for p in (s/root).rglob('*'):
   rel=p.relative_to(s)
   if p.is_file() and not set(rel.parts)&skip and p.suffix!='.pyc': paths.add(p)
 return {p.relative_to(s).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths)}
before=snapshot();assert len(before)==800
(a/'source-before.json').write_text(json.dumps(before,indent=2)+'\n')
node=json.loads((a.parent/'sdk-host-hash-streaming/probe-runtime.json').read_text())['node20']
env={**os.environ,'PATH':str(Path(node).parent)+os.pathsep+os.environ['PATH'],'CARGO_TARGET_DIR':'/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-final-framework-pin/target','CARGO_BUILD_JOBS':'1','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_INCREMENTAL':'0','TMPDIR':str(runtime),'UV_PYTHON':'3.10.19','PYTHONDONTWRITEBYTECODE':'1','TZ':'UTC'}
versions=[]
for command in [['node','--version'],['npm','--version'],['uv','run','--no-project','python','-c','import sys; print(sys.version); print(sys.executable)'],['java','-version'],['mvn','-version'],['cargo','--version']]:
 probe_cwd=s/'sdks/python' if command[0]=='uv' else s
 r=subprocess.run(command,cwd=probe_cwd,env=env,capture_output=True);assert r.returncode==0,command
 versions.append({'command':command,'cwd':str(probe_cwd),'exitCode':r.returncode,'stdout':r.stdout.decode(),'stderr':r.stderr.decode()})
(a/'versions.json').write_text(json.dumps(versions,indent=2)+'\n')
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=s,text=True).strip()
changed=subprocess.check_output(['git','diff','--name-only'],cwd=s,text=True).splitlines()
receipt={'command':['bash','scripts/check-language-sdks.sh'],'headAtStart':head,'sourceInputFiles':800,'environment':{k:env[k] for k in ['PATH','CARGO_TARGET_DIR','CARGO_BUILD_JOBS','CARGO_PROFILE_DEV_DEBUG','CARGO_PROFILE_TEST_DEBUG','CARGO_INCREMENTAL','TMPDIR','UV_PYTHON','PYTHONDONTWRITEBYTECODE','TZ']},'uncommittedRuntimeInputsAtStart':[p for p in changed if p in before],'startedNs':time.time_ns(),'status':'running'}
(a/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
with (a/'full-language.log').open('wb') as log:r=subprocess.run(receipt['command'],cwd=s,env=env,stdout=log,stderr=subprocess.STDOUT)
receipt.update(finishedNs=time.time_ns(),exitCode=r.returncode,status='completed',headAtFinish=subprocess.check_output(['git','rev-parse','HEAD'],cwd=s,text=True).strip())
after=snapshot();(a/'source-after.json').write_text(json.dumps(after,indent=2)+'\n')
receipt.update(inputsUnchanged=before==after,addedInputs=sorted(set(after)-set(before)),removedInputs=sorted(set(before)-set(after)),changedInputs=[p for p in before.keys()&after.keys() if before[p]!=after[p]])
log_bytes=(a/'full-language.log').read_bytes();receipt.update(logBytes=len(log_bytes),logSha256=hashlib.sha256(log_bytes).hexdigest(),truncated=False)
host=Path(env['CARGO_TARGET_DIR'])/'debug/echo-agent-sdk-host';h=hashlib.sha256()
with host.open('rb') as stream:
 for chunk in iter(lambda:stream.read(1024*1024),b''):h.update(chunk)
receipt.update(hostSha256=h.hexdigest(),hostBytes=host.stat().st_size)
xmls=[];totals={'tests':0,'failures':0,'errors':0,'skipped':0}
for p in sorted((s/'sdks/java/target/surefire-reports').glob('TEST-*.xml')):
 stamp=p.stat().st_mtime_ns;fresh=receipt['startedNs']<=stamp<=receipt['finishedNs']
 root=ET.fromstring(p.read_bytes());counts={k:int(root.attrib.get(k,'0')) for k in totals}
 for k,v in counts.items():totals[k]+=v
 xmls.append({'path':str(p),'mtimeNs':stamp,'fresh':fresh,'counts':counts,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
 if fresh:
  dest=a/'java-xml'/p.name;dest.parent.mkdir(exist_ok=True);shutil.copy2(p,dest)
(a/'java-xml-freshness.json').write_text(json.dumps(xmls,indent=2)+'\n')
receipt.update(java=totals,javaSuites=len(xmls),allJavaXmlFresh=bool(xmls) and all(x['fresh'] for x in xmls))
receipt['finalValidationExit']=0 if r.returncode==0 and before==after and receipt['allJavaXmlFresh'] and totals['failures']==totals['errors']==totals['skipped']==0 else 1
(a/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt,indent=2),flush=True)
raise SystemExit(receipt['finalValidationExit'])
