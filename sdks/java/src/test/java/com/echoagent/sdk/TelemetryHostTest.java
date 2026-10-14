package com.echoagent.sdk;

import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
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
    @FunctionalInterface interface Cleanup { void run() throws Exception; }

    /** Test-only owner: a setup or evidence failure cannot abandon --serve. */
    static final class FixtureOwner implements AutoCloseable {
        final Path work;
        final Process service;
        EchoAgentClient sdk;
        Cleanup closeSdk = () -> {
            if (sdk != null) sdk.closeAsync().toCompletableFuture().get(5, TimeUnit.SECONDS);
        };
        Cleanup evidence = () -> {};

        FixtureOwner(Path work) throws Exception {
            this.work = work;
            // No metadata/hash/readiness operation follows start outside the
            // try-with-resources owner in the caller.
            service = new ProcessBuilder("python3", FIXTURE.toString(), "--work", work.toString(), "--serve")
                    .redirectOutput(ProcessBuilder.Redirect.DISCARD)
                    .redirectError(work.resolve("service.stderr").toFile()).start();
        }

        void reap() throws Exception {
            if (!service.waitFor(5, TimeUnit.SECONDS)) {
                service.destroy();
                if (!service.waitFor(5, TimeUnit.SECONDS)) {
                    service.destroyForcibly();
                    assertTrue(service.waitFor(5, TimeUnit.SECONDS), "original fixture still alive after KILL");
                }
                throw new java.io.IOException("fixture required forced cleanup after stop deadline");
            }
            assertEquals(0, service.exitValue(), "fixture failed while cleaning up original collector");
        }

        @Override public void close() throws Exception {
            var failures = new ArrayList<Throwable>();
            for (Cleanup cleanup : List.<Cleanup>of(
                    () -> Files.writeString(work.resolve("release-export"), "cleanup actual response"),
                    () -> closeSdk.run(),
                    () -> helper(work, "--cleanup-host"),
                    () -> Files.writeString(work.resolve("stop"), "stop"),
                    this::reap,
                    () -> evidence.run())) {
                try { cleanup.run(); } catch (Throwable error) { failures.add(error); }
            }
            if (!failures.isEmpty()) {
                var failure = new Exception("telemetry fixture cleanup failed", failures.get(0));
                failures.stream().skip(1).forEach(failure::addSuppressed);
                throw failure;
            }
        }
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
            try (var owned = new FixtureOwner(work)) {
                var service = owned.service;
                ObjectNode facts = JsonSupport.MAPPER.createObjectNode().put("work", work.toString()).put("host", host);
                owned.evidence = () -> {
                    JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(), facts);
                    System.out.println("Telemetry evidence: " + work);
                };
                facts.put("host_sha256", FileSha256.sha256(Path.of(host)));
                var errors = facts.putObject("errors");
                EchoAgentClient sdk;
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
                owned.sdk = sdk;
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
        try (var owned = new FixtureOwner(work)) {
            var facts = JsonSupport.MAPPER.createObjectNode().put("work", work.toString()).put("host", host)
                    .put("mode", "Java public sdk.closeAsync only; no fixture EOF");
            owned.evidence = () -> {
                JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("wire-observed.json").toFile(), facts);
                System.out.println("Telemetry Java publicclose evidence: " + work);
            };
            facts.put("host_sha256", FileSha256.sha256(Path.of(host)));
            EchoAgentClient sdk;
            barrier(work, "ready"); var scenario = JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());
            sdk = await(EchoAgentClient.spawn("python3", List.of(FIXTURE.toString(), "--work", work.toString(), "--proxy-host", host),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                    Map.of("RUST_LOG", "info", "OTEL_METRIC_EXPORT_INTERVAL", "60000", "OTEL_BSP_SCHEDULE_DELAY", "60000")));
            owned.sdk = sdk;
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
        }
    }

    @Test void setupAndCleanupFaultsReapOriginalFixtureAndCollector() throws Exception {
        assumeTrue(System.getenv("ECHO_AGENT_SDK_OTLP_COLLECTOR") != null, "real collector required");
        Path root = Path.of(System.getenv().getOrDefault("ECHO_AGENT_SDK_TELEMETRY_ROOT", "../../target/telemetry-host")).toAbsolutePath().normalize();
        Files.createDirectories(root);
        Path work = Files.createTempDirectory(root, "java-cleanup-fault-");
        helper(work);
        long[] pids = new long[2];
        var closeError = new java.io.IOException("controlled SDK close failure");
        var evidenceError = new java.io.IOException("controlled evidence write failure");
        var primary = assertThrows(java.nio.file.NoSuchFileException.class, () -> {
            try (var owned = new FixtureOwner(work)) {
                pids[0] = owned.service.pid();
                owned.closeSdk = () -> { throw closeError; };
                owned.evidence = () -> { throw evidenceError; };
                barrier(work, "ready");
                pids[1] = JsonSupport.MAPPER.readTree(work.resolve("collector.json").toFile()).path("pid").longValue();
                assertTrue(pids[0] > 0 && pids[1] > 0);
                FileSha256.sha256(work.resolve("missing-host"));
            }
        });
        assertEquals(1, primary.getSuppressed().length);
        var cleanup = primary.getSuppressed()[0];
        assertSame(closeError, cleanup.getCause());
        assertTrue(List.of(cleanup.getSuppressed()).contains(evidenceError));
        for (long pid : pids) assertFalse(ProcessHandle.of(pid).map(ProcessHandle::isAlive).orElse(false), "original PID still alive: " + pid);
        assertTrue(Files.exists(work.resolve("stop")));
    }

}
