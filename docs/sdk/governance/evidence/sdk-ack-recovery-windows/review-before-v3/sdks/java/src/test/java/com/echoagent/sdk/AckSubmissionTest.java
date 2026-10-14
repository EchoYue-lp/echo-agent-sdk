package com.echoagent.sdk;

import com.agentclientprotocol.sdk.json.AcpJsonMapper;
import com.agentclientprotocol.sdk.json.TypeRef;
import com.agentclientprotocol.sdk.spec.AcpClientSession;
import com.agentclientprotocol.sdk.spec.AcpClientTransport;
import com.agentclientprotocol.sdk.spec.AcpSchema;
import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import reactor.core.publisher.Mono;
import java.math.BigInteger;
import java.nio.file.Path;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.Function;
import static org.junit.jupiter.api.Assertions.*;

/** Real official ACP session, controlled native submission completion, no fabricated Run receipt. */
class AckSubmissionTest {
    private static final class Transport implements AcpClientTransport {
        final BlockingQueue<CompletableFuture<Void>> entered = new LinkedBlockingQueue<>();
        final List<AcpSchema.JSONRPCMessage> messages = new CopyOnWriteArrayList<>();
        JsonNode replayResponse;
        Function<Mono<AcpSchema.JSONRPCMessage>,Mono<AcpSchema.JSONRPCMessage>> handler;
        private final AcpJsonMapper mapper=AcpJsonMapper.createDefault();
        @Override public Mono<Void> connect(Function<Mono<AcpSchema.JSONRPCMessage>,Mono<AcpSchema.JSONRPCMessage>> handler){this.handler=handler;return Mono.empty();}
        @Override public Mono<Void> sendMessage(AcpSchema.JSONRPCMessage message){return Mono.defer(()->{if(message instanceof AcpSchema.JSONRPCRequest request && replayResponse!=null)return handler.apply(Mono.just(new AcpSchema.JSONRPCResponse("2.0",request.id(),replayResponse,null))).then();var completion=new CompletableFuture<Void>();messages.add(message);entered.add(completion);return Mono.fromFuture(completion,true);});}
        @Override public <T>T unmarshalFrom(Object value,TypeRef<T> type){return mapper.convertValue(value,type);}
        @Override public Mono<Void> closeGracefully(){return Mono.empty();}
    }
    private static EchoAgentClient client(Transport transport)throws Exception{
        var session=new AcpClientSession(Duration.ofSeconds(1),transport,Map.of(),Map.of(),Function.identity());
        var catalog=new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"),Path.of("../shared/contract-digests.json"));
        var ctor=EchoAgentClient.class.getDeclaredConstructor(AcpClientSession.class,FacadeCatalog.class,AcpClientTransport.class);ctor.setAccessible(true);return ctor.newInstance(session,catalog,transport);
    }
    @SuppressWarnings("unchecked") private static Map<String,BoundedPublisher> feeds(EchoAgentClient sdk)throws Exception{var field=EchoAgentClient.class.getDeclaredField("eventPublishers");field.setAccessible(true);return (Map<String,BoundedPublisher>)field.get(sdk);}
    private static JsonNode event(WireHandle stream,String sequence){var value=JsonSupport.MAPPER.createObjectNode();value.set("stream",stream.toJson());value.putObject("envelope").put("sequence",sequence).put("stream_id",stream.id());return value;}
    private static JsonNode ack(WireHandle stream,String sequence){var value=JsonSupport.MAPPER.createObjectNode();value.putObject("ack").put("last_processed_sequence",sequence).set("stream",stream.toJson());return value;}
    @Test void nativeSubmissionKeepsValidatedParametersSnapshot() throws Exception {
        var transport = new Transport();
        var sdk = client(transport);
        var stream = new WireHandle("stream", "1", "stream");
        var publisher = new BoundedPublisher(8, ignored -> {}, stream);
        feeds(sdk).put(stream.id(), publisher);
        publisher.publishEvent(event(stream, "1"), stream.id());
        sdk.observeReplay(stream, replay(stream));
        var params = ack(stream, "10");
        var call = sdk.notify("_echo_agent/event/ack", params);
        var send = entered(transport);
        ((com.fasterxml.jackson.databind.node.ObjectNode) params.path("ack"))
                .put("last_processed_sequence", "2");
        var actual = (AcpSchema.JSONRPCNotification) transport.messages.stream().findFirst().orElseThrow();
        var serialized = JsonSupport.MAPPER.valueToTree(actual.params());
        send.complete(null);
        call.toCompletableFuture().get(2, TimeUnit.SECONDS);
        assertEquals("10", serialized.path("ack").path("last_processed_sequence").asText());
        assertEquals(BigInteger.TEN, publisher.lastSequence());
        sdk.close();
    }

