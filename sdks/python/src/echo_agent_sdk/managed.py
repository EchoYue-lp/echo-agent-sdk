from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass, fields
from enum import Enum
from typing import Any, ClassVar

from .errors import EchoAgentError


class ManagedValue:
    """Typed persistence facts; all CAS and lifecycle decisions belong to the backing store."""

    def to_wire(self) -> dict[str, Any]:
        values = {
            field.name: encode(getattr(self, field.name)) for field in fields(self)
        }
        definitions = SCHEMA.get(type(self).__name__, {}).get("fields")
        if definitions:
            for field, kind in definitions.items():
                decode(kind, values[field])
        if isinstance(self, TaskExecution):
            for field in ("result", "interruption_requested"):
                if values[field] is None:
                    values.pop(field)
        return values

    @classmethod
    def from_wire(cls, value: Mapping[str, Any]):
        return decode(cls.__name__, value)


class ConversationProjectionCapability(str, Enum):
    UNSUPPORTED = "unsupported"
    ATOMIC_V1 = "atomic_v1"


class PersistenceCallCapability(str, Enum):
    UNSUPPORTED = "unsupported"
    ABSOLUTE_DEADLINE_V1 = "absolute_deadline_v1"


class ConversationProjectionLifecycle(str, Enum):
    LIVE = "live"
    DELETED = "deleted"


class ConversationProjectionEpochStatus(str, Enum):
    CREATED = "created"
    ADOPTED_LEGACY = "adopted_legacy"
    EXISTING = "existing"
    RECREATED = "recreated"
    TOMBSTONED = "tombstoned"
    EPOCH_CONFLICT = "epoch_conflict"


class TranscriptProjectionConflictKind(str, Enum):
    OPERATION_IDENTITY = "operation_identity"
    ORDINAL_DIGEST = "ordinal_digest"
    REVISION = "revision"
    MANAGED_IMPORT = "managed_import"


class TranscriptProjectionApplyStatus(ManagedValue):
    _enum_tag: ClassVar[str]

    def to_wire(self):
        values = super().to_wire()
        return {self._enum_tag: values} if values else self._enum_tag


@dataclass(frozen=True, slots=True)
class TranscriptProjectionApplyStatusApplied(TranscriptProjectionApplyStatus):
    _enum_tag: ClassVar[str] = "applied"


@dataclass(frozen=True, slots=True)
class TranscriptProjectionApplyStatusAlreadyApplied(TranscriptProjectionApplyStatus):
    _enum_tag: ClassVar[str] = "already_applied"


@dataclass(frozen=True, slots=True)
class TranscriptProjectionApplyStatusFenced(TranscriptProjectionApplyStatus):
    _enum_tag: ClassVar[str] = "fenced"
    current_epoch: str
    lifecycle: ConversationProjectionLifecycle


@dataclass(frozen=True, slots=True)
class TranscriptProjectionApplyStatusConflict(TranscriptProjectionApplyStatus):
    _enum_tag: ClassVar[str] = "conflict"
    current_epoch: str
    current_revision: str
    kind: TranscriptProjectionConflictKind


class TranscriptProjectionSettlementStatus(str, Enum):
    SETTLED = "settled"
    DEFERRED = "deferred"
    BLOCKED = "blocked"
    CONFLICT = "conflict"


class TranscriptProjectionErrorClass(str, Enum):
    OUTCOME_UNKNOWN = "outcome_unknown"
    TRANSIENT_NO_COMMIT = "transient_no_commit"
    REVISION_CONFLICT = "revision_conflict"
    DEADLINE_EXCEEDED = "deadline_exceeded"
    UNSUPPORTED = "unsupported"
    INVALID_CONFIGURATION = "invalid_configuration"
    CORRUPT_STATE = "corrupt_state"
    SEMANTIC_CONFLICT = "semantic_conflict"


class ManagedConversationDeleteStatus(str, Enum):
    DELETED = "deleted"
    ALREADY_DELETED = "already_deleted"
    EPOCH_CONFLICT = "epoch_conflict"
    RECEIPT_EXPIRED = "receipt_expired"
    IDENTITY_CONFLICT = "identity_conflict"


class RuntimeStateCapability(str, Enum):
    UNSUPPORTED = "unsupported"
    REVISIONED_V1 = "revisioned_v1"


class RuntimeStateVersion(ManagedValue):
    _enum_tag: ClassVar[str]

    def to_wire(self):
        values = super().to_wire()
        return {self._enum_tag: values} if values else self._enum_tag


@dataclass(frozen=True, slots=True)
class RuntimeStateVersionAbsent(RuntimeStateVersion):
    _enum_tag: ClassVar[str] = "absent"


@dataclass(frozen=True, slots=True)
class RuntimeStateVersionUnmanaged(RuntimeStateVersion):
    _enum_tag: ClassVar[str] = "unmanaged"
    digest: str


@dataclass(frozen=True, slots=True)
class RuntimeStateVersionManaged(RuntimeStateVersion):
    _enum_tag: ClassVar[str] = "managed"
    revision: str


@dataclass(frozen=True, slots=True)
class RuntimeStateVersionRetired(RuntimeStateVersion):
    _enum_tag: ClassVar[str] = "retired"
    revision: str
    operation_id: str


class RuntimeStateExpectedVersion(ManagedValue):
    _enum_tag: ClassVar[str]

    def to_wire(self):
        values = super().to_wire()
        return {self._enum_tag: values} if values else self._enum_tag


@dataclass(frozen=True, slots=True)
class RuntimeStateExpectedVersionAbsent(RuntimeStateExpectedVersion):
    _enum_tag: ClassVar[str] = "absent"


@dataclass(frozen=True, slots=True)
class RuntimeStateExpectedVersionUnmanaged(RuntimeStateExpectedVersion):
    _enum_tag: ClassVar[str] = "unmanaged"
    digest: str


@dataclass(frozen=True, slots=True)
class RuntimeStateExpectedVersionManaged(RuntimeStateExpectedVersion):
    _enum_tag: ClassVar[str] = "managed"
    revision: str


class RuntimeScopeLifecycle(str, Enum):
    ACTIVE = "active"
    RETIRING = "retiring"
    TOMBSTONED = "tombstoned"


class RuntimeCheckpointCasStatus(str, Enum):
    APPLIED = "applied"
    ALREADY_CURRENT = "already_current"
    REVISION_CONFLICT = "revision_conflict"
    GENERATION_RETIRED = "generation_retired"
    SCOPE_FENCED = "scope_fenced"


class RuntimeGenerationRetireStatus(str, Enum):
    RETIRED = "retired"
    ALREADY_RETIRED = "already_retired"
    CONFLICT = "conflict"
    PENDING_PROJECTION = "pending_projection"
    SCOPE_FENCED = "scope_fenced"


class ScopeRetirementItemStatus(str, Enum):
    PENDING = "pending"
    DROPPED_BY_DELETE = "dropped_by_delete"


class ScopeRetirementAdvance(ManagedValue):
    _enum_tag: ClassVar[str]

    def to_wire(self):
        values = super().to_wire()
        return {self._enum_tag: values} if values else self._enum_tag


@dataclass(frozen=True, slots=True)
class ScopeRetirementAdvanceConversationDeleted(ScopeRetirementAdvance):
    _enum_tag: ClassVar[str] = "conversation_deleted"
    receipt: ManagedConversationDeleteReceipt


@dataclass(frozen=True, slots=True)
class ScopeRetirementAdvanceGenerationDropped(ScopeRetirementAdvance):
    _enum_tag: ClassVar[str] = "generation_dropped"
    runtime_state_id: str
    pending_operation_id: str | None


@dataclass(frozen=True, slots=True)
class ScopeRetirementAdvanceComplete(ScopeRetirementAdvance):
    _enum_tag: ClassVar[str] = "complete"


