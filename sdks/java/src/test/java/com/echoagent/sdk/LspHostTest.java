package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.TimeUnit;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

class LspHostTest {
    @BeforeAll static void requireHost() {
        var host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertNotNull(host, "required Host LSP E2E binary missing");
        assumeTrue(host != null && !host.isBlank(), "real Host required");
    }
    private static JsonNode await(CompletionStage<JsonNode> stage) throws Exception { return stage.toCompletableFuture().get(3, TimeUnit.SECONDS); }
    private static EchoAgentException failure(CompletionStage<?> stage) {
        var thrown = assertThrows(java.util.concurrent.ExecutionException.class, () -> stage.toCompletableFuture().get(3, TimeUnit.SECONDS));
        var cause = thrown.getCause();
        while (cause instanceof java.util.concurrent.CompletionException && cause.getCause() != null) cause = cause.getCause();
        return assertInstanceOf(EchoAgentException.class, cause);
    }
    @Test void commonValuesFailuresAndManagerLifecycle() throws Exception {
        var work = Files.createTempDirectory("sdk-java-lsp-");
        System.out.println("LSP evidence " + work);
        var fixture = Path.of("../shared/testing/lsp_fault_server.py").toAbsolutePath();
        var prepare = new ProcessBuilder("python3", fixture.toString(), "--prepare", "--work", work.toString()).inheritIO().start();
        assertEquals(0, prepare.waitFor());
        try (var sdk = EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"), List.of("--config", work.resolve("host.json").toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of(), List.of(), List.of()).toCompletableFuture().get(10, TimeUnit.SECONDS)) {
            var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var session = agent.createSession().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var manager = WireHandle.fromJson(await(session.invoke("lsp.manager.open_exact", List.of())));
            await(session.invoke("lsp.manager.load_config", List.of(manager, JsonSupport.MAPPER.readValue(work.resolve("lsp.json").toFile(), new com.fasterxml.jackson.core.type.TypeReference<Map<String, Object>>() {}))));
            await(session.invoke("lsp.manager.start_server", List.of(manager, "test")));
            var option = await(session.invoke("lsp.manager.get_client", List.of(manager, "test")));
            assertEquals("variant", option.path("kind").textValue());
            assertEquals("some", option.path("value").path("variant").textValue());
            JsonNode clientNode = null;
            for (var field : option.path("value").path("fields")) if ("client".equals(field.path("name").textValue())) clientNode = field.path("value").path("value");
            assertNotNull(clientNode);
            var client = WireHandle.fromJson(clientNode);
            var observed = JsonSupport.MAPPER.createArrayNode();
            for (var item : JsonSupport.MAPPER.readTree(work.resolve("corpus.json").toFile())) {
                String operation = "lsp.client." + item.path("operation").textValue();
                var arguments = List.of(client, "file:///" + item.path("case").textValue(), Map.of("line", 0, "character", 3));
                var entry = JsonSupport.MAPPER.createObjectNode().put("operation", item.path("operation").textValue()).put("case", item.path("case").textValue());
                if (item.has("error")) {
                    var error = failure(session.invoke(operation, arguments));
                    assertEquals(item.path("error").textValue(), error.code());
                    assertTrue(error.getMessage().contains("Server returned error"));
                    assertEquals("never", error.retryable());
                    entry.put("error", error.code()).put("message", error.getMessage()).put("retryable", error.retryable()).put("error_operation", error.operation());
                    entry.set("details", error.details() == null ? JsonSupport.MAPPER.nullNode() : error.details());
                    assertEquals(JsonSupport.MAPPER.readTree("{\"contents\":\"中文😀\",\"range\":null}"), await(session.invoke("lsp.client.hover", List.of(client, "file:///hover-string", Map.of("line", 0, "character", 3)))));
                } else { var value = await(session.invoke(operation, arguments)); assertEquals(item.get("expected"), value); entry.set("value", value); }
                observed.add(entry);
            }
            long pid = Long.parseLong(Files.readString(work.resolve("server.pid")));
            assertTrue(ProcessHandle.of(pid).map(ProcessHandle::isAlive).orElse(false));
            await(session.invoke("lsp.manager.shutdown_all", List.of(manager)));
            assertFalse(ProcessHandle.of(pid).map(ProcessHandle::isAlive).orElse(false));
            var stale = failure(session.invoke("lsp.client.initialize", List.of(client, "file:///cannot-revive")));
            assertEquals("framework_error", stale.code());
            assertTrue(stale.getMessage().toLowerCase().contains("not initialized"));
            assertEquals(pid, Long.parseLong(Files.readString(work.resolve("server.pid"))));
            observed.add(JsonSupport.MAPPER.createObjectNode().put("case", "manager-close").put("process_alive_before", true).put("process_alive_after", false).put("stale_error", "framework_error").put("resurrected", false));
            Files.writeString(work.resolve("observed.json"), observed.toString());
            session.close(); agent.close();
        }
    }
}
