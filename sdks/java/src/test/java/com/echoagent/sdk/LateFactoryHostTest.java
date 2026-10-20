package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import java.util.function.BooleanSupplier;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Real constructor cancellation must not manufacture completion of a still-running user job. */
class LateFactoryHostTest {
    private static <T> T await(CompletionStage<T> value) throws Exception { return value.toCompletableFuture().get(15, TimeUnit.SECONDS); }
    private static void until(BooleanSupplier condition) throws Exception {
        long end = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
        while (!condition.getAsBoolean()) {
            if (System.nanoTime() > end) throw new TimeoutException("late factory lifecycle barrier not reached");
            java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(5));
        }
    }
    private static JsonNode outcome(String operation, JsonNode value) {
        var result = JsonSupport.MAPPER.createObjectNode().put("operation", operation); result.set("value", value);
        return JsonSupport.MAPPER.createObjectNode().put("outcome", "result").set("result", result);
    }
    private static List<JsonNode> frames(Path work) { return frames(work, "frames.jsonl"); }
    private static List<JsonNode> frames(Path work, String name) {
        try {
            var result = new ArrayList<JsonNode>();
            Path file = work.resolve(name); if (!Files.exists(file)) return result;
            for (var line : Files.readAllLines(file)) {
                try { result.add(JsonSupport.MAPPER.readTree(line)); }
                catch (java.io.IOException ignored) { /* The final trace line may still be in flight. */ }
            }
            return result;
        } catch (java.io.IOException error) { throw new CompletionException(error); }
    }
    private static long cleanupRequests(Path work, WireHandle child) {
        if (child == null) return 0;
        return frames(work).stream().filter(frame -> "_echo_agent/extension/invoke".equals(frame.path("method").textValue())
                && "agent_close".equals(frame.path("params").path("invocation").path("operation").textValue())
                && child.id().equals(frame.path("params").path("extension").path("id").textValue())).count();
    }
    private static Object evalCase() {
        var value = new LinkedHashMap<String, Object>();
        value.put("id", "late-constructor"); value.put("name", "case"); value.put("description", "constructor survives timeout");
        value.put("domain", null); value.put("task", "return SDK"); value.put("project_fixture", null);
        value.put("success_criteria", Map.of("type", "output_contains", "substring", "SDK")); value.put("constraints", Map.of());
        return value;
    }
    @Test void timedOutConstructorKeepsExactCleanupOwnerUntilItsActualFutureSettles() throws Exception {
        verifyLateConstructor(false);
    }
    @Test void heldCleanupKeepsExactOwnerUntilItsActualFutureSettles() throws Exception {
        verifyLateConstructor(true);
    }
    private void verifyLateConstructor(boolean holdCleanup) throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean provided = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(provided, "required Host E2E binary is missing");
        assumeTrue(provided, "late factory needs real Host");
        var work = Files.createTempDirectory("sdk-java-late-factory-");
        System.out.println("late factory lifecycle evidence " + work);
        var release = new CompletableFuture<Void>();
        var cleanupRelease = new CompletableFuture<Void>();
        var cleanupCalls = new AtomicInteger();
        var cleanupCancellation = new AtomicReference<ExtensionCancellation>();
        var cleanupJob = new AtomicReference<CompletableFuture<JsonNode>>();
        try (var sdk = await(EchoAgentClient.spawn("python3", List.of(Path.of("../python/tests/protocol_host_proxy.py").toAbsolutePath().toString(),
                "--host", host, "--work", work.toString(), "--mode", "normal"), Path.of("../shared/facade-operation-catalog.json"),
                Path.of("../shared/contract-digests.json"), Map.of()))) {
            var parent = new AtomicReference<WireHandle>(); var child = new AtomicReference<WireHandle>();
            var userCancellation = new AtomicReference<ExtensionCancellation>(); var job = new AtomicReference<CompletableFuture<JsonNode>>();
            var objects = new ConcurrentHashMap<WireHandle, Object>(); var constructed = new CopyOnWriteArrayList<WireHandle>();
            var closed = new CopyOnWriteArrayList<WireHandle>(); var executed = new AtomicInteger(); var violations = new CopyOnWriteArrayList<String>();
            var descriptor = JsonSupport.MAPPER.createObjectNode().put("kind", "agent_factory").put("descriptor_version", 1);
            var factory = await(sdk.registerExtension("agent_factory", "java-late-factory", descriptor, (call, cancellation) -> {
                String operation = call.path("invocation").path("operation").textValue();
                try {
                    WireHandle extension = WireHandle.fromJson(call.path("extension"));
                    if (operation.equals("factory_create_agent")) {
                        if (!extension.equals(parent.get())) throw new IllegalStateException("constructor parent mismatch");
                        var reserved = WireHandle.fromJson(call.path("invocation").path("input").path("instance"));
                        if (!child.compareAndSet(null, reserved)) throw new IllegalStateException("unexpected replacement constructor");
                        userCancellation.set(cancellation);
                        var construction = release.thenApply(ignored -> {
                            if (objects.putIfAbsent(reserved, new Object()) != null) throw new IllegalStateException("object already constructed");
                            constructed.add(reserved);
                            var value = JsonSupport.MAPPER.createObjectNode().put("name", "factory").put("model_name", "factory").put("system_prompt", "factory");
                            value.putArray("tool_names"); return outcome("factory_create_agent", value);
                        });
                        job.set(construction); return construction;
                    }
                    if (!operation.equals("agent_close")) { executed.incrementAndGet(); throw new IllegalStateException("timed-out object must never execute"); }
                    if (!extension.equals(child.get()) || !parent.get().equals(WireHandle.fromJson(call.path("factory_origin")))) throw new IllegalStateException("cleanup identity mismatch");
                    cleanupCancellation.set(cancellation);
                    cleanupCalls.incrementAndGet();
                    if (objects.remove(extension) == null) throw new IllegalStateException("cleanup ran before actual object construction or ran twice");
                    if (holdCleanup) {
                        var original = cleanupRelease.thenApply(ignored -> {
                            closed.add(extension);
                            return outcome("agent_close", com.fasterxml.jackson.databind.node.NullNode.instance);
                        });
                        cleanupJob.set(original); return original;
                    }
                    closed.add(extension); return CompletableFuture.completedFuture(outcome("agent_close", com.fasterxml.jackson.databind.node.NullNode.instance));
                } catch (RuntimeException error) { violations.add(error.toString()); return CompletableFuture.failedFuture(error); }
            }, Duration.ofMillis(100)));
            parent.set(factory.wire());
            var agent = await(sdk.createAgent()); var session = await(agent.createSession(work.toString()));
            var runner = WireHandle.fromJson(await(sdk.call("echo_agent::eval::runner::EvalRunner::new", agent.wire(), List.of(session.wire(), WireValues.utf8Path(work.resolve("eval").toString())))));
            var report = sdk.call("echo_agent::eval::runner::EvalRunner::run_all", agent.wire(), List.of(session.wire(), runner, List.of(evalCase()), factory.wire()));
            until(() -> userCancellation.get() != null && userCancellation.get().isCancelled());
            assertEquals(0, cleanupRequests(work, child.get()), "a live constructor cannot be cleaned before its original response");
            assertFalse(report.toCompletableFuture().isDone(), "unresolved construction cannot manufacture an Eval result");
            assertNotNull(job.get()); assertFalse(job.get().isDone()); assertFalse(job.get().isCancelled());
            assertTrue(objects.isEmpty()); assertTrue(constructed.isEmpty()); assertTrue(closed.isEmpty());
            assertTrue(violations.isEmpty(), "ACP cancellation must not release cleanup into an unconstructed object: " + violations);
            var unregister = assertThrows(ExecutionException.class, () -> await(factory.unregister()));
            assertEquals("extension_conflict", EchoAgentException.from(unregister, "_echo_agent/extension/unregister").code(), "pending child retains parent cleanup ownership");
            release.complete(null);
            if (holdCleanup) {
                until(() -> cleanupCalls.get() > 0);
                until(() -> {
                    var trace = frames(work);
                    var originalClose = trace.stream().filter(frame -> "_echo_agent/extension/invoke".equals(frame.path("method").textValue())
                            && "agent_close".equals(frame.path("params").path("invocation").path("operation").textValue())
                            && child.get().id().equals(frame.path("params").path("extension").path("id").textValue())).findFirst().orElse(null);
                    return originalClose != null && trace.stream().anyMatch(frame -> "_echo_agent/extension/cancel".equals(frame.path("method").textValue())
                            && frame.path("params").path("invocation_id").equals(originalClose.path("params").path("invocation_id"))
                            && "timeout".equals(frame.path("params").path("reason").textValue()));
                });
                assertNotNull(cleanupJob.get()); assertFalse(cleanupJob.get().isDone()); assertFalse(cleanupJob.get().isCancelled());
                assertNotNull(cleanupCancellation.get()); assertFalse(cleanupCancellation.get().isCancelled(), "individual request deadlines do not cancel the original cleanup owner");
                assertEquals(1, cleanupCalls.get(), "the original pending cleanup executes once");
                assertTrue(closed.isEmpty(), "cleanup outcome is still pending");
                assertFalse(cleanupRelease.isCancelled(), "ACP timeout cannot cancel the actual cleanup owner");
                assertTrue(violations.isEmpty(), violations.toString());
                var pendingParent = assertThrows(ExecutionException.class, () -> await(factory.unregister()));
                assertEquals("extension_conflict", EchoAgentException.from(pendingParent, "_echo_agent/extension/unregister").code());
                cleanupRelease.complete(null);
            }
            until(() -> closed.size() == 1);
            assertTrue(job.get().isDone()); assertFalse(job.get().isCancelled());
            assertEquals(List.of(child.get()), constructed); assertEquals(constructed, closed); assertTrue(objects.isEmpty());
            assertEquals(0, executed.get()); assertTrue(violations.isEmpty(), violations.toString());
            var result = await(report); assertEquals(1, result.path("results").size());
            assertTrue(result.path("results").path(0).path("success").isBoolean());
            assertFalse(result.path("results").path(0).path("success").booleanValue(), "timeout must not publish successful Agent construction");
            var trace = frames(work);
            var original = trace.stream().filter(frame -> "_echo_agent/extension/invoke".equals(frame.path("method").textValue())
                    && "factory_create_agent".equals(frame.path("params").path("invocation").path("operation").textValue())
                    && child.get().id().equals(frame.path("params").path("invocation").path("input").path("instance").path("id").textValue())).findFirst().orElse(null);
            assertNotNull(original);
            var replies = frames(work, "client-frames.jsonl").stream().filter(frame -> frame.path("id").equals(original.path("id")) && (frame.has("result") || frame.has("error"))).toList();
            assertEquals(1, replies.size()); var reply = replies.get(0); assertFalse(reply.has("error"));
            assertEquals("error", reply.path("result").path("outcome").textValue());
            assertEquals("cancelled", reply.path("result").path("error").path("code").textValue());
            until(() -> {
                try { return await(factory.unregister()).path("released").asBoolean(); }
                catch (Exception error) { return false; }
            });
            session.close(); agent.close();
        } finally { release.complete(null); cleanupRelease.complete(null); }
    }
}