class ScopeRetirementStatus(str, Enum):
    BEGUN = "begun"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    ALREADY_COMPLETED = "already_completed"
    REVISION_CONFLICT = "revision_conflict"
    EPOCH_CONFLICT = "epoch_conflict"
    RECEIPT_EXPIRED = "receipt_expired"
    IDENTITY_CONFLICT = "identity_conflict"


class RuntimeInterruptionDisposition(ManagedValue):
    _enum_tag: ClassVar[str]

    def to_wire(self):
        values = super().to_wire()
        return {self._enum_tag: values} if values else self._enum_tag


@dataclass(frozen=True, slots=True)
class RuntimeInterruptionDispositionCancelled(RuntimeInterruptionDisposition):
    _enum_tag: ClassVar[str] = "cancelled"


@dataclass(frozen=True, slots=True)
class RuntimeInterruptionDispositionPaused(RuntimeInterruptionDisposition):
    _enum_tag: ClassVar[str] = "paused"
    reason: str


class TaskGraphExecutionMode(str, Enum):
    SEQUENTIAL = "sequential"
    PARALLEL = "parallel"


class WireTaskStatus(ManagedValue):
    _enum_tag: ClassVar[str]

    def to_wire(self):
        return {"status": self._enum_tag, **super().to_wire()}


@dataclass(frozen=True, slots=True)
class WireTaskStatusPending(WireTaskStatus):
    _enum_tag: ClassVar[str] = "pending"


@dataclass(frozen=True, slots=True)
class WireTaskStatusRunning(WireTaskStatus):
    _enum_tag: ClassVar[str] = "running"


@dataclass(frozen=True, slots=True)
class WireTaskStatusBlocked(WireTaskStatus):
    _enum_tag: ClassVar[str] = "blocked"
    reason: str


@dataclass(frozen=True, slots=True)
class WireTaskStatusCompleted(WireTaskStatus):
    _enum_tag: ClassVar[str] = "completed"


@dataclass(frozen=True, slots=True)
class WireTaskStatusFailed(WireTaskStatus):
    _enum_tag: ClassVar[str] = "failed"
    error: str


@dataclass(frozen=True, slots=True)
class WireTaskStatusSkipped(WireTaskStatus):
    _enum_tag: ClassVar[str] = "skipped"


@dataclass(frozen=True, slots=True)
class WireTaskStatusCancelled(WireTaskStatus):
    _enum_tag: ClassVar[str] = "cancelled"


@dataclass(frozen=True, slots=True)
class WireTaskStatusTimedOut(WireTaskStatus):
    _enum_tag: ClassVar[str] = "timed_out"
    error: str


@dataclass(frozen=True, slots=True)
class WireTaskStatusRetrying(WireTaskStatus):
    _enum_tag: ClassVar[str] = "retrying"
    attempt: int
    last_error: str


@dataclass(frozen=True, slots=True)
class WireTaskStatusPaused(WireTaskStatus):
    _enum_tag: ClassVar[str] = "paused"
    reason: str


@dataclass(frozen=True, slots=True)
class PersistenceCallContext(ManagedValue):
    absolute_deadline_unix_ms: str


@dataclass(frozen=True, slots=True)
class ConversationProjectionAuthority(ManagedValue):
    conversation_id: str
    epoch: str
    revision: str
    lifecycle: ConversationProjectionLifecycle
    delete_receipt_retention_floor_epoch: str


@dataclass(frozen=True, slots=True)
class NewConversation(ManagedValue):
    conversation_id: str
    user_id: str
    agent_type: str | None
    title: str | None


@dataclass(frozen=True, slots=True)
class StoredMessage(ManagedValue):
    id: str | None
    conversation_id: str
    role: str
    content: str | None
    attachments_json: str | None
    tool_calls_json: str | None
    tool_result_json: str | None
    created_at: str


@dataclass(frozen=True, slots=True)
class EnsureConversationProjectionRequest(ManagedValue):
    conversation: NewConversation
    expected_tombstone_epoch: str | None


@dataclass(frozen=True, slots=True)
class ConversationProjectionEpochReceipt(ManagedValue):
    authority: ConversationProjectionAuthority
    status: ConversationProjectionEpochStatus


@dataclass(frozen=True, slots=True)
class TranscriptProjectionItem(ManagedValue):
    ordinal: str
    digest: str
    message: StoredMessage


@dataclass(frozen=True, slots=True)
class TranscriptProjectionBatch(ManagedValue):
    schema_version: int
    operation_id: str
    payload_digest: str
    conversation_id: str
    conversation_epoch: str
    generation_id: str
    items: tuple[TranscriptProjectionItem, ...]


@dataclass(frozen=True, slots=True)
class TranscriptProjectionApplyReceipt(ManagedValue):
    operation_id: str
    payload_digest: str
    authority: ConversationProjectionAuthority
    status: TranscriptProjectionApplyStatus


@dataclass(frozen=True, slots=True)
class TranscriptProjectionSettlement(ManagedValue):
    status: TranscriptProjectionSettlementStatus
    operation_id: str | None
    conversation_id: str | None
    generation_id: str | None
    attempt: int
    error_class: TranscriptProjectionErrorClass | None
    detail: str | None


@dataclass(frozen=True, slots=True)
class ManagedConversationDelete(ManagedValue):
    schema_version: int
    operation_id: str
    payload_digest: str
    conversation_id: str
    expected_epoch: str


@dataclass(frozen=True, slots=True)
class ManagedConversationDeleteReceipt(ManagedValue):
    operation_id: str
    payload_digest: str
    deleted_epoch: str
    retention_floor_epoch: str
    status: ManagedConversationDeleteStatus


@dataclass(frozen=True, slots=True)
class ManagedConversationImport(ManagedValue):
    schema_version: int
    operation_id: str
    payload_digest: str
    conversation_id: str
    expected_epoch: str
    expected_revision: str
    generation_id: str | None
    messages: tuple[StoredMessage, ...]


@dataclass(frozen=True, slots=True)
class RuntimeScopeAuthority(ManagedValue):
    scope_id: str
    conversation_epoch: str | None
    revision: str
    lifecycle: RuntimeScopeLifecycle


@dataclass(frozen=True, slots=True)
class ManagedRuntimeStateSnapshot(ManagedValue):
    scope: RuntimeScopeAuthority
    runtime_state_id: str
    version: RuntimeStateVersion
    checkpoint: AgentCheckpoint | None


@dataclass(frozen=True, slots=True)
class RuntimeCheckpointCasRequest(ManagedValue):
    scope_id: str
    runtime_state_id: str
    conversation_epoch: str | None
    expected_scope_revision: str
    expected_state_version: RuntimeStateExpectedVersion
    checkpoint: AgentCheckpoint
    managed_import: ManagedImportEpochTransition | None


@dataclass(frozen=True, slots=True)
class ManagedImportEpochTransition(ManagedValue):
    request: ManagedConversationImport
    receipt: TranscriptProjectionApplyReceipt


@dataclass(frozen=True, slots=True)
class RuntimeCheckpointCasReceipt(ManagedValue):
    scope: RuntimeScopeAuthority
    runtime_state_id: str
    version: RuntimeStateVersion
    status: RuntimeCheckpointCasStatus


@dataclass(frozen=True, slots=True)
class RuntimeTranscriptAckRequest(ManagedValue):
    scope_id: str
    runtime_state_id: str
    conversation_epoch: str
    expected_scope_revision: str
    expected_state_version: RuntimeStateExpectedVersion
    projection_receipt: TranscriptProjectionApplyReceipt
    checkpoint: AgentCheckpoint


