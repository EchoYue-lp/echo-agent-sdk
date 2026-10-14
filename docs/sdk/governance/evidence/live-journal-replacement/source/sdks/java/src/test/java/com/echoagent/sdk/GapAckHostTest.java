package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import org.junit.jupiter.params.provider.ValueSource;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.Flow;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.function.BooleanSupplier;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Same shared model barriers and raw Host assertions as TypeScript/Python. */
class GapAckHostTest {
    @ParameterizedTest
    @CsvSource({"window,false", "oversized,false", "window,true", "oversized,true"})
    void gapAckSnapshotContinuation(String mode, boolean consume) throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        assumeTrue(host != null && !host.isBlank(), "ECHO_AGENT_SDK_HOST is required for real Host E2E");
        Path work = Files.createTempDirectory("echo-sdk-gap-java-");
        Path testing = Path.of("../shared/testing").toAbsolutePath().normalize();
        Process model = new ProcessBuilder("python3", testing.resolve("gap_ack_fixture.py").toString(),
                "--work", work.toString(), "--mode", mode).inheritIO().start();
        try {
            waitFor(() -> Files.exists(work.resolve("ready")));
            try (EchoAgentClient sdk = EchoAgentClient.spawn("python3",
                    List.of(testing.resolve("gap_ack_host_proxy.py").toString(), host,
                            work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"),
                    Path.of("../shared/contract-digests.json"),
                    Map.of("NO_PROXY", "127.0.0.1,localhost", "no_proxy", "127.0.0.1,localhost",
                            "RUST_LOG", "error")).toCompletableFuture().get(10, TimeUnit.SECONDS)) {
                var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var session = agent.createSession().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var run = session.startChat("gap ACK test 中文🌊").toCompletableFuture().get(5, TimeUnit.SECONDS);
                waitFor(() -> Files.exists(work.resolve("parked")));
                List<JsonNode> before = notifications(work);
                JsonNode gap = before.stream().filter(value -> "_echo_agent/gap".equals(value.path("method").asText()))
                        .findFirst().orElseThrow();
                long watermark = Long.parseLong(gap.path("params").path("gap").path("snapshot_watermark").asText());
                assertEquals("window".equals(mode) ? 2 : 1, before.size());
                NextItem subscriber = null;
                if (consume) {
                    subscriber = new NextItem();
                    run.events().subscribe(subscriber);
                    JsonNode item = subscriber.next();
                    if ("window".equals(mode)) {
                        assertEquals("1", item.path("envelope").path("sequence").asText());
                        item = subscriber.next();
                    }
                    assertEquals(Long.toString(watermark), item.path("gap").path("snapshot_watermark").asText());
                }
                var futureAck = JsonSupport.MAPPER.createObjectNode();
                var futureCursor = JsonSupport.MAPPER.createObjectNode();
                futureCursor.set("stream", run.stream().toJson());
                futureCursor.put("last_processed_sequence", Long.toString(watermark + 1));
                futureAck.set("ack", futureCursor);
                sdk.notify("_echo_agent/event/ack", futureAck).toCompletableFuture().get(5, TimeUnit.SECONDS);
                JsonNode snapshot = run.get().toCompletableFuture().get(5, TimeUnit.SECONDS);
                assertTrue(Long.parseLong(snapshot.path("last_sequence").asText()) >= watermark);
                assertEquals(before.size(), notifications(work).size(), "future ACK must not resume delivery");
                JsonNode replay = run.replay(Long.toString(watermark), 64).toCompletableFuture().get(5, TimeUnit.SECONDS);
                assertFalse(replay.path("events").isEmpty());
                for (JsonNode event : replay.path("events")) {
                    assertTrue(Long.parseLong(event.path("sequence").asText()) > watermark);
                }
                var ack = JsonSupport.MAPPER.createObjectNode();
                var cursor = JsonSupport.MAPPER.createObjectNode();
                cursor.set("stream", run.stream().toJson());
                cursor.put("last_processed_sequence", Long.toString(watermark));
                ack.set("ack", cursor);
                sdk.notify("_echo_agent/event/ack", ack).toCompletableFuture().get(5, TimeUnit.SECONDS);
                if (subscriber != null) {
                    JsonNode resumed = subscriber.next();
                    String sequence = resumed.has("gap") ? resumed.path("gap").path("from_sequence").asText()
                            : resumed.path("envelope").path("sequence").asText();
                    assertTrue(Long.parseLong(sequence) > watermark);
                }
                waitFor(() -> {
                    try { return notifications(work).size() > before.size(); }
                    catch (Exception error) { throw new IllegalStateException(error); }
                });
                List<JsonNode> after = notifications(work);
                assertAfterWatermark(after.subList(before.size(), after.size()), watermark);
                int announced = notifications(work).size();
                sdk.notify("_echo_agent/event/ack", ack).toCompletableFuture().get(5, TimeUnit.SECONDS);
                run.get().toCompletableFuture().get(5, TimeUnit.SECONDS);
                assertEquals(announced, notifications(work).size(), "late ACK cannot retract the next gap");
                Files.createFile(work.resolve("release"));
                assertTrue(run.waitForSettlement().toCompletableFuture().get(5, TimeUnit.SECONDS).path("settled").asBoolean());
                assertEquals("completed", run.get().toCompletableFuture().get(5, TimeUnit.SECONDS).path("status").asText());
                JsonNode continuation = run.replay(replay.path("next_cursor").path("last_processed_sequence").asText(), 64)
                        .toCompletableFuture().get(5, TimeUnit.SECONDS);
                assertFalse(continuation.path("events").isEmpty());
                for (JsonNode event : continuation.path("events")) {
                    assertTrue(Long.parseLong(event.path("sequence").asText()) > watermark);
                }
                List<JsonNode> finalNotifications = notifications(work);
                assertAfterWatermark(finalNotifications.subList(before.size(), finalNotifications.size()), watermark);
                if (subscriber != null) subscriber.subscription.cancel();
                session.close();
                agent.close();
            }
        } finally {
            model.destroy();
            assertTrue(model.waitFor(5, TimeUnit.SECONDS), "fixture must exit");
            try (var files = Files.walk(work)) {
                for (Path file : files.sorted(Comparator.reverseOrder()).toList()) Files.delete(file);
            }
        }
    }


