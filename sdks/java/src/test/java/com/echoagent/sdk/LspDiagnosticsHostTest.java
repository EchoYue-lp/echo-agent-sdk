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

class LspDiagnosticsHostTest {
    @BeforeAll static void requireHost() {
        var host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertNotNull(host, "required LSP diagnostics Host missing");
        assumeTrue(host != null && !host.isBlank(), "real Host required");
    }
    private static JsonNode await(CompletionStage<JsonNode> stage) throws Exception { return stage.toCompletableFuture().get(3, TimeUnit.SECONDS); }
    private static EchoAgentException failure(CompletionStage<?> stage) {
        var thrown = assertThrows(java.util.concurrent.ExecutionException.class, () -> stage.toCompletableFuture().get(3, TimeUnit.SECONDS));
        var cause = thrown.getCause();
        while (cause instanceof java.util.concurrent.CompletionException && cause.getCause() != null) cause = cause.getCause();
        return assertInstanceOf(EchoAgentException.class, cause);
    }
    private record Resources(WireHandle manager, WireHandle client) {}
    private static Resources open(SessionHandle session, Path work) throws Exception {
        var manager = WireHandle.fromJson(await(session.invoke("lsp.manager.open_exact", List.of())));
        var config = JsonSupport.MAPPER.readValue(work.resolve("lsp.json").toFile(), new com.fasterxml.jackson.core.type.TypeReference<Map<String, Object>>() {});
        await(session.invoke("lsp.manager.load_config", List.of(manager, config)));
        await(session.invoke("lsp.manager.start_server", List.of(manager, "test")));
        var option = await(session.invoke("lsp.manager.get_client", List.of(manager, "test")));
        assertEquals("variant", option.path("kind").textValue());
        assertEquals("some", option.path("value").path("variant").textValue());
        for (var field : option.path("value").path("fields")) if ("client".equals(field.path("name").textValue())) return new Resources(manager, WireHandle.fromJson(field.path("value").path("value")));
        throw new AssertionError("missing typed derived LSP client");
    }
    @Test void diagnosticReplacementErrorIsolationAndRuntimeSettlement() throws Exception {
        var work = Files.createTempDirectory("sdk-java-lsp-diagnostics-");
        System.out.println("LSP diagnostics evidence " + work);
        var fixture = Path.of("../shared/testing/lsp_diagnostic_server.py").toAbsolutePath();
        var prepare = new ProcessBuilder("python3", fixture.toString(), "--prepare", "--work", work.toString()).inheritIO().start();
        assertEquals(0, prepare.waitFor());
        try (var sdk = EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"), List.of("--config", work.resolve("host.json").toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of(), List.of(), List.of()).toCompletableFuture().get(10, TimeUnit.SECONDS)) {
            var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var session = agent.createSession().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var resources = open(session, work);
            assertEquals(JsonSupport.MAPPER.readTree("{\"relatedInformation\":false}"), JsonSupport.MAPPER.readTree(work.resolve("advertisement.json").toFile()));
            var corpus = JsonSupport.MAPPER.readTree(work.resolve("diagnostic-corpus.json").toFile());
            var other = corpus.path(0).get("expected");
            var observed = JsonSupport.MAPPER.createArrayNode();
            for (var item : corpus) {
                await(session.invoke("lsp.client.hover", List.of(resources.client(), "file:///" + item.path("case").textValue(), Map.of("line", 0, "character", 3))));
                var entry = JsonSupport.MAPPER.createObjectNode().put("case", item.path("case").textValue());
                if (item.has("error")) {
                    var error = failure(session.invoke("lsp.client.diagnostics", List.of(resources.client(), item.path("uri").textValue())));
                    assertEquals(item.path("error").textValue(), error.code());
                    assertTrue(error.getMessage().contains("publishDiagnostics"));
                    entry.put("error", error.code()).put("message", error.getMessage()).put("retryable", error.retryable()).put("error_operation", error.operation());
                    entry.set("details", error.details() == null ? JsonSupport.MAPPER.nullNode() : error.details());
                    assertEquals(other, await(session.invoke("lsp.client.diagnostics", List.of(resources.client(), "file:///other.test"))));
                } else { var value = await(session.invoke("lsp.client.diagnostics", List.of(resources.client(), item.path("uri").textValue()))); assertEquals(item.get("expected"), value); entry.set("value", value); }
                observed.add(entry);
            }
            long pid = Long.parseLong(Files.readString(work.resolve("server.pid")));
            await(session.invoke("lsp.manager.shutdown_all", List.of(resources.manager())));
            assertFalse(ProcessHandle.of(pid).map(ProcessHandle::isAlive).orElse(false));
            for (var fatal : List.of("missing-uri", "wrong-uri", "missing-params")) {
                var next = open(session, work);
                pid = Long.parseLong(Files.readString(work.resolve("server.pid")));
                assertEquals("framework_error", failure(session.invoke("lsp.client.hover", List.of(next.client(), "file:///" + fatal, Map.of("line", 0, "character", 3)))).code());
                long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(3);
                JsonNode status;
                do { status = await(session.invoke("lsp.client.status", List.of(next.client()))); if (!status.path("running").booleanValue()) break; TimeUnit.MILLISECONDS.sleep(10); } while (System.nanoTime() < deadline);
                assertFalse(status.path("running").booleanValue()); assertFalse(status.path("initialized").booleanValue()); assertTrue(status.path("pid").isNull()); assertTrue(status.path("last_error").textValue().contains("publishDiagnostics"));
                assertFalse(ProcessHandle.of(pid).map(ProcessHandle::isAlive).orElse(false));
                var error = failure(session.invoke("lsp.client.diagnostics", List.of(next.client(), "file:///中文😀.test")));
                assertEquals("framework_error", error.code()); assertTrue(error.getMessage().contains("publishDiagnostics"));
                await(session.invoke("lsp.manager.shutdown_all", List.of(next.manager())));
                observed.add(JsonSupport.MAPPER.createObjectNode().put("case", fatal).put("pending_error", "framework_error").put("runtime_running", false).put("process_alive", false).put("diagnostics_error", "framework_error"));
            }
            Files.writeString(work.resolve("observed-diagnostics.json"), observed.toString());
            session.close(); agent.close();
        }
    }
}
