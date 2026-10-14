package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HexFormat;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Every Host call is made by the native Java SDK, never by the Python oracle. */
class RagProjectRulesHostTest {
    static final Path FIXTURE = Path.of("../shared/testing/rag_project_rules_fixture.py").toAbsolutePath();
    static <T> T await(CompletionStage<T> stage) throws Exception {
        return stage.toCompletableFuture().get(15, TimeUnit.SECONDS);
    }
    static String helper(Path work, String... args) throws Exception {
        var command = new ArrayList<>(List.of("python3", FIXTURE.toString(), "--work", work.toString()));
        command.addAll(List.of(args));
        var process = new ProcessBuilder(command).redirectErrorStream(true).start();
        String output = new String(process.getInputStream().readAllBytes(), java.nio.charset.StandardCharsets.UTF_8);
        assertEquals(0, process.waitFor(), output);
        return output;
    }
    static JsonNode rebase(JsonNode value, Path work) {
        if (value.isTextual()) return JsonSupport.MAPPER.getNodeFactory().textNode(value.textValue().replace("$WORK", work.toString()));
        if (value.isObject()) {
            var result = JsonSupport.MAPPER.createObjectNode();
            value.fields().forEachRemaining(entry -> result.set(entry.getKey(), rebase(entry.getValue(), work)));
            return result;
        }
        return value;
    }
    static ObjectNode error(EchoAgentException failure) {
        var result = JsonSupport.MAPPER.createObjectNode().put("code", failure.code()).put("message", failure.getMessage()).put("operation", failure.operation()).put("retryable", failure.retryable());
        result.set("details", failure.details());
        result.set("handle", failure.handle() == null ? JsonSupport.MAPPER.nullNode() : failure.handle().toJson());
        return result;
    }