    @ParameterizedTest
    @ValueSource(ints = {1, 64})
    void replayConsumedCursorAck(int pageSize) throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        assumeTrue(host != null && !host.isBlank(), "ECHO_AGENT_SDK_HOST is required for real Host E2E");
        Path work = Files.createTempDirectory("echo-sdk-page-java-");
        Path testing = Path.of("../shared/testing").toAbsolutePath().normalize();
        Process model = new ProcessBuilder("python3", testing.resolve("gap_ack_fixture.py").toString(),
                "--work", work.toString(), "--mode", "window").inheritIO().start();
        try {
            waitFor(() -> Files.exists(work.resolve("ready")));
            try (EchoAgentClient sdk = EchoAgentClient.spawn("python3",
                    List.of(testing.resolve("gap_ack_host_proxy.py").toString(), host, work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                    Map.of("NO_PROXY", "127.0.0.1,localhost", "no_proxy", "127.0.0.1,localhost", "RUST_LOG", "error"))
                    .toCompletableFuture().get(10, TimeUnit.SECONDS)) {
                var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var session = agent.createSession().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var run = session.startChat("replay consumed cursor 中文🌊").toCompletableFuture().get(5, TimeUnit.SECONDS);
                waitFor(() -> Files.exists(work.resolve("parked")));
                List<JsonNode> before = notifications(work);
                JsonNode gap = before.stream().filter(value -> "_echo_agent/gap".equals(value.path("method").asText())).findFirst().orElseThrow();
                long parkedTail = Long.parseLong(run.get().toCompletableFuture().get(5, TimeUnit.SECONDS).path("last_sequence").asText());
                long cursor = 0;
                var consumed = new java.util.LinkedHashMap<Long, String>();
                int pages = 0;
                while (cursor < parkedTail) {
                    JsonNode page = run.replay(Long.toString(cursor), pageSize).toCompletableFuture().get(5, TimeUnit.SECONDS);
                    assertFalse(page.path("events").isEmpty());
                    for (JsonNode event : page.path("events")) {
                        assertEquals(cursor + 1, Long.parseLong(event.path("sequence").asText()));
                        cursor = Long.parseLong(event.path("sequence").asText());
                        consumed.put(cursor, event.path("event_id").asText());
                    }
                    assertEquals(Long.toString(cursor), page.path("next_cursor").path("last_processed_sequence").asText());
                    pages++;
                }
                assertTrue(cursor > Long.parseLong(gap.path("params").path("gap").path("snapshot_watermark").asText()));
                assertEquals(pageSize == 1, pages > 1);
                JsonNode empty = run.replay(Long.toString(parkedTail + 100), pageSize).toCompletableFuture().get(5, TimeUnit.SECONDS);
                assertTrue(empty.path("events").isEmpty());
                acknowledge(sdk, run, empty.path("next_cursor").path("last_processed_sequence").asText());
                run.get().toCompletableFuture().get(5, TimeUnit.SECONDS);
                assertEquals(before.size(), notifications(work).size());
                acknowledge(sdk, run, Long.toString(cursor));
                run.get().toCompletableFuture().get(5, TimeUnit.SECONDS);
                assertEquals(before.size(), notifications(work).size(), "consumed replay pages must not be sent live again");
                Files.createFile(work.resolve("release"));
                waitFor(() -> {
                    try { return notifications(work).size() > before.size(); }
                    catch (Exception error) { throw new IllegalStateException(error); }
                });
                List<JsonNode> live = notifications(work);
                assertAfterWatermark(live.subList(before.size(), live.size()), cursor);
                assertTrue(run.waitForSettlement().toCompletableFuture().get(5, TimeUnit.SECONDS).path("settled").asBoolean());
                long terminalTail = Long.parseLong(run.get().toCompletableFuture().get(5, TimeUnit.SECONDS).path("last_sequence").asText());
                while (cursor < terminalTail) {
                    JsonNode page = run.replay(Long.toString(cursor), pageSize).toCompletableFuture().get(5, TimeUnit.SECONDS);
                    assertFalse(page.path("events").isEmpty());
                    for (JsonNode event : page.path("events")) {
                        assertEquals(cursor + 1, Long.parseLong(event.path("sequence").asText()));
                        cursor = Long.parseLong(event.path("sequence").asText());
                        assertFalse(consumed.containsKey(cursor));
                        consumed.put(cursor, event.path("event_id").asText());
                    }
                }
                acknowledge(sdk, run, Long.toString(cursor));
                assertEquals(terminalTail, consumed.size());
                JsonNode all = run.replay("0", 64).toCompletableFuture().get(5, TimeUnit.SECONDS);
                int terminalCount = 0;
                List<String> ids = new ArrayList<>();
                for (JsonNode event : all.path("events")) {
                    ids.add(event.path("event_id").asText());
                    if ("final_answer".equals(event.path("payload").path("event_type").asText())) terminalCount++;
                }
                assertEquals(1, terminalCount);
                assertEquals(new ArrayList<>(consumed.values()), ids);
                session.close();
                agent.close();
            }
        } finally {
            model.destroy();
            assertTrue(model.waitFor(5, TimeUnit.SECONDS));
            try (var files = Files.walk(work)) {
                for (Path file : files.sorted(Comparator.reverseOrder()).toList()) Files.delete(file);
            }
        }
    }

