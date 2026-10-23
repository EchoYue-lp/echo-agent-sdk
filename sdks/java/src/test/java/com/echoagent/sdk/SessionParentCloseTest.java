package com.echoagent.sdk;

import com.agentclientprotocol.sdk.json.AcpJsonMapper;
import com.agentclientprotocol.sdk.json.TypeRef;
import com.agentclientprotocol.sdk.spec.AcpClientSession;
import com.agentclientprotocol.sdk.spec.AcpClientTransport;
import com.agentclientprotocol.sdk.spec.AcpSchema;
import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import reactor.core.publisher.Mono;

import java.nio.file.Path;
import java.time.Duration;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.Flow;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Function;

import static org.junit.jupiter.api.Assertions.*;

/** Original ACP request adapter and publishers, with controlled Host RPC replies. */
class SessionParentCloseTest {
    private static final WireHandle AGENT = new WireHandle("agent", "1", "agent");
    private static final WireHandle SESSION = new WireHandle("session", "1", "session");
    private static final WireHandle RUN = new WireHandle("run", "1", "run");
    private static final WireHandle STREAM = new WireHandle("stream", "1", "stream");

    private static JsonNode started() {
        var value = JsonSupport.MAPPER.createObjectNode();
        value.set("run", RUN.toJson());
        value.set("stream", STREAM.toJson());
        return value;
    }

    private static final class Transport implements AcpClientTransport {
        private final AcpJsonMapper mapper = AcpJsonMapper.createDefault();
        private Function<Mono<AcpSchema.JSONRPCMessage>, Mono<AcpSchema.JSONRPCMessage>> handler;
        Function<String, CompletionStage<JsonNode>> reply = method -> CompletableFuture.completedFuture(
                method.equals("_echo_agent/run/start") ? started() : JsonSupport.MAPPER.createObjectNode());
        final List<String> calls = new CopyOnWriteArrayList<>();

        @Override public Mono<Void> connect(Function<Mono<AcpSchema.JSONRPCMessage>, Mono<AcpSchema.JSONRPCMessage>> value) {
            handler = value;
            return Mono.empty();
        }
        @Override public Mono<Void> sendMessage(AcpSchema.JSONRPCMessage message) {
            return Mono.defer(() -> {
                if (!(message instanceof AcpSchema.JSONRPCRequest request)) return Mono.empty();
                calls.add(request.method());
                var response = reply.apply(request.method()).handle((value, failure) -> {
                    if (failure == null) return new AcpSchema.JSONRPCResponse("2.0", request.id(), value, null);
                    var typed = EchoAgentException.from(failure, request.method());
                    var data = JsonSupport.MAPPER.createObjectNode().put("code", typed.code())
                            .put("message", typed.getMessage()).put("retryable", typed.retryable());
                    return new AcpSchema.JSONRPCResponse("2.0", request.id(), null,
                            new AcpSchema.JSONRPCError(-32000, typed.getMessage(), data));
                });
                return Mono.fromFuture(response.toCompletableFuture(), true)
                        .flatMap(value -> handler.apply(Mono.just(value))).then();
            });
        }
        @Override public <T> T unmarshalFrom(Object value, TypeRef<T> type) { return mapper.convertValue(value, type); }
        @Override public Mono<Void> closeGracefully() { return Mono.empty(); }
    }

    private record Fixture(EchoAgentClient client, SessionHandle session, Transport transport) implements AutoCloseable {
        @Override public void close() { client.close(); }
    }

    private static Fixture fixture() throws Exception {
        var transport = new Transport();
        var session = new AcpClientSession(Duration.ofSeconds(2), transport, Map.of(), Map.of(), Function.identity());
        var catalog = new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"));
        var constructor = EchoAgentClient.class.getDeclaredConstructor(AcpClientSession.class, FacadeCatalog.class, AcpClientTransport.class);
        constructor.setAccessible(true);
        var client = constructor.newInstance(session, catalog, transport);
        return new Fixture(client, new SessionHandle(client, AGENT, SESSION, "acp", null), transport);
    }

    private static <T> T await(CompletionStage<T> value) throws Exception {
        return value.toCompletableFuture().get(2, TimeUnit.SECONDS);
    }

    private static CompletableFuture<Void> completion(Flow.Publisher<JsonNode> publisher) {
        var complete = new CompletableFuture<Void>();
        publisher.subscribe(new Flow.Subscriber<>() {
            @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
            @Override public void onNext(JsonNode value) {}
            @Override public void onError(Throwable error) { complete.completeExceptionally(error); }
            @Override public void onComplete() { complete.complete(null); }
        });
        return complete;
    }

