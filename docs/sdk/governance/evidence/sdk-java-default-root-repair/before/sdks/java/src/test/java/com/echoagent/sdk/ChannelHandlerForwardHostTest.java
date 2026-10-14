package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import org.junit.jupiter.api.*;
import java.nio.file.*;
import java.security.MessageDigest;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

/** Native language callbacks are awaited through their real finally; no simulated Host/backend. */
class ChannelHandlerForwardHostTest {
    static final Path SHARED = Path.of("../shared/testing").toAbsolutePath();
    static JsonNode spec;
    static Path evidenceRoot;
    static final List<JsonNode> scenarios = new CopyOnWriteArrayList<>();
    @BeforeAll static void initialize() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertTrue(host != null && !host.isBlank(), "new-contract Host required");
        assumeTrue(host != null && !host.isBlank(), "new-contract real Host required");
        spec = JsonSupport.MAPPER.readTree(SHARED.resolve("channel_forward_scenarios.json").toFile());
        String configured = System.getenv("ECHO_AGENT_SDK_CHANNEL_FORWARD_ROOT");
        Path root = configured == null ? Path.of("../../target/channel-forward") : Path.of(configured);
        Files.createDirectories(root); evidenceRoot = Files.createTempDirectory(root, "java-channel-forward-");
    }
    static String sha(Path path) throws Exception { var digest = MessageDigest.getInstance("SHA-256"); try (var stream = Files.newInputStream(path)) { byte[] buffer = new byte[1024 * 1024]; int count; while ((count = stream.read(buffer)) != -1) digest.update(buffer, 0, count); } return HexFormat.of().formatHex(digest.digest()); }
    @AfterAll static void receipt() throws Exception {
        if (evidenceRoot == null) return;
        var result = obj().put("language", "java").put("host_sha256", sha(Path.of(System.getenv("ECHO_AGENT_SDK_HOST"))));
        var sources = result.putObject("source_hashes");
        for (var path : List.of(Path.of("src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java"), SHARED.resolve("channel_forward_scenarios.json"))) sources.put(path.toAbsolutePath().toString(), sha(path));
        result.set("scenarios", tree(scenarios)); write(evidenceRoot.resolve("receipt.json"), result);
        String exported = System.getenv("ECHO_AGENT_SDK_CHANNEL_FORWARD_EVIDENCE"); if (exported != null) { Files.createDirectories(Path.of(exported)); write(Path.of(exported).resolve("java.json"), result); }
        System.out.println("Channel forward Java receipt: " + evidenceRoot.resolve("receipt.json"));
    }
    static ObjectNode obj() { return JsonSupport.MAPPER.createObjectNode(); }
    static JsonNode tree(Object value) { return JsonSupport.MAPPER.valueToTree(value); }
    static Object nativeValue(JsonNode value) { return JsonSupport.MAPPER.convertValue(value, Object.class); }
    static <T> T await(CompletionStage<T> value) throws Exception { return value.toCompletableFuture().get(15, TimeUnit.SECONDS); }
    static void write(Path path, JsonNode value) throws Exception { JsonSupport.MAPPER.writerWithDefaultPrettyPrinter().writeValue(path.toFile(), value); }
    static JsonNode result(String operation, JsonNode value) { return obj().put("outcome", "result").set("result", obj().put("operation", operation).set("value", value == null ? JsonSupport.MAPPER.nullNode() : value)); }
    static JsonNode errorOutcome(String mode) { return obj().put("outcome", "error").set("error", spec.path("callback_behaviors").path(mode).path("error")); }
    static ObjectNode normalize(JsonNode value) { var out = (ObjectNode) value.deepCopy(); if (!out.has("reply_to")) out.putNull("reply_to"); if (!out.has("attachments")) out.putArray("attachments"); for (var attachment : out.path("attachments")) if (!attachment.has("filename")) ((ObjectNode) attachment).putNull("filename"); return out; }
    static String variant(JsonNode value) { return value.path("value").path("variant").textValue(); }
    static Map<String, JsonNode> fields(JsonNode value) { var fields = new LinkedHashMap<String, JsonNode>(); for (var field : value.path("value").path("fields")) fields.put(field.path("name").textValue(), JsonSupport.fromWire(field.path("value"))); return fields; }
    static JsonNode failure(CompletionStage<?> value, String code) throws Exception {
        var caught = assertThrows(ExecutionException.class, () -> await(value)); Throwable cause = caught.getCause(); while (cause instanceof CompletionException && cause.getCause() != null) cause = cause.getCause();
        var error = assertInstanceOf(EchoAgentException.class, cause); assertEquals(code, error.code(), error.getMessage());
        var result = obj().put("code", error.code()).put("message", error.getMessage()).put("operation", error.operation()); result.set("details", error.details()); return result;
    }
    static void cancelled(ExtensionCancellation cancellation) {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
        while (!cancellation.isCancelled() && System.nanoTime() < deadline) java.util.concurrent.locks.LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(1));
        if (!cancellation.isCancelled()) throw new IllegalStateException("original callback never received cancellation");
    }
    record Owner(SessionHandle session, WireHandle manager, String channel, ExtensionRegistration handler, ExtensionRegistration plugin) {}
    static final class Native implements AutoCloseable {
        final Path work;
        final EchoAgentClient sdk;
        final AgentHandle agent;
        final List<SessionHandle> sessions = new ArrayList<>();
        final List<JsonNode> calls = new CopyOnWriteArrayList<>(), pluginCalls = new CopyOnWriteArrayList<>(), metadata = new CopyOnWriteArrayList<>();
        final AtomicInteger finalies = new AtomicInteger();
        final Map<String, CompletableFuture<Void>> started = new ConcurrentHashMap<>(), cancellationObserved = new ConcurrentHashMap<>(), release = new ConcurrentHashMap<>();
        final Set<String> modes = ConcurrentHashMap.newKeySet();
        final AtomicBoolean startFail = new AtomicBoolean(), startProbe = new AtomicBoolean();
        Native(String name) throws Exception {
            work = evidenceRoot.resolve(name); Files.createDirectories(work);
            var prep = new ProcessBuilder("python3", SHARED.resolve("mcp_channel_config_fixture.py").toString(), work.toString()).inheritIO().start(); assertEquals(0, prep.waitFor());
            var path = work.resolve("host.json"); var config = (ObjectNode) JsonSupport.MAPPER.readTree(path.toFile()); ((ObjectNode) config.path("sdk_profile").path("limits")).put("shutdown_timeout_secs", 5); write(path, config);
            sdk = await(EchoAgentClient.spawn(System.getenv("ECHO_AGENT_SDK_HOST"), List.of("--config", path.toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()));
            agent = await(sdk.createAgent()); sessions.add(await(agent.createSession())); sessions.add(await(agent.createSession()));
        }
        CompletionStage<JsonNode> call(Owner owner, String operation, Object... args) { return sdk.call(operation, owner.session().wire(), Arrays.asList(args)); }
        CompletionStage<JsonNode> message(Owner owner, String operation, JsonNode message) { return call(owner, operation, owner.manager(), owner.channel(), nativeValue(message)); }
        Owner register(SessionHandle session, WireHandle manager) throws Exception { return register(session, manager, spec.path("outbound").path("text").textValue()); }
        Owner register(SessionHandle session, WireHandle manager, String answer) throws Exception {
            String channel = spec.path("channel_id").textValue(), handlerId = session == sessions.get(0) ? spec.path("handler_id").textValue() : "second-forward-handler";
            var handler = await(sdk.registerChannelMessageHandler(handlerId, obj().put("kind", "channel_message_handler").put("descriptor_version", 1).put("handler_id", handlerId), (callback, cancel) -> CompletableFuture.supplyAsync(() -> {
                var invocation = callback.path("invocation"); String operation = invocation.path("operation").textValue(); var input = invocation.path("input").path("message");
                String mode = input.has("message_id") ? input.path("message_id").textValue() : input.path("text").textValue();
                var fact = obj().put("operation", operation); fact.set("message", input); fact.set("extension", callback.path("extension")); fact.set("invocation_id", callback.path("invocation_id")); fact.set("deadline", callback.path("deadline")); calls.add(fact);
                try {
                    var ids = await(sdk.call("channels.manager.ids", session.wire(), List.of(manager))); var listing = await(sdk.call("channels.manager.list", session.wire(), List.of(manager)));
                    var meta = obj(); meta.set("ids", ids); meta.set("list", listing); metadata.add(meta);
                    if (started.containsKey(mode)) started.get(mode).complete(null);
                    if (modes.contains(mode)) {
                        if ("full".equals(mode)) { var writer = sdk.streamWriter(WireHandle.fromJson(callback.path("stream"))); for (var value : spec.path("stream_items")) await(writer.chunk(obj().put("kind", "channel").set("value", value))); }
                        cancelled(cancel); cancellationObserved.get(mode).complete(null); release.get(mode).get(10, TimeUnit.SECONDS);
                        return "channel_handle".equals(operation) ? result(operation, spec.path("outbound")) : obj().put("outcome", "stream").set("stream", callback.path("stream"));
                    }
                    if ("error".equals(mode) || "handle-error".equals(mode)) return errorOutcome("channel_handle".equals(operation) ? "fail-handle" : "fail-reply");
                    if ("channel_reply".equals(operation)) return result(operation, null);
                    if ("channel_handle".equals(operation)) { var outbound = (ObjectNode) spec.path("outbound").deepCopy(); outbound.put("text", answer); return result(operation, outbound); }
                    assertEquals("channel_handle_stream", operation);
                    var writer = sdk.streamWriter(WireHandle.fromJson(callback.path("stream")));
                    await(writer.chunk(obj().put("kind", "channel").set("value", spec.path("stream_items").get(0))));
                    await(writer.complete(obj().put("kind", "channel").set("value", spec.path("stream_items").get(1))));
                    return obj().put("outcome", "stream").set("stream", callback.path("stream"));
                } catch (Exception error) { throw new CompletionException(error); }
                finally { finalies.incrementAndGet(); }
            })));
            var descriptor = obj().put("kind", "channel_plugin").put("descriptor_version", 1).put("channel_id", channel).put("label", "native 中文🙂").put("handler_id", handlerId);
            descriptor.set("capabilities", tree(Map.of("chat_types", List.of("direct", "group"), "supports_media", true, "supports_threads", true)));
            var plugin = await(sdk.registerChannelPlugin("plugin-" + channel + "-" + manager.id(), descriptor, (callback, cancel) -> CompletableFuture.supplyAsync(() -> {
                String operation = callback.path("invocation").path("operation").textValue(); var fact = obj().put("operation", operation).put("channel", channel); fact.set("input", callback.path("invocation").path("input")); pluginCalls.add(fact);
                try {
                    if ("channel_start".equals(operation)) {
                        if (startProbe.get()) failure(sdk.call("channels.handler.handle", session.wire(), List.of(manager, channel, nativeValue(spec.path("inbound")))), "invalid_value");
                        if (startFail.compareAndSet(true, false)) return errorOutcome("fail-start-once");
                    }
                    return result(operation, null);
                } catch (Exception error) { throw new CompletionException(error); }
            })));
            assertTrue(await(sdk.call("channels.plugin.register", session.wire(), List.of(manager, plugin.wire(), handler.wire()))).path("registered").booleanValue());
            return new Owner(session, manager, channel, handler, plugin);
        }
        Owner primary() throws Exception { return register(sessions.get(0), WireHandle.fromJson(await(sdk.call("channels.manager.open", sessions.get(0).wire(), List.of())).path("resource"))); }
        JsonNode start(Owner owner) throws Exception { return await(call(owner, "channels.manager.start", owner.manager(), owner.handler().wire())); }
        CompletionStage<JsonNode> stop(Owner owner) { return call(owner, "channels.manager.stop_exact", owner.manager(), owner.channel()); }
        CompletionStage<JsonNode> closeManager(Owner owner) {
            String operation = "facade.resource.close"; var request = obj().put("operation", operation).put("signature_digest", sdk.catalog().resolve(operation).signature()); request.set("handle", owner.session().wire().toJson()); request.putArray("arguments").add(WireValues.value(owner.manager())); return sdk.request("_echo_agent/facade/invoke", request);
        }
        JsonNode park(String mode) { modes.add(mode); started.put(mode, new CompletableFuture<>()); cancellationObserved.put(mode, new CompletableFuture<>()); release.put(mode, new CompletableFuture<>()); var message = (ObjectNode) spec.path("inbound").deepCopy(); message.put("message_id", mode); return message; }
        void archive(List<JsonNode> rows) throws Exception { scenarios.addAll(rows); var facts = obj().put("finally_count", finalies.get()); facts.set("calls", tree(calls)); facts.set("plugin_calls", tree(pluginCalls)); facts.set("metadata", tree(metadata)); facts.set("rows", tree(rows)); write(work.resolve("observed.json"), facts); }
        @Override public void close() { release.values().forEach(value -> value.complete(null)); sdk.close(); }
    }

    @Test void singleStreamReplyTypedFidelityRejectsInvalidInputBeforeNativeCallback() throws Exception {
        var nativeClient = new Native("core"); var rows = new ArrayList<JsonNode>();
        try (nativeClient) {
            var owner = nativeClient.primary(); assertEquals(tree(Map.of("channels", "1", "failures", "0")), nativeClient.start(owner));
            var single = await(nativeClient.message(owner, "channels.handler.handle", spec.path("inbound")));
            WireHandle stream = WireHandle.fromJson(await(nativeClient.message(owner, "channels.handler.handle_stream", spec.path("inbound"))));
            var events = new ArrayList<JsonNode>(); for (int index = 0; index < 3; index++) events.add(await(nativeClient.call(owner, "channels.stream.next", stream)));
            assertEquals(List.of("item", "item", "complete"), events.stream().map(ChannelHandlerForwardHostTest::variant).toList());
            ObjectNode output = obj(); output.set("single", normalize(single)); var items = output.putArray("items"); var sequences = output.putArray("sequences");
            for (var event : events) { var fields = fields(event); assertEquals(stream.toJson(), fields.get("stream")); sequences.add(fields.get("sequence")); if (fields.containsKey("value")) items.add(normalize(fields.get("value"))); }
            output.put("terminal", "complete"); output.set("reply", await(nativeClient.message(owner, "channels.handler.reply", spec.path("outbound"))));
            var counts = output.putObject("callback_counts"); for (var call : nativeClient.calls) { String operation = call.path("operation").textValue(); counts.put(operation, counts.path(operation).intValue() + 1); }
            assertEquals(spec.path("normalized_core_expected"), output); rows.add(obj().put("scenario", "single-stream-reply").set("observed", output));
            int before = nativeClient.calls.size();
            for (var invalid : spec.path("invalid_inbound")) {
                var message = (ObjectNode) spec.path("inbound").deepCopy(); if (invalid.has("remove")) message.remove(invalid.path("remove").textValue()); else message.set(invalid.path("field").textValue(), invalid.path("value"));
                var error = failure(nativeClient.message(owner, "channels.handler.handle", message), "invalid_value");
                var row = obj().put("scenario", invalid.path("id").textValue()).put("callback_delta_expected", 0).put("callback_delta", nativeClient.calls.size() - before); row.set("error", error); rows.add(row);
            }
            assertEquals(spec.path("inbound"), nativeClient.calls.get(0).path("message"));
            for (String optional : List.of("missing", "null", "empty")) {
                var message = (ObjectNode) spec.path("inbound").deepCopy();
                for (var attachment : message.path("attachments")) { var value = (ObjectNode) attachment; if ("missing".equals(optional)) value.remove("filename"); else if ("null".equals(optional)) value.putNull("filename"); else value.put("filename", ""); }
                await(nativeClient.message(owner, "channels.handler.handle", message));
                for (var attachment : nativeClient.calls.get(nativeClient.calls.size() - 1).path("message").path("attachments")) if ("empty".equals(optional)) assertEquals("", attachment.path("filename").textValue()); else assertFalse(attachment.hasNonNull("filename"));
                var reply = (ObjectNode) spec.path("outbound").deepCopy(); if ("missing".equals(optional)) reply.remove("reply_to"); else if ("null".equals(optional)) reply.putNull("reply_to"); else reply.put("reply_to", "");
                await(nativeClient.message(owner, "channels.handler.reply", reply)); var received = nativeClient.calls.get(nativeClient.calls.size() - 1).path("message"); if ("empty".equals(optional)) assertEquals("", received.path("reply_to").textValue()); else assertFalse(received.hasNonNull("reply_to"));
            }
            var bare = (ObjectNode) spec.path("inbound").deepCopy(); bare.remove("attachments"); await(nativeClient.message(owner, "channels.handler.handle", bare)); assertEquals(0, nativeClient.calls.get(nativeClient.calls.size() - 1).path("message").path("attachments").size());
            await(nativeClient.stop(owner));
        } finally { nativeClient.archive(rows); }
    }

    @Test void completeOwnerBindingStartRollbackAndClosedFamilyDigestFence() throws Exception {
        var nativeClient = new Native("owner-binding"); var rows = new ArrayList<JsonNode>();
        try (nativeClient) {
            var owner = nativeClient.primary(); int before = nativeClient.calls.size();
            var inactive = failure(nativeClient.message(owner, "channels.handler.handle", spec.path("inbound")), "invalid_value");
            var row = obj().put("scenario", "unstarted").put("callback_delta_expected", 0).put("callback_delta", nativeClient.calls.size() - before); row.set("error", inactive); rows.add(row);
            var wrong = await(nativeClient.sdk.registerChannelMessageHandler("other-handler", obj().put("kind", "channel_message_handler").put("descriptor_version", 1).put("handler_id", "other-handler"), (call, cancellation) -> CompletableFuture.completedFuture(result("channel_reply", null))));
            int starts = nativeClient.pluginCalls.size(); failure(nativeClient.call(owner, "channels.manager.start", owner.manager(), wrong.wire()), "invalid_value"); assertEquals(starts, nativeClient.pluginCalls.size());
            var changed = new WireHandle(owner.handler().wire().id(), "18446744073709551615", "extension"); failure(nativeClient.call(owner, "channels.manager.start", owner.manager(), changed), "invalid_value"); assertEquals(starts, nativeClient.pluginCalls.size());
            var batch = WireHandle.fromJson(await(nativeClient.call(owner, "channels.manager.open")).path("resource")); var batchStarts = new AtomicInteger();
            for (String name : List.of("batch-a", "batch-b")) {
                String boundId = "batch-a".equals(name) ? spec.path("handler_id").textValue() : "other-handler"; var bound = "batch-a".equals(name) ? owner.handler() : wrong;
                var descriptor = obj().put("kind", "channel_plugin").put("descriptor_version", 1).put("channel_id", name).put("label", name).put("handler_id", boundId); descriptor.set("capabilities", tree(Map.of("chat_types", List.of("direct"), "supports_media", false, "supports_threads", false)));
                var plugin = await(nativeClient.sdk.registerChannelPlugin(name, descriptor, (callback, cancel) -> { if ("channel_start".equals(callback.path("invocation").path("operation").textValue())) batchStarts.incrementAndGet(); return CompletableFuture.completedFuture(result(callback.path("invocation").path("operation").textValue(), null)); }));
                await(nativeClient.call(owner, "channels.plugin.register", batch, plugin.wire(), bound.wire()));
            }
            failure(nativeClient.call(owner, "channels.manager.start", batch, owner.handler().wire()), "invalid_value"); assertEquals(0, batchStarts.get()); rows.add(obj().put("scenario", "batch-original-binding-before-all-effects").put("plugin_start_effects", 0));
            nativeClient.startProbe.set(true); nativeClient.startFail.set(true); assertEquals("1", nativeClient.start(owner).path("failures").textValue());
            failure(nativeClient.message(owner, "channels.handler.handle", spec.path("inbound")), "invalid_value"); assertEquals("0", nativeClient.start(owner).path("failures").textValue()); nativeClient.startProbe.set(false);
            failure(nativeClient.sdk.call("channels.handler.handle", nativeClient.sessions.get(1).wire(), List.of(owner.manager(), owner.channel(), nativeValue(spec.path("inbound")))), "invalid_value");
            var managerB = WireHandle.fromJson(await(nativeClient.sdk.call("channels.manager.open", nativeClient.sessions.get(1).wire(), List.of())).path("resource"));
            var second = new Owner(nativeClient.sessions.get(1), managerB, owner.channel(), owner.handler(), owner.plugin());
            assertTrue(await(nativeClient.sdk.call("channels.plugin.register", second.session().wire(), List.of(managerB, owner.plugin().wire(), owner.handler().wire()))).path("registered").booleanValue());
            assertEquals("0", nativeClient.start(second).path("failures").textValue()); var secondMessage = (ObjectNode) spec.path("inbound").deepCopy(); secondMessage.put("message_id", "second-owner-message"); assertEquals(spec.path("outbound"), normalize(await(nativeClient.message(second, "channels.handler.handle", secondMessage)))); assertEquals("second-owner-message", nativeClient.calls.get(nativeClient.calls.size() - 1).path("message").path("message_id").textValue());
            var stale = new WireHandle(owner.manager().id(), "0", owner.manager().kind());
            var staleError = failure(nativeClient.call(owner, "channels.handler.handle", stale, owner.channel(), nativeValue(spec.path("inbound"))), "stale_handle");
            WireHandle stream = WireHandle.fromJson(await(nativeClient.message(owner, "channels.handler.handle_stream", spec.path("inbound"))));
            failure(nativeClient.sdk.call("channels.stream.next", nativeClient.sessions.get(1).wire(), List.of(stream)), "invalid_value"); failure(nativeClient.call(owner, "channels.stream.next", new WireHandle(stream.id(), "0", "stream")), "stale_handle"); failure(nativeClient.call(owner, "workflow.stream.next", stream), "invalid_value"); await(nativeClient.call(owner, "channels.stream.close", stream));
            var request = obj().put("operation", "channels.handler.handle").put("signature_digest", "sha256:" + "0".repeat(64)); request.set("handle", owner.session().wire().toJson()); var arguments = request.putArray("arguments"); arguments.add(WireValues.value(owner.manager())); arguments.add(WireValues.value(owner.channel())); arguments.add(WireValues.value(nativeValue(spec.path("inbound"))));
            var digestError = failure(nativeClient.sdk.request("_echo_agent/channels/op", request), "invalid_value");
            var fact = obj().put("scenario", "owner-generation-binding-start-digest").put("original_entry_survived", true); fact.set("stale", staleError); fact.set("digest", digestError); rows.add(fact);
            assertEquals(spec.path("outbound"), normalize(await(nativeClient.message(owner, "channels.handler.handle", spec.path("inbound")))));
            await(nativeClient.stop(owner)); assertEquals(spec.path("outbound"), normalize(await(nativeClient.message(second, "channels.handler.handle", spec.path("inbound"))))); nativeClient.start(owner); assertEquals(spec.path("outbound"), normalize(await(nativeClient.message(owner, "channels.handler.handle", spec.path("inbound")))));
            var failing = (ObjectNode) spec.path("inbound").deepCopy(); failing.put("message_id", "handle-error"); var handleError = failure(nativeClient.message(owner, "channels.handler.handle", failing), "framework_error"); assertTrue(handleError.path("message").textValue().contains(spec.path("callback_behaviors").path("fail-handle").path("error").path("message").textValue())); var failureRow = obj().put("scenario", "fail-handle").put("error_code", "framework_error").put("callback_delta", 1); failureRow.set("error", handleError); rows.add(failureRow);
            await(nativeClient.stop(second)); wrong.close();
        } finally { nativeClient.archive(rows); }
    }

    @Test void nativeCancelKeepsPeerAndCloseWaitsFullBufferCallbackFinally() throws Exception {
        var nativeClient = new Native("lifecycle"); var rows = new ArrayList<JsonNode>();
        try (nativeClient) {
            var owner = nativeClient.primary(); nativeClient.start(owner);
            WireHandle stream = WireHandle.fromJson(await(nativeClient.message(owner, "channels.handler.handle_stream", nativeClient.park("park")))); nativeClient.started.get("park").get(5, TimeUnit.SECONDS);
            assertTrue(await(nativeClient.call(owner, "channels.stream.cancel", stream)).booleanValue()); var terminal = nativeClient.call(owner, "channels.stream.next", stream).toCompletableFuture();
            nativeClient.cancellationObserved.get("park").get(5, TimeUnit.SECONDS); assertFalse(terminal.isDone()); assertEquals(spec.path("outbound"), normalize(await(nativeClient.message(owner, "channels.handler.handle", spec.path("inbound")))));
            nativeClient.release.get("park").complete(null); assertEquals("cancelled", variant(await(terminal)));
            stream = WireHandle.fromJson(await(nativeClient.message(owner, "channels.handler.handle_stream", nativeClient.park("full")))); nativeClient.started.get("full").get(5, TimeUnit.SECONDS);
            var closing = nativeClient.call(owner, "channels.stream.close", stream).toCompletableFuture(); nativeClient.cancellationObserved.get("full").get(5, TimeUnit.SECONDS); assertFalse(closing.isDone()); nativeClient.release.get("full").complete(null); await(closing); failure(nativeClient.call(owner, "channels.stream.next", stream), "closed_handle");
            var pending = nativeClient.message(owner, "channels.handler.handle", nativeClient.park("caller")).toCompletableFuture(); nativeClient.started.get("caller").get(5, TimeUnit.SECONDS); pending.cancel(true);
            var stop = nativeClient.stop(owner).toCompletableFuture(); nativeClient.cancellationObserved.get("caller").get(5, TimeUnit.SECONDS); assertFalse(stop.isDone()); nativeClient.release.get("caller").complete(null); await(stop); nativeClient.start(owner);
            var outbound = (ObjectNode) spec.path("outbound").deepCopy(); outbound.put("text", "error"); var replyError = failure(nativeClient.message(owner, "channels.handler.reply", outbound), "framework_error"); assertTrue(replyError.path("message").textValue().contains(spec.path("callback_behaviors").path("fail-reply").path("error").path("message").textValue()));
            stream = WireHandle.fromJson(await(nativeClient.message(owner, "channels.handler.handle_stream", nativeClient.park("resource")))); nativeClient.started.get("resource").get(5, TimeUnit.SECONDS);
            closing = nativeClient.closeManager(owner).toCompletableFuture(); nativeClient.cancellationObserved.get("resource").get(5, TimeUnit.SECONDS); assertFalse(closing.isDone()); nativeClient.release.get("resource").complete(null); await(closing); failure(nativeClient.message(owner, "channels.handler.handle", spec.path("inbound")), "closed_handle");
            assertEquals(nativeClient.calls.size(), nativeClient.finalies.get()); var row = obj().put("scenario", "cancel-peer-full-close-caller-drop-resource-close").put("cancel_peer_survived", true).put("cleanup_waited", true).put("callback_finally", nativeClient.finalies.get()).put("callbacks", nativeClient.calls.size()); row.set("reply_error", replyError); rows.add(row);
        } finally { nativeClient.archive(rows); }
    }
}
