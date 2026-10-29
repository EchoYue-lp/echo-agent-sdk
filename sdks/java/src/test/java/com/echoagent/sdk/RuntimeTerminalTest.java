package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.Flow;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** SDK真实feed/ACP适配的合成输入，不能冒充真实Framework/Host运行验收。 */
class RuntimeTerminalTest {
    @Test void nativeForwardingRetainsPendingSendWhenSourceFails() throws Exception {
        for (boolean sourceThrows : List.of(false, true)) {
            for (boolean sendFails : List.of(false, true)) {
                var transport = new StreamCancellationRoutesTest.Transport();
                var sdk = StreamCancellationRoutesTest.client(transport);
                CompletableFuture<Void> held = null;
                try {
                    var reads = new java.util.concurrent.atomic.AtomicInteger();
                    var notice = JsonSupport.MAPPER.readTree("{\"event\":\"final_answer\",\"data\":{\"text\":\"original notice\"}}");
                    var terminal = JsonSupport.MAPPER.readTree("{\"event\":\"execution_status\",\"data\":{\"execution_id\":\"original\",\"sequence\":\"2\",\"phase\":\"Terminal\",\"status\":\"settled\"}}");
                    Iterable<JsonNode> source = () -> new java.util.Iterator<>() {
                        int position;
                        @Override public boolean hasNext() { return position < 3; }
                        @Override public JsonNode next() {
                            reads.incrementAndGet();
                            if (position++ == 0) return notice;
                            if (sourceThrows) throw new IllegalArgumentException("original source iterator failed");
                            return position == 2 ? terminal : notice;
                        }
                    };
                    var result = RichMessageAgentExample.forwardNativeOutput(
                            sdk.streamWriter(new WireHandle("held-native", "1", "stream")), source).toCompletableFuture();
                    held = StreamCancellationRoutesTest.send(transport);
                    assertEquals(1, reads.get(), "held original send must apply backpressure to the source");
                    assertFalse(result.isDone(), "source failure cannot detach the original pending send");
                    if (sendFails) held.completeExceptionally(new IllegalStateException("original notify failed"));
                    else held.complete(null);
                    var failure = assertThrows(java.util.concurrent.ExecutionException.class,
                            () -> result.get(2, TimeUnit.SECONDS));
                    Throwable cause = failure;
                    while (cause.getCause() != null) cause = cause.getCause();
                    assertTrue(cause.getMessage().contains(sendFails ? "original notify failed"
                            : sourceThrows ? "original source iterator failed" : "after its runtime terminal"));
                    assertTrue(transport.entered.isEmpty(), "failed source must not submit runtime completion");
                } finally {
                    if (held != null) held.complete(null);
                    sdk.closeAsync().toCompletableFuture().get(2, TimeUnit.SECONDS);
                }
            }
        }
    }

    @Test void reverseNativeStatusRequiresOriginalTextAndCorrectTransportPosition() throws Exception {
        for (String kind : List.of("started", "waiting", "tool_pending", "candidate", "settled", "failed", "cancelled")) {
            boolean terminal = List.of("settled", "failed", "cancelled").contains(kind);
            var data = JsonSupport.MAPPER.createObjectNode().put("execution_id", "原执行🧪")
                    .put("sequence", "18446744073709551615").put("phase", terminal ? "Terminal" : "Thinking").put("status", kind);
            var value = JsonSupport.MAPPER.createObjectNode().put("event", "execution_status"); value.set("data", data);
            var payload = JsonSupport.MAPPER.createObjectNode().put("kind", "agent"); payload.set("value", value);
            RuntimeTerminal.validateReverseAgentValue(payload, terminal);
            assertThrows(EchoAgentException.class, () -> RuntimeTerminal.validateReverseAgentValue(payload, !terminal));
            data.put("sequence", 1);
            assertThrows(EchoAgentException.class, () -> RuntimeTerminal.validateReverseAgentValue(payload, terminal));
            data.put("sequence", "18446744073709551615").put("adapter_generated", true);
            assertThrows(EchoAgentException.class, () -> RuntimeTerminal.validateReverseAgentValue(payload, terminal));
        }
    }

