"""Compare actual language evidence without treating clocks/opaque IDs as equal."""
import json
from pathlib import Path

ROOT=Path(__file__).parent
EXPECTED_HOST='cb9b7fcad3946d9428cdef5a7b7258a334ee387ed34ca0612b4f40b262ef20a2'


def decode_wire(value):
    kind=value['kind']
    if kind=='null':return None
    if kind in {'string','bool','f64','u64','i64'}:return value['value']
    if kind=='map':return {decode_wire(item['key']):decode_wire(item['value'])for item in value['value']}
    if kind=='list':return [decode_wire(item)for item in value['value']]
    raise AssertionError('comparison needs an explicit decoder for '+kind)


def normalize(value,work):
    if isinstance(value,list):return [normalize(item,work)for item in value]
    if isinstance(value,dict):
        result={key:normalize(item,work)for key,item in value.items()}
        if 'namespace'in value and 'key'in value and 'created_at'in value:
            for field in ('created_at','updated_at','last_accessed'):
                if value.get(field)is not None:
                    assert isinstance(value[field],str)and value[field].isascii()and value[field].isdigit()
                    assert int(value[field])<(1<<64)
                    result[field]='$OBSERVED_CANONICAL_U64_TIME'
        if 'memory_type'in value and value.get('last_recalled_at')is not None:
            assert isinstance(value['last_recalled_at'],str)and value['last_recalled_at'].isdigit()
            result['last_recalled_at']='$OBSERVED_CANONICAL_U64_RECALL_TIME'
        return result
    if isinstance(value,str):return value.replace(work,'$WORK')
    return value


summaries={};canonical=[]
for language in ('python','typescript','java'):
    doc=json.loads((ROOT/(language+'-real-evidence.json')).read_text())
    assert doc['host_sha256']==EXPECTED_HOST
    rows=doc['results'];work=doc['work']
    operations=[]
    for row in rows:
        if'operation'not in row or'value'not in row:continue
        value=normalize(row['value'],work)
        if row['operation']=='memory.resource.search_with':
            assert {item['key']for item in value['items']}=={'one','two'}
            value['items'].sort(key=lambda item:item['key'])
        if row['operation']=='memory.resource.list':
            assert len(value['items'])==1 and value['items'][0]['key']in{'one','two'}
            assert value['items'][0]['value']=={'content':'duplicate-needle中文🌊'}
            value['items'][0]['key']='$ORIGINAL_DUPLICATE_CONTENT_SURVIVOR'
        operations.append({'operation':row['operation'],'value':value})
    errors=[normalize(row,work)for row in rows if'negative'in row]
    assert [row['code']for row in errors]==['invalid_value','framework_error']
    assert all(row['retryable']=='never'for row in errors)
    assert all('handle'in row and'details'in row for row in errors)
    context=next(row for row in rows if'permission_reverse_context'in row)
    wire=context['permission_reverse_context']['invocation']['input']
    assert wire['request_id']=='permission-native-request'
    assert wire['agent_name']=='memory-fixture'
    assert wire['tool_name']=='candidate_write'
    assert wire['timeout']=={'seconds':'1','nanos':0}
    approval=decode_wire(wire['approval_context'])
    assert approval['working_directory']==work
    assert approval['affected_files']==[work+'/effect.txt']
    assert approval['estimated_impact']=='one fixture file'
    assert approval['metadata']=={'中文':'🌊','__proto__':'plain data'}
    assert decode_wire(wire['args'])=={'path':work+'/context.txt'}
    assert context['returned_check']=={'decision':'allow','updated_input':{'path':work+'/modified.txt','中文':'🌊'}}
    assert context['approval_receipt_present']is False
    assert str(next(row['actual_ttl_ms']for row in rows if'actual_ttl_ms'in row))=='1800000'
    effect=next(row['actual_permission_effect']for row in rows if'actual_permission_effect'in row)
    assert effect['denied_file_exists']is False and effect['denied_tool_calls']==0
    assert len(effect['approved_tool_calls'])==1
    assert effect['approved_tool_calls'][0]['phase']=='allow'
    assert Path(work+'/effect-allow.txt').read_text()=='accepted permission effect 中文🌊'
    assert not Path(work+'/effect-deny.txt').exists()
    assert effect['denial_settlement']['settled']and effect['approval_settlement']['settled']
    backend=next(row['backend_inspection']for row in rows if'backend_inspection'in row)
    assert [row['authority']for row in backend['store_calls'][:2]]==['A','A']
    assert all(row['authority']=='B'for row in backend['store_calls'][2:])
    assert [row['operation']for row in backend['store_calls']]==['store_put','store_get','store_get','store_put','store_search_with','store_delete','store_delete','store_put','store_get','store_put','store_get']
    recall=next(row for row in rows if'actual_model_recall'in row)
    user_contents=[item.get('content','')for item in recall['model_request']['messages']if item['role']=='user']
    joined='\n'.join(user_contents)
    assert joined.count('SDK-Recall-Active 中文🌊')==1
    assert 'SDK-Recall-Draft'not in joined and 'SDK-Recall-Legacy'not in joined
    assert '[memory_context]'in joined
    events=[item.get('envelope',item)for item in recall['journal']['events']]
    assert [int(item['sequence'])for item in events]==list(range(1,len(events)+1))
    assert len({item['event_id']for item in events})==len(events)
    assert len({item['stream_id']for item in events})==1
    memory_events=[item for item in events if item['payload']['event_type']=='memory_recalled']
    assert len(memory_events)==1
    assert decode_wire(memory_events[0]['payload']['data'])=={'count':'1'}
    assert sum(item['payload']['event_type']=='final_answer'for item in events)==1
    assert all(item['payload']['event_type']not in{'error','cancelled'}for item in events)
    canonical.append((operations,errors,normalize(approval,work),normalize(effect['approved_tool_calls'],work)))
    summaries[language]={'recorded_rows':len(rows),'family_operation_results':len(operations),'typed_failures':len(errors),'store_callback_operations':len(backend['store_calls']),'real_run_effect_assertions':True,'actual_model_recall_assertions':True,'journal_memory_recalled_count':1,'actual_permission_ttl_ms':1800000,'ttl_expiration_observed':False,'permission_approval_receipt_returned':False}
