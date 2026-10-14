package com.echoagent.sdk;
import com.fasterxml.jackson.databind.node.ObjectNode;
/** Versioned descriptor for the framework AgentCallback boundary. */
public record CallbackDescriptor() implements ExtensionDescriptor {
    @Override public String kind() { return "agent_callback"; }
    @Override public ObjectNode toJson() { return TypedExtensionSupport.baseDescriptor(kind()); }
}
