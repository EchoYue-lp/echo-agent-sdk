package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import java.math.BigInteger;
import java.net.URI;
import java.net.http.*;
import java.nio.file.*;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.BooleanSupplier;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;
import static com.echoagent.sdk.ManagedPersistence.*;

/** The HTTP fixture persists backend transactions; real Host/framework owns recovery and Run settlement. */
class ManagedStoreHostTest {
    private static final HttpClient HTTP = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(5)).build();
    @BeforeAll static void requireHost() {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean present = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(present, "required Host E2E binary is missing");
        assumeTrue(present, "managed-store tests require real Host binary");
    }
    private static <T> T await(CompletionStage<T> value) throws Exception { return value.toCompletableFuture().get(45, TimeUnit.SECONDS); }
    private static void until(BooleanSupplier condition) throws Exception {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(35);
        while (!condition.getAsBoolean()) {
            if (System.nanoTime() > deadline) throw new TimeoutException("managed backend observation barrier not reached");
            java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(10));
        }
    }
    private static CompletionStage<JsonNode> post(String url, String route, JsonNode value) {
        try {
            var request = HttpRequest.newBuilder(URI.create(url + route)).timeout(Duration.ofSeconds(40))
                    .header("Content-Type", "application/json").POST(HttpRequest.BodyPublishers.ofString(JsonSupport.MAPPER.writeValueAsString(value))).build();
            return HTTP.sendAsync(request, HttpResponse.BodyHandlers.ofString()).thenApply(response -> {
                if (response.statusCode() != 200) throw new IllegalStateException("backend HTTP failure " + response.statusCode() + ": " + response.body());
                try { return JsonSupport.MAPPER.readTree(response.body()); }
                catch (java.io.IOException error) { throw new CompletionException(error); }
            });
        } catch (java.io.IOException error) { return CompletableFuture.failedFuture(error); }
    }
    private static JsonNode get(String url, String route) {
        try {
            var response = HTTP.send(HttpRequest.newBuilder(URI.create(url + route)).timeout(Duration.ofSeconds(5)).GET().build(), HttpResponse.BodyHandlers.ofString());
            if (response.statusCode() != 200) throw new IllegalStateException("backend inspect HTTP failure " + response.statusCode());
            return JsonSupport.MAPPER.readTree(response.body());
        } catch (java.io.IOException | InterruptedException error) { throw new CompletionException(error); }
    }
    private static final class Backend implements AutoCloseable {
        final Path work;
        final Process process;
        final String url;
        final CopyOnWriteArrayList<String> called = new CopyOnWriteArrayList<>();
        final CopyOnWriteArrayList<EchoAgentException> errors = new CopyOnWriteArrayList<>();
        final CopyOnWriteArrayList<JsonNode> returnedReceipts = new CopyOnWriteArrayList<>();
        Backend(String fault) throws Exception {
            work = Files.createTempDirectory("sdk-java-managed-");
            process = new ProcessBuilder("python3", Path.of("../shared/testing/managed_persistence_backend.py").toAbsolutePath().toString(),
                    "--work", work.toString(), "--fault", fault).redirectError(work.resolve("backend.stderr").toFile())
                    .redirectOutput(work.resolve("backend.stdout").toFile()).start();
            until(() -> Files.exists(work.resolve("ready")) || !process.isAlive());
            assertTrue(process.isAlive(), Files.readString(work.resolve("backend.stderr")));
            url = JsonSupport.MAPPER.readTree(work.resolve("backend.json").toFile()).path("url").textValue();
        }
        JsonNode inspect() { return get(url, "/inspect"); }
        EchoAgentClient spawn() throws Exception {
            return await(EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"), List.of("--config", work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                    Map.of("NO_PROXY", "127.0.0.1,localhost", "no_proxy", "127.0.0.1,localhost")));
        }
        List<ExtensionRegistration> register(EchoAgentClient sdk) throws Exception {
            AgentComponentHandler handler = (call, cancellation) -> {
                called.add(call.componentOperation());
                ObjectNode wire = JsonSupport.MAPPER.createObjectNode().put("operation", call.componentOperation());
                try { wire.set("input", requestFields(call.request())); }
                catch (ReflectiveOperationException error) { return CompletableFuture.failedFuture(error); }
                return post(url, "/call", wire).thenApply(response -> {
                    if ("error".equals(response.path("kind").textValue())) {
                        JsonNode error = response.path("error");
                        EchoAgentException typed = EchoAgentException.from(error, call.componentOperation());
                        errors.add(typed);
                        // Preserve the backend's typed error metadata, including
                        // persistence_error_class; successful values always use DTOs below.
                        return (ExtensionOutcome) new BackendFailure(typed);
                    }
                    if (!"result".equals(response.path("kind").textValue())) throw new IllegalArgumentException("unknown backend outcome");
                    JsonNode result = response.path("result");
                    if (!call.componentOperation().equals(result.path("operation").textValue())) throw new IllegalArgumentException("backend returned mismatched operation");
                    var fact = typedResult(call.componentOperation(), result.path("value"));
                    assertEquals(result.path("value"), reparse(fact.value()), "typed backend fact must survive complete field conversion");
                    if (result.path("value").has("receipt")) returnedReceipts.add(result.path("value").path("receipt").deepCopy());
                    return AgentComponentOutcome.result(call.component(), fact);
                });
            };
            return List.of(await(sdk.registerAgentComponent("java-managed-conversation", AgentComponentDescriptor.managedConversationStore("managed conversation"), handler)),
                    await(sdk.registerAgentComponent("java-managed-runtime", AgentComponentDescriptor.managedRuntimeStateStore("managed runtime"), handler)));
        }
        @Override public void close() {
            try { await(post(url, "/release", JsonSupport.MAPPER.createObjectNode())); }
            catch (Exception ignored) { /* Backend shutdown releases a held response even when its client is gone. */ }
            process.destroy();
            try { if (!process.waitFor(5, TimeUnit.SECONDS)) process.destroyForcibly(); }
            catch (InterruptedException error) { Thread.currentThread().interrupt(); process.destroyForcibly(); }
        }
    }
    private record BackendFailure(EchoAgentException error) implements ExtensionOutcome {
        @Override public JsonNode toJson() {
            var value = JsonSupport.MAPPER.createObjectNode().put("code", error.code()).put("message", error.getMessage()).put("retryable", error.retryable());
            if (error.operation() != null) value.put("operation", error.operation());
            if (error.handle() != null) value.set("handle", error.handle().toJson());
            if (error.details() != null) value.set("details", error.details().deepCopy());
            return JsonSupport.MAPPER.createObjectNode().put("outcome", "error").set("error", value);
        }
    }
    private static JsonNode reparse(JsonNode value) {
        try { return JsonSupport.MAPPER.readTree(value.toString()); }
        catch (java.io.IOException error) { throw new IllegalArgumentException(error); }
    }
    private static JsonNode requestFields(AgentComponentRequest request) throws ReflectiveOperationException {
        var fields = JsonSupport.MAPPER.createObjectNode();
        for (var component : request.getClass().getRecordComponents()) {
            String name = component.getName().replaceAll("([a-z0-9])([A-Z])", "$1_$2").toLowerCase(Locale.ROOT);
            Object value = component.getAccessor().invoke(request);
            if (value == null) fields.putNull(name);
            else if (value instanceof ManagedPersistence.Value fact) fields.set(name, fact.toJson());
            else if (value instanceof String text) fields.put(name, text);
            else if (value instanceof BigInteger number) fields.put(name, number.toString());
            else throw new IllegalArgumentException("unknown typed managed request field: " + name);
        }
        return fields;
    }
    private static <T extends ManagedPersistence.Value> T optional(JsonNode value, Class<T> type) {
        return value == null || value.isNull() ? null : ManagedPersistence.decode(value, type);
    }
    private static AgentComponentResult typedResult(String operation, JsonNode value) {
        return switch (operation) {
            case "conversation_ensure_projection_epoch_with_context" -> new AgentComponentResult.ConversationProjectionEpochEnsured(ManagedPersistence.decode(value.path("receipt"), ConversationProjectionEpochReceipt.class));
            case "conversation_get_projection_authority_with_context" -> new AgentComponentResult.ConversationProjectionAuthorityLoaded(optional(value.get("authority"), ConversationProjectionAuthority.class));
            case "conversation_apply_transcript_projection_with_context" -> new AgentComponentResult.ConversationProjectionApplied(ManagedPersistence.decode(value.path("receipt"), TranscriptProjectionApplyReceipt.class));
            case "runtime_load_managed_state_with_context" -> new AgentComponentResult.RuntimeManagedStateLoaded(optional(value.get("snapshot"), ManagedRuntimeStateSnapshot.class));
            case "runtime_load_scope_authority_with_context" -> new AgentComponentResult.RuntimeScopeAuthorityLoaded(optional(value.get("authority"), RuntimeScopeAuthority.class));
            case "runtime_compare_and_save_checkpoint_with_context" -> new AgentComponentResult.RuntimeCheckpointComparedAndSaved(ManagedPersistence.decode(value.path("receipt"), RuntimeCheckpointCasReceipt.class));
            case "runtime_acknowledge_transcript_projection_with_context" -> new AgentComponentResult.RuntimeTranscriptProjectionAcknowledged(ManagedPersistence.decode(value.path("receipt"), RuntimeCheckpointCasReceipt.class));
            // The real fixture rejects retirement/delete and TaskStore CAS. No test
            // invents successful receipts for operations the backend does not implement.
            default -> throw new IllegalArgumentException("unsupported backend result operation " + operation);
        };
    }
    private static SessionHandle session(EchoAgentClient sdk, AgentHandle agent, String stableId, boolean load) throws Exception {
        var request = JsonSupport.MAPPER.createObjectNode().put("session_id", stableId); request.set("agent", agent.wire().toJson()); request.putNull("working_dir");
        var response = await(sdk.request(load ? "_echo_agent/session/load" : "_echo_agent/session/create", request));
        return new SessionHandle(sdk, agent.wire(), WireHandle.fromJson(response.path("session")), response.path("acp_session_id").textValue(), WireHandle.fromJson(response.path("task_run")));
    }
    private static JsonNode settle(RunHandle run) throws Exception {
        var settled = await(run.waitForSettlement(Duration.ofSeconds(30))); assertTrue(settled.path("settled").booleanValue(), settled.toString()); return settled;
    }
    private static boolean hasPending(JsonNode state) {
        for (var snapshot : state.path("runtime")) {
            try { if (JsonSupport.MAPPER.readTree(snapshot.path("checkpoint").path("messages_json").textValue()).hasNonNull("pending_transcript_projection")) return true; }
            catch (java.io.IOException error) { throw new IllegalArgumentException(error); }
        }
        return false;
    }
    private static List<TranscriptProjectionSettlement> settlements(JsonNode replay) {
        var result = new ArrayList<TranscriptProjectionSettlement>();
        for (var event : replay.path("events")) {
            if ("transcript_projection_settlement".equals(event.path("payload").path("event_type").textValue())) {
                result.add(ManagedPersistence.decodeSettlementEvent(event.path("payload").path("data")));
            }
        }
        return result;
    }
    private static void unchangedReceiptAndUniqueProjection(JsonNode before, JsonNode after) {
        before.path("receipts").fields().forEachRemaining(item -> assertEquals(item.getValue(), after.path("receipts").path(item.getKey()), "recovery changed committed receipt"));
        long items = 0;
        for (var conversation : after.path("conversations")) {
            var ordinals = new HashSet<String>();
            for (var item : conversation.path("messages")) { assertTrue(ordinals.add(item.path("ordinal").textValue()), "duplicate durable projection ordinal"); items++; }
        }
        assertEquals(items, after.path("counters").path("projected_items").longValue());
    }
    private static void releaseHandles(List<ExtensionRegistration> registrations, RunHandle first, RunHandle second, SessionHandle session, AgentHandle agent) {
        if (first != null) first.close(); if (second != null) second.close(); session.close(); registrations.forEach(ExtensionRegistration::close); agent.close();
    }
    @Test void lostApplyAcknowledgementRecoversExactDurableBatchWithoutDuplicateProjection() throws Exception {
        try (var backend = new Backend("lose_first_apply_response"); var sdk = backend.spawn()) {
            var stores = backend.register(sdk); var agent = await(sdk.createAgent()); var session = session(sdk, agent, "java-managed-loss", false);
            var first = await(session.startChat("first managed 中文🙂"));
            var firstSettlement = settle(first);
            assertEquals("failed", firstSettlement.path("terminal").path("status").textValue(), firstSettlement.toString());
            assertEquals("transcript_projection_deferred",firstSettlement.path("terminal").path("failure").path("code").textValue());
            assertEquals("runtime_state",firstSettlement.path("terminal").path("failure").path("category").textValue());
            // Business completion and deferred persistence remain independent
            // framework-owned facts; retain the exact observed terminal as evidence.
            JsonSupport.MAPPER.writeValue(backend.work.resolve("java-lost-ack-terminal.json").toFile(), firstSettlement);
            var before = backend.inspect();
            assertEquals(1, before.path("observations").path("lost_responses").intValue());
            assertTrue(before.path("counters").path("projected_items").longValue() > 0);
            assertTrue(before.path("receipts").size() > 0); assertTrue(hasPending(before), "lost acknowledgement must retain durable pending intent");
            var observed = settlements(await(first.replay("0", 512)));
            assertTrue(observed.stream().anyMatch(value -> value.status() == TranscriptProjectionSettlementStatus.deferred
                    && value.errorClass() == TranscriptProjectionErrorClass.outcome_unknown), "lost receipt must yield typed Deferred/OutcomeUnknown");
            assertTrue(backend.errors.stream().anyMatch(error -> error.details() != null && error.details().path("fields").toString().contains("outcome_unknown")));
            var second = await(session.startChat("second managed 中文🙂"));
            assertEquals("completed", settle(second).path("terminal").path("status").textValue());
            var after = backend.inspect();
            assertTrue(after.path("counters").path("duplicate_applies").longValue() > 0, "same batch must return AlreadyApplied on recovery");
            unchangedReceiptAndUniqueProjection(before, after);
            assertTrue(backend.returnedReceipts.stream().anyMatch(receipt -> "already_applied".equals(receipt.path("status").textValue())));
            assertTrue(settlements(await(second.replay("0", 512))).stream().anyMatch(value -> value.status() == TranscriptProjectionSettlementStatus.settled));
            assertTrue(backend.called.containsAll(List.of("conversation_ensure_projection_epoch_with_context", "conversation_apply_transcript_projection_with_context", "runtime_compare_and_save_checkpoint_with_context", "runtime_acknowledge_transcript_projection_with_context")));
            releaseHandles(stores, first, second, session, agent);
        }
    }
    @Test void absoluteDeadlineExpiresBeforeAnyDurableMutationOrModelCall() throws Exception {
        try (var backend = new Backend("expire_before_work_once"); var sdk = backend.spawn()) {
            var stores = backend.register(sdk); var agent = await(sdk.createAgent()); var session = session(sdk, agent, "java-managed-expiry", false);
            var run = await(session.startChat("expired managed deadline"));
            var settled = settle(run); assertEquals("failed", settled.path("terminal").path("status").textValue(), settled.toString());
            until(() -> backend.inspect().path("observations").path("expired").intValue() == 1);
            var state = backend.inspect();
            assertEquals(0, state.path("counters").path("mutations").intValue());
            assertEquals(0, state.path("counters").path("projected_items").intValue());
            assertEquals(0, state.path("receipts").size()); assertEquals(0, state.path("runtime").size());
            assertEquals(0, state.path("observations").path("model_calls").intValue());
            assertEquals("failed", settled.path("receipt").path("outcome").textValue());
            releaseHandles(stores, run, null, session, agent);
        }
    }
    @Test void committedHeldResponseSurvivesRealTransportCloseAndHostRestart() throws Exception {
        try (var backend = new Backend("hold_first_apply_response")) {
            var beforeSdk = backend.spawn();
            JsonNode before;
            WireHandle oldSession;
            try {
                backend.register(beforeSdk); var agent = await(beforeSdk.createAgent()); var session = session(beforeSdk, agent, "java-managed-restart", false); oldSession = session.wire();
                await(session.startChat("commit then disconnect"));
                until(() -> backend.inspect().path("observations").path("held_responses").intValue() == 1);
                before = backend.inspect(); assertTrue(before.path("receipts").size() > 0); assertTrue(hasPending(before));
            } finally { await(beforeSdk.closeAsync()); }
            await(post(backend.url, "/release", JsonSupport.MAPPER.createObjectNode()));
            try (var sdk = backend.spawn()) {
                var stores = backend.register(sdk); var agent = await(sdk.createAgent()); var session = session(sdk, agent, "java-managed-restart", true);
                assertNotEquals(oldSession.generation(), session.wire().generation(), "Host restart must mint a new handle generation");
                var run = await(session.startChat("continue after restart"));
                assertEquals("completed", settle(run).path("terminal").path("status").textValue());
                var after = backend.inspect(); assertTrue(after.path("counters").path("duplicate_applies").longValue() > 0);
                unchangedReceiptAndUniqueProjection(before, after);
                assertFalse(hasPending(after), "recovered projection must reconcile the retained pending intent");
                releaseHandles(stores, run, null, session, agent);
            }
        }
    }
}