for index in (1,2):
    if canonical[0]!=canonical[index]:
        for part,(left,right)in enumerate(zip(canonical[0],canonical[index])):
            if left!=right:
                print('mismatch language',index,'part',part)
                if part==0:
                    for n,(a,b)in enumerate(zip(left,right)):
                        if a!=b:print('row',n,a,b)
                else:print(left,right)
assert canonical[0]==canonical[1]==canonical[2],'same scenarios must agree across languages after only documented clock/root normalization'
(ROOT/'cross-language-comparison.json').write_text(json.dumps({'classification':'actual temporary real Host consumer evidence; not final repository CI/main acceptance','host_sha256':EXPECTED_HOST,'languages':summaries,'comparisons':['same ordered operation/result values','same complete captured typed error code/method/handle/details/retry metadata','actual context fields and modified args retained','all11Storecallback operation/authority trace patterns agree','actual denied versus approved Tool file effects','Active recalled once in actual user model request; Draft/Legacy excluded'],'normalization':['independent isolated fixture root paths','only observed Store created_at/updated_at/last_accessed and recall telemetry timestamps; each canonicalu64 checked before normalization','equal-score resource search ordered by original key; native hash-map tie order has no ordering promise','dedup survivor validated as one exact original key/content, then compared by content equivalence; chosen key is unspecified'],'limitations':['actual TTL expiration not observed; accepted contract exposes no shortTTL setter','approval_receipt is new framework field not in current acceptedfield inventory; no automatic API expansion','generic numeric Store callback roundtrip must preserve WireValue type; current fixture does not claim arbitraryu64store-nativeprojection coverage']},indent=2)+'\n')
print(json.dumps(summaries,indent=2))
