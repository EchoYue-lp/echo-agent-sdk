package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;

import java.util.concurrent.CompletionStage;

/** Connection-owned registration for a host-language implementation. */
public final class ExtensionRegistration implements AutoCloseable {
    private final EchoAgentClient client;
    private final WireHandle wire;
    private boolean closed;
    private CompletionStage<JsonNode> unregisterStage;

    ExtensionRegistration(EchoAgentClient client, WireHandle wire) {
        this.client = client;
        this.wire = wire;
    }

    public WireHandle wire() { return wire; }

    public synchronized CompletionStage<JsonNode> unregister() {
        if (closed) return java.util.concurrent.CompletableFuture.completedFuture(JsonSupport.MAPPER.createObjectNode());
        if (unregisterStage != null) return unregisterStage;
        var result = new java.util.concurrent.CompletableFuture<JsonNode>();
        unregisterStage = result;
        client.unregisterExtension(wire).whenComplete((value, error) -> {
            synchronized (this) {
                if (error == null) closed = true;
                unregisterStage = null;
            }
            if (error == null) result.complete(value);
            else result.completeExceptionally(error);
        });
        return result;
    }

    @Override public void close() {
        unregister().toCompletableFuture().join();
    }
}
