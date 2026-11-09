package com.echoagent.sdk;

import static org.junit.jupiter.api.Assertions.*;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.Flow;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

/** 真实Java客户端→原stdio Host→实际provider HTTP；没有本地Agent替身。 */
class ConsumerProviderHttpTest {
    private static <T>T await(CompletionStage<T> value)throws Exception {
        return value.toCompletableFuture().get(25,TimeUnit.SECONDS);
    }
    private static JsonNode field(JsonNode value,String name) {
        for(var entry:value.path("value")) if(name.equals(entry.path("key").path("value").asText())) return entry.path("value");
        return JsonSupport.MAPPER.nullNode();
    }
    private static void barrier(Path file,Process process)throws Exception {
        long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(10);
        while(!Files.exists(file)) {
            assertTrue(process.isAlive(),"fixture exited before barrier");
            assertTrue(System.nanoTime()<deadline,"fixture barrier timed out");
            Thread.sleep(10);
        }
    }
    @Test void actualProviderHttpAndWholeStream()throws Exception {
        String host=System.getenv("ECHO_AGENT_SDK_HOST");boolean configured=host!=null&&!host.isBlank();
        if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(configured,"required Host missing");
        Assumptions.assumeTrue(configured,"actual Host required");
        String output=System.getenv("ECHO_AGENT_SDK_CONSUMER_ROOT");assertNotNull(output,"consumer root required");
        Path sdkRoot=Path.of("../..").toAbsolutePath().normalize();
        Path runtime=Path.of(output).toAbsolutePath().normalize();assertTrue(runtime.startsWith(sdkRoot.getParent())&&!runtime.equals(sdkRoot.getParent()));
        Files.createDirectories(runtime);
        for(String protocol:List.of("responses","anthropic","chat_completions")) {
            for(String mode:List.of("success","stop_continue","cancel","eof")) scenario(host,sdkRoot,runtime,protocol,mode);
        }
    }
    private void scenario(String host,Path sdkRoot,Path runtime,String protocol,String mode)throws Exception {
        Path work=Files.createTempDirectory(runtime,"http-java-"+protocol+"-"+mode+"-");
        var command=new ArrayList<>(List.of(System.getenv().getOrDefault("PYTHON","python3"),
            sdkRoot.resolve("sdks/shared/testing/consumer_provider_http.py").toString(),"--work",work.toString(),"--protocol",protocol,"--mode",mode));
        if("1".equals(System.getenv("ECHO_AGENT_SDK_CONSUMER_IMAGE"))) command.add("--image");
        Process server=new ProcessBuilder(command).redirectOutput(work.resolve("fixture.stdout").toFile()).redirectError(work.resolve("fixture.stderr").toFile()).start();
        var tools=new CopyOnWriteArrayList<String>();var publicFacts=new CopyOnWriteArrayList<JsonNode>();
        var live=new CopyOnWriteArrayList<JsonNode>();var stopCalls=new AtomicInteger();
        try {
            barrier(work.resolve("ready"),server);
            var spec=JsonSupport.MAPPER.readTree(work.resolve("spec.json").toFile());
            try(var sdk=await(EchoAgentClient.spawn(host,List.of("--config",work.resolve("host.json").toString()),null,null,Map.of()))) {
                var registrations=new ArrayList<ExtensionRegistration>();
                for(String name:List.of("consumer_image","consumer_error")) {
                    var descriptor=(ObjectNode)spec.path("tool_descriptor").deepCopy();descriptor.put("name",name);
                    registrations.add(await(sdk.registerExtension("tool",name,descriptor,(call,cancel)->{
                        String operation=call.path("invocation").path("operation").asText();
                        var result=JsonSupport.MAPPER.createObjectNode().put("operation",operation);
                        if("tool_validate_parameters".equals(operation)) result.putNull("value");
                        else {
                            assertEquals("tool_execute",operation);tools.add(name);
                            try {Files.writeString(work.resolve(name+"-effect"),Long.toString(tools.stream().filter(name::equals).count()));}
                            catch(Exception error) {return CompletableFuture.failedFuture(error);}
                            result.set("value",spec.path("results").path(name).deepCopy());
                        }
                        var outcome=JsonSupport.MAPPER.createObjectNode().put("outcome","result");outcome.set("result",result);
                        return CompletableFuture.completedFuture(outcome);
                    })));
                }
                if("stop_continue".equals(mode)) {
                    var descriptor=JsonSupport.MAPPER.createObjectNode().put("kind","hook").put("descriptor_version",1);descriptor.putArray("events").add("Stop");
                    registrations.add(await(sdk.registerExtension("hook","continue-once",descriptor,(call,cancel)->{
                        var result=(ObjectNode)spec.path("hook_allow").deepCopy();
                        if(stopCalls.getAndIncrement()==0) result.put("continue_reason","continue original execution");
                        var wrapped=JsonSupport.MAPPER.createObjectNode().put("operation",call.path("invocation").path("operation").asText());wrapped.set("value",result);
                        var outcome=JsonSupport.MAPPER.createObjectNode().put("outcome","result");outcome.set("result",wrapped);return CompletableFuture.completedFuture(outcome);
                    })));
                }
                var agent=await(sdk.createAgent());var session=await(agent.createSession(work.toString()));
                var run=await(session.startChat("two actual tool results and final validation"));
                var done=new CompletableFuture<Void>();
                run.events().subscribe(new Flow.Subscriber<JsonNode>() {
                    public void onSubscribe(Flow.Subscription subscription){subscription.request(Long.MAX_VALUE);}
                    public void onNext(JsonNode item) {
                        assertTrue(item.has("envelope"),"unexpected gap");var envelope=item.required("envelope");live.add(envelope.deepCopy());
                        var payload=envelope.path("payload");String kind=payload.path("event_type").asText();var data=payload.path("data");
                        var fact=JsonSupport.MAPPER.createObjectNode().put("kind",kind);
                        if("assistant_message".equals(kind)) fact.put("text",field(field(data,"block"),"delta").path("value").asText());
                        else if("final_answer".equals(kind)) fact.put("text",data.path("value").asText());
                        else if("execution_status".equals(kind)) fact.put("phase",field(data,"phase").path("value").asText()).put("status",field(data,"status").path("value").asText()).put("execution_id",field(data,"execution_id").path("value").asText());
                        else return;
                        publicFacts.add(fact);
                    }
                    public void onError(Throwable error){done.completeExceptionally(error);}
                    public void onComplete(){done.complete(null);}
                });
                if("cancel".equals(mode)){barrier(work.resolve("cancel-ready"),server);await(run.cancel());}
                var receipt=await(run.waitForSettlement());done.get(25,TimeUnit.SECONDS);
                var replay=await(run.replay("0",512));var facts=JsonSupport.MAPPER.createObjectNode();
                facts.set("receipt",receipt);facts.put("stop_calls",stopCalls.get());
                var calls=facts.putArray("tool_calls");tools.forEach(calls::add);
                var visibility=facts.putArray("public");publicFacts.forEach(visibility::add);
                var captured=facts.putArray("live");live.forEach(captured::add);
                facts.set("replay",replay.path("events"));
                JsonSupport.MAPPER.writeValue(work.resolve("facts.json").toFile(),facts);
                var verify=new ArrayList<>(command);verify.add("--verify");
                Process checker=new ProcessBuilder(verify).redirectOutput(work.resolve("verify.stdout").toFile()).redirectError(work.resolve("verify.stderr").toFile()).start();
                assertTrue(checker.waitFor(10,TimeUnit.SECONDS),"verifier timed out");assertEquals(0,checker.exitValue(),Files.readString(work.resolve("verify.stderr")));
                run.close();session.close();agent.close();Collections.reverse(registrations);registrations.forEach(ExtensionRegistration::close);
            }
        } finally {
            if(server.isAlive()) server.destroy();assertTrue(server.waitFor(10,TimeUnit.SECONDS),"fixture process remained alive");
            Files.writeString(work.resolve("fixture-exit.json"),"{\"exit\":"+server.exitValue()+"}");
        }
    }
}
