package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.List;
import java.util.Map;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.BooleanSupplier;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Replay consumption ends only the local delivery projection, never the Run authority. */
class ReplayTerminalHostTest {
    private static <T> T await(CompletionStage<T> stage) throws Exception {
        return stage.toCompletableFuture().get(10, TimeUnit.SECONDS);
    }
    private static void until(BooleanSupplier predicate) throws Exception {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
        while (!predicate.getAsBoolean()) {
            if (System.nanoTime() > deadline) throw new TimeoutException("terminal replay fixture barrier");
            java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(5));
        }
    }
    private static int notifications(Path work) throws Exception {
        int count = 0;
        for (String line : Files.readAllLines(work.resolve("notifications.jsonl"))) {
            JsonNode frame = JsonSupport.MAPPER.readTree(line);
            String method = frame.path("method").textValue();
            if ("_echo_agent/event".equals(method) || "_echo_agent/gap".equals(method)) count++;
        }
        return count;
    }
    @Test void replayedTerminalExplicitAckCompletesPublisherWithoutCoveredRedelivery() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean provided = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(provided);
        assumeTrue(provided, "terminal replay regression requires real Host");
        Path work = Files.createTempDirectory("sdk-java-replay-terminal-");
        Path testing = Path.of("../shared/testing").toAbsolutePath();
        Process model = new ProcessBuilder("python3", testing.resolve("gap_ack_fixture.py").toString(),
                "--work", work.toString(), "--mode", "oversized")
                .redirectError(work.resolve("model.stderr").toFile()).redirectOutput(work.resolve("model.stdout").toFile()).start();
        try {
            until(() -> Files.exists(work.resolve("ready")) || !model.isAlive());
            assertTrue(model.isAlive());
            try (var sdk = await(EchoAgentClient.spawn("python3",
                    List.of(testing.resolve("gap_ack_host_proxy.py").toString(), host, work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                    Map.of("NO_PROXY", "127.0.0.1,localhost", "no_proxy", "127.0.0.1,localhost")))) {
                var agent = await(sdk.createAgent());
                var session = await(agent.createSession());
                var run = await(session.startChat("replayed terminal EOF 中文🌊"));
                until(() -> Files.exists(work.resolve("parked")));
                var gaps = new CountDownLatch(1);
                var completed = new CountDownLatch(1);
                var failure = new AtomicReference<Throwable>();
                var unexpected = new CopyOnWriteArrayList<JsonNode>();
                run.events().subscribe(new Flow.Subscriber<>() {
                    @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
                    @Override public void onNext(JsonNode value) { if (value.has("gap")) gaps.countDown(); else unexpected.add(value); }
                    @Override public void onError(Throwable error) { failure.set(error); completed.countDown(); }
                    @Override public void onComplete() { completed.countDown(); }
                });
                assertTrue(gaps.await(5, TimeUnit.SECONDS));
                Files.createFile(work.resolve("release"));
                assertTrue(await(run.waitForSettlement(Duration.ofSeconds(10))).path("settled").booleanValue());
                var page = await(run.replay("0", 64));
                int terminals = 0;
                for (JsonNode event : page.path("events")) if ("final_answer".equals(event.path("payload").path("event_type").textValue())) terminals++;
                assertEquals(1, terminals);
                String tail = page.path("next_cursor").path("last_processed_sequence").textValue();
                assertEquals(tail, page.path("events").path(page.path("events").size() - 1).path("sequence").textValue());
                int announced = notifications(work);
                assertEquals(1L, completed.getCount(), "replay fetch alone is not terminal consumption");
                var future = JsonSupport.MAPPER.createObjectNode().put("last_processed_sequence", new java.math.BigInteger(tail).add(java.math.BigInteger.ONE).toString());
                future.set("stream", run.stream().toJson());
                await(sdk.notify("_echo_agent/event/ack", JsonSupport.MAPPER.createObjectNode().set("ack", future)));
                await(run.get());
                assertEquals(1L, completed.getCount(), "uncovered future ACK must not end the feed");
                var ack = JsonSupport.MAPPER.createObjectNode().put("last_processed_sequence", tail);
                ack.set("stream", run.stream().toJson());
                var params = JsonSupport.MAPPER.createObjectNode().set("ack", ack);
                await(sdk.notify("_echo_agent/event/ack", params));
                await(run.get());
                assertEquals(announced, notifications(work));
                assertTrue(completed.await(2, TimeUnit.SECONDS), "replayed terminal ACK did not release publisher EOF");
                assertNull(failure.get());
                assertTrue(unexpected.isEmpty());
                await(sdk.notify("_echo_agent/event/ack", params));
                await(run.get());
                assertEquals(announced, notifications(work));
                session.close();
                agent.close();
            }
        } finally {
            model.destroy();
            if (!model.waitFor(5, TimeUnit.SECONDS)) model.destroyForcibly();
        }
    }
}
