package com.echoagent.sdk;

import com.fasterxml.jackson.databind.node.ObjectNode;

import java.math.BigInteger;
import java.util.Set;

/** Typed descriptor for a Host-consumed Agent infrastructure component. */
public final class AgentComponentDescriptor implements ExtensionDescriptor {
    private static final Set<String> COMPONENTS = Set.of(
            "conversation_store", "run_store", "runtime_state_store",
            "audit_logger", "context_projector", "memory_trigger_sink", "guard",
            "search_provider", "workflow_checkpoint_store", "revisioned_task_store",
            "sandbox_executor", "mcp_transport", "embedder", "memory_promoter", "workflow",
            "intent_classifier", "skill_load_policy");

    private final String component;
    private final String name;
    private final String isolationLevel;
    private final boolean supportsStreaming;
    private final boolean supportsNotifications;
    private final boolean executionCasV1;
    private final boolean protectedClaimV1;
    private final BigInteger claimHeartbeatIntervalMs;
    private final ManagedPersistence.ConversationProjectionCapability conversationProjection;
    private final ManagedPersistence.RuntimeStateCapability runtimeState;
    private final ManagedPersistence.PersistenceCallCapability persistenceCall;

    private AgentComponentDescriptor(
            String component, String name, String isolationLevel, boolean supportsStreaming,
            boolean supportsNotifications, BigInteger claimHeartbeatIntervalMs) {
        this(component, name, isolationLevel, supportsStreaming, supportsNotifications, claimHeartbeatIntervalMs,
                ManagedPersistence.ConversationProjectionCapability.unsupported,
                ManagedPersistence.RuntimeStateCapability.unsupported,
                ManagedPersistence.PersistenceCallCapability.unsupported);
    }

    private AgentComponentDescriptor(String component, String name, String isolationLevel,
            boolean supportsStreaming, boolean supportsNotifications, BigInteger claimHeartbeatIntervalMs,
            ManagedPersistence.ConversationProjectionCapability conversationProjection,
            ManagedPersistence.RuntimeStateCapability runtimeState,
            ManagedPersistence.PersistenceCallCapability persistenceCall) {
        this(component, name, isolationLevel, supportsStreaming, supportsNotifications, claimHeartbeatIntervalMs,
                conversationProjection, runtimeState, persistenceCall, false);
    }

    private AgentComponentDescriptor(String component, String name, String isolationLevel,
            boolean supportsStreaming, boolean supportsNotifications, BigInteger claimHeartbeatIntervalMs,
            ManagedPersistence.ConversationProjectionCapability conversationProjection,
            ManagedPersistence.RuntimeStateCapability runtimeState,
            ManagedPersistence.PersistenceCallCapability persistenceCall, boolean executionCasV1) {
        this(component, name, isolationLevel, supportsStreaming, supportsNotifications, claimHeartbeatIntervalMs,
                conversationProjection, runtimeState, persistenceCall, executionCasV1, false);
    }

    private AgentComponentDescriptor(String component, String name, String isolationLevel,
            boolean supportsStreaming, boolean supportsNotifications, BigInteger claimHeartbeatIntervalMs,
            ManagedPersistence.ConversationProjectionCapability conversationProjection,
            ManagedPersistence.RuntimeStateCapability runtimeState,
            ManagedPersistence.PersistenceCallCapability persistenceCall, boolean executionCasV1,
            boolean protectedClaimV1) {
        if (!COMPONENTS.contains(component)) {
            throw new IllegalArgumentException("unknown Agent component kind");
        }
        this.component = component;
        this.name = TypedExtensionSupport.requiredText(name, "name", 256);
        this.isolationLevel = isolationLevel;
        this.supportsStreaming = supportsStreaming;
        this.supportsNotifications = supportsNotifications;
        if (executionCasV1 && !"revisioned_task_store".equals(component)) {
            throw new IllegalArgumentException("execution CAS capability requires a revisioned task store");
        }
        this.executionCasV1 = executionCasV1;
        if (protectedClaimV1 && !"workflow_checkpoint_store".equals(component)) {
            throw new IllegalArgumentException("protected claim capability requires a workflow checkpoint store");
        }
        this.protectedClaimV1 = protectedClaimV1;
        this.claimHeartbeatIntervalMs = claimHeartbeatIntervalMs;
        if (conversationProjection == null || runtimeState == null || persistenceCall == null) {
            throw new IllegalArgumentException("managed capabilities must not be null");
        }
        if (conversationProjection != ManagedPersistence.ConversationProjectionCapability.unsupported
                && !"conversation_store".equals(component)) throw new IllegalArgumentException("conversation projection requires conversation_store");
        if (runtimeState != ManagedPersistence.RuntimeStateCapability.unsupported
                && !"runtime_state_store".equals(component)) throw new IllegalArgumentException("runtime state requires runtime_state_store");
        if (persistenceCall != ManagedPersistence.PersistenceCallCapability.unsupported
                && !Set.of("conversation_store", "runtime_state_store").contains(component)) {
            throw new IllegalArgumentException("persistence deadline requires a managed store");
        }
        this.conversationProjection = conversationProjection;
        this.runtimeState = runtimeState;
        this.persistenceCall = persistenceCall;
        if (supportsStreaming && !Set.of("sandbox_executor", "workflow").contains(component)) {
            throw new IllegalArgumentException("streaming is only valid for sandbox and workflow");
        }
        if ("workflow_checkpoint_store".equals(component)) {
            if (claimHeartbeatIntervalMs == null
                    || claimHeartbeatIntervalMs.compareTo(BigInteger.ONE) < 0
                    || claimHeartbeatIntervalMs.compareTo(BigInteger.valueOf(300_000)) > 0) {
                throw new IllegalArgumentException(
                        "workflow checkpoint stores require a claim heartbeat from 1 to 300000 ms");
            }
        } else if (claimHeartbeatIntervalMs != null) {
            throw new IllegalArgumentException(
                    "claim heartbeat is only valid for workflow checkpoint stores");
        }
    }

