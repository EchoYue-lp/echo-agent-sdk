package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.BooleanSupplier;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** External replay consumption retires only its ACK-covered live queue prefix. */
class ExternalAckHostTest {
    private static <T> T await(CompletionStage<T> stage) throws Exception { return stage.toCompletableFuture().get(10, TimeUnit.SECONDS); }
    private static void until(BooleanSupplier value) throws Exception {
        long end = System.nanoTime() + TimeUnit.SECONDS.toNanos(8);
        while (!value.getAsBoolean()) {
            if (System.nanoTime() > end) throw new TimeoutException("external ACK Host barrier not reached");
            java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(5));
        }
    }
    private static List<JsonNode> notifications(Path work) {
        try {
            var result = new ArrayList<JsonNode>(); Path file = work.resolve("notifications.jsonl");
            if (!Files.exists(file)) return result;
            for (var line : Files.readAllLines(file)) {
                try { var frame = JsonSupport.MAPPER.readTree(line); if (frame.has("method")) result.add(frame); }
                catch (java.io.IOException ignored) { /* Trace writes are line-buffered, with a possible in-flight suffix. */ }
            }
            return result;
        } catch (java.io.IOException error) { throw new CompletionException(error); }
    }
    private static List<JsonNode> gaps(Path work) {
        return notifications(work).stream().filter(frame -> "_echo_agent/gap".equals(frame.path("method").textValue())).toList();
    }
    private static void addConsumed(Map<BigInteger, JsonNode> consumed, JsonNode event) {
        BigInteger sequence = new BigInteger(event.path("sequence").textValue());
        assertNull(consumed.putIfAbsent(sequence, canonicalMaps(event)), "duplicate consumed sequence " + sequence);
    }
    private static JsonNode canonicalMaps(JsonNode value) {
        // WireValue maps represent associations, and framework HashMap metadata
        // can enumerate them differently on live conversion and durable replay.
        // Compare the complete entry multiset: sorting retains duplicate pairs,
        // while sequence duplicate admission remains forbidden above.
        if (value.isObject()) {
            var copy = JsonSupport.MAPPER.createObjectNode();
            value.fields().forEachRemaining(field -> copy.set(field.getKey(), canonicalMaps(field.getValue())));
            if ("map".equals(copy.path("kind").textValue()) && copy.path("value").isArray()) {
                var entries = new ArrayList<JsonNode>(); copy.path("value").forEach(entries::add);
                entries.sort(Comparator.comparing(JsonNode::toString));
                var sorted = JsonSupport.MAPPER.createArrayNode(); entries.forEach(sorted::add); copy.set("value", sorted);
            }
            return copy;
        }
        if (value.isArray()) { var copy = JsonSupport.MAPPER.createArrayNode(); value.forEach(item -> copy.add(canonicalMaps(item))); return copy; }
        return value.deepCopy();
    }
    private static CompletionStage<Void> ack(EchoAgentClient sdk, WireHandle stream, String sequence) {
        var cursor = JsonSupport.MAPPER.createObjectNode().put("last_processed_sequence", sequence); cursor.set("stream", stream.toJson());
        return sdk.notify("_echo_agent/event/ack", JsonSupport.MAPPER.createObjectNode().set("ack", cursor));
    }
    @ParameterizedTest @ValueSource(ints = {1, 2})
    void snapshotAckBeforeLateAttachmentRetiresCoveredPrefixAndPreservesContinuation(int window) throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST"); boolean provided = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(provided, "required Host E2E binary is missing");
        assumeTrue(provided, "external ACK regression requires real Host");
        var work = Files.createTempDirectory("sdk-java-external-ack-"); Path testing = Path.of("../shared/testing").toAbsolutePath();
        var model = new ProcessBuilder("python3", testing.resolve("gap_ack_fixture.py").toString(), "--work", work.toString(), "--mode", "window")
                .redirectError(work.resolve("model.stderr").toFile()).redirectOutput(work.resolve("model.stdout").toFile()).start();
        try {
            until(() -> Files.exists(work.resolve("ready")) || !model.isAlive()); assertTrue(model.isAlive());
            var config = JsonSupport.MAPPER.readTree(work.resolve("host.json").toFile());
            ((com.fasterxml.jackson.databind.node.ObjectNode) config.path("sdk_profile").path("limits")).put("max_outstanding_live_events", window);
            JsonSupport.MAPPER.writeValue(work.resolve("host.json").toFile(), config);
            try (var sdk = await(EchoAgentClient.spawn("python3", List.of(testing.resolve("gap_ack_host_proxy.py").toString(), host, work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                    Map.of("NO_PROXY", "127.0.0.1,localhost", "no_proxy", "127.0.0.1,localhost", "RUST_LOG", "error")))) {
                var agent = await(sdk.createAgent()); var session = await(agent.createSession()); var run = await(session.startChat("external ACK 中文🙂"));
                until(() -> Files.exists(work.resolve("parked")) && !gaps(work).isEmpty());
                JsonNode initialGap = gaps(work).get(0).path("params").path("gap");
                BigInteger firstWatermark = new BigInteger(initialGap.path("snapshot_watermark").textValue());
                assertEquals(BigInteger.valueOf(window + 1L), firstWatermark);
                var prefix = await(run.replay("0", 64)); var consumed = new TreeMap<BigInteger, JsonNode>();
                for (var event : prefix.path("events")) {
                    if (new BigInteger(event.path("sequence").textValue()).compareTo(firstWatermark) <= 0) addConsumed(consumed, event);
                }
                assertEquals(firstWatermark.intValueExact(), consumed.size());
                await(ack(sdk, run.stream(), firstWatermark.toString()));
                Files.createFile(work.resolve("release"));
                until(() -> gaps(work).size() >= 2);
                JsonNode newGap = gaps(work).get(1).path("params").path("gap");
                assertTrue(new BigInteger(newGap.path("from_sequence").textValue()).compareTo(firstWatermark) > 0);
                assertEquals("completed", await(run.waitForSettlement(Duration.ofSeconds(5))).path("terminal").path("status").textValue());
                var received = new ArrayBlockingQueue<JsonNode>(64); var failure = new AtomicReference<Throwable>(); var completed = new CountDownLatch(1);
                run.events().subscribe(new Flow.Subscriber<>() {
                    @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
                    @Override public void onNext(JsonNode value) { if (!received.offer(value)) throw new IllegalStateException("test recorder overflow"); }
                    @Override public void onError(Throwable error) { failure.set(error); completed.countDown(); }
                    @Override public void onComplete() { completed.countDown(); }
                });
                JsonNode delayedGap = null;
                for (int count = 0; count <= window; count++) {
                    var value = received.poll(5, TimeUnit.SECONDS);
                    assertNotNull(value, "late queue terminated or lost resumed data: " + failure.get());
                    if (value.has("gap")) { delayedGap = value.path("gap"); break; }
                    var event = value.path("envelope");
                    assertTrue(new BigInteger(event.path("sequence").textValue()).compareTo(firstWatermark) > 0, "late subscriber repeated the externally consumed prefix");
                    addConsumed(consumed, event);
                }
                assertNotNull(delayedGap, "new gap must remain observable at bounded capacity");
                assertTrue(new BigInteger(delayedGap.path("from_sequence").textValue()).compareTo(firstWatermark) > 0, "coalescing must not resurrect an ACK-covered gap range");
                var continuation = await(run.replay(consumed.lastKey().toString(), 64));
                continuation.path("events").forEach(event -> addConsumed(consumed, event));
                await(ack(sdk, run.stream(), continuation.path("next_cursor").path("last_processed_sequence").textValue()));
                var all = await(run.replay("0", 64));
                assertEquals(all.path("events").size(), consumed.size());
                var expected = new TreeMap<BigInteger, JsonNode>(); all.path("events").forEach(event -> addConsumed(expected, event));
                assertEquals(expected, consumed, "consumed replay prefix + delayed live + recovery must equal full journal");
                assertEquals(1, consumed.values().stream().filter(event -> "final_answer".equals(event.path("payload").path("event_type").textValue())).count());
                assertNull(failure.get()); run.close(); session.close(); agent.close();
            }
        } finally { model.destroy(); if (!model.waitFor(5, TimeUnit.SECONDS)) model.destroyForcibly(); }
    }
    private static JsonNode localEvent(WireHandle stream, String sequence) {
        var event = JsonSupport.MAPPER.createObjectNode(); event.set("stream", stream.toJson());
        event.set("envelope", JsonSupport.MAPPER.createObjectNode().put("stream_id", stream.id()).put("sequence", sequence)); return event;
    }
    private static JsonNode localGap(WireHandle stream, String from, String to, String watermark) {
        var value = JsonSupport.MAPPER.createObjectNode(); value.set("stream", stream.toJson());
        value.set("gap", JsonSupport.MAPPER.createObjectNode().put("from_sequence", from).put("to_sequence", to)
                .put("snapshot_watermark", watermark).put("reason", "fixture gap")); return value;
    }
    @org.junit.jupiter.api.Test void consumedProjectionRetiresPendingAndSubscribedQueuesWithoutFutureAckAdmission() throws Exception {
        for (boolean subscribed : List.of(false, true)) {
            var stream = new WireHandle("local-stream", "1", "stream"); var publisher = new BoundedPublisher(1, ignored -> {}, stream);
            var values = new CopyOnWriteArrayList<JsonNode>(); var demand = new AtomicReference<Flow.Subscription>(); var complete = new CountDownLatch(1);
            Flow.Subscriber<JsonNode> subscriber = new Flow.Subscriber<>() {
                @Override public void onSubscribe(Flow.Subscription subscription) { demand.set(subscription); }
                @Override public void onNext(JsonNode value) { values.add(value); }
                @Override public void onError(Throwable error) { complete.countDown(); }
                @Override public void onComplete() { complete.countDown(); }
            };
            if (subscribed) publisher.subscribe(subscriber);
            publisher.publishEvent(localEvent(stream, "1"), stream.id()); publisher.publishGap(localGap(stream, "2", "5", "5"), stream.id());
            var stale = new WireHandle(stream.id(), "2", "stream");
            assertFalse(publisher.ownsStream(stale));
            publisher.acknowledgeConsumedCursor(stale, BigInteger.valueOf(5));
            assertFalse(publisher.isClosed(), "stale user ACK must not close the valid feed");
            assertEquals(BigInteger.valueOf(5), publisher.lastSequence());
            assertTrue(publisher.canAcknowledgeObserved(BigInteger.ONE)); assertTrue(publisher.canAcknowledgeObserved(BigInteger.valueOf(5)));
            assertFalse(publisher.canAcknowledgeObserved(BigInteger.valueOf(3)), "an arbitrary cursor inside a gap needs replay proof");
            assertFalse(publisher.canAcknowledgeObserved(BigInteger.valueOf(6)), "future ACK cannot clear buffers");
            publisher.acknowledgeConsumedCursor(stream, BigInteger.valueOf(5));
            publisher.publishEvent(localEvent(stream, "1"), stream.id()); // Transport retry remains locally covered.
            publisher.publishGap(localGap(stream, "2", "5", "5"), stream.id());
            publisher.publishEvent(localEvent(stream, "6"), stream.id()); publisher.publishGap(localGap(stream, "7", "7", "7"), stream.id());
            assertFalse(publisher.isClosed(), "external consumption releases exactly the covered data quota"); publisher.close();
            if (!subscribed) publisher.subscribe(subscriber); demand.get().request(Long.MAX_VALUE);
            assertTrue(complete.await(2, TimeUnit.SECONDS)); assertEquals(2, values.size());
            assertEquals("6", values.get(0).path("envelope").path("sequence").textValue());
            assertEquals("7", values.get(1).path("gap").path("from_sequence").textValue());
        }
    }
    @org.junit.jupiter.api.Test void partialProvenReplayConsumptionTrimsOnlyCoveredGapRange() throws Exception {
        var stream = new WireHandle("local-stream", "1", "stream"); var publisher = new BoundedPublisher(1, ignored -> {}, stream);
        publisher.publishEvent(localEvent(stream, "1"), stream.id()); publisher.publishGap(localGap(stream, "2", "5", "5"), stream.id());
        publisher.acknowledgeConsumedCursor(stream, BigInteger.valueOf(3));
        assertEquals(BigInteger.valueOf(5), publisher.lastSequence(), "consumed cursor must not retract the live watermark");
        publisher.acknowledgeConsumedCursor(stream, BigInteger.ONE); // Late ACK cannot resurrect consumed data.
        publisher.close(); var values = new CopyOnWriteArrayList<JsonNode>(); var complete = new CountDownLatch(1);
        publisher.subscribe(new Flow.Subscriber<>() {
            @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
            @Override public void onNext(JsonNode value) { values.add(value); }
            @Override public void onError(Throwable error) { complete.countDown(); }
            @Override public void onComplete() { complete.countDown(); }
        });
        assertTrue(complete.await(2, TimeUnit.SECONDS)); assertEquals(1, values.size());
        assertEquals("4", values.get(0).path("gap").path("from_sequence").textValue());
        assertEquals("5", values.get(0).path("gap").path("snapshot_watermark").textValue());
    }
}
