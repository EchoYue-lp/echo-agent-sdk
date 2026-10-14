import com.echoagent.sdk.*;
import com.fasterxml.jackson.databind.*;
import com.fasterxml.jackson.databind.node.*;
import java.nio.file.*;
import java.security.MessageDigest;
import java.util.*;
import java.util.concurrent.*;
public final class ChartBounds {
 static final ObjectMapper J=new ObjectMapper();static final Path S=Path.of("/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance");
 static <T>T await(CompletionStage<T> s)throws Exception{return s.toCompletableFuture().get(15,TimeUnit.SECONDS);}
 static String sha(Path p)throws Exception{return HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256").digest(Files.readAllBytes(p)));}
 static ObjectNode snapshot(Path p)throws Exception{var o=J.createObjectNode();for(String n:List.of("stats.csv","input.csv","lookup.csv","quality.csv","input.txt","sentinel.txt","null-index.csv","collision.csv"))o.put(n,sha(p.resolve(n)));return o;}
 public static void main(String[] argv)throws Exception{
  Path d=Path.of(argv[0]),work=d.resolve("java-runtime");String host=System.getenv("ECHO_AGENT_SDK_HOST");var prep=new ProcessBuilder("python3",d.resolve("prepare.py").toString(),work.toString()).inheritIO().start();if(prep.waitFor()!=0)throw new IllegalStateException("prepare failed");var before=snapshot(work);var rows=J.createArrayNode();JsonNode identity;List<String> processes;
  try(var sdk=await(EchoAgentClient.spawn(host,List.of("--config",work.resolve("host.json").toString()),S.resolve("sdks/shared/facade-operation-catalog.json"),S.resolve("sdks/shared/contract-digests.json"),Map.of()))){
   var agent=await(sdk.createAgent());var session=await(agent.createSession(work.toString()));var id=J.createObjectNode();id.set("agent",WireValues.value(agent.wire()).path("value"));id.set("session",WireValues.value(session.wire()).path("value"));id.set("capability",sdk.capability());identity=id;var ps=new ProcessBuilder("ps","-axo","pid,ppid,command").start();processes=new String(ps.getInputStream().readAllBytes(),java.nio.charset.StandardCharsets.UTF_8).lines().filter(l->l.contains(host)&&!l.contains("ps -axo")).toList();ps.waitFor();
   for(JsonNode step:J.readTree(work.resolve("bounds.json").toFile())){
    var args=new ArrayList<Object>();int index=0;for(JsonNode v:step.path("arguments")){Object value=J.convertValue(v,Object.class);if((index==6||index==7)&&v.isIntegralNumber())value=Long.valueOf(v.longValue());args.add(value);index++;}
    var row=J.createObjectNode().put("id",step.path("id").asText()).put("operation",step.path("operation").asText());var wire=row.putArray("arguments_wire");for(Object v:args)wire.add(WireValues.value(v));
    try{row.set("result",await(session.invoke(step.path("operation").asText(),args)));}
    catch(ExecutionException ex){if(!(ex.getCause() instanceof EchoAgentException error))throw ex;var er=row.putObject("error").put("code",error.code()).put("message",error.getMessage()).put("operation",error.operation()).put("retryable",error.retryable());er.set("handle",error.handle()==null?NullNode.instance:WireValues.value(error.handle()).path("value"));er.set("details",error.details()==null?NullNode.instance:error.details());}
    rows.add(row);
   }
   await(session.closeAsync());await(agent.closeAsync());
  }
  var out=J.createObjectNode().put("language","java").put("host",host).put("host_sha256",sha(Path.of(host))).put("pid",ProcessHandle.current().pid()).put("work",work.toString());out.set("hostProcess",J.valueToTree(processes));out.set("identity",identity);out.set("protected_before",before);out.set("protected_after",snapshot(work));out.set("rows",rows);J.writerWithDefaultPrettyPrinter().writeValue(d.resolve("java.json").toFile(),out);System.out.println("observed "+rows.size()+" native Chart calls");
 }
}
