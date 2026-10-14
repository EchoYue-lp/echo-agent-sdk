const schema = { "ConversationProjectionCapability": { "variants": { "unsupported": { "class": "ConversationProjectionCapabilityUnsupported", "fields": {} }, "atomic_v1": { "class": "ConversationProjectionCapabilityAtomicV1", "fields": {} } }, "unit": true }, "PersistenceCallCapability": { "variants": { "unsupported": { "class": "PersistenceCallCapabilityUnsupported", "fields": {} }, "absolute_deadline_v1": { "class": "PersistenceCallCapabilityAbsoluteDeadlineV1", "fields": {} } }, "unit": true }, "ConversationProjectionLifecycle": { "variants": { "live": { "class": "ConversationProjectionLifecycleLive", "fields": {} }, "deleted": { "class": "ConversationProjectionLifecycleDeleted", "fields": {} } }, "unit": true }, "ConversationProjectionEpochStatus": { "variants": { "created": { "class": "ConversationProjectionEpochStatusCreated", "fields": {} }, "adopted_legacy": { "class": "ConversationProjectionEpochStatusAdoptedLegacy", "fields": {} }, "existing": { "class": "ConversationProjectionEpochStatusExisting", "fields": {} }, "recreated": { "class": "ConversationProjectionEpochStatusRecreated", "fields": {} }, "tombstoned": { "class": "ConversationProjectionEpochStatusTombstoned", "fields": {} }, "epoch_conflict": { "class": "ConversationProjectionEpochStatusEpochConflict", "fields": {} } }, "unit": true }, "TranscriptProjectionConflictKind": { "variants": { "operation_identity": { "class": "TranscriptProjectionConflictKindOperationIdentity", "fields": {} }, "ordinal_digest": { "class": "TranscriptProjectionConflictKindOrdinalDigest", "fields": {} }, "revision": { "class": "TranscriptProjectionConflictKindRevision", "fields": {} }, "managed_import": { "class": "TranscriptProjectionConflictKindManagedImport", "fields": {} } }, "unit": true }, "TranscriptProjectionApplyStatus": { "variants": { "applied": { "class": "TranscriptProjectionApplyStatusApplied", "fields": {} }, "already_applied": { "class": "TranscriptProjectionApplyStatusAlreadyApplied", "fields": {} }, "fenced": { "class": "TranscriptProjectionApplyStatusFenced", "fields": { "current_epoch": "WireU64", "lifecycle": "ConversationProjectionLifecycleWire" } }, "conflict": { "class": "TranscriptProjectionApplyStatusConflict", "fields": { "current_epoch": "WireU64", "current_revision": "WireU64", "kind": "TranscriptProjectionConflictKindWire" } } }, "unit": false }, "TranscriptProjectionSettlementStatus": { "variants": { "settled": { "class": "TranscriptProjectionSettlementStatusSettled", "fields": {} }, "deferred": { "class": "TranscriptProjectionSettlementStatusDeferred", "fields": {} }, "blocked": { "class": "TranscriptProjectionSettlementStatusBlocked", "fields": {} }, "conflict": { "class": "TranscriptProjectionSettlementStatusConflict", "fields": {} } }, "unit": true }, "TranscriptProjectionErrorClass": { "variants": { "outcome_unknown": { "class": "TranscriptProjectionErrorClassOutcomeUnknown", "fields": {} }, "transient_no_commit": { "class": "TranscriptProjectionErrorClassTransientNoCommit", "fields": {} }, "revision_conflict": { "class": "TranscriptProjectionErrorClassRevisionConflict", "fields": {} }, "deadline_exceeded": { "class": "TranscriptProjectionErrorClassDeadlineExceeded", "fields": {} }, "unsupported": { "class": "TranscriptProjectionErrorClassUnsupported", "fields": {} }, "invalid_configuration": { "class": "TranscriptProjectionErrorClassInvalidConfiguration", "fields": {} }, "corrupt_state": { "class": "TranscriptProjectionErrorClassCorruptState", "fields": {} }, "semantic_conflict": { "class": "TranscriptProjectionErrorClassSemanticConflict", "fields": {} } }, "unit": true }, "ManagedConversationDeleteStatus": { "variants": { "deleted": { "class": "ManagedConversationDeleteStatusDeleted", "fields": {} }, "already_deleted": { "class": "ManagedConversationDeleteStatusAlreadyDeleted", "fields": {} }, "epoch_conflict": { "class": "ManagedConversationDeleteStatusEpochConflict", "fields": {} }, "receipt_expired": { "class": "ManagedConversationDeleteStatusReceiptExpired", "fields": {} }, "identity_conflict": { "class": "ManagedConversationDeleteStatusIdentityConflict", "fields": {} } }, "unit": true }, "RuntimeStateCapability": { "variants": { "unsupported": { "class": "RuntimeStateCapabilityUnsupported", "fields": {} }, "revisioned_v1": { "class": "RuntimeStateCapabilityRevisionedV1", "fields": {} } }, "unit": true }, "RuntimeStateVersion": { "variants": { "absent": { "class": "RuntimeStateVersionAbsent", "fields": {} }, "unmanaged": { "class": "RuntimeStateVersionUnmanaged", "fields": { "digest": "String" } }, "managed": { "class": "RuntimeStateVersionManaged", "fields": { "revision": "WireU64" } }, "retired": { "class": "RuntimeStateVersionRetired", "fields": { "revision": "WireU64", "operation_id": "String" } } }, "unit": false }, "RuntimeStateExpectedVersion": { "variants": { "absent": { "class": "RuntimeStateExpectedVersionAbsent", "fields": {} }, "unmanaged": { "class": "RuntimeStateExpectedVersionUnmanaged", "fields": { "digest": "String" } }, "managed": { "class": "RuntimeStateExpectedVersionManaged", "fields": { "revision": "WireU64" } } }, "unit": false }, "RuntimeScopeLifecycle": { "variants": { "active": { "class": "RuntimeScopeLifecycleActive", "fields": {} }, "retiring": { "class": "RuntimeScopeLifecycleRetiring", "fields": {} }, "tombstoned": { "class": "RuntimeScopeLifecycleTombstoned", "fields": {} } }, "unit": true }, "RuntimeCheckpointCasStatus": { "variants": { "applied": { "class": "RuntimeCheckpointCasStatusApplied", "fields": {} }, "already_current": { "class": "RuntimeCheckpointCasStatusAlreadyCurrent", "fields": {} }, "revision_conflict": { "class": "RuntimeCheckpointCasStatusRevisionConflict", "fields": {} }, "generation_retired": { "class": "RuntimeCheckpointCasStatusGenerationRetired", "fields": {} }, "scope_fenced": { "class": "RuntimeCheckpointCasStatusScopeFenced", "fields": {} } }, "unit": true }, "RuntimeGenerationRetireStatus": { "variants": { "retired": { "class": "RuntimeGenerationRetireStatusRetired", "fields": {} }, "already_retired": { "class": "RuntimeGenerationRetireStatusAlreadyRetired", "fields": {} }, "conflict": { "class": "RuntimeGenerationRetireStatusConflict", "fields": {} }, "pending_projection": { "class": "RuntimeGenerationRetireStatusPendingProjection", "fields": {} }, "scope_fenced": { "class": "RuntimeGenerationRetireStatusScopeFenced", "fields": {} } }, "unit": true }, "ScopeRetirementItemStatus": { "variants": { "pending": { "class": "ScopeRetirementItemStatusPending", "fields": {} }, "dropped_by_delete": { "class": "ScopeRetirementItemStatusDroppedByDelete", "fields": {} } }, "unit": true }, "ScopeRetirementAdvance": { "variants": { "conversation_deleted": { "class": "ScopeRetirementAdvanceConversationDeleted", "fields": { "receipt": "ManagedConversationDeleteReceiptWire" } }, "generation_dropped": { "class": "ScopeRetirementAdvanceGenerationDropped", "fields": { "runtime_state_id": "String", "pending_operation_id": "Option<String>" } }, "complete": { "class": "ScopeRetirementAdvanceComplete", "fields": {} } }, "unit": false }, "ScopeRetirementStatus": { "variants": { "begun": { "class": "ScopeRetirementStatusBegun", "fields": {} }, "in_progress": { "class": "ScopeRetirementStatusInProgress", "fields": {} }, "completed": { "class": "ScopeRetirementStatusCompleted", "fields": {} }, "already_completed": { "class": "ScopeRetirementStatusAlreadyCompleted", "fields": {} }, "revision_conflict": { "class": "ScopeRetirementStatusRevisionConflict", "fields": {} }, "epoch_conflict": { "class": "ScopeRetirementStatusEpochConflict", "fields": {} }, "receipt_expired": { "class": "ScopeRetirementStatusReceiptExpired", "fields": {} }, "identity_conflict": { "class": "ScopeRetirementStatusIdentityConflict", "fields": {} } }, "unit": true }, "RuntimeInterruptionDisposition": { "variants": { "cancelled": { "class": "RuntimeInterruptionDispositionCancelled", "fields": {} }, "paused": { "class": "RuntimeInterruptionDispositionPaused", "fields": { "reason": "String" } } }, "unit": false }, "TaskGraphExecutionMode": { "variants": { "sequential": { "class": "TaskGraphExecutionModeSequential", "fields": {} }, "parallel": { "class": "TaskGraphExecutionModeParallel", "fields": {} } }, "unit": true }, "WireTaskStatus": { "variants": { "pending": { "class": "WireTaskStatusPending", "fields": {} }, "running": { "class": "WireTaskStatusRunning", "fields": {} }, "blocked": { "class": "WireTaskStatusBlocked", "fields": { "reason": "String" } }, "completed": { "class": "WireTaskStatusCompleted", "fields": {} }, "failed": { "class": "WireTaskStatusFailed", "fields": { "error": "String" } }, "skipped": { "class": "WireTaskStatusSkipped", "fields": {} }, "cancelled": { "class": "WireTaskStatusCancelled", "fields": {} }, "timed_out": { "class": "WireTaskStatusTimedOut", "fields": { "error": "String" } }, "retrying": { "class": "WireTaskStatusRetrying", "fields": { "attempt": "u32", "last_error": "String" } }, "paused": { "class": "WireTaskStatusPaused", "fields": { "reason": "String" } } }, "unit": false }, "PersistenceCallContext": { "fields": { "absolute_deadline_unix_ms": "WireI64" } }, "ConversationProjectionAuthority": { "fields": { "conversation_id": "String", "epoch": "WireU64", "revision": "WireU64", "lifecycle": "ConversationProjectionLifecycleWire", "delete_receipt_retention_floor_epoch": "WireU64" } }, "NewConversation": { "fields": { "conversation_id": "String", "user_id": "String", "agent_type": "Option<String>", "title": "Option<String>" } }, "StoredMessage": { "fields": { "id": "Option<WireI64>", "conversation_id": "String", "role": "String", "content": "Option<String>", "attachments_json": "Option<String>", "tool_calls_json": "Option<String>", "tool_result_json": "Option<String>", "created_at": "String" } }, "EnsureConversationProjectionRequest": { "fields": { "conversation": "NewConversationWire", "expected_tombstone_epoch": "Option<WireU64>" } }, "ConversationProjectionEpochReceipt": { "fields": { "authority": "ConversationProjectionAuthorityWire", "status": "ConversationProjectionEpochStatusWire" } }, "TranscriptProjectionItem": { "fields": { "ordinal": "WireU64", "digest": "String", "message": "StoredMessageWire" } }, "TranscriptProjectionBatch": { "fields": { "schema_version": "u16", "operation_id": "String", "payload_digest": "String", "conversation_id": "String", "conversation_epoch": "WireU64", "generation_id": "String", "items": "Vec<TranscriptProjectionItemWire>" } }, "TranscriptProjectionApplyReceipt": { "fields": { "operation_id": "String", "payload_digest": "String", "authority": "ConversationProjectionAuthorityWire", "status": "TranscriptProjectionApplyStatusWire" } }, "TranscriptProjectionSettlement": { "fields": { "status": "TranscriptProjectionSettlementStatusWire", "operation_id": "Option<String>", "conversation_id": "Option<String>", "generation_id": "Option<String>", "attempt": "u32", "error_class": "Option<TranscriptProjectionErrorClassWire>", "detail": "Option<String>" } }, "ManagedConversationDelete": { "fields": { "schema_version": "u16", "operation_id": "String", "payload_digest": "String", "conversation_id": "String", "expected_epoch": "WireU64" } }, "ManagedConversationDeleteReceipt": { "fields": { "operation_id": "String", "payload_digest": "String", "deleted_epoch": "WireU64", "retention_floor_epoch": "WireU64", "status": "ManagedConversationDeleteStatusWire" } }, "ManagedConversationImport": { "fields": { "schema_version": "u16", "operation_id": "String", "payload_digest": "String", "conversation_id": "String", "expected_epoch": "WireU64", "expected_revision": "WireU64", "generation_id": "Option<String>", "messages": "Vec<StoredMessageWire>" } }, "RuntimeScopeAuthority": { "fields": { "scope_id": "String", "conversation_epoch": "Option<WireU64>", "revision": "WireU64", "lifecycle": "RuntimeScopeLifecycleWire" } }, "ManagedRuntimeStateSnapshot": { "fields": { "scope": "RuntimeScopeAuthorityWire", "runtime_state_id": "String", "version": "RuntimeStateVersionWire", "checkpoint": "Option<AgentCheckpointWire>" } }, "RuntimeCheckpointCasRequest": { "fields": { "scope_id": "String", "runtime_state_id": "String", "conversation_epoch": "Option<WireU64>", "expected_scope_revision": "WireU64", "expected_state_version": "RuntimeStateExpectedVersionWire", "checkpoint": "AgentCheckpointWire", "managed_import": "Option<ManagedImportEpochTransitionWire>" } }, "ManagedImportEpochTransition": { "fields": { "request": "ManagedConversationImportWire", "receipt": "TranscriptProjectionApplyReceiptWire" } }, "RuntimeCheckpointCasReceipt": { "fields": { "scope": "RuntimeScopeAuthorityWire", "runtime_state_id": "String", "version": "RuntimeStateVersionWire", "status": "RuntimeCheckpointCasStatusWire" } }, "RuntimeTranscriptAckRequest": { "fields": { "scope_id": "String", "runtime_state_id": "String", "conversation_epoch": "WireU64", "expected_scope_revision": "WireU64", "expected_state_version": "RuntimeStateExpectedVersionWire", "projection_receipt": "TranscriptProjectionApplyReceiptWire", "checkpoint": "AgentCheckpointWire" } }, "RuntimeGenerationRetireRequest": { "fields": { "schema_version": "u16", "operation_id": "String", "payload_digest": "String", "scope_id": "String", "runtime_state_id": "String", "expected_scope_revision": "WireU64", "expected_state_version": "RuntimeStateExpectedVersionWire" } }, "RuntimeGenerationRetireReceipt": { "fields": { "scope": "RuntimeScopeAuthorityWire", "runtime_state_id": "String", "version": "RuntimeStateVersionWire", "status": "RuntimeGenerationRetireStatusWire" } }, "ScopeRetirementItem": { "fields": { "runtime_state_id": "String", "state_version": "RuntimeStateVersionWire", "pending_operation_id": "Option<String>", "status": "ScopeRetirementItemStatusWire" } }, "ScopeRetirementManifest": { "fields": { "delete_operation_id": "String", "payload_digest": "String", "expected_conversation_epoch": "WireU64", "items": "Vec<ScopeRetirementItemWire>", "conversation_delete_receipt": "Option<ManagedConversationDeleteReceiptWire>" } }, "ScopeRetirementRequest": { "fields": { "schema_version": "u16", "delete_operation_id": "String", "payload_digest": "String", "scope_id": "String", "expected_scope_revision": "WireU64", "expected_conversation_epoch": "WireU64" } }, "ScopeRetirementReceipt": { "fields": { "scope": "RuntimeScopeAuthorityWire", "manifest": "ScopeRetirementManifestWire", "dropped_operation_ids": "Vec<String>", "retention_floor_epoch": "WireU64", "status": "ScopeRetirementStatusWire" } }, "AgentCheckpoint": { "fields": { "conversation_id": "String", "messages_json": "String", "current_plan": "Option<String>", "active_skills": "Vec<String>", "blocked_reason": "Option<String>", "working_dir": "Option<WirePath>", "timestamp": "WireTimestamp" } }, "TaskSpec": { "fields": { "id": "String", "title": "String", "description": "String", "depends_on": "Vec<String>", "max_retries": "u32", "extension": "WireValue" } }, "TaskClaim": { "fields": { "claim_id": "String", "revision": "WireU64", "attempt": "u32", "spec_hash": "String" } }, "TaskExecution": { "fields": { "task_id": "String", "status": "WireTaskStatus", "retry_count": "u32", "failure_fingerprint": "Option<String>", "claim": "Option<TaskClaimWire>", "result": "Option<WireValue>", "interruption_requested": "Option<RuntimeInterruptionDispositionWire>" } }, "Task": { "fields": { "spec": "TaskSpecWire", "execution": "TaskExecutionWire" } }, "RuntimePlanSnapshot": { "fields": { "revision": "WireU64", "tasks": "Vec<TaskWire>" } }, "TaskGraphContext": { "fields": { "goal": "String", "assumptions": "Vec<String>", "risks": "Vec<String>", "execution_mode": "TaskGraphExecutionModeWire", "metadata": "WireValue" } }, "RevisionedTaskGraph": { "fields": { "snapshot": "RuntimePlanSnapshotWire", "context": "TaskGraphContextWire" } }, "TaskPatchEffects": { "fields": { "inserted_task_ids": "Vec<String>", "updated_task_ids": "Vec<String>", "skipped_task_ids": "Vec<String>", "reset_task_ids": "Vec<String>", "progressed_task_ids": "Vec<String>", "reordered": "bool" } }, "TaskGraphCommit": { "fields": { "expected_revision": "Option<WireU64>", "expected_executions": "Option<BTreeMap<String, TaskExecutionWire>>", "next": "RevisionedTaskGraphWire", "reason": "String", "effects": "TaskPatchEffectsWire" } }, "TaskExecutionCommit": { "fields": { "expected_revision": "WireU64", "expected_executions": "BTreeMap<String, TaskExecutionWire>", "next": "RevisionedTaskGraphWire" } } };
export const MANAGED_OPERATIONS = { "conversation_ensure_projection_epoch_with_context": [{ "request": "EnsureConversationProjectionRequestWire", "context": "PersistenceCallContextWire" }, "receipt", "ConversationProjectionEpochReceiptWire"], "conversation_get_projection_authority_with_context": [{ "conversation_id": "String", "context": "PersistenceCallContextWire" }, "authority", "Option<ConversationProjectionAuthorityWire>"], "conversation_apply_transcript_projection_with_context": [{ "batch": "TranscriptProjectionBatchWire", "context": "PersistenceCallContextWire" }, "receipt", "TranscriptProjectionApplyReceiptWire"], "conversation_delete_managed_with_context": [{ "request": "ManagedConversationDeleteWire", "context": "PersistenceCallContextWire" }, "receipt", "ManagedConversationDeleteReceiptWire"], "runtime_load_managed_state_with_context": [{ "scope_id": "String", "runtime_state_id": "String", "context": "PersistenceCallContextWire" }, "snapshot", "Option<ManagedRuntimeStateSnapshotWire>"], "runtime_load_scope_authority_with_context": [{ "scope_id": "String", "context": "PersistenceCallContextWire" }, "authority", "Option<RuntimeScopeAuthorityWire>"], "runtime_compare_and_save_checkpoint_with_context": [{ "request": "RuntimeCheckpointCasRequestWire", "context": "PersistenceCallContextWire" }, "receipt", "RuntimeCheckpointCasReceiptWire"], "runtime_acknowledge_transcript_projection_with_context": [{ "request": "RuntimeTranscriptAckRequestWire", "context": "PersistenceCallContextWire" }, "receipt", "RuntimeCheckpointCasReceiptWire"], "runtime_retire_generation_with_context": [{ "request": "RuntimeGenerationRetireRequestWire", "context": "PersistenceCallContextWire" }, "receipt", "RuntimeGenerationRetireReceiptWire"], "runtime_begin_scope_retirement_with_context": [{ "request": "ScopeRetirementRequestWire", "context": "PersistenceCallContextWire" }, "receipt", "ScopeRetirementReceiptWire"], "runtime_continue_scope_retirement_with_context": [{ "scope_id": "String", "delete_operation_id": "String", "expected_scope_revision": "WireU64", "advance": "ScopeRetirementAdvanceWire", "context": "PersistenceCallContextWire" }, "receipt", "ScopeRetirementReceiptWire"] };
export function validateManaged(type, value) {
    if (type.startsWith("BTreeMap<String, ")) {
        if (typeof value !== "object" || value === null || Array.isArray(value))
            throw new TypeError("managed map malformed");
        Object.values(value).forEach(item => validateManaged(type.slice(17, -1), item));
        return;
    }
    if (type === "WireValue") {
        if (typeof value !== "object" || value === null || typeof value.kind !== "string")
            throw new TypeError("opaque value must retain WireValue wrapper");
        return;
    }
    if (type === "bool") {
        if (typeof value !== "boolean")
            throw new TypeError("managed bool must be boolean");
        return;
    }
    if (type === "WireTaskStatus") {
        if (typeof value !== "object" || value === null)
            throw new TypeError("task status malformed");
        const object = value;
        const variant = schema.WireTaskStatus?.variants?.[String(object.status)];
        if (!variant)
            throw new TypeError("task status unknown");
        const { status, ...payload } = object;
        validateManagedFields(variant.fields, payload);
        return;
    }
    if (type.startsWith("Option<")) {
        if (value !== null && value !== undefined)
            validateManaged(type.slice(7, -1), value);
        return;
    }
    if (type.startsWith("Vec<")) {
        if (!Array.isArray(value))
            throw new TypeError("managed vector must be array");
        value.forEach(item => validateManaged(type.slice(4, -1), item));
        return;
    }
    if (type === "String") {
        if (typeof value !== "string")
            throw new TypeError("managed text must be string");
        return;
    }
    if (type === "WireU64" || type === "WireI64") {
        if (typeof value !== "string")
            throw new TypeError("managed integer must be decimal text");
        const integer = BigInt(value);
        if (integer.toString() !== value || integer < (type === "WireI64" ? -(1n << 63n) : 0n) || integer > (type === "WireI64" ? (1n << 63n) - 1n : (1n << 64n) - 1n))
            throw new TypeError("managed integer out of range");
        return;
    }
    if (type === "u16" || type === "u32") {
        if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value >= 2 ** Number(type.slice(1)))
            throw new TypeError("managed integer out of range");
        return;
    }
    if (type === "WirePath" || type === "WireTimestamp") {
        if (typeof value !== "object" || value === null)
            throw new TypeError("managed path/time must be wire object");
        if (type === "WirePath")
            validateManagedPath(value);
        if (type === "WireTimestamp") {
            const v = value;
            validateManaged("WireI64", v.unix_seconds);
            if (typeof v.nanos !== "number" || !Number.isInteger(v.nanos) || v.nanos < 0 || v.nanos >= 1e9)
                throw new TypeError("nanos out of range");
        }
        return;
    }
    const typeName = type.replace(/Wire$/u, "");
    const definition = Object.hasOwn(schema, typeName) ? schema[typeName] : undefined;
    if (!definition)
        throw new TypeError("unknown managed type");
    if (definition.variants) {
        if (typeof value === "string") {
            if (!definition.variants[value] || Object.keys(definition.variants[value].fields).length)
                throw new TypeError("unknown managed variant");
            return;
        }
        if (typeof value !== "object" || value === null || Array.isArray(value) || Object.keys(value).length !== 1)
            throw new TypeError("managed variant malformed");
        const [tag, payload] = Object.entries(value)[0];
        const variant = definition.variants[tag];
        if (!variant)
            throw new TypeError("unknown managed variant");
        validateManagedFields(variant.fields, payload);
        return;
    }
    validateManagedFields(definition.fields ?? {}, value);
}
export function validateManagedFields(fields, value) {
    if (typeof value !== "object" || value === null || Array.isArray(value))
        throw new TypeError("managed record must be object");
    const object = value;
    if (Object.keys(object).some(key => !Object.hasOwn(fields, key)) || Object.keys(fields).some(key => !(key in object) && !fields[key]?.startsWith("Option<")))
        throw new TypeError("managed record fields mismatch");
    for (const [key, type] of Object.entries(fields))
        validateManaged(type, object[key]);
}
function validateManagedPath(value) {
    const encoding = value.encoding;
    if (encoding === "utf8") {
        const path = value.path;
        if (typeof path !== "string" || path.includes("\0") || !(path.startsWith("/") || path.startsWith("\\\\") || /^[A-Za-z]:[\\/]/u.test(path)) || Object.keys(value).some(key => !["encoding", "path"].includes(key)))
            throw new TypeError("managed UTF8 path malformed");
        return;
    }
    if (encoding !== "unix" && encoding !== "windows")
        throw new TypeError("unknown managed path encoding");
    const field = encoding === "unix" ? "bytes_base64" : "utf16_base64";
    const encoded = value[field];
    if (typeof encoded !== "string" || Object.keys(value).some(key => !["encoding", field, "display"].includes(key)) || value.display != null && typeof value.display !== "string")
        throw new TypeError("managed path fields malformed");
    const raw = Buffer.from(encoded, "base64");
    if (raw.toString("base64").replace(/=+$/u, "") !== encoded)
        throw new TypeError("managed path base64 malformed");
    if (encoding === "unix") {
        if (raw.at(0) !== 47 || raw.includes(0))
            throw new TypeError("managed unix path is not absolute or contains NUL");
        return;
    }
    if (raw.length % 2)
        throw new TypeError("managed Windows path byte count malformed");
    const units = Array.from({ length: raw.length / 2 }, (_value, index) => raw.readUInt16LE(index * 2));
    const first = units[0], second = units[1], third = units[2];
    if (units.includes(0) || !(first !== undefined && ((first >= 65 && first <= 90 || first >= 97 && first <= 122) && second === 58 && (third === 47 || third === 92) || first === 92 && second === 92)))
        throw new TypeError("managed Windows path is not absolute or contains NUL");
}
export function decodeSettlementEvent(value) {
    if (typeof value !== "object" || value === null || Array.isArray(value))
        throw new TypeError("settlement event requires wire map");
    const wire = value;
    if (wire.kind !== "map" || !Array.isArray(wire.value))
        throw new TypeError("settlement event requires wire map");
    const fields = {};
    for (const entry of wire.value) {
        if (typeof entry !== "object" || entry === null || typeof entry.key !== "object" || entry.key === null || entry.key.kind !== "string" || typeof entry.key.value !== "string" || !("value" in entry))
            throw new TypeError("settlement field malformed");
        const name = entry.key.value;
        if (!Object.hasOwn(schema.TranscriptProjectionSettlement?.fields ?? {}, name))
            throw new TypeError("unknown settlement field");
        if (Object.hasOwn(fields, name))
            throw new TypeError("duplicate settlement field");
        const raw = entry.value;
        if (name === "attempt") {
            if (raw?.kind !== "u64")
                throw new TypeError("settlement attempt requires wire u64");
            validateManaged("WireU64", raw.value);
            const integer = BigInt(raw.value);
            if (integer > 4294967295n)
                throw new TypeError("settlement attempt exceeds u32");
            fields[name] = Number(integer);
        }
        else if (raw?.kind === "null")
            fields[name] = null;
        else if (raw?.kind === "string" && typeof raw.value === "string")
            Object.defineProperty(fields, name, { value: raw.value, enumerable: true, writable: true });
        else
            throw new TypeError("settlement scalar is malformed");
    }
    const definitions = schema.TranscriptProjectionSettlement?.fields;
    if (!definitions || Object.keys(fields).length !== Object.keys(definitions).length)
        throw new TypeError("settlement fields missing");
    validateManaged("TranscriptProjectionSettlement", fields);
    return fields;
}
//# sourceMappingURL=managed.js.map