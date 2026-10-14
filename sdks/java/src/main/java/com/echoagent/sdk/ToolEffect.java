package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import java.math.BigInteger;

/** Framework-emitted execution facts; display output is never used to infer these effects. */
public sealed interface ToolEffect permits ToolEffect.FileRead, ToolEffect.FileEdit, ToolEffect.TestRun, ToolEffect.SubagentRun {
    ObjectNode toJson();
    record FileRead(String path) implements ToolEffect {
        public FileRead { require(path, "path"); }
        @Override public ObjectNode toJson() { return base("file_read").put("path", path); }
    }
    record FileEdit(String path) implements ToolEffect {
        public FileEdit { require(path, "path"); }
        @Override public ObjectNode toJson() { return base("file_edit").put("path", path); }
    }
    record TestRun(String command, boolean passed, BigInteger failureCount) implements ToolEffect {
        public TestRun { require(command, "command"); if (failureCount != null) WireValues.u64(failureCount); }
        @Override public ObjectNode toJson() {
            var result = base("test_run").put("command", command).put("passed", passed);
            if (failureCount != null) result.put("failure_count", failureCount.toString());
            return result;
        }
    }
    record SubagentRun(String agentName, String task, String outcome) implements ToolEffect {
        public SubagentRun { require(agentName, "agent_name"); require(task, "task"); require(outcome, "outcome"); }
        @Override public ObjectNode toJson() { return base("subagent_run").put("agent_name", agentName).put("task", task).put("outcome", outcome); }
    }
    static ToolEffect from(JsonNode value) {
        String effect = text(value, "effect");
        return switch (effect) {
            case "file_read" -> { ManagedPersistence.fields(value, "effect", "path"); yield new FileRead(text(value, "path")); }
            case "file_edit" -> { ManagedPersistence.fields(value, "effect", "path"); yield new FileEdit(text(value, "path")); }
            case "test_run" -> {
                ManagedPersistence.fields(value, "effect", "command", "passed", "failure_count");
                if (!value.path("passed").isBoolean()) throw new IllegalArgumentException("passed must be boolean");
                var count = value.get("failure_count");
                BigInteger failureCount = count == null || count.isNull() ? null
                        : new BigInteger(TypedExtensionSupport.canonicalU64(text(value, "failure_count"), "failure_count"));
                yield new TestRun(text(value, "command"), value.path("passed").booleanValue(), failureCount);
            }
            case "subagent_run" -> {
                ManagedPersistence.fields(value, "effect", "agent_name", "task", "outcome");
                yield new SubagentRun(text(value, "agent_name"), text(value, "task"), text(value, "outcome"));
            }
            default -> throw new IllegalArgumentException("unknown tool effect: " + effect);
        };
    }
    private static ObjectNode base(String effect) { return JsonSupport.MAPPER.createObjectNode().put("effect", effect); }
    private static void require(String value, String field) { if (value == null) throw new IllegalArgumentException(field + " must not be null"); }
    private static String text(JsonNode value, String field) {
        if (value == null || !value.isObject() || !value.path(field).isTextual()) throw new IllegalArgumentException(field + " must be text");
        return value.path(field).textValue();
    }
}