    @Test void ragProjectRulesRetainAuthorityFailuresAndOriginalFiles() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank(), "required RAG/project-rules Host missing");
        assumeTrue(host != null && !host.isBlank(), "real Host required");
        Path root = Path.of(System.getenv().getOrDefault("ECHO_AGENT_SDK_RAG_RULES_ROOT", "../../target/rag-project-rules")).toAbsolutePath();
        Files.createDirectories(root);
        Path work = Files.createTempDirectory(root, "java-");
        helper(work);
        var server = new ProcessBuilder("python3", FIXTURE.toString(), "--work", work.toString(), "--serve").redirectOutput(ProcessBuilder.Redirect.DISCARD).redirectError(work.resolve("server.stderr").toFile()).start();
        EchoAgentClient sdk = null;
        var rows = JsonSupport.MAPPER.createArrayNode();
        try {
            for (int count = 0; count < 200 && !Files.exists(work.resolve("endpoint.json")); count++) Thread.sleep(10);
            String endpoint = JsonSupport.MAPPER.readTree(work.resolve("endpoint.json").toFile()).path("url").asText();
            var environment = Map.of("EMBEDDING_BASEURL", endpoint, "EMBEDDING_MODEL", "fixture-中文-embedding", "EMBEDDING_APIKEY", "fixture-key-not-secret");
            sdk = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), environment));
            var agent = await(sdk.createAgent());
            var sessions = new HashMap<String, SessionHandle>();
            for (var entry : Map.of("a", "repo/nested/leaf", "b", "other", "empty", "empty", "new", "repo/nested/leaf").entrySet()) sessions.put(entry.getKey(), await(agent.createSession(work.resolve(entry.getValue()).toString())));
            var values = new HashMap<String, JsonNode>();
            for (var step : JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile()).path("steps")) {
                if (step.has("action")) {
                    if (step.path("action").asText().equals("close")) await(sessions.get(step.path("session").asText()).closeAsync());
                    else { Files.writeString(work.resolve("action.json"), step.toString()); helper(work, "--action", work.resolve("action.json").toString()); }
                    continue;
                }
                var session = sessions.get(step.path("session").asText());
                var arguments = new ArrayList<Object>();
                var observedArguments = JsonSupport.MAPPER.createArrayNode();
                for (var argument : step.path("arguments")) {
                    if (argument.isTextual() && argument.asText().equals("$session")) {
                        arguments.add(session.wire());
                        observedArguments.add(JsonSupport.MAPPER.createObjectNode().put("kind", "handle").set("value", session.wire().toJson()));
                    } else if (argument.isTextual() && argument.asText().startsWith("$") && !argument.asText().startsWith("$WORK")) {
                        JsonNode result = values.get(argument.asText().substring(1));
                        assertNotNull(result, "required previous receiver result");
                        var handle = JsonSupport.MAPPER.createObjectNode().put("kind", "handle");
                        handle.set("value", result);
                        arguments.add(handle); observedArguments.add(handle);
                    } else {
                        JsonNode result = rebase(argument, work);
                        arguments.add(result.isObject() && result.has("kind") ? result : JsonSupport.MAPPER.convertValue(result, Object.class));
                        observedArguments.add(result);
                    }
                }
                String operation = step.path("operation").asText();
                var row = JsonSupport.MAPPER.createObjectNode().put("id", step.path("id").asText()).put("operation", operation);
                row.set("arguments", observedArguments);
                try {
                    JsonNode result = await(sdk.call(operation, operation.startsWith("echo_core::") ? agent.wire() : session.wire(), arguments));
                    row.set("result", result);
                    if (step.path("id").asText().startsWith("chunk-")) {
                        var preview = JsonSupport.MAPPER.readTree(result.path("output").asText());
                        String content = (String) arguments.get(0);
                        int scalarCount = content.codePointCount(0, content.length());
                        assertEquals(1, preview.path("total_chunks").asInt());
                        var chunk = preview.path("chunks").get(0);
                        assertEquals(scalarCount, chunk.path("char_count").asInt());
                        assertEquals(content.substring(0, content.offsetByCodePoints(0, Math.min(200, scalarCount))), chunk.path("preview").asText());
                        assertEquals(scalarCount > 200, chunk.path("truncated").asBoolean());
                    }
                    if (!step.path("save").isNull() && step.path("save").isTextual()) values.put(step.path("save").asText(), result);
                } catch (ExecutionException failure) {
                    var typed = assertInstanceOf(EchoAgentException.class, failure.getCause());
                    row.set("error", error(typed));
                }
                rows.add(row); Files.writeString(work.resolve("rows.json"), rows.toPrettyString());
            }
            for (String name : List.of("b", "empty", "new")) await(sessions.get(name).closeAsync());
            await(agent.closeAsync()); sdk.close(); sdk = null;
            var badEnvironment = new HashMap<>(environment); badEnvironment.put("EMBEDDING_MODEL", "");
            try (var bad = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), badEnvironment))) {
                var badAgent = await(bad.createAgent()); var session = await(badAgent.createSession(work.resolve("empty").toString()));
                var failed = assertThrows(ExecutionException.class, () -> await(bad.call("rag_search", session.wire(), List.of("query", 1))));
                var typed = assertInstanceOf(EchoAgentException.class, failed.getCause());
                rows.add(JsonSupport.MAPPER.createObjectNode().put("id", "invalid-config").put("operation", "rag_search").set("error", error(typed)));
                await(session.closeAsync()); await(badAgent.closeAsync());
            }
            var evidence = JsonSupport.MAPPER.createObjectNode().put("language", "java").put("host", host).put("host_sha256", HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256").digest(Files.readAllBytes(Path.of(host))))).put("work", work.toString()).put("server_pid", server.pid());
            evidence.set("rows", rows);
            Files.writeString(work.resolve("results.json"), evidence.toPrettyString());
            System.out.println("RAG/project-rules verified: " + helper(work, "--verify", work.resolve("results.json").toString()) + " evidence: " + work);
        } finally {
            if (sdk != null) sdk.close();
            server.destroy(); assertTrue(server.waitFor(5, TimeUnit.SECONDS), "embedding server physically exits");
            System.out.println("RAG/project-rules retained: " + work);
        }
    }
}
