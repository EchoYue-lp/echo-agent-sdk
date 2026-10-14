package com.echoagent.sdk;

import com.fasterxml.jackson.annotation.*;
import com.fasterxml.jackson.databind.JsonNode;
import java.math.BigInteger;
import java.util.List;
import java.util.Set;

/** Closed AgentCallback inputs, preserving tool call identity independently of transport identity. */
public final class CallbackCall implements ExtensionCall {
    private final String operation;
    private final Request request;
    private final JsonNode raw;
    private CallbackCall(String operation, Request request, JsonNode raw) {
        this.operation = operation; this.request = request; this.raw = raw.deepCopy();
    }
    public sealed interface Request extends ManagedPersistence.Value permits ThinkStart, ThinkEnd, ToolStart, ToolEnd, ToolError, ToolInterrupted, FinalAnswer, Iteration {}
    public record ThinkStart(String agent, List<CallbackMessage> messages) implements Request {
        public ThinkStart { require(agent, "agent"); messages = List.copyOf(messages); }
    }
    public record ThinkEnd(String agent, List<Step> steps, BigInteger promptTokens, BigInteger completionTokens) implements Request {
        public ThinkEnd {
            require(agent, "agent"); steps = List.copyOf(steps); WireValues.u64(promptTokens); WireValues.u64(completionTokens);
        }
    }
    @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "kind")
    @JsonSubTypes({@JsonSubTypes.Type(value = Step.Thought.class, name = "thought"), @JsonSubTypes.Type(value = Step.Call.class, name = "call")})
    public sealed interface Step extends ManagedPersistence.Value {
        record Thought(String text) implements Step { public Thought { require(text, "text"); } }
        record Call(String toolCallId, String functionName, JsonNode arguments) implements Step {
            public Call { require(toolCallId, "tool_call_id"); require(functionName, "function_name"); arguments = wire(arguments); }
            @Override public JsonNode arguments() { return arguments.deepCopy(); }
        }
    }
    public record ToolStart(String agent, @JsonInclude(JsonInclude.Include.NON_NULL) String callId, String tool, JsonNode args) implements Request {
        public ToolStart { require(agent, "agent"); require(tool, "tool"); args = wire(args); }
        @Override public JsonNode args() { return args.deepCopy(); }
    }
    public record ToolEnd(String agent, @JsonInclude(JsonInclude.Include.NON_NULL) String callId, String tool, String result) implements Request {
        public ToolEnd { require(agent, "agent"); require(tool, "tool"); require(result, "result"); }
    }
    public record ToolError(String agent, @JsonInclude(JsonInclude.Include.NON_NULL) String callId, String tool, String error) implements Request {
        public ToolError { require(agent, "agent"); require(tool, "tool"); require(error, "error"); }
    }
    public record ToolInterrupted(String agent, String callId, String tool, JsonNode input, String error) implements Request {
        public ToolInterrupted { require(agent, "agent"); require(callId, "call_id"); require(tool, "tool"); input = wire(input); require(error, "error"); }
        @Override public JsonNode input() { return input.deepCopy(); }
    }
    public record FinalAnswer(String agent, String answer) implements Request {
        public FinalAnswer { require(agent, "agent"); require(answer, "answer"); }
    }
    public record Iteration(String agent, BigInteger iteration) implements Request {
        public Iteration { require(agent, "agent"); WireValues.u64(iteration); }
    }
    public static CallbackCall from(JsonNode call) {
        JsonNode invocation = ToolCall.invocation(call);
        ManagedPersistence.fields(invocation, "operation", "input");
        String operation = ToolCall.operation(invocation, Set.of("callback_on_think_start", "callback_on_think_end",
                "callback_on_tool_start", "callback_on_tool_end", "callback_on_tool_error", "callback_on_tool_interrupted",
                "callback_on_final_answer", "callback_on_iteration"), "AgentCallback");
        JsonNode input = ToolCall.requiredObject(invocation, "input", "callback input");
        Request request = switch (operation) {
            case "callback_on_think_start" -> ManagedPersistence.decode(input, ThinkStart.class);
            case "callback_on_think_end" -> ManagedPersistence.decode(input, ThinkEnd.class);
            case "callback_on_tool_start" -> ManagedPersistence.decode(input, ToolStart.class);
            case "callback_on_tool_end" -> ManagedPersistence.decode(input, ToolEnd.class);
            case "callback_on_tool_error" -> ManagedPersistence.decode(input, ToolError.class);
            case "callback_on_tool_interrupted" -> ManagedPersistence.decode(input, ToolInterrupted.class);
            case "callback_on_final_answer" -> ManagedPersistence.decode(input, FinalAnswer.class);
            case "callback_on_iteration" -> ManagedPersistence.decode(input, Iteration.class);
            default -> throw new IllegalArgumentException("unknown callback operation");
        };
        return new CallbackCall(operation, request, call);
    }
    public Request request() { return request; }
    public String callId() {
        if (request instanceof ToolStart value) return value.callId();
        if (request instanceof ToolEnd value) return value.callId();
        if (request instanceof ToolError value) return value.callId();
        if (request instanceof ToolInterrupted value) return value.callId();
        return null;
    }
    /** Exact unit result for the received callback operation. */
    public ExtensionOutcome completed() { return JsonExtensionOutcome.result(operation, com.fasterxml.jackson.databind.node.NullNode.instance); }
    @Override public String operation() { return operation; }
    @Override public JsonNode input() { return request.toJson(); }
    @Override public JsonNode context() { return ToolCall.nullable(raw, "context"); }
    @Override public JsonNode raw() { return raw.deepCopy(); }
    private static void require(String value, String name) { if (value == null) throw new IllegalArgumentException(name + " must not be null"); }
    private static JsonNode wire(JsonNode value) {
        if (value == null || !value.isObject() || !value.path("kind").isTextual()) throw new IllegalArgumentException("expected WireValue");
        return JsonSupport.validateWire(value);
    }
}
