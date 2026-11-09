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

class ConsumerCompatibilityHostTest {
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
    void consumerCompatibilityWholeLoop() throws Exception {
        String selectedHost=System.getenv("ECHO_AGENT_SDK_HOST");
        boolean hasHost=selectedHost!=null&&!selectedHost.isBlank();
        if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(hasHost,"required Host missing");
        Assumptions.assumeTrue(hasHost,"real Host required");
        String configuredRoot=System.getenv("ECHO_AGENT_SDK_CONSUMER_ROOT");
        assertNotNull(configuredRoot,"consumer runtime root required");
        Path work=Path.of(configuredRoot).toAbsolutePath().normalize();
        Path workspace=Path.of("../..").toAbsolutePath().normalize().getParent();
        assertTrue(work.startsWith(workspace)&&!work.equals(workspace));
        for (String protocol : List.of("responses", "anthropic", "chat_completions")) {
            for (boolean streaming : List.of(false, true)) {
                Path scope=work.resolve(protocol + "-" + streaming);
                java.nio.file.Files.createDirectories(scope);
                scenario(scope, protocol, streaming);
            }
        }
    }

    private void scenario(Path work, String protocol, boolean streaming) throws Exception {
        String selectedHost = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean configured = selectedHost != null && !selectedHost.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) {
            assertTrue(configured, "required native Host missing");
        }
        Assumptions.assumeTrue(configured, "real Host required");
        Path root = Path.of("../..").toAbsolutePath().normalize();
        var spec = JSON.readTree(root.resolve("sdks/shared/testing/consumer_compatibility_host.json").toFile());
        spec=JSON.readTree(JSON.writeValueAsString(spec.path(protocol)).replace("__WORK__", work.toString()));
        final var scenario=spec;
        java.nio.file.Files.writeString(work.resolve("fixture.txt"), "actual-tool-a");
        var producers=new CopyOnWriteArrayList<CompletableFuture<Void>>();
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
                    .put("supports_streaming", streaming);
            var registration = await(sdk.registerLlmClient("native-java", descriptor, (call, cancel) -> {
                requests.add(call.deepCopy());
                assertTrue(requests.size() <= scenario.path("responses").size(), "unexpected extra model request");
                var response=scenario.path("responses").get(requests.size()-1);
                if (streaming) {
                    WireHandle stream=WireHandle.fromJson(call.path("stream"));
                    var writer=sdk.streamWriter(stream);
                    var chunk=JSON.createObjectNode().put("role","assistant");
                    var message=response.path("message");
                    chunk.put("content",message.path("content").path("value").asText());
                    chunk.set("assistant_blocks",message.path("assistant_blocks"));
                    if(message.has("reasoning_blocks")) chunk.set("reasoning_blocks",message.path("reasoning_blocks"));
                    if(message.has("tool_calls")) {
                        var calls=chunk.putArray("tool_calls");int index=0;
                        for(var tool:message.path("tool_calls")) {
                            var item=calls.addObject().put("index",index++).put("id",tool.path("id").asText()).put("call_type","function");
                            item.set("provider_source",tool.path("provider_source"));
                            item.putObject("function").put("name",tool.path("function_name").asText()).put("arguments",tool.path("arguments").asText());
                        }
                    }
                    producers.add(CompletableFuture.runAsync(()->{
                        try {
                            var chunkValue=JSON.createObjectNode().put("kind","llm");chunkValue.set("value",chunk);
                            var completeValue=JSON.createObjectNode().put("kind","llm");
                            completeValue.set("value",JSON.createObjectNode().put("finish_reason",response.path("finish_reason").asText()));
                            await(writer.chunk(chunkValue));await(writer.complete(completeValue));
                        }
                        catch(Exception error) {throw new java.util.concurrent.CompletionException(error);}
                    }));
                    var outcome=JSON.createObjectNode().put("outcome","stream");outcome.set("stream",call.path("stream"));
                    return CompletableFuture.completedFuture(outcome);
                }
                var result=JSON.createObjectNode().put("operation","llm_chat");result.set("value",response.deepCopy());
                var outcome=JSON.createObjectNode().put("outcome","result");outcome.set("result",result);
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
            for(var producer:producers) producer.get(10,TimeUnit.SECONDS);
            var replay = await(run.replay("0", 512));
            assertTrue(receipt.path("settled").asBoolean());
            assertEquals("completed", receipt.path("terminal").path("status").asText());
            assertEquals(spec.path("expected_model_calls").asInt(), requests.size());
            var liveEnvelopes = live.stream().map(value -> value.required("envelope")).toList();
            var replayEnvelopes = new ArrayList<JsonNode>();
            replay.path("events").forEach(replayEnvelopes::add);
            Path livePage=work.resolve("complete-live.json"),replayPage=work.resolve("complete-replay.json");
            var liveDocument=JSON.createObjectNode();liveDocument.set("events",JSON.valueToTree(liveEnvelopes));
            var replayDocument=JSON.createObjectNode();replayDocument.set("events",JSON.valueToTree(replayEnvelopes));
            JSON.writeValue(livePage.toFile(),liveDocument);JSON.writeValue(replayPage.toFile(),replayDocument);
            var comparison=new ProcessBuilder(System.getenv().getOrDefault("PYTHON","python3"),
                    root.resolve("sdks/shared/testing/gap_ack_fixture.py").toString(),"--work",work.toString(),
                    "--mode","window","--compare-pages",livePage.toString(),replayPage.toString())
                    .redirectErrorStream(true).redirectOutput(work.resolve("complete-replay-compare.log").toFile()).start();
            try {
                assertTrue(comparison.waitFor(10,TimeUnit.SECONDS),"complete replay comparison timed out");
                assertEquals(0,comparison.exitValue(),java.nio.file.Files.readString(work.resolve("complete-replay-compare.log")));
            } finally {
                if(comparison.isAlive()) {comparison.destroyForcibly();assertTrue(comparison.waitFor(5,TimeUnit.SECONDS));}
            }
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
            assertEquals(java.util.stream.StreamSupport.stream(scenario.path("expected_phases").spliterator(),false).map(JsonNode::asText).toList(),
                    blocks.stream().map(block -> field(block, "phase").path("value").asText()).toList());
            assertEquals(spec.path("final_answer").asText(), field(blocks.get(blocks.size()-1), "delta").path("value").asText());
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
            var toolMessages=requests.get(1).path("invocation").path("input").path("messages");
            for(String id:List.of("call-a","call-b")) {
                JsonNode found=null;
                for(var item:toolMessages) if(id.equals(item.path("tool_call_id").asText())) found=item;
                assertNotNull(found,"original tool result missing");
                assertTrue(found.has("tool_result_is_error"));assertEquals("call-b".equals(id),found.path("tool_result_is_error").asBoolean());
            }
            boolean reasoningFound=false;
            for(var item:requests.get(2).path("invocation").path("input").path("messages"))
                for(var reason:item.path("reasoning_blocks")) if(reason.equals(scenario.path("reasoning"))) reasoningFound=true;
            assertTrue(reasoningFound);
            await(session.closeAsync());
            await(agent.closeAsync());
            registration.close();
        } finally {
            // 原SDK close已结算所属Host，随后观察同一批发送future，失败原件保留。
            var cleanupFailures=new ArrayList<String>();
            for(var producer:producers) {
                try {producer.get(10,TimeUnit.SECONDS);}
                catch(Exception error) {cleanupFailures.add(error.toString());}
            }
            java.nio.file.Files.writeString(work.resolve("producer-cleanup.json"),JSON.writeValueAsString(cleanupFailures));
        }
    }
}
