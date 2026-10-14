package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.util.HexFormat;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

class TelemetryHostTest {
    private static <T> T await(CompletionStage<T> stage) throws Exception {
        return stage.toCompletableFuture().get(15, TimeUnit.SECONDS);
    }

    @Test void acceptedTelemetryInitializesThroughRealHost() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank());
        assumeTrue(host != null && !host.isBlank(), "real Host required");
        Path root = Path.of(System.getenv().getOrDefault("ECHO_AGENT_SDK_TELEMETRY_ROOT", "../../target/telemetry-host")).toAbsolutePath();
        Files.createDirectories(root);
        Path work = Files.createTempDirectory(root, "java-");
        var prepare = new ProcessBuilder("python3", Path.of("../shared/testing/telemetry_host_fixture.py").toAbsolutePath().toString(), "--work", work.toString()).inheritIO().start();
        assertEquals(0, prepare.waitFor());
        JsonNode scenario = JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());
        ObjectNode facts = JsonSupport.MAPPER.createObjectNode().put("work", work.toString()).put("host", host)
                .put("host_sha256", HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256").digest(Files.readAllBytes(Path.of(host)))));
        facts.set("scenario", scenario);
        try (var sdk = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()),
                Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()))) {
            var agent = await(sdk.createAgent()); var session = await(agent.createSession());
            facts.set("before", await(session.invoke("telemetry.status", List.of())));
            JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(), facts);
            try { facts.set("init", session.invoke("telemetry.init", List.of(scenario.path("endpoint").textValue(), scenario.path("service_name").textValue(), false)).toCompletableFuture().get(2, TimeUnit.SECONDS)); }
            catch (ExecutionException exception) {
                assertInstanceOf(EchoAgentException.class, exception.getCause());
                var error = (EchoAgentException) exception.getCause();
                var failure = facts.putObject("error").put("code", error.code()).put("message", error.getMessage()).put("operation", error.operation()).put("retryable", error.retryable());
                failure.set("details", error.details()); failure.set("handle", error.handle() == null ? null : error.handle().toJson());
            }
            catch (java.util.concurrent.TimeoutException exception) {
                facts.putObject("error").put("code", "observation_timeout").put("message", "accepted telemetry.init did not finish within observation window");
            }
            if (!"observation_timeout".equals(facts.path("error").path("code").asText())) facts.set("after", await(session.invoke("telemetry.status", List.of())));
            else facts.put("after_unavailable", "initialization request still pending; status was not queried through the blocked dispatcher");
            JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(), facts);
            if (!facts.has("error")) { session.close(); agent.close(); }
            assertFalse(facts.has("error"), "accepted initialization failed: " + facts.path("error"));
            assertTrue(facts.path("init").path("initialized").booleanValue());
            assertTrue(facts.path("after").path("initialized").booleanValue());
        } finally {
            JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(), facts);
            System.out.println("Telemetry evidence: " + work);
        }
    }
}
