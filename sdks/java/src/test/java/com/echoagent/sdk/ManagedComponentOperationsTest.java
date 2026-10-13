package com.echoagent.sdk;

import com.fasterxml.jackson.databind.JsonNode;
import org.junit.jupiter.api.Test;
import java.math.BigInteger;
import java.util.List;
import static org.junit.jupiter.api.Assertions.*;
import static com.echoagent.sdk.ManagedPersistence.*;

class ManagedComponentOperationsTest {
    private static final BigInteger U = new BigInteger("18446744073709551615");
    private static final PersistenceCallContext CONTEXT = new PersistenceCallContext(new BigInteger("9223372036854775807"));
    private static JsonNode input(String key, Value value) {
        var result = JsonSupport.MAPPER.createObjectNode(); result.set(key, value.toJson());
        result.set("context", CONTEXT.toJson()); return result;
    }
    private static AgentComponentRequest request(String operation, JsonNode input) {
        var call = JsonSupport.MAPPER.createObjectNode().put("operation", operation); call.set("input", input);
        return AgentComponentRequest.from(call);
    }
    @Test void allElevenManagedOperationsRetainTypedInputsAndReceipts() {
        var conversation = new NewConversation("conversation中文", "owner", "agent", "title😀");
        var ensure = new EnsureConversationProjectionRequest(conversation, U);
        assertEquals(ensure, assertInstanceOf(AgentComponentRequest.ConversationEnsureProjectionEpochWithContext.class,
                request("conversation_ensure_projection_epoch_with_context", input("request", ensure))).request());
        var authorityInput = JsonSupport.MAPPER.createObjectNode().put("conversation_id", conversation.conversationId()); authorityInput.set("context", CONTEXT.toJson());
        assertEquals(CONTEXT, assertInstanceOf(AgentComponentRequest.ConversationGetProjectionAuthorityWithContext.class,
                request("conversation_get_projection_authority_with_context", authorityInput)).context());
        var message = new StoredMessage(new BigInteger("-9223372036854775808"), conversation.conversationId(), "tool", "内容😀", "attachments", "calls", "result", "timestamp");
        var batch = new TranscriptProjectionBatch(65535, "operation", "digest", conversation.conversationId(), U, "generation", List.of(new TranscriptProjectionItem(U, "digest", message)));
        assertEquals(batch, assertInstanceOf(AgentComponentRequest.ConversationApplyTranscriptProjectionWithContext.class,
                request("conversation_apply_transcript_projection_with_context", input("batch", batch))).batch());
        var delete = new ManagedConversationDelete(65535, "delete", "digest", conversation.conversationId(), U);
        assertEquals(delete, assertInstanceOf(AgentComponentRequest.ConversationDeleteManagedWithContext.class,
                request("conversation_delete_managed_with_context", input("request", delete))).request());
        var scopeInput = JsonSupport.MAPPER.createObjectNode().put("scope_id", "scope"); scopeInput.set("context", CONTEXT.toJson());
        assertEquals("scope", assertInstanceOf(AgentComponentRequest.RuntimeLoadScopeAuthorityWithContext.class,
                request("runtime_load_scope_authority_with_context", scopeInput)).scopeId());
        var stateInput = scopeInput.deepCopy().put("runtime_state_id", "state");
        assertEquals("state", assertInstanceOf(AgentComponentRequest.RuntimeLoadManagedStateWithContext.class,
                request("runtime_load_managed_state_with_context", stateInput)).runtimeStateId());
        var authority = new ConversationProjectionAuthority(conversation.conversationId(), U, U, ConversationProjectionLifecycle.live, U);
        var receipt = new TranscriptProjectionApplyReceipt("operation", "digest", authority, new TranscriptProjectionApplyStatus.Fenced(U, ConversationProjectionLifecycle.deleted));
        var checkpoint = new AgentCheckpoint(conversation.conversationId(), "{\"schema_version\":5}", null, List.of(), null,
                new CheckpointPath.Windows("QwA6AFwAeAA", "C:\\x"), new CheckpointTimestamp(BigInteger.valueOf(-1), 123456789L, "1969-12-31T23:59:59.123456789Z"));
        var expected = new RuntimeStateExpectedVersion.Unmanaged("digest");
        var transition = new ManagedImportEpochTransition(new ManagedConversationImport(1, "import", "digest", conversation.conversationId(), U, U, "generation", List.of(message)), receipt);
        var cas = new RuntimeCheckpointCasRequest("scope", "state", U, U, expected, checkpoint, transition);
        assertEquals(cas, assertInstanceOf(AgentComponentRequest.RuntimeCompareAndSaveCheckpointWithContext.class,
                request("runtime_compare_and_save_checkpoint_with_context", input("request", cas))).request());
        var ack = new RuntimeTranscriptAckRequest("scope", "state", U, U, expected, receipt, checkpoint);
        assertEquals(ack, assertInstanceOf(AgentComponentRequest.RuntimeAcknowledgeTranscriptProjectionWithContext.class,
                request("runtime_acknowledge_transcript_projection_with_context", input("request", ack))).request());
        var retire = new RuntimeGenerationRetireRequest(1, "retire", "digest", "scope", "state", U, expected);
        assertEquals(retire, assertInstanceOf(AgentComponentRequest.RuntimeRetireGenerationWithContext.class,
                request("runtime_retire_generation_with_context", input("request", retire))).request());
        var begin = new ScopeRetirementRequest(1, "delete", "digest", "scope", U, U);
        assertEquals(begin, assertInstanceOf(AgentComponentRequest.RuntimeBeginScopeRetirementWithContext.class,
                request("runtime_begin_scope_retirement_with_context", input("request", begin))).request());
        var advance = new ScopeRetirementAdvance.GenerationDropped("state", "pending");
        var continueInput = scopeInput.deepCopy().put("delete_operation_id", "delete").put("expected_scope_revision", U.toString()); continueInput.set("advance", advance.toJson());
        assertEquals(advance, assertInstanceOf(AgentComponentRequest.RuntimeContinueScopeRetirementWithContext.class,
                request("runtime_continue_scope_retirement_with_context", continueInput)).advance());
        var scope = new RuntimeScopeAuthority("scope", U, U, RuntimeScopeLifecycle.retiring);
        var version = new RuntimeStateVersion.Retired(U, "operation");
        var snapshot = new ManagedRuntimeStateSnapshot(scope, "state", version, checkpoint);
        var casReceipt = new RuntimeCheckpointCasReceipt(scope, "state", version, RuntimeCheckpointCasStatus.scope_fenced);
        var retireReceipt = new RuntimeGenerationRetireReceipt(scope, "state", version, RuntimeGenerationRetireStatus.pending_projection);
        var deleteReceipt = new ManagedConversationDeleteReceipt("delete", "digest", U, U, ManagedConversationDeleteStatus.receipt_expired);
        var manifest = new ScopeRetirementManifest("delete", "digest", U, List.of(new ScopeRetirementItem("state", version, "pending", ScopeRetirementItemStatus.dropped_by_delete)), deleteReceipt);
        var scopeReceipt = new ScopeRetirementReceipt(scope, manifest, List.of("pending"), U, ScopeRetirementStatus.already_completed);
        var results = List.<AgentComponentResult>of(
                new AgentComponentResult.ConversationProjectionEpochEnsured(new ConversationProjectionEpochReceipt(authority, ConversationProjectionEpochStatus.recreated)),
                new AgentComponentResult.ConversationProjectionAuthorityLoaded(authority), new AgentComponentResult.ConversationProjectionApplied(receipt),
                new AgentComponentResult.ManagedConversationDeleted(deleteReceipt), new AgentComponentResult.RuntimeManagedStateLoaded(snapshot),
                new AgentComponentResult.RuntimeScopeAuthorityLoaded(scope), new AgentComponentResult.RuntimeCheckpointComparedAndSaved(casReceipt),
                new AgentComponentResult.RuntimeTranscriptProjectionAcknowledged(casReceipt), new AgentComponentResult.RuntimeGenerationRetired(retireReceipt),
                new AgentComponentResult.RuntimeScopeRetirementBegun(scopeReceipt), new AgentComponentResult.RuntimeScopeRetirementContinued(scopeReceipt));
        for (var result : results) {
            var outcome = AgentComponentOutcome.result(result.component(), result).toJson();
            assertEquals(result.operation(), outcome.path("result").path("value").path("result").path("operation").textValue());
            assertEquals(result.value(), outcome.path("result").path("value").path("result").path("value"));
        }
        for (Value value : List.of(snapshot, casReceipt, retireReceipt, deleteReceipt, scopeReceipt)) {
            assertEquals(value, ManagedPersistence.decode(value.toJson(), value.getClass()));
        }
    }
    @Test void componentEnvelopeCannotRouteManagedOperationToWrongComponent() {
        var input = JsonSupport.MAPPER.createObjectNode().put("component", "run_store");
        var invocation = JsonSupport.MAPPER.createObjectNode().put("operation", "agent_component_call"); invocation.set("input", input);
        var call = JsonSupport.MAPPER.createObjectNode().put("operation", "conversation_get_projection_authority_with_context");
        var query = JsonSupport.MAPPER.createObjectNode().put("conversation_id", "c"); query.set("context", CONTEXT.toJson());
        call.set("input", query); input.set("call", call);
        var envelope = JsonSupport.MAPPER.createObjectNode(); envelope.set("invocation", invocation);
        assertThrows(IllegalArgumentException.class, () -> AgentComponentCall.from(envelope));
        input.put("component", "conversation_store"); assertEquals("conversation_store", AgentComponentCall.from(envelope).component());
        query.put("unknown", "bad"); assertThrows(IllegalArgumentException.class, () -> AgentComponentCall.from(envelope));
    }
    private static JsonNode requestFields(AgentComponentRequest request) throws ReflectiveOperationException {
        var fields = JsonSupport.MAPPER.createObjectNode();
        for (var field : request.getClass().getRecordComponents()) {
            String wireName = field.getName().replaceAll("([a-z0-9])([A-Z])", "$1_$2").toLowerCase(java.util.Locale.ROOT);
            Object value = field.getAccessor().invoke(request);
            if (value == null) fields.putNull(wireName);
            else if (value instanceof ManagedPersistence.Value managed) fields.set(wireName, managed.toJson());
            else if (value instanceof String text) fields.put(wireName, text);
            else if (value instanceof BigInteger integer) fields.put(wireName, integer.toString());
            else throw new IllegalArgumentException("unexpected managed request field type: " + value.getClass());
        }
        return fields;
    }
    private static <T extends ManagedPersistence.Value> T nullableFact(JsonNode value, Class<T> type) {
        return value == null || value.isNull() ? null : ManagedPersistence.decode(value, type);
    }
    private static AgentComponentResult corpusResult(String operation, JsonNode result) {
        return switch (operation) {
            case "conversation_ensure_projection_epoch_with_context" -> new AgentComponentResult.ConversationProjectionEpochEnsured(
                    ManagedPersistence.decode(result.path("receipt"), ConversationProjectionEpochReceipt.class));
            case "conversation_get_projection_authority_with_context" -> new AgentComponentResult.ConversationProjectionAuthorityLoaded(
                    nullableFact(result.get("authority"), ConversationProjectionAuthority.class));
            case "conversation_apply_transcript_projection_with_context" -> new AgentComponentResult.ConversationProjectionApplied(
                    ManagedPersistence.decode(result.path("receipt"), TranscriptProjectionApplyReceipt.class));
            case "conversation_delete_managed_with_context" -> new AgentComponentResult.ManagedConversationDeleted(
                    ManagedPersistence.decode(result.path("receipt"), ManagedConversationDeleteReceipt.class));
            case "runtime_load_managed_state_with_context" -> new AgentComponentResult.RuntimeManagedStateLoaded(
                    nullableFact(result.get("snapshot"), ManagedRuntimeStateSnapshot.class));
            case "runtime_load_scope_authority_with_context" -> new AgentComponentResult.RuntimeScopeAuthorityLoaded(
                    nullableFact(result.get("authority"), RuntimeScopeAuthority.class));
            case "runtime_compare_and_save_checkpoint_with_context" -> new AgentComponentResult.RuntimeCheckpointComparedAndSaved(
                    ManagedPersistence.decode(result.path("receipt"), RuntimeCheckpointCasReceipt.class));
            case "runtime_acknowledge_transcript_projection_with_context" -> new AgentComponentResult.RuntimeTranscriptProjectionAcknowledged(
                    ManagedPersistence.decode(result.path("receipt"), RuntimeCheckpointCasReceipt.class));
            case "runtime_retire_generation_with_context" -> new AgentComponentResult.RuntimeGenerationRetired(
                    ManagedPersistence.decode(result.path("receipt"), RuntimeGenerationRetireReceipt.class));
            case "runtime_begin_scope_retirement_with_context" -> new AgentComponentResult.RuntimeScopeRetirementBegun(
                    ManagedPersistence.decode(result.path("receipt"), ScopeRetirementReceipt.class));
            case "runtime_continue_scope_retirement_with_context" -> new AgentComponentResult.RuntimeScopeRetirementContinued(
                    ManagedPersistence.decode(result.path("receipt"), ScopeRetirementReceipt.class));
            default -> throw new IllegalArgumentException("unexpected shared managed scenario operation: " + operation);
        };
    }
    @Test void sharedElevenOperationCorpusRoundTripsThroughTypedCallbackInputsAndReceipts() throws Exception {
        var corpus = JsonSupport.MAPPER.readTree(java.nio.file.Path.of("../shared/fixtures/managed-context-corpus.json").toFile());
        assertTrue(corpus.isArray()); assertEquals(11, corpus.size());
        var scenarios = new java.util.HashSet<String>();
        var operations = new java.util.HashSet<String>();
        for (var item : corpus) {
            String scenario = item.path("scenario_id").textValue();
            String operation = item.path("operation").textValue();
            String component = item.path("component").textValue();
            assertTrue(scenarios.add(scenario), "duplicate managed scenario");
            assertTrue(operations.add(operation), "duplicate managed operation");
            var wireCall = JsonSupport.MAPPER.createObjectNode().put("operation", operation);
            wireCall.set("input", item.path("input"));
            var request = AgentComponentRequest.from(wireCall);
            assertEquals(operation, request.operation(), scenario);
            assertEquals(component, request.component(), scenario);
            assertEquals(item.path("input"), JsonSupport.MAPPER.readTree(requestFields(request).toString()), scenario);
            var componentInput = JsonSupport.MAPPER.createObjectNode().put("component", component); componentInput.set("call", wireCall);
            var invocation = JsonSupport.MAPPER.createObjectNode().put("operation", "agent_component_call"); invocation.set("input", componentInput);
            var envelope = JsonSupport.MAPPER.createObjectNode(); envelope.set("invocation", invocation);
            assertEquals(request, AgentComponentCall.from(envelope).request(), scenario);
            var result = corpusResult(operation, item.path("result"));
            assertEquals(operation, result.operation(), scenario); assertEquals(component, result.component(), scenario);
            // Jackson IntNode/LongNode represent the same JSON integer; compare the actual encoded JSON contract.
            assertEquals(item.path("result"), JsonSupport.MAPPER.readTree(result.value().toString()), scenario);
            var outcome = AgentComponentOutcome.result(component, result).toJson();
            assertEquals("result", outcome.path("outcome").textValue(), scenario);
            assertEquals("agent_component_call", outcome.path("result").path("operation").textValue(), scenario);
            assertEquals(component, outcome.path("result").path("value").path("component").textValue(), scenario);
            assertEquals(operation, outcome.path("result").path("value").path("result").path("operation").textValue(), scenario);
            assertEquals(item.path("result"), JsonSupport.MAPPER.readTree(outcome.path("result").path("value").path("result").path("value").toString()), scenario);
        }
    }
}