    @Test void parentCloseFinishesWaitingChildAndInvalidatesItsQueries() throws Exception {
        try (var f = fixture()) {
            var run = await(f.session().startChat("task"));
            var ended = completion(run.events());
            await(f.session().closeAsync());
            await(ended);
            await(run.closeAsync());
            var error = assertThrows(java.util.concurrent.ExecutionException.class, () -> await(run.get()));
            assertEquals("stale_handle", EchoAgentException.from(error, "get").code());
        }
    }

    @Test void firstSubscriptionsAfterParentCloseAreFinite() throws Exception {
        try (var f = fixture()) {
            var run = await(f.session().startChat("task"));
            await(f.session().closeAsync());
            await(completion(run.events()));
            await(completion(f.session().updates()));
        }
    }

    @Test void lateStartReplyCannotReopenChildAfterParentClose() throws Exception {
        try (var f = fixture()) {
            var start = new CompletableFuture<JsonNode>();
            f.transport().reply = method -> method.equals("_echo_agent/run/start")
                    ? start : CompletableFuture.completedFuture(JsonSupport.MAPPER.createObjectNode());
            var starting = f.session().startChat("task");
            await(f.session().closeAsync());
            start.complete(started());
            var run = await(starting);
            await(completion(run.events()));
            assertThrows(java.util.concurrent.ExecutionException.class, () -> await(run.cancel()));
        }
    }

    @Test void failedParentCloseRetainsChildAndAllowsRetry() throws Exception {
        try (var f = fixture()) {
            var attempts = new AtomicInteger();
            f.transport().reply = method -> {
                if (method.equals("_echo_agent/run/start")) return CompletableFuture.completedFuture(started());
                if (method.equals("_echo_agent/session/close") && attempts.incrementAndGet() == 1)
                    return CompletableFuture.failedFuture(new EchoAgentException("busy", "retry close", "after_delay", method, null));
                return CompletableFuture.completedFuture(JsonSupport.MAPPER.createObjectNode());
            };
            var run = await(f.session().startChat("task"));
            var ended = completion(run.events());
            assertThrows(java.util.concurrent.ExecutionException.class, () -> await(f.session().closeAsync()));
            assertFalse(ended.isDone());
            await(run.get());
            await(f.session().closeAsync());
            await(ended);
            assertEquals(2, attempts.get());
        }
    }

    @Test void concurrentParentClosersShareOriginalRequestEvenIfOneWaiterCancels() throws Exception {
        try (var f = fixture()) {
            var reply = new CompletableFuture<JsonNode>();
            f.transport().reply = method -> reply;
            var first = f.session().closeAsync().toCompletableFuture();
            var second = f.session().closeAsync().toCompletableFuture();
            first.cancel(true);
            assertFalse(second.isDone(), "a caller cannot cancel the shared close owner");
            reply.complete(JsonSupport.MAPPER.createObjectNode());
            await(second);
            assertEquals(1, f.transport().calls.stream().filter("_echo_agent/session/close"::equals).count());
        }
    }

    @Test void childCloseReleasesLocalPublisherAfterAuthoritativeHandleLoss() throws Exception {
        for (var code : List.of("closed_handle", "stale_handle", "host_exited")) {
            try (var f = fixture()) {
                f.transport().reply = method -> CompletableFuture.failedFuture(new EchoAgentException(code, "owner gone", "never", method, null));
                var run = new RunHandle(f.client(), RUN, STREAM);
                var ended = completion(run.events());
                await(run.closeAsync());
                await(ended);
                await(run.closeAsync());
            }
        }
    }

    @Test void shutdownInProgressRetainsChildPublisherAndTypedFailure() throws Exception {
        try (var f = fixture()) {
            f.transport().reply = method -> CompletableFuture.failedFuture(new EchoAgentException(
                    "host_shutting_down", "not disposed yet", "after_delay", method, null));
            var run = new RunHandle(f.client(), RUN, STREAM);
            var ended = completion(run.events());
            var error = assertThrows(java.util.concurrent.ExecutionException.class, () -> await(run.closeAsync()));
            assertEquals("host_shutting_down", EchoAgentException.from(error, "close").code());
            assertFalse(ended.isDone());
        }
    }

