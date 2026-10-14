package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Exercises only accepted EvalRunner operations against the real ACP Host. */
class FactoryHostTest {
    @BeforeAll static void requireHost() {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        boolean present = host != null && !host.isBlank();
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(present, "required Host E2E binary is missing");
        assumeTrue(present, "factory tests require real Host binary");
    }
    private static <T> T await(CompletionStage<T> stage) throws Exception {
        return stage.toCompletableFuture().get(10, TimeUnit.SECONDS);
    }
    private static JsonNode outcome(String operation, JsonNode value) {
        var result = JsonSupport.MAPPER.createObjectNode().put("operation", operation); result.set("value", value);
        return JsonSupport.MAPPER.createObjectNode().put("outcome", "result").set("result", result);
    }
    private static Object evalCase(String id) {
        var value = new java.util.LinkedHashMap<String, Object>();
        value.put("id", id); value.put("name", "case"); value.put("description", "same descriptor per-instance routing");
        value.put("domain", null); value.put("task", "return SDK"); value.put("project_fixture", null);
        value.put("success_criteria", Map.of("type", "output_contains", "substring", "SDK:"));
        value.put("constraints", Map.of()); return value;
    }
    private static void successfulReport(JsonNode report, int count) {
        assertTrue(report.path("results").isArray(), report.toString());
        assertEquals(count, report.path("results").size(), report.toString());
        report.path("results").forEach(result -> assertTrue(result.path("success").booleanValue(), result.toString()));
    }
    private static final class Instance {
        final WireHandle handle;
        final String state;
        final AtomicBoolean closed = new AtomicBoolean();
        Instance(WireHandle handle, int ordinal) { this.handle = handle; state = "SDK:" + ordinal + ":" + handle.id(); }
    }
    @Test void sameDescriptorInstancesExecuteAndCloseIndependentlyWhileParentSurvives() throws Exception {
        var work = Files.createTempDirectory("sdk-java-factory-");
        try (var sdk = await(EchoAgentClient.spawn("python3", List.of(
                Path.of("../python/tests/protocol_host_proxy.py").toAbsolutePath().toString(),
                "--host", System.getenv("ECHO_AGENT_SDK_HOST"), "--work", work.toString(), "--mode", "normal"),
                Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()))) {
            var instances = new ConcurrentHashMap<WireHandle, Instance>();
            var created = new CopyOnWriteArrayList<WireHandle>();
            var closed = new CopyOnWriteArrayList<WireHandle>();
            var executed = new ConcurrentHashMap<WireHandle, String>();
            var violations = new CopyOnWriteArrayList<String>();
            var descriptors = new CopyOnWriteArrayList<JsonNode>();
            var ordinal = new AtomicInteger();
            var parent = new AtomicReference<WireHandle>();
            var concurrent = new AtomicBoolean();
            var concurrentFirst = new AtomicReference<WireHandle>();
            var twoExecuting = new CountDownLatch(2);
            var firstClosed = new CompletableFuture<Void>();
            var coordinatedClose = new AtomicBoolean();
            var descriptor = JsonSupport.MAPPER.createObjectNode().put("kind", "agent_factory").put("descriptor_version", 1);
            var factory = await(sdk.registerAgentFactory("java-factory", descriptor, (call, cancellation) -> {
                String operation = call.path("invocation").path("operation").textValue();
                try {
                    var extension = WireHandle.fromJson(call.path("extension"));
                    if (operation.equals("factory_create_agent")) {
                        if (!extension.equals(parent.get()) || call.hasNonNull("factory_origin")) throw new IllegalStateException("constructor parent identity differs");
                        var handle = WireHandle.fromJson(call.path("invocation").path("input").path("instance"));
                        var state = new Instance(handle, ordinal.incrementAndGet());
                        if (handle.equals(extension) || !handle.generation().equals(extension.generation()) || instances.putIfAbsent(handle, state) != null) throw new IllegalStateException("child identity reused");
                        created.add(handle);
                        var sameDescriptor = JsonSupport.MAPPER.createObjectNode().put("name", "factory").put("model_name", "factory").put("system_prompt", "factory");
                        sameDescriptor.putArray("tool_names"); descriptors.add(sameDescriptor.deepCopy());
                        return CompletableFuture.completedFuture(outcome(operation, sameDescriptor));
                    }
                    if (!parent.get().equals(WireHandle.fromJson(call.path("factory_origin")))) throw new IllegalStateException("child does not route through its own parent");
                    var state = instances.get(extension);
                    if (state == null || !extension.equals(state.handle)) throw new IllegalStateException("instance object missing or crossed");
                    if (operation.equals("agent_close")) {
                        if (!state.closed.compareAndSet(false, true)) throw new IllegalStateException("duplicate object close");
                        instances.remove(extension, state); closed.add(extension);
                        if (concurrent.get() && extension.equals(concurrentFirst.get())) firstClosed.complete(null);
                        return CompletableFuture.completedFuture(outcome(operation, com.fasterxml.jackson.databind.node.NullNode.instance));
                    }
                    if (state.closed.get()) throw new IllegalStateException("executed closed instance");
                    executed.put(extension, state.state);
                    if (operation.equals("agent_execute") || operation.equals("agent_chat")) {
                        return CompletableFuture.completedFuture(outcome(operation, com.fasterxml.jackson.databind.node.TextNode.valueOf(state.state)));
                    }
                    if (!operation.equals("agent_execute_stream") && !operation.equals("agent_chat_stream")) throw new IllegalStateException("unexpected operation " + operation);
                    var stream = WireHandle.fromJson(call.path("stream"));
                    boolean coordinate = concurrent.get();
                    boolean first = coordinate && concurrentFirst.compareAndSet(null, extension);
                    if (coordinate) twoExecuting.countDown();
                    CompletableFuture.runAsync(() -> {
                        try {
                            if (coordinate) {
                                if (!twoExecuting.await(5, TimeUnit.SECONDS)) throw new IllegalStateException("two independent sessions did not execute concurrently");
                                if (!first) {
                                    firstClosed.get(5, TimeUnit.SECONDS);
                                    if (state.closed.get() || instances.get(extension) != state) throw new IllegalStateException("closing sibling destroyed live instance");
                                    coordinatedClose.set(true);
                                }
                            }
                            var terminal = JsonSupport.MAPPER.createObjectNode().put("event", "complete").put("sequence", "1");
                            terminal.set("stream", stream.toJson());
                            terminal.set("value", JsonSupport.MAPPER.createObjectNode().put("kind", "agent").set("value",
                                    JsonSupport.MAPPER.createObjectNode().put("event", "final_answer").set("data", JsonSupport.MAPPER.createObjectNode().put("text", state.state))));
                            await(sdk.notify("_echo_agent/extension/stream", terminal));
                        } catch (Exception error) { violations.add(error.toString()); }
                    });
                    return CompletableFuture.completedFuture(ExtensionOutcome.stream(stream).toJson());
                } catch (RuntimeException error) {
                    violations.add(error.toString()); return CompletableFuture.failedFuture(error);
                }
            }));
            parent.set(factory.wire());
            var agent = await(sdk.createAgent());
            var one = await(agent.createSession(work.resolve("one").toString()));
            var two = await(agent.createSession(work.resolve("two").toString()));
            var runnerOne = WireHandle.fromJson(await(sdk.call("echo_agent::eval::runner::EvalRunner::new", agent.wire(), List.of(one.wire(), WireValues.utf8Path(work.resolve("eval-one").toString())))));
            var runnerTwo = WireHandle.fromJson(await(sdk.call("echo_agent::eval::runner::EvalRunner::new", agent.wire(), List.of(two.wire(), WireValues.utf8Path(work.resolve("eval-two").toString())))));
            successfulReport(await(sdk.call("echo_agent::eval::runner::EvalRunner::run_all", agent.wire(), List.of(one.wire(), runnerOne, List.of(evalCase("one"), evalCase("two")), factory.wire()))), 2);
            assertEquals(2, closed.size()); assertTrue(instances.isEmpty());
            concurrent.set(true);
            var reportOne = sdk.call("echo_agent::eval::runner::EvalRunner::run_all", agent.wire(), List.of(one.wire(), runnerOne, List.of(evalCase("concurrent-one")), factory.wire()));
            var reportTwo = sdk.call("echo_agent::eval::runner::EvalRunner::run_all", agent.wire(), List.of(two.wire(), runnerTwo, List.of(evalCase("concurrent-two")), factory.wire()));
            successfulReport(await(reportOne), 1); successfulReport(await(reportTwo), 1);
            assertTrue(coordinatedClose.get(), "second instance must remain live after first close");
            concurrent.set(false);
            successfulReport(await(sdk.call("echo_agent::eval::runner::EvalRunner::run_all", agent.wire(), List.of(one.wire(), runnerOne, List.of(evalCase("parent-still-live")), factory.wire()))), 1);
            assertEquals(5, created.size()); assertEquals(5, new java.util.HashSet<>(created).size());
            assertEquals(new java.util.HashSet<>(created), new java.util.HashSet<>(closed)); assertEquals(5, closed.size());
            assertEquals(5, executed.size()); assertTrue(instances.isEmpty()); assertTrue(violations.isEmpty(), violations.toString());
            for (var child : created) assertTrue(executed.get(child).endsWith(child.id()), "exact instance output identity");
            for (var sameDescriptor : descriptors) assertEquals(descriptors.get(0), sameDescriptor);
            factory.close(); one.close(); two.close(); agent.close();
        }
    }
}
