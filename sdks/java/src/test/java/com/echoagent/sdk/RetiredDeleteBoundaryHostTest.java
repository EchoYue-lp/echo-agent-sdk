package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.function.Supplier;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** 只调用当前public API，真实退休后catalog上验证stage交付，不放宽原failure helper。 */
class RetiredDeleteBoundaryHostTest {
    private static final String DELETE = "echo_agent::state::delete_persisted_conversation";
    private static final String METHOD = "_echo_agent/facade/invoke";
    private static final String OLD_SIGNATURE = "sha256:97b7ff491c492fa6e3b26e6130c5f51285ff1f1608ae64d447de744152c501e2";
    private static final ObjectMapper JSON = new ObjectMapper();

    @BeforeAll static void requireHost() {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean present = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(present, "required retirement Host is missing");
        assumeTrue(present, "real Host required");
    }

    private static <T> T await(CompletionStage<T> stage) throws Exception {
        return stage.toCompletableFuture().get(10, TimeUnit.SECONDS);
    }
    private static EchoAgentException failedStage(Supplier<CompletionStage<JsonNode>> call) {
        CompletionStage<JsonNode> stage = assertDoesNotThrow(call::get, "public source invocation must return a stage before delivering refusal");
        assertNotNull(stage);
        ExecutionException wrapped = assertThrows(ExecutionException.class, () -> await(stage));
        return assertInstanceOf(EchoAgentException.class, wrapped.getCause());
    }
    private static void unavailable(EchoAgentException error) {
        assertEquals("feature_unavailable", error.code());
        assertEquals("never", error.retryable());
        assertEquals(DELETE, error.operation());
        assertTrue(error.getMessage().contains("unmanaged conversation deletion is retired"));
    }
    private record Fixture(EchoAgentClient client, AgentHandle agent, SessionHandle session, Path work) implements AutoCloseable {
        @Override public void close() { client.close(); }
    }
    private static Fixture fixture(String label) throws Exception {
        String configured = System.getenv("ECHO_AGENT_SDK_RETIRED_DELETE_RUNTIME_ROOT");
        Path root = configured == null ? Path.of("../../target/retired-delete-boundary").toAbsolutePath() : Path.of(configured);
        Files.createDirectories(root);
        Path work = Files.createTempDirectory(root, "java-" + label + "-");
        EchoAgentClient client = await(EchoAgentClient.spawn("python3", List.of(
                Path.of("../python/tests/protocol_host_proxy.py").toAbsolutePath().toString(),
                "--host", System.getenv("ECHO_AGENT_SDK_HOST"), "--work", work.toString(), "--mode", "normal"),
                Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()));
        try {
            assertTrue(client.catalog().operations().stream().noneMatch(item -> item.operation().equals(DELETE)), "requires actual retired catalog");
            AgentHandle agent = await(client.createAgent());
            SessionHandle session = await(agent.createSession());
            await(agent.describe());
            return new Fixture(client, agent, session, work);
        } catch (Exception | AssertionError error) {
            try { client.close(); }
            catch (RuntimeException cleanupError) { error.addSuppressed(cleanupError); }
            throw error;
        }
    }
    private static List<JsonNode> requests(Path work) throws Exception {
        var result = new ArrayList<JsonNode>();
        for (String line : Files.readAllLines(work.resolve("client-frames.jsonl"))) {
            if (line.isBlank()) continue;
            JsonNode frame = JSON.readTree(line);
            if (frame.has("id") && frame.path("method").isTextual()) result.add(frame);
        }
        return List.copyOf(result);
    }
    private static void barrierOnly(Fixture value, List<JsonNode> before, String label) throws Exception {
        await(value.agent().describe());
        List<JsonNode> after = requests(value.work());
        assertTrue(after.size() >= before.size());
        assertEquals(before, after.subList(0, before.size()), "original submitted frames must remain an unchanged prefix");
        List<JsonNode> delta = after.subList(before.size(), after.size());
        Files.writeString(value.work().resolve(label + "-submission-proof.json"), JSON.writerWithDefaultPrettyPrinter().writeValueAsString(delta));
        assertEquals(1, delta.size(), "local refusal must submit only the explicit observation barrier");
        assertEquals("_echo_agent/agent/describe", delta.get(0).path("method").asText());
    }

