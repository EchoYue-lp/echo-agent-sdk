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
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.time.Duration;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.CompletionException;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

public class WorkflowCheckpointHostTest {
 private static String helper(Path fixture,Path work,String... arguments)throws Exception{
  var args=new ArrayList<String>(List.of("python3",fixture.toString(),"--work",work.toString()));args.addAll(List.of(arguments));
  var outputPath=Files.createTempFile(work,"workflow-helper-",".txt");
  var child=new ProcessBuilder(args).redirectErrorStream(true).redirectOutput(outputPath.toFile()).start();
  try{assertTrue(child.waitFor(10,TimeUnit.SECONDS),"Workflow helper timed out");var output=Files.readString(outputPath);assertEquals(0,child.exitValue(),output);return output;}
  finally{if(child.isAlive()){child.destroy();if(!child.waitFor(1,TimeUnit.SECONDS)){child.destroyForcibly();assertTrue(child.waitFor(3,TimeUnit.SECONDS));}}}
 }
 @Test public void routingCheckpointClaimsBranchOwnerAndShapeBoundary()throws Exception{
  var host=System.getenv("ECHO_AGENT_SDK_HOST");if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")))assertTrue(host!=null&&!host.isBlank(),"required Workflow Host missing");assumeTrue(host!=null&&!host.isBlank());
  var fixture=Path.of("../shared/testing/workflow_checkpoint_fixture.py").toAbsolutePath();var work=Files.createTempDirectory("sdk-java-workflow-");EchoAgentClient sdk=null;boolean passed=false;
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
  }finally{if(sdk!=null)sdk.close();if(passed&&System.getenv("ECHO_AGENT_SDK_RETAIN_EVIDENCE")==null){try(var files=Files.walk(work)){for(var path:files.sorted(java.util.Comparator.reverseOrder()).toList())Files.deleteIfExists(path);}}else System.err.println("Workflow evidence retained: "+work);}
 }
 private static <T>T protectedAwait(CompletionStage<T> value)throws Exception{return value.toCompletableFuture().get(20,TimeUnit.SECONDS);}
 private static JsonNode protectedHttp(HttpClient client,String url,String path,JsonNode value)throws Exception{
  var request=HttpRequest.newBuilder(URI.create(url+path)).timeout(Duration.ofSeconds(15));
  if(value==null)request.GET();else request.header("Content-Type","application/json").POST(HttpRequest.BodyPublishers.ofString(value.toString()));
  var reply=client.send(request.build(),HttpResponse.BodyHandlers.ofString());assertEquals(200,reply.statusCode(),reply.body());return JsonSupport.MAPPER.readTree(reply.body());
 }
 private static AgentComponentResult protectedStoreResult(JsonNode reply){
  return switch(reply.path("operation").textValue()){
   case "workflow_checkpoint_save"->new AgentComponentResult.WorkflowCheckpointSaved();
   case "workflow_checkpoint_save_if_generation"->new AgentComponentResult.WorkflowCheckpointSavedIfGeneration(reply.path("value").path("committed").booleanValue());
   case "workflow_checkpoint_load"->new AgentComponentResult.WorkflowCheckpointLoaded(reply.path("value").path("checkpoint").isNull()?null:reply.path("value").path("checkpoint"));
   case "workflow_checkpoint_claim"->new AgentComponentResult.WorkflowCheckpointClaimed(reply.path("value").path("checkpoint").isNull()?null:reply.path("value").path("checkpoint"));
   case "workflow_checkpoint_protect_claim"->new AgentComponentResult.WorkflowCheckpointClaimProtected();
   case "workflow_checkpoint_ack_claim"->new AgentComponentResult.WorkflowCheckpointClaimAcked();
   case "workflow_checkpoint_requeue_claim"->new AgentComponentResult.WorkflowCheckpointClaimRequeued();
   case "workflow_checkpoint_renew_claim"->new AgentComponentResult.WorkflowCheckpointClaimRenewed();
   default->throw new IllegalArgumentException("unimplemented fixture Store operation");
  };
 }
 private static List<Object> protectedArguments(JsonNode resolved){
  var arguments=new ArrayList<Object>();for(var argument:resolved)arguments.add(argument.isObject()&&argument.has("kind")&&java.util.Set.of("null","bool","string","i64","u64","f64","bytes","duration","timestamp","path","handle","list","map","record","variant","unknown").contains(argument.path("kind").asText())?argument:JsonSupport.MAPPER.convertValue(argument,Object.class));return arguments;
 }
 private static CompletionStage<JsonNode> protectedCall(EchoAgentClient sdk,SessionHandle session,JsonNode step,List<Object> arguments){
  return sdk.call(step.path("operation").textValue(),session.wire(),arguments).handle((value,failure)->{
   var row=JsonSupport.MAPPER.createObjectNode().put("id",step.path("id").textValue());
   if(failure==null){row.put("outcome","result").set("value",value);return row;}
   Throwable cause=failure;while((cause instanceof CompletionException||cause instanceof ExecutionException)&&cause.getCause()!=null)cause=cause.getCause();
   var error=assertInstanceOf(EchoAgentException.class,cause);
   row.put("outcome","error").put("code",error.code()).put("message",error.getMessage()).put("retryable",error.retryable()).put("operation",error.operation());
   row.set("details",error.details());row.set("handle",error.handle()==null?null:error.handle().toJson());return row;
  });
 }
 private static Object closeField(Object owner,String name)throws ReflectiveOperationException{
  var field=owner.getClass().getDeclaredField(name);field.setAccessible(true);return field.get(owner);
 }
 private static Map<String,Object> closeSnapshot(Object transport,Process process,long started,Map<String,ExtensionCancellation> addresses,String invocation,ExtensionCancellation originalAddress)throws ReflectiveOperationException{
  var snapshot=new java.util.LinkedHashMap<String,Object>();snapshot.put("elapsed_nanos",System.nanoTime()-started);
  snapshot.put("pid",process.pid());snapshot.put("process_alive",process.isAlive());if(!process.isAlive())snapshot.put("process_exit",process.exitValue());
  for(String name:List.of("ready","writeTail","exited","stderrDrained","closeStage")){
   Object value=closeField(transport,name);if(value instanceof CompletableFuture<?> future)snapshot.put(name,Map.of("done",future.isDone(),"exceptional",future.isCompletedExceptionally()));
  }
  for(String name:List.of("writing","nativeFailure","observedExit")){
   var value=((java.util.concurrent.atomic.AtomicReference<?>)closeField(transport,name)).get();
   snapshot.put(name,value instanceof CompletableFuture<?> future?Map.of("done",future.isDone(),"exceptional",future.isCompletedExceptionally()):String.valueOf(value));
  }
  snapshot.put("expectedClose",((java.util.concurrent.atomic.AtomicBoolean)closeField(transport,"expectedClose")).get());
  // These auxiliary flags are read without taking the transport monitor,
  // which disposeAfterExit may hold while waiting for a native I/O close.
  for(String name:List.of("cleaned","delegateClosed","stdoutClosed","stderrClosed"))snapshot.put(name,closeField(transport,name));
  snapshot.put("original_cancel_address_same",addresses.get(invocation)==originalAddress);snapshot.put("original_cancel_signalled",originalAddress.isCancelled());snapshot.put("process_alive_after_address_sample",process.isAlive());
  return snapshot;
 }
 private static void protectedCloseWithDiagnostics(EchoAgentClient sdk,Path work,JsonNode results)throws Exception{
  Object transport=closeField(sdk,"transport");Process process=(Process)closeField(transport,"process");long started=System.nanoTime();
  var binding=results.required("while-physical-held").required("value").required("physical").required("binding");String invocation=binding.required("invocation_id").asText();
  @SuppressWarnings("unchecked") var addresses=(Map<String,ExtensionCancellation>)closeField(sdk,"invocationCancellations");
  var originalAddress=addresses.get(invocation);assertNotNull(originalAddress,"unknown original unary callback must retain its cancellation address");assertFalse(originalAddress.isCancelled());
  assertFalse(results.required("after-external-join").required("value").required("physical").required("alive").asBoolean());
  var samples=java.util.Collections.synchronizedList(new ArrayList<Map<String,Object>>());var diagnostics=new java.util.LinkedHashMap<String,Object>();
  diagnostics.put("started_nanos",started);diagnostics.put("samples",samples);diagnostics.put("original_invocation_id",invocation);diagnostics.put("original_cancel_address_retained",true);samples.add(closeSnapshot(transport,process,started,addresses,invocation,originalAddress));
  process.onExit().whenComplete((child,error)->{
   try{Files.writeString(work.resolve("java-host-exit.json"),JsonSupport.MAPPER.writeValueAsString(Map.of("elapsed_nanos",System.nanoTime()-started,"pid",process.pid(),"exit",process.exitValue())));}
   catch(Exception failure){System.err.println("Host exit diagnostic failed: "+failure);}
  });
  var sampler=new Thread(()->{
   try{while(!Thread.currentThread().isInterrupted()){samples.add(closeSnapshot(transport,process,started,addresses,invocation,originalAddress));Thread.sleep(100);}}
   catch(InterruptedException stopped){Thread.currentThread().interrupt();}
   catch(ReflectiveOperationException failure){System.err.println("Host close diagnostic failed: "+failure);}
  },"protected-claim-close-observer");sampler.setDaemon(true);sampler.start();
  try{
   sdk.closeAsync().toCompletableFuture().join();
   // A completed local transport close makes no claim about the protected
   // Graph attempt. The Host may exit with a shutdown error before the watchdog.
   assertFalse(process.isAlive());assertNotEquals(0,process.exitValue(),"unknown callback cannot report successful Host shutdown");assertSame(process,closeField(transport,"process"));assertTrue((Boolean)closeField(transport,"cleaned"));assertTrue(addresses.isEmpty());
   diagnostics.put("outcome","local_transport_cleanup_returned");diagnostics.put("same_host_exit",process.exitValue());
  }
  catch(RuntimeException failure){
   diagnostics.put("outcome","error");diagnostics.put("error",failure.toString());
   var stacks=new java.util.LinkedHashMap<String,Object>();Thread.getAllStackTraces().forEach((thread,stack)->stacks.put(thread.getName()+"/"+thread.getId(),java.util.Arrays.stream(stack).map(Object::toString).toList()));diagnostics.put("jvm_stacks",stacks);
   Throwable cause=failure;while(cause instanceof CompletionException&&cause.getCause()!=null)cause=cause.getCause();var typed=assertInstanceOf(EchoAgentException.class,cause);
   assertEquals("timeout",typed.code());assertEquals("after_delay",typed.retryable());assertEquals("client/close",typed.operation());assertEquals("Host transport did not exit within the five-second close deadline",typed.getMessage());
   diagnostics.put("typed_error",Map.of("code",typed.code(),"retryable",typed.retryable(),"operation",typed.operation(),"message",typed.getMessage()));diagnostics.put("after_failed_close",closeSnapshot(transport,process,started,addresses,invocation,originalAddress));
   // The original callback has no accepted late unary result. This deadline
   // is an expected failed graceful close, never a successful Host receipt.
   synchronized(samples){assertTrue(samples.stream().anyMatch(sample->((Long)sample.get("elapsed_nanos"))>=TimeUnit.SECONDS.toNanos(4)&&Boolean.TRUE.equals(sample.get("process_alive"))&&Boolean.TRUE.equals(sample.get("expectedClose"))&&"null".equals(sample.get("writing"))&&"null".equals(sample.get("nativeFailure"))&&Map.of("done",true,"exceptional",false).equals(sample.get("writeTail"))&&Map.of("done",false,"exceptional",false).equals(sample.get("exited"))),"must observe original Host pending exit after native writes drained");}
   synchronized(samples){assertTrue(samples.stream().filter(sample->Boolean.TRUE.equals(sample.get("process_alive"))&&Boolean.TRUE.equals(sample.get("process_alive_after_address_sample"))).allMatch(sample->Boolean.TRUE.equals(sample.get("original_cancel_address_same"))),"original callback address cannot disappear before actual Host exit");}
   assertTrue(process.waitFor(5,TimeUnit.SECONDS),"original SDK cleanup must reap its same Host");assertNotEquals(0,process.exitValue());assertTrue(originalAddress.isCancelled());
   sdk.closeAsync().toCompletableFuture().get(5,TimeUnit.SECONDS);assertSame(process,closeField(transport,"process"));assertTrue((Boolean)closeField(transport,"cleaned"));assertTrue(addresses.isEmpty());
   diagnostics.put("outcome","expected_unknown_deadline_then_same_owner_cleaned");diagnostics.put("host_graceful_shutdown",false);diagnostics.put("same_host_exit",process.exitValue());diagnostics.put("same_owner_retry_closed",true);
  }finally{
   diagnostics.put("completed_elapsed_nanos",System.nanoTime()-started);sampler.interrupt();sampler.join(1000);samples.add(closeSnapshot(transport,process,started,addresses,invocation,originalAddress));
   long prematureAddressLoss;synchronized(samples){prematureAddressLoss=samples.stream().filter(sample->Boolean.TRUE.equals(sample.get("process_alive"))&&Boolean.TRUE.equals(sample.get("process_alive_after_address_sample"))&&!Boolean.TRUE.equals(sample.get("original_cancel_address_same"))).count();}
   diagnostics.put("live_address_violations",prematureAddressLoss);diagnostics.put("framework_receipt_proved",false);
   Files.writeString(work.resolve("java-close-diagnostic.json"),JsonSupport.MAPPER.writeValueAsString(diagnostics));
   assertEquals(0,prematureAddressLoss,"original callback address cannot disappear while its Host is alive");
  }
 }
 @Test public void protectedWorkflowClaimKeepsBoundStoreAndOriginalLlmOwner()throws Exception{
  var host=System.getenv("ECHO_AGENT_SDK_HOST");if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")))assertTrue(host!=null&&!host.isBlank());assumeTrue(host!=null&&!host.isBlank());
  var fixture=Path.of("../shared/testing/workflow_checkpoint_fixture.py").toAbsolutePath();
  var root=Path.of("../../target/workflow-protected-claims").toAbsolutePath().normalize();Files.createDirectories(root);var work=Files.createTempDirectory(root,"java-");
  var backend=new ProcessBuilder("python3",fixture.toString(),"--work",work.toString(),"--serve-protected").redirectError(work.resolve("backend-stderr.txt").toFile()).redirectOutput(work.resolve("backend-stdout.txt").toFile()).start();
  EchoAgentClient sdk=null;String url=null;var http=HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(5)).build();
  var flights=new HashMap<String,CompletionStage<JsonNode>>();var rows=JsonSupport.MAPPER.createArrayNode();var results=JsonSupport.MAPPER.createObjectNode();
  try{
   long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(10);while(!Files.exists(work.resolve("protected-ready.json"))){assertTrue(System.nanoTime()<deadline,"owned Workflow fixture not ready");Thread.sleep(10);}
   url=JsonSupport.MAPPER.readTree(Files.readString(work.resolve("protected-ready.json"))).path("url").textValue();final String base=url;
   sdk=protectedAwait(EchoAgentClient.spawn(host,List.of("--config",work.resolve("host.json").toString()),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of()));
   protectedAwait(sdk.registerLlmClient("protected-llm",LlmClientDescriptor.builder().modelName("protected-fixture").supportsStreaming(false).build(),(call,cancellation)->CompletableFuture.supplyAsync(()->{
    try{var reply=protectedHttp(http,base,"/llm",call.raw());if(reply.has("error")){LlmOutcome error=()->JsonSupport.MAPPER.createObjectNode().put("outcome","error").set("error",reply.path("error"));return error;}
     var response=reply.path("response");return LlmOutcome.result(LlmChatResponse.builder().message(response.path("message")).raw(response.path("raw")).finishReason(response.path("finish_reason").textValue()).build());
    }catch(Exception error){throw new CompletionException(error);}
   })));
   var agent=protectedAwait(sdk.createAgent());var session=protectedAwait(agent.createSession());
   var scenario=JsonSupport.MAPPER.readTree(Files.readString(work.resolve("protected-scenario.json")));
   for(var step:scenario.path("steps")){
    String action=step.path("action").asText(),id=step.path("id").textValue();com.fasterxml.jackson.databind.node.ObjectNode row;
    if(action.equals("bind_store")){
     String store=step.path("store").textValue();var descriptor=AgentComponentDescriptor.workflowCheckpointStore(store,java.math.BigInteger.valueOf(1000)).withProtectedClaimV1(step.path("supported").booleanValue());
     var registration=protectedAwait(sdk.registerAgentComponent("protected-"+store,descriptor,(call,cancellation)->CompletableFuture.supplyAsync(()->{
      try{var reply=protectedHttp(http,base,"/store/"+store,call.raw());if(reply.has("error"))return AgentComponentOutcome.error(reply.path("error").path("code").textValue(),reply.path("error").path("message").textValue(),reply.path("error").path("retryable").textValue());
       return AgentComponentOutcome.result("workflow_checkpoint_store",protectedStoreResult(reply));
      }catch(Exception error){throw new CompletionException(error);}
     })));row=JsonSupport.MAPPER.createObjectNode().put("id",id).put("outcome","registered");row.set("wire",registration.wire().toJson());
    }else if(java.util.Set.of("inspect","wait_protect","release_protect","release_join").contains(action)){
     String path=action.equals("inspect")?"/inspect":"/"+action;row=JsonSupport.MAPPER.createObjectNode().put("id",id).put("outcome","observed");row.set("value",protectedHttp(http,base,path,action.equals("inspect")?null:JsonSupport.MAPPER.createObjectNode()));
    }else if(action.equals("await_call")){row=(com.fasterxml.jackson.databind.node.ObjectNode)protectedAwait(flights.get(step.path("flight").textValue()));row.put("id",id);}
    else{
     var resolve=JsonSupport.MAPPER.createObjectNode();resolve.set("value",step.path("arguments"));resolve.set("results",results);Files.writeString(work.resolve("protected-resolve.json"),resolve.toString());
     var arguments=protectedArguments(JsonSupport.MAPPER.readTree(helper(fixture,work,"--resolve",work.resolve("protected-resolve.json").toString())));
     var call=protectedCall(sdk,session,step,arguments);
     if(action.equals("start_call")){flights.put(id,call);row=JsonSupport.MAPPER.createObjectNode().put("id",id).put("outcome","started");}
     else row=(com.fasterxml.jackson.databind.node.ObjectNode)protectedAwait(call);
    }
    rows.add(row);results.set(id,row);Files.writeString(work.resolve("protected-results.json"),rows.toString());
   }
   System.out.println(helper(fixture,work,"--verify-protected",work.resolve("protected-results.json").toString()));
  }finally{
   try{if(url!=null){protectedHttp(http,url,"/release_join",JsonSupport.MAPPER.createObjectNode());for(var flight:flights.values())protectedAwait(flight);}}
   finally{try{if(sdk!=null){
    protectedCloseWithDiagnostics(sdk,work,results);
    var afterClose=protectedHttp(http,url,"/inspect",null);var beforeClose=results.required("after-external-join").required("value");
    assertEquals(beforeClose.required("stores"),afterClose.required("stores"),"Host transport cleanup cannot settle or reclaim the protected attempt");assertEquals(beforeClose.required("llm_calls"),afterClose.required("llm_calls"),"Host cleanup cannot repeat external work");assertEquals(beforeClose.required("calls"),afterClose.required("calls"),"Host cleanup cannot invent Store settlement calls");
    Files.writeString(work.resolve("java-after-close-backend.json"),afterClose.toString());
   }}finally{
    try{if(url!=null)protectedHttp(http,url,"/shutdown",JsonSupport.MAPPER.createObjectNode());else backend.destroy();assertTrue(backend.waitFor(5,TimeUnit.SECONDS));assertEquals(0,backend.exitValue());}
    finally{if(backend.isAlive()){backend.destroyForcibly();assertTrue(backend.waitFor(5,TimeUnit.SECONDS));}System.out.println("Protected Workflow evidence: "+work);}
   }}
  }
 }

}
