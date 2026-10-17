package com.echoagent.sdk;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeAll;
import static org.junit.jupiter.api.Assumptions.assumeTrue;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeUnit;
import static org.junit.jupiter.api.Assertions.*;

class ProtocolHostTest {
    @BeforeAll static void requireHostWhenGateEnabled() {
        var host=System.getenv("ECHO_AGENT_SDK_HOST");
        if("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E")))assertNotNull(host,"required Host E2E binary is missing");
        assumeTrue(host!=null&&!host.isBlank(),"real Host tests require binary");
    }
    private EchoAgentClient spawn(String mode) throws Exception {
        var work = Files.createTempDirectory("sdk-java-protocol-");
        return EchoAgentClient.spawn("python3", List.of(Path.of("../python/tests/protocol_host_proxy.py").toAbsolutePath().toString(), "--host", System.getenv("ECHO_AGENT_SDK_HOST"), "--work", work.toString(), "--mode", mode), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of(), List.of(), List.of()).toCompletableFuture().get(10, TimeUnit.SECONDS);
    }
    private static EchoAgentException failure(java.util.concurrent.CompletionStage<?> stage) {
        var thrown = assertThrows(java.util.concurrent.ExecutionException.class, () -> stage.toCompletableFuture().get(10, TimeUnit.SECONDS));
        var cause = thrown.getCause();
        while (cause instanceof java.util.concurrent.CompletionException && cause.getCause() != null) cause = cause.getCause();
        return assertInstanceOf(EchoAgentException.class, cause);
    }
    @Test void negotiationRejectsVersionAndDigest() {
        for (var mode : List.of("bad-version", "bad-digest", "bad-acp-version")) {
            var error = assertThrows(java.util.concurrent.ExecutionException.class, () -> spawn(mode));
            var typed = assertInstanceOf(EchoAgentException.class, error.getCause());
            assertEquals("extension_capability_mismatch", typed.code());
        }
    }
    @org.junit.jupiter.params.ParameterizedTest
    @org.junit.jupiter.params.provider.ValueSource(strings = {"overflow-acp-version", "overflow-extension-version", "nontext-feature", "null-capability", "empty-capabilities", "missing-limits", "duplicate-feature", "unsorted-features", "duplicate-capability", "zero-limit", "overflow-limit", "numeric-limit", "invalid-timeout"})
    void negotiationRejectsLossyVersionAndMalformedMembers(String mode) throws Exception {
        EchoAgentClient accepted = null;
        try {
            try {
                accepted = spawn(mode);
            } catch (java.util.concurrent.ExecutionException failure) {
                var error = assertInstanceOf(EchoAgentException.class, failure.getCause());
                assertEquals("extension_capability_mismatch", error.code());
                assertEquals("initialize", error.operation());
                return;
            }
            fail("malformed negotiation advertisement was accepted: " + mode);
        } finally {
            if (accepted != null) accepted.close();
        }
    }
    @Test void negotiationAcceptsRustUnicodeFeatureOrder() throws Exception {
        try (var sdk = spawn("unicode-features")) {
            assertEquals("\uE000", sdk.capability().path("features").get(0).asText());
            assertEquals("\uD800\uDC00", sdk.capability().path("features").get(1).asText());
        }
    }
    @Test void errorsPreserveHandlesAndIndependentSessions() throws Exception {
        try (var sdk = spawn("normal")) {
            var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var one = agent.createSession(null).toCompletableFuture().get(5, TimeUnit.SECONDS);
            var two = agent.createSession(null).toCompletableFuture().get(5, TimeUnit.SECONDS);
            assertNotEquals(one.wire().id(), two.wire().id());
            var stale = new WireHandle(one.wire().id(), "0", "session");
            var error = failure(sdk.request("_echo_agent/session/close", JsonSupport.MAPPER.createObjectNode().set("session", stale.toJson())));
            assertEquals("stale_handle", error.code());
            assertEquals(stale, error.handle());
            var special = Map.of("__proto__", Map.of("中文", "🙂"), "constructor", "safe");
            two.invoke("memory.store.put", List.of(List.of("protocol"), "特殊键", special)).toCompletableFuture().get(5, TimeUnit.SECONDS);
            var stored = two.invoke("memory.store.get", List.of(List.of("protocol"), "特殊键")).toCompletableFuture().get(5, TimeUnit.SECONDS);
            assertEquals(JsonSupport.MAPPER.valueToTree(special), stored.path("value"));
            one.close();
            assertEquals("echo-agent", two.invoke("echo_core::agent::Agent::name", List.of()).toCompletableFuture().get(5, TimeUnit.SECONDS).textValue());
            two.close(); agent.close();
        }
    }
    @Test void missingEnvelopeIsFailure() throws Exception {
        try (var sdk = spawn("no-value")) {
            var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var session = agent.createSession(null).toCompletableFuture().get(5, TimeUnit.SECONDS);
            assertEquals("serialization_violation", failure(session.invoke("echo_core::agent::Agent::name", List.of())).code());
            session.close();agent.close();
        }
    }
    @Test void agentCloseRetries() throws Exception {
        try (var sdk = spawn("agent-close-retry")) {
            var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
            assertEquals("extension_timeout", failure(agent.closeAsync()).code());
            agent.close();
        }
    }
    @Test void sessionCloseRetries() throws Exception {
        try (var sdk = spawn("close-retry")) {
            var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var session = agent.createSession(null).toCompletableFuture().get(5, TimeUnit.SECONDS);
            assertEquals("extension_timeout", failure(session.closeAsync()).code());
            session.close();agent.close();
        }
    }
    @Test void registrationAndRunCloseRetainLifecycleForRetry() throws Exception {
        for (var mode : List.of("unregister-retry", "run-close-retry")) {
            try (var sdk = spawn(mode)) {
                var calls = new java.util.concurrent.atomic.AtomicInteger();
                var descriptor = JsonSupport.MAPPER.createObjectNode().put("kind", "llm_client").put("descriptor_version", 1).put("model_name", "protocol-model").put("supports_streaming", false);
                var registration = sdk.registerLlmClient("protocol-model", descriptor, (call, cancel) -> {
                    calls.incrementAndGet();
                    try {
                        return java.util.concurrent.CompletableFuture.completedFuture(JsonSupport.MAPPER.readTree("{\"outcome\":\"result\",\"result\":{\"operation\":\"llm_chat\",\"value\":{\"message\":{\"role\":\"assistant\",\"content\":{\"kind\":\"string\",\"value\":\"中文🙂\"}},\"finish_reason\":\"stop\",\"raw\":{\"kind\":\"map\",\"value\":[]}}}}"));
                    } catch (java.io.IOException error) { return java.util.concurrent.CompletableFuture.failedFuture(error); }
                }).toCompletableFuture().get(5, TimeUnit.SECONDS);
                if (mode.equals("unregister-retry")) assertEquals("extension_timeout", failure(registration.unregister()).code());
                var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
                var session = agent.createSession(null).toCompletableFuture().get(5, TimeUnit.SECONDS);
                var run = session.startChat("中文🙂").toCompletableFuture().get(5, TimeUnit.SECONDS);
                if (mode.equals("run-close-retry")) {
                    assertEquals("cancellation_timeout", failure(run.closeAsync()).code());
                    run.close();
                } else {
                    assertTrue(run.waitForSettlement(java.time.Duration.ofSeconds(5)).toCompletableFuture().get(6, TimeUnit.SECONDS).path("settled").asBoolean());
                    assertTrue(calls.get() > 0);
                    run.close();
                }
                registration.close();session.close();agent.close();
            }
        }
    }
    @Test void asynchronousCallbackFailureSettlesHost() throws Exception {
        try (var sdk = spawn("normal")) {
            var descriptor = JsonSupport.MAPPER.createObjectNode().put("kind", "llm_client").put("descriptor_version", 1).put("model_name", "failing-model").put("supports_streaming", false);
            var registration = sdk.registerLlmClient("failing-model", descriptor, (call, cancel) -> java.util.concurrent.CompletableFuture.failedFuture(new IllegalStateException("callback-中文🙂"))).toCompletableFuture().get(5, TimeUnit.SECONDS);
            var agent = sdk.createAgent().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var session = agent.createSession().toCompletableFuture().get(5, TimeUnit.SECONDS);
            var run = session.startChat("trigger").toCompletableFuture().get(5, TimeUnit.SECONDS);
            var settled = run.waitForSettlement(java.time.Duration.ofSeconds(5)).toCompletableFuture().get(6, TimeUnit.SECONDS);
            assertTrue(settled.path("settled").asBoolean());
            assertEquals("failed", settled.path("terminal").path("status").asText());
            assertEquals("failed", settled.path("receipt").path("outcome").asText());
            run.close();session.close();registration.close();agent.close();
        }
    }

}
