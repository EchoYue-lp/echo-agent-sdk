"""Only exercises the candidate HTTP helper, never starts or substitutes Host."""
import json
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path
with tempfile.TemporaryDirectory() as tmp:
    work=Path(tmp)
    child=subprocess.Popen([sys.executable,'fixture.py','--work',str(work)])
    try:
        end=time.monotonic()+5
        while not (work/'ready').exists():
            if time.monotonic()>end:raise TimeoutError('HTTP fixture startup')
            time.sleep(.01)
        url=json.loads((work/'backend.json').read_text())['url']
        def post(path,value):
            request=urllib.request.Request(url+path,json.dumps(value).encode(),{'Content-Type':'application/json'})
            with urllib.request.urlopen(request,timeout=3)as response:return json.load(response)
        result=post('/store',{'authority':'A','operation':'store_put','input':{'namespace':['sdk'],'key':'x','value':{'中文':'🌊'}}})
        assert result['result']['value']is None
        assert post('/store',{'authority':'A','operation':'store_get','input':{'namespace':['sdk'],'key':'x'}})['result']['value']['value']=={'中文':'🌊'}
        reply=post('/v1/chat/completions',{'messages':[{'role':'user','content':'record-only 中文🌊'}]})
        assert reply['choices'][0]['message']['content']=='SDK consumer complete'
        with urllib.request.urlopen(url+'/inspect',timeout=3)as response:inspection=json.load(response)
        assert len(inspection['store_calls'])==2 and len(inspection['model_requests'])==1
        print('HTTP candidate only: Store put/get and actual request recording passed; no Host proof')
    finally:
        child.terminate()
        child.wait(timeout=3)