    @Test void nativeFailureRetainsUnconsumedQueuePrefixAndAllowsRetry()throws Exception{
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);
        publisher.publishEvent(event(stream,"1"),stream.id());publisher.publishEvent(event(stream,"2"),stream.id());
        var failed=sdk.notify("_echo_agent/event/ack",ack(stream,"2"));var send=transport.entered.poll(1,TimeUnit.SECONDS);assertNotNull(send);send.completeExceptionally(new IllegalStateException("native write rejected"));assertThrows(ExecutionException.class,()->failed.toCompletableFuture().get(1,TimeUnit.SECONDS));
        publisher.close();var values=new CopyOnWriteArrayList<String>();var complete=new CountDownLatch(1);
        publisher.subscribe(new Flow.Subscriber<>(){public void onSubscribe(Flow.Subscription sub){sub.request(Long.MAX_VALUE);}public void onNext(JsonNode value){values.add(value.path("envelope").path("sequence").textValue());}public void onError(Throwable error){complete.countDown();}public void onComplete(){complete.countDown();}});
        assertTrue(complete.await(2,TimeUnit.SECONDS));assertEquals(List.of("1","2"),values);
    }
    private static JsonNode replay(WireHandle stream) {
        var value=JsonSupport.MAPPER.createObjectNode().put("requested_after_sequence","0");
        value.putObject("next_cursor").put("last_processed_sequence","10");
        value.putArray("events").addObject().put("sequence","10").putObject("payload").put("event_type","final_answer");
        return value;
    }
    private static CompletableFuture<Void> entered(Transport transport)throws Exception {
        var completion=transport.entered.poll(2,TimeUnit.SECONDS);assertNotNull(completion);return completion;
    }
    @Test void concurrentCompletionPermutationsCommitOnlySuccessfulCursor()throws Exception{
        for(var order:List.of(List.of(2,10),List.of(10,2)))for(var successes:List.of(Set.of(2),Set.of(10),Set.of(2,10),Set.<Integer>of())){
            var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");
            var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());sdk.observeReplay(stream,replay(stream));
            var calls=new HashMap<Integer,CompletionStage<Void>>();var sends=new HashMap<Integer,CompletableFuture<Void>>();
            for(int sequence:List.of(2,10)){calls.put(sequence,sdk.notify("_echo_agent/event/ack",ack(stream,Integer.toString(sequence))));sends.put(sequence,entered(transport));}
            assertFalse(publisher.isClosed());assertEquals(BigInteger.ONE,publisher.lastSequence());
            int committed=0;
            for(int sequence:order){
                if(successes.contains(sequence)){sends.get(sequence).complete(null);calls.get(sequence).toCompletableFuture().get(2,TimeUnit.SECONDS);committed=Math.max(committed,sequence);}
                else{ sends.get(sequence).completeExceptionally(new IllegalStateException("native write rejected"));var error=assertThrows(ExecutionException.class,()->calls.get(sequence).toCompletableFuture().get(2,TimeUnit.SECONDS));assertEquals("outcome_unknown",EchoAgentException.from(error,"ACK").details().path("ack_submission").textValue()); }
                assertEquals(committed>=10,publisher.isClosed());assertEquals(BigInteger.valueOf(Math.max(1,committed)),publisher.lastSequence());
            }
            sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
        }
    }
    @Test void duplicateOlderFailureCannotEraseLaterSuccess()throws Exception{
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());sdk.observeReplay(stream,replay(stream));
        var first=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));var one=entered(transport);var second=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));var two=entered(transport);
        two.complete(null);second.toCompletableFuture().get(2,TimeUnit.SECONDS);one.completeExceptionally(new IllegalStateException("older duplicate rejected"));assertThrows(ExecutionException.class,()->first.toCompletableFuture().get(2,TimeUnit.SECONDS));
        assertTrue(publisher.isClosed());assertEquals(BigInteger.TEN,publisher.lastSequence());sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
    }
    @Test void callerCancellationDoesNotCancelActualSubmissionOrLateSuccess()throws Exception{
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());sdk.observeReplay(stream,replay(stream));
        var caller=sdk.notify("_echo_agent/event/ack",ack(stream,"10")).toCompletableFuture();var send=entered(transport);assertTrue(caller.cancel(true));assertFalse(send.isCancelled());assertFalse(publisher.isClosed());assertEquals(BigInteger.ONE,publisher.lastSequence());
        send.complete(null);assertTrue(publisher.isClosed());assertEquals(BigInteger.TEN,publisher.lastSequence());sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
    }
    @Test void pendingCoveragePermitsActualLiveContinuationButCannotRetireBufferedPrefix()throws Exception{
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());sdk.observeReplay(stream,replay(stream));
        var caller=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));var send=entered(transport);publisher.publishEvent(event(stream,"11"),stream.id());
        send.completeExceptionally(new IllegalStateException("write outcome unknown after peer continued"));assertThrows(ExecutionException.class,()->caller.toCompletableFuture().get(2,TimeUnit.SECONDS));assertFalse(publisher.isClosed());assertEquals(BigInteger.valueOf(11),publisher.lastSequence());
        publisher.publishEvent(event(stream,"12"),stream.id());publisher.close();var values=new CopyOnWriteArrayList<String>();var complete=new CountDownLatch(1);
        publisher.subscribe(new Flow.Subscriber<>(){public void onSubscribe(Flow.Subscription sub){sub.request(Long.MAX_VALUE);}public void onNext(JsonNode value){values.add(value.path("envelope").path("sequence").textValue());}public void onError(Throwable error){complete.countDown();}public void onComplete(){complete.countDown();}});
        assertTrue(complete.await(2,TimeUnit.SECONDS));assertEquals(List.of("1","11","12"),values);sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
    }
    @Test void nativeFailureThenRetryCanCommitTerminalWithoutChangingEarlierFailureFact()throws Exception{
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());sdk.observeReplay(stream,replay(stream));
        var failure=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));entered(transport).completeExceptionally(new IllegalStateException("write rejected"));assertThrows(ExecutionException.class,()->failure.toCompletableFuture().get(2,TimeUnit.SECONDS));assertFalse(publisher.isClosed());assertEquals(BigInteger.ONE,publisher.lastSequence());
        var retry=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));entered(transport).complete(null);retry.toCompletableFuture().get(2,TimeUnit.SECONDS);assertTrue(publisher.isClosed());assertEquals(BigInteger.TEN,publisher.lastSequence());sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
    }
    @Test void sdkCloseSettlesItsPendingSubmissionOwnerWithoutSuccessfulCursor()throws Exception{
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());sdk.observeReplay(stream,replay(stream));
        var caller=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));entered(transport);sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);assertThrows(ExecutionException.class,()->caller.toCompletableFuture().get(2,TimeUnit.SECONDS));assertEquals(BigInteger.ONE,publisher.lastSequence());
    }
    @Test void lowerPendingCursorCannotPermitBackwardsLiveEvent()throws Exception{
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());sdk.observeReplay(stream,replay(stream));
        var low=sdk.notify("_echo_agent/event/ack",ack(stream,"2"));var one=entered(transport);var high=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));var two=entered(transport);
        publisher.publishEvent(event(stream,"11"),stream.id());publisher.publishEvent(event(stream,"3"),stream.id());assertEquals(BigInteger.valueOf(11),publisher.lastSequence());
        var failed=new CompletableFuture<Throwable>();publisher.subscribe(new Flow.Subscriber<>(){public void onSubscribe(Flow.Subscription subscription){subscription.request(Long.MAX_VALUE);}public void onNext(JsonNode value){fail("backwards live fact cannot be delivered");}public void onError(Throwable error){failed.complete(error);}public void onComplete(){failed.completeExceptionally(new IllegalStateException("invalid feed cannot report successful EOF"));}});
        assertEquals("event_gap",EchoAgentException.from(failed.get(2,TimeUnit.SECONDS),"event").code());one.complete(null);two.complete(null);low.toCompletableFuture().get(2,TimeUnit.SECONDS);high.toCompletableFuture().get(2,TimeUnit.SECONDS);sdk.closeAsync().toCompletableFuture().get(2,TimeUnit.SECONDS);
    }

    @Test void retentionReplayBridgeRequiresOfficialSubmissionSuccess() throws Exception {
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");
        var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());
        transport.replayResponse=JsonSupport.MAPPER.readTree("{\"requested_after_sequence\":\"1\",\"gap\":{\"from_sequence\":\"2\",\"to_sequence\":\"5\",\"snapshot_watermark\":\"5\",\"reason\":\"retention floor\"},\"next_cursor\":{\"stream_id\":\"stream\",\"last_processed_sequence\":\"6\"},\"events\":[{\"stream_id\":\"stream\",\"sequence\":\"6\",\"event_id\":\"event6\",\"turn_id\":\"turn\",\"content_hash\":\"sha256:"+"0".repeat(64)+"\",\"schema_version\":1,\"timestamp\":{\"unix_seconds\":\"0\",\"nanos\":0},\"payload\":{\"event_type\":\"delta\"}}]}");
        var run=new RunHandle(sdk,new WireHandle("run","1","run"),stream);
        var retention=transport.replayResponse;var empty=JsonSupport.MAPPER.createObjectNode().put("requested_after_sequence","5");empty.putArray("events");empty.putObject("next_cursor").put("stream_id",stream.id()).put("last_processed_sequence","5");transport.replayResponse=empty;
        run.replay("5",8).toCompletableFuture().get(2,TimeUnit.SECONDS);transport.replayResponse=retention;
        run.replay("1",8).toCompletableFuture().get(2,TimeUnit.SECONDS);
        assertEquals(BigInteger.ONE,publisher.lastSequence());
        var unknown=sdk.notify("_echo_agent/event/ack",ack(stream,"3"));entered(transport).complete(null);unknown.toCompletableFuture().get(2,TimeUnit.SECONDS);assertEquals(BigInteger.ONE,publisher.lastSequence());
        var call=sdk.notify("_echo_agent/event/ack",ack(stream,"6"));var send=entered(transport);assertEquals(BigInteger.ONE,publisher.lastSequence());send.complete(null);call.toCompletableFuture().get(2,TimeUnit.SECONDS);
        assertEquals(BigInteger.valueOf(6),publisher.lastSequence());
        publisher.publishEvent(event(stream,"7"),stream.id());publisher.close();
        var values=new CopyOnWriteArrayList<String>();var complete=new CompletableFuture<Void>();
        publisher.subscribe(new Flow.Subscriber<>(){public void onSubscribe(Flow.Subscription sub){sub.request(Long.MAX_VALUE);}public void onNext(JsonNode value){values.add(value.path("envelope").path("sequence").asText());}public void onError(Throwable error){complete.completeExceptionally(error);}public void onComplete(){complete.complete(null);}});
        complete.get(2,TimeUnit.SECONDS);assertEquals(List.of("7"),values);sdk.close();
    }
    @Test void pendingReplayAckPermitsGapWithoutCommittingOrLosingPrefix() throws Exception {
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");
        var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());
        var page=(com.fasterxml.jackson.databind.node.ObjectNode)replay(stream);page.putArray("events");sdk.observeReplay(stream,page);
        var call=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));var send=entered(transport);
        var gap=JsonSupport.MAPPER.createObjectNode();gap.set("stream",stream.toJson());gap.putObject("gap").put("from_sequence","11").put("to_sequence","12").put("snapshot_watermark","12").put("reason","oversized event");publisher.publishGap(gap,stream.id());
        assertEquals(BigInteger.valueOf(12),publisher.lastSequence());assertFalse(publisher.isClosed());
        send.completeExceptionally(new IllegalStateException("outcome unknown after peer gap"));assertThrows(ExecutionException.class,()->call.toCompletableFuture().get(2,TimeUnit.SECONDS));
        publisher.publishEvent(event(stream,"13"),stream.id());publisher.close();var values=new CopyOnWriteArrayList<String>();var complete=new CompletableFuture<Void>();
        publisher.subscribe(new Flow.Subscriber<>(){public void onSubscribe(Flow.Subscription sub){sub.request(Long.MAX_VALUE);}public void onNext(JsonNode value){values.add(value.has("gap")?"gap12":value.path("envelope").path("sequence").asText());}public void onError(Throwable error){complete.completeExceptionally(error);}public void onComplete(){complete.complete(null);}});
        complete.get(2,TimeUnit.SECONDS);assertEquals(List.of("1","gap12","13"),values);sdk.close();
    }

    @Test void gapPlusEmptyAfterGapPageCannotAuthorizeMissingCursor() throws Exception {
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");
        var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());
        var gap=JsonSupport.MAPPER.createObjectNode();gap.set("stream",stream.toJson());gap.putObject("gap").put("from_sequence","2").put("to_sequence","12").put("snapshot_watermark","12").put("reason","pause");publisher.publishGap(gap,stream.id());
        var page=JsonSupport.MAPPER.createObjectNode().put("requested_after_sequence","10");page.putArray("events");page.putObject("next_cursor").put("stream_id",stream.id()).put("last_processed_sequence","10");sdk.observeReplay(stream,page);
        var call=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));entered(transport).complete(null);call.toCompletableFuture().get(2,TimeUnit.SECONDS);publisher.close();
        var values=new CopyOnWriteArrayList<String>();var complete=new CompletableFuture<Void>();publisher.subscribe(new Flow.Subscriber<>(){public void onSubscribe(Flow.Subscription sub){sub.request(Long.MAX_VALUE);}public void onNext(JsonNode value){values.add(value.has("gap")?"gap12":value.path("envelope").path("sequence").asText());}public void onError(Throwable error){complete.completeExceptionally(error);}public void onComplete(){complete.complete(null);}});
        complete.get(2,TimeUnit.SECONDS);assertEquals(List.of("1","gap12"),values);sdk.close();
    }

    @Test void pendingPredecessorCannotAuthorizeOverlappingGap() throws Exception {
        var transport=new Transport();var sdk=client(transport);var stream=new WireHandle("stream","1","stream");
        var publisher=new BoundedPublisher(8,ignored->{},stream);feeds(sdk).put(stream.id(),publisher);publisher.publishEvent(event(stream,"1"),stream.id());
        var page=(com.fasterxml.jackson.databind.node.ObjectNode)replay(stream);page.putArray("events");sdk.observeReplay(stream,page);
        var call=sdk.notify("_echo_agent/event/ack",ack(stream,"10"));var send=entered(transport);
        var gap=JsonSupport.MAPPER.createObjectNode();gap.set("stream",stream.toJson());gap.putObject("gap").put("from_sequence","11").put("to_sequence","12").put("snapshot_watermark","12").put("reason","oversized");publisher.publishGap(gap,stream.id());
        assertEquals(BigInteger.valueOf(12),publisher.lastSequence());
        ((com.fasterxml.jackson.databind.node.ObjectNode)gap.path("gap")).put("to_sequence","13").put("snapshot_watermark","13");publisher.publishGap(gap,stream.id());
        assertEquals(BigInteger.valueOf(12),publisher.lastSequence());
        var failure=new CompletableFuture<Throwable>();publisher.subscribe(new Flow.Subscriber<>(){public void onSubscribe(Flow.Subscription sub){sub.request(Long.MAX_VALUE);}public void onNext(JsonNode value){fail("overlapping gap cannot be delivered");}public void onError(Throwable error){failure.complete(error);}public void onComplete(){failure.completeExceptionally(new IllegalStateException("overlapping gap cannot produce EOF"));}});
        assertEquals("serialization_violation",EchoAgentException.from(failure.get(2,TimeUnit.SECONDS),"gap").code());
        send.completeExceptionally(new IllegalStateException("unknown completion"));assertThrows(ExecutionException.class,()->call.toCompletableFuture().get(2,TimeUnit.SECONDS));sdk.close();
    }
}
