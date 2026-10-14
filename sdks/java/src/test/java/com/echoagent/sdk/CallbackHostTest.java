package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.List;
import java.util.Map;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Verifies callback identity and cancellation settlement through real Host/framework execution. */
class CallbackHostTest {
    @BeforeAll static void requireHost() {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean present = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(present, "required Host E2E binary is missing");
        assumeTrue(present, "callback tests require real Host binary");
    }
    private static <T> T await(CompletionStage<T> value) throws Exception {
        return value.toCompletableFuture().get(10, TimeUnit.SECONDS);
    }
    @Test void cancelledToolWaitsForInterruptedCallbackSideEffectAndPreservesInputIdentity() throws Exception {
        var work = Files.createTempDirectory("sdk-java-callback-");
        try (var sdk = await(EchoAgentClient.spawn("python3", List.of(
                Path.of("../python/tests/protocol_host_proxy.py").toAbsolutePath().toString(), "--host", System.getenv("ECHO_AGENT_SDK_HOST"),
                "--work", work.toString(), "--mode", "normal"), Path.of("../shared/facade-operation-catalog.json"),
                Path.of("../shared/contract-digests.json"), Map.of()))) {
            var startIdentity = new AtomicReference<String>();
            var interrupted = new AtomicReference<CallbackCall.ToolInterrupted>();
            var effectCompleted = new AtomicBoolean();
            var effectCount = new AtomicInteger();
            var parked = new CompletableFuture<Void>();
            var originalParameters = new AtomicReference<JsonNode>();
            var observer = await(sdk.registerAgentCallback("java-terminal-observer", new CallbackDescriptor(), (call, cancellation) -> {
                if (call.request() instanceof CallbackCall.ToolStart start) startIdentity.set(start.callId());
                if (call.request() instanceof CallbackCall.ToolInterrupted value) {
                    interrupted.set(value);
                    return CompletableFuture.supplyAsync(() -> {
                        effectCount.incrementAndGet(); effectCompleted.set(true); return call.completed();
                    }, CompletableFuture.delayedExecutor(250, TimeUnit.MILLISECONDS));
                }
                return CompletableFuture.completedFuture(call.completed());
            }));
            var toolDescriptor = ToolDescriptor.builder().name("hung_fixture").description("park until cancelled")
                    .parametersValue(Map.of("type", "object", "properties", Map.of("key", Map.of("type", "string")), "required", List.of("key")))
                    .build();
            var tool = await(sdk.registerTool("java-hung-tool", toolDescriptor, (call, cancellation) -> {
                if (call.operation().equals("tool_validate_parameters")) return CompletableFuture.completedFuture(ToolOutcome.validated());
                originalParameters.set(call.parameters()); parked.complete(null);
                return CompletableFuture.supplyAsync(() -> {
                    long until = System.nanoTime() + TimeUnit.SECONDS.toNanos(5);
                    while (!cancellation.isCancelled() && System.nanoTime() < until) java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(1));
                    if (!cancellation.isCancelled()) throw new IllegalStateException("Host did not deliver tool cancellation");
                    return ToolOutcome.error("cancelled", "cancelled", "never");
                });
            }));
            var modelCount = new AtomicInteger();
            var model = await(sdk.registerLlmClient("java-tool-model", LlmClientDescriptor.builder().modelName("tool-model").build(), (call, cancellation) -> {
                int index = modelCount.getAndIncrement();
                var message = JsonSupport.MAPPER.createObjectNode().put("role", "assistant");
                message.set("content", WireValues.value(index == 0 ? "" : "done"));
                if (index == 0) message.putArray("tool_calls").add(JsonSupport.MAPPER.createObjectNode().put("id", "model-call")
                        .put("call_type", "function").put("function_name", "hung_fixture").put("arguments", "{\"key\":\"中文🙂\"}"));
                var result = LlmChatResponse.builder().message(message).finishReason(index == 0 ? "tool_calls" : "stop").raw(Map.of()).build();
                return CompletableFuture.completedFuture(LlmOutcome.result(result));
            }));
            var agent = await(sdk.createAgent());
            var session = await(agent.createSession());
            await(session.invoke("echo_agent::agent::react::ReactAgent::set_permission_mode",List.of("full-auto")));
            var run = await(session.startChat("cancel tool"));
            parked.get(5, TimeUnit.SECONDS);
            await(run.cancel());
            var settled = await(run.waitForSettlement(Duration.ofSeconds(5)));
            assertTrue(settled.path("settled").booleanValue(), settled.toString());
            assertEquals("cancelled", settled.path("terminal").path("status").textValue(), settled.toString());
            assertTrue(effectCompleted.get(), "Run terminal must follow interrupted callback's completed side effect");
            assertEquals(1, effectCount.get());
            assertNotNull(startIdentity.get()); assertFalse(startIdentity.get().isBlank());
            var callback = interrupted.get(); assertNotNull(callback);
            assertEquals(startIdentity.get(), callback.callId()); assertEquals("hung_fixture", callback.tool());
            assertEquals(originalParameters.get(), callback.input());
            assertEquals(JsonSupport.MAPPER.valueToTree(Map.of("key", "中文🙂")), JsonSupport.fromWire(callback.input()));
            run.close(); session.close(); tool.close(); model.close(); observer.close(); agent.close();
        }
    }
}