@dataclass(frozen=True, slots=True)
class RuntimeGenerationRetireRequest(ManagedValue):
    schema_version: int
    operation_id: str
    payload_digest: str
    scope_id: str
    runtime_state_id: str
    expected_scope_revision: str
    expected_state_version: RuntimeStateExpectedVersion


@dataclass(frozen=True, slots=True)
class RuntimeGenerationRetireReceipt(ManagedValue):
    scope: RuntimeScopeAuthority
    runtime_state_id: str
    version: RuntimeStateVersion
    status: RuntimeGenerationRetireStatus


@dataclass(frozen=True, slots=True)
class ScopeRetirementItem(ManagedValue):
    runtime_state_id: str
    state_version: RuntimeStateVersion
    pending_operation_id: str | None
    status: ScopeRetirementItemStatus


@dataclass(frozen=True, slots=True)
class ScopeRetirementManifest(ManagedValue):
    delete_operation_id: str
    payload_digest: str
    expected_conversation_epoch: str
    items: tuple[ScopeRetirementItem, ...]
    conversation_delete_receipt: ManagedConversationDeleteReceipt | None


@dataclass(frozen=True, slots=True)
class ScopeRetirementRequest(ManagedValue):
    schema_version: int
    delete_operation_id: str
    payload_digest: str
    scope_id: str
    expected_scope_revision: str
    expected_conversation_epoch: str


@dataclass(frozen=True, slots=True)
class ScopeRetirementReceipt(ManagedValue):
    scope: RuntimeScopeAuthority
    manifest: ScopeRetirementManifest
    dropped_operation_ids: tuple[str, ...]
    retention_floor_epoch: str
    status: ScopeRetirementStatus


@dataclass(frozen=True, slots=True)
class AgentCheckpoint(ManagedValue):
    conversation_id: str
    messages_json: str
    current_plan: str | None
    active_skills: tuple[str, ...]
    blocked_reason: str | None
    working_dir: Mapping[str, Any] | None
    timestamp: Mapping[str, Any]


@dataclass(frozen=True, slots=True)
class TaskSpec(ManagedValue):
    id: str
    title: str
    description: str
    depends_on: tuple[str, ...]
    max_retries: int
    extension: Mapping[str, Any]


@dataclass(frozen=True, slots=True)
class TaskClaim(ManagedValue):
    claim_id: str
    revision: str
    attempt: int
    spec_hash: str


@dataclass(frozen=True, slots=True)
class TaskExecution(ManagedValue):
    task_id: str
    status: WireTaskStatus
    retry_count: int
    failure_fingerprint: str | None
    claim: TaskClaim | None
    result: Mapping[str, Any] | None
    interruption_requested: RuntimeInterruptionDisposition | None


@dataclass(frozen=True, slots=True)
class Task(ManagedValue):
    spec: TaskSpec
    execution: TaskExecution


@dataclass(frozen=True, slots=True)
class RuntimePlanSnapshot(ManagedValue):
    revision: str
    tasks: tuple[Task, ...]


@dataclass(frozen=True, slots=True)
class TaskGraphContext(ManagedValue):
    goal: str
    assumptions: tuple[str, ...]
    risks: tuple[str, ...]
    execution_mode: TaskGraphExecutionMode
    metadata: Mapping[str, Any]


@dataclass(frozen=True, slots=True)
class RevisionedTaskGraph(ManagedValue):
    snapshot: RuntimePlanSnapshot
    context: TaskGraphContext


@dataclass(frozen=True, slots=True)
class TaskPatchEffects(ManagedValue):
    inserted_task_ids: tuple[str, ...]
    updated_task_ids: tuple[str, ...]
    skipped_task_ids: tuple[str, ...]
    reset_task_ids: tuple[str, ...]
    progressed_task_ids: tuple[str, ...]
    reordered: bool


@dataclass(frozen=True, slots=True)
class TaskGraphCommit(ManagedValue):
    expected_revision: str | None
    expected_executions: Mapping[str, TaskExecution] | None
    next: RevisionedTaskGraph
    reason: str
    effects: TaskPatchEffects


@dataclass(frozen=True, slots=True)
class TaskExecutionCommit(ManagedValue):
    expected_revision: str
    expected_executions: Mapping[str, TaskExecution]
    next: RevisionedTaskGraph


