package com.echoagent.sdk;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** The model inspects actual Host messages; this consumer never resolves or injects rules. */
class SessionRulesPromptHostTest {
    static final Path FIXTURE = Path.of("../shared/testing/session_rules_prompt_fixture.py").toAbsolutePath().normalize();
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

    @Test void sessionCwdRulesReachActualModelAndNewSessionReload() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank(), "required Session rules Host missing");
        assumeTrue(host != null && !host.isBlank(), "real Host required");
        Path root = Path.of(System.getenv().getOrDefault("ECHO_AGENT_SDK_SESSION_RULES_ROOT", "../../target/session-rules-prompt")).toAbsolutePath().normalize();
        Files.createDirectories(root);
        Path work = Files.createTempDirectory(root, "java-");
        helper(work);
        var model = new ProcessBuilder("python3", FIXTURE.toString(), "--work", work.toString(), "--serve").redirectOutput(ProcessBuilder.Redirect.DISCARD).redirectError(work.resolve("server.stderr").toFile()).start();
        EchoAgentClient sdk = null;
        try {
            long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
            while (!Files.exists(work.resolve("ready"))) {
                assertTrue(model.isAlive(), "model fixture exited");
                assertTrue(System.nanoTime() < deadline, "model fixture ready timeout");
                Thread.sleep(10);
            }
            sdk = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()));
            var agent = await(sdk.createAgent());
            var rows = JsonSupport.MAPPER.createArrayNode();
            for (var step : JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile())) {
                if (step.path("id").asText().equals("a2")) helper(work, "--mutate");
                var session = await(agent.createSession(work.resolve(step.path("cwd").asText()).toString()));
                var run = await(session.startChat(step.path("user").asText()));
                assertTrue(await(run.waitForSettlement(Duration.ofSeconds(15))).path("settled").asBoolean());
                var row = rows.addObject().put("id", step.path("id").asText());
                row.set("outcome", await(run.outcomeStatus()));
                row.set("replay", await(run.replay("0", 512)));
                await(session.closeAsync());
            }
            await(agent.closeAsync());
            Files.writeString(work.resolve("native-results.json"), rows.toString());
            var proof = JsonSupport.MAPPER.readTree(helper(work, "--verify"));
            assertEquals(3, proof.path("cases").asInt());
            System.out.println("session-rules-prompt work=" + work + " " + proof);
        } finally {
            if (sdk != null) await(sdk.closeAsync());
            model.destroy();
            assertTrue(model.waitFor(5, TimeUnit.SECONDS), "model process must exit");
        }
    }
}
