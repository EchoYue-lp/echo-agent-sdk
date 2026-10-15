package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicBoolean;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

class StreamCancellationRouteHostTest {
    @Test void setupAckRetainsRealHostCancelRoute() throws Exception {
        String host=System.getenv("ECHO_AGENT_SDK_HOST");assumeTrue(host!=null,"real Host required");
        Path work=Files.createTempDirectory("sdk-java-early-ack-");
        var config=new ProcessBuilder("python3","../shared/testing/mcp_channel_config_fixture.py",work.toString()).inheritIO().start();assertEquals(0,config.waitFor());
        var sdk=EchoAgentClient.spawn(host,List.of("--config",work.resolve("host.json").toString()),Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"),Map.of()).toCompletableFuture().get(5,TimeUnit.SECONDS);
        var entered=new CountDownLatch(1);var cancelled=new CountDownLatch(1);var release=new CountDownLatch(1);var shutdown=new AtomicBoolean();var facts=new CopyOnWriteArrayList<String>();var jobs=new CopyOnWriteArrayList<CompletableFuture<Void>>();
        try {
            var descriptor=JsonSupport.MAPPER.readTree("{\"kind\":\"custom_agent\",\"descriptor_version\":1,\"name\":\"route-target\",\"model_name\":\"fixture\",\"system_prompt\":\"fixture\",\"tool_names\":[]}");
            var reg=sdk.registerExtension("custom_agent","route-target",descriptor,(call,cancellation)->{
                String operation=call.path("invocation").path("operation").asText();var outcome=JsonSupport.MAPPER.createObjectNode();
                if("agent_close".equals(operation)){outcome.put("outcome","result");outcome.putObject("result").put("operation",operation).putNull("value");return CompletableFuture.completedFuture(outcome);}
                assertEquals("agent_execute_stream",operation);
                var job=CompletableFuture.runAsync(()->{try{while(!cancellation.isCancelled()&&!shutdown.get())Thread.sleep(5);if(!cancellation.isCancelled())return;facts.add("signal");cancelled.countDown();release.await();sdk.streamWriter(WireHandle.fromJson(call.path("stream"))).complete(JsonSupport.MAPPER.readTree("{\"kind\":\"agent\",\"value\":{\"event\":\"cancelled\"}}")).toCompletableFuture().get(3,TimeUnit.SECONDS);facts.add("terminal-success");}catch(Exception error){throw new CompletionException(error);}finally{facts.add("finally");}});
                jobs.add(job);facts.add("setup-ack");entered.countDown();outcome.put("outcome","stream");outcome.set("stream",call.path("stream"));return CompletableFuture.completedFuture(outcome);
            }).toCompletableFuture().get(5,TimeUnit.SECONDS);
            var agent=sdk.createAgent().toCompletableFuture().get(5,TimeUnit.SECONDS);var session=agent.createSession().toCompletableFuture().get(5,TimeUnit.SECONDS);
            var params=JsonSupport.MAPPER.createObjectNode().put("idempotency_id","cancel-route");params.set("session",session.wire().toJson());params.set("request",WireValues.value(Map.of("agent_name","route-target","task","retain cancel after setup 中文🌊","mode_override","sync")));
            var dispatch=sdk.request("_echo_agent/subagent/dispatch",params).toCompletableFuture().get(5,TimeUnit.SECONDS);assertTrue(entered.await(2,TimeUnit.SECONDS));
            var control=JsonSupport.MAPPER.createObjectNode().put("action","cancel");control.set("subagent",dispatch.path("subagent"));sdk.request("_echo_agent/subagent/control",control).toCompletableFuture().get(5,TimeUnit.SECONDS);
            assertTrue(cancelled.await(2,TimeUnit.SECONDS),"setup ACK cancellation did not reach original native producer");assertEquals(List.of("setup-ack","signal"),facts);
            release.countDown();CompletableFuture.allOf(jobs.toArray(CompletableFuture[]::new)).get(5,TimeUnit.SECONDS);assertEquals(List.of("setup-ack","signal","terminal-success","finally"),facts);
            session.close();reg.close();agent.close();
        } finally {release.countDown();shutdown.set(true);CompletableFuture.allOf(jobs.toArray(CompletableFuture[]::new)).handle((value,error)->null).get(5,TimeUnit.SECONDS);sdk.close();}
    }
}