    /** Delegates every real ACP byte; only ACK completion is held past peer continuation. */
    private static final class GatedTransport implements com.agentclientprotocol.sdk.spec.AcpClientTransport {
        final com.agentclientprotocol.sdk.spec.AcpClientTransport nativeTransport;
        final java.util.concurrent.CompletableFuture<Void> entered = new java.util.concurrent.CompletableFuture<>();
        final java.util.concurrent.CompletableFuture<Void> completion = new java.util.concurrent.CompletableFuture<>();
        GatedTransport(com.agentclientprotocol.sdk.spec.AcpClientTransport nativeTransport) { this.nativeTransport = nativeTransport; }
        @Override public reactor.core.publisher.Mono<Void> connect(java.util.function.Function<reactor.core.publisher.Mono<com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCMessage>, reactor.core.publisher.Mono<com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCMessage>> handler) { return nativeTransport.connect(handler); }
        @Override public reactor.core.publisher.Mono<Void> sendMessage(com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCMessage message) {
            var nativeWrite = nativeTransport.sendMessage(message);
            if (message instanceof com.agentclientprotocol.sdk.spec.AcpSchema.JSONRPCNotification notification
                    && "_echo_agent/event/ack".equals(notification.method())) {
                return nativeWrite.then(reactor.core.publisher.Mono.defer(() -> {
                    entered.complete(null);
                    return reactor.core.publisher.Mono.fromFuture(completion, true);
                }));
            }
            return nativeWrite;
        }
        @Override public <T> T unmarshalFrom(Object value, com.agentclientprotocol.sdk.json.TypeRef<T> type) { return nativeTransport.unmarshalFrom(value, type); }
        @Override public reactor.core.publisher.Mono<Void> closeGracefully() { return nativeTransport.closeGracefully(); }
    }

