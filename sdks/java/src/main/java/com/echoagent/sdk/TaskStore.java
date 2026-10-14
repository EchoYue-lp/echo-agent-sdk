package com.echoagent.sdk;

import com.fasterxml.jackson.annotation.*;
import com.fasterxml.jackson.databind.JsonNode;
import java.math.BigInteger;
import java.util.List;
import java.util.Map;

/** Typed revisioned task-store projections; framework remains the graph and execution authority. */
public final class TaskStore {
    private TaskStore() {}
    public interface Value extends ManagedPersistence.Value {}
    private static <T> T required(T value, String name) {
        if (value == null) throw new IllegalArgumentException(name + " must not be null");
        return value;
    }
    private static Long uint32(Long value, String name) {
        if (value == null || value < 0 || value > 4294967295L) throw new IllegalArgumentException(name + " must be u32");
        return value;
    }
    private static BigInteger unsigned(BigInteger value) { WireValues.u64(value); return value; }
    private static JsonNode wire(JsonNode value) {
        if (value == null || !value.isObject() || !value.path("kind").isTextual()) throw new IllegalArgumentException("expected WireValue");
        return JsonSupport.validateWire(value);
    }
    public record TaskSpec(String id, String title, String description, List<String> dependsOn,
                           Long maxRetries, JsonNode extension) implements Value {
        public TaskSpec {
            required(id, "id"); required(title, "title"); required(description, "description");
            dependsOn = List.copyOf(required(dependsOn, "depends_on"));
            maxRetries = uint32(maxRetries, "max_retries"); extension = wire(extension);
        }
        @Override public JsonNode extension() { return extension.deepCopy(); }
    }
    public record TaskClaim(String claimId, BigInteger revision, Long attempt, String specHash) implements Value {
        public TaskClaim {
            required(claimId, "claim_id"); revision = unsigned(revision);
            attempt = uint32(attempt, "attempt"); required(specHash, "spec_hash");
        }
    }
    @JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "status")
    @JsonSubTypes({
            @JsonSubTypes.Type(value = TaskStatus.Pending.class, name = "pending"),
            @JsonSubTypes.Type(value = TaskStatus.Running.class, name = "running"),
            @JsonSubTypes.Type(value = TaskStatus.Blocked.class, name = "blocked"),
            @JsonSubTypes.Type(value = TaskStatus.Completed.class, name = "completed"),
            @JsonSubTypes.Type(value = TaskStatus.Failed.class, name = "failed"),
            @JsonSubTypes.Type(value = TaskStatus.Skipped.class, name = "skipped"),
            @JsonSubTypes.Type(value = TaskStatus.Cancelled.class, name = "cancelled"),
            @JsonSubTypes.Type(value = TaskStatus.TimedOut.class, name = "timed_out"),
            @JsonSubTypes.Type(value = TaskStatus.Retrying.class, name = "retrying"),
            @JsonSubTypes.Type(value = TaskStatus.Paused.class, name = "paused")})
    public sealed interface TaskStatus extends Value {
        record Pending() implements TaskStatus {}
        record Running() implements TaskStatus {}
        record Blocked(String reason) implements TaskStatus { public Blocked { required(reason, "reason"); } }
        record Completed() implements TaskStatus {}
        record Failed(String error) implements TaskStatus { public Failed { required(error, "error"); } }
        record Skipped() implements TaskStatus {}
        record Cancelled() implements TaskStatus {}
        record TimedOut(String error) implements TaskStatus { public TimedOut { required(error, "error"); } }
        record Retrying(Long attempt, String lastError) implements TaskStatus {
            public Retrying { attempt = uint32(attempt, "attempt"); required(lastError, "last_error"); }
        }
        record Paused(String reason) implements TaskStatus { public Paused { required(reason, "reason"); } }
    }
    public sealed interface RuntimeInterruptionDisposition extends Value {
        record Cancelled() implements RuntimeInterruptionDisposition {}
        record Paused(String reason) implements RuntimeInterruptionDisposition {
            public Paused { required(reason, "reason"); }
        }
        @JsonValue default JsonNode wireJson() {
            if (this instanceof Cancelled) return com.fasterxml.jackson.databind.node.TextNode.valueOf("cancelled");
            var object = JsonSupport.MAPPER.createObjectNode();
            object.set("paused", JsonSupport.MAPPER.createObjectNode().put("reason", ((Paused) this).reason()));
            return object;
        }
        @JsonCreator(mode = JsonCreator.Mode.DELEGATING)
        static RuntimeInterruptionDisposition from(JsonNode value) {
            if (value.isTextual() && "cancelled".equals(value.textValue())) return new Cancelled();
            ManagedPersistence.fields(value, "paused");
            ManagedPersistence.fields(value.get("paused"), "reason");
            if (!value.get("paused").path("reason").isTextual()) throw new IllegalArgumentException("paused requires reason");
            return new Paused(value.get("paused").path("reason").textValue());
        }
    }
    public record TaskExecution(String taskId, TaskStatus status, Long retryCount,
            String failureFingerprint, TaskClaim claim,
            @JsonInclude(JsonInclude.Include.NON_NULL) JsonNode result,
            @JsonInclude(JsonInclude.Include.NON_NULL) RuntimeInterruptionDisposition interruptionRequested) implements Value {
        public TaskExecution {
            required(taskId, "task_id"); required(status, "status"); retryCount = uint32(retryCount, "retry_count");
            if (result != null) result = wire(result);
        }
        @Override public JsonNode result() { return result == null ? null : result.deepCopy(); }
    }
    public record Task(TaskSpec spec, TaskExecution execution) implements Value {
        public Task { required(spec, "spec"); required(execution, "execution"); }
    }
    public record RuntimePlanSnapshot(BigInteger revision, List<Task> tasks) implements Value {
        public RuntimePlanSnapshot { revision = unsigned(revision); tasks = List.copyOf(required(tasks, "tasks")); }
    }
    public enum TaskGraphExecutionMode { sequential, parallel }
    public record TaskGraphContext(String goal, List<String> assumptions, List<String> risks,
            TaskGraphExecutionMode executionMode, JsonNode metadata) implements Value {
        public TaskGraphContext {
            required(goal, "goal"); assumptions = List.copyOf(required(assumptions, "assumptions"));
            risks = List.copyOf(required(risks, "risks")); required(executionMode, "execution_mode"); metadata = wire(metadata);
        }
        @Override public JsonNode metadata() { return metadata.deepCopy(); }
    }
    public record RevisionedTaskGraph(RuntimePlanSnapshot snapshot, TaskGraphContext context) implements Value {
        public RevisionedTaskGraph { required(snapshot, "snapshot"); required(context, "context"); }
    }
    public record TaskPatchEffects(List<String> insertedTaskIds, List<String> updatedTaskIds,
            List<String> skippedTaskIds, List<String> resetTaskIds, List<String> progressedTaskIds, Boolean reordered) implements Value {
        public TaskPatchEffects {
            insertedTaskIds = List.copyOf(required(insertedTaskIds, "inserted_task_ids"));
            updatedTaskIds = List.copyOf(required(updatedTaskIds, "updated_task_ids"));
            skippedTaskIds = List.copyOf(required(skippedTaskIds, "skipped_task_ids"));
            resetTaskIds = List.copyOf(required(resetTaskIds, "reset_task_ids"));
            progressedTaskIds = List.copyOf(required(progressedTaskIds, "progressed_task_ids")); required(reordered, "reordered");
        }
    }
    public record TaskGraphCommit(BigInteger expectedRevision,
            @JsonInclude(JsonInclude.Include.NON_NULL) Map<String, TaskExecution> expectedExecutions,
            RevisionedTaskGraph next, String reason, TaskPatchEffects effects) implements Value {
        public TaskGraphCommit {
            if (expectedRevision != null) expectedRevision = unsigned(expectedRevision);
            if (expectedExecutions != null) expectedExecutions = Map.copyOf(expectedExecutions);
            required(next, "next"); required(reason, "reason"); required(effects, "effects");
        }
    }
    public record TaskExecutionCommit(BigInteger expectedRevision, Map<String, TaskExecution> expectedExecutions,
            RevisionedTaskGraph next) implements Value {
        public TaskExecutionCommit {
            expectedRevision = unsigned(expectedRevision);
            expectedExecutions = Map.copyOf(required(expectedExecutions, "expected_executions")); required(next, "next");
        }
    }
}
