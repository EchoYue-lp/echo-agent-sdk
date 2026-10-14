package com.echoagent.sdk;

import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.time.Duration;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Same real collector/model scenario used by TypeScript and Python. */
class TelemetryHostTest {
    static final Path FIXTURE = Path.of("../shared/testing/telemetry_host_fixture.py").toAbsolutePath().normalize();
    static <T> T await(CompletionStage<T> stage) throws Exception {
        return stage.toCompletableFuture().get(20, TimeUnit.SECONDS);
    }
    static String helper(Path work, String... args) throws Exception {
        var command = new ArrayList<>(List.of("python3", FIXTURE.toString(), "--work", work.toString()));
        command.addAll(List.of(args));
        var process = new ProcessBuilder(command).redirectErrorStream(true).start();
        var output = new String(process.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
        assertEquals(0, process.waitFor(), output);
        return output;
    }
    static void failure(ObjectNode errors, String name, CompletionStage<?> operation) throws Exception {
        var exception = assertThrows(ExecutionException.class, () -> operation.toCompletableFuture().get(10, TimeUnit.SECONDS));
        var cause = assertInstanceOf(EchoAgentException.class, exception.getCause());
        errors.put(name, cause.code());
    }
    @Test void acceptedTelemetryExportsRealBusinessRecordsAndFlushes() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank());
        assumeTrue(host != null && !host.isBlank(), "real Host required");
        for (boolean console : List.of(false, true)) {
            Path root = Path.of(System.getenv().getOrDefault("ECHO_AGENT_SDK_TELEMETRY_ROOT", "../../target/telemetry-host")).toAbsolutePath().normalize();
            Files.createDirectories(root);
            Path work = Files.createTempDirectory(root, "java-");
            helper(work, console ? new String[]{"--console"} : new String[]{});
            var service = new ProcessBuilder("python3", FIXTURE.toString(), "--work", work.toString(), "--serve")
                    .redirectOutput(ProcessBuilder.Redirect.DISCARD).redirectError(work.resolve("service.stderr").toFile()).start();
            ObjectNode facts = JsonSupport.MAPPER.createObjectNode().put("work", work.toString()).put("host", host)
                    .put("host_sha256", HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256").digest(Files.readAllBytes(Path.of(host)))));
            var errors = facts.putObject("errors");
            EchoAgentClient sdk = null;
            try {
                long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
                while (!Files.exists(work.resolve("ready"))) {
                    assertTrue(service.isAlive(), "collector/model fixture exited");
                    assertTrue(System.nanoTime() < deadline, "collector/model ready timeout");
                    Thread.sleep(10);
                }
                var scenario = JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());
                facts.set("scenario", scenario);
                sdk = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()),
                        Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                        Map.of("RUST_LOG", "info", "OTEL_METRIC_EXPORT_INTERVAL", "60000", "OTEL_BSP_SCHEDULE_DELAY", "60000")));
                var agent = await(sdk.createAgent()); var session = await(agent.createSession(work.toString()));
                facts.set("before", await(session.invoke("telemetry.status", List.of())));
                failure(errors, "shutdown_before", session.invoke("telemetry.shutdown", List.of()));
                var endpoint = scenario.path("endpoint").textValue(); var name = scenario.path("service_name").textValue();
                failure(errors, "invalid_console", session.invoke("telemetry.init", List.of(endpoint, name, "false")));
                failure(errors, "invalid_endpoint", session.invoke("telemetry.init", List.of("", name, false)));
                failure(errors, "invalid_service", session.invoke("telemetry.init", List.of(endpoint, " ", false)));
                facts.set("init", await(session.invoke("telemetry.init", List.of(endpoint, name, console))));
                facts.set("after", await(session.invoke("telemetry.status", List.of())));
                failure(errors, "repeat_init", session.invoke("telemetry.init", List.of(endpoint, name, console)));
                var run = await(session.startChat("Read probe.txt using read_file and consume its full content."));
                assertTrue(await(run.waitForSettlement(Duration.ofSeconds(15))).path("settled").asBoolean());
                facts.set("run_outcome", await(run.outcomeStatus())); facts.set("replay", await(run.replay("0", 512)));
                facts.set("shutdown", await(session.invoke("telemetry.shutdown", List.of())));
                facts.put("collector_at_shutdown", Files.readString(work.resolve("otlp.jsonl")));
                facts.set("after_shutdown", await(session.invoke("telemetry.status", List.of())));
                failure(errors, "repeat_shutdown", session.invoke("telemetry.shutdown", List.of()));
                await(session.closeAsync()); await(agent.closeAsync());
                JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(), facts);
                var proof = JsonSupport.MAPPER.readTree(helper(work, "--verify"));
                assertTrue(proof.path("shutdown_flush").asBoolean());
                assertTrue(proof.path("business_file_content_consumed").asBoolean());
            } finally {
                JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(), facts);
                if (sdk != null) await(sdk.closeAsync());
                Files.writeString(work.resolve("stop"), "stop");
                if (!service.waitFor(5, TimeUnit.SECONDS)) { service.destroy(); assertTrue(service.waitFor(5, TimeUnit.SECONDS)); }
                System.out.println("Telemetry evidence: " + work);
            }
        }
    }
    static void barrier(Path work, String name) throws Exception {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
        while (!Files.exists(work.resolve(name))) {
            assertTrue(System.nanoTime() < deadline, "fixture barrier timed out: " + name);
            Thread.sleep(5);
        }
    }
    @Test void publicCloseWaitsForRealSlowTelemetryFlush() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank());
        assumeTrue(host != null && !host.isBlank(), "real Host required");
        Path root = Path.of(System.getenv().getOrDefault("ECHO_AGENT_SDK_TELEMETRY_ROOT", "../../target/telemetry-host")).toAbsolutePath().normalize();
        Files.createDirectories(root); Path work = Files.createTempDirectory(root, "java-public-close-");
        helper(work);
        var service = new ProcessBuilder("python3", FIXTURE.toString(), "--work", work.toString(), "--serve")
                .redirectOutput(ProcessBuilder.Redirect.DISCARD).redirectError(work.resolve("service.stderr").toFile()).start();
        var facts = JsonSupport.MAPPER.createObjectNode().put("work", work.toString()).put("host", host)
                .put("mode", "Java public sdk.closeAsync only; no fixture EOF");
        EchoAgentClient sdk = null;
        try {
            barrier(work, "ready"); var scenario = JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());
            sdk = await(EchoAgentClient.spawn("python3", List.of(FIXTURE.toString(), "--work", work.toString(), "--proxy-host", host),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                    Map.of("RUST_LOG", "info", "OTEL_METRIC_EXPORT_INTERVAL", "60000", "OTEL_BSP_SCHEDULE_DELAY", "60000")));
            var agent = await(sdk.createAgent()); var session = await(agent.createSession(work.toString()));
            await(session.invoke("telemetry.init", List.of(scenario.path("endpoint").textValue(), scenario.path("service_name").textValue(), false)));
            var run = await(session.startChat("Read probe.txt using read_file and consume its full content."));
            assertTrue(await(run.waitForSettlement(Duration.ofSeconds(15))).path("settled").asBoolean());
            Files.writeString(work.resolve("hold-export"), "hold actual export response");
            var shutdown = session.invoke("telemetry.shutdown", List.of()).toCompletableFuture();
            // Observe public errors without allowing an abandoned stage warning.
            var shutdownOutcome = shutdown.handle((value, error) -> error == null ? "completed" : error.toString());
            barrier(work, "export-held");
            facts.set("status_while_export_held", await(session.invoke("telemetry.status", List.of())));
            assertFalse(shutdown.isDone());
            var close = sdk.closeAsync().toCompletableFuture();
            Thread.sleep(100);
            facts.putObject("before_release").put("public_close_done", close.isDone()).put("real_host_exited", Files.exists(work.resolve("host-exit.json")));
            JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("wire-observed.json").toFile(), facts);
            assertFalse(close.isDone(), "public close finished while actual exporter work was held");
            assertFalse(Files.exists(work.resolve("host-exit.json")));
            Files.writeString(work.resolve("release-export"), "release actual response");
            await(close); facts.put("shutdown_waiter_outcome", await(shutdownOutcome)); barrier(work, "host-exit.json");
            var exit = JsonSupport.MAPPER.readTree(work.resolve("host-exit.json").toFile()); facts.set("exit", exit);
            assertEquals(0, exit.path("returncode").asInt());
            facts.put("collector_after_exit", Files.readString(work.resolve("otlp.jsonl")));
            assertTrue(facts.path("collector_after_exit").textValue().contains("llm.calls"));
        } finally {
            Files.writeString(work.resolve("release-export"), "cleanup actual response");
            if (sdk != null) await(sdk.closeAsync());
            helper(work, "--cleanup-host"); Files.writeString(work.resolve("stop"), "stop");
            assertTrue(service.waitFor(5, TimeUnit.SECONDS));
            JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("wire-observed.json").toFile(), facts);
            System.out.println("Telemetry Java publicclose evidence: " + work);
        }
    }

}