SCHEMA = {
    "ConversationProjectionCapability": {
        "variants": {
            "unsupported": {
                "class": "ConversationProjectionCapabilityUnsupported",
                "fields": {},
            },
            "atomic_v1": {
                "class": "ConversationProjectionCapabilityAtomicV1",
                "fields": {},
            },
        },
        "unit": True,
    },
    "PersistenceCallCapability": {
        "variants": {
            "unsupported": {
                "class": "PersistenceCallCapabilityUnsupported",
                "fields": {},
            },
            "absolute_deadline_v1": {
                "class": "PersistenceCallCapabilityAbsoluteDeadlineV1",
                "fields": {},
            },
        },
        "unit": True,
    },
    "ConversationProjectionLifecycle": {
        "variants": {
            "live": {"class": "ConversationProjectionLifecycleLive", "fields": {}},
            "deleted": {
                "class": "ConversationProjectionLifecycleDeleted",
                "fields": {},
            },
        },
        "unit": True,
    },
    "ConversationProjectionEpochStatus": {
        "variants": {
            "created": {
                "class": "ConversationProjectionEpochStatusCreated",
                "fields": {},
            },
            "adopted_legacy": {
                "class": "ConversationProjectionEpochStatusAdoptedLegacy",
                "fields": {},
            },
            "existing": {
                "class": "ConversationProjectionEpochStatusExisting",
                "fields": {},
            },
            "recreated": {
                "class": "ConversationProjectionEpochStatusRecreated",
                "fields": {},
            },
            "tombstoned": {
                "class": "ConversationProjectionEpochStatusTombstoned",
                "fields": {},
            },
            "epoch_conflict": {
                "class": "ConversationProjectionEpochStatusEpochConflict",
                "fields": {},
            },
        },
        "unit": True,
    },
    "TranscriptProjectionConflictKind": {
        "variants": {
            "operation_identity": {
                "class": "TranscriptProjectionConflictKindOperationIdentity",
                "fields": {},
            },
            "ordinal_digest": {
                "class": "TranscriptProjectionConflictKindOrdinalDigest",
                "fields": {},
            },
            "revision": {
                "class": "TranscriptProjectionConflictKindRevision",
                "fields": {},
            },
            "managed_import": {
                "class": "TranscriptProjectionConflictKindManagedImport",
                "fields": {},
            },
        },
        "unit": True,
    },
    "TranscriptProjectionApplyStatus": {
        "variants": {
            "applied": {
                "class": "TranscriptProjectionApplyStatusApplied",
                "fields": {},
            },
            "already_applied": {
                "class": "TranscriptProjectionApplyStatusAlreadyApplied",
                "fields": {},
            },
            "fenced": {
                "class": "TranscriptProjectionApplyStatusFenced",
                "fields": {
                    "current_epoch": "WireU64",
                    "lifecycle": "ConversationProjectionLifecycleWire",
                },
            },
            "conflict": {
                "class": "TranscriptProjectionApplyStatusConflict",
                "fields": {
                    "current_epoch": "WireU64",
                    "current_revision": "WireU64",
                    "kind": "TranscriptProjectionConflictKindWire",
                },
            },
        },
        "unit": False,
    },
    "TranscriptProjectionSettlementStatus": {
        "variants": {
            "settled": {
                "class": "TranscriptProjectionSettlementStatusSettled",
                "fields": {},
            },
            "deferred": {
                "class": "TranscriptProjectionSettlementStatusDeferred",
                "fields": {},
            },
            "blocked": {
                "class": "TranscriptProjectionSettlementStatusBlocked",
                "fields": {},
            },
            "conflict": {
                "class": "TranscriptProjectionSettlementStatusConflict",
                "fields": {},
            },
        },
        "unit": True,
    },
    "TranscriptProjectionErrorClass": {
        "variants": {
            "outcome_unknown": {
                "class": "TranscriptProjectionErrorClassOutcomeUnknown",
                "fields": {},
            },
            "transient_no_commit": {
                "class": "TranscriptProjectionErrorClassTransientNoCommit",
                "fields": {},
            },
            "revision_conflict": {
                "class": "TranscriptProjectionErrorClassRevisionConflict",
                "fields": {},
            },
            "deadline_exceeded": {
                "class": "TranscriptProjectionErrorClassDeadlineExceeded",
                "fields": {},
            },
            "unsupported": {
                "class": "TranscriptProjectionErrorClassUnsupported",
                "fields": {},
            },
            "invalid_configuration": {
                "class": "TranscriptProjectionErrorClassInvalidConfiguration",
                "fields": {},
            },
            "corrupt_state": {
                "class": "TranscriptProjectionErrorClassCorruptState",
                "fields": {},
            },
            "semantic_conflict": {
                "class": "TranscriptProjectionErrorClassSemanticConflict",
                "fields": {},
            },
        },
        "unit": True,
    },
    "ManagedConversationDeleteStatus": {
        "variants": {
            "deleted": {
                "class": "ManagedConversationDeleteStatusDeleted",
                "fields": {},
            },
            "already_deleted": {
                "class": "ManagedConversationDeleteStatusAlreadyDeleted",
                "fields": {},
            },
            "epoch_conflict": {
                "class": "ManagedConversationDeleteStatusEpochConflict",
                "fields": {},
            },
            "receipt_expired": {
                "class": "ManagedConversationDeleteStatusReceiptExpired",
                "fields": {},
            },
            "identity_conflict": {
                "class": "ManagedConversationDeleteStatusIdentityConflict",
                "fields": {},
            },
        },
        "unit": True,
    },
    "RuntimeStateCapability": {
        "variants": {
            "unsupported": {"class": "RuntimeStateCapabilityUnsupported", "fields": {}},
            "revisioned_v1": {
                "class": "RuntimeStateCapabilityRevisionedV1",
                "fields": {},
            },
        },
        "unit": True,
    },
    "RuntimeStateVersion": {
        "variants": {
            "absent": {"class": "RuntimeStateVersionAbsent", "fields": {}},
            "unmanaged": {
                "class": "RuntimeStateVersionUnmanaged",
                "fields": {"digest": "String"},
            },
            "managed": {
                "class": "RuntimeStateVersionManaged",
                "fields": {"revision": "WireU64"},
            },
            "retired": {
                "class": "RuntimeStateVersionRetired",
                "fields": {"revision": "WireU64", "operation_id": "String"},
            },
        },
        "unit": False,
    },
    "RuntimeStateExpectedVersion": {
        "variants": {
            "absent": {"class": "RuntimeStateExpectedVersionAbsent", "fields": {}},
            "unmanaged": {
                "class": "RuntimeStateExpectedVersionUnmanaged",
                "fields": {"digest": "String"},
            },
            "managed": {
                "class": "RuntimeStateExpectedVersionManaged",
                "fields": {"revision": "WireU64"},
            },
        },
        "unit": False,
    },
    "RuntimeScopeLifecycle": {
        "variants": {
            "active": {"class": "RuntimeScopeLifecycleActive", "fields": {}},
            "retiring": {"class": "RuntimeScopeLifecycleRetiring", "fields": {}},
            "tombstoned": {"class": "RuntimeScopeLifecycleTombstoned", "fields": {}},
        },
        "unit": True,
    },
    "RuntimeCheckpointCasStatus": {
        "variants": {
            "applied": {"class": "RuntimeCheckpointCasStatusApplied", "fields": {}},
            "already_current": {
                "class": "RuntimeCheckpointCasStatusAlreadyCurrent",
                "fields": {},
            },
            "revision_conflict": {
                "class": "RuntimeCheckpointCasStatusRevisionConflict",
                "fields": {},
            },
            "generation_retired": {
                "class": "RuntimeCheckpointCasStatusGenerationRetired",
                "fields": {},
            },
            "scope_fenced": {
                "class": "RuntimeCheckpointCasStatusScopeFenced",
                "fields": {},
            },
        },
        "unit": True,
    },
    "RuntimeGenerationRetireStatus": {
        "variants": {
            "retired": {"class": "RuntimeGenerationRetireStatusRetired", "fields": {}},
            "already_retired": {
                "class": "RuntimeGenerationRetireStatusAlreadyRetired",
                "fields": {},
            },
            "conflict": {
                "class": "RuntimeGenerationRetireStatusConflict",
                "fields": {},
            },
            "pending_projection": {
                "class": "RuntimeGenerationRetireStatusPendingProjection",
                "fields": {},
            },
            "scope_fenced": {
                "class": "RuntimeGenerationRetireStatusScopeFenced",
                "fields": {},
            },
        },
        "unit": True,
    },
    "ScopeRetirementItemStatus": {
        "variants": {
            "pending": {"class": "ScopeRetirementItemStatusPending", "fields": {}},
            "dropped_by_delete": {
                "class": "ScopeRetirementItemStatusDroppedByDelete",
                "fields": {},
            },
        },
        "unit": True,
    },
    "ScopeRetirementAdvance": {
        "variants": {
            "conversation_deleted": {
                "class": "ScopeRetirementAdvanceConversationDeleted",
                "fields": {"receipt": "ManagedConversationDeleteReceiptWire"},
            },
            "generation_dropped": {
                "class": "ScopeRetirementAdvanceGenerationDropped",
                "fields": {
                    "runtime_state_id": "String",
                    "pending_operation_id": "Option<String>",
                },
            },
            "complete": {"class": "ScopeRetirementAdvanceComplete", "fields": {}},
        },
        "unit": False,
    },
    "ScopeRetirementStatus": {
        "variants": {
            "begun": {"class": "ScopeRetirementStatusBegun", "fields": {}},
            "in_progress": {"class": "ScopeRetirementStatusInProgress", "fields": {}},
            "completed": {"class": "ScopeRetirementStatusCompleted", "fields": {}},
            "already_completed": {
                "class": "ScopeRetirementStatusAlreadyCompleted",
                "fields": {},
            },
            "revision_conflict": {
                "class": "ScopeRetirementStatusRevisionConflict",
                "fields": {},
            },
            "epoch_conflict": {
                "class": "ScopeRetirementStatusEpochConflict",
                "fields": {},
            },
            "receipt_expired": {
                "class": "ScopeRetirementStatusReceiptExpired",
                "fields": {},
            },
            "identity_conflict": {
                "class": "ScopeRetirementStatusIdentityConflict",
                "fields": {},
            },
        },
        "unit": True,
    },
    "RuntimeInterruptionDisposition": {
        "variants": {
            "cancelled": {
                "class": "RuntimeInterruptionDispositionCancelled",
                "fields": {},
            },
            "paused": {
                "class": "RuntimeInterruptionDispositionPaused",
                "fields": {"reason": "String"},
            },
        },
        "unit": False,
    },
    "TaskGraphExecutionMode": {
        "variants": {
            "sequential": {"class": "TaskGraphExecutionModeSequential", "fields": {}},
            "parallel": {"class": "TaskGraphExecutionModeParallel", "fields": {}},
        },
        "unit": True,
    },
    "WireTaskStatus": {
        "variants": {
            "pending": {"class": "WireTaskStatusPending", "fields": {}},
            "running": {"class": "WireTaskStatusRunning", "fields": {}},
            "blocked": {
                "class": "WireTaskStatusBlocked",
                "fields": {"reason": "String"},
            },
            "completed": {"class": "WireTaskStatusCompleted", "fields": {}},
            "failed": {"class": "WireTaskStatusFailed", "fields": {"error": "String"}},
            "skipped": {"class": "WireTaskStatusSkipped", "fields": {}},
            "cancelled": {"class": "WireTaskStatusCancelled", "fields": {}},
            "timed_out": {
                "class": "WireTaskStatusTimedOut",
                "fields": {"error": "String"},
            },
            "retrying": {
                "class": "WireTaskStatusRetrying",
                "fields": {"attempt": "u32", "last_error": "String"},
            },
            "paused": {"class": "WireTaskStatusPaused", "fields": {"reason": "String"}},
        },
        "unit": False,
    },
    "PersistenceCallContext": {"fields": {"absolute_deadline_unix_ms": "WireI64"}},
    "ConversationProjectionAuthority": {
        "fields": {
            "conversation_id": "String",
            "epoch": "WireU64",
            "revision": "WireU64",
            "lifecycle": "ConversationProjectionLifecycleWire",
            "delete_receipt_retention_floor_epoch": "WireU64",
        }
    },
    "NewConversation": {
        "fields": {
            "conversation_id": "String",
            "user_id": "String",
            "agent_type": "Option<String>",
            "title": "Option<String>",
        }
    },
    "StoredMessage": {
        "fields": {
            "id": "Option<WireI64>",
            "conversation_id": "String",
            "role": "String",
            "content": "Option<String>",
            "attachments_json": "Option<String>",
            "tool_calls_json": "Option<String>",
            "tool_result_json": "Option<String>",
            "created_at": "String",
        }
    },
    "EnsureConversationProjectionRequest": {
        "fields": {
            "conversation": "NewConversationWire",
            "expected_tombstone_epoch": "Option<WireU64>",
        }
    },
    "ConversationProjectionEpochReceipt": {
        "fields": {
            "authority": "ConversationProjectionAuthorityWire",
            "status": "ConversationProjectionEpochStatusWire",
        }
    },
    "TranscriptProjectionItem": {
        "fields": {
            "ordinal": "WireU64",
            "digest": "String",
            "message": "StoredMessageWire",
        }
    },
    "TranscriptProjectionBatch": {
        "fields": {
            "schema_version": "u16",
            "operation_id": "String",
            "payload_digest": "String",
            "conversation_id": "String",
            "conversation_epoch": "WireU64",
            "generation_id": "String",
            "items": "Vec<TranscriptProjectionItemWire>",
        }
    },
    "TranscriptProjectionApplyReceipt": {
        "fields": {
            "operation_id": "String",
            "payload_digest": "String",
            "authority": "ConversationProjectionAuthorityWire",
            "status": "TranscriptProjectionApplyStatusWire",
        }
    },
    "TranscriptProjectionSettlement": {
        "fields": {
            "status": "TranscriptProjectionSettlementStatusWire",
            "operation_id": "Option<String>",
            "conversation_id": "Option<String>",
            "generation_id": "Option<String>",
            "attempt": "u32",
            "error_class": "Option<TranscriptProjectionErrorClassWire>",
            "detail": "Option<String>",
        }
    },
    "ManagedConversationDelete": {
        "fields": {
            "schema_version": "u16",
            "operation_id": "String",
            "payload_digest": "String",
            "conversation_id": "String",
            "expected_epoch": "WireU64",
        }
    },
    "ManagedConversationDeleteReceipt": {
        "fields": {
            "operation_id": "String",
            "payload_digest": "String",
            "deleted_epoch": "WireU64",
            "retention_floor_epoch": "WireU64",
            "status": "ManagedConversationDeleteStatusWire",
        }
    },
    "ManagedConversationImport": {
        "fields": {
            "schema_version": "u16",
            "operation_id": "String",
            "payload_digest": "String",
            "conversation_id": "String",
            "expected_epoch": "WireU64",
            "expected_revision": "WireU64",
            "generation_id": "Option<String>",
            "messages": "Vec<StoredMessageWire>",
        }
    },
    "RuntimeScopeAuthority": {
        "fields": {
            "scope_id": "String",
            "conversation_epoch": "Option<WireU64>",
            "revision": "WireU64",
            "lifecycle": "RuntimeScopeLifecycleWire",
        }
    },
    "ManagedRuntimeStateSnapshot": {
        "fields": {
            "scope": "RuntimeScopeAuthorityWire",
            "runtime_state_id": "String",
            "version": "RuntimeStateVersionWire",
            "checkpoint": "Option<AgentCheckpointWire>",
        }
    },
    "RuntimeCheckpointCasRequest": {
        "fields": {
            "scope_id": "String",
            "runtime_state_id": "String",
            "conversation_epoch": "Option<WireU64>",
            "expected_scope_revision": "WireU64",
            "expected_state_version": "RuntimeStateExpectedVersionWire",
            "checkpoint": "AgentCheckpointWire",
            "managed_import": "Option<ManagedImportEpochTransitionWire>",
        }
    },
    "ManagedImportEpochTransition": {
        "fields": {
            "request": "ManagedConversationImportWire",
            "receipt": "TranscriptProjectionApplyReceiptWire",
        }
    },
    "RuntimeCheckpointCasReceipt": {
        "fields": {
            "scope": "RuntimeScopeAuthorityWire",
            "runtime_state_id": "String",
            "version": "RuntimeStateVersionWire",
            "status": "RuntimeCheckpointCasStatusWire",
        }
    },
    "RuntimeTranscriptAckRequest": {
        "fields": {
            "scope_id": "String",
            "runtime_state_id": "String",
            "conversation_epoch": "WireU64",
            "expected_scope_revision": "WireU64",
            "expected_state_version": "RuntimeStateExpectedVersionWire",
            "projection_receipt": "TranscriptProjectionApplyReceiptWire",
            "checkpoint": "AgentCheckpointWire",
        }
    },
    "RuntimeGenerationRetireRequest": {
        "fields": {
            "schema_version": "u16",
            "operation_id": "String",
            "payload_digest": "String",
            "scope_id": "String",
            "runtime_state_id": "String",
            "expected_scope_revision": "WireU64",
            "expected_state_version": "RuntimeStateExpectedVersionWire",
        }
    },
    "RuntimeGenerationRetireReceipt": {
        "fields": {
            "scope": "RuntimeScopeAuthorityWire",
            "runtime_state_id": "String",
            "version": "RuntimeStateVersionWire",
            "status": "RuntimeGenerationRetireStatusWire",
        }
    },
    "ScopeRetirementItem": {
        "fields": {
            "runtime_state_id": "String",
            "state_version": "RuntimeStateVersionWire",
            "pending_operation_id": "Option<String>",
            "status": "ScopeRetirementItemStatusWire",
        }
    },
    "ScopeRetirementManifest": {
        "fields": {
            "delete_operation_id": "String",
            "payload_digest": "String",
            "expected_conversation_epoch": "WireU64",
            "items": "Vec<ScopeRetirementItemWire>",
            "conversation_delete_receipt": "Option<ManagedConversationDeleteReceiptWire>",
        }
    },
    "ScopeRetirementRequest": {
        "fields": {
            "schema_version": "u16",
            "delete_operation_id": "String",
            "payload_digest": "String",
            "scope_id": "String",
            "expected_scope_revision": "WireU64",
            "expected_conversation_epoch": "WireU64",
        }
    },
    "ScopeRetirementReceipt": {
        "fields": {
            "scope": "RuntimeScopeAuthorityWire",
            "manifest": "ScopeRetirementManifestWire",
            "dropped_operation_ids": "Vec<String>",
            "retention_floor_epoch": "WireU64",
            "status": "ScopeRetirementStatusWire",
        }
    },
    "AgentCheckpoint": {
        "fields": {
            "conversation_id": "String",
            "messages_json": "String",
            "current_plan": "Option<String>",
            "active_skills": "Vec<String>",
            "blocked_reason": "Option<String>",
            "working_dir": "Option<WirePath>",
            "timestamp": "WireTimestamp",
        }
    },
    "TaskSpec": {
        "fields": {
            "id": "String",
            "title": "String",
            "description": "String",
            "depends_on": "Vec<String>",
            "max_retries": "u32",
            "extension": "WireValue",
        }
    },
    "TaskClaim": {
        "fields": {
            "claim_id": "String",
            "revision": "WireU64",
            "attempt": "u32",
            "spec_hash": "String",
        }
    },
    "TaskExecution": {
        "fields": {
            "task_id": "String",
            "status": "WireTaskStatus",
            "retry_count": "u32",
            "failure_fingerprint": "Option<String>",
            "claim": "Option<TaskClaimWire>",
            "result": "Option<WireValue>",
            "interruption_requested": "Option<RuntimeInterruptionDispositionWire>",
        }
    },
    "Task": {"fields": {"spec": "TaskSpecWire", "execution": "TaskExecutionWire"}},
    "RuntimePlanSnapshot": {
        "fields": {"revision": "WireU64", "tasks": "Vec<TaskWire>"}
    },
    "TaskGraphContext": {
        "fields": {
            "goal": "String",
            "assumptions": "Vec<String>",
            "risks": "Vec<String>",
            "execution_mode": "TaskGraphExecutionModeWire",
            "metadata": "WireValue",
        }
    },
    "RevisionedTaskGraph": {
        "fields": {
            "snapshot": "RuntimePlanSnapshotWire",
            "context": "TaskGraphContextWire",
        }
    },
    "TaskPatchEffects": {
        "fields": {
            "inserted_task_ids": "Vec<String>",
            "updated_task_ids": "Vec<String>",
            "skipped_task_ids": "Vec<String>",
            "reset_task_ids": "Vec<String>",
            "progressed_task_ids": "Vec<String>",
            "reordered": "bool",
        }
    },
    "TaskGraphCommit": {
        "fields": {
            "expected_revision": "Option<WireU64>",
            "expected_executions": "Option<BTreeMap<String, TaskExecutionWire>>",
            "next": "RevisionedTaskGraphWire",
            "reason": "String",
            "effects": "TaskPatchEffectsWire",
        }
    },
    "TaskExecutionCommit": {
        "fields": {
            "expected_revision": "WireU64",
            "expected_executions": "BTreeMap<String, TaskExecutionWire>",
            "next": "RevisionedTaskGraphWire",
        }
    },
}


