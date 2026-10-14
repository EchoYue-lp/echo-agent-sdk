package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** The shared peer speaks real A2A HTTP/SSE; all operations execute the Host. */
class A2AHostTest {
    static final Path FIXTURE = Path.of("../shared/testing/a2a_host_fixture.py").toAbsolutePath().normalize();
    static <T> T await(CompletionStage<T> stage) throws Exception {
        return stage.toCompletableFuture().get(15, TimeUnit.SECONDS);
    }
    @Test void acceptedA2AHttpSseOwnerAndErrors() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank());
        assumeTrue(host != null && !host.isBlank(), "real Host required");
        Path root = Path.of(System.getenv().getOrDefault("ECHO_AGENT_SDK_A2A_ROOT", "../../target/a2a-native")).toAbsolutePath().normalize();
        Files.createDirectories(root); Path work = Files.createTempDirectory(root, "java-");
        var peer = new ProcessBuilder("python3", FIXTURE.toString(), "--work", work.toString())
                .redirectOutput(ProcessBuilder.Redirect.DISCARD).redirectError(work.resolve("peer.stderr").toFile()).start();
        EchoAgentClient sdk = null; var rows = JsonSupport.MAPPER.createArrayNode();
        Map<String,JsonNode> values = new HashMap<>();
        try {
            long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
            while (!Files.exists(work.resolve("ready"))) {
                assertTrue(peer.isAlive()); assertTrue(System.nanoTime() < deadline); Thread.sleep(5);
            }
            sdk = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"),
                    Map.of("NO_PROXY", "127.0.0.1,localhost", "no_proxy", "127.0.0.1,localhost")));
            var agent = await(sdk.createAgent());
            var sessions = Map.of("a", await(agent.createSession(work.toString())), "b", await(agent.createSession(work.toString())));
            for (var step : JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile())) {
                var row = rows.addObject().put("id", step.path("id").asText());
                if ("close_session".equals(step.path("action").asText())) {
                    await(sessions.get("a").closeAsync()); row.put("value", true); continue;
                }
                var args = new ArrayList<Object>();
                for (var arg : step.path("args")) {
                    JsonNode value = arg;
                    if (arg.isObject() && arg.has("ref")) {
                        value = values.get(arg.path("ref").asText());
                        if (arg.has("field")) value = value.get(arg.path("field").asText());
                        if (arg.has("generation_delta")) {
                            ObjectNode modified = value.deepCopy();
                            modified.put("generation", new BigInteger(value.path("generation").asText()).add(BigInteger.valueOf(arg.path("generation_delta").asLong())).toString());
                            value = modified;
                        }
                    }
                    if (value.isObject() && value.has("id") && value.has("generation") && value.has("kind")) args.add(WireHandle.fromJson(value));
                    else if (value.isTextual()) args.add(value.textValue());
                    else if (value.isNull()) args.add(null);
                    else if (value.isIntegralNumber()) args.add(value.longValue());
                    else throw new IllegalArgumentException("unsupported shared fixture argument: " + value);

                }
                try {
                    var result = await(sessions.get(step.path("session").asText("a")).invoke(step.path("op").asText(), args));
                    assertFalse(step.has("error"), step.path("id").asText() + " unexpectedly succeeded");
                    row.set("value", result); values.put(step.path("id").asText(), result);
                } catch (ExecutionException exception) {
                    assertTrue(step.has("error"), step.path("id").asText()+": "+exception.getCause());
                    var error = assertInstanceOf(EchoAgentException.class, exception.getCause());
                    var failure = row.putObject("error").put("code", error.code()).put("message", error.getMessage())
                            .put("operation", error.operation()).put("retryable", error.retryable());
                    failure.set("details", error.details()); failure.set("handle", error.handle() == null ? null : error.handle().toJson());
                    assertEquals(step.path("error").asText(), error.code());
                }
                Files.writeString(work.resolve("results.json"), rows.toString());
            }
            await(sessions.get("b").closeAsync()); await(agent.closeAsync());
            Files.writeString(work.resolve("results.json"), rows.toString());
            var check = new ProcessBuilder("python3", FIXTURE.toString(), "--work", work.toString(), "--verify").redirectErrorStream(true).start();
            String output = new String(check.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
            assertEquals(0, check.waitFor(), output);
            assertEquals(9, JsonSupport.MAPPER.readTree(output).path("operations").size());
            System.out.println("A2A evidence " + work);
        } finally {
            Files.writeString(work.resolve("results.json"), rows.toString());
            if (sdk != null) await(sdk.closeAsync());
            Files.writeString(work.resolve("stop"), "stop");
            assertTrue(peer.waitFor(5, TimeUnit.SECONDS), "own A2A peer must exit");
        }
    }
}