    @Test void nativeForwardingPreservesOriginalSourceEofAndNeverMintsSuccess() throws Exception {
        var transport = new StreamCancellationRoutesTest.Transport();
        var sdk = StreamCancellationRoutesTest.client(transport);
        try {
            var writer = sdk.streamWriter(new WireHandle("native-forward", "1", "stream"));
            var notice = JsonSupport.MAPPER.readTree("{\"event\":\"final_answer\",\"data\":{\"text\":\"原确认🙂\"}}");
            var terminal = JsonSupport.MAPPER.readTree("{\"event\":\"execution_status\",\"data\":{\"execution_id\":\"original\",\"sequence\":\"18446744073709551615\",\"phase\":\"Terminal\",\"status\":\"settled\"}}");
            var ended = new java.util.concurrent.atomic.AtomicBoolean(false);
            Iterable<JsonNode> source = () -> new java.util.Iterator<>() {
                final java.util.Iterator<JsonNode> items = List.of(notice, terminal).iterator();
                @Override public boolean hasNext() { boolean next = items.hasNext(); if (!next) ended.set(true); return next; }
                @Override public JsonNode next() { return items.next(); }
            };
            var forwarded = RichMessageAgentExample.forwardNativeOutput(writer, source);
            assertFalse(ended.get(), "source iteration waits for the original first send");
            StreamCancellationRoutesTest.send(transport).complete(null);
            assertTrue(ended.get(), "the terminal submission follows the original source EOF");
            StreamCancellationRoutesTest.send(transport).complete(null);
            forwarded.toCompletableFuture().get(2, TimeUnit.SECONDS);
            assertThrows(IllegalStateException.class, () -> writer.chunk(JsonSupport.MAPPER.nullNode()));
            var incomplete = sdk.streamWriter(new WireHandle("incomplete-native", "1", "stream"));
            var missing = RichMessageAgentExample.forwardNativeOutput(incomplete, List.of(notice));
            StreamCancellationRoutesTest.send(transport).complete(null);
            assertThrows(java.util.concurrent.ExecutionException.class, () -> missing.toCompletableFuture().get(2, TimeUnit.SECONDS));
        } finally { sdk.closeAsync().toCompletableFuture().get(2, TimeUnit.SECONDS); }
    }

    private static final WireHandle STREAM = new WireHandle("stream", "1", "stream");
    private static JsonNode status(String kind) {
        var value = JsonSupport.MAPPER.createObjectNode().put("execution_id", "physical-execution")
                .put("request_id", "request").put("sequence", "18446744073709551615")
                .put("phase", "Terminal").put("status", kind);
        return value;
    }
    private static JsonNode envelope(int sequence, String kind, JsonNode data) {
        var result = JsonSupport.MAPPER.createObjectNode().put("schema_version", 4).put("event_id", "event-" + sequence)
                .put("content_hash", "sha256:" + "a".repeat(64)).put("stream_id", "stream")
                .put("sequence", Integer.toString(sequence)).put("turn_id", "turn").put("execution_id", "outer-execution");
        result.putObject("timestamp").put("unix_seconds", "0").put("nanos", 0);
        result.putObject("payload").put("event_type", kind).set("data", WireValues.value(JSONtoObject(data)));
        return result;
    }
    private static Object JSONtoObject(JsonNode data) { return JsonSupport.MAPPER.convertValue(data, Object.class); }
    private static JsonNode event(JsonNode envelope) {
        var value = JsonSupport.MAPPER.createObjectNode(); value.set("stream", STREAM.toJson()); value.set("envelope", envelope); return value;
    }

    @Test void nativeTerminalPreservesFinalBodyObservationAndAllKinds() throws Exception {
        for (String kind : List.of("settled", "cancelled", "failed")) {
            try (var fixture = SessionParentCloseTest.fixture()) {
                var run = fixture.session().startChat("task").toCompletableFuture().get(2, TimeUnit.SECONDS);
                var values = new CopyOnWriteArrayList<JsonNode>();
                var done = new CompletableFuture<Void>();
                run.events().subscribe(new Flow.Subscriber<>() {
                    @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
                    @Override public void onNext(JsonNode value) { values.add(value); }
                    @Override public void onError(Throwable error) { done.completeExceptionally(error); }
                    @Override public void onComplete() { done.complete(null); }
                });
                var emptyFinal = envelope(1, "final_answer", JsonSupport.MAPPER.getNodeFactory().textNode(""));
                var observation = envelope(2, "error", JsonSupport.MAPPER.createObjectNode().put("message", "ExecutionNotSettled"));
                var candidate = envelope(3, "execution_status", status("candidate"));
                var body = envelope(4, "assistant_message", JsonSupport.MAPPER.createObjectNode().put("delta", "尾部正文🙂"));
                for (var item : List.of(emptyFinal, observation, candidate, body)) {
                    fixture.client().receiveEvent(event(item)).block();
                    assertFalse(done.isDone(), item.toString());
                }
                fixture.client().receiveEvent(event(envelope(5, "execution_status", status(kind)))).block();
                done.get(2, TimeUnit.SECONDS);
                assertEquals(5, values.size());
                assertEquals("assistant_message", values.get(3).path("envelope").path("payload").path("event_type").asText());
            }
        }
    }

