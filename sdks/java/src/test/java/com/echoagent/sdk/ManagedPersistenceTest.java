package com.echoagent.sdk;
import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.math.BigInteger;
import java.util.List;
import static org.junit.jupiter.api.Assertions.*;
class ManagedPersistenceTest {
    private static final BigInteger MAX = new BigInteger("18446744073709551615");
    private static JsonNode json(String value) throws Exception { return JsonSupport.MAPPER.readTree(value); }
    @Test void authorityRoundTripPreservesUnsignedBoundaryAndUnicode() throws Exception {
        var authority = new ManagedPersistence.ConversationProjectionAuthority(
                "中文😀", MAX, MAX, ManagedPersistence.ConversationProjectionLifecycle.live, MAX);
        assertEquals(MAX.toString(), authority.toJson().path("revision").textValue());
        assertEquals(authority, ManagedPersistence.decode(authority.toJson(), ManagedPersistence.ConversationProjectionAuthority.class));
        var invalid = authority.toJson().deepCopy();
        ((com.fasterxml.jackson.databind.node.ObjectNode) invalid).put("extra", 1);
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decode(invalid, ManagedPersistence.ConversationProjectionAuthority.class));
        for (String bad : List.of("01", "+1", "-0", "-1", "18446744073709551616")) {
            var malformed = authority.toJson().deepCopy();
            ((com.fasterxml.jackson.databind.node.ObjectNode) malformed).put("revision", bad);
            assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decode(malformed, ManagedPersistence.ConversationProjectionAuthority.class));
        }
    }
    @Test void managedCallAndReceiptAreTyped() throws Exception {
        var call = json("{\"operation\":\"conversation_get_projection_authority_with_context\",\"input\":{\"conversation_id\":\"中文\",\"context\":{\"absolute_deadline_unix_ms\":\"-9223372036854775808\"}}}");
        var request = assertInstanceOf(AgentComponentRequest.ConversationGetProjectionAuthorityWithContext.class, AgentComponentRequest.from(call));
        assertEquals(new BigInteger("-9223372036854775808"), request.context().absoluteDeadlineUnixMs());
        var authority = new ManagedPersistence.ConversationProjectionAuthority("中文", MAX, MAX, ManagedPersistence.ConversationProjectionLifecycle.deleted, MAX);
        var receipt = new ManagedPersistence.TranscriptProjectionApplyReceipt("op", "digest", authority,
                new ManagedPersistence.TranscriptProjectionApplyStatus.Conflict(MAX, MAX, ManagedPersistence.TranscriptProjectionConflictKind.managed_import));
        assertEquals(receipt, ManagedPersistence.decode(receipt.toJson(), ManagedPersistence.TranscriptProjectionApplyReceipt.class));
        var result = new AgentComponentResult.ConversationProjectionApplied(receipt);
        assertEquals(receipt.toJson(), result.value().path("receipt"));
    }
    @Test void managedDescriptorAdvertisesOnlyMatchingCapabilities() {
        var descriptor = AgentComponentDescriptor.managedConversationStore("store").toJson();
        assertEquals("atomic_v1", descriptor.path("capabilities").path("conversation_projection").textValue());
        assertEquals("absolute_deadline_v1", descriptor.path("capabilities").path("persistence_call").textValue());
        assertEquals("revisioned_v1", AgentComponentDescriptor.managedRuntimeStateStore("runtime").toJson().path("capabilities").path("runtime_state").textValue());
    }
    @Test void effectsAndInterruptedCallbacksPreserveCanonicalIdentity() throws Exception {
        var effects = List.<ToolEffect>of(new ToolEffect.FileRead("中文"), new ToolEffect.FileEdit("😀"), new ToolEffect.TestRun("test", false, MAX), new ToolEffect.SubagentRun("sub", "task", "cancelled"));
        var result = ToolResult.success("ok").withEffects(effects);
        assertEquals(MAX.toString(), result.toJson().path("effects").path(2).path("failure_count").textValue());
        assertEquals(effects.get(2), ToolEffect.from(result.toJson().path("effects").path(2)));
        var callback = CallbackCall.from(json("{\"invocation\":{\"operation\":\"callback_on_tool_interrupted\",\"input\":{\"agent\":\"中文\",\"call_id\":\"run/tool/1\",\"tool\":\"shell\",\"input\":{\"kind\":\"string\",\"value\":\"😀\"},\"error\":\"cancelled\"}}}"));
        assertEquals("run/tool/1", callback.callId());
        assertInstanceOf(CallbackCall.ToolInterrupted.class, callback.request());
    }
    @Test void checkpointAndSettlementRetainOpaqueSchemaFiveAndTypedPathTime() {
        String schemaFive = "{\"schema_version\":5,\"execution_facts\":[{\"call_id\":\"run/tool/1\",\"effects\":[{\"effect\":\"file_edit\",\"path\":\"中文😀\"}]}]}";
        var checkpoint = new ManagedPersistence.AgentCheckpoint("中文", schemaFive, "plan", List.of("skill"), "blocked",
                new ManagedPersistence.CheckpointPath.Unix("L3RtcC//", "non UTF-8"),
                new ManagedPersistence.CheckpointTimestamp(new BigInteger("-9223372036854775808"), 999999999L, null));
        assertEquals(checkpoint, ManagedPersistence.decode(checkpoint.toJson(), ManagedPersistence.AgentCheckpoint.class));
        assertEquals(schemaFive, checkpoint.toJson().path("messages_json").textValue());
        for (var status : ManagedPersistence.TranscriptProjectionSettlementStatus.values()) {
            var settlement = new ManagedPersistence.TranscriptProjectionSettlement(status, "op", "中文", "gen", 4294967295L,
                    ManagedPersistence.TranscriptProjectionErrorClass.outcome_unknown, "detail😀");
            assertEquals(settlement, ManagedPersistence.decode(settlement.toJson(), ManagedPersistence.TranscriptProjectionSettlement.class));
        }
    }
    @Test void unionPayloadsNeverIgnoreUnknownOrMissingFields() throws Exception {
        for (String invalid : List.of("{\"managed\":{\"revision\":\"1\",\"extra\":1}}", "{\"managed\":{}}", "{\"absent\":{}}", "\"future\"")) {
            var value = json(invalid);
            assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decode(value, ManagedPersistence.RuntimeStateVersion.class));
        }
        var invalid = json("{\"status\":\"settled\",\"attempt\":\"1\"}");
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decode(invalid, ManagedPersistence.TranscriptProjectionSettlement.class));
        assertThrows(IllegalArgumentException.class, () -> AgentComponentDescriptor.of("run_store", "x").withManagedCapabilities(
                ManagedPersistence.ConversationProjectionCapability.atomic_v1, ManagedPersistence.RuntimeStateCapability.unsupported,
                ManagedPersistence.PersistenceCallCapability.unsupported));
        assertThrows(IllegalArgumentException.class, () -> AgentComponentDescriptor.of("conversation_store", "x").withManagedCapabilities(
                ManagedPersistence.ConversationProjectionCapability.unsupported, ManagedPersistence.RuntimeStateCapability.revisioned_v1,
                ManagedPersistence.PersistenceCallCapability.absolute_deadline_v1));
    }
    @Test void numericEnumsAndFractionalNativeIntegersNeverCoerce() throws Exception {
        var authority = json("{\"conversation_id\":\"c\",\"epoch\":\"1\",\"revision\":\"1\",\"lifecycle\":0,\"delete_receipt_retention_floor_epoch\":\"0\"}");
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decode(authority, ManagedPersistence.ConversationProjectionAuthority.class));
        var settlement = json("{\"status\":\"settled\",\"attempt\":1.5}");
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decode(settlement, ManagedPersistence.TranscriptProjectionSettlement.class));
    }
    @Test void executionCasIsExplicitBooleanAndRestrictedToRevisionedTaskStore() {
        var ordinary = AgentComponentDescriptor.of("revisioned_task_store", "tasks");
        assertTrue(ordinary.toJson().path("capabilities").path("execution_cas_v1").isBoolean());
        assertFalse(ordinary.toJson().path("capabilities").path("execution_cas_v1").booleanValue());
        var execution = ordinary.withExecutionCasV1(true);
        assertTrue(execution.toJson().path("capabilities").path("execution_cas_v1").booleanValue());
        assertFalse(ordinary.toJson().path("capabilities").path("execution_cas_v1").booleanValue(), "descriptor updates must remain immutable");
        assertFalse(execution.withExecutionCasV1(false).toJson().path("capabilities").path("execution_cas_v1").booleanValue());
        for (String component : List.of("conversation_store", "runtime_state_store", "run_store", "guard", "workflow")) {
            var descriptor = AgentComponentDescriptor.of(component, "component");
            assertFalse(descriptor.toJson().path("capabilities").path("execution_cas_v1").booleanValue());
            assertThrows(IllegalArgumentException.class, () -> descriptor.withExecutionCasV1(true));
            assertFalse(descriptor.withExecutionCasV1(false).toJson().path("capabilities").path("execution_cas_v1").booleanValue());
        }
        assertTrue(execution.withManagedCapabilities(ManagedPersistence.ConversationProjectionCapability.unsupported,
                ManagedPersistence.RuntimeStateCapability.unsupported, ManagedPersistence.PersistenceCallCapability.unsupported)
                .toJson().path("capabilities").path("execution_cas_v1").booleanValue(), "other descriptor updates preserve the explicit opt-in");
    }
    private static com.fasterxml.jackson.databind.node.ObjectNode settlementEvent(String attempt) {
        var fields = new java.util.LinkedHashMap<String, Object>();
        fields.put("status", "deferred"); fields.put("operation_id", "operation中文🙂"); fields.put("conversation_id", "conversation中文🙂");
        fields.put("generation_id", "generation"); fields.put("attempt", WireValues.u64(attempt));
        fields.put("error_class", "outcome_unknown"); fields.put("detail", null);
        return WireValues.value(fields);
    }
    @Test void rawSettlementEventRetainsAttemptNumericMeaningWithoutGeneralStringCoercion() {
        var raw = settlementEvent("4294967295");
        var settlement = ManagedPersistence.decodeSettlementEvent(raw);
        assertEquals(4294967295L, settlement.attempt()); assertEquals("operation中文🙂", settlement.operationId());
        assertEquals(ManagedPersistence.TranscriptProjectionSettlementStatus.deferred, settlement.status());
        assertEquals(ManagedPersistence.TranscriptProjectionErrorClass.outcome_unknown, settlement.errorClass());
        assertEquals(raw, settlementEvent("4294967295"), "event decoding must not mutate the raw facts");
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decode(JsonSupport.fromWire(raw), ManagedPersistence.TranscriptProjectionSettlement.class));
    }
    @Test void rawSettlementEventRejectsMalformedAttemptAndUnknownOrMissingFields() {
        for (String attempt : List.of("4294967296", "18446744073709551615")) {
            var raw = settlementEvent(attempt); assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decodeSettlementEvent(raw));
        }
        for (var attempt : List.of(WireValues.value("1"), WireValues.value(1.5), WireValues.value(null), JsonSupport.MAPPER.createObjectNode().put("kind", "u64").put("value", "01"))) {
            var raw = settlementEvent("1");
            ((com.fasterxml.jackson.databind.node.ObjectNode) raw.path("value").path(4)).set("value", attempt);
            assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decodeSettlementEvent(raw));
        }
        var missing = settlementEvent("1"); ((com.fasterxml.jackson.databind.node.ArrayNode) missing.path("value")).remove(4);
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decodeSettlementEvent(missing));
        var unknown = settlementEvent("1"); ((com.fasterxml.jackson.databind.node.ArrayNode) unknown.path("value")).add(
                JsonSupport.MAPPER.createObjectNode().set("key", WireValues.value("future")));
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decodeSettlementEvent(unknown));
        var duplicate = settlementEvent("1"); ((com.fasterxml.jackson.databind.node.ArrayNode) duplicate.path("value")).add(duplicate.path("value").path(0).deepCopy());
        assertThrows(IllegalArgumentException.class, () -> ManagedPersistence.decodeSettlementEvent(duplicate));
    }
}
