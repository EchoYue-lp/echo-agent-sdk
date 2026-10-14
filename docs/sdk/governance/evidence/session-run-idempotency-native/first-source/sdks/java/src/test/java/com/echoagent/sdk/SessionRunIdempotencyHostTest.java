package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.junit.jupiter.api.Test;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import java.util.function.BooleanSupplier;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Native public callbacks count effects; the framework/Host alone own publication. */
class SessionRunIdempotencyHostTest {
    private static final Path FIXTURE = Path.of("../shared/testing/session_run_idempotency_fixture.py").toAbsolutePath();
    private static <T> T await(CompletionStage<T> value) throws Exception { return value.toCompletableFuture().get(10, TimeUnit.SECONDS); }
    private static ObjectNode object() { return JsonSupport.MAPPER.createObjectNode(); }
    private static String helper(Path work, String... args) throws Exception {
        var command = new ArrayList<>(List.of("python3", FIXTURE.toString(), "--work", work.toString())); command.addAll(List.of(args));
        Path output = Files.createTempFile(work,"helper-",".out"), error = Files.createTempFile(work,"helper-",".err");
        var child = new ProcessBuilder(command).redirectOutput(output.toFile()).redirectError(error.toFile()).start();
        try { assertTrue(child.waitFor(5,TimeUnit.SECONDS)); assertEquals(0,child.exitValue(),Files.readString(error)); return Files.readString(output); }
        finally { if(child.isAlive()){child.destroyForcibly();assertTrue(child.waitFor(5,TimeUnit.SECONDS));} }
    }
    private static ObjectNode errorFact(EchoAgentException error) {
        var fact=object().put("code",error.code()).put("retryable",error.retryable()).put("operation",error.operation());
        fact.set("details",error.details()==null?JsonSupport.MAPPER.nullNode():error.details()); fact.set("handle",error.handle()==null?JsonSupport.MAPPER.nullNode():error.handle().toJson()); return fact;
    }
    private static void until(BooleanSupplier predicate) throws Exception {
        long end=System.nanoTime()+TimeUnit.SECONDS.toNanos(5);
        while(!predicate.getAsBoolean()){assertTrue(System.nanoTime()<end,"native provider barrier missing");Thread.sleep(5);}
    }
    private static int notices(Path work,JsonNode stream) throws Exception {
        int count=0;
        for(var line:Files.readAllLines(work.resolve("notifications.jsonl"))){
            var row=JsonSupport.MAPPER.readTree(line);
            if(List.of("_echo_agent/event","_echo_agent/gap").contains(row.path("method").asText())&&row.path("params").path("stream").path("id").equals(stream.path("id")))count++;
        }
        return count;
    }
    private static ObjectNode sessionPayload(WireHandle agent,String key) {var row=object().put("idempotency_id",key);row.set("agent",agent.toJson());return row;}
    private static ObjectNode runPayload(JsonNode session,String text,String key) {var row=object();row.set("session",session);row.set("input",object().put("kind","chat").put("text",text));if(key!=null)row.put("idempotency_id",key);return row;}
    private static final class NativeCalls {
        final EchoAgentClient sdk; final List<JsonNode> requests=new CopyOnWriteArrayList<>();
        NativeCalls(EchoAgentClient sdk){this.sdk=sdk;}
        CompletionStage<JsonNode> submit(String method,JsonNode params){
            var fact=object().put("method",method);fact.set("params",params.deepCopy());requests.add(fact);
            return sdk.request(method,params).whenComplete((value,error)->{if(error==null)fact.set("response",value.deepCopy());else fact.set("error",errorFact(EchoAgentException.from(error,method)));});
        }
        JsonNode rpc(String method,JsonNode params)throws Exception{return await(submit(method,params));}
        JsonNode fail(String method,JsonNode params)throws Exception{var failed=assertThrows(ExecutionException.class,()->rpc(method,params));return errorFact(assertInstanceOf(EchoAgentException.class,failed.getCause()));}
        JsonNode settled(JsonNode response)throws Exception{var params=object();params.set("run",response.path("run"));params.set("timeout",object().put("seconds","5").put("nanos",0));return rpc("_echo_agent/run/wait",params);}
    }

