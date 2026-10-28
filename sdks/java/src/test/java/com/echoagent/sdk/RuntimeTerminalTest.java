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