    @Test void concurrentChildClosersShareSettlementAndProtectTheirOwnerFromCancellation() throws Exception {
        try (var f = fixture()) {
            var reply = new CompletableFuture<JsonNode>();
            f.transport().reply = method -> method.equals("_echo_agent/run/wait")
                    ? reply : CompletableFuture.completedFuture(JsonSupport.MAPPER.createObjectNode());
            var run = new RunHandle(f.client(), RUN, STREAM);
            var first = run.closeAsync().toCompletableFuture();
            var second = run.closeAsync().toCompletableFuture();
            first.cancel(true);
            assertFalse(second.isDone(), "a caller cannot cancel the shared run cleanup owner");
            reply.complete(JsonSupport.MAPPER.createObjectNode().put("settled", true));
            await(second);
            assertEquals(1, f.transport().calls.stream().filter("_echo_agent/run/cancel"::equals).count());
        }
    }

    @Test void parentDisposalReleasesChildCloseObserverWithoutCancellingOriginalRpc() throws Exception {
        try (var f = fixture()) {
            var reply = new CompletableFuture<JsonNode>();
            var entered = new CompletableFuture<Void>();
            f.transport().reply = method -> {
                if (method.equals("_echo_agent/run/start")) return CompletableFuture.completedFuture(started());
                if (method.equals("_echo_agent/run/wait")) { entered.complete(null); return reply; }
                return CompletableFuture.completedFuture(JsonSupport.MAPPER.createObjectNode());
            };
            var run = await(f.session().startChat("task"));
            var closing = run.closeAsync();
            await(entered);
            await(f.session().closeAsync());
            await(closing);
            assertFalse(reply.isDone(), "parent cleanup cannot cancel the original submitted RPC");
            reply.complete(JsonSupport.MAPPER.createObjectNode().put("settled", true));
        }
    }

    @Test void parentDisposalPreservesBufferedFactWithoutFabricatingATerminal() throws Exception {
        try (var f = fixture()) {
            var run = await(f.session().startChat("task"));
            var publisher = run.events();
            var event = JsonSupport.MAPPER.createObjectNode();
            event.set("stream", STREAM.toJson());
            event.putObject("envelope").put("stream_id", STREAM.id()).put("sequence", "1")
                    .putObject("payload").put("event_type", "delta");
            f.client().receiveEvent(event).block();
            await(f.session().closeAsync());
            var values = new CopyOnWriteArrayList<JsonNode>();
            var complete = new CompletableFuture<Void>();
            publisher.subscribe(new Flow.Subscriber<>() {
                @Override public void onSubscribe(Flow.Subscription value) { value.request(Long.MAX_VALUE); }
                @Override public void onNext(JsonNode value) { values.add(value); }
                @Override public void onError(Throwable error) { complete.completeExceptionally(error); }
                @Override public void onComplete() { complete.complete(null); }
            });
            await(complete);
            assertEquals(List.of(event), values);
        }
    }

    @Test void failedRunSettlementRetainsPublisherAndAllowsCloseRetry() throws Exception {
        try (var f = fixture()) {
            var waits = new AtomicInteger();
            f.transport().reply = method -> CompletableFuture.completedFuture(method.equals("_echo_agent/run/wait")
                    ? JsonSupport.MAPPER.createObjectNode().put("settled", waits.incrementAndGet() > 1)
                    : JsonSupport.MAPPER.createObjectNode());
            var run = new RunHandle(f.client(), RUN, STREAM);
            var ended = completion(run.events());
            var error = assertThrows(java.util.concurrent.ExecutionException.class, () -> await(run.closeAsync()));
            assertEquals("cancellation_timeout", EchoAgentException.from(error, "close").code());
            assertFalse(ended.isDone());
            await(run.closeAsync());
            await(ended);
            assertEquals(2, waits.get());
        }
    }

    @Test void parentScopeRetainsEveryLocalAliasOfTheSameHostRun() throws Exception {
        try (var f = fixture()) {
            var reply = new CompletableFuture<JsonNode>();
            var entered = new CompletableFuture<Void>();
            f.transport().reply = method -> {
                if (method.equals("_echo_agent/run/start")) return CompletableFuture.completedFuture(started());
                if (method.equals("_echo_agent/run/wait")) { entered.complete(null); return reply; }
                return CompletableFuture.completedFuture(JsonSupport.MAPPER.createObjectNode());
            };
            var first = await(f.session().startChat("task"));
            var second = await(f.session().startChat("task"));
            assertEquals(first.wire(), second.wire());
            var closing = first.closeAsync();
            await(entered);
            await(f.session().closeAsync());
            await(closing);
            assertThrows(java.util.concurrent.ExecutionException.class, () -> await(first.get()));
            assertThrows(java.util.concurrent.ExecutionException.class, () -> await(second.get()));
            assertFalse(reply.isDone());
            reply.complete(JsonSupport.MAPPER.createObjectNode().put("settled", true));
        }
    }
}