def encode(value):
    if isinstance(value, ManagedValue):
        return value.to_wire()
    if isinstance(value, Enum):
        return value.value
    if isinstance(value, (list, tuple)):
        return [encode(item) for item in value]
    if isinstance(value, Mapping):
        return {key: encode(item) for key, item in value.items()}
    return value


def decode(type_name: str, value: Any):
    if type_name.startswith("Option<"):
        return None if value is None else decode(type_name[7:-1], value)
    if type_name.startswith("BTreeMap<String, "):
        if not isinstance(value, Mapping) or not all(
            isinstance(key, str) for key in value
        ):
            raise EchoAgentError(
                "serialization_violation", "managed map requires string keys"
            )
        return {key: decode(type_name[17:-1], item) for key, item in value.items()}
    if type_name == "WireValue":
        if not isinstance(value, Mapping) or not isinstance(value.get("kind"), str):
            raise EchoAgentError(
                "serialization_violation", "opaque value must retain WireValue wrapper"
            )
        return dict(value)
    if type_name.startswith("Vec<"):
        if not isinstance(value, (list, tuple)):
            raise EchoAgentError(
                "serialization_violation", "managed vector must be an array"
            )
        return tuple(decode(type_name[4:-1], item) for item in value)
    if type_name == "String":
        if not isinstance(value, str):
            raise EchoAgentError(
                "serialization_violation", "managed text must be string"
            )
        return value
    if type_name in {"WireI64", "WireU64"}:
        from .wire import wire_i64, wire_u64

        try:
            (wire_i64 if type_name == "WireI64" else wire_u64)(value)
        except (TypeError, ValueError, OverflowError) as error:
            raise EchoAgentError("serialization_violation", str(error)) from error
        if not isinstance(value, str):
            raise EchoAgentError(
                "serialization_violation",
                "managed integer requires canonical decimal text",
            )
        return value
    if type_name == "bool":
        if type(value) is not bool:
            raise EchoAgentError(
                "serialization_violation", "managed bool must be boolean"
            )
        return value
    if type_name in {"u16", "u32"}:
        if type(value) is not int or not 0 <= value < 1 << int(type_name[1:]):
            raise EchoAgentError(
                "serialization_violation", "managed integer is out of range"
            )
        return value
    if type_name in {"WirePath", "WireTimestamp"}:
        if not isinstance(value, Mapping):
            raise EchoAgentError(
                "serialization_violation", "managed path/time must retain wire object"
            )
        if type_name == "WirePath":
            import base64

            encoding = value.get("encoding")
            if encoding == "utf8":
                from .wire import wire_utf8_path

                if set(value) != {"encoding", "path"}:
                    raise EchoAgentError(
                        "serialization_violation", "UTF8 path fields invalid"
                    )
                try:
                    wire_utf8_path(value.get("path"))
                except (TypeError, ValueError) as error:
                    raise EchoAgentError(
                        "serialization_violation", str(error)
                    ) from error
            elif encoding in {"unix", "windows"}:
                field = "bytes_base64" if encoding == "unix" else "utf16_base64"
                encoded = value.get(field)
                if (
                    set(value) - {"encoding", field, "display"}
                    or not isinstance(encoded, str)
                    or value.get("display") is not None
                    and not isinstance(value["display"], str)
                ):
                    raise EchoAgentError(
                        "serialization_violation", "native path fields invalid"
                    )
                try:
                    raw = base64.b64decode(
                        encoded + "=" * ((-len(encoded)) % 4), validate=True
                    )
                except (ValueError, TypeError) as error:
                    raise EchoAgentError(
                        "serialization_violation", "native path invalid base64"
                    ) from error
                if base64.b64encode(raw).decode().rstrip("=") != encoded:
                    raise EchoAgentError(
                        "serialization_violation", "native path noncanonical base64"
                    )
                if encoding == "unix":
                    valid = raw.startswith(b"/") and b"\x00" not in raw
                else:
                    units = [
                        int.from_bytes(raw[index : index + 2], "little")
                        for index in range(0, len(raw), 2)
                    ]
                    valid = (
                        len(raw) % 2 == 0
                        and 0 not in units
                        and (
                            (
                                len(units) >= 3
                                and (65 <= units[0] <= 90 or 97 <= units[0] <= 122)
                                and units[1] == 58
                                and units[2] in {47, 92}
                            )
                            or len(units) >= 2
                            and units[:2] == [92, 92]
                        )
                    )
                if not valid:
                    raise EchoAgentError(
                        "serialization_violation",
                        "native path must be absolute and NUL-free",
                    )
            else:
                raise EchoAgentError("serialization_violation", "unknown path encoding")
        if type_name == "WireTimestamp":
            decode("WireI64", value.get("unix_seconds"))
            nanos = value.get("nanos")
            if type(nanos) is not int or not 0 <= nanos < 1_000_000_000:
                raise EchoAgentError(
                    "serialization_violation", "timestamp nanos out of range"
                )
        return dict(value)
    name = type_name.removesuffix("Wire")
    definition = SCHEMA[name]
    if name == "WireTaskStatus":
        if (
            not isinstance(value, Mapping)
            or value.get("status") not in definition["variants"]
        ):
            raise EchoAgentError("serialization_violation", "task status is malformed")
        variant = definition["variants"][value["status"]]
        return _decode_fields(
            variant["class"],
            variant["fields"],
            {field: item for field, item in value.items() if field != "status"},
        )
    if "variants" in definition:
        if definition["unit"]:
            if not isinstance(value, str) or value not in definition["variants"]:
                raise EchoAgentError("serialization_violation", "unknown managed enum")
            return globals()[name](value)
        tag = (
            value
            if isinstance(value, str)
            else next(iter(value))
            if isinstance(value, Mapping) and len(value) == 1
            else None
        )
        variant = definition["variants"].get(tag)
        if variant is None:
            raise EchoAgentError("serialization_violation", "unknown managed variant")
        payload = {} if isinstance(value, str) else value[tag]
        return _decode_fields(variant["class"], variant["fields"], payload)
    return _decode_fields(name, definition["fields"], value)


