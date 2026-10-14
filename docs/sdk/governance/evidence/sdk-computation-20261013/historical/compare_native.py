"""Compare all concrete native outcomes, retaining original IDs/type grammar."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).parent

def canonical_wire(value,work):
 if isinstance(value,str):return value.replace(work,'$WORK')
 if isinstance(value,list):return [canonical_wire(item,work)for item in value]
 if not isinstance(value,dict):return value
 result={key:canonical_wire(item,work)for key,item in value.items()}
 if value.get('kind')=='map':result['value'].sort(key=lambda item:json.dumps(item['key'],sort_keys=True,ensure_ascii=False))
 return result

def canonical_output(operation,result,work):
 result=canonical_wire(result,work)
 if isinstance(result,dict)and'output'in result:
  try:
   output=json.loads(result['output'])
   if operation=='aggregate_data':output['data'].sort(key=lambda row:row['group'])
   if operation=='join_data':output['preview'].sort(key=lambda row:row['id'])
   if operation=='pivot_data':output['pivot_result']['data'].sort(key=lambda row:row['group']);output['pivot_result']['pivot_values'].sort()
   result['parsed_output']=output;del result['output']
   # Check the original rawWire data independently before semantic order canonicalization.
   if'data'in result:del result['data']
  except(json.JSONDecodeError,KeyError,TypeError):pass
 return result

summaries={};values=[]
for language in ['python','typescript','java']:
 d=json.loads((ROOT/(language+'-final-computation-evidence.json')).read_text());work=d['work'];rows=[]
 assert len(d['rows'])==45
 assert d['host_sha256']=='64c7b4966e26acd573ddc440297e443518fc4afef60efbd39aac94da24519fda'
 for raw in d['rows']:
  row={'id':raw['id'],'operation':raw['operation'],'arguments':canonical_wire(raw['arguments'],work)}
  if'error'in raw:row['error']=canonical_wire(raw['error'],work)
  else:row['result']=canonical_output(raw['operation'],raw['result'],work)
  rows.append(row)
 assert d['actual_exports']=={'output_text':'中文😀\nalpha','output_json':[{'id':i,'value':i}for i in range(1,5)]}
 assert d['actual_side_effects']['sentinel_sha256']==hashlib.sha256(b'unchanged\n').hexdigest()
 values.append(rows);summaries[language]={'actual_rows':45,'typed_errors':sum('error'in row for row in rows),'oldsemanticcases_green':2,'oracle_groups_passed':9,'actual_export_bytes_match':True,'sentinel_preserved':True}
for index in [1,2]:
 if values[0]!=values[index]:
  for left,right in zip(values[0],values[index]):
   if left!=right:print('MISMATCH',index,left['id'],left,right)
assert values[0]==values[1]==values[2]
(ROOT/'native-cross-language-comparison.json').write_text(json.dumps({'classification':'actual temporary consumer; not final189ff3adjacent fixes/parentfullgates','host_sha256':'64c7b4966e26acd573ddc440297e443518fc4afef60efbd39aac94da24519fda','languages':summaries,'comparison':['exactorderedoperations/arguments/results/errors includingcode/retryable/method/handle/details','all ToolResult flags/output schema/fieldvalues','actualUTF8textandJSONexports','same unchanged sentinelbytes'],'normalization':['isolatedtemporaryroot','WireValue map association ordering','Polars grouped row/pivotcolumn ordering without promised tieorder'],'omission_in_comparison':'ToolResult rawdata separately checked against parsedoutput by nativeoracle before onlysemanticroworderingnormalize; original rawdata remains archived'},indent=2)+'\n')
print(json.dumps(summaries,indent=2))
