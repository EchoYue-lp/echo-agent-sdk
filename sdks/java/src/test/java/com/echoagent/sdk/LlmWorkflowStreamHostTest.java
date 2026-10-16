package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import java.nio.file.*;
import java.time.Duration;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Native public SDK callbacks; framework and Host retain graph/stream/Run authority. */
class LlmWorkflowStreamHostTest {
    private static final Path FIXTURE = Path.of("../shared/testing/llm_workflow_stream_fixture.py").toAbsolutePath();
    @BeforeAll static void requireHost() {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank());
        assumeTrue(host != null && !host.isBlank(), "real Host required");
    }
    private static <T> T await(CompletionStage<T> result) throws Exception { return result.toCompletableFuture().get(20, TimeUnit.SECONDS); }
    private static String helper(Path work, String... args) throws Exception {
        var command = new ArrayList<>(List.of("python3", FIXTURE.toString(), "--work", work.toString())); command.addAll(List.of(args));
        var process = new ProcessBuilder(command).redirectErrorStream(true).start();
        String output = new String(process.getInputStream().readAllBytes(), java.nio.charset.StandardCharsets.UTF_8);
        assertEquals(0, process.waitFor(), output); return output;
    }
    private record Setup(Path work, JsonNode spec, EchoAgentClient sdk) {}
    private static Setup setup(String group) throws Exception {
        String configured = System.getenv("ECHO_AGENT_SDK_LLM_WORKFLOW_ROOT");
        Path root = (configured == null ? Path.of("../../target/llm-workflow-stream") : Path.of(configured)).toAbsolutePath().normalize();
        Files.createDirectories(root); Path work = Files.createTempDirectory(root, "java-" + group + "-");
        helper(work, "--prepare"); var spec = JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile());
        var sdk = await(EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"), List.of("--config", work.resolve("host.json").toString()),
                Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()));
        return new Setup(work, spec, sdk);
    }
    private static ObjectNode errorFact(EchoAgentException error) {
        var result = JsonSupport.MAPPER.createObjectNode().put("code", error.code()).put("message", error.getMessage()).put("operation", error.operation());
        result.set("details", error.details()); return result;
    }
    private static ObjectNode observedError(CompletionStage<?> operation) throws Exception {
        var wrapped = assertThrows(ExecutionException.class, () -> await(operation)); assertInstanceOf(EchoAgentException.class, wrapped.getCause());
        return errorFact((EchoAgentException) wrapped.getCause());
    }
    private static List<JsonNode> collect(SessionHandle session, WireHandle stream) throws Exception {
        var events = new ArrayList<JsonNode>();
        for (int i = 0; i < 12; i++) { var event = await(session.invoke("workflow.stream.next", List.of(stream))); events.add(event);
            if (!"item".equals(event.path("value").path("variant").textValue())) return events; }
        throw new AssertionError("workflow did not reach bounded terminal");
    }
    private static void save(Path work, JsonNode facts) throws Exception {
        JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(), facts);
        System.out.println("LLM/Workflow evidence: " + work);
    }
    private static void verify(Setup setup, String group, JsonNode facts) throws Exception { save(setup.work(), facts); System.out.println(helper(setup.work(), "--verify", group)); }

    @Test void registeredLlmFullRequestUsageFinishFailureAndCancellation() throws Exception {
        var setup = setup("llm"); var rows = JsonSupport.MAPPER.createArrayNode();
        try (var sdk = setup.sdk()) {
            var spec = setup.spec();
            for (String mode : List.of("nonstream", "stream", "missing_finish", "error", "cancel")) {
                var row = rows.addObject().put("mode", mode).put("finally", 0).put("cancel_observed", false);
                var calls = row.putArray("calls"); var sent = row.putArray("sent");
                var started = new CompletableFuture<Void>(); var producers = new CopyOnWriteArrayList<CompletableFuture<Void>>();
                var descriptor = LlmClientDescriptor.builder().modelName("fixture-model").supportsStreaming("stream".equals(mode)).build();
                var registration = await(sdk.registerLlmClient("proof-" + mode, descriptor, (call, cancellation) -> {
                    calls.addObject().put("operation", call.operation()).set("request", call.input());
                    var fact = (ObjectNode) calls.get(calls.size() - 1); fact.set("invocation_id", call.raw().path("invocation_id")); fact.set("deadline", call.raw().path("deadline")); started.complete(null);
                    return CompletableFuture.supplyAsync(() -> {
                        try {
                            if ("error".equals(mode)) throw new IllegalStateException(spec.path("failure").path("message").textValue());
                            if ("cancel".equals(mode)) {
                                long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
                                while (!cancellation.isCancelled() && System.nanoTime() < deadline) java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(1));
                                if (!cancellation.isCancelled()) throw new IllegalStateException("Host did not cancel registered LLM callback");
                                row.put("cancel_observed", true);
                            }
                            if ("stream".equals(mode)) {
                                WireHandle stream = WireHandle.fromJson(call.raw().path("stream")); var writer = sdk.streamWriter(stream);
                                producers.add(CompletableFuture.runAsync(() -> {
                                    try {
                                        int sequence = 0;
                                        for (var chunk : spec.path("llm_chunks")) { await(writer.chunk(chunk)); sent.addObject().put("event", "chunk").put("sequence", Integer.toString(++sequence)).set("value", chunk); }
                                        await(writer.complete(spec.path("llm_complete"))); sent.addObject().put("event", "complete").put("sequence", "3").set("value", spec.path("llm_complete"));
                                    } catch (Exception error) { throw new CompletionException(error); }
                                }));
                                return LlmOutcome.stream(stream);
                            }
                            var response = spec.path("response");
                            var builder = LlmChatResponse.builder().message(response.path("message")).usage(response.path("usage")).raw(response.path("raw"));
                            if (!"missing_finish".equals(mode)) builder.finishReason(response.path("finish_reason").textValue());
                            return LlmOutcome.result(builder.build());
                        } finally { row.put("finally", row.path("finally").intValue() + 1); }
                    });
                }));
                var agent = await(sdk.createAgent()); var session = await(agent.createSession());
                await(session.invoke("echo_agent::agent::react::ReactAgent::set_temperature", List.of(spec.path("temperature").doubleValue())));
                await(session.invoke("echo_agent::agent::react::ReactAgent::set_max_tokens", List.of(WireValues.u64(spec.path("max_tokens").asText()))));
                var run = await(session.startChat(spec.path("prompt").textValue()));
                if ("cancel".equals(mode)) { started.get(5, TimeUnit.SECONDS); await(run.cancel()); }
                row.set("settled", await(run.waitForSettlement(Duration.ofSeconds(15)))); row.set("usage", await(run.usage())); row.set("replay", await(run.replay("0", 512)));
                for (var producer : producers) producer.get(5, TimeUnit.SECONDS);
                if ("nonstream".equals(mode)) { var history = await(session.startChat(spec.path("history_prompt").textValue())); row.set("history_settled", await(history.waitForSettlement(Duration.ofSeconds(15)))); history.close(); }
                run.close(); session.close(); agent.close(); registration.close();
            }
        } finally { save(setup.work(), rows); }
        verify(setup, "llm", rows);
    }

    @Test void workflowExtensionActualRunStreamsCauseCancelCloseAndOwner() throws Exception {
        var setup = setup("workflow"); var facts = JsonSupport.MAPPER.createObjectNode(); var calls = facts.putArray("calls");
        var completed = new AtomicInteger(); var callbacksCompleted = new AtomicInteger(); var producerErrors = new CopyOnWriteArrayList<String>();
        var producers = new CopyOnWriteArrayList<CompletableFuture<Void>>();
        var started = new ConcurrentHashMap<String, CompletableFuture<Void>>(); var releases = new ConcurrentHashMap<String, CompletableFuture<Void>>();
        var controlTerminals = new ConcurrentHashMap<String, Integer>(); controlTerminals.put("cancel", 0); controlTerminals.put("close", 0);
        var controlItems = facts.putObject("control_items");
        try (var sdk = setup.sdk()) {
            var spec = setup.spec();
            var registration = await(sdk.registerAgentComponent("proof-workflow", AgentComponentDescriptor.workflow("proof-workflow", true), (call, cancellation) -> {
                var mode = call.input().path("call").path("input").path("input").textValue();
                calls.addObject().put("operation", call.operation()).put("component_operation", call.componentOperation()).put("input", mode).set("context", call.context());
                return CompletableFuture.supplyAsync(() -> {
                    try {
                        if ("workflow_run".equals(call.componentOperation())) return AgentComponentOutcome.result("workflow", new AgentComponentResult.WorkflowCompleted(spec.path("workflow_output")));
                        WireHandle stream = WireHandle.fromJson(call.raw().path("stream")); var writer = sdk.streamWriter(stream); started.get(mode).complete(null);
                        producers.add(CompletableFuture.runAsync(() -> {
                            try {
                                if (releases.containsKey(mode)) { await(writer.chunk(spec.path("workflow_chunks").path(0))); releases.get(mode).join(); await(writer.complete(spec.path("workflow_complete"))); controlTerminals.merge(mode, 1, Integer::sum); }
                                else if ("failure".equals(mode)) { await(writer.chunk(spec.path("workflow_error_chunk"))); await(writer.failed(spec.path("failure"))); }
                                else { for (var chunk : spec.path("workflow_chunks")) await(writer.chunk(chunk)); await(writer.complete(spec.path("workflow_complete"))); }
                            } catch (Exception error) { producerErrors.add(error.toString()); throw new CompletionException(error); }
                            finally { completed.incrementAndGet(); }
                        }));
                        return ExtensionOutcome.stream(stream);
                    } finally { callbacksCompleted.incrementAndGet(); }
                });
            }));
            var agent = await(sdk.createAgent()); var a = await(agent.createSession()); var b = await(agent.createSession());
            facts.set("run", await(a.invoke("workflow.extension.run", List.of(registration.wire(), "input🙂"))));
            for (String mode : List.of("success", "failure")) {
                started.put(mode, new CompletableFuture<>());
                WireHandle stream = WireHandle.fromJson(await(a.invoke("workflow.extension.run_stream", List.of(registration.wire(), mode))));
                if ("success".equals(mode)) { started.get(mode).get(5, TimeUnit.SECONDS); facts.put("callback_count_before_wrong_owner", calls.size()); facts.set("wrong_owner", observedError(b.invoke("workflow.stream.next", List.of(stream)))); facts.put("callback_count_after_wrong_owner", calls.size()); }
                facts.set(mode, JsonSupport.MAPPER.valueToTree(collect(a, stream)));
            }
            for (String mode : List.of("cancel", "close")) {
                started.put(mode, new CompletableFuture<>()); releases.put(mode, new CompletableFuture<>());
                WireHandle stream = WireHandle.fromJson(await(a.invoke("workflow.extension.run_stream", List.of(registration.wire(), mode))));
                started.get(mode).get(5, TimeUnit.SECONDS);
                controlItems.set(mode, await(a.invoke("workflow.stream.next", List.of(stream))));
                if ("cancel".equals(mode)) { facts.set("cancel", await(a.invoke("workflow.stream.cancel", List.of(stream)))); releases.get(mode).complete(null); facts.set("cancel_terminal", await(a.invoke("workflow.stream.next", List.of(stream)))); }
                else { var closing = a.invoke("workflow.stream.close", List.of(stream)); releases.get(mode).complete(null); facts.set("close", await(closing)); facts.set("close_again", await(a.invoke("workflow.stream.close", List.of(stream)))); facts.set("after_close", observedError(a.invoke("workflow.stream.next", List.of(stream)))); }
            }
            for (var producer : producers) producer.get(5, TimeUnit.SECONDS);
            a.close(); b.close(); agent.close(); registration.close();
        } finally {
            releases.values().forEach(release -> release.complete(null));
            facts.put("producer_completed", completed.get()).put("callbacks_completed", callbacksCompleted.get()); facts.set("control_terminals", JsonSupport.MAPPER.valueToTree(controlTerminals)); facts.set("producer_errors", JsonSupport.MAPPER.valueToTree(producerErrors)); save(setup.work(), facts);
        }
        verify(setup, "workflow", facts);
    }

    @Test void workflowObservationRetainsRealLateCompletedPayload() throws Exception {
        var setup = setup("observation"); var sdk = setup.sdk(); var spec = setup.spec();
        var facts = JsonSupport.MAPPER.createObjectNode(); var calls = facts.putArray("calls");
        var release = new CompletableFuture<Void>(); var observed = new CompletableFuture<Void>();
        var producers = new CopyOnWriteArrayList<CompletableFuture<Void>>();
        var physicalCompleted = new AtomicInteger(); var actualCompleteSent = new AtomicInteger();
        try {
            var registration = await(sdk.registerAgentComponent("observed-workflow", AgentComponentDescriptor.workflow("observed-workflow", true), (call, cancellation) -> {
                calls.addObject().put("component_operation", call.componentOperation());
                assertEquals("workflow_run_stream", call.componentOperation());
                WireHandle stream = WireHandle.fromJson(call.raw().path("stream"));
                // This producer and its identities originate in this callback,
                // independently of the RPC request and facade handle.
                var binding = facts.putObject("binding")
                        .put("run_id", "workflow-original-" + UUID.randomUUID() + "-中文🙂")
                        .put("execution_id", "workflow-attempt-" + UUID.randomUUID() + "-🧪");
                binding.set("stream", stream.toJson()); binding.set("extension", call.raw().path("extension"));
                var error = JsonSupport.MAPPER.createObjectNode()
                        .put("code", "framework_error").put("retryable", "never")
                        .put("message", "original Workflow producer remains held")
                        .put("operation", "_echo_agent/extension/invoke");
                var details = error.putObject("details").putArray("fields");
                details.addObject().put("key", "observation").put("value", "not_settled");
                details.addObject().put("key", "run_id").put("value", binding.path("run_id").textValue());
                details.addObject().put("key", "execution_id").put("value", binding.path("execution_id").textValue());
                var writer = sdk.streamWriter(stream);
                producers.add(CompletableFuture.runAsync(() -> {
                    try {
                        await(writer.failed(error)); observed.complete(null);
                        release.get(20, TimeUnit.SECONDS);
                        physicalCompleted.incrementAndGet();
                        await(writer.complete(spec.path("workflow_complete")));
                        actualCompleteSent.incrementAndGet();
                    } catch (Exception failure) { throw new CompletionException(failure); }
                }));
                return CompletableFuture.completedFuture(ExtensionOutcome.stream(stream));
            }));
            var agent = await(sdk.createAgent()); var session = await(agent.createSession());
            WireHandle stream = WireHandle.fromJson(await(session.invoke("workflow.extension.run_stream", List.of(registration.wire(), "original held work"))));
            facts.set("facade_stream", stream.toJson()); observed.get(10, TimeUnit.SECONDS);
            var wrapped = assertThrows(ExecutionException.class, () -> await(session.invoke("workflow.stream.next", List.of(stream))));
            var error = assertInstanceOf(EchoAgentException.class, wrapped.getCause());
            assertEquals("framework_error", error.code()); assertEquals("never", error.retryable());
            assertEquals("workflow.stream.next", error.operation()); assertEquals(stream, error.handle());
            var fact = errorFact(error).put("retryable", error.retryable()); fact.set("handle", error.handle().toJson()); facts.set("observed", fact);
            facts.putObject("held").put("physical_completed", physicalCompleted.get()).put("actual_complete_sent", actualCompleteSent.get());
            assertEquals(0, physicalCompleted.get()); assertEquals(0, actualCompleteSent.get()); assertEquals(1, calls.size());
            release.complete(null); for (var producer : producers) producer.get(10, TimeUnit.SECONDS);
            var item = await(session.invoke("workflow.stream.next", List.of(stream))); facts.set("item", item);
            assertEquals("item", item.path("value").path("variant").textValue(), "real Completed payload replaced by generic terminal");
            facts.set("terminal", await(session.invoke("workflow.stream.next", List.of(stream))));
            facts.set("close", await(session.invoke("workflow.stream.close", List.of(stream))));
            session.close(); agent.close(); registration.close();
        } finally {
            release.complete(null);
            try { for (var producer : producers) producer.get(10, TimeUnit.SECONDS); }
            finally {
                sdk.close(); facts.put("physical_completed", physicalCompleted.get()).put("actual_complete_sent", actualCompleteSent.get()); save(setup.work(), facts);
            }
        }
        verify(setup, "observation", facts);
    }

    @Test void pureGraphStreamRejectsForeignOwnerAndPreservesFrameworkCancelCause() throws Exception {
        var setup = setup("graph"); var facts = JsonSupport.MAPPER.createObjectNode(); var called = new AtomicInteger();
        try (var sdk = setup.sdk()) {
            var spec = setup.spec();
            var guard = await(sdk.registerLlmClient("graph-model-guard", LlmClientDescriptor.builder().modelName("fixture-model").build(), (call, cancel) -> { called.incrementAndGet(); return CompletableFuture.failedFuture(new IllegalStateException("pure router graph must not invoke model")); }));
            var agent = await(sdk.createAgent()); var a = await(agent.createSession()); var b = await(agent.createSession());
            WireHandle graph = WireHandle.fromJson(await(a.invoke("workflow.graph.build", List.of(spec.path("graph").toString()))).path("resource"));
            Object state = JsonSupport.MAPPER.convertValue(spec.path("state"), Object.class);
            facts.set("wrong_owner", observedError(b.invoke("workflow.graph.run_stream", List.of(graph, state)))); facts.set("wrong_cancel", observedError(b.invoke("workflow.graph.cancel", List.of(graph))));
            WireHandle stream = WireHandle.fromJson(await(a.invoke("workflow.graph.run_stream", List.of(graph, state)))); facts.set("events", JsonSupport.MAPPER.valueToTree(collect(a, stream)));
            facts.set("cancel", await(a.invoke("workflow.graph.cancel", List.of(graph)))); stream = WireHandle.fromJson(await(a.invoke("workflow.graph.run_stream", List.of(graph, state)))); facts.set("cancelled_events", JsonSupport.MAPPER.valueToTree(collect(a, stream)));
            a.close(); b.close(); agent.close(); guard.close();
        } finally { facts.put("callback_count", called.get()); save(setup.work(), facts); }
        verify(setup, "graph", facts);
    }
}
