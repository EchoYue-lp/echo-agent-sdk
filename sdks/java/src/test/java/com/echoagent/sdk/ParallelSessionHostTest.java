package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.nio.file.*;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.BooleanSupplier;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Two independent Host Agents: canceling A must not settle or close B. */
class ParallelSessionHostTest {
    private static <T> T await(CompletionStage<T> stage) throws Exception { return stage.toCompletableFuture().get(15, TimeUnit.SECONDS); }
    private static void until(BooleanSupplier condition) throws Exception {
        long end = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
        while (!condition.getAsBoolean()) { if (System.nanoTime() > end) throw new TimeoutException("parallel fixture barrier"); Thread.sleep(10); }
    }
    private static List<JsonNode> collect(RunHandle run, CountDownLatch started, AtomicReference<Throwable> failure, CountDownLatch done) {
        var values = new CopyOnWriteArrayList<JsonNode>();
        run.events().subscribe(new Flow.Subscriber<>() {
            @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
            @Override public void onNext(JsonNode value) { assertFalse(value.has("gap")); assertEquals(run.stream().toJson(),value.path("stream")); assertEquals(run.stream().id(),value.path("envelope").path("stream_id").asText()); values.add(value); started.countDown(); }
            @Override public void onError(Throwable error) { failure.set(error); done.countDown(); }
            @Override public void onComplete() { done.countDown(); }
        });
        return values;
    }
    private static void verifyJournal(RunHandle run,List<JsonNode> observed) throws Exception {
        var replay=await(run.replay("0",512)).path("events"); assertEquals(observed.size(),replay.size());
        for(int i=0;i<observed.size();i++){
            var envelope=observed.get(i).path("envelope"); assertEquals(Integer.toString(i+1),envelope.path("sequence").asText());
            assertEquals(replay.path(i).path("event_id"),envelope.path("event_id"));
        }
        assertEquals(1,observed.stream().filter(v->List.of("final_answer","cancelled","error").contains(v.path("envelope").path("payload").path("event_type").asText())).count());
    }
    @Test void independentAgentsKeepBAliveAfterACancelAndEachOwnJournal() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST"); assumeTrue(host != null && !host.isBlank());
        Path work = Files.createTempDirectory("sdk-java-parallel-");
        Path testing = Path.of("../shared/testing").toAbsolutePath();
        Process model = new ProcessBuilder("python3", testing.resolve("parallel_session_fixture.py").toString(), "--work", work.toString()).start();
        try {
            until(() -> Files.exists(work.resolve("ready")));
            try (var sdk = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()))) {
                var agentA = await(sdk.createAgent()); var agentB = await(sdk.createAgent());
                var sessionA = await(agentA.createSession(null)); var sessionB = await(agentB.createSession(null));
                var runA = await(sessionA.startChat("lane-a 中文🌊")); var runB = await(sessionB.startChat("lane-b 中文🌊"));
                assertNotEquals(sessionA.wire().id(), sessionB.wire().id()); assertNotEquals(runA.stream().id(), runB.stream().id());
                var startedA = new CountDownLatch(1); var startedB = new CountDownLatch(1); var doneA = new CountDownLatch(1); var doneB = new CountDownLatch(1);
                var errorA = new AtomicReference<Throwable>(); var errorB = new AtomicReference<Throwable>();
                var valuesA = collect(runA, startedA, errorA, doneA); var valuesB = collect(runB, startedB, errorB, doneB);
                assertTrue(startedA.await(5, TimeUnit.SECONDS)); assertTrue(startedB.await(5, TimeUnit.SECONDS));
                until(() -> Files.exists(work.resolve("parked-a")) && Files.exists(work.resolve("parked-b")));
                await(runA.cancel()); assertEquals("cancelled", await(runA.waitForSettlement(Duration.ofSeconds(10))).path("terminal").path("status").asText());
                assertTrue(doneA.await(5, TimeUnit.SECONDS)); assertNull(errorA.get()); assertEquals("cancelled", valuesA.get(valuesA.size()-1).path("envelope").path("payload").path("event_type").asText());
                verifyJournal(runA,valuesA); sessionA.close(); agentA.close();
                assertFalse(doneB.await(250, TimeUnit.MILLISECONDS), "closing A must not complete B feed");
                assertFalse(await(runB.waitForSettlement(Duration.ZERO)).path("settled").asBoolean());
                Files.createFile(work.resolve("release-b"));
                assertEquals("completed", await(runB.waitForSettlement(Duration.ofSeconds(10))).path("terminal").path("status").asText());
                assertTrue(doneB.await(5, TimeUnit.SECONDS)); assertNull(errorB.get()); assertEquals("final_answer", valuesB.get(valuesB.size()-1).path("envelope").path("payload").path("event_type").asText());
                var idsA = new HashSet<String>(); valuesA.forEach(v -> idsA.add(v.path("envelope").path("event_id").asText()));
                valuesB.forEach(v -> assertFalse(idsA.contains(v.path("envelope").path("event_id").asText())));
                verifyJournal(runB,valuesB);
                sessionB.close(); agentB.close();
            }
        } finally { model.destroy(); if (!model.waitFor(5, TimeUnit.SECONDS)) model.destroyForcibly(); }
    }
}
