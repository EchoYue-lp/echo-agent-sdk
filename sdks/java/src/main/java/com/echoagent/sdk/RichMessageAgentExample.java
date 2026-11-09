package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.util.function.BiFunction;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

/** 示例：业务实现接收原完整 Message，包括 Parts、ResourceLink meta 与工具身份。 */
public final class RichMessageAgentExample {
    private RichMessageAgentExample() {}

    /** 只转运原 producer 的 wire 事件；迭代真实 EOF 后才提交原终态。 */
    public static CompletionStage<Void> forwardNativeOutput(ExtensionStreamWriter writer, Iterable<JsonNode> source) {
        var terminal = new java.util.concurrent.atomic.AtomicReference<JsonNode>();
        // 复用已有 Reactor：零预取按原 send 完成推进迭代；源异常也等待当前 send，
        // 不新建线程/executor，不丢弃正在传输的原 future。
        return Flux.defer(() -> {
            var iterator = source.iterator();
            // fromIterable 会提前读取下一项；generate 只在原 send 完成后的 demand 读取。
            return Flux.<JsonNode>generate(sink -> {
                if (iterator.hasNext()) sink.next(iterator.next());
                else sink.complete();
            });
        }).concatMapDelayError(event -> {
            if (terminal.get() != null) return Mono.<Void>error(new IllegalArgumentException("native producer emitted data after its runtime terminal"));
            var payload = JsonSupport.MAPPER.createObjectNode().put("kind", "agent");
            payload.set("value", event.deepCopy());
            boolean runtimeTerminal = "execution_status".equals(event.path("event").asText())
                    && java.util.Set.of("settled", "failed", "cancelled").contains(event.path("data").path("status").asText());
            if (runtimeTerminal) { terminal.set(payload); return Mono.<Void>empty(); }
            return Mono.fromCompletionStage(() -> writer.chunk(payload));
        }, false, 0).then(Mono.defer(() -> {
            JsonNode original = terminal.get();
            if (original == null) return Mono.<Void>error(new IllegalArgumentException("native producer EOF without its runtime terminal"));
            // deepCopy 保留原序号字符串、签名/密文；不经过默认 BigInteger 数字编码。
            return Mono.fromCompletionStage(() -> writer.complete(original));
        })).toFuture();
    }

    public static CompletionStage<ExtensionRegistration> register(
            EchoAgentClient sdk,
            BiFunction<JsonNode, ExtensionCancellation, CompletionStage<String>> consume) {
        var descriptor = JsonSupport.MAPPER.createObjectNode().put("kind", "custom_agent")
                .put("descriptor_version", 1).put("name", "message-example")
                .put("model_name", "example").put("system_prompt", "保留完整输入");
        descriptor.putArray("tool_names");
        return sdk.registerCustomAgent("message-example", descriptor, (call, cancel) -> {
            var invocation = call.required("invocation");
            String operation = invocation.required("operation").asText();
            if ("agent_close".equals(operation))
                return CompletableFuture.completedFuture(result(operation, JsonSupport.MAPPER.nullNode()));
            String field = switch (operation) {
                case "agent_execute", "agent_execute_stream" -> "task";
                case "agent_chat", "agent_chat_stream" -> "message";
                default -> throw new IllegalArgumentException("unexpected message Agent operation");
            };
            // consume接收整个operation/input，包含完整Message与可选history。
            invocation.required("input").required(field);
            return consume.apply(invocation, cancel).thenCompose(answer -> {
                if (!operation.endsWith("_stream"))
                    return CompletableFuture.completedFuture(result(operation, JsonSupport.MAPPER.getNodeFactory().textNode(answer)));
                var stream = WireHandle.fromJson(call.required("stream"));
                var data = JsonSupport.MAPPER.createObjectNode().put("text", answer);
                var terminal = JsonSupport.MAPPER.createObjectNode().put("event", "final_answer");
                terminal.set("data", data);
                var payload = JsonSupport.MAPPER.createObjectNode().put("kind", "agent");
                payload.set("value", terminal);
                return sdk.streamWriter(stream).complete(payload).thenApply(ignored -> ExtensionOutcome.stream(stream).toJson());
            });
        });
    }

    private static JsonNode result(String operation, JsonNode value) {
        var result = JsonSupport.MAPPER.createObjectNode().put("operation", operation);
        result.set("value", value);
        var outcome = JsonSupport.MAPPER.createObjectNode().put("outcome", "result");
        outcome.set("result", result);
        return outcome;
    }
}
