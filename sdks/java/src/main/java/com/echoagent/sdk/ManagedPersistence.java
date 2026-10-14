package com.echoagent.sdk;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.*;
import com.fasterxml.jackson.databind.json.JsonMapper;
import com.fasterxml.jackson.databind.module.SimpleModule;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.ser.std.StdSerializer;
import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.annotation.JsonInclude;
import java.io.IOException;
import java.math.BigInteger;
import java.util.List;
import java.util.Map;
import java.util.Set;

/** Lossless managed-store facts. These values do not implement CAS, fencing or recovery. */
public final class ManagedPersistence {
    private ManagedPersistence() {}
    public interface Value { default JsonNode toJson() { return encode(this); } }

    private static final ObjectMapper CODEC = codec();
    private static ObjectMapper codec() {
        var module = new SimpleModule();
        module.addSerializer(BigInteger.class, new StdSerializer<BigInteger>(BigInteger.class) {
            @Override public void serialize(BigInteger value, JsonGenerator generator, SerializerProvider provider) throws IOException {
                generator.writeString(value.toString());
            }
        });
        module.addDeserializer(BigInteger.class, new StdDeserializer<BigInteger>(BigInteger.class) {
            @Override public BigInteger deserialize(JsonParser parser, DeserializationContext context) throws IOException {
                if (!parser.hasToken(com.fasterxml.jackson.core.JsonToken.VALUE_STRING)) {
                    throw new IllegalArgumentException("managed integer must be canonical decimal text");
                }
                String value = parser.getText();
                if (!value.matches("-?(0|[1-9][0-9]*)") || "-0".equals(value)) {
                    throw new IllegalArgumentException("managed integer must be canonical decimal text");
                }
                return new BigInteger(value);
            }
        });
        module.addSerializer(TranscriptProjectionApplyStatus.class, new StdSerializer<TranscriptProjectionApplyStatus>(TranscriptProjectionApplyStatus.class) {
            @Override public void serialize(TranscriptProjectionApplyStatus value, JsonGenerator generator, SerializerProvider provider) throws IOException {
                generator.writeTree(unionJson(value));
            }
        });
        module.addDeserializer(TranscriptProjectionApplyStatus.class, new StdDeserializer<TranscriptProjectionApplyStatus>(TranscriptProjectionApplyStatus.class) {
            @Override public TranscriptProjectionApplyStatus deserialize(JsonParser parser, DeserializationContext context) throws IOException {
                return TranscriptProjectionApplyStatus.from(parser.readValueAsTree());
            }
        });
        module.addSerializer(RuntimeStateVersion.class, new StdSerializer<RuntimeStateVersion>(RuntimeStateVersion.class) {
            @Override public void serialize(RuntimeStateVersion value, JsonGenerator generator, SerializerProvider provider) throws IOException {
                generator.writeTree(unionJson(value));
            }
        });
        module.addDeserializer(RuntimeStateVersion.class, new StdDeserializer<RuntimeStateVersion>(RuntimeStateVersion.class) {
            @Override public RuntimeStateVersion deserialize(JsonParser parser, DeserializationContext context) throws IOException {
                return RuntimeStateVersion.from(parser.readValueAsTree());
            }
        });
        module.addSerializer(RuntimeStateExpectedVersion.class, new StdSerializer<RuntimeStateExpectedVersion>(RuntimeStateExpectedVersion.class) {
            @Override public void serialize(RuntimeStateExpectedVersion value, JsonGenerator generator, SerializerProvider provider) throws IOException {
                generator.writeTree(unionJson(value));
            }
        });
        module.addDeserializer(RuntimeStateExpectedVersion.class, new StdDeserializer<RuntimeStateExpectedVersion>(RuntimeStateExpectedVersion.class) {
            @Override public RuntimeStateExpectedVersion deserialize(JsonParser parser, DeserializationContext context) throws IOException {
                return RuntimeStateExpectedVersion.from(parser.readValueAsTree());
            }
        });
        module.addSerializer(ScopeRetirementAdvance.class, new StdSerializer<ScopeRetirementAdvance>(ScopeRetirementAdvance.class) {
            @Override public void serialize(ScopeRetirementAdvance value, JsonGenerator generator, SerializerProvider provider) throws IOException {
                generator.writeTree(unionJson(value));
            }
        });
        module.addDeserializer(ScopeRetirementAdvance.class, new StdDeserializer<ScopeRetirementAdvance>(ScopeRetirementAdvance.class) {
            @Override public ScopeRetirementAdvance deserialize(JsonParser parser, DeserializationContext context) throws IOException {
                return ScopeRetirementAdvance.from(parser.readValueAsTree());
            }
        });
        module.addSerializer(CheckpointPath.class, new StdSerializer<CheckpointPath>(CheckpointPath.class) {
            @Override public void serialize(CheckpointPath value, JsonGenerator generator, SerializerProvider provider) throws IOException {
                ObjectNode object = JsonSupport.MAPPER.createObjectNode();
                if (value instanceof CheckpointPath.Utf8 p) object.put("encoding", "utf8").put("path", p.path());
                else if (value instanceof CheckpointPath.Unix p) {
                    object.put("encoding", "unix").put("bytes_base64", p.bytesBase64());
                    if (p.display() != null) object.put("display", p.display());
                } else if (value instanceof CheckpointPath.Windows p) {
                    object.put("encoding", "windows").put("utf16_base64", p.utf16Base64());
                    if (p.display() != null) object.put("display", p.display());
                }
                generator.writeTree(object);
            }
        });
        module.addDeserializer(CheckpointPath.class, new StdDeserializer<CheckpointPath>(CheckpointPath.class) {
            @Override public CheckpointPath deserialize(JsonParser parser, DeserializationContext context) throws IOException {
                JsonNode value = parser.readValueAsTree();
                return switch (text(value, "encoding")) {
                    case "utf8" -> { fields(value, "encoding", "path"); yield new CheckpointPath.Utf8(text(value, "path")); }
                    case "unix" -> { fields(value, "encoding", "bytes_base64", "display"); yield new CheckpointPath.Unix(text(value, "bytes_base64"), nullableText(value, "display")); }
                    case "windows" -> { fields(value, "encoding", "utf16_base64", "display"); yield new CheckpointPath.Windows(text(value, "utf16_base64"), nullableText(value, "display")); }
                    default -> throw new IllegalArgumentException("unknown checkpoint path encoding");
                };
            }
        });
        return JsonMapper.builder().propertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE)
                .disable(MapperFeature.ALLOW_COERCION_OF_SCALARS)
                .disable(DeserializationFeature.ACCEPT_FLOAT_AS_INT)
                .enable(DeserializationFeature.FAIL_ON_NUMBERS_FOR_ENUMS)
                .enable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES)
                .enable(DeserializationFeature.FAIL_ON_NULL_FOR_PRIMITIVES).addModule(module).build();
    }
    public static JsonNode encode(Value value) {
        if (value == null) throw new IllegalArgumentException("managed value must not be null");
        return CODEC.valueToTree(value);
    }
    public static <T extends Value> T decode(JsonNode value, Class<T> type) {
        if (value == null || value.isNull()) throw new IllegalArgumentException("managed value must not be null");
        try { return CODEC.treeToValue(value, type); }
        catch (IOException | IllegalArgumentException error) {
            throw new IllegalArgumentException("invalid managed " + type.getSimpleName(), error);
        }
    }
    /** Decode the accepted settlement event's WireValue map without coercing other DTO scalar contracts. */
    public static TranscriptProjectionSettlement decodeSettlementEvent(JsonNode wire) {
        if (wire == null || !wire.isObject() || !"map".equals(wire.path("kind").textValue())
                || !wire.path("value").isArray()) throw new IllegalArgumentException("settlement event requires WireValue map");
        fields(wire, "kind", "value");
        var names = Set.of("status", "operation_id", "conversation_id", "generation_id", "attempt", "error_class", "detail");
        var object = JsonSupport.MAPPER.createObjectNode();
        for (JsonNode entry : wire.path("value")) {
            fields(entry, "key", "value");
            JsonNode key = entry.get("key");
            if (key == null || !key.isObject() || !"string".equals(key.path("kind").textValue())
                    || !key.path("value").isTextual()) throw new IllegalArgumentException("settlement field key must be wire string");
            fields(key, "kind", "value");
            String name = key.path("value").textValue();
            if (!names.contains(name) || object.has(name)) throw new IllegalArgumentException("unknown or duplicate settlement field: " + name);
            JsonNode value = entry.get("value");
            if (value == null || !value.isObject()) throw new IllegalArgumentException("settlement field requires explicit wire value");
            if ("attempt".equals(name)) {
                fields(value, "kind", "value");
                if (!"u64".equals(value.path("kind").textValue()) || !value.path("value").isTextual()) {
                    throw new IllegalArgumentException("settlement attempt must be wire u64");
                }
                String canonical = value.path("value").textValue();
                WireValues.u64(canonical);
                BigInteger attempt = new BigInteger(canonical);
                if (attempt.compareTo(BigInteger.valueOf(4294967295L)) > 0) throw new IllegalArgumentException("settlement attempt must fit u32");
                object.put(name, attempt.longValueExact());
            } else if ("null".equals(value.path("kind").textValue())) {
                fields(value, "kind");
                if ("status".equals(name)) throw new IllegalArgumentException("settlement status must not be null");
                object.putNull(name);
            } else {
                fields(value, "kind", "value");
                if (!"string".equals(value.path("kind").textValue()) || !value.path("value").isTextual()) {
                    throw new IllegalArgumentException("settlement " + name + " must be wire string or null");
                }
                object.put(name, value.path("value").textValue());
            }
        }
        if (object.size() != names.size()) throw new IllegalArgumentException("settlement event is missing fields");
        return decode(object, TranscriptProjectionSettlement.class);
    }

    private static <T> T required(T value, String name) {
        if (value == null) throw new IllegalArgumentException(name + " must not be null");
        return value;
    }
    private static BigInteger unsigned(BigInteger value, String name) {
        WireValues.u64(required(value, name)); return value;
    }
    private static BigInteger signed(BigInteger value, String name) {
        WireValues.i64(required(value, name)); return value;
    }
    private static Integer schema(Integer value) {
        if (value == null || value < 0 || value > 65535) throw new IllegalArgumentException("schema_version must be u16");
        return value;
    }
    private static Long uint32(Long value, String name) {
        if (value == null || value < 0 || value > 4294967295L) throw new IllegalArgumentException(name + " must be u32");
        return value;
    }
    static void fields(JsonNode node, String... allowed) {
        if (node == null || !node.isObject()) throw new IllegalArgumentException("expected object");
        var names = Set.of(allowed);
        node.fieldNames().forEachRemaining(field -> {
            if (!names.contains(field)) throw new IllegalArgumentException("unknown field: " + field);
        });
    }
    private interface Union extends Value {
        String variant();
        Map<String, ?> payload();
    }
    private static JsonNode unionJson(Union value) {
        if (value.payload() == null) return com.fasterxml.jackson.databind.node.TextNode.valueOf(value.variant());
        var object = CODEC.createObjectNode();
        object.set(value.variant(), CODEC.valueToTree(value.payload()));
        return object;
    }
    private static String variant(JsonNode value) {
        if (value.isTextual()) return value.textValue();
        if (value.isObject() && value.size() == 1) return value.fieldNames().next();
        throw new IllegalArgumentException("invalid managed variant");
    }
    private static JsonNode payload(JsonNode value, String tag, String... names) {
        if (!value.isObject()) throw new IllegalArgumentException("variant requires payload");
        JsonNode payload = value.get(tag); fields(payload, names); return payload;
    }
    private static BigInteger number(JsonNode value, String field, boolean unsigned) {
        JsonNode node = value.get(field);
        if (node == null || !node.isTextual()) throw new IllegalArgumentException(field + " must be canonical integer text");
        String text = node.textValue();
        if (!text.matches("-?(0|[1-9][0-9]*)") || "-0".equals(text)) throw new IllegalArgumentException("noncanonical integer");
        BigInteger result = new BigInteger(text);
        return unsigned ? unsigned(result, field) : signed(result, field);
    }
    private static String text(JsonNode value, String field) {
        JsonNode node = value.get(field);
        if (node == null || !node.isTextual()) throw new IllegalArgumentException(field + " must be text");
        return node.textValue();
    }
    private static String nullableText(JsonNode value, String field) {
        var node = value.get(field); return node == null || node.isNull() ? null : text(value, field);
    }
    public enum ConversationProjectionCapability { unsupported, atomic_v1 }
    public enum PersistenceCallCapability { unsupported, absolute_deadline_v1 }
    public enum ConversationProjectionLifecycle { live, deleted }
    public enum ConversationProjectionEpochStatus { created, adopted_legacy, existing, recreated, tombstoned, epoch_conflict }
    public enum TranscriptProjectionConflictKind { operation_identity, ordinal_digest, revision, managed_import }
    public enum TranscriptProjectionSettlementStatus { settled, deferred, blocked, conflict }
    public enum TranscriptProjectionErrorClass { outcome_unknown, transient_no_commit, revision_conflict, deadline_exceeded, unsupported, invalid_configuration, corrupt_state, semantic_conflict }
    public enum ManagedConversationDeleteStatus { deleted, already_deleted, epoch_conflict, receipt_expired, identity_conflict }
    public enum RuntimeStateCapability { unsupported, revisioned_v1 }
    public enum RuntimeScopeLifecycle { active, retiring, tombstoned }
    public enum RuntimeCheckpointCasStatus { applied, already_current, revision_conflict, generation_retired, scope_fenced }
    public enum RuntimeGenerationRetireStatus { retired, already_retired, conflict, pending_projection, scope_fenced }
    public enum ScopeRetirementItemStatus { pending, dropped_by_delete }
    public enum ScopeRetirementStatus { begun, in_progress, completed, already_completed, revision_conflict, epoch_conflict, receipt_expired, identity_conflict }
    public record PersistenceCallContext(BigInteger absoluteDeadlineUnixMs) implements Value {
        public PersistenceCallContext {
            absoluteDeadlineUnixMs = signed(absoluteDeadlineUnixMs, "absolute_deadline_unix_ms");
        }
    }

    public record ConversationProjectionAuthority(String conversationId,
            BigInteger epoch,
            BigInteger revision,
            ConversationProjectionLifecycle lifecycle,
            BigInteger deleteReceiptRetentionFloorEpoch) implements Value {
        public ConversationProjectionAuthority {
            required(conversationId, "conversation_id");
            epoch = unsigned(epoch, "epoch");
            revision = unsigned(revision, "revision");
            required(lifecycle, "lifecycle");
            deleteReceiptRetentionFloorEpoch = unsigned(deleteReceiptRetentionFloorEpoch, "delete_receipt_retention_floor_epoch");
        }
    }

    public record NewConversation(String conversationId,
            String userId,
            String agentType,
            String title) implements Value {
        public NewConversation {
            required(conversationId, "conversation_id");
            required(userId, "user_id");
        }
    }

    public record StoredMessage(BigInteger id,
            String conversationId,
            String role,
            String content,
            String attachmentsJson,
            String toolCallsJson,
            String toolResultJson,
            String createdAt) implements Value {
        public StoredMessage {
            if (id != null) { id = signed(id, "id"); }
            required(conversationId, "conversation_id");
            required(role, "role");
            required(createdAt, "created_at");
        }
    }

    public record EnsureConversationProjectionRequest(NewConversation conversation,
            BigInteger expectedTombstoneEpoch) implements Value {
        public EnsureConversationProjectionRequest {
            required(conversation, "conversation");
            if (expectedTombstoneEpoch != null) { expectedTombstoneEpoch = unsigned(expectedTombstoneEpoch, "expected_tombstone_epoch"); }
        }
    }

    public record ConversationProjectionEpochReceipt(ConversationProjectionAuthority authority,
            ConversationProjectionEpochStatus status) implements Value {
        public ConversationProjectionEpochReceipt {
            required(authority, "authority");
            required(status, "status");
        }
    }

    public record TranscriptProjectionItem(BigInteger ordinal,
            String digest,
            StoredMessage message) implements Value {
        public TranscriptProjectionItem {
            ordinal = unsigned(ordinal, "ordinal");
            required(digest, "digest");
            required(message, "message");
        }
    }

    public record TranscriptProjectionBatch(Integer schemaVersion,
            String operationId,
            String payloadDigest,
            String conversationId,
            BigInteger conversationEpoch,
            String generationId,
            List<TranscriptProjectionItem> items) implements Value {
        public TranscriptProjectionBatch {
            schemaVersion = schema(schemaVersion);
            required(operationId, "operation_id");
            required(payloadDigest, "payload_digest");
            required(conversationId, "conversation_id");
            conversationEpoch = unsigned(conversationEpoch, "conversation_epoch");
            required(generationId, "generation_id");
            items = List.copyOf(required(items, "items"));
        }
    }

    public record TranscriptProjectionApplyReceipt(String operationId,
            String payloadDigest,
            ConversationProjectionAuthority authority,
            TranscriptProjectionApplyStatus status) implements Value {
        public TranscriptProjectionApplyReceipt {
            required(operationId, "operation_id");
            required(payloadDigest, "payload_digest");
            required(authority, "authority");
            required(status, "status");
        }
    }

    public record TranscriptProjectionSettlement(TranscriptProjectionSettlementStatus status,
            String operationId,
            String conversationId,
            String generationId,
            Long attempt,
            TranscriptProjectionErrorClass errorClass,
            String detail) implements Value {
        public TranscriptProjectionSettlement {
            required(status, "status");
            attempt = uint32(attempt, "attempt");
        }
    }

    public record ManagedConversationDelete(Integer schemaVersion,
            String operationId,
            String payloadDigest,
            String conversationId,
            BigInteger expectedEpoch) implements Value {
        public ManagedConversationDelete {
            schemaVersion = schema(schemaVersion);
            required(operationId, "operation_id");
            required(payloadDigest, "payload_digest");
            required(conversationId, "conversation_id");
            expectedEpoch = unsigned(expectedEpoch, "expected_epoch");
        }
    }

    public record ManagedConversationDeleteReceipt(String operationId,
            String payloadDigest,
            BigInteger deletedEpoch,
            BigInteger retentionFloorEpoch,
            ManagedConversationDeleteStatus status) implements Value {
        public ManagedConversationDeleteReceipt {
            required(operationId, "operation_id");
            required(payloadDigest, "payload_digest");
            deletedEpoch = unsigned(deletedEpoch, "deleted_epoch");
            retentionFloorEpoch = unsigned(retentionFloorEpoch, "retention_floor_epoch");
            required(status, "status");
        }
    }

    public record ManagedConversationImport(Integer schemaVersion,
            String operationId,
            String payloadDigest,
            String conversationId,
            BigInteger expectedEpoch,
            BigInteger expectedRevision,
            @JsonInclude(JsonInclude.Include.NON_NULL) String generationId,
            List<StoredMessage> messages) implements Value {
        public ManagedConversationImport {
            schemaVersion = schema(schemaVersion);
            required(operationId, "operation_id");
            required(payloadDigest, "payload_digest");
            required(conversationId, "conversation_id");
            expectedEpoch = unsigned(expectedEpoch, "expected_epoch");
            expectedRevision = unsigned(expectedRevision, "expected_revision");
            messages = List.copyOf(required(messages, "messages"));
        }
    }

    public record RuntimeScopeAuthority(String scopeId,
            BigInteger conversationEpoch,
            BigInteger revision,
            RuntimeScopeLifecycle lifecycle) implements Value {
        public RuntimeScopeAuthority {
            required(scopeId, "scope_id");
            if (conversationEpoch != null) { conversationEpoch = unsigned(conversationEpoch, "conversation_epoch"); }
            revision = unsigned(revision, "revision");
            required(lifecycle, "lifecycle");
        }
    }

    public record ManagedRuntimeStateSnapshot(RuntimeScopeAuthority scope,
            String runtimeStateId,
            RuntimeStateVersion version,
            AgentCheckpoint checkpoint) implements Value {
        public ManagedRuntimeStateSnapshot {
            required(scope, "scope");
            required(runtimeStateId, "runtime_state_id");
            required(version, "version");
        }
    }

    public record RuntimeCheckpointCasRequest(String scopeId,
            String runtimeStateId,
            BigInteger conversationEpoch,
            BigInteger expectedScopeRevision,
            RuntimeStateExpectedVersion expectedStateVersion,
            AgentCheckpoint checkpoint,
            @JsonInclude(JsonInclude.Include.NON_NULL) ManagedImportEpochTransition managedImport) implements Value {
        public RuntimeCheckpointCasRequest {
            required(scopeId, "scope_id");
            required(runtimeStateId, "runtime_state_id");
            if (conversationEpoch != null) { conversationEpoch = unsigned(conversationEpoch, "conversation_epoch"); }
            expectedScopeRevision = unsigned(expectedScopeRevision, "expected_scope_revision");
            required(expectedStateVersion, "expected_state_version");
            required(checkpoint, "checkpoint");
        }
    }

    public record ManagedImportEpochTransition(ManagedConversationImport request,
            TranscriptProjectionApplyReceipt receipt) implements Value {
        public ManagedImportEpochTransition {
            required(request, "request");
            required(receipt, "receipt");
        }
    }

    public record RuntimeCheckpointCasReceipt(RuntimeScopeAuthority scope,
            String runtimeStateId,
            RuntimeStateVersion version,
            RuntimeCheckpointCasStatus status) implements Value {
        public RuntimeCheckpointCasReceipt {
            required(scope, "scope");
            required(runtimeStateId, "runtime_state_id");
            required(version, "version");
            required(status, "status");
        }
    }

    public record RuntimeTranscriptAckRequest(String scopeId,
            String runtimeStateId,
            BigInteger conversationEpoch,
            BigInteger expectedScopeRevision,
            RuntimeStateExpectedVersion expectedStateVersion,
            TranscriptProjectionApplyReceipt projectionReceipt,
            AgentCheckpoint checkpoint) implements Value {
        public RuntimeTranscriptAckRequest {
            required(scopeId, "scope_id");
            required(runtimeStateId, "runtime_state_id");
            conversationEpoch = unsigned(conversationEpoch, "conversation_epoch");
            expectedScopeRevision = unsigned(expectedScopeRevision, "expected_scope_revision");
            required(expectedStateVersion, "expected_state_version");
            required(projectionReceipt, "projection_receipt");
            required(checkpoint, "checkpoint");
        }
    }

    public record RuntimeGenerationRetireRequest(Integer schemaVersion,
            String operationId,
            String payloadDigest,
            String scopeId,
            String runtimeStateId,
            BigInteger expectedScopeRevision,
            RuntimeStateExpectedVersion expectedStateVersion) implements Value {
        public RuntimeGenerationRetireRequest {
            schemaVersion = schema(schemaVersion);
            required(operationId, "operation_id");
            required(payloadDigest, "payload_digest");
            required(scopeId, "scope_id");
            required(runtimeStateId, "runtime_state_id");
            expectedScopeRevision = unsigned(expectedScopeRevision, "expected_scope_revision");
            required(expectedStateVersion, "expected_state_version");
        }
    }

    public record RuntimeGenerationRetireReceipt(RuntimeScopeAuthority scope,
            String runtimeStateId,
            RuntimeStateVersion version,
            RuntimeGenerationRetireStatus status) implements Value {
        public RuntimeGenerationRetireReceipt {
            required(scope, "scope");
            required(runtimeStateId, "runtime_state_id");
            required(version, "version");
            required(status, "status");
        }
    }

    public record ScopeRetirementItem(String runtimeStateId,
            RuntimeStateVersion stateVersion,
            String pendingOperationId,
            ScopeRetirementItemStatus status) implements Value {
        public ScopeRetirementItem {
            required(runtimeStateId, "runtime_state_id");
            required(stateVersion, "state_version");
            required(status, "status");
        }
    }

    public record ScopeRetirementManifest(String deleteOperationId,
            String payloadDigest,
            BigInteger expectedConversationEpoch,
            List<ScopeRetirementItem> items,
            ManagedConversationDeleteReceipt conversationDeleteReceipt) implements Value {
        public ScopeRetirementManifest {
            required(deleteOperationId, "delete_operation_id");
            required(payloadDigest, "payload_digest");
            expectedConversationEpoch = unsigned(expectedConversationEpoch, "expected_conversation_epoch");
            items = List.copyOf(required(items, "items"));
        }
    }

    public record ScopeRetirementRequest(Integer schemaVersion,
            String deleteOperationId,
            String payloadDigest,
            String scopeId,
            BigInteger expectedScopeRevision,
            BigInteger expectedConversationEpoch) implements Value {
        public ScopeRetirementRequest {
            schemaVersion = schema(schemaVersion);
            required(deleteOperationId, "delete_operation_id");
            required(payloadDigest, "payload_digest");
            required(scopeId, "scope_id");
            expectedScopeRevision = unsigned(expectedScopeRevision, "expected_scope_revision");
            expectedConversationEpoch = unsigned(expectedConversationEpoch, "expected_conversation_epoch");
        }
    }

    public record ScopeRetirementReceipt(RuntimeScopeAuthority scope,
            ScopeRetirementManifest manifest,
            List<String> droppedOperationIds,
            BigInteger retentionFloorEpoch,
            ScopeRetirementStatus status) implements Value {
        public ScopeRetirementReceipt {
            required(scope, "scope");
            required(manifest, "manifest");
            droppedOperationIds = List.copyOf(required(droppedOperationIds, "dropped_operation_ids"));
            retentionFloorEpoch = unsigned(retentionFloorEpoch, "retention_floor_epoch");
            required(status, "status");
        }
    }

    public record AgentCheckpoint(String conversationId,
            String messagesJson,
            String currentPlan,
            List<String> activeSkills,
            String blockedReason,
            CheckpointPath workingDir,
            CheckpointTimestamp timestamp) implements Value {
        public AgentCheckpoint {
            required(conversationId, "conversation_id");
            required(messagesJson, "messages_json");
            activeSkills = List.copyOf(required(activeSkills, "active_skills"));
            required(timestamp, "timestamp");
        }
    }

    public sealed interface TranscriptProjectionApplyStatus extends Union permits TranscriptProjectionApplyStatus.Applied, TranscriptProjectionApplyStatus.AlreadyApplied, TranscriptProjectionApplyStatus.Fenced, TranscriptProjectionApplyStatus.Conflict {
        record Applied() implements TranscriptProjectionApplyStatus {
            @Override public String variant() { return "applied"; }
            @Override public Map<String, ?> payload() { return null; }
        }
        record AlreadyApplied() implements TranscriptProjectionApplyStatus {
            @Override public String variant() { return "already_applied"; }
            @Override public Map<String, ?> payload() { return null; }
        }
        record Fenced(BigInteger currentEpoch, ConversationProjectionLifecycle lifecycle) implements TranscriptProjectionApplyStatus {
            public Fenced {
            currentEpoch = unsigned(currentEpoch, "current_epoch");
            required(lifecycle, "lifecycle");
            }
            @Override public String variant() { return "fenced"; }
            @Override public Map<String, ?> payload() { return Map.of("current_epoch", currentEpoch, "lifecycle", lifecycle); }
        }
        record Conflict(BigInteger currentEpoch, BigInteger currentRevision, TranscriptProjectionConflictKind kind) implements TranscriptProjectionApplyStatus {
            public Conflict {
            currentEpoch = unsigned(currentEpoch, "current_epoch");
            currentRevision = unsigned(currentRevision, "current_revision");
            required(kind, "kind");
            }
            @Override public String variant() { return "conflict"; }
            @Override public Map<String, ?> payload() { return Map.of("current_epoch", currentEpoch, "current_revision", currentRevision, "kind", kind); }
        }
        static TranscriptProjectionApplyStatus from(JsonNode value) {
            String tag = ManagedPersistence.variant(value);
            return switch (tag) {
                case "applied" -> {
                    if (!value.isTextual()) throw new IllegalArgumentException("unit variant must be text");
                    yield new Applied();
                }
                case "already_applied" -> {
                    if (!value.isTextual()) throw new IllegalArgumentException("unit variant must be text");
                    yield new AlreadyApplied();
                }
                case "fenced" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "current_epoch", "lifecycle");
                    yield new Fenced(number(data, "current_epoch", true), ConversationProjectionLifecycle.valueOf(text(data, "lifecycle")));
                }
                case "conflict" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "current_epoch", "current_revision", "kind");
                    yield new Conflict(number(data, "current_epoch", true), number(data, "current_revision", true), TranscriptProjectionConflictKind.valueOf(text(data, "kind")));
                }
                default -> throw new IllegalArgumentException("unknown TranscriptProjectionApplyStatus variant: " + tag);
            };
        }
    }

    public sealed interface RuntimeStateVersion extends Union permits RuntimeStateVersion.Absent, RuntimeStateVersion.Unmanaged, RuntimeStateVersion.Managed, RuntimeStateVersion.Retired {
        record Absent() implements RuntimeStateVersion {
            @Override public String variant() { return "absent"; }
            @Override public Map<String, ?> payload() { return null; }
        }
        record Unmanaged(String digest) implements RuntimeStateVersion {
            public Unmanaged {
            required(digest, "digest");
            }
            @Override public String variant() { return "unmanaged"; }
            @Override public Map<String, ?> payload() { return Map.of("digest", digest); }
        }
        record Managed(BigInteger revision) implements RuntimeStateVersion {
            public Managed {
            revision = unsigned(revision, "revision");
            }
            @Override public String variant() { return "managed"; }
            @Override public Map<String, ?> payload() { return Map.of("revision", revision); }
        }
        record Retired(BigInteger revision, String operationId) implements RuntimeStateVersion {
            public Retired {
            revision = unsigned(revision, "revision");
            required(operationId, "operation_id");
            }
            @Override public String variant() { return "retired"; }
            @Override public Map<String, ?> payload() { return Map.of("revision", revision, "operation_id", operationId); }
        }
        static RuntimeStateVersion from(JsonNode value) {
            String tag = ManagedPersistence.variant(value);
            return switch (tag) {
                case "absent" -> {
                    if (!value.isTextual()) throw new IllegalArgumentException("unit variant must be text");
                    yield new Absent();
                }
                case "unmanaged" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "digest");
                    yield new Unmanaged(text(data, "digest"));
                }
                case "managed" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "revision");
                    yield new Managed(number(data, "revision", true));
                }
                case "retired" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "revision", "operation_id");
                    yield new Retired(number(data, "revision", true), text(data, "operation_id"));
                }
                default -> throw new IllegalArgumentException("unknown RuntimeStateVersion variant: " + tag);
            };
        }
    }

    public sealed interface RuntimeStateExpectedVersion extends Union permits RuntimeStateExpectedVersion.Absent, RuntimeStateExpectedVersion.Unmanaged, RuntimeStateExpectedVersion.Managed {
        record Absent() implements RuntimeStateExpectedVersion {
            @Override public String variant() { return "absent"; }
            @Override public Map<String, ?> payload() { return null; }
        }
        record Unmanaged(String digest) implements RuntimeStateExpectedVersion {
            public Unmanaged {
            required(digest, "digest");
            }
            @Override public String variant() { return "unmanaged"; }
            @Override public Map<String, ?> payload() { return Map.of("digest", digest); }
        }
        record Managed(BigInteger revision) implements RuntimeStateExpectedVersion {
            public Managed {
            revision = unsigned(revision, "revision");
            }
            @Override public String variant() { return "managed"; }
            @Override public Map<String, ?> payload() { return Map.of("revision", revision); }
        }
        static RuntimeStateExpectedVersion from(JsonNode value) {
            String tag = ManagedPersistence.variant(value);
            return switch (tag) {
                case "absent" -> {
                    if (!value.isTextual()) throw new IllegalArgumentException("unit variant must be text");
                    yield new Absent();
                }
                case "unmanaged" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "digest");
                    yield new Unmanaged(text(data, "digest"));
                }
                case "managed" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "revision");
                    yield new Managed(number(data, "revision", true));
                }
                default -> throw new IllegalArgumentException("unknown RuntimeStateExpectedVersion variant: " + tag);
            };
        }
    }

    public sealed interface ScopeRetirementAdvance extends Union permits ScopeRetirementAdvance.ConversationDeleted, ScopeRetirementAdvance.GenerationDropped, ScopeRetirementAdvance.Complete {
        record ConversationDeleted(ManagedConversationDeleteReceipt receipt) implements ScopeRetirementAdvance {
            public ConversationDeleted {
            required(receipt, "receipt");
            }
            @Override public String variant() { return "conversation_deleted"; }
            @Override public Map<String, ?> payload() { return Map.of("receipt", receipt); }
        }
        record GenerationDropped(String runtimeStateId, String pendingOperationId) implements ScopeRetirementAdvance {
            public GenerationDropped {
            required(runtimeStateId, "runtime_state_id");
            }
            @Override public String variant() { return "generation_dropped"; }
            @Override public Map<String, ?> payload() {
                var value = new java.util.LinkedHashMap<String, Object>();
                value.put("runtime_state_id", runtimeStateId);
                value.put("pending_operation_id", pendingOperationId);
                return value;
            }
        }
        record Complete() implements ScopeRetirementAdvance {
            @Override public String variant() { return "complete"; }
            @Override public Map<String, ?> payload() { return null; }
        }
        static ScopeRetirementAdvance from(JsonNode value) {
            String tag = ManagedPersistence.variant(value);
            return switch (tag) {
                case "conversation_deleted" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "receipt");
                    yield new ConversationDeleted(decode(data.get("receipt"), ManagedConversationDeleteReceipt.class));
                }
                case "generation_dropped" -> {
                    JsonNode data = ManagedPersistence.payload(value, tag, "runtime_state_id", "pending_operation_id");
                    yield new GenerationDropped(text(data, "runtime_state_id"), nullableText(data, "pending_operation_id"));
                }
                case "complete" -> {
                    if (!value.isTextual()) throw new IllegalArgumentException("unit variant must be text");
                    yield new Complete();
                }
                default -> throw new IllegalArgumentException("unknown ScopeRetirementAdvance variant: " + tag);
            };
        }
    }

    public record CheckpointTimestamp(BigInteger unixSeconds, Long nanos,
            @JsonInclude(JsonInclude.Include.NON_NULL) String rfc3339) implements Value {
        public CheckpointTimestamp {
            unixSeconds = signed(unixSeconds, "unix_seconds");
            if (nanos == null || nanos < 0 || nanos >= 1_000_000_000) throw new IllegalArgumentException("invalid timestamp nanos");
        }
    }
    public sealed interface CheckpointPath extends Value permits CheckpointPath.Utf8, CheckpointPath.Unix, CheckpointPath.Windows {
        record Utf8(String path) implements CheckpointPath {
            public Utf8 { WireValues.utf8Path(path); }
        }
        record Unix(String bytesBase64, String display) implements CheckpointPath {
            public Unix { validatePathBytes(bytesBase64, false); }
        }
        record Windows(String utf16Base64, String display) implements CheckpointPath {
            public Windows { validatePathBytes(utf16Base64, true); }
        }
    }
    private static void validatePathBytes(String base64, boolean windows) {
        if (base64 == null || base64.contains("=")) throw new IllegalArgumentException("path requires unpadded base64");
        byte[] bytes;
        try { bytes = java.util.Base64.getDecoder().decode(base64); }
        catch (IllegalArgumentException error) { throw new IllegalArgumentException("invalid path base64", error); }
        if (!java.util.Base64.getEncoder().withoutPadding().encodeToString(bytes).equals(base64)) throw new IllegalArgumentException("noncanonical path base64");
        if (!windows) {
            if (bytes.length == 0 || bytes[0] != '/') throw new IllegalArgumentException("Unix path must be absolute");
            for (byte item : bytes) if (item == 0) throw new IllegalArgumentException("NUL in Unix path");
        } else {
            if (bytes.length % 2 != 0) throw new IllegalArgumentException("invalid UTF-16 path");
            var chars = new StringBuilder();
            for (int i = 0; i < bytes.length; i += 2) {
                char c = (char) ((bytes[i] & 255) | (bytes[i + 1] & 255) << 8);
                if (c == 0) throw new IllegalArgumentException("NUL in Windows path");
                chars.append(c);
            }
            String path = chars.toString();
            if (!(path.startsWith("\\\\") || path.matches("[A-Za-z]:[\\\\/].*"))) throw new IllegalArgumentException("Windows path must be absolute");
        }
    }
}
