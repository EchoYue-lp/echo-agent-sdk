from pathlib import Path
import hashlib,json,sys
D=Path(__file__).parent
languages=['typescript','python','java'];normal={};checks=[]
for lang in languages:
 d=json.loads((D/(lang+'.json')).read_text());rows=d['rows'];nr=[]
 checks.append({'language':lang,'check':'protected-file-hashes','passed':d['protected_before']==d['protected_after']})
 for r in rows:
  row={'id':r['id'],'operation':r['operation'],'arguments_wire':r['arguments_wire']}
  if 'result' in r:
   spec=json.loads(r['result']['output']);row.update({'success':r['result']['success'],'spec':spec})
  else:row['error']=r['error']
  nr.append(row)
 normal[lang]=nr
 for r in nr:
  bad=r['id'] in ['width-overflow','height-overflow','width-negative','height-negative']
  if bad:
   er=r.get('error',{});details=er.get('details',{});passed=er.get('code')=='framework_error' and er.get('operation')=='generate_chart' and er.get('retryable')=='never' and er.get('handle') is None and details=={'fields':[{'key':'framework_failure_category','value':'tool'},{'key':'framework_terminal_kind','value':'failed'},{'key':'framework_failure_code','value':'tool'},{'key':'framework_retryable','value':'false'}]} and ((r['id']=='width-overflow' and '4294967296 is greater than the maximum of 4294967295' in er.get('message','')) or (r['id']=='height-overflow' and '4294967297 is greater than the maximum of 4294967295' in er.get('message','')) or ('negative' in r['id'] and '-1 is less than the minimum of 0' in er.get('message','')));reason='out-of-range dimensions reject with original typed operation/category/retryability and exact numerical boundary'
  elif 'type' in r['id']:passed='error' in r and r['error']['code']=='framework_error';reason='existing integer schema rejects wrong type'
  else:
   expected={'width-u32-max':(4294967295,123),'height-u32-max':(321,4294967295),'zero':(0,0),'null':(600,400),'omitted-dimensions':(600,400)}[r['id']]
   passed=r.get('success') is True and (r['spec']['width'],r['spec']['height'])==expected and r['spec']['title']=='中😀边界' and r['spec']['data']['values']==[{'label':'中文😀','value':1},{'label':'尾','value':4}];reason='accepted dimension/default and Unicode payload control'
  checks.append({'language':lang,'check':r['id'],'passed':passed,'reason':reason})
same=normal['typescript']==normal['python']==normal['java']
print('strict normalized three-language equality',same)
report={'three_language_exact_normalized_equality':same,'normalized':normal,'checks':checks,'behavior_counterexamples_per_language':{lang:sum(not x['passed'] for x in checks if x['language']==lang) for lang in languages},'scope':'real native after exact published framework Chart pin; strict original typed rejection and accepted controls','source_pin':'be5f0a8405b593c4ef964cd6fd2df4beabf4956c','host_sha256':'bcb0389ae8d8a946fab9bb123945100722711029bafa8d9d6cadf95a76c9c345'}
(D/'comparison.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(report['behavior_counterexamples_per_language']))
for x in checks:
 if not x['passed']:print('FAIL',x['language'],x['check'],x['reason'])
raise SystemExit(1 if any(not x['passed'] for x in checks) or not same else 0)
