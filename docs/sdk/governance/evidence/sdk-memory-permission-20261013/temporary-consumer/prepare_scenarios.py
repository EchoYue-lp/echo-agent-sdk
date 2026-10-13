"""Freeze candidate scenarios and verify every planned invocation is accepted."""
import copy
import hashlib
import json
from pathlib import Path

REPO = Path('/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance')
catalog_path = REPO / 'contracts/sdk/accepted-facade-operation-catalog.json'
catalog = json.loads(catalog_path.read_text())
accepted = {r['operation'] for r in catalog['routes'] if r.get('operation')}
accepted.update(op for family in catalog['families'] for op in family.get('supported_operations', family.get('operations', [])) if isinstance(op, str))
# Family handler aliases are explicitly accepted operation entries too.
for family in catalog['families']:
    for entry in family.get('operations', []):
        if isinstance(entry, dict) and entry.get('operation'):
            accepted.add(entry['operation'])

MEMORY_SET = 'echo_agent::agent::react::ReactAgent::set_store'
NEW_STORE = 'echo_state::memory::store::InMemoryStore::new'
PUT_RAW = 'echo_state::memory::store::InMemoryStore::put_raw'
meta = {'memory_type': 'user_preference', 'confidence': 1.0, 'stability': 0.85, 'recall_weight': 0.9, 'risk': 'low', 'status': 'active', 'source': 'explicit_save', 'topic': 'sdk-governance', 'revision_count': 0, 'recall_count': 0, 'provenance': {'trust': 'user', 'evidence': [{'role': 'user', 'quote': 'SDK-Recall-Active 中文🌊 prefers Rust'}], 'approval': {'approval_id': 'candidate-fixture-reviewed-snapshot', 'reviewer': 'test-fixture-owner', 'approved_at': 1}}}
active = {'content': 'SDK-Recall-Active 中文🌊 prefers Rust', 'meta': meta}
draft = copy.deepcopy(active); draft['content'] = 'SDK-Recall-Draft do-not-inject'; draft['meta']['status'] = 'draft'; draft['meta']['provenance'].pop('approval')
legacy = copy.deepcopy(active); legacy['content'] = 'SDK-Recall-Legacy do-not-inject'; legacy['meta'].pop('provenance')
rule = {'matcher': {'type': 'tool', 'name': 'candidate_write'}, 'behavior': {'type': 'deny', 'reason': 'fixture deny 中文🌊'}, 'source': 'session', 'description': 'exact accepted rule'}
context = {'scope_id': 'shared-scope-中文🌊', 'request_id': 'permission-native-request', 'session_id': '$sessionA.acp_id', 'agent_name': 'memory-fixture', 'timeout_ms': 1000, 'permission': {'working_directory': '$work', 'affected_files': ['$work/effect.txt'], 'estimated_impact': 'one fixture file', 'metadata': {'中文': '🌊', '__proto__': 'plain data'}}, 'classifier': {'workspace_path': '$work', 'project_type': 'fixture', 'recent_files': ['effect.txt'], 'risk_context': {'has_sensitive_files': False, 'is_destructive': False, 'directory_depth': 1, 'repetition_count': 0}}}
scenarios = [
 {'id': 'raw-native-store-resource', 'mechanism': 'real framework InMemoryStore via accepted resource receiver', 'steps': [
  {'operation': NEW_STORE, 'receiver': '$agent', 'args': ['$sessionA'], 'save': '$store'},
  {'operation': 'memory.resource.put', 'receiver': '$sessionA', 'args': ['$store', ['sdk','中文'], 'one', {'content': 'duplicate-needle中文🌊'}]},
  {'operation': 'memory.resource.put', 'receiver': '$sessionA', 'args': ['$store', ['sdk','中文'], 'two', {'content': 'duplicate-needle中文🌊'}]},
  {'operation': PUT_RAW, 'receiver': '$agent', 'args': ['$sessionA','$store',{'namespace':['sdk','中文'],'key':'expired','value':{'content':'expired-needle'},'created_at':1,'updated_at':1,'score':None,'importance':5.0,'last_accessed':None,'expires_at':2}]},
  {'operation': 'memory.resource.get', 'receiver': '$sessionA', 'args': ['$store',['sdk','中文'],'expired'], 'assert': 'raw get preserves expires_at=2 before explicit prune; do not invent implicit deletion'},
  {'operation':'memory.resource.search_with','receiver':'$sessionA','args':['$store',['sdk','中文'],{'text':'duplicate-needle','limit':5,'mode':'Keyword'}],'assert':'exact two original keys, unicode preserved'},
  {'operation':'memory.resource.prune_expired','receiver':'$sessionA','args':['$store',['sdk','中文']],'assert':'1 then 0; expired get null, two others retained'},
  {'operation':'memory.resource.dedup_by_content','receiver':'$sessionA','args':['$store',['sdk','中文']],'assert':'1 then 0; one exact original content survives'},
  {'operation':'memory.resource.list','receiver':'$sessionA','args':['$store',['sdk','中文']],'assert':'one item only; truncated false'},
  {'operation':'memory.resource.list_namespaces','receiver':'$sessionA','args':['$store',['sdk']],'assert':'[[sdk,中文]]'},
  {'operation':'memory.resource.get','receiver':'$sessionB','args':['$store',['sdk','中文'],'one'],'expect_error':'owner mismatch, exact actual code to record; A remains valid'},
 ], 'coverage_limits':['Keyword only, no semantic embedder score claim','resource APIs use real native Store, test-only backend dedup is not proof']},
 {'id':'reverse-store-live-authority-replacement','mechanism':'two registered Store callbacks A/B, exact Session Agent set_store changes main CRUD authority', 'steps':[
  {'operation':MEMORY_SET,'receiver':'$agent','args':['$sessionA','$extensionA']},
  {'operation':'memory.store.put','receiver':'$sessionA','args':[['sdk','中文'],'key',{'中文':'🌊','__proto__':{'id':'domain','generation':'literal'}}]},
  {'operation':'memory.store.get','receiver':'$sessionA','args':[['sdk','中文'],'key'],'assert':'A callback read same exact value; trace has Host issued invocation/extension IDs'},
  {'operation':MEMORY_SET,'receiver':'$agent','args':['$sessionA','$extensionB']},
  {'operation':'memory.store.get','receiver':'$sessionA','args':[['sdk','中文'],'key'],'assert':'B missing/null; A old callback is not called after switch'},
  {'operation':'memory.store.put','receiver':'$sessionA','args':[['sdk','中文'],'key','B-new']},
  {'operation':'memory.store.search','receiver':'$sessionA','args':[['sdk','中文'],'B-new',5],'assert':'B key only; A inspect still original'},
  {'operation':'memory.store.delete','receiver':'$sessionA','args':[['sdk','中文'],'key'],'assert':'true then false; B get null'},
 ],'faults':['arm store_put native OSError before mutation; Host returns error, A/B counters unchanged except invocation trace, retry returns new real commit','invalid namespace [sdk,3] fails before reverse callback; preserve typed code/method metadata'],'coverage_limits':['set_store is raw Store replacement, does not install MemoryLayerManager/promoter','do not call unaccepted set_memory_store']},
 {'id':'real-automatic-recall-default-store','mechanism':'enable_memory default real FileStore plus real Run, model input is independent consumer evidence', 'steps':[
  {'operation':'memory.store.put','receiver':'$sessionA','args':[['agent','memories'],'approved-active',active]},
  {'operation':'memory.store.put','receiver':'$sessionA','args':[['agent','memories'],'unapproved-draft',draft]},
  {'operation':'memory.store.put','receiver':'$sessionA','args':[['agent','memories'],'legacy-unapproved',legacy]},
  {'operation':'memory.store.get','receiver':'$sessionA','args':[['agent','memories'],'approved-active'],'assert':'same exact seeded provenance before Run'},
 ], 'runtime_action':'start accepted Run chat SDK-Recall; wait actual settlement; inspect loopback actual model request includes Active marker exactly once in current user memory_context, excludes Draft/Legacy; journal memory_recalled count agrees; afterwards get metadata same content/provenance with telemetry increase only', 'coverage_limits':['fixture seed uses existing generic Store value grammar, not a new typed activation API','approved test fixture data does not claim production activation workflow evidence','separate Store replacement scenario runs fresh Session to avoid prior memory-context projection']},
 {'id':'permission-rules-effects-context-and-session-isolation','mechanism':'same AgentDefinition two real Sessions, independent live PermissionService owners', 'steps':[
  {'operation':'permission.clear_rules','receiver':'$sessionA','args':[]},
  {'operation':'permission.clear_cache','receiver':'$sessionA','args':[]},
  {'operation':'permission.add_rule','receiver':'$sessionA','args':[rule]},
  {'operation':'permission.all_rules','receiver':'$sessionA','args':[],'assert':'exact rule/source/description; B rules empty'},
  {'operation':'permission.check_with_permissions_result_in_mode_and_context','receiver':'$sessionA','args':['candidate_write',{'path':'$work/effect.txt'},['write'],'default',context],'assert':'deny exact reason and updated_input null, no effect file'},
  {'operation':'permission.remove_rule','receiver':'$sessionA','args':[rule],'assert':'true then false'},
  {'operation':'permission.record_approval','receiver':'$sessionA','args':['same-scope','candidate_write',{'x':1},'session']},
  {'operation':'permission.is_approved','receiver':'$sessionA','args':['same-scope','candidate_write',{'x':1}],'assert':'true; x2 false; different scope false; B same exact scope/tool/input false'},
  {'operation':'permission.revoke_cache','receiver':'$sessionA','args':['same-scope','candidate_write'],'assert':'A false; B independent previously recorded approval remains'},
  {'operation':'permission.record_approval','receiver':'$sessionA','args':['same-scope','candidate_write',{'x':1},'once'],'assert':'never cached'},
  {'operation':'permission.record_approval','receiver':'$sessionA','args':['same-scope','candidate_write',{'x':1},'session_tool'],'assert':'A x1/x2 true; B false; revokeA false'},
  {'operation':'permission.stats','receiver':'$sessionA','args':[],'assert':'exact canonical u64 projection counts; ttl_ms records actual configured TTL'},
  {'operation':'permission.cleanup_expired','receiver':'$sessionA','args':[],'assert':'0 when entries younger than actual TTL; do not claim expiration happened'},
 ], 'runtime_action':'register accepted HumanLoopProvider before Sessions; issue context check without deny rule and with strict_confirm; reverse callback must see request/session/agent ids, original args and approval_context fields; answer modified_args scope once and check updated_input identical; do not infer effect executed from check result', 'effect_action':'separate real Agent Run selects registered candidate_write Tool; denial callback means tool counter0/file absent, approval means actual tool counter1/file content; requires fixture model tool-call mode addition before publishing candidate', 'coverage_limits':['no accepted shortTTL setter or fake clock; actual30min boundary opt-in only','PermissionCheck approval_receipt currently omitted by Host check_value; existing accepted response does not expose new field, log source potential contract gap rather than invent JSON','direct facade tool execution alone is not permission enforcement proof']},
]
missing=[]
for scenario in scenarios:
 for step in scenario['steps']:
  if step['operation'] not in accepted:missing.append(step['operation'])
# Family aliases reside under family.operations records in this repository.
if missing:
 for route in catalog['routes']:
  if route.get('handler_operation'):accepted.add(route['handler_operation'])
 missing=sorted(set(step['operation'] for scenario in scenarios for step in scenario['steps'] if step['operation'] not in accepted))
if missing:raise SystemExit('unaccepted planned operation: '+repr(missing))
Path('scenarios.json').write_text(json.dumps({'classification':'unexecuted candidate scenarios, no Host evidence yet','catalog_sha256':hashlib.sha256(catalog_path.read_bytes()).hexdigest(),'scenarios':scenarios,'limitations':['short permission TTL configuration not accepted','no new recall/install/clock API','all actual Host results pending parent approved publish/run']},ensure_ascii=False,indent=2)+'\n')
print('accepted-only scenarios:',len(scenarios),'steps:',sum(len(s['steps']) for s in scenarios),'catalog',hashlib.sha256(catalog_path.read_bytes()).hexdigest())
