"""Deterministic actual MCP stdio process, not a SDK/Host protocol emulator."""
import json
import os
import sys
from pathlib import Path
work=Path(sys.argv[1]);label=sys.argv[2]
(work/(label+'.pid')).write_text(str(os.getpid()))
resources=[{'uri':'fixture://中文/text','name':'中文🙂','description':'text meta','mimeType':'text/plain'}, {'uri':'fixture://binary','name':'binary','mimeType':'application/octet-stream'}]
tools=[{'name':'echo','description':'echo Unicode','inputSchema':{'type':'object','properties':{'text':{'type':'string'}},'required':['text']}}]
prompts=[{'name':'compose','description':'prompt 中文🙂','arguments':[{'name':'topic','description':'主题','required':True}]}]
for line in sys.stdin:
 request=json.loads(line)
 with (work/(label+'-requests.jsonl')).open('a') as f:f.write(json.dumps(request,ensure_ascii=False)+'\n')
 if 'id' not in request:continue
 method=request['method'];params=request.get('params',{})
 if method=='initialize':result={'protocolVersion':'2024-11-05','capabilities':{'tools':{},'resources':{'subscribe':False,'listChanged':False},'prompts':{'listChanged':False}},'serverInfo':{'name':'same-name','version':'1'}}
 elif method=='tools/list':result={'tools':tools}
 elif method=='resources/list':result={'resources':resources}
 elif method=='prompts/list':result={'prompts':prompts}
 elif method=='resources/templates/list':result={'resourceTemplates':[{'uriTemplate':'fixture://item/{id}','name':'模板🙂','description':'template','mimeType':'text/plain'}]}
 elif method=='resources/read':result={'contents':[{'uri':params['uri'],'mimeType':'application/octet-stream','blob':'AP9BgA=='}]} if params['uri']=='fixture://binary' else {'contents':[{'uri':params['uri'],'mimeType':'text/plain','text':'资源 中文🙂'}]}
 elif method=='prompts/get':result={'description':'topic prompt','messages':[{'role':'user','content':{'type':'text','text':'主题 '+params['arguments']['topic']}}]}
 elif method=='tools/call':result={'content':[{'type':'text','text':params['arguments']['text']}],'isError':False}
 elif method=='ping':result={}
 else:
  print(json.dumps({'jsonrpc':'2.0','id':request['id'],'error':{'code':-32601,'message':'unknown method'}}),flush=True);continue
 print(json.dumps({'jsonrpc':'2.0','id':request['id'],'result':result},ensure_ascii=False),flush=True)
(work/(label+'.stdin-eof')).touch()
