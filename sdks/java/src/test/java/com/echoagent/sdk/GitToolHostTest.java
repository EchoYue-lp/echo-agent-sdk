package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.NullNode;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

class GitToolHostTest {
    static final String HOST = System.getenv("ECHO_AGENT_SDK_HOST");
    static final Path FIXTURE = Path.of("../shared/testing/git_tool_host_fixture.py").toAbsolutePath();
    static <T> T await(CompletionStage<T> value) throws Exception { return value.toCompletableFuture().get(20, TimeUnit.SECONDS); }
    static String oracle(Path work, String... arguments) throws Exception {
        var command = new ArrayList<>(List.of("python3", FIXTURE.toString(), "--work", work.toString()));
        command.addAll(List.of(arguments));
        var outputFile = Files.createTempFile(work, "git-oracle-", ".log");
        var process = new ProcessBuilder(command).redirectErrorStream(true).redirectOutput(outputFile.toFile()).start();
        boolean passed = false;
        try {
            assertTrue(process.waitFor(30, TimeUnit.SECONDS), "owned Git oracle timed out; output: " + outputFile);
            String output = Files.readString(outputFile, java.nio.charset.StandardCharsets.UTF_8);
            assertEquals(0, process.exitValue(), output + " evidence: " + work);
            passed = true;
            return output;
        } finally {
            if (process.isAlive()) {
                process.destroy();
                if (!process.waitFor(1, TimeUnit.SECONDS)) {
                    process.destroyForcibly();
                    assertTrue(process.waitFor(3, TimeUnit.SECONDS), "owned Git oracle did not settle after termination");
                }
            }
            if (passed) Files.deleteIfExists(outputFile);
        }
    }
    static JsonNode resolve(JsonNode value, Map<String, JsonNode> results) {
        if (value.isArray()) { var array = JsonSupport.MAPPER.createArrayNode(); for (var item : value) array.add(resolve(item, results)); return array; }
        if (value.isObject()) {
            if (value.has("$ref")) return results.get(value.path("$ref").asText());
            var object = JsonSupport.MAPPER.createObjectNode(); value.fields().forEachRemaining(field -> object.set(field.getKey(), resolve(field.getValue(), results))); return object;
        }
        return value;
    }
    @org.junit.jupiter.api.Test void acceptedGitToolsAndWorktreeSourcesPreserveExactLocalEffects() throws Exception {
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertNotNull(HOST, "required Git Host missing");
        assumeTrue(HOST != null && !HOST.isBlank(), "real Host required");
        var root = Path.of(System.getenv().getOrDefault("ECHO_AGENT_SDK_GIT_ROOT", "../../target/git-native")).toAbsolutePath().normalize(); Files.createDirectories(root);
        var work = Files.createTempDirectory(root, "java-");
        oracle(work, "--prepare"); var scenario = JsonSupport.MAPPER.readTree(Files.readString(work.resolve("scenario.json")));
        var rows = JsonSupport.MAPPER.createArrayNode(); var results = new HashMap<String, JsonNode>();
        var env = new HashMap<String, String>(); scenario.path("host_env").fields().forEachRemaining(field -> env.put(field.getKey(), field.getValue().asText()));
        String hostHash = FileSha256.sha256(Path.of(HOST));
        try (var sdk = await(EchoAgentClient.spawn(HOST, List.of("--config", work.resolve("host.json").toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), env))) {
            var agent = await(sdk.createAgent()); var session = await(agent.createSession(scenario.path("repo").asText()));
            for (var step : scenario.path("rows")) {
                if (!step.path("checkpoint").isNull()) oracle(work, "--checkpoint", step.path("checkpoint").asText());
                var row = JsonSupport.MAPPER.createObjectNode().put("id", step.path("id").asText()).put("operation", step.path("operation").asText());
                var arguments = resolve(step.path("arguments"), results); row.set("arguments", arguments); row.set("before", JsonSupport.MAPPER.readTree(oracle(work, "--snapshot")));
                List<Object> nativeArguments = new ArrayList<>(); for (var argument : arguments) nativeArguments.add(argument.isObject() && argument.has("kind") ? argument : JsonSupport.MAPPER.convertValue(argument, Object.class));
                try { var value = await(session.invoke(step.path("operation").asText(), nativeArguments)); row.set("result", value); results.put(step.path("id").asText(), value); }
                catch (ExecutionException failure) {
                    var error = assertInstanceOf(EchoAgentException.class, failure.getCause());
                    var value = row.putObject("error").put("code", error.code()).put("message", error.getMessage()).put("operation", error.operation()).put("retryable", error.retryable());
                    value.set("handle", error.handle() == null ? NullNode.instance : error.handle().toJson()); value.set("details", error.details() == null ? NullNode.instance : error.details());
                }
                row.set("after", JsonSupport.MAPPER.readTree(oracle(work, "--snapshot"))); rows.add(row);
                var data = JsonSupport.MAPPER.createObjectNode().put("work", work.toString()).put("host", HOST).put("host_sha256", hostHash); data.set("rows", rows); data.set("issued_session", session.wire().toJson());
                JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("results.json").toFile(), data);
            }
            await(session.closeAsync()); await(agent.closeAsync());
            var proof = JsonSupport.MAPPER.readTree(oracle(work, "--verify")); assertEquals(9, proof.path("tool_operations").size()); assertEquals(4, proof.path("source_operations").size()); assertEquals(3, proof.path("groups").size()); assertEquals(3, proof.path("checkpoint_operations").size()); assertEquals(7, proof.path("checkpoint_calls").intValue());
        } finally { System.out.println("Git evidence retained " + work); }
    }
}