    @Test void invokeCallAndSessionReturnTypedFailedStagesWithoutSubmittedIds() throws Exception {
        try (Fixture value = fixture("local")) {
            List<JsonNode> before = requests(value.work());
            unavailable(failedStage(() -> value.client().invoke(DELETE, value.agent().wire(), List.of(value.session().wire(), "scope"))));
            unavailable(failedStage(() -> value.client().call(DELETE, value.agent().wire(), List.of(value.session().wire(), "scope"))));
            unavailable(failedStage(() -> value.session().invoke(DELETE, List.of("scope"))));
            barrierOnly(value, before, "local");
        }
    }

    @Test void metadataAndUnknownOperationsKeepSynchronousLookupErrors() throws Exception {
        try (Fixture value = fixture("metadata")) {
            List<JsonNode> before = requests(value.work());
            assertThrows(IllegalArgumentException.class, () -> value.client().catalog().signature(DELETE));
            assertThrows(IllegalArgumentException.class, () -> value.client().catalog().resolve(DELETE));
            for (String unknown : List.of(DELETE + "_unknown", "prefix::" + DELETE, DELETE.toUpperCase(java.util.Locale.ROOT))) {
                assertThrows(IllegalArgumentException.class, () -> value.client().invoke(unknown, value.agent().wire(), List.of()));
                assertThrows(IllegalArgumentException.class, () -> value.client().call(unknown, value.agent().wire(), List.of()));
                assertThrows(IllegalArgumentException.class, () -> value.session().invoke(unknown, List.of()));
            }
            barrierOnly(value, before, "unknown");
        }
    }

    @Test void closedSessionWinsBeforeRetiredAndUnknownLookup() throws Exception {
        try (Fixture value = fixture("closed-session")) {
            await(value.session().closeAsync());
            List<JsonNode> before = requests(value.work());
            for (String operation : List.of(DELETE, DELETE + "_unknown")) {
                EchoAgentException error = failedStage(() -> value.session().invoke(operation, List.of()));
                assertEquals("stale_handle", error.code()); assertEquals("never", error.retryable()); assertEquals(METHOD, error.operation());
            }
            barrierOnly(value, before, "closed-session");
        }
    }

    @Test void clientCloseStartedAndCompletedKeepHostExitedOnlyForExactDelete() throws Exception {
        try (Fixture value = fixture("closed-client")) {
            CompletionStage<Void> close = value.client().closeAsync();
            Files.writeString(value.work().resolve("close-state-at-probe.json"), "{\"close_stage_done\":" + close.toCompletableFuture().isDone() + "}\n");
            for (Supplier<CompletionStage<JsonNode>> call : List.<Supplier<CompletionStage<JsonNode>>>of(
                    () -> value.client().invoke(DELETE, value.agent().wire(), List.of()),
                    () -> value.client().call(DELETE, value.agent().wire(), List.of()),
                    () -> value.session().invoke(DELETE, List.of()))) {
                EchoAgentException error = failedStage(call);
                assertEquals("host_exited", error.code()); assertEquals(METHOD, error.operation());
            }
            await(close);
            List<JsonNode> before = requests(value.work());
            assertEquals("host_exited", failedStage(() -> value.client().invoke(DELETE, value.agent().wire(), List.of())).code());
            assertEquals("host_exited", failedStage(() -> value.client().call(DELETE, value.agent().wire(), List.of())).code());
            assertEquals("host_exited", failedStage(() -> value.session().invoke(DELETE, List.of())).code());
            assertThrows(IllegalArgumentException.class, () -> value.client().invoke(DELETE + "_unknown", value.agent().wire(), List.of()));
            assertThrows(IllegalArgumentException.class, () -> value.client().call(DELETE + "_unknown", value.agent().wire(), List.of()));
            assertThrows(IllegalArgumentException.class, () -> value.session().invoke(DELETE + "_unknown", List.of()));
            assertEquals(before, requests(value.work()), "no request after completed client close");
        }
    }

    @Test void directHostRetirementDoesNotReclassifyUnknownOrMalformedRequests() throws Exception {
        try (Fixture value = fixture("host")) {
            var params = JSON.createObjectNode().put("operation", DELETE).put("signature_digest", OLD_SIGNATURE).putNull("handle");
            params.putArray("arguments");
            unavailable(failedStage(() -> value.client().request(METHOD, params)));
            var unknown = params.deepCopy().put("operation", DELETE + "_unknown");
            EchoAgentException unknownError = failedStage(() -> value.client().request(METHOD, unknown));
            assertEquals("invalid_value", unknownError.code()); assertEquals("never", unknownError.retryable());
            var malformed = params.deepCopy().put("arguments", "not-an-array");
            assertEquals("invalid_request", failedStage(() -> value.client().request(METHOD, malformed)).code());
        }
    }
}