def _decode_fields(name, definitions, value):
    if (
        not isinstance(value, Mapping)
        or set(value) - set(definitions)
        or any(
            field not in value and not kind.startswith("Option<")
            for field, kind in definitions.items()
        )
    ):
        raise EchoAgentError(
            "serialization_violation", name + " has missing/unknown fields"
        )
    values = {
        field: decode(kind, value.get(field)) for field, kind in definitions.items()
    }
    return globals()[name](**values)


MANAGED_OPERATIONS = {
    "conversation_ensure_projection_epoch_with_context": (
        {
            "request": "EnsureConversationProjectionRequestWire",
            "context": "PersistenceCallContextWire",
        },
        "receipt",
        "ConversationProjectionEpochReceiptWire",
    ),
    "conversation_get_projection_authority_with_context": (
        {"conversation_id": "String", "context": "PersistenceCallContextWire"},
        "authority",
        "Option<ConversationProjectionAuthorityWire>",
    ),
    "conversation_apply_transcript_projection_with_context": (
        {
            "batch": "TranscriptProjectionBatchWire",
            "context": "PersistenceCallContextWire",
        },
        "receipt",
        "TranscriptProjectionApplyReceiptWire",
    ),
    "conversation_delete_managed_with_context": (
        {
            "request": "ManagedConversationDeleteWire",
            "context": "PersistenceCallContextWire",
        },
        "receipt",
        "ManagedConversationDeleteReceiptWire",
    ),
    "runtime_load_managed_state_with_context": (
        {
            "scope_id": "String",
            "runtime_state_id": "String",
            "context": "PersistenceCallContextWire",
        },
        "snapshot",
        "Option<ManagedRuntimeStateSnapshotWire>",
    ),
    "runtime_load_scope_authority_with_context": (
        {"scope_id": "String", "context": "PersistenceCallContextWire"},
        "authority",
        "Option<RuntimeScopeAuthorityWire>",
    ),
    "runtime_compare_and_save_checkpoint_with_context": (
        {
            "request": "RuntimeCheckpointCasRequestWire",
            "context": "PersistenceCallContextWire",
        },
        "receipt",
        "RuntimeCheckpointCasReceiptWire",
    ),
    "runtime_acknowledge_transcript_projection_with_context": (
        {
            "request": "RuntimeTranscriptAckRequestWire",
            "context": "PersistenceCallContextWire",
        },
        "receipt",
        "RuntimeCheckpointCasReceiptWire",
    ),
    "runtime_retire_generation_with_context": (
        {
            "request": "RuntimeGenerationRetireRequestWire",
            "context": "PersistenceCallContextWire",
        },
        "receipt",
        "RuntimeGenerationRetireReceiptWire",
    ),
    "runtime_begin_scope_retirement_with_context": (
        {
            "request": "ScopeRetirementRequestWire",
            "context": "PersistenceCallContextWire",
        },
        "receipt",
        "ScopeRetirementReceiptWire",
    ),
    "runtime_continue_scope_retirement_with_context": (
        {
            "scope_id": "String",
            "delete_operation_id": "String",
            "expected_scope_revision": "WireU64",
            "advance": "ScopeRetirementAdvanceWire",
            "context": "PersistenceCallContextWire",
        },
        "receipt",
        "ScopeRetirementReceiptWire",
    ),
}


