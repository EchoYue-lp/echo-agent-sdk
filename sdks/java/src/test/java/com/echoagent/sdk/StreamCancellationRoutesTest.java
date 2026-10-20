package com.echoagent.sdk;

import com.agentclientprotocol.sdk.spec.*;
import com.agentclientprotocol.sdk.json.*;
import com.fasterxml.jackson.databind.JsonNode;
import reactor.core.publisher.Mono;
import org.junit.jupiter.api.Test;
import java.nio.file.Path;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.Function;
import static org.junit.jupiter.api.Assertions.*;

class StreamCancellationRoutesTest {
    static class Transport implements AcpClientTransport {
        final BlockingQueue<CompletableFuture<Void>> entered=new LinkedBlockingQueue<>();
        final AcpJsonMapper mapper=AcpJsonMapper.createDefault();
        public Mono<Void> connect(Function<Mono<AcpSchema.JSONRPCMessage>,Mono<AcpSchema.JSONRPCMessage>> handler){return Mono.empty();}
        public Mono<Void> sendMessage(AcpSchema.JSONRPCMessage message){return Mono.defer(()->{var done=new CompletableFuture<Void>();entered.add(done);return Mono.fromFuture(done,true);});}
        public <T>T unmarshalFrom(Object value,TypeRef<T> type){return mapper.convertValue(value,type);}
        boolean failClose;
        public Mono<Void> closeGracefully(){if(failClose){failClose=false;return Mono.error(new IllegalStateException("native teardown failed"));}return Mono.empty();}
    }
    static EchoAgentClient client(Transport transport)throws Exception{
        var session=new AcpClientSession(Duration.ofSeconds(2),transport,Map.of(),Map.of(),Function.identity());
        var ctor=EchoAgentClient.class.getDeclaredConstructor(AcpClientSession.class,FacadeCatalog.class,AcpClientTransport.class);ctor.setAccessible(true);
        return ctor.newInstance(session,new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json")),transport);
    }
    @SuppressWarnings("unchecked") static Map<String,ExtensionHandler> handlers(EchoAgentClient sdk)throws Exception{var f=EchoAgentClient.class.getDeclaredField("extensionHandlers");f.setAccessible(true);return(Map<String,ExtensionHandler>)f.get(sdk);}
    @SuppressWarnings("unchecked") static Map<String,ExtensionCancellation> cancellations(EchoAgentClient sdk)throws Exception{var f=EchoAgentClient.class.getDeclaredField("invocationCancellations");f.setAccessible(true);return(Map<String,ExtensionCancellation>)f.get(sdk);}
    static Mono<JsonNode> invoke(EchoAgentClient sdk,JsonNode call)throws Exception{var f=EchoAgentClient.class.getDeclaredMethod("invokeExtension",JsonNode.class);f.setAccessible(true);@SuppressWarnings("unchecked") var result=(Mono<JsonNode>)f.invoke(sdk,call);return result;}
    static JsonNode call(){var p=JsonSupport.MAPPER.createObjectNode().put("invocation_id","original");p.set("extension",new WireHandle("extension","1","extension").toJson());p.set("stream",new WireHandle("stream","1","stream").toJson());p.putObject("invocation").put("operation","agent_execute_stream").putObject("input").put("task","中文🌊");return p;}
    static JsonNode outcome(JsonNode call){var p=JsonSupport.MAPPER.createObjectNode().put("outcome","stream");p.set("stream",call.path("stream"));return p;}
    static CompletableFuture<Void> send(Transport transport)throws Exception{var done=transport.entered.poll(2,TimeUnit.SECONDS);assertNotNull(done);return done;}
    static JsonNode terminal(String generation){var p=JsonSupport.MAPPER.createObjectNode().put("event","cancelled").put("sequence","1");p.set("stream",new WireHandle("stream",generation,"stream").toJson());return p;}
    @Test void earlySetupAckWrongGenerationAndFailedTerminalKeepCancellationRoute()throws Exception{
        var transport=new Transport();var sdk=client(transport);handlers(sdk).put("extension",(c,cancel)->CompletableFuture.completedFuture(outcome(c)));
        assertEquals("stream",invoke(sdk,call()).block(Duration.ofSeconds(2)).path("outcome").asText());var token=cancellations(sdk).get("original");assertNotNull(token);
        var wrong=sdk.notify("_echo_agent/extension/stream",terminal("2"));send(transport).complete(null);wrong.toCompletableFuture().get(2,TimeUnit.SECONDS);assertSame(token,cancellations(sdk).get("original"));
        var failed=sdk.notify("_echo_agent/extension/stream",terminal("1"));send(transport).completeExceptionally(new IllegalStateException("native completion unknown"));assertThrows(ExecutionException.class,()->failed.toCompletableFuture().get(2,TimeUnit.SECONDS));assertSame(token,cancellations(sdk).get("original"));
        var actual=sdk.notify("_echo_agent/extension/stream",terminal("1"));send(transport).complete(null);actual.toCompletableFuture().get(2,TimeUnit.SECONDS);assertFalse(cancellations(sdk).containsKey("original"));sdk.close();
    }
    @Test void cancelledSetupAckKeepsAddressUntilDeclaredTerminal()throws Exception{
        var transport=new Transport();var sdk=client(transport);var result=new CompletableFuture<JsonNode>();handlers(sdk).put("extension",(c,cancel)->result);
        var response=invoke(sdk,call()).toFuture();var token=cancellations(sdk).get("original");assertNotNull(token);token.cancel();result.complete(outcome(call()));assertEquals("cancelled",response.get(2,TimeUnit.SECONDS).path("error").path("code").asText());assertSame(token,cancellations(sdk).get("original"));
        var actual=sdk.notify("_echo_agent/extension/stream",terminal("1"));send(transport).complete(null);actual.toCompletableFuture().get(2,TimeUnit.SECONDS);assertTrue(cancellations(sdk).isEmpty());sdk.close();
    }
    @Test void terminalBeforeSetupResponseCannotResurrectTheRoute()throws Exception{
        var transport=new Transport();var sdk=client(transport);var result=new CompletableFuture<JsonNode>();handlers(sdk).put("extension",(c,cancel)->result);
        var response=invoke(sdk,call()).toFuture();var actual=sdk.notify("_echo_agent/extension/stream",terminal("1"));send(transport).complete(null);actual.toCompletableFuture().get(2,TimeUnit.SECONDS);result.complete(outcome(call()));response.get(2,TimeUnit.SECONDS);assertTrue(cancellations(sdk).isEmpty());sdk.close();
    }

    @Test void failedCloseRetainsTheSameCancellationAddressUntilSuccessfulRetry()throws Exception {
        var transport=new Transport();var sdk=client(transport);handlers(sdk).put("extension",(c,cancel)->CompletableFuture.completedFuture(outcome(c)));
        invoke(sdk,call()).block(Duration.ofSeconds(2));var token=cancellations(sdk).get("original");assertNotNull(token);transport.failClose=true;
        assertThrows(ExecutionException.class,()->sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS));assertTrue(token.isCancelled());assertSame(token,cancellations(sdk).get("original"));
        sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertTrue(cancellations(sdk).isEmpty());
    }
    static JsonNode observationCases()throws Exception {
        return JsonSupport.MAPPER.readTree(java.nio.file.Files.readString(Path.of("../shared/testing/cancellation-observation-cases.json"))).get("cases");
    }
    @Test void publicWriterKeepsOnlyCompleteLegitimateObservationsOpen()throws Exception {
        for(var item:observationCases()) {
            var transport=new Transport();var sdk=client(transport);
            try {
                var writer=sdk.streamWriter(new WireHandle("original-stream","1","stream"));
                var sent=writer.failed(item.get("error").deepCopy());send(transport).complete(null);sent.toCompletableFuture().get(2,TimeUnit.SECONDS);
                var actual=JsonSupport.MAPPER.readTree("{\"kind\":\"agent\",\"value\":{\"event\":\"final_answer\",\"data\":{\"text\":\"actual final value\"}}}");
                if(item.path("nonterminal").asBoolean()) {
                    var terminal=writer.complete(actual);send(transport).complete(null);terminal.toCompletableFuture().get(2,TimeUnit.SECONDS);
                } else assertThrows(IllegalStateException.class,()->writer.complete(actual),item.path("name").asText());
            } finally {sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);}
        }
    }
    @Test void officialObservationSendRetainsOriginalCancelRouteUntilTerminal()throws Exception {
        var transport=new Transport();var sdk=client(transport);
        try {
            handlers(sdk).put("extension",(call,cancel)->CompletableFuture.completedFuture(outcome(call)));
            invoke(sdk,call()).block(Duration.ofSeconds(2));var original=cancellations(sdk).get("original");assertNotNull(original);
            var params=JsonSupport.MAPPER.createObjectNode().put("event","failed").put("sequence","1");params.set("stream",new WireHandle("stream","1","stream").toJson());params.set("error",observationCases().get(0).get("error"));
            var sent=sdk.notify("_echo_agent/extension/stream",params);send(transport).complete(null);sent.toCompletableFuture().get(2,TimeUnit.SECONDS);
            assertSame(original,cancellations(sdk).get("original"));
            var actual=sdk.notify("_echo_agent/extension/stream",terminal("1"));send(transport).complete(null);actual.toCompletableFuture().get(2,TimeUnit.SECONDS);assertTrue(cancellations(sdk).isEmpty());
        } finally {sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);}
    }
    @Test void explicitObservationWinsLocalCallbackCancelAndKeepsOwner()throws Exception {
        for(var item:observationCases()) {
            var transport=new Transport();var sdk=client(transport);var error=item.get("error");
            try {
                handlers(sdk).put("extension",(call,cancel)->{cancel.cancel();var outcome=JsonSupport.MAPPER.createObjectNode().put("outcome","error");outcome.set("error",error);return CompletableFuture.completedFuture(outcome);});
                var result=invoke(sdk,call()).block(Duration.ofSeconds(2));
                if(item.path("nonterminal").asBoolean()) {
                    assertEquals(error,result.path("error"),item.path("name").asText());assertTrue(cancellations(sdk).containsKey("original"));
                } else {
                    assertEquals("cancelled",result.path("error").path("code").asText(),item.path("name").asText());assertFalse(cancellations(sdk).containsKey("original"));
                }
            } finally {sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);}
        }
    }

    @Test void lateObservationCannotReopenWriterAfterConcurrentTerminal()throws Exception {
        var submitEntered=new CountDownLatch(1);var release=new CountDownLatch(1);
        var firstSend=new java.util.concurrent.atomic.AtomicBoolean(true);
        var transport=new Transport(){
            @Override public Mono<Void> sendMessage(AcpSchema.JSONRPCMessage message){
                return Mono.defer(()->{
                    if(firstSend.compareAndSet(true,false)) {
                        submitEntered.countDown();
                        try{if(!release.await(2,TimeUnit.SECONDS))return Mono.error(new IllegalStateException("fixture first submit was not released"));}
                        catch(InterruptedException interrupted){Thread.currentThread().interrupt();return Mono.error(interrupted);}
                    }
                    return super.sendMessage(message);
                });
            }
        };
        var sdk=client(transport);var executor=Executors.newSingleThreadExecutor();
        var writer=sdk.streamWriter(new WireHandle("original-stream","1","stream"));
        try {
            var first=executor.submit(()->writer.failed(observationCases().get(0).get("error")));
            assertTrue(submitEntered.await(2,TimeUnit.SECONDS));
            // Hold failed() within its real synchronous notify submission, not
            // merely its returned stage: Java commits closed before stage completion.
            var terminal=writer.complete(JsonSupport.MAPPER.readTree("{\"kind\":\"agent\",\"value\":{\"event\":\"final_answer\",\"data\":{\"text\":\"original final\"}}}"));
            send(transport).complete(null);terminal.toCompletableFuture().get(2,TimeUnit.SECONDS);
            release.countDown();var observation=first.get(2,TimeUnit.SECONDS);
            send(transport).complete(null);observation.toCompletableFuture().get(2,TimeUnit.SECONDS);
            assertThrows(IllegalStateException.class,()->writer.chunk(JsonSupport.MAPPER.nullNode()));
            assertTrue(transport.entered.isEmpty());
        } finally {
            release.countDown();executor.shutdownNow();assertTrue(executor.awaitTermination(3,TimeUnit.SECONDS));
            sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
        }
    }

}
