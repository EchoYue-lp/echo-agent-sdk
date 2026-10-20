package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionException;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

class ToolBehaviorsHostTest {
    @Test void sixFamiliesPreserveNativeResultsFailuresAndActualLocalEffects() throws Exception {
        runToolStory(false);
    }

    @Test void acceptedNonTextMediaPreservesParsedFilesPixelsAndTypedFailures() throws Exception {
        runToolStory(true);
    }

    private void runToolStory(boolean media) throws Exception {
        var host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertNotNull(host, "required tool behavior Host binary missing");
        assumeTrue(host != null && !host.isBlank(), "real Host binary required");
        var fixturePath = Path.of("../shared/testing/tool_behavior_fixture.py").toAbsolutePath();
        var work = Files.createTempDirectory("sdk-java-tool-behaviors-");
        var fixtureArgs = new ArrayList<>(List.of("python3", fixturePath.toString(), "--work", work.toString()));
        if (media) fixtureArgs.add("--media");
        var fixture = new ProcessBuilder(fixtureArgs).redirectError(ProcessBuilder.Redirect.INHERIT).start();
        EchoAgentClient sdk = null;
        boolean passed = false;
        try {
            long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
            while (!Files.exists(work.resolve("ready"))) {
                assertTrue(System.nanoTime() < deadline && fixture.isAlive(), "fixture preparation failed");
                Thread.sleep(10);
            }
            var scenario = JsonSupport.MAPPER.readTree(Files.readString(work.resolve("scenario.json")));
            sdk = EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of("TMPDIR",work.resolve("artifact-temp").toString()), List.of(), List.of())
                    .toCompletableFuture().get(10, TimeUnit.SECONDS);
            var agent = sdk.createAgent().toCompletableFuture().get(10, TimeUnit.SECONDS);
            var sessions = new HashMap<String, SessionHandle>();
            for (var name : List.of("a", "b")) sessions.put(name, agent.createSession(scenario.path("sessions").path(name).textValue()).toCompletableFuture().get(10, TimeUnit.SECONDS));
            var results = JsonSupport.MAPPER.createArrayNode();
            for (var step : scenario.path("steps")) {
                var result = JsonSupport.MAPPER.createObjectNode().put("id", step.path("id").textValue());
                var session = sessions.get(step.path("session").textValue());
                if (step.path("action").asText().equals("close")) {
                    session.close(); result.put("outcome", "closed"); results.add(result); continue;
                }
                var arguments = new ArrayList<Object>();
                for (var argument : step.path("arguments")) arguments.add(JsonSupport.MAPPER.convertValue(argument, Object.class));
                try {
                    JsonNode value = sdk.call(step.path("operation").textValue(), session.wire(), arguments).toCompletableFuture().get(20, TimeUnit.SECONDS);
                    result.put("outcome", "result").set("value", value);
                } catch (ExecutionException error) {
                    Throwable cause = error.getCause();
                    while (cause instanceof CompletionException && cause.getCause() != null) cause = cause.getCause();
                    var typed = assertInstanceOf(EchoAgentException.class, cause);
                    result.put("outcome", "error").put("code", typed.code()).put("message", typed.getMessage()).put("operation", typed.operation()).put("retryable",typed.retryable()).set("details",typed.details());
                }
                results.add(result);
            }
            sessions.get("b").close(); agent.close(); sdk.close(); sdk = null;
            Files.writeString(work.resolve("results.json"), JsonSupport.MAPPER.writeValueAsString(results));
            var verifyArgs = new ArrayList<>(List.of("python3", fixturePath.toString(), "--work", work.toString(), "--verify", work.resolve("results.json").toString()));
            if (media) verifyArgs.add("--media");
            var verified = new ProcessBuilder(verifyArgs).redirectErrorStream(true).start();
            var output = new String(verified.getInputStream().readAllBytes(), java.nio.charset.StandardCharsets.UTF_8);
            assertTrue(verified.waitFor(10, TimeUnit.SECONDS));
            assertEquals(0, verified.exitValue(), output + "\nEvidence: " + work);
            var evidence = JsonSupport.MAPPER.readTree(output);
            if (media) assertEquals(12, evidence.path("successful_operations").intValue());
            else assertEquals(6, evidence.path("capability_families").intValue());
            assertEquals(scenario.path("steps").size(), evidence.path("native_calls").intValue());
            System.out.println("Tool behavior verified: " + evidence);
            if (media) System.out.println("Accepted media evidence: " + work);
            passed = true;
        } finally {
            if (sdk != null) sdk.close();
            fixture.destroy(); assertTrue(fixture.waitFor(5, TimeUnit.SECONDS));
            if (passed && !media) {
                try (var files = Files.walk(work)) {
                    for (var file : files.sorted(java.util.Comparator.reverseOrder()).toList()) Files.deleteIfExists(file);
                }
            } else if (!passed) System.err.println("Tool behavior evidence retained: " + work);
        }
    }
}
