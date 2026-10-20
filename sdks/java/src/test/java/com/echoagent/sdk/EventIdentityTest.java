package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

class EventIdentityTest {
    @Test
    void eventIdentitiesPreserveValidationAndImmutableUpdates() {
        assertEquals("evt-1", EventId.newId("evt-1").asStr());
        assertEquals("stream-1", StreamId.newId("stream-1").toString());
        assertThrows(IllegalArgumentException.class, () -> EventId.newId("  "));
        EventIdentity identity = EventIdentity.newIdentity("stream-1", "turn-1")
                .withRunId("run-1")
                .withMessageId("message-1")
                .withExecutionId("exec-1")
                .withConversationId("conversation-1")
                .withParentEventId("event-0");
        assertEquals("stream-1", identity.streamId().asStr());
        assertEquals("turn-1", identity.turnId());
        assertEquals("event-0", identity.parentEventId());
        assertEquals("run-2", EventIdentity.forRun("run-2").executionId());
        assertEquals("message-2", EventIdentity.forChat("conversation-2", "turn-2", "message-2", "run-2").messageId());
        assertEquals("exec-3", EventIdentity.fromRuntimeContext(Map.of("run_id", "run-3", "execution_id", "exec-3")).turnId());
    }

    @Test
    void runtimeLineageParentSurvivesNestedMapAndInvocationProjection() {
        Map<String, Object> runtime = Map.of("turn_id", "turn-中文🙂", "run_id", "run-1",
                "conversation_id", "conversation-1", "message_id", "message-1",
                "execution_id", "execution-1", "subagent_lineage", Map.of("parent_event_id", "parent-中文🙂"));
        var identity = EventIdentity.fromRuntimeContext(runtime);
        assertEquals("parent-中文🙂", identity.parentEventId());
        assertEquals("turn-中文🙂", identity.turnId());
        assertEquals("run-1", identity.runId());
        assertEquals("conversation-1", identity.conversationId());
        assertEquals("message-1", identity.messageId());
        assertEquals("execution-1", identity.executionId());
        org.junit.jupiter.api.Assertions.assertFalse(identity.streamId().asStr().isBlank());
        assertEquals("changed", identity.withParentEventId("changed").parentEventId());
        assertEquals("parent-中文🙂", identity.parentEventId());
        assertEquals("parent-中文🙂", EventIdentity.fromInvocation(Map.of("runtime", runtime)).parentEventId());
        assertEquals(Map.of("parent_event_id", "parent-中文🙂"), runtime.get("subagent_lineage"));
        org.junit.jupiter.api.Assertions.assertNull(EventIdentity.fromRuntimeContext(Map.of("turn_id", "t")).parentEventId());
        org.junit.jupiter.api.Assertions.assertNull(EventIdentity.fromRuntimeContext(Map.of("turn_id", "t", "subagent_lineage", Map.of())).parentEventId());
        var mutable = new java.util.HashMap<String, Object>(runtime);
        mutable.put("subagent_lineage", null);
        org.junit.jupiter.api.Assertions.assertNull(EventIdentity.fromRuntimeContext(mutable).parentEventId());
        mutable.put("subagent_lineage", runtime.get("subagent_lineage"));
        mutable.put("parent_event_id", "flat");
        assertEquals("flat", EventIdentity.fromRuntimeContext(mutable).parentEventId());
        mutable.put("parent_event_id", null);
        assertEquals("parent-中文🙂", EventIdentity.fromRuntimeContext(mutable).parentEventId());
        // Keep Java's original blank-flat rejection rather than changing normalization.
        mutable.put("parent_event_id", "");
        assertThrows(IllegalArgumentException.class, () -> EventIdentity.fromRuntimeContext(mutable));
        assertEquals("7", EventIdentity.fromRuntimeContext(Map.of("turn_id", "t", "subagent_lineage", Map.of("parent_event_id", 7))).parentEventId());
        for (String parent : java.util.List.of("", " ")) {
            assertThrows(IllegalArgumentException.class, () -> EventIdentity.fromRuntimeContext(
                    Map.of("turn_id", "t", "subagent_lineage", Map.of("parent_event_id", parent))));
        }
    }

    @Test
    void eventIdentityMappingsAreComplete() throws Exception {
        JsonNode manifest = JsonSupport.MAPPER.readTree(
                Files.readString(Path.of("../../contracts/sdk/parity-manifest.json")));
        int count = 0;
        for (JsonNode entry : manifest.path("entries")) {
            if (entry.path("canonical").asBoolean()
                    && entry.path("languages").path("java").path("contract_test")
                    .asText().endsWith("/event_identity_values")) {
                count++;
                assertEquals("done", entry.path("languages").path("java").path("status").asText());
            }
        }
        assertEquals(29, count);
    }
}
