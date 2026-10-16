package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.file.*;
import java.net.URI;
import java.net.http.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeAll;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Real-Host native SDK consumer; backend owns CAS/storage only, not scheduling. */
public class TaskSubagentHostTest {
 @BeforeAll public static void requireHost(){var host=System.getenv("ECHO_AGENT_SDK_HOST");if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")))assertTrue(host!=null&&!host.isBlank(),"required real Task/Subagent Host binary missing");assumeTrue(host!=null&&!host.isBlank(),"real Host binary required");}
 static final ObjectMapper JSON=new ObjectMapper();
 static final HttpClient HTTP=HttpClient.newBuilder().build();
 static final Path SDK=Path.of("..").toAbsolutePath();
 static JsonNode json(Object value){return JSON.valueToTree(value);}
 static void check(boolean value,String message){if(!value)throw new AssertionError(message);}
 static JsonNode await(CompletionStage<JsonNode> stage)throws Exception{return stage.toCompletableFuture().get(15,TimeUnit.SECONDS);}
 static JsonNode rpc(EchoAgentClient sdk,String method,Object params)throws Exception{return await(sdk.request("_echo_agent/"+method,json(params)));}
 static JsonNode http(String url,String path,Object body)throws Exception{var request=HttpRequest.newBuilder(URI.create(url+path));if(body!=null)request.header("Content-Type","application/json").POST(HttpRequest.BodyPublishers.ofString(JSON.writeValueAsString(body)));else request.GET();var response=HTTP.send(request.build(),HttpResponse.BodyHandlers.ofString());check(response.statusCode()==200,"fixture HTTP failed");return JSON.readTree(response.body());}
 @FunctionalInterface interface Condition{boolean test()throws Exception;}
 static void until(Condition condition)throws Exception{long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(15);while(!condition.test()){check(System.nanoTime()<deadline,"framework condition missing");Thread.sleep(10);}}
 static EchoAgentException failure(CompletionStage<JsonNode> stage)throws Exception{try{await(stage);throw new AssertionError("unexpected success");}catch(ExecutionException error){Throwable cause=error.getCause();while(cause instanceof CompletionException&&cause.getCause()!=null)cause=cause.getCause();check(cause instanceof EchoAgentException,"error not typed: "+cause);return(EchoAgentException)cause;}}
 static JsonNode outcome(String operation,Object value){return json(Map.of("outcome","result","result",new LinkedHashMap<String,Object>(){{put("operation",operation);put("value",value);}}));}
 static ExtensionRegistration store(EchoAgentClient sdk,String url,boolean execution)throws Exception{
  return sdk.registerExtension("agent_component","task-store",json(Map.of("kind","agent_component","descriptor_version",1,"component","revisioned_task_store","name","task-store","capabilities",Map.of("execution_cas_v1",execution))), (call,cancel)->CompletableFuture.supplyAsync(()->{try{var response=http(url,"/store",call.path("invocation").path("input").path("call"));return response.path("kind").asText().equals("error")?json(Map.of("outcome","error","error",response.get("error"))):outcome("agent_component_call",Map.of("component","revisioned_task_store","result",response.get("result")));}catch(Exception error){throw new CompletionException(error);}})).toCompletableFuture().get(5,TimeUnit.SECONDS);
 }
 static String status(JsonNode listed,WireHandle handle){for(var task:listed.path("tasks"))if(task.path("task").path("id").asText().equals(handle.id()))return task.path("status").path("status").asText();throw new AssertionError("task missing");}
 static void scenario(String action)throws Exception{
  Path work=Files.createTempDirectory("sdk-java-task-"+action+"-");System.out.println("Task evidence "+work);
  var backendArguments=new ArrayList<String>(List.of("python3",SDK.resolve("shared/testing/task_subagent_fixture.py").toString(),"--work",work.toString()));if(action.equals("observation"))backendArguments.add("--observation");
  Process backend=new ProcessBuilder(backendArguments).inheritIO().start();String[] fixtureUrl={null};var formalFacts=new ConcurrentHashMap<String,Object>();formalFacts.put("action",action);
  // Two held streams each own a producer and a blocking HTTP gate. Keep these
  // test jobs off the common pool needed by the canonical Task Store callbacks.
  var producers=Executors.newFixedThreadPool(4);
  try{until(()->Files.exists(work.resolve("ready")));String url=JSON.readTree(work.resolve("fixture.json").toFile()).path("url").asText();fixtureUrl[0]=url;
   try(var sdk=EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"),List.of("--config",work.resolve("host.json").toString()),SDK.resolve("shared/facade-operation-catalog.json"),SDK.resolve("shared/contract-digests.json"),Map.of(),List.of(),List.of()).toCompletableFuture().get(10,TimeUnit.SECONDS)){
    var registrations=new ArrayList<ExtensionRegistration>();registrations.add(store(sdk,url,true));var counts=new ConcurrentHashMap<String,AtomicInteger>();var prompts=new ConcurrentHashMap<String,String>();var jobs=new CopyOnWriteArrayList<CompletableFuture<Void>>();var canonicalTaskRunScope=new AtomicReference<String>();
    try{
    for(var lane:List.of("a","b","c","d")){counts.put(lane,new AtomicInteger());registrations.add(sdk.registerExtension("custom_agent","task-"+lane,json(Map.of("kind","custom_agent","descriptor_version",1,"name","task-"+lane,"model_name","fixture","system_prompt","fixture","tool_names",List.of())),(call,cancel)->{
     String operation=call.path("invocation").path("operation").asText();if(operation.equals("agent_close"))return CompletableFuture.completedFuture(outcome(operation,null));check(operation.equals("agent_execute_stream"),operation);check(call.path("context").path("run_id").isTextual()&&call.path("context").path("run_id").asText().equals(canonicalTaskRunScope.get()),"Task callback canonical run_id mismatch");int attempt=counts.get(lane).incrementAndGet();String task=call.path("invocation").path("input").path("task").asText();prompts.put(lane+"/"+attempt,task);
     try{http(url,"/agent",Map.of("lane",lane,"attempt",attempt,"task",task,"extension",call.get("extension")));}catch(Exception e){return CompletableFuture.failedFuture(e);}
     if(action.equals("observation")&&lane.equals("a")){
      try{
       var binding=http(url,"/physical/admit",call);check(binding.path("context").equals(call.path("context"))&&binding.path("factory_origin").isNull(),"original Task callback binding changed");
       var error=json(Map.of("code","framework_error","retryable","never","message","same Task attempt remains physically held 中文🙂","operation","_echo_agent/extension/invoke","details",Map.of("fields",List.of(Map.of("key","observation","value","not_settled"),Map.of("key","run_id","value",binding.path("run_id").asText()),Map.of("key","execution_id","value",binding.path("execution_id").asText())))));
       var writer=sdk.streamWriter(WireHandle.fromJson(call.get("stream")));
       var job=CompletableFuture.runAsync(()->{try{
        writer.failed(error).toCompletableFuture().get(5,TimeUnit.SECONDS);formalFacts.put("original",json(Map.of("call",call,"binding",binding,"error",error)));
        formalFacts.put("joined",http(url,"/physical/join",Map.of()));
        writer.complete(json(Map.of("kind","agent","value",Map.of("event","final_answer","data",Map.of("text","A-after-physical-close 中文🧪"))))).toCompletableFuture().get(5,TimeUnit.SECONDS);
       }catch(Exception failure){throw new CompletionException(failure);}},producers);jobs.add(job);
       return CompletableFuture.completedFuture(json(Map.of("outcome","stream","stream",call.get("stream"))));
      }catch(Exception failure){return CompletableFuture.failedFuture(failure);}
     }
     var stream=WireHandle.fromJson(call.get("stream"));jobs.add(CompletableFuture.runAsync(()->{try{if((lane.equals("a")||lane.equals("b"))&&attempt==1){var released=CompletableFuture.runAsync(()->{try{http(url,"/gate",Map.of("lane",lane,"attempt",attempt));}catch(Exception e){throw new CompletionException(e);}},producers);while(!released.isDone()&&!cancel.isCancelled())Thread.sleep(5);}JsonNode terminal=cancel.isCancelled()?json(Map.of("kind","agent","value",Map.of("event","cancelled"))):json(Map.of("kind","agent","value",Map.of("event","final_answer","data",Map.of("text",lane.toUpperCase()+"-canonical-result 中文🧪"))));sdk.streamWriter(stream).complete(terminal).toCompletableFuture().get(5,TimeUnit.SECONDS);}catch(Exception e){throw new CompletionException(e);}},producers));return CompletableFuture.completedFuture(json(Map.of("outcome","stream","stream",call.get("stream"))));
    }).toCompletableFuture().get(5,TimeUnit.SECONDS));}
    var agent=sdk.createAgent().toCompletableFuture().get(5,TimeUnit.SECONDS);var session=agent.createSession().toCompletableFuture().get(5,TimeUnit.SECONDS);var taskRun=session.taskRun().toJson();canonicalTaskRunScope.set(session.acpSessionId());var tasks=new ArrayList<Map<String,Object>>();tasks.add(Map.of("id","a","title","A","description","controlled A","extension",Map.of("subagent","task-a")));tasks.add(Map.of("id","b","title","B","description","independent B","extension",Map.of("subagent","task-b")));tasks.add(Map.of("id","c","title","C","description","after B","depends_on",List.of("b"),"extension",Map.of("subagent","task-c")));if(action.equals("pause")||action.equals("observation"))tasks.add(Map.of("id","d","title","D","description",action.equals("observation")?"after A and B":"after resume A and B","depends_on",List.of("a","b"),"extension",Map.of("subagent","task-d")));
    var created=rpc(sdk,"task/create",Map.of("task_run",taskRun,"spec",WireValues.value(Map.of("execution_mode","parallel","tasks",tasks))));var handles=new HashMap<String,WireHandle>();for(int i=0;i<tasks.size();i++)handles.put((String)tasks.get(i).get("id"),WireHandle.fromJson(created.path("tasks").get(i)));
    var renamed=rpc(sdk,"task/update",Map.of("task_run",taskRun,"patch",WireValues.value(Map.of("base_revision",Long.parseLong(created.path("revision").asText()),"reason","fixture rename","operations",List.of(Map.of("op","update","task_id","a","patch",Map.of("title","A canonical v2")))))));
    var before=http(url,"/inspect",null);var stale=failure(sdk.request("_echo_agent/task/update",json(Map.of("task_run",taskRun,"patch",WireValues.value(Map.of("base_revision",Long.parseLong(created.path("revision").asText()),"reason","stale writer","operations",List.of(Map.of("op","update","task_id","a","patch",Map.of("title","must not commit")))))))));check(stale.code().equals("framework_error"),stale.getMessage());check(http(url,"/inspect",null).equals(before),"stale update mutated");
    rpc(sdk,"task/execute",Map.of("task_run",taskRun));until(()->counts.get("a").get()==1&&counts.get("b").get()==1);var duplicate=failure(sdk.request("_echo_agent/task/execute",json(Map.of("task_run",taskRun))));check(duplicate.code().equals("invalid_value")&&duplicate.getMessage().contains("already executing"),"duplicate execute error");
    if(action.equals("observation")){
     until(()->formalFacts.containsKey("original"));http(url,"/release",Map.of("lane","b","attempt",1));final JsonNode[] snapshots={null};
     until(()->{snapshots[0]=rpc(sdk,"task/list",Map.of("task_run",taskRun));return status(snapshots[0],handles.get("b")).equals("completed");});
     formalFacts.put("held",http(url,"/inspect",null));formalFacts.put("held_list",snapshots[0]);var heldCounts=new TreeMap<String,Integer>();counts.forEach((key,value)->heldCounts.put(key,value.get()));formalFacts.put("held_counts",heldCounts);
     http(url,"/physical/release",Map.of());CompletableFuture.allOf(jobs.toArray(CompletableFuture[]::new)).get(10,TimeUnit.SECONDS);
     until(()->{snapshots[0]=rpc(sdk,"task/list",Map.of("task_run",taskRun));for(var handle:handles.values())if(!status(snapshots[0],handle).equals("completed"))return false;return true;});
     var finalCounts=new TreeMap<String,Integer>();counts.forEach((key,value)->finalCounts.put(key,value.get()));formalFacts.put("counts",finalCounts);formalFacts.put("prompts",new TreeMap<>(prompts));formalFacts.put("scope",canonicalTaskRunScope.get());formalFacts.put("revision",renamed.get("revision"));formalFacts.put("after",http(url,"/inspect",null));formalFacts.put("listed",snapshots[0]);formalFacts.put("listed_again",rpc(sdk,"task/list",Map.of("task_run",taskRun)));
     Files.writeString(work.resolve("observed.json"),JSON.writeValueAsString(formalFacts));
     var verifier=new ProcessBuilder("python3",SDK.resolve("shared/testing/task_subagent_fixture.py").toString(),"--work",work.toString(),"--verify-formal",work.resolve("observed.json").toString()).redirectErrorStream(true).redirectOutput(work.resolve("formal-verifier.log").toFile()).start();check(verifier.waitFor(5,TimeUnit.SECONDS),"formal verifier did not exit");check(verifier.exitValue()==0,Files.readString(work.resolve("formal-verifier.log")));System.out.println(Files.readString(work.resolve("formal-verifier.log")));
     session.close();for(var registration:registrations)registration.close();agent.close();return;
    }
    check(rpc(sdk,"task/control",Map.of("task_run",taskRun,"task",handles.get("a").toJson(),"action",action)).path("accepted").asBoolean(),"control not accepted");
    final JsonNode[] listed={null};until(()->{listed[0]=rpc(sdk,"task/list",Map.of("task_run",taskRun));return status(listed[0],handles.get("a")).equals(action.equals("pause")?"paused":"cancelled");});check(status(listed[0],handles.get("b")).equals("running")&&status(listed[0],handles.get("c")).equals("pending"),"control disturbed sibling");http(url,"/release",Map.of("lane","b","attempt",1));until(()->{listed[0]=rpc(sdk,"task/list",Map.of("task_run",taskRun));return status(listed[0],handles.get("b")).equals("completed")&&status(listed[0],handles.get("c")).equals("completed");});check(prompts.get("c/1").contains("Completed dependency b")&&prompts.get("c/1").contains("B-canonical-result"),"dependency result missing");
    if(action.equals("pause")){check(rpc(sdk,"task/control",Map.of("task_run",taskRun,"task",handles.get("a").toJson(),"action","resume")).path("accepted").asBoolean(),"resume rejected");until(()->{try{rpc(sdk,"task/execute",Map.of("task_run",taskRun));return true;}catch(ExecutionException error){var cause=(EchoAgentException)error.getCause();if(!cause.code().equals("invalid_value")||!cause.getMessage().contains("already executing"))throw error;return false;}});until(()->{listed[0]=rpc(sdk,"task/list",Map.of("task_run",taskRun));for(var h:handles.values())if(!status(listed[0],h).equals("completed"))return false;return true;});check(counts.get("a").get()==2&&counts.get("b").get()==1&&counts.get("c").get()==1&&counts.get("d").get()==1,"resume redispatch");check(prompts.get("d/1").contains("Completed dependency b")&&prompts.get("d/1").contains("B-canonical-result"),"persisted B lost");}else{check(counts.get("a").get()==1&&counts.get("b").get()==1&&counts.get("c").get()==1&&counts.get("d").get()==0,"cancel must preserve exact invocation counts A1/B1/C1/D0");}
    var persisted=http(url,"/inspect",null);var graph=persisted.path("graphs").elements().next();check(graph.path("snapshot").path("revision").equals(renamed.get("revision")),"relation revision drift");JsonNode bCompleted=null;for(var commit:persisted.path("commits"))if(commit.path("operation").asText().equals("revisioned_task_compare_and_commit_execution"))for(var task:commit.path("after").path("snapshot").path("tasks")){var execution=task.path("execution");if(execution.path("status").path("status").asText().equals("running")){check(execution.path("claim").path("revision").equals(renamed.get("revision")),"claim scope revision wrong");}if(task.path("spec").path("id").asText().equals("b")&&execution.path("status").path("status").asText().equals("completed")){if(bCompleted==null)bCompleted=execution.deepCopy();else check(bCompleted.equals(execution),"B result rewritten on resume");}}check(bCompleted!=null&&java.util.stream.StreamSupport.stream(bCompleted.path("result").path("value").spliterator(),false).anyMatch(entry->entry.path("key").path("value").asText().equals("output")&&entry.path("value").path("kind").asText().equals("string")&&entry.path("value").path("value").asText().equals("B-canonical-result 中文🧪")),"canonical B result absent");
    var normalized=new TreeMap<String,Integer>();counts.forEach((k,v)->normalized.put(k,v.get()));var output=json(Map.of("action",action,"counts",normalized,"graph",graph,"persisted",persisted,"statuses",listed[0].path("tasks")));Files.writeString(work.resolve("observed.json"),output.toString());http(url,"/release",Map.of("lane","a","attempt",1));session.close();for(var reg:registrations)reg.close();agent.close();CompletableFuture.allOf(jobs.toArray(CompletableFuture[]::new)).get(5,TimeUnit.SECONDS);
    }catch(Exception|AssertionError failure){if(action.equals("observation"))formalFacts.put("failure",Map.of("name",failure.getClass().getName(),"message",String.valueOf(failure.getMessage())));throw failure;}
    finally{if(action.equals("observation")){
     try{http(url,"/release",Map.of("lane","b","attempt",1));http(url,"/physical/release",Map.of());formalFacts.put("cleanup_join",http(url,"/physical/join",Map.of()));CompletableFuture.allOf(jobs.toArray(CompletableFuture[]::new)).get(10,TimeUnit.SECONDS);}
     finally{Files.writeString(work.resolve("observed.json"),JSON.writeValueAsString(formalFacts));}
    }}
   }
  }finally{
   try{if(action.equals("observation")&&fixtureUrl[0]!=null){http(fixtureUrl[0],"/shutdown",Map.of());check(backend.waitFor(5,TimeUnit.SECONDS)&&backend.exitValue()==0,"original fixture backend did not exit normally");formalFacts.put("backend_exit",backend.exitValue());}else{backend.destroy();backend.waitFor(3,TimeUnit.SECONDS);}}
   finally{
    try{if(action.equals("observation")&&backend.isAlive()){backend.destroyForcibly();check(backend.waitFor(5,TimeUnit.SECONDS),"owned backend survived failure cleanup");formalFacts.put("forced_backend_cleanup",true);}}
    finally{producers.shutdownNow();check(producers.awaitTermination(5,TimeUnit.SECONDS),"fixture producer threads did not stop");if(action.equals("observation"))Files.writeString(work.resolve("observed.json"),JSON.writeValueAsString(formalFacts));}
   }
  }
 }
 static void single(String kind)throws Exception{
  Path work=Files.createTempDirectory("sdk-java-task-"+kind+"-");System.out.println("Task evidence "+work);Process backend=new ProcessBuilder("python3",SDK.resolve("shared/testing/task_subagent_fixture.py").toString(),"--work",work.toString()).inheritIO().start();
  try{until(()->Files.exists(work.resolve("ready")));String url=JSON.readTree(work.resolve("fixture.json").toFile()).path("url").asText();try(var sdk=EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"),List.of("--config",work.resolve("host.json").toString()),SDK.resolve("shared/facade-operation-catalog.json"),SDK.resolve("shared/contract-digests.json"),Map.of(),List.of(),List.of()).toCompletableFuture().get(10,TimeUnit.SECONDS)){
   var registrations=new ArrayList<ExtensionRegistration>();if(kind.equals("crud-only"))registrations.add(store(sdk,url,false));AtomicInteger invocations=new AtomicInteger();AtomicReference<JsonNode> raw=new AtomicReference<>();var jobs=new CopyOnWriteArrayList<CompletableFuture<Void>>();
   registrations.add(sdk.registerExtension("custom_agent","await-target",json(Map.of("kind","custom_agent","descriptor_version",1,"name","await-target","model_name","fixture","system_prompt","fixture","tool_names",List.of())),(call,cancel)->{
    String operation=call.path("invocation").path("operation").asText();if(operation.equals("agent_close"))return CompletableFuture.completedFuture(outcome(operation,null));check(operation.equals("agent_execute_stream")||operation.equals("agent_execute"),operation);invocations.incrementAndGet();raw.set(call);
    try{http(url,"/agent",Map.of("lane","await","attempt",1,"task",call.path("invocation").path("input").path("task").asText(),"extension",call.get("extension")));}catch(Exception error){return CompletableFuture.failedFuture(error);}
    if(operation.endsWith("_stream")){jobs.add(CompletableFuture.runAsync(()->{try{http(url,"/gate",Map.of("lane","await","attempt",1));sdk.streamWriter(WireHandle.fromJson(call.get("stream"))).complete(json(Map.of("kind","agent","value",Map.of("event","final_answer","data",Map.of("text","full retained result 中文🧪"))))).toCompletableFuture().get(5,TimeUnit.SECONDS);}catch(Exception error){throw new CompletionException(error);}}));return CompletableFuture.completedFuture(json(Map.of("outcome","stream","stream",call.get("stream"))));}
    return CompletableFuture.supplyAsync(()->{try{http(url,"/gate",Map.of("lane","await","attempt",1));return outcome(operation,"full retained result 中文🧪");}catch(Exception error){throw new CompletionException(error);}});
   }).toCompletableFuture().get(5,TimeUnit.SECONDS));var agent=sdk.createAgent().toCompletableFuture().get(5,TimeUnit.SECONDS);var session=agent.createSession().toCompletableFuture().get(5,TimeUnit.SECONDS);var taskRun=session.taskRun().toJson();JsonNode observed;
   if(kind.equals("crud-only")){rpc(sdk,"task/create",Map.of("task_run",taskRun,"spec",WireValues.value(Map.of("tasks",List.of(Map.of("id","pending","title","pending","description","no user call","extension",Map.of("subagent","await-target")))))));var before=http(url,"/inspect",null);var error=failure(sdk.request("_echo_agent/task/execute",json(Map.of("task_run",taskRun))));check(error.code().equals("feature_unavailable")&&error.handle().equals(session.taskRun()),"CRUD preflight error mismatch");check(http(url,"/inspect",null).equals(before)&&invocations.get()==0,"CRUD executed side effect");for(var task:rpc(sdk,"task/list",Map.of("task_run",taskRun)).path("tasks"))check(task.path("status").path("status").asText().equals("pending"),"CRUD changed state");observed=json(Map.of("kind",kind,"invocations",0,"execution_commits",0,"error","feature_unavailable"));}
   else{var payload=json(Map.of("session",session.wire().toJson(),"request",WireValues.value(Map.of("agent_name","await-target","task","await retained 中文🧪","mode_override","sync","constraints",List.of("constraint retained 🧪"),"prompt_context",Map.of("acceptance_criteria",List.of("field preserved")))),"idempotency_id","one-receipt"));var oneStage=sdk.request("_echo_agent/subagent/dispatch",payload);var twoStage=sdk.request("_echo_agent/subagent/dispatch",payload);var one=await(oneStage);var two=await(twoStage);check(one.get("subagent").equals(two.get("subagent")),"receipt duplicate attempt");until(()->invocations.get()==1);var shortWait=rpc(sdk,"subagent/await",Map.of("subagent",one.get("subagent"),"timeout",Map.of("seconds","0","nanos",5000000)));check(!shortWait.path("settled").asBoolean()&&!shortWait.has("result"),"short await consumed result");var conflict=payload.deepCopy();((ObjectNode)conflict).set("request",WireValues.value(Map.of("agent_name","await-target","task","conflicting payload")));check(failure(sdk.request("_echo_agent/subagent/dispatch",conflict)).code().equals("invalid_value"),"receipt conflict missing");http(url,"/release",Map.of("lane","await","attempt",1));var full=rpc(sdk,"subagent/await",Map.of("subagent",one.get("subagent")));check(full.equals(rpc(sdk,"subagent/await",Map.of("subagent",one.get("subagent"))))&&full.path("settled").asBoolean(),"repeated await changed");boolean output=false;for(var field:full.path("result").path("value"))if(field.path("key").path("value").asText().equals("output"))output=field.path("value").path("value").asText().equals("full retained result 中文🧪");check(output,"result field missing");check(raw.get().path("invocation").path("input").path("task").asText().contains("constraint retained 🧪"),"constraints lost");check(!rpc(sdk,"subagent/control",Map.of("subagent",one.get("subagent"),"action","guidance","payload","late")).path("accepted").asBoolean()&&invocations.get()==1,"late dispatch misuse");observed=json(Map.of("kind",kind,"invocations",1,"short",shortWait,"full",full,"rawCall",raw.get()));}
   Files.writeString(work.resolve("observed.json"),observed.toString());session.close();for(var registration:registrations)registration.close();agent.close();CompletableFuture.allOf(jobs.toArray(CompletableFuture[]::new)).get(5,TimeUnit.SECONDS);
  }}finally{backend.destroy();backend.waitFor(3,TimeUnit.SECONDS);}
 }
 @Test public void taskPausePreservesIndependentSiblingAndCanonicalResult()throws Exception{scenario("pause");}
 @Test public void taskCancelPreservesIndependentSiblingAndCanonicalResult()throws Exception{scenario("cancel");}
 @Test public void matchingTaskObservationSettlesOnlySameRealPhysicalAttempt()throws Exception{scenario("observation");}
 @Test public void subagentAwaitRetainsResultAndIdempotentReceipt()throws Exception{single("subagent-await");}
 @Test public void crudOnlyTaskStoreRejectsExecutionWithoutSideEffects()throws Exception{single("crud-only");}

 static JsonNode observationHttp(String url,String path,Object payload)throws Exception {
  var request=HttpRequest.newBuilder(URI.create(url+path)).timeout(java.time.Duration.ofSeconds(35));
  if(payload==null)request.GET();else request.header("Content-Type","application/json").POST(HttpRequest.BodyPublishers.ofString(JSON.writeValueAsString(payload)));
  var response=HTTP.send(request.build(),HttpResponse.BodyHandlers.ofString());check(response.statusCode()==200,"physical fixture HTTP failed");return JSON.readTree(response.body());
 }
 static void observationScenario(String carrier)throws Exception {
  Path root=SDK.normalize().getParent().resolve("target/cancellation-observation");Files.createDirectories(root);
  Path work=Files.createTempDirectory(root,"java-"+carrier+"-");System.out.println("B observation evidence "+work);
  Process backend=new ProcessBuilder("python3",SDK.resolve("shared/testing/task_subagent_fixture.py").toString(),"--work",work.toString(),"--observation").inheritIO().start();
  var producers=Executors.newFixedThreadPool(2);EchoAgentClient sdk=null;String url=null;Process originalHost=null;Object originalTransport=null;
  var facts=new ConcurrentHashMap<String,Object>();facts.put("carrier",carrier);facts.put("source_kind",carrier.equals("stream")?"issued_stream_with_late_terminal":"issued_stream_without_successor_terminal");facts.put("actual_complete_sent",false);
  var callbacks=new CopyOnWriteArrayList<JsonNode>();var closes=new CopyOnWriteArrayList<JsonNode>();
  facts.put("callbacks",callbacks);facts.put("closes",closes);
  var jobs=new CopyOnWriteArrayList<CompletableFuture<Void>>();
  var observed=new CompletableFuture<Void>();var factory=new AtomicReference<WireHandle>();
  try {
   until(()->Files.exists(work.resolve("ready")));url=JSON.readTree(work.resolve("fixture.json").toFile()).path("url").asText();final String endpoint=url;
   sdk=EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"),List.of("--config",work.resolve("host.json").toString()),SDK.resolve("shared/facade-operation-catalog.json"),SDK.resolve("shared/contract-digests.json"),Map.of(),List.of(),List.of()).toCompletableFuture().get(10,TimeUnit.SECONDS);
   var transportField=EchoAgentClient.class.getDeclaredField("transport");transportField.setAccessible(true);originalTransport=transportField.get(sdk);
   var processField=originalTransport.getClass().getDeclaredField("process");processField.setAccessible(true);originalHost=(Process)processField.get(originalTransport);facts.put("original_host_pid",originalHost.pid());
   final var client=sdk;
   var registration=sdk.registerAgentFactory("observation-factory",json(Map.of("kind","agent_factory","descriptor_version",1)),(call,cancel)->{
    try {
     String operation=call.path("invocation").path("operation").asText();callbacks.add(call.deepCopy());observationHttp(endpoint,"/agent",call);
     if(operation.equals("factory_create_agent"))return CompletableFuture.completedFuture(outcome(operation,Map.of("name","original-physical","model_name","fixture","system_prompt","","tool_names",List.of())));
     check(call.get("factory_origin").equals(factory.get().toJson()),"original factory origin changed");
     if(operation.equals("agent_close")) {
      var physical=observationHttp(endpoint,"/inspect",null).path("physical");check(call.get("extension").equals(physical.path("binding").path("child")),"cleanup targeted another child");check(physical.path("stopped").asBoolean()&&physical.path("exit_code").isInt()&&physical.path("exit_code").intValue()==0,"child close preceded actual physical join");closes.add(call.deepCopy());return CompletableFuture.completedFuture(outcome(operation,null));
     }
     check(operation.equals("agent_execute_stream"),"unexpected reverse operation: "+operation);
     var binding=observationHttp(endpoint,"/physical/admit",call);facts.put("binding",binding);
     check(binding.path("context").equals(call.path("context"))&&!call.path("context").hasNonNull("run_id"),"ordinary turn context was rewritten");check(binding.path("execution_id").equals(call.path("context").path("execution_id"))&&binding.path("invocation_id").equals(call.path("invocation_id")),"original execution/invocation identity changed");
     var error=json(Map.of("code","framework_error","retryable","never","message","original physical cleanup remains held 中文🙂","operation","_echo_agent/extension/invoke","details",Map.of("fields",List.of(Map.of("key","observation","value","not_settled"),Map.of("key","run_id","value",binding.path("run_id").asText()),Map.of("key","execution_id","value",binding.path("execution_id").asText())))));
     facts.put("error",error);var writer=client.streamWriter(WireHandle.fromJson(call.get("stream")));
     var job=CompletableFuture.runAsync(()->{
      try {
       if(carrier.equals("stream")){writer.failed(error).toCompletableFuture().get(5,TimeUnit.SECONDS);observed.complete(null);}
       var joined=observationHttp(endpoint,"/physical/join",Map.of());check(joined.equals(json(Map.of("admitted",true,"exit_code",0,"stopped",binding))),"original physical join mismatch");facts.put("joined",joined);
       if(carrier.equals("stream")){writer.complete(json(Map.of("kind","agent","value",Map.of("event","final_answer","data",Map.of("text","same physical producer actually exited"))))).toCompletableFuture().get(5,TimeUnit.SECONDS);facts.put("actual_complete_sent",true);}
      }catch(Exception failure){observed.completeExceptionally(failure);throw new CompletionException(failure);}
     },producers);jobs.add(job);
     if(carrier.equals("setup")){observed.complete(null);return CompletableFuture.completedFuture(json(Map.of("outcome","error","error",error)));}
     return CompletableFuture.completedFuture(json(Map.of("outcome","stream","stream",call.get("stream"))));
    }catch(Exception failure){observed.completeExceptionally(failure);return CompletableFuture.failedFuture(failure);}
   }).toCompletableFuture().get(5,TimeUnit.SECONDS);factory.set(registration.wire());
   var agent=sdk.createAgent().toCompletableFuture().get(5,TimeUnit.SECONDS);var session=agent.createSession().toCompletableFuture().get(5,TimeUnit.SECONDS);
   var run=session.startRun("delegate original physical work",true).toCompletableFuture().get(5,TimeUnit.SECONDS);
   observed.get(15,TimeUnit.SECONDS);var waited=await(run.waitForSettlement(java.time.Duration.ofMillis(250)));var state=await(run.get());var replay=await(run.replay("0",512));var held=observationHttp(url,"/inspect",null);
   facts.put("run",run.wire().toJson());facts.put("waited",waited);facts.put("state",state);facts.put("replay",replay);facts.put("held",held);
   check(waited.equals(json(Map.of("settled",false))),"observation manufactured a settled Run");check(state.path("status").asText().equals("running")&&!state.has("terminal")&&!state.has("receipt"),"Run observation became a business terminal");
   for(var row:replay.path("events"))check(!Set.of("final_answer","cancelled","error").contains(row.required("payload").required("event_type").asText()),"journal fabricated terminal");
   check(held.path("model_requests").size()==1&&closes.isEmpty(),"Agent continued or closed during original physical work");check(held.path("physical").path("alive").asBoolean()&&!held.path("physical").path("stopped").asBoolean(),"callback cancellation stopped original producer");check(held.path("physical").path("binding").equals(json(facts.get("binding"))),"original binding changed");
   check(callbacks.stream().filter(c->c.path("invocation").path("operation").asText().equals("factory_create_agent")).count()==1&&callbacks.stream().filter(c->c.path("invocation").path("operation").asText().equals("agent_execute_stream")).count()==1,"duplicate constructor/execute");
   var originalBinding=json(facts.get("binding"));var created=callbacks.stream().filter(c->c.path("invocation").path("operation").asText().equals("factory_create_agent")).findFirst().orElseThrow();var executed=callbacks.stream().filter(c->c.path("invocation").path("operation").asText().equals("agent_execute_stream")).findFirst().orElseThrow();check(originalBinding.path("child").equals(created.path("invocation").path("input").path("instance"))&&originalBinding.path("stream").equals(executed.path("stream")),"physical work not owned by the original issued child/stream");
   try {session.startRun("must not admit another execution",true).toCompletableFuture().get(5,TimeUnit.SECONDS);throw new AssertionError("unknown Run lost its admission lease");}
   catch(ExecutionException rejected){check(rejected.getCause() instanceof EchoAgentException,"admission refusal not typed");var error=(EchoAgentException)rejected.getCause();check(error.code().equals("invalid_request")&&error.retryable().equals("after_delay")&&error.operation().equals("_echo_agent/run/start"),"original admission refusal changed");var refusal=JSON.createObjectNode().put("code",error.code()).put("message",error.getMessage()).put("retryable",error.retryable()).put("operation",error.operation());refusal.set("details",error.details());refusal.set("handle",error.handle()==null?JSON.nullNode():error.handle().toJson());facts.put("held_admission_error",refusal);}
   observationHttp(url,"/physical/release",Map.of());CompletableFuture.allOf(jobs.toArray(CompletableFuture[]::new)).get(10,TimeUnit.SECONDS);facts.put("after",observationHttp(url,"/inspect",null));
   if(carrier.equals("stream"))until(()->closes.size()==1);
   check(!originalBinding.path("run_id").asText().equals(run.wire().id()),"foreign work borrowed Host Run identity");
   var first=run.waitForSettlement(java.time.Duration.ofMillis(250));var second=run.waitForSettlement(java.time.Duration.ofMillis(250));var one=await(first);var two=await(second);check(one.equals(two)&&one.equals(json(Map.of("settled",false))),"foreign completion manufactured a Host receipt");facts.put("after_wait",one);
   var afterRun=await(run.get());facts.put("after_run",afterRun);check(afterRun.path("status").asText().equals("running")&&!afterRun.has("terminal")&&!afterRun.has("receipt"),"foreign completion settled another Run");
   var afterReplay=await(run.replay("0",512));facts.put("after_replay",afterReplay);for(var row:afterReplay.path("events"))check(!Set.of("final_answer","cancelled","error").contains(row.required("payload").required("event_type").asText()),"foreign completion fabricated journal terminal");
   var after=observationHttp(url,"/inspect",null);facts.put("after",after);check(Boolean.valueOf(carrier.equals("stream")).equals(facts.get("actual_complete_sent"))&&closes.size()==(carrier.equals("stream")?1:0)&&after.path("model_requests").size()==1&&callbacks.stream().filter(c->c.path("invocation").path("operation").asText().equals("agent_execute_stream")).count()==1,"foreign completion duplicated work or lost original cleanup");

   facts.put("body_assertions_completed",true);
  }catch(Exception|AssertionError failure){facts.put("body_failure",Map.of("type",failure.getClass().getName(),"message",String.valueOf(failure.getMessage())));throw failure;}
  finally {
   try {if(url!=null){observationHttp(url,"/physical/release",Map.of());facts.put("cleanup_join",observationHttp(url,"/physical/join",Map.of()));CompletableFuture.allOf(jobs.toArray(CompletableFuture[]::new)).get(10,TimeUnit.SECONDS);}}
   finally {
    try {
     if(sdk!=null){
      check(originalHost!=null&&originalHost.isAlive(),"original unknown Run Host exited before close observation");
      var beforeClose=observationHttp(url,"/inspect",null);facts.put("backend_before_close",beforeClose);
      facts.put("original_host_alive_before_close",true);facts.put("close_started_nanos",System.nanoTime());
      try {
       sdk.closeAsync().toCompletableFuture().get(10,TimeUnit.SECONDS);facts.put("local_close_returned",true);
       check(!originalHost.isAlive()&&originalHost.exitValue()!=0,"unknown Run cannot report a graceful Host exit");
       var nativeFailure=originalTransport.getClass().getDeclaredField("nativeFailure");nativeFailure.setAccessible(true);
       check(((java.util.concurrent.atomic.AtomicReference<?>)nativeFailure.get(originalTransport)).get()==null,"SDK close cannot report success after its watchdog or native failure");
       facts.put("successful_close_without_watchdog_failure",true);
      } catch(ExecutionException failure) {
       Throwable cause=failure;while((cause instanceof ExecutionException||cause instanceof CompletionException)&&cause.getCause()!=null)cause=cause.getCause();
       check(cause instanceof EchoAgentException,"unknown close failure must be typed");var typed=(EchoAgentException)cause;
       check("timeout".equals(typed.code())&&"after_delay".equals(typed.retryable())&&"client/close".equals(typed.operation())&&"Host transport did not exit within the five-second close deadline".equals(typed.getMessage()),"unexpected unknown Run close failure");
       facts.put("typed_close_failure",Map.of("code",typed.code(),"retryable",typed.retryable(),"operation",typed.operation(),"message",typed.getMessage()));
       // Observe the existing SDK watchdog reaping its own original child;
       // this wait sends no signal and grants no new graceful-close deadline.
       check(originalHost.waitFor(5,TimeUnit.SECONDS)&&originalHost.exitValue()!=0,"original SDK close must reap its same Host without claiming graceful success");
       sdk.closeAsync().toCompletableFuture().get(5,TimeUnit.SECONDS);facts.put("same_owner_retry_closed",true);
      }
      var retained=EchoAgentClient.class.getDeclaredField("transport");retained.setAccessible(true);check(retained.get(sdk)==originalTransport,"close retry replaced original transport owner");
      var processField=originalTransport.getClass().getDeclaredField("process");processField.setAccessible(true);check(processField.get(originalTransport)==originalHost,"close retry replaced original Host process");
      var cleaned=originalTransport.getClass().getDeclaredField("cleaned");cleaned.setAccessible(true);check(cleaned.getBoolean(originalTransport),"same exited Host local I/O cleanup remains incomplete");
      var afterClose=observationHttp(url,"/inspect",null);facts.put("backend_after_close",afterClose);
      check(beforeClose.size()==afterClose.size(),"transport close added or removed backend evidence");
      var beforeFields=beforeClose.fields();while(beforeFields.hasNext()){var field=beforeFields.next();if(!field.getKey().equals("agent_calls"))check(field.getValue().equals(afterClose.required(field.getKey())),"transport close changed backend field: "+field.getKey());}
      if(carrier.equals("stream"))check(beforeClose.equals(afterClose),"stream transport close repeated completed child work");
      var beforeCalls=beforeClose.required("agent_calls");var afterCalls=afterClose.required("agent_calls");check(beforeCalls.isArray()&&afterCalls.isArray()&&afterCalls.size()>=beforeCalls.size(),"close rewrote original callback evidence");
      for(int index=0;index<beforeCalls.size();index++)check(beforeCalls.get(index).equals(afterCalls.get(index)),"close rewrote a prior callback");
      var originalBinding=json(facts.get("binding"));var constructor=callbacks.stream().filter(call->"factory_create_agent".equals(call.path("invocation").path("operation").asText())).findFirst().orElseThrow();String cleanupInvocation=constructor.required("invocation_id").asText()+"/cleanup";
      check(afterCalls.size()==beforeCalls.size()+(carrier.equals("setup")?1:0),"close added unexpected callback work");
      for(var call:closes)check("agent_close".equals(call.required("invocation").required("operation").asText())&&(!carrier.equals("setup")||call.required("invocation_id").asText().equals(cleanupInvocation))&&call.required("extension").equals(originalBinding.required("child"))&&call.required("factory_origin").equals(originalBinding.required("factory_origin")),"cleanup lost its original constructor/child/factory identity");
      check(closes.size()==1&&afterCalls.size()==3,"original child user cleanup must complete exactly once");
      for(int index=beforeCalls.size();index<afterCalls.size();index++)check(afterCalls.get(index).equals(closes.get(0)),"close appended work other than the one validated original child cleanup");
      facts.put("close_finished_nanos",System.nanoTime());facts.put("original_host_alive_after_close",originalHost.isAlive());facts.put("original_host_exit",originalHost.exitValue());facts.put("host_graceful_shutdown",false);facts.put("framework_receipt_proved",false);facts.put("same_transport_owner",true);
     }
    }
    finally {
     try {
      if(url!=null)observationHttp(url,"/shutdown",Map.of());else backend.destroy();
      check(backend.waitFor(5,TimeUnit.SECONDS),"owned fixture backend did not exit");check(backend.exitValue()==0,"owned fixture backend failed");facts.put("backend_exit",backend.exitValue());
     }finally {
      try {if(backend.isAlive()){backend.destroyForcibly();check(backend.waitFor(5,TimeUnit.SECONDS),"owned fixture backend survived forced cleanup");facts.put("forced_backend_cleanup",true);}}
      finally {producers.shutdownNow();check(producers.awaitTermination(5,TimeUnit.SECONDS),"owned fixture threads did not stop");Files.writeString(work.resolve("observed.json"),JSON.writeValueAsString(facts));}
     }
    }
   }
  }
 }
 @Test public void setupObservationPreservesRealRunAndSamePhysicalProducer()throws Exception{observationScenario("setup");}
 @Test public void streamObservationPreservesRealRunAndSamePhysicalProducer()throws Exception{observationScenario("stream");}

}
