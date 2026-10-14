import type { WirePath, WireValue } from "./types.js";
export type ConversationProjectionCapability = "unsupported" | "atomic_v1";
export type PersistenceCallCapability = "unsupported" | "absolute_deadline_v1";
export type ConversationProjectionLifecycle = "live" | "deleted";
export type ConversationProjectionEpochStatus = "created" | "adopted_legacy" | "existing" | "recreated" | "tombstoned" | "epoch_conflict";
export type TranscriptProjectionConflictKind = "operation_identity" | "ordinal_digest" | "revision" | "managed_import";
export type TranscriptProjectionApplyStatus = "applied" | "already_applied" | {
    readonly fenced: {
        readonly current_epoch: string;
        readonly lifecycle: ConversationProjectionLifecycle;
    };
} | {
    readonly conflict: {
        readonly current_epoch: string;
        readonly current_revision: string;
        readonly kind: TranscriptProjectionConflictKind;
    };
};
export type TranscriptProjectionSettlementStatus = "settled" | "deferred" | "blocked" | "conflict";
export type TranscriptProjectionErrorClass = "outcome_unknown" | "transient_no_commit" | "revision_conflict" | "deadline_exceeded" | "unsupported" | "invalid_configuration" | "corrupt_state" | "semantic_conflict";
export type ManagedConversationDeleteStatus = "deleted" | "already_deleted" | "epoch_conflict" | "receipt_expired" | "identity_conflict";
export type RuntimeStateCapability = "unsupported" | "revisioned_v1";
export type RuntimeStateVersion = "absent" | {
    readonly unmanaged: {
        readonly digest: string;
    };
} | {
    readonly managed: {
        readonly revision: string;
    };
} | {
    readonly retired: {
        readonly revision: string;
        readonly operation_id: string;
    };
};
export type RuntimeStateExpectedVersion = "absent" | {
    readonly unmanaged: {
        readonly digest: string;
    };
} | {
    readonly managed: {
        readonly revision: string;
    };
};
export type RuntimeScopeLifecycle = "active" | "retiring" | "tombstoned";
export type RuntimeCheckpointCasStatus = "applied" | "already_current" | "revision_conflict" | "generation_retired" | "scope_fenced";
export type RuntimeGenerationRetireStatus = "retired" | "already_retired" | "conflict" | "pending_projection" | "scope_fenced";
export type ScopeRetirementItemStatus = "pending" | "dropped_by_delete";
export type ScopeRetirementAdvance = {
    readonly conversation_deleted: {
        readonly receipt: ManagedConversationDeleteReceipt;
    };
} | {
    readonly generation_dropped: {
        readonly runtime_state_id: string;
        readonly pending_operation_id: string | null;
    };
} | "complete";
export type ScopeRetirementStatus = "begun" | "in_progress" | "completed" | "already_completed" | "revision_conflict" | "epoch_conflict" | "receipt_expired" | "identity_conflict";
export type RuntimeInterruptionDisposition = "cancelled" | {
    readonly paused: {
        readonly reason: string;
    };
};
export type TaskGraphExecutionMode = "sequential" | "parallel";
export type WireTaskStatus = {
    readonly status: "pending" | "running" | "completed" | "skipped" | "cancelled";
} | {
    readonly status: "blocked" | "paused";
    readonly reason: string;
} | {
    readonly status: "failed" | "timed_out";
    readonly error: string;
} | {
    readonly status: "retrying";
    readonly attempt: number;
    readonly last_error: string;
};
export interface PersistenceCallContext {
    readonly absolute_deadline_unix_ms: string;
}
export interface ConversationProjectionAuthority {
    readonly conversation_id: string;
    readonly epoch: string;
    readonly revision: string;
    readonly lifecycle: ConversationProjectionLifecycle;
    readonly delete_receipt_retention_floor_epoch: string;
}
export interface NewConversation {
    readonly conversation_id: string;
    readonly user_id: string;
    readonly agent_type: string | null;
    readonly title: string | null;
}
export interface StoredMessage {
    readonly id: string | null;
    readonly conversation_id: string;
    readonly role: string;
    readonly content: string | null;
    readonly attachments_json: string | null;
    readonly tool_calls_json: string | null;
    readonly tool_result_json: string | null;
    readonly created_at: string;
}
export interface EnsureConversationProjectionRequest {
    readonly conversation: NewConversation;
    readonly expected_tombstone_epoch: string | null;
}
export interface ConversationProjectionEpochReceipt {
    readonly authority: ConversationProjectionAuthority;
    readonly status: ConversationProjectionEpochStatus;
}
export interface TranscriptProjectionItem {
    readonly ordinal: string;
    readonly digest: string;
    readonly message: StoredMessage;
}
export interface TranscriptProjectionBatch {
    readonly schema_version: number;
    readonly operation_id: string;
    readonly payload_digest: string;
    readonly conversation_id: string;
    readonly conversation_epoch: string;
    readonly generation_id: string;
    readonly items: readonly (TranscriptProjectionItem)[];
}
export interface TranscriptProjectionApplyReceipt {
    readonly operation_id: string;
    readonly payload_digest: string;
    readonly authority: ConversationProjectionAuthority;
    readonly status: TranscriptProjectionApplyStatus;
}
export interface TranscriptProjectionSettlement {
    readonly status: TranscriptProjectionSettlementStatus;
    readonly operation_id: string | null;
    readonly conversation_id: string | null;
    readonly generation_id: string | null;
    readonly attempt: number;
    readonly error_class: TranscriptProjectionErrorClass | null;
    readonly detail: string | null;
}
export interface ManagedConversationDelete {
    readonly schema_version: number;
    readonly operation_id: string;
    readonly payload_digest: string;
    readonly conversation_id: string;
    readonly expected_epoch: string;
}
export interface ManagedConversationDeleteReceipt {
    readonly operation_id: string;
    readonly payload_digest: string;
    readonly deleted_epoch: string;
    readonly retention_floor_epoch: string;
    readonly status: ManagedConversationDeleteStatus;
}
export interface ManagedConversationImport {
    readonly schema_version: number;
    readonly operation_id: string;
    readonly payload_digest: string;
    readonly conversation_id: string;
    readonly expected_epoch: string;
    readonly expected_revision: string;
    readonly generation_id: string | null;
    readonly messages: readonly (StoredMessage)[];
}
export interface RuntimeScopeAuthority {
    readonly scope_id: string;
    readonly conversation_epoch: string | null;
    readonly revision: string;
    readonly lifecycle: RuntimeScopeLifecycle;
}
export interface ManagedRuntimeStateSnapshot {
    readonly scope: RuntimeScopeAuthority;
    readonly runtime_state_id: string;
    readonly version: RuntimeStateVersion;
    readonly checkpoint: AgentCheckpoint | null;
}
export interface RuntimeCheckpointCasRequest {
    readonly scope_id: string;
    readonly runtime_state_id: string;
    readonly conversation_epoch: string | null;
    readonly expected_scope_revision: string;
    readonly expected_state_version: RuntimeStateExpectedVersion;
    readonly checkpoint: AgentCheckpoint;
    readonly managed_import: ManagedImportEpochTransition | null;
}
export interface ManagedImportEpochTransition {
    readonly request: ManagedConversationImport;
    readonly receipt: TranscriptProjectionApplyReceipt;
}
export interface RuntimeCheckpointCasReceipt {
    readonly scope: RuntimeScopeAuthority;
    readonly runtime_state_id: string;
    readonly version: RuntimeStateVersion;
    readonly status: RuntimeCheckpointCasStatus;
}
export interface RuntimeTranscriptAckRequest {
    readonly scope_id: string;
    readonly runtime_state_id: string;
    readonly conversation_epoch: string;
    readonly expected_scope_revision: string;
    readonly expected_state_version: RuntimeStateExpectedVersion;
    readonly projection_receipt: TranscriptProjectionApplyReceipt;
    readonly checkpoint: AgentCheckpoint;
}
export interface RuntimeGenerationRetireRequest {
    readonly schema_version: number;
    readonly operation_id: string;
    readonly payload_digest: string;
    readonly scope_id: string;
    readonly runtime_state_id: string;
    readonly expected_scope_revision: string;
    readonly expected_state_version: RuntimeStateExpectedVersion;
}
export interface RuntimeGenerationRetireReceipt {
    readonly scope: RuntimeScopeAuthority;
    readonly runtime_state_id: string;
    readonly version: RuntimeStateVersion;
    readonly status: RuntimeGenerationRetireStatus;
}
export interface ScopeRetirementItem {
    readonly runtime_state_id: string;
    readonly state_version: RuntimeStateVersion;
    readonly pending_operation_id: string | null;
    readonly status: ScopeRetirementItemStatus;
}
export interface ScopeRetirementManifest {
    readonly delete_operation_id: string;
    readonly payload_digest: string;
    readonly expected_conversation_epoch: string;
    readonly items: readonly (ScopeRetirementItem)[];
    readonly conversation_delete_receipt: ManagedConversationDeleteReceipt | null;
}
export interface ScopeRetirementRequest {
    readonly schema_version: number;
    readonly delete_operation_id: string;
    readonly payload_digest: string;
    readonly scope_id: string;
    readonly expected_scope_revision: string;
    readonly expected_conversation_epoch: string;
}
export interface ScopeRetirementReceipt {
    readonly scope: RuntimeScopeAuthority;
    readonly manifest: ScopeRetirementManifest;
    readonly dropped_operation_ids: readonly (string)[];
    readonly retention_floor_epoch: string;
    readonly status: ScopeRetirementStatus;
}
export interface AgentCheckpoint {
    readonly conversation_id: string;
    readonly messages_json: string;
    readonly current_plan: string | null;
    readonly active_skills: readonly (string)[];
    readonly blocked_reason: string | null;
    readonly working_dir: WirePath | null;
    readonly timestamp: {
        readonly unix_seconds: string;
        readonly nanos: number;
        readonly rfc3339?: string | null;
    };
}
export interface TaskSpec {
    readonly id: string;
    readonly title: string;
    readonly description: string;
    readonly depends_on: readonly (string)[];
    readonly max_retries: number;
    readonly extension: WireValue;
}
export interface TaskClaim {
    readonly claim_id: string;
    readonly revision: string;
    readonly attempt: number;
    readonly spec_hash: string;
}
export interface TaskExecution {
    readonly task_id: string;
    readonly status: WireTaskStatus;
    readonly retry_count: number;
    readonly failure_fingerprint: string | null;
    readonly claim: TaskClaim | null;
    readonly result?: WireValue | null;
    readonly interruption_requested?: RuntimeInterruptionDisposition | null;
}
export interface Task {
    readonly spec: TaskSpec;
    readonly execution: TaskExecution;
}
export interface RuntimePlanSnapshot {
    readonly revision: string;
    readonly tasks: readonly (Task)[];
}
export interface TaskGraphContext {
    readonly goal: string;
    readonly assumptions: readonly (string)[];
    readonly risks: readonly (string)[];
    readonly execution_mode: TaskGraphExecutionMode;
    readonly metadata: WireValue;
}
export interface RevisionedTaskGraph {
    readonly snapshot: RuntimePlanSnapshot;
    readonly context: TaskGraphContext;
}
export interface TaskPatchEffects {
    readonly inserted_task_ids: readonly (string)[];
    readonly updated_task_ids: readonly (string)[];
    readonly skipped_task_ids: readonly (string)[];
    readonly reset_task_ids: readonly (string)[];
    readonly progressed_task_ids: readonly (string)[];
    readonly reordered: boolean;
}
export interface TaskGraphCommit {
    readonly expected_revision: string | null;
    readonly expected_executions: Readonly<Record<string, TaskExecution>> | null;
    readonly next: RevisionedTaskGraph;
    readonly reason: string;
    readonly effects: TaskPatchEffects;
}
export interface TaskExecutionCommit {
    readonly expected_revision: string;
    readonly expected_executions: Readonly<Record<string, TaskExecution>>;
    readonly next: RevisionedTaskGraph;
}
export type ManagedComponentCall = {
    readonly operation: "conversation_ensure_projection_epoch_with_context";
    readonly input: {
        readonly request: EnsureConversationProjectionRequest;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "conversation_get_projection_authority_with_context";
    readonly input: {
        readonly conversation_id: string;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "conversation_apply_transcript_projection_with_context";
    readonly input: {
        readonly batch: TranscriptProjectionBatch;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "conversation_delete_managed_with_context";
    readonly input: {
        readonly request: ManagedConversationDelete;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "runtime_load_managed_state_with_context";
    readonly input: {
        readonly scope_id: string;
        readonly runtime_state_id: string;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "runtime_load_scope_authority_with_context";
    readonly input: {
        readonly scope_id: string;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "runtime_compare_and_save_checkpoint_with_context";
    readonly input: {
        readonly request: RuntimeCheckpointCasRequest;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "runtime_acknowledge_transcript_projection_with_context";
    readonly input: {
        readonly request: RuntimeTranscriptAckRequest;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "runtime_retire_generation_with_context";
    readonly input: {
        readonly request: RuntimeGenerationRetireRequest;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "runtime_begin_scope_retirement_with_context";
    readonly input: {
        readonly request: ScopeRetirementRequest;
        readonly context: PersistenceCallContext;
    };
} | {
    readonly operation: "runtime_continue_scope_retirement_with_context";
    readonly input: {
        readonly scope_id: string;
        readonly delete_operation_id: string;
        readonly expected_scope_revision: string;
        readonly advance: ScopeRetirementAdvance;
        readonly context: PersistenceCallContext;
    };
};
export type ManagedComponentResult = {
    readonly operation: "conversation_ensure_projection_epoch_with_context";
    readonly value: {
        readonly receipt: ConversationProjectionEpochReceipt;
    };
} | {
    readonly operation: "conversation_get_projection_authority_with_context";
    readonly value: {
        readonly authority: ConversationProjectionAuthority | null;
    };
} | {
    readonly operation: "conversation_apply_transcript_projection_with_context";
    readonly value: {
        readonly receipt: TranscriptProjectionApplyReceipt;
    };
} | {
    readonly operation: "conversation_delete_managed_with_context";
    readonly value: {
        readonly receipt: ManagedConversationDeleteReceipt;
    };
} | {
    readonly operation: "runtime_load_managed_state_with_context";
    readonly value: {
        readonly snapshot: ManagedRuntimeStateSnapshot | null;
    };
} | {
    readonly operation: "runtime_load_scope_authority_with_context";
    readonly value: {
        readonly authority: RuntimeScopeAuthority | null;
    };
} | {
    readonly operation: "runtime_compare_and_save_checkpoint_with_context";
    readonly value: {
        readonly receipt: RuntimeCheckpointCasReceipt;
    };
} | {
    readonly operation: "runtime_acknowledge_transcript_projection_with_context";
    readonly value: {
        readonly receipt: RuntimeCheckpointCasReceipt;
    };
} | {
    readonly operation: "runtime_retire_generation_with_context";
    readonly value: {
        readonly receipt: RuntimeGenerationRetireReceipt;
    };
} | {
    readonly operation: "runtime_begin_scope_retirement_with_context";
    readonly value: {
        readonly receipt: ScopeRetirementReceipt;
    };
} | {
    readonly operation: "runtime_continue_scope_retirement_with_context";
    readonly value: {
        readonly receipt: ScopeRetirementReceipt;
    };
};
export declare const MANAGED_OPERATIONS: Readonly<Record<string, readonly [Record<string, string>, string, string]>>;
export declare function validateManaged(type: string, value: unknown): void;
export declare function validateManagedFields(fields: Record<string, string>, value: unknown): void;
export declare function decodeSettlementEvent(value: unknown): TranscriptProjectionSettlement;