    @Test void firstEventDoesNotTreatEmptyFinalAsDone() throws Exception {
        try (var fixture = SessionParentCloseTest.fixture()) {
            fixture.client().publishFirstEvent(STREAM, envelope(1, "final_answer", JsonSupport.MAPPER.getNodeFactory().textNode("")));
            fixture.client().publishFirstEvent(STREAM, envelope(2, "execution_status", status("settled")));
            // 实际publisher已经收到两个事实；完成仍来自最后runtime状态。
            var run = new RunHandle(fixture.client(), new WireHandle("run", "1", "run"), STREAM);
            var done = new CompletableFuture<Void>(); var values = new CopyOnWriteArrayList<JsonNode>();
            run.events().subscribe(new Flow.Subscriber<>() {
                @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
                @Override public void onNext(JsonNode value) { values.add(value); }
                @Override public void onError(Throwable error) { done.completeExceptionally(error); }
                @Override public void onComplete() { done.complete(null); }
            });
            done.get(2, TimeUnit.SECONDS); assertEquals(2, values.size());
        }
    }

    @Test void malformedRuntimeStatusCannotBeSuccess() {
        var value = status("settled").deepCopy();
        ((com.fasterxml.jackson.databind.node.ObjectNode) value).put("sequence", "01");
        var error = assertThrows(EchoAgentException.class, () -> RuntimeTerminal.isTerminal(envelope(1, "execution_status", value)));
        assertEquals("serialization_violation", error.code());
    }

    @Test void invalidWaitingWireIntegerHasTypedCode() {
        for (String encoded : List.of("01", "18446744073709551616")) {
            var waiting = (com.fasterxml.jackson.databind.node.ObjectNode) status("waiting");
            waiting.put("phase", "Thinking");
            var item = envelope(1, "execution_status", waiting);
            for (var entry : item.path("payload").path("data").path("value")) {
                if ("sequence".equals(entry.path("key").path("value").asText())) {
                    var invalid = JsonSupport.MAPPER.createObjectNode().put("kind", "u64").put("value", encoded);
                    ((com.fasterxml.jackson.databind.node.ObjectNode) entry).set("value", invalid);
                }
            }
            var error = assertThrows(EchoAgentException.class, () -> RuntimeTerminal.isTerminal(item));
            assertEquals("serialization_violation", error.code());
        }
    }

    @Test void replayClosesOnlyOnRuntimeEnvelopeAckAndKeepsGeneration() throws Exception {
        try (var fixture = SessionParentCloseTest.fixture()) {
            var publisher = (BoundedPublisher) fixture.client().events(STREAM);
            var first = replayPage(0, envelope(1, "final_answer", JsonSupport.MAPPER.getNodeFactory().textNode("")));
            fixture.client().observeReplay(STREAM, first);
            fixture.client().notify("_echo_agent/event/ack", ack(1)).toCompletableFuture().get(2, TimeUnit.SECONDS);
            assertFalse(publisher.isClosed());
            var last = replayPage(1, envelope(2, "execution_status", status("settled")));
            fixture.client().observeReplay(new WireHandle("stream", "2", "stream"), last);
            assertFalse(publisher.isClosed());
            fixture.client().observeReplay(STREAM, last);
            assertFalse(publisher.isClosed());
            fixture.client().notify("_echo_agent/event/ack", ack(2)).toCompletableFuture().get(2, TimeUnit.SECONDS);
            assertTrue(publisher.isClosed());
        }
    }

    private static JsonNode replayPage(int after, JsonNode item) {
        var value = JsonSupport.MAPPER.createObjectNode().put("requested_after_sequence", Integer.toString(after));
        value.putArray("events").add(item);
        value.putObject("next_cursor").put("stream_id", STREAM.id()).put("last_processed_sequence", item.path("sequence").asText());
        return value;
    }
    private static JsonNode ack(int sequence) {
        var value = JsonSupport.MAPPER.createObjectNode();
        var ack = value.putObject("ack").put("last_processed_sequence", Integer.toString(sequence));
        ack.set("stream", STREAM.toJson()); return value;
    }
}
