package com.echoagent.sdk;

import com.fasterxml.jackson.annotation.*;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import java.io.IOException;
import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;
import java.util.Set;

/** 原LlmMessageWire的完整回调投影；消息内的执行观察不授予本回调运行时结算权。 */
public record CallbackMessage(String role, JsonNode content,
        @JsonInclude(JsonInclude.Include.NON_NULL) List<ToolCallValue> toolCalls,
        @JsonInclude(JsonInclude.Include.NON_NULL) String name,
        @JsonInclude(JsonInclude.Include.NON_NULL) String toolCallId,
        @JsonInclude(JsonInclude.Include.NON_NULL) String reasoningContent,
        @JsonInclude(JsonInclude.Include.NON_NULL) List<ReasoningBlock> reasoningBlocks,
        @JsonInclude(JsonInclude.Include.NON_NULL) List<AssistantBlock> assistantBlocks,
        @JsonInclude(JsonInclude.Include.NON_NULL) ExecutionStatus executionStatus,
        @JsonInclude(JsonInclude.Include.NON_NULL) List<AssistantBlock> approvedAssistantBlocks) implements ManagedPersistence.Value {
    public CallbackMessage {
        require(role, "role");
        if (content == null || !content.isObject() || !content.path("kind").isTextual()) throw new IllegalArgumentException("content must be WireValue");
        content = JsonSupport.validateWire(content);
        if (toolCalls != null) toolCalls = List.copyOf(toolCalls);
        if (reasoningBlocks != null) reasoningBlocks = List.copyOf(reasoningBlocks);
        if (assistantBlocks != null) assistantBlocks = List.copyOf(assistantBlocks);
        // null与空列表分别是“未提供批准投影”和“明确禁止原文回退”，不能相互归一。
        if (approvedAssistantBlocks != null) approvedAssistantBlocks = List.copyOf(approvedAssistantBlocks);
    }
    @Override public JsonNode content() { return content.deepCopy(); }
    public record ToolCallValue(String id, String callType, String functionName, String arguments,
            @JsonInclude(JsonInclude.Include.NON_NULL) ProviderOutputSource providerSource) {
        public ToolCallValue { require(id, "id"); require(callType, "call_type"); require(functionName, "function_name"); require(arguments, "arguments"); }
    }
    /** 已有ProviderOutputSourceWire的值载体；原始phase与u32位置不由SDK猜测或重写。 */
    public record ProviderOutputSource(String protocol, String requestId, String itemId, Long outputIndex,
            @JsonInclude(JsonInclude.Include.NON_NULL) Long contentIndex,
            @JsonInclude(JsonInclude.Include.NON_NULL) String contentKind,
            @JsonInclude(JsonInclude.Include.NON_NULL) String phase) {
        public ProviderOutputSource {
            require(protocol, "protocol"); require(requestId, "request_id"); require(itemId, "item_id");
            if (Arrays.stream(LlmApiProtocol.values()).noneMatch(value -> value.asStr().equals(protocol)))
                throw new IllegalArgumentException("unknown provider protocol");
            u32(outputIndex, "output_index");
            if (contentIndex != null) u32(contentIndex, "content_index");
        }
    }
    public record AssistantBlock(
            @JsonInclude(JsonInclude.Include.NON_NULL) ProviderOutputSource providerSource,
            String blockId, String requestId, BigInteger sequence,
            @JsonDeserialize(using = AssistantPhaseDecoder.class) String phase,
            String delta, Boolean complete,
            @JsonInclude(JsonInclude.Include.NON_NULL) String provider,
            @JsonInclude(JsonInclude.Include.NON_NULL) ReasoningBlock reasoning) {
        public AssistantBlock {
            identity(blockId, "block_id"); identity(requestId, "request_id"); WireValues.u64(sequence);
            // Protocol允许缺phase，语义保持Unknown；provider原phase仍独立存于providerSource。
            if (phase == null) phase = "unknown";
            if (!Set.of("reasoning", "commentary", "partial_answer", "final_answer", "unknown").contains(phase))
                throw new IllegalArgumentException("unknown assistant phase");
            require(delta, "delta");
            if (complete == null) throw new IllegalArgumentException("complete must not be null");
        }
    }
    /** 原wire缺phase时为Unknown，显式null仍非法；局部区分，不放宽其它字段的codec。 */
    public static final class AssistantPhaseDecoder extends StdDeserializer<String> {
        public AssistantPhaseDecoder() { super(String.class); }
        @Override public String getAbsentValue(DeserializationContext context) { return "unknown"; }
        @Override public String getNullValue(DeserializationContext context) {
            throw new IllegalArgumentException("assistant phase must not be null");
        }
        @Override public String deserialize(JsonParser parser, DeserializationContext context) throws IOException {
            if (!parser.hasToken(JsonToken.VALUE_STRING)) throw new IllegalArgumentException("assistant phase must be text");
            return parser.getText();
        }
    }
    public record ExecutionStatus(String executionId,
            @JsonInclude(JsonInclude.Include.NON_NULL) String requestId,
            BigInteger sequence, String phase, String status,
            @JsonInclude(JsonInclude.Include.NON_NULL) String reason) {
        public ExecutionStatus {
            identity(executionId, "execution_id");
            if (requestId != null) identity(requestId, "request_id");
            WireValues.u64(sequence); require(phase, "phase"); require(status, "status");
            if (!Set.of("Thinking", "Acting", "Terminal").contains(phase)
                    || !Set.of("started", "waiting", "tool_pending", "candidate", "settled", "cancelled", "failed").contains(status))
                throw new IllegalArgumentException("unknown execution status");
        }
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
    private static void identity(String value, String name) {
        require(value, name);
        if (value.isBlank()) throw new IllegalArgumentException(name + " must not be blank");
    }
    private static void u32(Long value, String name) {
        if (value == null || value < 0 || value > 4294967295L)
            throw new IllegalArgumentException(name + " must be a u32");
    }
}
