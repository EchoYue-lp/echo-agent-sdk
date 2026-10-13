package com.echoagent.sdk;

import com.fasterxml.jackson.annotation.*;
import com.fasterxml.jackson.databind.JsonNode;
import java.util.List;

/** Complete typed LLM message view used by AgentCallback.onThinkStart. */
public record CallbackMessage(String role, JsonNode content,
        @JsonInclude(JsonInclude.Include.NON_NULL) List<ToolCallValue> toolCalls,
        @JsonInclude(JsonInclude.Include.NON_NULL) String name,
        @JsonInclude(JsonInclude.Include.NON_NULL) String toolCallId,
        @JsonInclude(JsonInclude.Include.NON_NULL) String reasoningContent,
        @JsonInclude(JsonInclude.Include.NON_NULL) List<ReasoningBlock> reasoningBlocks) implements ManagedPersistence.Value {
    public CallbackMessage {
        require(role, "role");
        if (content == null || !content.isObject() || !content.path("kind").isTextual()) throw new IllegalArgumentException("content must be WireValue");
        content = JsonSupport.validateWire(content);
        if (toolCalls != null) toolCalls = List.copyOf(toolCalls);
        if (reasoningBlocks != null) reasoningBlocks = List.copyOf(reasoningBlocks);
    }
    @Override public JsonNode content() { return content.deepCopy(); }
    public record ToolCallValue(String id, String callType, String functionName, String arguments) {
        public ToolCallValue { require(id, "id"); require(callType, "call_type"); require(functionName, "function_name"); require(arguments, "arguments"); }
    }
    @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "kind")
    @JsonSubTypes({@JsonSubTypes.Type(value = ReasoningBlock.Signed.class, name = "signed"),
            @JsonSubTypes.Type(value = ReasoningBlock.Redacted.class, name = "redacted"),
            @JsonSubTypes.Type(value = ReasoningBlock.Opaque.class, name = "opaque")})
    public sealed interface ReasoningBlock {
        record Signed(String thinking, String signature) implements ReasoningBlock {
            public Signed { require(thinking, "thinking"); require(signature, "signature"); }
        }
        record Redacted(String data) implements ReasoningBlock {
            public Redacted { require(data, "data"); }
        }
        record Opaque(String provider, String id, String data, List<String> summary) implements ReasoningBlock {
            public Opaque { require(provider, "provider"); require(id, "id"); require(data, "data"); summary = List.copyOf(summary); }
        }
    }
    private static void require(String value, String name) { if (value == null) throw new IllegalArgumentException(name + " must not be null"); }
}