    @Test void sessionRunKeysRetainOnePublicationAndRetryPreEffectBusyAdmission() throws Exception {
        String host=System.getenv("ECHO_AGENT_SDK_HOST");if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")))assertNotNull(host,"required Host missing");assumeTrue(host!=null&&!host.isBlank(),"real Host required");
        String configured=System.getenv("ECHO_AGENT_SDK_IDEMPOTENCY_ROOT");Path root=configured==null?Path.of(System.getProperty("java.io.tmpdir")):Path.of(configured);Files.createDirectories(root);Path work=Files.createTempDirectory(root,"java-");helper(work);
        var spec=JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());var facts=object();var groups=facts.putObject("groups");var calls=new CopyOnWriteArrayList<JsonNode>();var finalized=new AtomicInteger();var held=new AtomicReference<CompletableFuture<Void>>();var sessions=new ArrayList<JsonNode>();
        try(var sdk=await(EchoAgentClient.spawn("python3",List.of(Path.of("../shared/testing/gap_ack_host_proxy.py").toAbsolutePath().toString(),host,work.resolve("host.json").toString()),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of("NO_PROXY","127.0.0.1,localhost","no_proxy","127.0.0.1,localhost")))) {
            var registration=await(sdk.registerLlmClient("idempotency-native",spec.path("descriptor"),(call,cancel)->{
                var fact=object().put("operation",call.path("invocation").path("operation").asText()).put("invocation_id",call.path("invocation_id").asText());fact.set("context",call.path("context").deepCopy());fact.set("input",call.path("invocation").path("input").deepCopy());calls.add(fact);
                var gate=held.get();CompletionStage<Void> pending=gate==null?CompletableFuture.completedFuture(null):gate;
                return pending.thenApply(ignored->{var result=object().put("operation","llm_chat");result.set("value",spec.path("response").deepCopy());var out=object().put("outcome","result");out.set("result",result);return (JsonNode)out;}).whenComplete((value,error)->finalized.incrementAndGet());
            }));
            var nativeCalls=new NativeCalls(sdk);
            try {
                var agent=await(sdk.createAgent());var sessionParams=sessionPayload(agent.wire(),spec.path("keys").path("session").asText());var first=nativeCalls.rpc("_echo_agent/session/create",sessionParams);sessions.add(first.path("session"));
                var one=nativeCalls.submit("_echo_agent/session/create",sessionParams);var two=nativeCalls.submit("_echo_agent/session/create",sessionParams);var duplicate=await(one);var concurrent=await(two);assertEquals(first,duplicate);assertEquals(first,concurrent);
                var changedParams=sessionParams.deepCopy();changedParams.set("working_dir",object().put("encoding","utf8").put("path",work.resolve("changed-cwd").toString()));var changed=nativeCalls.fail("_echo_agent/session/create",changedParams);
                var sessionGroup=groups.putObject("session");sessionGroup.set("first",first);sessionGroup.set("duplicate",duplicate);sessionGroup.set("concurrent",concurrent);sessionGroup.set("changed",changed);sessionGroup.put("creations",JsonSupport.MAPPER.readTree(work.resolve("state/host/session-index.json").toFile()).size());
                held.set(new CompletableFuture<>());var runParams=runPayload(first.path("session"),spec.path("prompt").asText(),spec.path("keys").path("run").asText());var original=nativeCalls.rpc("_echo_agent/run/start",runParams);until(()->calls.size()==1);int before=notices(work,original.path("stream"));
                var replay=nativeCalls.rpc("_echo_agent/run/start",runParams);assertEquals(original,replay);assertEquals(1,calls.size());assertEquals(before,notices(work,original.path("stream")));
                var changedRun=nativeCalls.fail("_echo_agent/run/start",runPayload(first.path("session"),spec.path("changed_prompt").asText(),spec.path("keys").path("run").asText()));held.getAndSet(null).complete(null);var settlement=nativeCalls.settled(original);
                var get=object();get.set("run",original.path("run"));nativeCalls.rpc("_echo_agent/run/get",get);int completedNotice=notices(work,original.path("stream"));var completed=nativeCalls.rpc("_echo_agent/run/start",runParams);
                new RunHandle(sdk,WireHandle.fromJson(original.path("run")),WireHandle.fromJson(original.path("stream"))).close();var afterClose=nativeCalls.rpc("_echo_agent/run/start",runParams);
                var runGroup=groups.putObject("run");runGroup.set("first",original);runGroup.set("concurrent",replay);runGroup.set("completed",completed);runGroup.set("after_local_close",afterClose);runGroup.set("changed",changedRun);runGroup.set("settlement",settlement);runGroup.put("provider_calls",calls.size()).put("notifications_before_retry",completedNotice).put("notifications_after_retry",notices(work,original.path("stream")));
                var otherAgent=await(sdk.createAgent());var otherSession=nativeCalls.rpc("_echo_agent/session/create",sessionPayload(otherAgent.wire(),spec.path("keys").path("session").asText()));sessions.add(otherSession.path("session"));int count=calls.size();var otherRun=nativeCalls.rpc("_echo_agent/run/start",runPayload(otherSession.path("session"),spec.path("prompt").asText(),spec.path("keys").path("run").asText()));nativeCalls.settled(otherRun);
                var owner=groups.putObject("owner").put("distinct_session",!first.path("session").equals(otherSession.path("session"))).put("distinct_run",!original.path("run").equals(otherRun.path("run"))).put("provider_calls",calls.size()-count);owner.set("session",otherSession);owner.set("run",otherRun);
                held.set(new CompletableFuture<>());int busyBase=calls.size();var busyOriginal=nativeCalls.rpc("_echo_agent/run/start",runPayload(otherSession.path("session"),spec.path("busy_prompt").asText(),null));until(()->calls.size()==busyBase+1);
                var keyed=runPayload(otherSession.path("session"),spec.path("retry_prompt").asText(),spec.path("keys").path("busy").asText());var rejection=nativeCalls.fail("_echo_agent/run/start",keyed);int rejectCount=calls.size()-busyBase;held.getAndSet(null).complete(null);nativeCalls.settled(busyOriginal);var retry=nativeCalls.rpc("_echo_agent/run/start",keyed);nativeCalls.settled(retry);
                var busy=groups.putObject("busy").put("provider_calls_while_rejected",rejectCount).put("provider_calls_after_retry",calls.size()-busyBase);busy.set("original",busyOriginal);busy.set("rejection",rejection);busy.set("retry",retry);
                var close=object();close.set("session",first.path("session"));nativeCalls.rpc("_echo_agent/session/close",close);var closed=groups.putObject("closed");closed.set("session",nativeCalls.fail("_echo_agent/session/create",sessionParams));closed.set("run",nativeCalls.fail("_echo_agent/run/start",runParams));
                facts.put("provider_calls",calls.size()).put("provider_finally",finalized.get()).put("session_index_count",JsonSupport.MAPPER.readTree(work.resolve("state/host/session-index.json").toFile()).size());facts.set("calls",JsonSupport.MAPPER.valueToTree(calls));facts.set("requests",JsonSupport.MAPPER.valueToTree(nativeCalls.requests));
                close=object();close.set("session",otherSession.path("session"));nativeCalls.rpc("_echo_agent/session/close",close);otherAgent.close();agent.close();registration.close();
                Files.writeString(work.resolve("observed.json"),facts.toPrettyString());System.out.println("idempotency evidence: "+work+" "+helper(work,"--verify"));
            } finally {var gate=held.getAndSet(null);if(gate!=null)gate.complete(null);for(var session:sessions){var params=object();params.set("session",session);try{await(sdk.request("_echo_agent/session/close",params));}catch(ExecutionException ignored){}}registration.close();}
        }
    }
}
