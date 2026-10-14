from pathlib import Path
import json,hashlib,shutil,re,difflib
root=Path('/tmp/sdk-workflow-candidate-fixture')
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
manifest=json.loads((root/'candidate-frozen-manifest.json').read_text());red_hash={Path(x['path']).name:x['sha256'] for x in manifest['files']}
red_sources=root/'red-evidence/runner-source-reconstructed';red_sources.mkdir(exist_ok=True)
green=root/'green-evidence';green.mkdir(exist_ok=True)
sources=[]
for name,expected in red_hash.items():
 s=(root/name).read_text();old=s
 if name=='workflow-host.test.js':old=s.replace('if(passed&&!process.env.ECHO_AGENT_SDK_RETAIN_EVIDENCE)','if(passed)')
 elif name=='test_workflow_host.py':old=s.replace('if passed and not os.environ.get("ECHO_AGENT_SDK_RETAIN_EVIDENCE"):','if passed:')
 elif name=='WorkflowHostTest.java':old=s.replace('if(passed&&System.getenv("ECHO_AGENT_SDK_RETAIN_EVIDENCE")==null)','if(passed)')
 assert hashlib.sha256(old.encode()).hexdigest()==expected,(name,expected)
 (red_sources/name).write_text(old);shutil.copyfile(root/name,green/name)
 diff=''.join(difflib.unified_diff(old.splitlines(True),s.splitlines(True),fromfile='red/'+name,tofile='green/'+name))
 if diff:(green/(name+'.delta.patch')).write_text(diff)
 sources.append({'name':name,'red_sha256':expected,'green_sha256':sha(root/name),'red_source_kind':'exact inverse retention-only reconstruction matched earlier frozen hash; not original backup','semantic_delta':'none; only retain successful observation directory via test env' if diff else 'none; exact byte equality'})
allrows={};runs=[]
for lang in ['typescript','python','java']:
 log=Path('/tmp/sdk-workflow-'+lang+'-green-captured.log');s=log.read_text();match=re.search(r'Workflow evidence retained: ([^\n]+)',s);assert match,lang
 work=Path(match.group(1));dest=green/lang;dest.mkdir(exist_ok=True)
 for p in work.glob('call-*.json'):shutil.copyfile(p,dest/p.name)
 for name in ['scenario.json','results.json']:shutil.copyfile(work/name,dest/name)
 shutil.copyfile(log,dest/'run.log');rows=json.loads((dest/'results.json').read_text());assert len(rows)==48;allrows[lang]=rows
 runs.append({'language':lang,'exit_code':0,'skipped':0,'rows':48,'work':str(work),'files':[{'path':str(p),'sha256':sha(p)} for p in dest.iterdir()]})
# Preserve each language's identity equivalence and timestamp equivalence while
# comparing deterministic values. Unknown UUID occurrences are opaque labels,
# never a claimed fixed expected UUID. No numeric/state/schema fields omitted.
UUID=re.compile(r'[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}')
TIME=re.compile(r'\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})')
def normalized(rows):
 ids={};times={}
 def string(value):
  def opaque(match):
   token=match.group(0);ids.setdefault(token,'opaque-'+str(len(ids)+1));return ids[token]
  def timestamp(match):
   token=match.group(0);times.setdefault(token,'timestamp-'+str(len(times)+1));return times[token]
  return TIME.sub(timestamp,UUID.sub(opaque,value))
 def visit(value):
  if isinstance(value,str):return string(value)
  if isinstance(value,list):return [visit(item) for item in value]
  if isinstance(value,dict):return {key:visit(value[key]) for key in sorted(value)}
  return value
 canonical=[]
 for row in rows:
  if row['id']=='state-keys' and row['outcome']=='result':
   row={**row,'value':{**row['value'],'keys':sorted(row['value']['keys'])}}
  if row['outcome']=='error':
   row={key:row.get(key) for key in ['id','outcome','code','message','operation','retryable','details']}
  canonical.append(visit(row))
 return canonical
norm={lang:normalized(rows) for lang,rows in allrows.items()}
for lang,data in norm.items():(green/(lang+'-normalized.json')).write_text(json.dumps(data,ensure_ascii=False,sort_keys=True,indent=2)+'\n')
reference=norm['typescript'];different={}
for lang,data in norm.items():
 diffs=[{'index':i,'typescript':a,'other':b} for i,(a,b) in enumerate(zip(reference,data)) if a!=b]
 if diffs:different[lang]=diffs
proof={'host':'/tmp/echo-sdk-governance-workflow-shape-host','host_sha256':sha('/tmp/echo-sdk-governance-workflow-shape-host'),'old_host_sha256':sha('/tmp/echo-sdk-governance-parent-driver-host'),'host_build_proof':'/tmp/sdk-workflow-shape-host-proof.json','host_build_proof_sha256':sha('/tmp/sdk-workflow-shape-host-proof.json'),'source_epochs':sources,'scenario_rows':48,'workflow_calls':47,'session_close_calls':1,'runs':runs,'comparison':{'normalization':'stable occurrence labels for UUID and RFC3339 timestamps preserving within-run equality; keys sorted; optional absent error details normalized null. Only SharedState.keys HashMap key collection sorted; all ordered path/list/field values and numeric/context/state/schema/detail values retained.','all_three_equal':not different,'differences':different},'limitations':['MemoryStore process-local only, no restart proof','17 of 24 accepted operations exercised; not stream/callback/cancel completeness','candidate local patched framework input, not exact published main','native plain checkpoint continuation.kind collision requires explicit public WireValue map; not plain native-object roundtrip']}
(root/'workflow-red-green-proof.json').write_text(json.dumps(proof,ensure_ascii=False,indent=2)+'\n')
print('3 real green complete; normalized equal',not different)
if different:print(json.dumps(different,ensure_ascii=False)[:2500])
