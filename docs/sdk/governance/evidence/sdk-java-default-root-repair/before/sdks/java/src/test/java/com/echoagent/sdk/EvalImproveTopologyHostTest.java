package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.junit.jupiter.api.Test;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Native Java factory callbacks; shared fixture contains inputs and assertions only. */
class EvalImproveTopologyHostTest {
    static final Path FIXTURE = Path.of("../shared/testing/eval_improve_topology_fixture.py").toAbsolutePath();
    static <T> T await(CompletionStage<T> stage) throws Exception { return stage.toCompletableFuture().get(25, TimeUnit.SECONDS); }
    static ObjectNode obj() { return JsonSupport.MAPPER.createObjectNode(); }
    static Object value(JsonNode value) { return JsonSupport.MAPPER.convertValue(value, Object.class); }
    static JsonNode outcome(String operation, JsonNode value) { return obj().put("outcome", "result").set("result", obj().put("operation", operation).set("value", value == null ? JsonSupport.MAPPER.nullNode() : value)); }
    static String helper(Path work, String... args) throws Exception {
        var command = new ArrayList<>(List.of("python3", FIXTURE.toString(), "--work", work.toString())); command.addAll(List.of(args));
        var process = new ProcessBuilder(command).redirectErrorStream(true).start(); String output = new String(process.getInputStream().readAllBytes(), java.nio.charset.StandardCharsets.UTF_8);
        assertEquals(0, process.waitFor(), output); return output;
    }
    static void save(Path work, JsonNode facts) throws Exception { JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(work.resolve("observed.json").toFile(), facts); }
    static Object resolve(JsonNode input, Map<String, JsonNode> results, Path work, WireHandle factory, WireHandle custom) {
        if (input.isArray()) { var list = new ArrayList<>(); for (var item : input) list.add(resolve(item, results, work, factory, custom)); return list; }
        if (!input.isObject()) return value(input);
        if (input.has("$ref")) { JsonNode selected = results.get(input.path("$ref").textValue()); for (var field : input.path("path")) selected = selected.path(field.textValue()); return selected.isObject() && selected.has("id") && selected.has("kind") && selected.has("generation") ? WireHandle.fromJson(selected) : value(selected); }
        if (input.has("$path")) return WireValues.utf8Path(work.resolve(input.path("$path").textValue()).toString());
        if (input.has("$string_path")) return work.resolve(input.path("$string_path").textValue()).toString();
        if (input.has("$u64")) return WireValues.u64(input.path("$u64").textValue());
        if (input.has("$f64")) return input.path("$f64").doubleValue();
        if (input.has("$factory")) return factory;
        if (input.has("$custom")) return custom;
        var result = new LinkedHashMap<String, Object>(); input.fields().forEachRemaining(item -> result.put(item.getKey(), resolve(item.getValue(), results, work, factory, custom))); return result;
    }
    static List<String> bytes(Path path) throws Exception {
        try (var files = Files.list(path)) { var paths = files.filter(file -> file.toString().endsWith(".jsonl")).sorted().toList(); var result = new ArrayList<String>(); for (var file : paths) result.add(Base64.getEncoder().encodeToString(Files.readAllBytes(file))); return result; }
    }
    @Test void evalNativeFactoryTimeoutGraderAndConstraints() throws Exception { scenario("eval"); }
    @Test void improvementEarlyMaximumTrajectoryReopenAndFailedWrite() throws Exception { scenario("improve"); }
    @Test void topologyOriginalTrackerMetadataExportsOwnerAndClear() throws Exception { scenario("topology"); }
    static void scenario(String group) throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST"); if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank()); assumeTrue(host != null && !host.isBlank(), "real Host required");
        String configured = System.getenv("ECHO_AGENT_SDK_ANALYSIS_ROOT"); Path root = configured == null ? Path.of("../../target/analysis-native") : Path.of(configured); Files.createDirectories(root); Path work = Files.createTempDirectory(root, "java-" + group + "-"); helper(work, "--prepare", group);
        JsonNode spec = JsonSupport.MAPPER.readTree(work.resolve("scenario.json").toFile()); var facts = obj().put("language", "java").put("group", group); var rows = facts.putArray("rows");
        var created = new CopyOnWriteArrayList<String>(); var closed = new CopyOnWriteArrayList<String>(); var executed = new CopyOnWriteArrayList<String>(); var callbacks = new CopyOnWriteArrayList<JsonNode>(); var instances = new ConcurrentHashMap<String, JsonNode>(); var finalies = new AtomicInteger(); var cancellations = new AtomicInteger(); var mode = new AtomicReference<>("plain"); var parent = new AtomicReference<WireHandle>(); var deadlineCancelled = new CompletableFuture<Void>(); var deadlineRelease = new CompletableFuture<Void>();
        try (var sdk = await(EchoAgentClient.spawn(host, List.of("--config", work.resolve("host.json").toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of("TMPDIR", work.resolve("runtime-temp").toString())))) {
            var factory = await(sdk.registerAgentFactory("analysis-factory", obj().put("kind", "agent_factory").put("descriptor_version", 1), (call, cancel) -> CompletableFuture.supplyAsync(() -> {
                String operation = call.path("invocation").path("operation").textValue(); callbacks.add(call.deepCopy());
                if ("factory_create_agent".equals(operation)) { var instance = call.path("invocation").path("input").path("instance"); String id = instance.path("id").textValue(); if (instances.putIfAbsent(id, instance) != null) throw new IllegalStateException("factory child reused"); created.add(id); return outcome(operation, obj().put("name", "native-case").put("model_name", "native-case").put("system_prompt", "native-case").set("tool_names", JsonSupport.MAPPER.createArrayNode())); }
                assertEquals(parent.get().toJson(), call.path("factory_origin")); String id = call.path("extension").path("id").textValue(); assertTrue(instances.containsKey(id));
                if ("agent_close".equals(operation)) { closed.add(id); instances.remove(id); return outcome(operation, null); }
                executed.add(id);
                try {
                    var input = call.path("invocation").path("input"); String task = input.has("task") ? input.path("task").textValue() : input.path("message").textValue();
                    if ("hang".equals(task)) { long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(15); while (!cancel.isCancelled() && System.nanoTime() < deadline) java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(1)); if (!cancel.isCancelled()) throw new IllegalStateException("native factory callback never received cancellation"); cancellations.incrementAndGet(); deadlineCancelled.complete(null); deadlineRelease.get(10, TimeUnit.SECONDS); return outcome(operation, JsonSupport.MAPPER.getNodeFactory().textNode("cancelled")); }
                    String answer = "bad".equals(task) ? "wrong response" : spec.path("factory_answer").textValue();
                    if (operation.endsWith("_stream")) { var stream = WireHandle.fromJson(call.path("stream")); var terminal = obj().put("kind", "agent").set("value", obj().put("event", "final_answer").set("data", obj().put("text", answer))); await(sdk.streamWriter(stream).complete(terminal)); return ExtensionOutcome.stream(stream).toJson(); }
                    return outcome(operation, JsonSupport.MAPPER.getNodeFactory().textNode(answer));
                } catch (Exception error) { throw new CompletionException(error); }
                finally { finalies.incrementAndGet(); }
            })));
            parent.set(factory.wire());
            var custom = await(sdk.registerCustomAgent("analysis-custom", obj().put("kind", "custom_agent").put("descriptor_version", 1).put("name", "analysis-custom").put("model_name", "analysis-custom").put("system_prompt", "analysis-custom").set("tool_names", JsonSupport.MAPPER.createArrayNode()), (call, cancel) -> CompletableFuture.supplyAsync(() -> {
                String operation = call.path("invocation").path("operation").textValue(); callbacks.add(call.deepCopy()); if ("agent_close".equals(operation)) return outcome(operation, null);
                String answer = "plain".equals(mode.get()) ? spec.path("factory_answer").textValue() : "grade".equals(mode.get()) ? spec.path("grade_good").toString() : "not valid JSON🙂";
                try { if (operation.endsWith("_stream")) { var stream = WireHandle.fromJson(call.path("stream")); await(sdk.streamWriter(stream).complete(obj().put("kind", "agent").set("value", obj().put("event", "final_answer").set("data", obj().put("text", answer))))); return ExtensionOutcome.stream(stream).toJson(); } return outcome(operation, JsonSupport.MAPPER.getNodeFactory().textNode(answer)); }
                catch (Exception error) { throw new CompletionException(error); }
            })));
            var agent = await(sdk.createAgent()); Map<String, SessionHandle> sessions = Map.of("a", await(agent.createSession()), "b", await(agent.createSession())); Map<String, JsonNode> values = new HashMap<>();
            for (var step : spec.path("steps")) {
                ObjectNode row = rows.addObject().put("id", step.path("id").textValue()); String action = step.path("action").asText("");
                if ("custom_mode".equals(action)) { mode.set(step.path("mode").textValue()); row.put("outcome", "result").put("value", mode.get()); }
                else if ("block_directory".equals(action) || "restore_directory".equals(action)) { Path path = work.resolve(step.path("directory").textValue()), backup = path.resolveSibling(path.getFileName() + ".retained"); if ("block_directory".equals(action)) { facts.set("trajectory_bytes_before", JsonSupport.MAPPER.valueToTree(bytes(path))); Files.move(path, backup); Files.writeString(path, "blocks write"); } else { Files.delete(path); Files.move(backup, path); facts.set("trajectory_bytes_after", JsonSupport.MAPPER.valueToTree(bytes(path))); } row.put("outcome", "result").put("value", true); }
                else {
                    var session = sessions.get(step.path("session").asText("a")); @SuppressWarnings("unchecked") List<Object> arguments = (List<Object>) resolve(step.path("arguments"), values, work, factory.wire(), custom.wire());
                    try {
                        JsonNode value;
                        if ("facade.resource.close".equals(step.path("operation").textValue())) { String operation = step.path("operation").textValue(); var request = obj().put("operation", operation).put("signature_digest", sdk.catalog().resolve(operation).signature()); request.set("handle", session.wire().toJson()); request.putArray("arguments").add(WireValues.value(arguments.get(0))); value = await(sdk.request("_echo_agent/facade/invoke", request)); }
                        else {
                            var request = session.invoke(step.path("operation").textValue(), arguments).toCompletableFuture();
                            if ("timeout".equals(step.path("id").textValue())) {
                                CompletableFuture.anyOf(request, deadlineCancelled).get(16, TimeUnit.SECONDS);
                                facts.put("deadline_waiter_pending_during_cleanup", deadlineCancelled.isDone() && !request.isDone());
                                try (var children = Files.list(work.resolve("eval-configured"))) { facts.put("workspace_exists_during_cleanup", children.anyMatch(child -> child.getFileName().toString().startsWith("eval-"))); }
                                deadlineRelease.complete(null);
                            }
                            value = await(request);
                        }
                        row.put("outcome", "result").set("value", value); values.put(step.path("id").textValue(), value);
                    } catch (ExecutionException error) { Throwable cause = error.getCause(); while (cause instanceof CompletionException && cause.getCause() != null) cause = cause.getCause(); var typed = assertInstanceOf(EchoAgentException.class, cause); row.put("outcome", "error").put("code", typed.code()).put("message", typed.getMessage()).put("operation", typed.operation()).set("details", typed.details()); }
                }
                save(work, facts);
            }
            assertTrue(instances.isEmpty()); sessions.get("a").close(); sessions.get("b").close(); agent.close(); custom.close(); factory.close();
        } finally { deadlineRelease.complete(null); facts.set("created", JsonSupport.MAPPER.valueToTree(created)); facts.set("closed", JsonSupport.MAPPER.valueToTree(closed)); facts.set("executed", JsonSupport.MAPPER.valueToTree(executed)); facts.set("callbacks", JsonSupport.MAPPER.valueToTree(callbacks)); facts.put("factory_finally", finalies.get()).put("cancel_observed", cancellations.get()); save(work, facts); System.out.println("Analysis native evidence: " + work); }
        System.out.println(helper(work, "--verify"));
    }
}
