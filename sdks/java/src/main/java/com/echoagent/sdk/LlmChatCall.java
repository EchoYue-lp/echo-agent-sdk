package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;

import java.util.Set;

/** Typed reverse invocation received by an LLM implementation. */
public final class LlmChatCall implements ExtensionCall {
    private static final Set<String> OPERATIONS = Set.of("llm_chat", "llm_chat_stream");
    private final String operation;
    private final JsonNode input;
    private final JsonNode context;
    private final JsonNode raw;

    private LlmChatCall(String operation, JsonNode input, JsonNode context, JsonNode raw) {
        this.operation = operation;
        this.input = input;
        this.context = context;
        this.raw = raw;
    }

    public static LlmChatCall from(JsonNode call) {
        var invocation = ToolCall.invocation(call);
        var operation = ToolCall.operation(invocation, OPERATIONS, "LlmClient");
        var input = ToolCall.requiredObject(invocation, "input", "LlmClient invocation input");
        if (!input.path("messages").isArray()) {
            throw new IllegalArgumentException("LlmClient invocation input must contain messages");
        }
        validateToolChoice(input.get("tool_choice"));
        return new LlmChatCall(operation, input, ToolCall.nullable(call, "context"), call.deepCopy());
    }

    @Override public String operation() { return operation; }
    @Override public JsonNode input() { return input.deepCopy(); }
    public JsonNode messages() { return input.path("messages").deepCopy(); }
    public JsonNode options() { return input.deepCopy(); }
    public JsonNode toolChoice() { return input.path("tool_choice").deepCopy(); }
    @Override public JsonNode context() { return context == null ? null : context.deepCopy(); }
    @Override public JsonNode raw() { return raw.deepCopy(); }

    private static void validateToolChoice(JsonNode value) {
        if (value == null || value.isNull()) return;
        if (value.isTextual() && Set.of("auto", "none", "required").contains(value.textValue())) return;
        if (value.isObject() && value.size() == 2 && "function".equals(value.path("type").textValue())) {
            var function = value.path("function");
            var name = function.path("name");
            // Unicode White_Space与Core trim一致；保留原名，不用Java较窄的isBlank规范化。
            if (function.isObject() && function.size() == 1 && name.isTextual()
                    && !name.textValue().matches("(?U)\\s*")) return;
        }
        throw new IllegalArgumentException("LlmClient tool_choice must be a canonical mode or named function");
    }
}
