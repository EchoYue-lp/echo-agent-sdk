package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** Original HTTP model input proves accepted steering consumption, not only ACK. */
class RunSteerHostTest {
    private static final Path FIXTURE=Path.of("../shared/testing/run_steer_fixture.py").toAbsolutePath();
    private static final String HOST=System.getenv("ECHO_AGENT_SDK_HOST");
    private static <T>T await(CompletionStage<T> value)throws Exception{return value.toCompletableFuture().get(10,TimeUnit.SECONDS);}
    private static void barrier(Path file)throws Exception{
        long deadline=System.nanoTime()+TimeUnit.SECONDS.toNanos(10);
        while(!Files.exists(file)){if(System.nanoTime()>deadline)throw new TimeoutException("original model barrier");Thread.sleep(10);}
    }
    @Test void acceptedSteeringReachesModelAndConcurrentWaitsShareOriginalReceipt()throws Exception{
        if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))&&(HOST==null||HOST.isBlank()))throw new AssertionError("required steering Host is missing");
        org.junit.jupiter.api.Assumptions.assumeTrue(HOST!=null&&!HOST.isBlank(),"real Host required");
        Path work=Files.createTempDirectory(Path.of(System.getenv().getOrDefault("TMPDIR",System.getProperty("java.io.tmpdir"))),"sdk-run-steer-java-");
        Process model=new ProcessBuilder("python3",FIXTURE.toString(),"--work",work.toString()).inheritIO().start();
        EchoAgentClient sdk=null;
        try{
            barrier(work.resolve("ready"));JsonNode scenario=JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());
            sdk=await(EchoAgentClient.spawn(HOST,List.of("--config",work.resolve("host.json").toString()),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of("NO_PROXY","127.0.0.1,localhost","no_proxy","127.0.0.1,localhost")));
            var agent=await(sdk.createAgent());var session=await(agent.createSession(work.toString()));var run=await(session.startChat(scenario.path("initial").asText()));
            var identity=JsonSupport.MAPPER.createObjectNode();identity.set("run",run.wire().toJson());identity.set("stream",run.stream().toJson());barrier(work.resolve("parked-1"));
            var first=run.waitForSettlement().toCompletableFuture();var secondWait=run.waitForSettlement().toCompletableFuture();
            var accepted=await(run.steer(scenario.path("steer").asText()));var acceptance=JsonSupport.MAPPER.createObjectNode();acceptance.set("identity",identity);acceptance.set("accepted",accepted);acceptance.set("snapshot",await(run.get()));JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("accepted.json").toFile(),acceptance);System.out.println("Run steer acceptance evidence "+work);assertTrue(accepted.path("accepted").booleanValue());assertFalse(accepted.path("steer_id").asText().isBlank());assertFalse(first.isDone());assertFalse(secondWait.isDone());Files.createFile(work.resolve("release-1"));barrier(work.resolve("parked-2"));
            var request=JsonSupport.MAPPER.readTree(work.resolve("request-2.json").toFile());List<String> contents=new ArrayList<>();for(var message:request.path("messages"))contents.add(message.path("content").asText());for(String field:List.of("initial","draft","steer"))assertTrue(contents.contains(scenario.path(field).asText()));assertFalse(first.isDone());assertFalse(secondWait.isDone());assertEquals("running",await(run.get()).path("status").asText());Files.createFile(work.resolve("release-2"));
            JsonNode one=await(first),two=await(secondWait);assertEquals(one,two);assertTrue(one.path("settled").booleanValue());assertEquals(run.wire().id(),one.path("receipt").path("turn_id").asText());assertEquals(scenario.path("answer"),one.path("receipt").path("final_answer"));assertEquals("2",one.path("receipt").path("llm_calls").asText());assertEquals("completed",one.path("terminal").path("status").asText());var snapshot=await(run.get());assertEquals(one.path("receipt"),snapshot.path("receipt"));assertEquals(identity.path("stream"),snapshot.path("stream"));var page=await(run.replay("0",64));int terminals=0;for(var event:page.path("events"))if("final_answer".equals(event.path("payload").path("event_type").asText()))terminals++;assertEquals(1,terminals);
            var rejected=await(run.steer("late steering"));assertFalse(rejected.path("accepted").booleanValue());assertTrue(rejected.path("steer_id").isMissingNode()||rejected.path("steer_id").isNull());var empty=assertThrows(ExecutionException.class,()->await(run.steer("")));var error=EchoAgentException.from(empty,"_echo_agent/run/steer");assertEquals("invalid_value",error.code());assertEquals("_echo_agent/run/steer",error.operation());assertEquals(one,await(run.waitForSettlement()));
            var observed=JsonSupport.MAPPER.createObjectNode();observed.set("identity",identity);observed.set("accepted",accepted);observed.set("rejected",rejected);observed.putArray("waits").add(one).add(two);observed.set("snapshot",snapshot);observed.set("page",page);observed.set("second",request);JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(),observed);System.out.println("Run steer native evidence "+work);await(session.closeAsync());await(agent.closeAsync());
        }finally{
            Files.writeString(work.resolve("release-1"),"");Files.writeString(work.resolve("release-2"),"");if(sdk!=null)sdk.close();model.destroy();if(!model.waitFor(2,TimeUnit.SECONDS))throw new AssertionError("original fixture model did not exit");
        }
    }
}