    @ParameterizedTest
    @ValueSource(booleans = {false, true})
    void realHostGapBeforeOfficialAckCompletion(boolean success) throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        assumeTrue(host != null && !host.isBlank(), "ECHO_AGENT_SDK_HOST is required");
        Path work = Files.createTempDirectory("echo-sdk-pending-gap-java-");
        Path testing = Path.of("../shared/testing").toAbsolutePath().normalize();
        Process model = new ProcessBuilder("python3", testing.resolve("gap_ack_fixture.py").toString(),
                "--work", work.toString(), "--mode", "oversized").inheritIO().start();
        try {
            waitFor(() -> Files.exists(work.resolve("ready")));
            try (EchoAgentClient sdk = EchoAgentClient.spawn("python3",
                    List.of(testing.resolve("gap_ack_host_proxy.py").toString(), host, work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()).toCompletableFuture().get(5, TimeUnit.SECONDS)) {
                var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var session = agent.createSession().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var run = session.startRun("pending ACK real continuation 中文🌊", false).toCompletableFuture().get(5, TimeUnit.SECONDS);
                waitFor(() -> Files.exists(work.resolve("parked")));
                var page = run.replay("0", 64).toCompletableFuture().get(5, TimeUnit.SECONDS);
                long cursor = Long.parseLong(page.path("next_cursor").path("last_processed_sequence").asText());
                var feedField = EchoAgentClient.class.getDeclaredField("eventPublishers"); feedField.setAccessible(true);
                @SuppressWarnings("unchecked") var feeds = (Map<String, BoundedPublisher>) feedField.get(sdk);
                var feed = feeds.get(run.stream().id()); assertTrue(cursor > feed.lastSequence().longValueExact());
                int before = notifications(work).size();
                var sessionField = EchoAgentClient.class.getDeclaredField("session"); sessionField.setAccessible(true);
                var officialSession = (com.agentclientprotocol.sdk.spec.AcpClientSession) sessionField.get(sdk);
                var transportField = com.agentclientprotocol.sdk.spec.AcpClientSession.class.getDeclaredField("transport"); transportField.setAccessible(true);
                var nativeTransport = (com.agentclientprotocol.sdk.spec.AcpClientTransport) transportField.get(officialSession);
                var gate = new GatedTransport(nativeTransport); transportField.set(officialSession, gate);
                var params = JsonSupport.MAPPER.createObjectNode(); params.putObject("ack").put("last_processed_sequence", Long.toString(cursor)).set("stream", run.stream().toJson());
                var pending = sdk.notify("_echo_agent/event/ack", params);
                try {
                    gate.entered.get(5, TimeUnit.SECONDS); assertFalse(feed.isClosed());
                    Files.createFile(work.resolve("release"));
                    waitFor(() -> { try { return notifications(work).size() > before; } catch (Exception error) { throw new IllegalStateException(error); } });
                    var nextGap = notifications(work).get(before).path("params").path("gap");
                    assertEquals(cursor + 1, Long.parseLong(nextGap.path("from_sequence").asText()));
                    long watermark = Long.parseLong(nextGap.path("snapshot_watermark").asText());
                    waitFor(() -> feed.lastSequence().equals(java.math.BigInteger.valueOf(watermark)));
                    assertFalse(feed.isClosed());
                    if (success) { gate.completion.complete(null); pending.toCompletableFuture().get(5, TimeUnit.SECONDS); }
                    else { gate.completion.completeExceptionally(new IllegalStateException("outcome unknown after real peer gap")); org.junit.jupiter.api.Assertions.assertThrows(java.util.concurrent.ExecutionException.class, () -> pending.toCompletableFuture().get(5, TimeUnit.SECONDS)); }
                    transportField.set(officialSession, nativeTransport);
                    var subscriber = new NextItem(); run.events().subscribe(subscriber); var item = subscriber.next();
                    assertEquals(success ? cursor + 1 : 1, Long.parseLong(item.path("gap").path("from_sequence").asText()));
                    assertTrue(Long.parseLong(item.path("gap").path("snapshot_watermark").asText()) > cursor);
                    assertFalse(feed.isClosed()); assertTrue(run.waitForSettlement().toCompletableFuture().get(5, TimeUnit.SECONDS).path("settled").asBoolean());
                    String evidence = System.getenv("ECHO_AGENT_SDK_RECOVERY_EVIDENCE");
                    if (evidence != null) {
                        var record = JsonSupport.MAPPER.createObjectNode().put("success", success).put("ack_cursor", Long.toString(cursor));
                        record.set("replay", page); record.set("continued_gap", nextGap); record.set("delivered", item);
                        record.set("notifications", JsonSupport.MAPPER.valueToTree(notifications(work)));
                        Files.writeString(Path.of(evidence).resolve("java-pending-gap-" + success + ".json"), record.toPrettyString());
                    }
                } finally { transportField.set(officialSession, nativeTransport); gate.completion.complete(null); }
                session.close(); agent.close();
            }
        } finally {
            model.destroy(); assertTrue(model.waitFor(5, TimeUnit.SECONDS));
            try (var files = Files.walk(work)) { for (Path file : files.sorted(Comparator.reverseOrder()).toList()) Files.delete(file); }
        }
    }

