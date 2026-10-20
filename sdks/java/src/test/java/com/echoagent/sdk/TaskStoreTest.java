package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.math.BigInteger;
import java.util.List;
import java.util.Map;
import static org.junit.jupiter.api.Assertions.*;

class TaskStoreTest {
    private static final BigInteger MAX = new BigInteger("18446744073709551615");
    private static final JsonNode NULL = WireValues.value(null);
    private static TaskStore.RevisionedTaskGraph graph(JsonNode result) {
        var claim = new TaskStore.TaskClaim("claim😀", MAX, 4294967295L, "hash");
        var execution = new TaskStore.TaskExecution("t", new TaskStore.TaskStatus.Retrying(4294967295L, "中文"),
                4294967295L, "fingerprint", claim, result, new TaskStore.RuntimeInterruptionDisposition.Paused("pause😀"));
        var spec = new TaskStore.TaskSpec("t", "标题", "描述", List.of("dep"), 4294967295L,
                WireValues.value(Map.of("__proto__", "中文", "number", MAX)));
        return new TaskStore.RevisionedTaskGraph(new TaskStore.RuntimePlanSnapshot(MAX, List.of(new TaskStore.Task(spec, execution))),
                new TaskStore.TaskGraphContext("目标", List.of("assumption"), List.of("risk"), TaskStore.TaskGraphExecutionMode.parallel, NULL));
    }
    @Test void typedTaskGraphRetainsClaimStatusAndExplicitWireNull() {
        var graph = graph(NULL);
        var raw = graph.toJson();
        assertEquals("18446744073709551615", raw.path("snapshot").path("revision").textValue());
        assertEquals("retrying", raw.path("snapshot").path("tasks").path(0).path("execution").path("status").path("status").textValue());
        assertEquals("null", raw.path("snapshot").path("tasks").path(0).path("execution").path("result").path("kind").textValue());
        assertEquals(graph, ManagedPersistence.decode(raw, TaskStore.RevisionedTaskGraph.class));
        var absent = graph(null);
        assertFalse(absent.toJson().path("snapshot").path("tasks").path(0).path("execution").has("result"));
        assertEquals(absent, ManagedPersistence.decode(absent.toJson(), TaskStore.RevisionedTaskGraph.class));
    }
    @Test void allTaskStoreOperationsAcceptTypedCommitsAndReturnTypedGraphs() {
        var graph = graph(NULL);
        var effects = new TaskStore.TaskPatchEffects(List.of("new"), List.of("edit"), List.of(), List.of(), List.of("t"), true);
        var commit = new TaskStore.TaskGraphCommit(MAX, Map.of("t", graph.snapshot().tasks().get(0).execution()), graph, "reason", effects);
        var input = JsonSupport.MAPPER.createObjectNode().put("scope_id", "scope");
        input.set("commit", commit.toJson());
        var call = JsonSupport.MAPPER.createObjectNode().put("operation", "revisioned_task_compare_and_commit"); call.set("input", input);
        assertEquals(commit, assertInstanceOf(AgentComponentRequest.RevisionedTaskCompareAndCommit.class, AgentComponentRequest.from(call)).commit());
        var execution = new TaskStore.TaskExecutionCommit(MAX, commit.expectedExecutions(), graph);
        call.put("operation", "revisioned_task_compare_and_commit_execution"); input.set("commit", execution.toJson());
        assertEquals(execution, assertInstanceOf(AgentComponentRequest.RevisionedTaskCompareAndCommitExecution.class, AgentComponentRequest.from(call)).commit());
        assertEquals(graph.toJson(), new AgentComponentResult.RevisionedTaskExecutionCommitted(graph).value().path("graph"));
        assertEquals(graph.toJson(), new AgentComponentResult.RevisionedTaskCommitted(graph).value().path("graph"));
        assertTrue(new AgentComponentResult.RevisionedTaskLoaded(null).value().path("graph").isNull());
    }
    @Test void taskStatesPreserveAllVariantsAndRejectUnknownFields() {
        var states = List.<TaskStore.TaskStatus>of(new TaskStore.TaskStatus.Pending(), new TaskStore.TaskStatus.Running(),
                new TaskStore.TaskStatus.Blocked("blocked"), new TaskStore.TaskStatus.Completed(), new TaskStore.TaskStatus.Failed("failed"),
                new TaskStore.TaskStatus.Skipped(), new TaskStore.TaskStatus.Cancelled(), new TaskStore.TaskStatus.TimedOut("timeout"),
                new TaskStore.TaskStatus.Retrying(4294967295L, "retry"), new TaskStore.TaskStatus.Paused("pause"));
        for (var state : states) assertEquals(state, ManagedPersistence.decode(state.toJson(), TaskStore.TaskStatus.class));
        var bad = graph(NULL).toJson();
        ((com.fasterxml.jackson.databind.node.ObjectNode) bad.path("snapshot").path("tasks").path(0).path("execution")).put("unknown", "x");
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decode(bad, TaskStore.RevisionedTaskGraph.class));
        assertThrows(IllegalArgumentException.class, () -> new TaskStore.TaskClaim("c", MAX, 4294967296L, "hash"));
        assertThrows(IllegalArgumentException.class, () -> new TaskStore.TaskExecution("t", states.get(0), 0L, null, null,
                com.fasterxml.jackson.databind.node.NullNode.instance, null));
    }
}