@dataclass(frozen=True, slots=True)
class ConversationEnsureProjectionEpochWithContextRequest(ManagedValue):
    request: EnsureConversationProjectionRequest
    context: PersistenceCallContext
    operation: ClassVar[str] = "conversation_ensure_projection_epoch_with_context"


@dataclass(frozen=True, slots=True)
class ConversationEnsureProjectionEpochWithContextResult(ManagedValue):
    receipt: ConversationProjectionEpochReceipt
    operation: ClassVar[str] = "conversation_ensure_projection_epoch_with_context"

    @property
    def component(self):
        return "conversation_store"

    @property
    def value(self):
        return {"receipt": encode(self.receipt)}

    def to_wire(self):
        value = super(
            ConversationEnsureProjectionEpochWithContextResult, self
        ).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class ConversationGetProjectionAuthorityWithContextRequest(ManagedValue):
    conversation_id: str
    context: PersistenceCallContext
    operation: ClassVar[str] = "conversation_get_projection_authority_with_context"


@dataclass(frozen=True, slots=True)
class ConversationGetProjectionAuthorityWithContextResult(ManagedValue):
    authority: ConversationProjectionAuthority | None
    operation: ClassVar[str] = "conversation_get_projection_authority_with_context"

    @property
    def component(self):
        return "conversation_store"

    @property
    def value(self):
        return {"authority": encode(self.authority)}

    def to_wire(self):
        value = super(
            ConversationGetProjectionAuthorityWithContextResult, self
        ).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class ConversationApplyTranscriptProjectionWithContextRequest(ManagedValue):
    batch: TranscriptProjectionBatch
    context: PersistenceCallContext
    operation: ClassVar[str] = "conversation_apply_transcript_projection_with_context"


@dataclass(frozen=True, slots=True)
class ConversationApplyTranscriptProjectionWithContextResult(ManagedValue):
    receipt: TranscriptProjectionApplyReceipt
    operation: ClassVar[str] = "conversation_apply_transcript_projection_with_context"

    @property
    def component(self):
        return "conversation_store"

    @property
    def value(self):
        return {"receipt": encode(self.receipt)}

    def to_wire(self):
        value = super(
            ConversationApplyTranscriptProjectionWithContextResult, self
        ).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class ConversationDeleteManagedWithContextRequest(ManagedValue):
    request: ManagedConversationDelete
    context: PersistenceCallContext
    operation: ClassVar[str] = "conversation_delete_managed_with_context"


@dataclass(frozen=True, slots=True)
class ConversationDeleteManagedWithContextResult(ManagedValue):
    receipt: ManagedConversationDeleteReceipt
    operation: ClassVar[str] = "conversation_delete_managed_with_context"

    @property
    def component(self):
        return "conversation_store"

    @property
    def value(self):
        return {"receipt": encode(self.receipt)}

    def to_wire(self):
        value = super(ConversationDeleteManagedWithContextResult, self).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class RuntimeLoadManagedStateWithContextRequest(ManagedValue):
    scope_id: str
    runtime_state_id: str
    context: PersistenceCallContext
    operation: ClassVar[str] = "runtime_load_managed_state_with_context"


@dataclass(frozen=True, slots=True)
class RuntimeLoadManagedStateWithContextResult(ManagedValue):
    snapshot: ManagedRuntimeStateSnapshot | None
    operation: ClassVar[str] = "runtime_load_managed_state_with_context"

    @property
    def component(self):
        return "runtime_state_store"

    @property
    def value(self):
        return {"snapshot": encode(self.snapshot)}

    def to_wire(self):
        value = super(RuntimeLoadManagedStateWithContextResult, self).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class RuntimeLoadScopeAuthorityWithContextRequest(ManagedValue):
    scope_id: str
    context: PersistenceCallContext
    operation: ClassVar[str] = "runtime_load_scope_authority_with_context"


