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
class ManagedRetirementHostTest {
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
        Backend(boolean enabled) throws Exception {
            String configured = System.getenv("ECHO_AGENT_SDK_RETIREMENT_RUNTIME_ROOT");
            Path root = (configured == null ? Path.of("../../target/managed-retirement") : Path.of(configured)).toAbsolutePath().normalize();
            Files.createDirectories(root);
            work = Files.createTempDirectory(root, "java-retirement-" + enabled + "-");
            var command = new ArrayList<>(List.of("python3", Path.of("../shared/testing/managed_persistence_backend.py").toAbsolutePath().toString(), "--work", work.toString()));
            if (enabled) command.add("--enable-retirement");
            process = new ProcessBuilder(command).redirectError(work.resolve("backend.stderr").toFile())
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
            case "runtime_retire_generation_with_context" -> new AgentComponentResult.RuntimeGenerationRetired(ManagedPersistence.decode(value.path("receipt"), RuntimeGenerationRetireReceipt.class));
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
    private static final String CLEAR = "echo_agent::state::clear_persisted_runtime_incarnation";
    private static final String DELETE = "echo_agent::state::delete_persisted_conversation";
    private static final String RETIRE = "runtime_retire_generation_with_context";
    private static JsonNode durable(JsonNode value) {
        var copy = (ObjectNode) value.deepCopy();
        copy.remove(List.of("observations", "supported_operations", "calls"));
        return copy;
    }
    private static EchoAgentException failure(CompletionStage<?> call) throws Exception {
        var wrapped = assertThrows(ExecutionException.class, () -> await(call));
        assertInstanceOf(EchoAgentException.class, wrapped.getCause());
        return (EchoAgentException) wrapped.getCause();
    }
    private static ObjectNode errorFact(EchoAgentException error) {
        var result = JsonSupport.MAPPER.createObjectNode().put("code", error.code()).put("operation", error.operation()).put("message", error.getMessage());
        result.set("details", error.details());
        return result;
    }
    @Test void unsupportedExactClearHasNoDurableMutation() throws Exception { scenario(false); }
    @Test void acceptedExactClearRetiresOnlyGenerationAndReplaysWithoutMutation() throws Exception { scenario(true); }
    private static void scenario(boolean enabled) throws Exception {
        try (var backend = new Backend(enabled); var sdk = backend.spawn()) {
            var proof = JsonSupport.MAPPER.createObjectNode().put("enabled", enabled);
            var errors = proof.putArray("errors");
            try {
                var registrations = backend.register(sdk);
                var agent = await(sdk.createAgent()); var session = await(agent.createSession());
                var run = await(session.startChat("managed retirement 中文🙂"));
                assertEquals("completed", settle(run).path("terminal").path("status").textValue());
                var before = backend.inspect(); assertEquals(1, before.path("runtime").size());
                var owner = before.path("runtime").fields().next(); var identity = owner.getKey(); var snapshot = owner.getValue();
                var scopeId = snapshot.path("scope").path("scope_id").textValue();
                assertFalse(snapshot.path("checkpoint").isNull()); assertFalse(hasPending(before));
                assertTrue(before.path("receipts").size() > 0); assertTrue(before.path("counters").path("projected_items").longValue() > 0);
                proof.set("before", before);
                var delete = failure(session.invoke(DELETE, List.of(scopeId)));
                assertEquals("framework_error", delete.code()); assertEquals(DELETE, delete.operation());
                assertTrue(delete.getMessage().contains("Managed runtime state requires compare-and-save"));
                assertTrue(delete.getMessage().contains("managed conversation deletion requires"));
                assertTrue(delete.details().path("fields").toString().contains("runtime_state"));
                errors.add(errorFact(delete));
                var afterDelete = backend.inspect(); assertEquals(durable(before), durable(afterDelete)); assertEquals(before.path("calls"), afterDelete.path("calls"));
                JsonNode after;
                if (!enabled) {
                    var unsupported = failure(session.invoke(CLEAR, List.of(scopeId, identity)));
                    assertEquals(CLEAR, unsupported.operation()); assertTrue(unsupported.getMessage().contains("does not implement " + RETIRE));
                    errors.add(errorFact(unsupported)); after = backend.inspect(); assertEquals(durable(before), durable(after));
                    assertFalse(after.path("supported_operations").toString().contains(RETIRE));
                } else {
                    var cleared = await(session.invoke(CLEAR, List.of(scopeId, identity)));
                    assertEquals(scopeId, cleared.path("scope_id").textValue()); assertEquals(identity, cleared.path("runtime_state_id").textValue()); assertTrue(cleared.path("checkpoint_removed").booleanValue());
                    after = backend.inspect(); var retired = after.path("runtime").path(identity);
                    assertTrue(retired.path("checkpoint").isNull()); assertEquals(identity, retired.path("runtime_state_id").textValue());
                    var expectedScope = (ObjectNode) snapshot.path("scope").deepCopy();
                    expectedScope.put("revision", new BigInteger(expectedScope.path("revision").textValue()).add(BigInteger.ONE).toString());
                    assertEquals(expectedScope, retired.path("scope")); assertEquals(expectedScope, after.path("scopes").path(scopeId)); assertEquals("active", expectedScope.path("lifecycle").textValue());
                    for (String field : List.of("conversations", "receipts", "batches")) assertEquals(before.path(field), after.path(field));
                    var counters = (ObjectNode) before.path("counters").deepCopy(); counters.put("mutations", counters.path("mutations").longValue() + 1); assertEquals(reparse(counters), after.path("counters"));
                    var request = after.path("cas_proofs").path(identity).path("request");
                    assertEquals(scopeId, request.path("scope_id").textValue()); assertEquals(identity, request.path("runtime_state_id").textValue());
                    assertEquals(snapshot.path("scope").path("revision"), request.path("expected_scope_revision")); assertEquals(snapshot.path("version"), request.path("expected_state_version"));
                    var version = JsonSupport.MAPPER.createObjectNode(); var tombstone = version.putObject("retired");
                    tombstone.put("revision", new BigInteger(snapshot.path("version").path("managed").path("revision").textValue()).add(BigInteger.ONE).toString()); tombstone.set("operation_id", request.path("operation_id"));
                    assertEquals(version, retired.path("version")); proof.set("clear_receipt", cleared);
                    var replayed = await(session.invoke(CLEAR, List.of(scopeId, identity))); var expected = (ObjectNode) cleared.deepCopy(); expected.put("checkpoint_removed", false);
                    assertEquals(expected, replayed); assertEquals(durable(after), durable(backend.inspect())); proof.set("replayed_clear_receipt", replayed);
                }
                var retirementCalls = new ArrayList<JsonNode>(); for (var call : after.path("calls")) if (RETIRE.equals(call.path("operation").textValue())) retirementCalls.add(call);
                assertEquals(1, retirementCalls.size()); var call = retirementCalls.get(0);
                assertEquals(identity, call.path("input").path("request").path("runtime_state_id").textValue());
                var deadline = call.path("input").path("context").path("absolute_deadline_unix_ms"); assertTrue(deadline.isTextual()); assertTrue(new BigInteger(deadline.textValue()).signum() > 0);
                proof.set("after", after);
                var previousGeneration = session.wire().generation();
                run.close(); session.close(); registrations.forEach(ExtensionRegistration::close); agent.close();
                if (enabled) {
                    await(sdk.closeAsync());
                    try (var restarted = backend.spawn()) {
                        var stores = backend.register(restarted); var nextAgent = await(restarted.createAgent());
                        var restored = session(restarted, nextAgent, scopeId, true);
                        assertNotEquals(previousGeneration, restored.wire().generation());
                        var expected = (ObjectNode) proof.path("clear_receipt").deepCopy(); expected.put("checkpoint_removed", false);
                        assertEquals(expected, await(restored.invoke(CLEAR, List.of(scopeId, identity))));
                        var recovered = backend.inspect(); assertEquals(durable(after), durable(recovered));
                        long retireCount = 0; for (var observed : recovered.path("calls")) if (RETIRE.equals(observed.path("operation").textValue())) retireCount++;
                        assertEquals(1, retireCount); proof.set("after_host_restart", recovered);
                        proof.putArray("host_generations").add(previousGeneration.toString()).add(restored.wire().generation().toString());
                        restored.close(); stores.forEach(ExtensionRegistration::close); nextAgent.close();
                    }
                }
            } finally {
                JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(backend.work.resolve("java-observed.json").toFile(), proof);
                System.out.println("Managed retirement evidence: " + backend.work);
            }
        }
    }
}
