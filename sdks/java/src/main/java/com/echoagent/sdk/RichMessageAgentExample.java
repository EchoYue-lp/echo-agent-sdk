package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.util.function.BiFunction;

/** 示例：业务实现接收原完整 Message，包括 Parts、ResourceLink meta 与工具身份。 */
public final class RichMessageAgentExample {
    private RichMessageAgentExample() {}

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