    public static AgentComponentDescriptor of(String component, String name) {
        return new AgentComponentDescriptor(component, name, null, false, false, null);
    }

    public static AgentComponentDescriptor sandbox(String name, String isolationLevel) {
        if (!Set.of("none", "process", "os-sandbox", "container", "orchestrated")
                .contains(isolationLevel)) {
            throw new IllegalArgumentException("unknown sandbox isolation level");
        }
        return new AgentComponentDescriptor("sandbox_executor", name, isolationLevel, false, false, null);
    }

    public static AgentComponentDescriptor sandbox(
            String name, String isolationLevel, boolean supportsStreaming) {
        if (!Set.of("none", "process", "os-sandbox", "container", "orchestrated")
                .contains(isolationLevel)) {
            throw new IllegalArgumentException("unknown sandbox isolation level");
        }
        return new AgentComponentDescriptor(
                "sandbox_executor", name, isolationLevel, supportsStreaming, false, null);
    }

    public static AgentComponentDescriptor workflow(String name, boolean supportsStreaming) {
        return new AgentComponentDescriptor("workflow", name, null, supportsStreaming, false, null);
    }

    public static AgentComponentDescriptor mcpTransport(String name, boolean supportsNotifications) {
        return new AgentComponentDescriptor("mcp_transport", name, null, false, supportsNotifications, null);
    }

    public static AgentComponentDescriptor workflowCheckpointStore(
            String name, BigInteger claimHeartbeatIntervalMs) {
        return new AgentComponentDescriptor(
                "workflow_checkpoint_store", name, null, false, false, claimHeartbeatIntervalMs);
    }

    public static AgentComponentDescriptor managedConversationStore(String name) {
        return of("conversation_store", name).withManagedCapabilities(
                ManagedPersistence.ConversationProjectionCapability.atomic_v1,
                ManagedPersistence.RuntimeStateCapability.unsupported,
                ManagedPersistence.PersistenceCallCapability.absolute_deadline_v1);
    }

    public static AgentComponentDescriptor managedRuntimeStateStore(String name) {
        return of("runtime_state_store", name).withManagedCapabilities(
                ManagedPersistence.ConversationProjectionCapability.unsupported,
                ManagedPersistence.RuntimeStateCapability.revisioned_v1,
                ManagedPersistence.PersistenceCallCapability.absolute_deadline_v1);
    }

    public AgentComponentDescriptor withManagedCapabilities(
            ManagedPersistence.ConversationProjectionCapability conversationProjection,
            ManagedPersistence.RuntimeStateCapability runtimeState,
            ManagedPersistence.PersistenceCallCapability persistenceCall) {
        return new AgentComponentDescriptor(component, name, isolationLevel, supportsStreaming,
                supportsNotifications, claimHeartbeatIntervalMs, conversationProjection, runtimeState, persistenceCall, executionCasV1, protectedClaimV1);
    }

    /** Opt into the existing typed execution CAS callback; framework alone owns transition semantics. */
    public AgentComponentDescriptor withExecutionCasV1(boolean supported) {
        return new AgentComponentDescriptor(component, name, isolationLevel, supportsStreaming,
                supportsNotifications, claimHeartbeatIntervalMs, conversationProjection, runtimeState, persistenceCall, supported, protectedClaimV1);
    }

    /** Promise that this same Store protects exact attempts before effects, not just CRUD. */
    public AgentComponentDescriptor withProtectedClaimV1(boolean supported) {
        return new AgentComponentDescriptor(component, name, isolationLevel, supportsStreaming,
                supportsNotifications, claimHeartbeatIntervalMs, conversationProjection, runtimeState,
                persistenceCall, executionCasV1, supported);
    }

    @Override public String kind() { return "agent_component"; }

    @Override public ObjectNode toJson() {
        var descriptor = TypedExtensionSupport.baseDescriptor(kind())
                .put("component", component)
                .put("name", name);
        var capabilities = descriptor.putObject("capabilities");
        capabilities.put("conversation_projection", conversationProjection.name());
        capabilities.put("runtime_state", runtimeState.name());
        capabilities.put("persistence_call", persistenceCall.name());
        capabilities.put("execution_cas_v1", executionCasV1);
        capabilities.put("protected_claim_v1", protectedClaimV1);
        if (isolationLevel == null) capabilities.putNull("isolation_level");
        else capabilities.put("isolation_level", isolationLevel);
        capabilities.put("supports_streaming", supportsStreaming);
        capabilities.put("supports_notifications", supportsNotifications);
        if (claimHeartbeatIntervalMs != null) {
            capabilities.put("claim_heartbeat_interval_ms", claimHeartbeatIntervalMs.toString());
        }
        return descriptor;
    }
}