@dataclass(frozen=True, slots=True)
class RuntimeLoadScopeAuthorityWithContextResult(ManagedValue):
    authority: RuntimeScopeAuthority | None
    operation: ClassVar[str] = "runtime_load_scope_authority_with_context"

    @property
    def component(self):
        return "runtime_state_store"

    @property
    def value(self):
        return {"authority": encode(self.authority)}

    def to_wire(self):
        value = super(RuntimeLoadScopeAuthorityWithContextResult, self).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class RuntimeCompareAndSaveCheckpointWithContextRequest(ManagedValue):
    request: RuntimeCheckpointCasRequest
    context: PersistenceCallContext
    operation: ClassVar[str] = "runtime_compare_and_save_checkpoint_with_context"


@dataclass(frozen=True, slots=True)
class RuntimeCompareAndSaveCheckpointWithContextResult(ManagedValue):
    receipt: RuntimeCheckpointCasReceipt
    operation: ClassVar[str] = "runtime_compare_and_save_checkpoint_with_context"

    @property
    def component(self):
        return "runtime_state_store"

    @property
    def value(self):
        return {"receipt": encode(self.receipt)}

    def to_wire(self):
        value = super(RuntimeCompareAndSaveCheckpointWithContextResult, self).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class RuntimeAcknowledgeTranscriptProjectionWithContextRequest(ManagedValue):
    request: RuntimeTranscriptAckRequest
    context: PersistenceCallContext
    operation: ClassVar[str] = "runtime_acknowledge_transcript_projection_with_context"


@dataclass(frozen=True, slots=True)
class RuntimeAcknowledgeTranscriptProjectionWithContextResult(ManagedValue):
    receipt: RuntimeCheckpointCasReceipt
    operation: ClassVar[str] = "runtime_acknowledge_transcript_projection_with_context"

    @property
    def component(self):
        return "runtime_state_store"

    @property
    def value(self):
        return {"receipt": encode(self.receipt)}

    def to_wire(self):
        value = super(
            RuntimeAcknowledgeTranscriptProjectionWithContextResult, self
        ).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class RuntimeRetireGenerationWithContextRequest(ManagedValue):
    request: RuntimeGenerationRetireRequest
    context: PersistenceCallContext
    operation: ClassVar[str] = "runtime_retire_generation_with_context"


@dataclass(frozen=True, slots=True)
class RuntimeRetireGenerationWithContextResult(ManagedValue):
    receipt: RuntimeGenerationRetireReceipt
    operation: ClassVar[str] = "runtime_retire_generation_with_context"

    @property
    def component(self):
        return "runtime_state_store"

    @property
    def value(self):
        return {"receipt": encode(self.receipt)}

    def to_wire(self):
        value = super(RuntimeRetireGenerationWithContextResult, self).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class RuntimeBeginScopeRetirementWithContextRequest(ManagedValue):
    request: ScopeRetirementRequest
    context: PersistenceCallContext
    operation: ClassVar[str] = "runtime_begin_scope_retirement_with_context"


@dataclass(frozen=True, slots=True)
class RuntimeBeginScopeRetirementWithContextResult(ManagedValue):
    receipt: ScopeRetirementReceipt
    operation: ClassVar[str] = "runtime_begin_scope_retirement_with_context"

    @property
    def component(self):
        return "runtime_state_store"

    @property
    def value(self):
        return {"receipt": encode(self.receipt)}

    def to_wire(self):
        value = super(RuntimeBeginScopeRetirementWithContextResult, self).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


@dataclass(frozen=True, slots=True)
class RuntimeContinueScopeRetirementWithContextRequest(ManagedValue):
    scope_id: str
    delete_operation_id: str
    expected_scope_revision: str
    advance: ScopeRetirementAdvance
    context: PersistenceCallContext
    operation: ClassVar[str] = "runtime_continue_scope_retirement_with_context"


@dataclass(frozen=True, slots=True)
class RuntimeContinueScopeRetirementWithContextResult(ManagedValue):
    receipt: ScopeRetirementReceipt
    operation: ClassVar[str] = "runtime_continue_scope_retirement_with_context"

    @property
    def component(self):
        return "runtime_state_store"

    @property
    def value(self):
        return {"receipt": encode(self.receipt)}

    def to_wire(self):
        value = super(RuntimeContinueScopeRetirementWithContextResult, self).to_wire()
        validate_result(self.operation, value)
        return {"operation": self.operation, "value": value}


def parse_request(operation, value):
    definitions = MANAGED_OPERATIONS[operation][0]
    if (
        not isinstance(value, Mapping)
        or set(value) - set(definitions)
        or any(
            field not in value and not kind.startswith("Option<")
            for field, kind in definitions.items()
        )
    ):
        raise EchoAgentError(
            "serialization_violation", "managed operation fields mismatch"
        )
    typed = {name: decode(kind, value[name]) for name, kind in definitions.items()}
    return globals()[
        "".join(item.title() for item in operation.split("_")) + "Request"
    ](**typed)


def validate_result(operation, value):
    _, field, kind = MANAGED_OPERATIONS[operation]
    if not isinstance(value, Mapping) or set(value) != {field}:
        raise EchoAgentError(
            "serialization_violation", "managed result fields mismatch"
        )
    decode(kind, value.get(field))


@dataclass(frozen=True, slots=True)
class RevisionedTaskExecutionCommitRequest(ManagedValue):
    scope_id: str
    commit: TaskExecutionCommit
    operation: ClassVar[str] = "revisioned_task_compare_and_commit_execution"


@dataclass(frozen=True, slots=True)
class RevisionedTaskExecutionCommittedResult(ManagedValue):
    graph: RevisionedTaskGraph
    operation: ClassVar[str] = "revisioned_task_compare_and_commit_execution"
    component: ClassVar[str] = "revisioned_task_store"

    @property
    def value(self):
        return {"graph": self.graph.to_wire()}

    def to_wire(self):
        value = {"graph": self.graph.to_wire()}
        decode("RevisionedTaskGraphWire", value["graph"])
        return {"operation": self.operation, "value": value}


def decode_settlement_event(value: Any) -> TranscriptProjectionSettlement:
    """Decode typed event facts without changing native WireU64 preservation."""
    if (
        not isinstance(value, Mapping)
        or value.get("kind") != "map"
        or not isinstance(value.get("value"), list)
    ):
        raise EchoAgentError(
            "serialization_violation", "settlement event requires wire map"
        )
    from .wire import from_wire

    fields = {}
    for entry in value["value"]:
        if (
            not isinstance(entry, Mapping)
            or not isinstance(entry.get("key"), Mapping)
            or entry["key"].get("kind") != "string"
            or not isinstance(entry["key"].get("value"), str)
            or "value" not in entry
        ):
            raise EchoAgentError(
                "serialization_violation", "settlement entry malformed"
            )
        name = entry["key"]["value"]
        if name in fields:
            raise EchoAgentError(
                "serialization_violation", "duplicate settlement field"
            )
        raw = entry["value"]
        if name == "attempt":
            if not isinstance(raw, Mapping) or raw.get("kind") != "u64":
                raise EchoAgentError(
                    "serialization_violation", "settlement attempt requires wire u64"
                )
            text = decode("WireU64", raw.get("value"))
            fields[name] = decode("u32", int(text))
        else:
            fields[name] = from_wire(raw)
    if set(fields) != set(SCHEMA["TranscriptProjectionSettlement"]["fields"]):
        raise EchoAgentError(
            "serialization_violation", "settlement fields incomplete or unknown"
        )
    return TranscriptProjectionSettlement.from_wire(fields)
