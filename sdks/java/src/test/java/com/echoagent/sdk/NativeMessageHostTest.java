package com.echoagent.sdk;

import static org.junit.jupiter.api.Assertions.*;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.Flow;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

class NativeMessageHostTest {
    private static final ObjectMapper JSON = new ObjectMapper();

    private static <T> T await(CompletionStage<T> value) throws Exception {
        return value.toCompletableFuture().get(10, TimeUnit.SECONDS);
    }

    private static JsonNode field(JsonNode value, String name) {
        for (var entry : value.path("value")) {
            if (name.equals(entry.path("key").path("value").asText())) {
                return entry.path("value");
            }
        }
        return JSON.nullNode();
    }

    @Test
    void nativeMessagesPreserveHistoryAndRuntimeTerminal(@TempDir Path work) throws Exception {
        String selectedHost = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean configured = selectedHost != null && !selectedHost.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) {
            assertTrue(configured, "required native Host missing");
        }
        Assumptions.assumeTrue(configured, "real Host required");
        Path root = Path.of("../..").toAbsolutePath().normalize();
        var spec = JSON.readTree(root.resolve("sdks/shared/testing/native_message_host_contract.json").toFile());
        var config = JSON.readTree(root.resolve("echo-sdk-host/config.sdk.example.json").toFile());
        ((ObjectNode) config.path("sdk_profile")).put("state_root", work.resolve("state").toString());
        ((ObjectNode) config.path("default_agent").path("agent"))
                .put("enable_tools", true).put("memory_path", work.resolve("memory.jsonl").toString());
        Path configPath = work.resolve("host.json");
        JSON.writeValue(configPath.toFile(), config);
        var requests = new CopyOnWriteArrayList<JsonNode>();
        var live = new CopyOnWriteArrayList<JsonNode>();
        try (var sdk = await(EchoAgentClient.spawn(selectedHost,
                List.of("--config", configPath.toString()), null, null, Map.of()))) {
            var descriptor = JSON.createObjectNode().put("kind", "llm_client")
                    .put("descriptor_version", 1).put("model_name", "controlled-native")
                    .put("supports_streaming", false);
            var registration = await(sdk.registerLlmClient("native-java", descriptor, (call, cancel) -> {
                requests.add(call.deepCopy());
                assertTrue(requests.size() <= spec.path("responses").size(), "unexpected extra model request");
                var result = JSON.createObjectNode().put("operation", "llm_chat");
                result.set("value", spec.path("responses").get(requests.size() - 1).deepCopy());
                var outcome = JSON.createObjectNode().put("outcome", "result");
                outcome.set("result", result);
                return CompletableFuture.completedFuture(outcome);
            }));
            var agent = await(sdk.createAgent());
            var session = await(agent.createSession(work.toString()));
            var run = await(session.startChat("native source probe"));
            var done = new CompletableFuture<Void>();
            run.events().subscribe(new Flow.Subscriber<JsonNode>() {
                public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
                public void onNext(JsonNode value) { live.add(value.deepCopy()); }
                public void onError(Throwable error) { done.completeExceptionally(error); }
                public void onComplete() { done.complete(null); }
            });
            var receipt = await(run.waitForSettlement());
            done.get(10, TimeUnit.SECONDS);
            var replay = await(run.replay("0", 512));
            assertTrue(receipt.path("settled").asBoolean());
            assertEquals("completed", receipt.path("terminal").path("status").asText());
            assertEquals(spec.path("expected_model_calls").asInt(), requests.size());
            var liveEnvelopes = live.stream().map(value -> value.required("envelope")).toList();
            var replayEnvelopes = new ArrayList<JsonNode>();
            replay.path("events").forEach(replayEnvelopes::add);
            assertEquals(liveEnvelopes, replayEnvelopes);
            assertFalse(liveEnvelopes.isEmpty());
            var last = liveEnvelopes.get(liveEnvelopes.size() - 1).required("payload");
            assertEquals("execution_status", last.path("event_type").asText());
            assertEquals("Terminal", field(last.path("data"), "phase").path("value").asText());
            assertEquals("settled", field(last.path("data"), "status").path("value").asText());
            var blocks = new ArrayList<JsonNode>();
            int terminalCount = 0;
            int compatibilityFinalCount = 0;
            int finalBlockIndex = -1;
            int compatibilityIndex = -1;
            int eventIndex = 0;
            for (var envelope : liveEnvelopes) {
                var payload = envelope.required("payload");
                switch (payload.path("event_type").asText()) {
                    case "assistant_message" -> {
                        blocks.add(field(payload.path("data"), "block"));
                        finalBlockIndex = eventIndex;
                    }
                    case "final_answer" -> {
                        compatibilityFinalCount++;
                        compatibilityIndex = eventIndex;
                        assertEquals(spec.path("final_answer").asText(),
                                payload.path("data").path("value").asText());
                    }
                    case "execution_status" -> {
                        assertTrue(field(payload.path("data"), "adapter_generated").isNull());
                        if ("Terminal".equals(field(payload.path("data"), "phase").path("value").asText())) {
                            terminalCount++;
                        }
                    }
                    default -> { }
                }
                eventIndex++;
            }
            assertEquals(1, terminalCount);
            assertEquals(1, compatibilityFinalCount);
            assertTrue(finalBlockIndex >= 0 && finalBlockIndex < compatibilityIndex
                    && compatibilityIndex < liveEnvelopes.size() - 1);
            assertEquals(List.of("commentary", "final_answer"),
                    blocks.stream().map(block -> field(block, "phase").path("value").asText()).toList());
            assertEquals(spec.path("final_answer").asText(), field(blocks.get(1), "delta").path("value").asText());
            for (var request : requests) {
                for (var message : request.path("invocation").path("input").path("messages")) {
                    assertFalse(message.path("execution_status").has("adapter_generated"));
                }
            }
            var expectedSource = spec.path("responses").get(0).path("message")
                    .path("assistant_blocks").get(0).required("provider_source");
            boolean originPreserved = false;
            for (var message : requests.get(1).path("invocation").path("input").path("messages")) {
                for (var block : message.path("assistant_blocks")) {
                    if (expectedSource.equals(block.path("provider_source"))) {
                        originPreserved = true;
                    }
                }
            }
            assertTrue(originPreserved, "history provenance loss");
            await(session.closeAsync());
            await(agent.closeAsync());
            registration.close();
        }
    }
}
