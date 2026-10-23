package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.junit.jupiter.api.Test;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.Flow;
import java.util.concurrent.TimeUnit;

import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Real original Host; no synthesized business terminal or replacement delivery. */
class SessionParentCloseHostTest {
    private static <T> T await(CompletionStage<T> value) throws Exception {
        return value.toCompletableFuture().get(5, TimeUnit.SECONDS);
    }

    private static CompletableFuture<List<JsonNode>> collect(Flow.Publisher<JsonNode> publisher) {
        var complete = new CompletableFuture<List<JsonNode>>();
        var values = new CopyOnWriteArrayList<JsonNode>();
        publisher.subscribe(new Flow.Subscriber<>() {
            @Override public void onSubscribe(Flow.Subscription subscription) { subscription.request(Long.MAX_VALUE); }
            @Override public void onNext(JsonNode value) { values.add(value); }
            @Override public void onError(Throwable error) { complete.completeExceptionally(error); }
            @Override public void onComplete() { complete.complete(List.copyOf(values)); }
        });
        return complete;
    }

    @Test void ackPausedParentCloseEndsBufferedAndFirstSubscriptionsAndPreservesOtherSession() throws Exception {
        for (var window : List.of(1, 128)) exercise(window);
    }

    private void exercise(int window) throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank(), "required parent-close Host missing");
        assumeTrue(host != null && !host.isBlank(), "real Host required");
        Path work = Files.createTempDirectory("echo-sdk-parent-java-");
        Path output = work.resolve("fixture.out"), error = work.resolve("fixture.err");
        var helper = new ProcessBuilder("python3", Path.of("../shared/testing/session_run_idempotency_fixture.py").toAbsolutePath().toString(), "--work", work.toString())
                .redirectOutput(output.toFile()).redirectError(error.toFile()).start();
        try {
            assertTrue(helper.waitFor(5, TimeUnit.SECONDS));
            assertEquals(0, helper.exitValue(), Files.readString(error));
        } finally { if (helper.isAlive()) { helper.destroyForcibly(); assertTrue(helper.waitFor(5, TimeUnit.SECONDS)); } }
        var config = (ObjectNode) JsonSupport.MAPPER.readTree(work.resolve("host.json").toFile());
        if (window == 1) ((ObjectNode) config.path("sdk_profile")).putObject("limits").put("max_outstanding_live_events", 1);
        Files.writeString(work.resolve("host.json"), config.toString());
        var spec = JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());
        try (var sdk = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()),
                Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()))) {
            var registration = await(sdk.registerLlmClient("parent-close-native", spec.path("descriptor"), (call, cancellation) -> {
                var result = JsonSupport.MAPPER.createObjectNode().put("operation", "llm_chat");
                result.set("value", spec.path("response").deepCopy());
                var outcome = JsonSupport.MAPPER.createObjectNode().put("outcome", "result");
                outcome.set("result", result);
                return CompletableFuture.completedFuture(outcome);
            }));
            var agent = await(sdk.createAgent());
            var parent = await(agent.createSession());
            var independent = await(agent.createSession());
            var run = await(parent.startChat("paused child 中文🌊"));
            var settled = await(run.waitForSettlement());
            assertTrue(settled.path("settled").asBoolean());
            assertEquals("completed", settled.path("terminal").path("status").asText());
            await(parent.closeAsync());
            // The first subscriptions occur only after the parent close reply.
            var events = await(collect(run.events()));
            await(collect(parent.updates()));
            assertFalse(events.isEmpty(), "buffered Host facts must be retained");
            if (window == 1) assertTrue(events.stream().anyMatch(value -> value.has("gap")), "real ACK pause required");
            assertTrue(events.stream().filter(value -> List.of("final_answer", "cancelled", "error").contains(value.path("envelope").path("payload").path("event_type").asText())).count() <= 1);
            var closed = assertThrows(java.util.concurrent.ExecutionException.class, () -> await(run.get()));
            assertEquals("stale_handle", EchoAgentException.from(closed, "get").code());
            var params = JsonSupport.MAPPER.createObjectNode(); params.set("run", run.wire().toJson());
            var hostClosed = assertThrows(java.util.concurrent.ExecutionException.class, () -> await(sdk.request("_echo_agent/run/get", params)));
            assertEquals("closed_handle", EchoAgentException.from(hostClosed, "get").code());
            await(run.closeAsync());
            var other = await(independent.startChat("independent Session still works"));
            var otherSettled = await(other.waitForSettlement());
            assertTrue(otherSettled.path("settled").asBoolean());
            assertEquals("completed", otherSettled.path("terminal").path("status").asText());
            await(independent.closeAsync());
            await(agent.closeAsync());
            await(registration.unregister());
            var observed = JsonSupport.MAPPER.createObjectNode().put("window", window);
            observed.set("events", JsonSupport.MAPPER.valueToTree(events));
            observed.set("independent", otherSettled);
            Files.writeString(work.resolve("observed.json"), observed.toPrettyString());
            System.out.println("parent-close real Host evidence: " + work);
        }
    }
}