    private static void acknowledge(EchoAgentClient sdk, RunHandle run, String sequence) throws Exception {
        var ack = JsonSupport.MAPPER.createObjectNode();
        var cursor = JsonSupport.MAPPER.createObjectNode();
        cursor.set("stream", run.stream().toJson());
        cursor.put("last_processed_sequence", sequence);
        ack.set("ack", cursor);
        sdk.notify("_echo_agent/event/ack", ack).toCompletableFuture().get(5, TimeUnit.SECONDS);
    }

    private static final class NextItem implements Flow.Subscriber<JsonNode> {
        private final ArrayBlockingQueue<Object> items = new ArrayBlockingQueue<>(16);
        private volatile Flow.Subscription subscription;
        private final java.util.concurrent.CountDownLatch subscribed = new java.util.concurrent.CountDownLatch(1);
        @Override public void onSubscribe(Flow.Subscription value) { subscription = value; subscribed.countDown(); }
        @Override public void onNext(JsonNode value) { items.add(value); }
        @Override public void onError(Throwable error) { items.add(error); }
        @Override public void onComplete() { items.add("completed"); }
        JsonNode next() throws Exception {
            assertTrue(subscribed.await(5, TimeUnit.SECONDS));
            subscription.request(1);
            Object value = items.poll(5, TimeUnit.SECONDS);
            if (value instanceof Throwable error) throw new IllegalStateException("event consumer failed", error);
            assertTrue(value instanceof JsonNode, "consumer must progress before its timeout");
            return (JsonNode) value;
        }
    }

