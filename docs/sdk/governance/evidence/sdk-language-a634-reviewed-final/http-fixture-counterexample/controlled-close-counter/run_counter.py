from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import threading, socket, json, subprocess, hashlib
root=Path(__file__).resolve().parent
for explicit_close in (False,True):
 events=[]; lock=threading.Lock();
 def event(**item):
  with lock: events.append(item)
 class Handler(BaseHTTPRequestHandler):
  def do_POST(self):
   body=self.rfile.read(int(self.headers.get('Content-Length','0')))
   event(kind='request',peer=list(self.client_address),path=self.path,body=body.decode())
   reply=b'{"result":null}'
   self.send_response(200); self.send_header('Content-Type','application/json');self.send_header('Content-Length',str(len(reply)))
   if explicit_close:self.send_header('Connection','close')
   self.end_headers();self.wfile.write(reply);self.wfile.flush()
   event(kind='response_flushed',peer=list(self.client_address),protocol=self.protocol_version,explicit_close=explicit_close)
   self.connection.settimeout(3)
   try:
    peek=self.connection.recv(4096,socket.MSG_PEEK)
    event(kind='before_nonpersistent_close',peer=list(self.client_address),peek=peek.decode(errors='replace'),same_connection_second_post=peek.startswith(b'POST '))
   except Exception as e:event(kind='close_observation_error',peer=list(self.client_address),error=repr(e))
  def log_message(self,*args):pass
 server=ThreadingHTTPServer(('127.0.0.1',0),Handler);thread=threading.Thread(target=server.serve_forever,daemon=True);thread.start()
 label='explicit-close' if explicit_close else 'original-no-close-header'
 run=subprocess.run(['/Library/Java/JavaVirtualMachines/jdk-21.jdk/Contents/Home/bin/java','-Djdk.httpclient.HttpClient.log=errors,requests,headers','-cp',str(root),'Counter',f'http://127.0.0.1:{server.server_port}'],capture_output=True,text=True,timeout=15)
 (root/(label+'.log')).write_text(run.stdout+run.stderr)
 server.shutdown();server.server_close();thread.join()
 (root/(label+'.json')).write_text(json.dumps({'exit_code':run.returncode,'events':events,'stdout':run.stdout},indent=2)+'\n')
 print(json.dumps({'label':label,'exit_code':run.returncode,'events':events}))
