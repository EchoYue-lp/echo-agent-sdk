from pathlib import Path
import json,re,hashlib,shutil
root=Path('/tmp/sdk-mcp-channel-candidate-fixture');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();runs=[];normalized={}
for lang,logname in [('typescript','sdk-mcp-channel-native-owned-processes.log'),('python','sdk-mcp-channel-python-four-green.log'),('java','sdk-mcp-channel-java-four.log')]:
 log=Path('/tmp')/logname;text=log.read_text();works=re.findall(r'MCP/channel evidence ([^\n]+)',text);assert len(works)==4
 dest=root/'three-language-evidence'/lang;dest.mkdir(parents=True,exist_ok=True);shutil.copyfile(log,dest/'run.log');scenarios={};files=[]
 for work in works:
  work=Path(work);name=work.name
  kind='mcp-stdio' if 'mcp-stdio' in name else 'mcp-transport-retry' if 'mcp-transport' in name else 'channels-normal' if 'channels-normal' in name else 'channels-retry'
  output=dest/kind;output.mkdir(exist_ok=True)
  for p in work.iterdir():
   if p.is_file() and p.name!='host.json':shutil.copyfile(p,output/p.name);files.append({'path':str(output/p.name),'sha256':sha(output/p.name)})
  observed=json.loads((output/'observed.json').read_text())
  if kind=='mcp-stdio':
   wanted={'server_capabilities','tools','resources','list_resources','list_resource_templates','read_resource','get_prompt','call_tool'}
   shared=[row for row in observed if row.get('operation','').rsplit('::',1)[-1] in wanted];assert len(shared)==9
   cleanup=next(row['process_cleanup'] for row in observed if 'process_cleanup' in row);assert all(cleanup.values());scenarios[kind]={'results':shared,'process_cleanup':cleanup}
  elif kind=='mcp-transport-retry':
   if lang=='java':summary=json.loads((output/'summary.json').read_text());calls=observed['calls']
   else:summary=observed;calls=observed['calls']
   assert summary['closeCount']==2 and summary['released_quota_refilled']==4
   scenarios[kind]={'closeCount':2,'released_quota_refilled':4,'operation_order':[c['operation'] for c in calls]}
  else:
   summary=json.loads((output/'summary.json').read_text()) if lang=='java' else observed
   calls=observed['calls'];sent=[c for c in calls if c['operation']=='channel_send'];assert len(sent)==1
   started=sorted([{'id':c['id'],'input':c['input']} for c in calls if c['operation']=='channel_start'],key=lambda c:c['id'])
   scenarios[kind]={'send':sent[0],'starts':started,'stops':summary['stops'],'running':summary['running'],'inbound_handler_calls':summary['inbound_handler_calls']}
 normalized[lang]=scenarios;(dest/'normalized.json').write_text(json.dumps(scenarios,ensure_ascii=False,indent=2,sort_keys=True)+'\n');runs.append({'language':lang,'exit_code':0,'actual_scenarios':4,'execution_kind':'public SDK native client calls; Java isolated javac/main assertions, not JUnit count' if lang=='java' else 'native public SDK test4passed0skip','roots':works,'log_sha256':sha(log),'raw_files':files,'normalized_sha256':sha(dest/'normalized.json')})
assert normalized['typescript']==normalized['python']==normalized['java']
p={'host':'/tmp/echo-sdk-governance-workflow-shape-host','host_sha256':sha('/tmp/echo-sdk-governance-workflow-shape-host'),'runs':runs,'all_three_common_projection_match':True,'common_projection':'Exact 9 MCP metadata/resource/template/prompt/tool result values and process cleanup facts; transport close callback count/order+quota reuse; exact channel start/send attachments and stop counts/running/inbound0. PID/handles/times retained raw, not compared as cross-process identifiers.','sources':[{'path':str(p),'sha256':sha(p)} for p in root.iterdir() if p.is_file() and p.suffix in ['.py','.js','.java','.md']],'known_red':'real ChannelHealth callback→manager.ids same-manager circular wait; candidate-proof.json retained1red not green','scope_limits':['4scenarios only, not all accepted operations','MessageHandler accepted inbound consumer unresolved, never fake input','held-close/caller-cancel cleanup and all aliases publication fault not yet proved','framework owned channel read lease design pending root; production unmodified','local patched framework/Host not published pin/main']};(root/'three-language-proof.json').write_text(json.dumps(p,ensure_ascii=False,indent=2)+'\n');print('four common scenario projections exactmatch three natives')