    private static List<JsonNode> notifications(Path work) throws Exception {
        String text = Files.readString(work.resolve("notifications.jsonl"));
        int completeEnd = text.lastIndexOf('\n');
        List<JsonNode> notifications = new ArrayList<>();
        if (completeEnd < 0) return notifications;
        for (String line : text.substring(0, completeEnd).split("\n")) {
            if (line.isEmpty()) continue;
            JsonNode value = JsonSupport.MAPPER.readTree(line);
            if (List.of("_echo_agent/event", "_echo_agent/gap").contains(value.path("method").asText())) notifications.add(value);
        }
        return notifications;
    }

    @org.junit.jupiter.api.Test
    void liveJournalDirectoryReplacementReturnsTypedErrorAndIsolatesOtherSession() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        assumeTrue(host != null && !host.isBlank(), "real Host required");
        String evidence = System.getenv("ECHO_AGENT_SDK_JOURNAL_EVIDENCE");
        Path work = evidence == null ? Files.createTempDirectory("sdk-journal-java-") : Files.createTempDirectory(Path.of(evidence), "java-runtime-");
        Path fixture = Path.of("../shared/testing/gap_ack_fixture.py").toAbsolutePath();
        Process model = new ProcessBuilder("python3", fixture.toString(), "--work", work.toString(), "--mode", "window").redirectError(work.resolve("model.stderr").toFile()).start();
        RunHandle run = null;
        boolean replaced = false;
        try {
            waitFor(() -> Files.exists(work.resolve("ready")));
            try (var sdk = EchoAgentClient.spawn("python3", List.of(Path.of("../shared/testing/gap_ack_host_proxy.py").toAbsolutePath().toString(), host, work.resolve("host.json").toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of("NO_PROXY", "127.0.0.1,localhost", "no_proxy", "127.0.0.1,localhost")).toCompletableFuture().get(10, TimeUnit.SECONDS)) {
                var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var session = agent.createSession().toCompletableFuture().get(5, TimeUnit.SECONDS);
                run = session.startChat("original journal 中文🌊").toCompletableFuture().get(5, TimeUnit.SECONDS);
                waitFor(() -> Files.exists(work.resolve("parked"))); Files.createFile(work.resolve("release"));
                var settled = run.waitForSettlement().toCompletableFuture().get(5, TimeUnit.SECONDS); assertTrue(settled.path("settled").booleanValue()); assertEquals("completed", settled.path("terminal").path("status").textValue());
                var before = run.replay("0", 64).toCompletableFuture().get(5, TimeUnit.SECONDS); var snapshot = run.get().toCompletableFuture().get(5, TimeUnit.SECONDS);
                assertFalse(before.path("events").isEmpty()); assertTrue(snapshot.has("receipt"));
                var otherAgent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS); var otherSession = otherAgent.createSession().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var other = otherSession.startChat("independent journal 中文🌊").toCompletableFuture().get(5, TimeUnit.SECONDS); assertTrue(other.waitForSettlement().toCompletableFuture().get(5, TimeUnit.SECONDS).path("settled").booleanValue());
                var otherBefore = other.replay("0", 64).toCompletableFuture().get(5, TimeUnit.SECONDS); assertNotSameStream(run.stream(), other.stream()); assertFalse(otherBefore.path("events").isEmpty());
                var fact = journalFixture(work, "--journal-action", "replace", "--stream-id", run.stream().id()); replaced = true;
                var errors = JsonSupport.MAPPER.createArrayNode(); JsonNode restored;
                try {
                    for (int count = 0; count < 2; count++) {
                        var future = run.replay("0", 64).toCompletableFuture();
                        var failure = org.junit.jupiter.api.Assertions.assertThrows(java.util.concurrent.ExecutionException.class, () -> future.get(5, TimeUnit.SECONDS));
                        var error = org.junit.jupiter.api.Assertions.assertInstanceOf(EchoAgentException.class, failure.getCause());
                        assertEquals("replay_unavailable", error.code()); assertEquals("after_delay", error.retryable()); assertEquals("_echo_agent/run/replay", error.operation()); assertTrue(error.getMessage().contains("directory identity changed")); org.junit.jupiter.api.Assertions.assertNull(error.details()); org.junit.jupiter.api.Assertions.assertNull(error.handle());
                        var row = JsonSupport.MAPPER.createObjectNode().put("code", error.code()).put("retryable", error.retryable()).put("operation", error.operation()).put("message", error.getMessage());
                        row.set("details", error.details() == null ? JsonSupport.MAPPER.nullNode() : error.details()); row.set("handle", error.handle() == null ? JsonSupport.MAPPER.nullNode() : error.handle().toJson()); errors.add(row);
                    }
                    assertEquals(snapshot, run.get().toCompletableFuture().get(5, TimeUnit.SECONDS), "failed replay cannot replace terminal/receipt");
                    sameJournalPage(work, "other", otherBefore, other.replay("0", 64).toCompletableFuture().get(5, TimeUnit.SECONDS));
                } finally { restored = journalFixture(work, "--journal-action", "restore", "--stream-id", run.stream().id()); replaced = false; }
                sameJournalPage(work, "restored", before, run.replay("0", 64).toCompletableFuture().get(5, TimeUnit.SECONDS));
                if (evidence != null) {
                    var proof = JsonSupport.MAPPER.createObjectNode().put("receipt_unchanged", true).put("other_journal_unchanged", true).put("replay_restored", true);
                    proof.set("stream", run.stream().toJson()); proof.set("other_stream", other.stream().toJson()); proof.set("errors", errors); proof.set("directory", fact); proof.set("page", before);
                    proof.set("restored", restored);
                    Files.writeString(Path.of(evidence).resolve("java.json"), proof.toPrettyString());
                }
                otherSession.close(); otherAgent.close(); session.close(); agent.close();
            }
        } finally {
            if (replaced && run != null) journalFixture(work, "--journal-action", "restore", "--stream-id", run.stream().id());
            model.destroy(); assertTrue(model.waitFor(5, TimeUnit.SECONDS));
        }
    }

    private static void assertNotSameStream(WireHandle one, WireHandle two) { org.junit.jupiter.api.Assertions.assertNotEquals(one, two); }
    private static JsonNode journalFixture(Path work, String... arguments) throws Exception {
        var command = new ArrayList<>(List.of("python3", Path.of("../shared/testing/gap_ack_fixture.py").toAbsolutePath().toString(), "--work", work.toString(), "--mode", "window")); command.addAll(List.of(arguments));
        Path output = Files.createTempFile(work, "action-", ".json"); Path error = Files.createTempFile(work, "action-", ".stderr");
        var child = new ProcessBuilder(command).redirectOutput(output.toFile()).redirectError(error.toFile()).start();
        try { assertTrue(child.waitFor(5, TimeUnit.SECONDS)); assertEquals(0, child.exitValue(), Files.readString(error)); return JsonSupport.MAPPER.readTree(Files.readString(output)); }
        finally { if (child.isAlive()) { child.destroyForcibly(); assertTrue(child.waitFor(5, TimeUnit.SECONDS)); } }
    }
    private static void sameJournalPage(Path work, String name, JsonNode before, JsonNode after) throws Exception {
        Path first = work.resolve(name + "-before.json"), second = work.resolve(name + "-after.json");
        Files.writeString(first, before.toString()); Files.writeString(second, after.toString());
        assertTrue(journalFixture(work, "--compare-pages", first.toString(), second.toString()).path("same_complete_page").booleanValue());
    }

    private static void assertAfterWatermark(List<JsonNode> notifications, long watermark) {
        for (JsonNode value : notifications) {
            JsonNode params = value.path("params");
            String sequence = params.has("gap") ? params.path("gap").path("from_sequence").asText()
                    : params.path("envelope").path("sequence").asText();
            assertTrue(Long.parseLong(sequence) > watermark,
                    "Host must never resend snapshot-covered events or gaps, even if the SDK deduplicates them");
        }
    }

    private static void waitFor(BooleanSupplier predicate) throws Exception {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
        while (!predicate.getAsBoolean()) {
            assertTrue(System.nanoTime() < deadline, "real Host scenario did not reach its barrier");
            Thread.sleep(10);
        }
    }
}
