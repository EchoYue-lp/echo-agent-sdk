//! Lossless projections of the framework revisioned task-store boundary.
//! The framework alone validates and commits graph/execution CAS; no scheduler
//! or transition policy belongs in these transport DTOs.

use crate::methods::WireTaskStatus;
use crate::scalar::{WireU64, WireValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskSpecWire {
    pub id: String,
    pub title: String,
    pub description: String,
    pub depends_on: Vec<String>,
    pub max_retries: u32,
    /// Product-owned extension; its JSON domain remains lossless wire data.
    pub extension: WireValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskClaimWire {
    pub claim_id: String,
    pub revision: WireU64,
    pub attempt: u32,
    pub spec_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum RuntimeInterruptionDispositionWire {
    Cancelled,
    Paused { reason: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskExecutionWire {
    pub task_id: String,
    pub status: WireTaskStatus,
    pub retry_count: u32,
    pub failure_fingerprint: Option<String>,
    pub claim: Option<TaskClaimWire>,
    /// Absence means no supplied result; an explicit JSON null is
    /// `Some(WireValue::Null)` and cannot collapse into absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<WireValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interruption_requested: Option<RuntimeInterruptionDispositionWire>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskWire {
    pub spec: TaskSpecWire,
    pub execution: TaskExecutionWire,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RuntimePlanSnapshotWire {
    pub revision: WireU64,
    pub tasks: Vec<TaskWire>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskGraphExecutionModeWire {
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskGraphContextWire {
    pub goal: String,
    pub assumptions: Vec<String>,
    pub risks: Vec<String>,
    pub execution_mode: TaskGraphExecutionModeWire,
    pub metadata: WireValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RevisionedTaskGraphWire {
    pub snapshot: RuntimePlanSnapshotWire,
    pub context: TaskGraphContextWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskPatchEffectsWire {
    pub inserted_task_ids: Vec<String>,
    pub updated_task_ids: Vec<String>,
    pub skipped_task_ids: Vec<String>,
    pub reset_task_ids: Vec<String>,
    pub progressed_task_ids: Vec<String>,
    pub reordered: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskGraphCommitWire {
    pub expected_revision: Option<WireU64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_executions: Option<BTreeMap<String, TaskExecutionWire>>,
    pub next: RevisionedTaskGraphWire,
    pub reason: String,
    pub effects: TaskPatchEffectsWire,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskExecutionCommitWire {
    pub expected_revision: WireU64,
    pub expected_executions: BTreeMap<String, TaskExecutionWire>,
    pub next: RevisionedTaskGraphWire,
}
