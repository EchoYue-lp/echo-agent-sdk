package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;

/** Stable SDK extension failure; callers must not parse ACP human messages for state. */
public final class EchoAgentException extends RuntimeException {
    private final String code;
    private final String retryable;
    private final String operation;
    private final JsonNode details;
    private final WireHandle handle;

    public EchoAgentException(String code, String message, String retryable, String operation, JsonNode details) {
        this(code, message, retryable, operation, details, null);
    }

    public EchoAgentException(String code, String message, String retryable, String operation, JsonNode details, WireHandle handle) {
        super(message);
        this.code = code;
        this.retryable = retryable;
        this.operation = operation;
        this.details = details;
        this.handle = handle;
    }

    public String code() { return code; }
    public String retryable() { return retryable; }
    public String operation() { return operation; }
    public JsonNode details() { return details; }
    public WireHandle handle() { return handle; }

    static EchoAgentException from(JsonNode data, String operation) {
        var object = data != null && data.isObject() ? data : null;
        return new EchoAgentException(
                text(object, "code", "transport_error"),
                text(object, "message", "ACP request failed"),
                text(object, "retryable", "never"),
                object != null && object.has("operation") && object.get("operation").isTextual()
                        ? object.get("operation").textValue() : operation,
                object == null ? null : object.get("details"),
                object != null && object.hasNonNull("handle") ? WireHandle.fromJson(object.get("handle")) : null);
    }

    static EchoAgentException from(Throwable error, String operation) {
        var cause = error;
        while ((cause instanceof java.util.concurrent.CompletionException || cause instanceof java.util.concurrent.ExecutionException) && cause.getCause() != null) cause = cause.getCause();
        if (cause instanceof EchoAgentException typed) return typed;
        Object data = cause instanceof com.agentclientprotocol.sdk.spec.AcpClientSession.AcpError acp ? acp.getData()
                : cause instanceof com.agentclientprotocol.sdk.error.AcpProtocolException acp ? acp.getData() : null;
        if (data != null) return from(JsonSupport.MAPPER.valueToTree(data), operation);
        return new EchoAgentException("transport_error", cause.getMessage() == null ? "ACP request failed" : cause.getMessage(), "never", operation, null);
    }

    private static String text(JsonNode node, String field, String fallback) {
        return node != null && node.has(field) && node.get(field).isTextual()
                ? node.get(field).textValue() : fallback;
    }
}
