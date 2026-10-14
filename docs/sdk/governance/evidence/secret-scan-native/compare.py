"""Exact three-native Security value/error proof; original raw objects are retained."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parent
facts={language:json.loads((ROOT/language/'observed.json').read_text()) for language in ('typescript','python','java')}
normalized={}
for language,value in facts.items():
    mapping={value['identity']['agent']['id']:'$agent',value['identity']['session']['id']:'$session'}
    def walk(item):
        if isinstance(item,str):return mapping.get(item,item)
        if isinstance(item,list):return [walk(child) for child in item]
        if isinstance(item,dict):return {key:walk(child) for key,child in item.items()}
        return item
    # All row/result/error/request fields are kept. Only opaque issued IDs differ.
    rows=walk(value['rows'])
    raw=walk(value['raw_scan'])
    entries=raw['value']['value'][0]['value']
    entries.sort(key=lambda item:item['key']['value'])
    normalized[language]={'rows':rows,'raw_scan':raw,'before':value['before'],'after':value['after'],'provider_calls':value['provider_calls'],'identity':walk(value['identity'])}
    (ROOT/language/'normalized.json').write_text(json.dumps(normalized[language],ensure_ascii=False,indent=2)+'\n')
assert normalized['typescript']==normalized['python']==normalized['java']
(ROOT/'comparison.json').write_text(json.dumps({'full_values_errors_requests_and_raw_scan_equal':True,'normalization':'only opaque issued Agent/Session IDs and complete WireValue map entry ordering; every scalar type, error field, generation and source operation retained','values':normalized},ensure_ascii=False,indent=2)+'\n')
print('Three languages match complete Security values, byte-position string, typed errors and zero-effect facts.')
