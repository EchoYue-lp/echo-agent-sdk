package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.sun.net.httpserver.HttpServer;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import java.net.InetSocketAddress;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.BooleanSupplier;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** A real Host with a legal 256-event window; scripted HTTP only supplies model tokens. */
class DelayedSubscriberHostTest {
    private static <T> T await(CompletionStage<T> value) throws Exception { return value.toCompletableFuture().get(10, TimeUnit.SECONDS); }
    private static void until(BooleanSupplier condition) throws Exception {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(5);
        while (!condition.getAsBoolean()) {
            if (System.nanoTime() > deadline) throw new TimeoutException("Host delivery did not reach the delayed attachment barrier");
            java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(5));
        }
    }
    @ParameterizedTest @ValueSource(ints = {20, 300})
    void delayedAttachmentRetainsCompleteJournalUnionAndExactlyOneTerminal(int tokens) throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean provided = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(provided, "required Host E2E binary is missing");
        assumeTrue(provided, "delayed attachment needs real Host");
        var work = Files.createTempDirectory("sdk-java-delayed-");
        var model = HttpServer.create(new InetSocketAddress("127.0.0.1", 0), 0);
        model.createContext("/v1/chat/completions", request -> {
            request.getRequestBody().readAllBytes();
            request.getResponseHeaders().set("Content-Type", "text/event-stream"); request.sendResponseHeaders(200, 0);
            try (var output = request.getResponseBody()) {
                for (int i = 0; i <= tokens; i++) {
                    var delta = JsonSupport.MAPPER.createObjectNode();
                    // The framework intentionally accumulates ordinary content
                    // chunks into a final Token. Reasoning chunks are real live
                    // AgentEvent::Token facts, so they exercise the 256-event window.
                    if (i < tokens) delta.put(tokens > 256 ? "reasoning_content" : "content", "token中文🙂");
                    else if (tokens > 256) delta.put("content", "final answer 中文🙂");
                    var choice = JsonSupport.MAPPER.createObjectNode().put("index", 0); choice.set("delta", delta);
                    if (i == tokens) choice.put("finish_reason", "stop"); else choice.putNull("finish_reason");
                    var chunk = JsonSupport.MAPPER.createObjectNode().put("id", "delayed-fixture"); chunk.putArray("choices").add(choice);
                    output.write(("data: " + chunk + "\n\n").getBytes(StandardCharsets.UTF_8)); output.flush();
                }
                output.write("data: [DONE]\n\n".getBytes(StandardCharsets.UTF_8));
            }
        });
        model.start();
        try {
            var config = JsonSupport.MAPPER.readTree(Path.of("../../echo-sdk-host/config.sdk.example.json").toFile());
            ((com.fasterxml.jackson.databind.node.ObjectNode) config.path("default_agent").path("model"))
                    .put("base_url", "http://127.0.0.1:" + model.getAddress().getPort() + "/v1/chat/completions");
            ((com.fasterxml.jackson.databind.node.ObjectNode) config.path("sdk_profile")).put("state_root", work.resolve("state").toString());
            ((com.fasterxml.jackson.databind.node.ObjectNode) config.path("sdk_profile").path("limits")).put("max_outstanding_live_events", 256);
            Path configPath = work.resolve("host.json"); JsonSupport.MAPPER.writeValue(configPath.toFile(), config);
            try (var sdk = await(EchoAgentClient.spawn(host, List.of("--config", configPath.toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                    Map.of("NO_PROXY", "127.0.0.1,localhost", "no_proxy", "127.0.0.1,localhost")))) {
                var agent = await(sdk.createAgent()); var session = await(agent.createSession()); var run = await(session.startChat("delayed attachment"));
                var settled = await(run.waitForSettlement(Duration.ofSeconds(5)));
                assertTrue(settled.path("settled").asBoolean(), settled.toString());
                assertEquals("completed", settled.path("terminal").path("status").textValue(), settled.toString());
                var journal = await(run.replay("0", 512));
                var expected = new TreeMap<java.math.BigInteger, JsonNode>();
                journal.path("events").forEach(event -> expected.put(new java.math.BigInteger(event.path("sequence").textValue()), event));
                assertFalse(expected.isEmpty());
                var feed = (BoundedPublisher) run.events();
                until(() -> tokens > 256 ? feed.lastSequence().compareTo(java.math.BigInteger.valueOf(256)) > 0 : feed.isClosed());
                var items = new ArrayBlockingQueue<JsonNode>(1024); var error = new AtomicReference<Throwable>(); var completed = new CountDownLatch(1);
                run.events().subscribe(new Flow.Subscriber<>() {
                    @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
                    @Override public void onNext(JsonNode item) { if (!items.offer(item)) throw new IllegalStateException("test recorder overflow"); }
                    @Override public void onError(Throwable failure) { error.set(failure); completed.countDown(); }
                    @Override public void onComplete() { completed.countDown(); }
                });
                var union = new TreeMap<java.math.BigInteger, JsonNode>(); boolean gap = false;
                while (true) {
                    JsonNode item = items.poll(5, TimeUnit.SECONDS);
                    assertNotNull(item, "missing delayed event; publisher error=" + error.get());
                    if (item.has("gap")) { gap = true; break; }
                    JsonNode event = item.path("envelope");
                    assertNull(union.put(new java.math.BigInteger(event.path("sequence").textValue()), event), "duplicate live event");
                    if (union.size() == expected.size()) break;
                }
                if (tokens > 256) {
                    assertTrue(gap, "full negotiated data window must still deliver gap control"); assertEquals(256, union.size());
                    var replay = await(run.replay(union.lastKey().toString(), 512));
                    replay.path("events").forEach(event -> assertNull(union.put(new java.math.BigInteger(event.path("sequence").textValue()), event), "duplicate recovery event"));
                    var params = JsonSupport.MAPPER.createObjectNode();
                    params.set("ack", JsonSupport.MAPPER.createObjectNode().set("stream", run.stream().toJson()));
                    ((com.fasterxml.jackson.databind.node.ObjectNode) params.path("ack")).put("last_processed_sequence", replay.path("next_cursor").path("last_processed_sequence").textValue());
                    await(sdk.notify("_echo_agent/event/ack", params));
                } else assertFalse(gap);
                assertEquals(expected, union, "live prefix plus recovery must equal every retained journal envelope");
                assertEquals(1, union.values().stream().filter(event -> "final_answer".equals(event.path("payload").path("event_type").textValue())).count());
                run.close(); assertTrue(completed.await(5, TimeUnit.SECONDS)); assertNull(error.get());
                session.close(); agent.close();
            }
        } finally { model.stop(0); }
    }
}
