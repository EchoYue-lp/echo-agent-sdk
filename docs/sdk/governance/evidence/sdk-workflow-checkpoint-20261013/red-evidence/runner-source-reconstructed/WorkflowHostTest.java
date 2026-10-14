// DRAFT. Intended final path sdks/java/src/test/java/com/echoagent/sdk/WorkflowHostTest.java.
package com.echoagent.sdk;
import org.junit.jupiter.api.Test;
import com.fasterxml.jackson.databind.JsonNode;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.CompletionException;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

class WorkflowHostTest {
 private static String helper(Path fixture,Path work,String... arguments)throws Exception{
  var args=new ArrayList<String>(List.of("python3",fixture.toString(),"--work",work.toString()));args.addAll(List.of(arguments));
  var child=new ProcessBuilder(args).redirectErrorStream(true).start();var output=new String(child.getInputStream().readAllBytes(),java.nio.charset.StandardCharsets.UTF_8);
  assertTrue(child.waitFor(10,TimeUnit.SECONDS));assertEquals(0,child.exitValue(),output);return output;
 }
 @Test void routingCheckpointClaimsBranchOwnerAndShapeBoundary()throws Exception{
  var host=System.getenv("ECHO_AGENT_SDK_HOST");if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")))assertNotNull(host,"required Workflow Host missing");assumeTrue(host!=null&&!host.isBlank());
  var fixture=Path.of("/tmp/sdk-workflow-candidate-fixture/workflow_fixture.py");var work=Files.createTempDirectory("sdk-java-workflow-");EchoAgentClient sdk=null;boolean passed=false;
  try{
   helper(fixture,work,"--prepare");var scenario=JsonSupport.MAPPER.readTree(Files.readString(work.resolve("scenario.json")));
   sdk=EchoAgentClient.spawn(host,List.of("--config",work.resolve("host.json").toString()),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of(),List.of(),List.of()).toCompletableFuture().get(10,TimeUnit.SECONDS);
   var agent=sdk.createAgent().toCompletableFuture().get(10,TimeUnit.SECONDS);var sessions=new HashMap<String,SessionHandle>();sessions.put("a",agent.createSession().toCompletableFuture().get(10,TimeUnit.SECONDS));sessions.put("b",agent.createSession().toCompletableFuture().get(10,TimeUnit.SECONDS));
   var rows=JsonSupport.MAPPER.createArrayNode();var results=JsonSupport.MAPPER.createObjectNode();
   for(var step:scenario.path("steps")){
    var session=sessions.get(step.path("session").textValue());var row=JsonSupport.MAPPER.createObjectNode().put("id",step.path("id").textValue());
    if(step.path("action").asText().equals("close")){session.close();row.put("outcome","closed");rows.add(row);results.set(step.path("id").textValue(),row);Files.writeString(work.resolve("results.json"),rows.toString());continue;}
    var resolution=work.resolve("resolve.json");var request=JsonSupport.MAPPER.createObjectNode();request.set("value",step.path("arguments"));request.set("results",results);Files.writeString(resolution,request.toString());
    var resolved=JsonSupport.MAPPER.readTree(helper(fixture,work,"--resolve",resolution.toString()));Files.writeString(work.resolve("call-"+step.path("id").textValue()+".json"),JsonSupport.MAPPER.createObjectNode().put("operation",step.path("operation").textValue()).set("arguments",resolved).toString());var arguments=new ArrayList<Object>();for(var argument:resolved)arguments.add(argument.isObject()&&argument.has("kind")&&java.util.Set.of("null","bool","string","i64","u64","f64","bytes","duration","timestamp","path","handle","list","map","record","variant","unknown").contains(argument.path("kind").asText())?argument:JsonSupport.MAPPER.convertValue(argument,Object.class));
    try{JsonNode value=sdk.call(step.path("operation").textValue(),session.wire(),arguments).toCompletableFuture().get(20,TimeUnit.SECONDS);row.put("outcome","result").set("value",value);}
    catch(ExecutionException error){Throwable cause=error.getCause();while(cause instanceof CompletionException&&cause.getCause()!=null)cause=cause.getCause();var typed=assertInstanceOf(EchoAgentException.class,cause);row.put("outcome","error").put("code",typed.code()).put("message",typed.getMessage()).put("operation",typed.operation()).put("retryable",typed.retryable()).set("details",typed.details());}
    rows.add(row);results.set(step.path("id").textValue(),row);Files.writeString(work.resolve("results.json"),rows.toString());
   }
   sessions.get("b").close();agent.close();sdk.close();sdk=null;var path=work.resolve("results.json");Files.writeString(path,rows.toString());System.out.println("Workflow evidence: "+helper(fixture,work,"--verify",path.toString()));passed=true;
  }finally{if(sdk!=null)sdk.close();if(passed){try(var files=Files.walk(work)){for(var path:files.sorted(java.util.Comparator.reverseOrder()).toList())Files.deleteIfExists(path);}}else System.err.println("Workflow evidence retained: "+work);}
 }
}
