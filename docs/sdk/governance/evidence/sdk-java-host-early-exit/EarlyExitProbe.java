import com.agentclientprotocol.sdk.client.transport.*;
import com.agentclientprotocol.sdk.json.*;
import com.agentclientprotocol.sdk.spec.*;
import com.echoagent.sdk.*;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.nio.file.*;
import java.time.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.Function;

class EarlyExitProbe {
 public static void main(String[] args)throws Exception {
  Path root=Path.of(args[0]);ObjectMapper json=new ObjectMapper();var rows=new ArrayList<Map<String,Object>>();
  for(String mode:List.of("before-initialize","during-initialize")){
   Path pidFile=root.resolve(mode+".pid");String program="import os,sys,pathlib;pathlib.Path(sys.argv[1]).write_text(str(os.getpid()));"+(mode.equals("during-initialize")?"sys.stdin.readline();":"")+"sys.stderr.write('controlled child exit before initialize response\\n');sys.stderr.flush();sys.exit(7)";
   var transport=new StdioAcpClientTransport(AgentParameters.builder("python3").args(List.of("-c",program,pidFile.toString())).build());var stderr=new CopyOnWriteArrayList<String>();transport.setStdErrorHandler(stderr::add);
   var session=new AcpClientSession(Duration.ofSeconds(30),transport,Map.of(),Map.of(),Function.identity());
   var pending=session.sendRequest("initialize",json.createObjectNode().put("protocolVersion",1),new TypeRef<JsonNode>(){}).toFuture();
   var field=StdioAcpClientTransport.class.getDeclaredField("process");field.setAccessible(true);Process child=(Process)field.get(transport);boolean exited=child.waitFor(2,TimeUnit.SECONDS);
   String outcome;try{pending.get(2,TimeUnit.SECONDS);outcome="unexpected-success";}catch(TimeoutException e){outcome="pending-after-real-child-exit";}catch(ExecutionException e){outcome=e.getCause().toString();}
   rows.add(new LinkedHashMap<>(Map.of("layer","official-ACP","mode",mode,"child_pid",child.pid(),"child_exited",exited,"child_alive",child.isAlive(),"child_exit",child.exitValue(),"initialize_done",pending.isDone(),"outcome",outcome,"stderr",stderr)));
   session.close();transport.closeGracefully().block(Duration.ofSeconds(3));
  }
  Path pid=root.resolve("public-sdk.pid");String program="import os,sys,pathlib;pathlib.Path(sys.argv[1]).write_text(str(os.getpid()));sys.stdin.readline();sys.stderr.write('controlled SDK child exited\\n');sys.stderr.flush();sys.exit(7)";
  var pending=EchoAgentClient.spawn("python3",List.of("-c",program,pid.toString()),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of()).toCompletableFuture();
  long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(2);while(!Files.exists(pid)&&System.nanoTime()<deadline)Thread.sleep(5);long childPid=Long.parseLong(Files.readString(pid));var handle=ProcessHandle.of(childPid);if(handle.isPresent())handle.get().onExit().get(2,TimeUnit.SECONDS);
  String outcome;try{pending.get(2,TimeUnit.SECONDS);outcome="unexpected-success";}catch(TimeoutException e){outcome="pending-after-real-child-exit";}catch(ExecutionException e){outcome=e.getCause().toString();}
  rows.add(new LinkedHashMap<>(Map.of("layer","public-SDK","child_pid",childPid,"child_alive",ProcessHandle.of(childPid).map(ProcessHandle::isAlive).orElse(false),"initialize_done",pending.isDone(),"outcome",outcome)));
  json.writerWithDefaultPrettyPrinter().writeValue(root.resolve("observed.json").toFile(),rows);System.out.println(json.writerWithDefaultPrettyPrinter().writeValueAsString(rows));
 }
}
