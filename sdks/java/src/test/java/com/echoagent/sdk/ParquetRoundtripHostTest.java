package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** 真实 ACP/Host 消费；共享 Python 只提供输入与独立结果检查。 */
class ParquetRoundtripHostTest {
    private static final ObjectMapper JSON = new ObjectMapper();
    private static <T> T await(CompletionStage<T> result) throws Exception {
        return result.toCompletableFuture().get(15, TimeUnit.SECONDS);
    }

    private static void helper(Path work, String label, String... command) throws Exception {
        // 子进程不能直接写 Surefire 的原生控制管道；原始输出保存后经 Java 日志发布。
        Path log = work.resolve(label + ".log");
        Process process = new ProcessBuilder(command).redirectErrorStream(true).redirectOutput(log.toFile()).start();
        boolean finished = process.waitFor(10, TimeUnit.SECONDS);
        if (!finished) {
            process.destroyForcibly();
            assertTrue(process.waitFor(5, TimeUnit.SECONDS), "fixture process did not exit");
        }
        assertTrue(finished, "fixture timeout: " + log);
        String output = Files.readString(log);
        assertEquals(0, process.exitValue(), output);
        if (!output.isBlank()) System.out.println(output);
    }

    @Test void parquetRoundtripPreservesValuesAndProtectedOutput() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean present = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) {
            assertTrue(present, "required Parquet Host binary is missing");
        }
        Assumptions.assumeTrue(present, "real Host required");
        Path work = Files.createTempDirectory(Path.of(System.getenv().getOrDefault("TMPDIR", System.getProperty("java.io.tmpdir"))), "sdk-parquet-java-");
        Path fixture = Path.of("../shared/testing/parquet_roundtrip_fixture.py").toAbsolutePath();
        String python = System.getenv().getOrDefault("PYTHON", "python3");
        helper(work, "prepare", python, fixture.toString(), "--work", work.toString());
        var rows = JSON.createArrayNode();
        String catalogOverride = System.getenv("ECHO_AGENT_SDK_CATALOG_PATH");
        Path catalog = catalogOverride == null ? null : Path.of(catalogOverride);
        Path digest = catalog == null ? null : catalog.resolveSibling("contract-digests.json");
        try (var sdk = await(EchoAgentClient.spawn(host, java.util.List.of("--config", work.resolve("host.json").toString()),
                catalog, digest, java.util.Map.of()))) {
            var agent = await(sdk.createAgent());
            var session = await(agent.createSession(work.toString()));
            for (JsonNode step : JSON.readTree(work.resolve("scenario.json").toFile())) {
                var row = ((com.fasterxml.jackson.databind.node.ObjectNode) step).deepCopy();
                var arguments = new ArrayList<Object>();
                step.path("arguments").forEach(value -> arguments.add(JSON.convertValue(value, Object.class)));
                try {
                    row.set("result", await(session.invoke(step.path("operation").textValue(), arguments)));
                    assertTrue(step.path("expected_error").isNull(), step.toString());
                } catch (java.util.concurrent.ExecutionException error) {
                    EchoAgentException typed = EchoAgentException.from(error, step.path("operation").textValue());
                    var failure = JSON.createObjectNode();
                    failure.put("code", typed.code()).put("message", typed.getMessage()).put("operation", typed.operation()).put("retryable", typed.retryable());
                    failure.set("handle", typed.handle() == null ? com.fasterxml.jackson.databind.node.NullNode.instance : typed.handle().toJson());
                    failure.set("details", typed.details() == null ? com.fasterxml.jackson.databind.node.NullNode.instance : typed.details());
                    row.set("error", failure);
                    assertEquals(step.path("expected_error").textValue(), typed.code());
                }
                rows.add(row);
            }
            await(session.closeAsync());
            await(agent.closeAsync());
        }
        var evidence = JSON.createObjectNode().put("host", host).put("host_sha256", FileSha256.sha256(Path.of(host)));
        evidence.put("catalog_path", catalogOverride);
        evidence.set("rows", rows);
        JSON.writerWithDefaultPrettyPrinter().writeValue(work.resolve("results.json").toFile(), evidence);
        System.out.println("Parquet evidence: " + work);
        helper(work, "verify", python, fixture.toString(), "--work", work.toString(), "--verify", work.resolve("results.json").toString());
    }
}
