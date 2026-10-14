---
schema_version: 1
id: map.sdk-integrations
kind: capability_map
title: 集成能力与精确 source invoke
risk: high
observed_at: bd8e42e732588181b7a2589d91e9ffdce02d23e1
boundary_refs:
- boundary.sdk-integrations
behavior_refs:
- behavior.sdk-integrations
- behavior.sdk-family-memory
- behavior.sdk-family-workflow
- behavior.sdk-family-delivery
- behavior.sdk-family-state
- behavior.sdk-family-trace
- behavior.sdk-family-eval
- behavior.sdk-family-improve
- behavior.sdk-family-permission
- behavior.sdk-family-mcp
- behavior.sdk-family-a2a
- behavior.sdk-family-lsp
- behavior.sdk-family-channels
- behavior.sdk-family-telemetry
- behavior.sdk-family-topology
- behavior.sdk-family-web
- behavior.sdk-family-files
- behavior.sdk-family-shell
- behavior.sdk-family-git
- behavior.sdk-family-database
- behavior.sdk-family-rag
- behavior.sdk-family-chart
- behavior.sdk-family-media
- behavior.sdk-family-data
- behavior.sdk-family-statistics
- behavior.sdk-family-research
- behavior.sdk-family-content-guard
- behavior.sdk-family-project-rules
- behavior.sdk-family-testing
- behavior.sdk-family-structured-output
- behavior.sdk-family-source-operation
- behavior.sdk-family-invoke
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-channel-message-handler-consumer-audit
- evidence.sdk-mcp-channel-owned-candidate
- evidence.sdk-computation-50
- evidence.sdk-managed-supporting-operation-boundary
- evidence.sdk-rag-scalar-verification
- evidence.sdk-task-memory-workflow-finite-observation
- evidence.sdk-tools-bridge-finite-observation
- evidence.sdk-integrations-finite-observation
finding_refs:
- finding.sdk-adapter-invalid-input
- finding.sdk-delivery-exact-claim
- finding.sdk-family-handle-argument-projection
- finding.sdk-feature-failure-cause-fidelity
- finding.sdk-live-agent-service-view
- finding.sdk-lsp-response-fidelity
- finding.sdk-mcp-batch-publication
- finding.sdk-mcp-manager-owner
- finding.sdk-trace-publication-quota
- finding.sdk-channel-message-handler-consumer
- finding.sdk-channel-resource-operation-admission
- finding.sdk-rag-unicode-character-count
- finding.sdk-database-describe-dynamic-value
- finding.sdk-tool-artifact-context-propagation
- finding.sdk-clinical-trials-total-count-request
- finding.sdk-chart-dimension-integer-loss
- finding.sdk-git-branch-switch-dirty-path-overwrite
audit_refs:
- audit.sdk-computation-50-rereview
- audit.sdk-task-memory-workflow-finite-trigger-input
- audit.sdk-task-memory-workflow-finite-result-side-effect
- audit.sdk-task-memory-workflow-finite-state-authority
- audit.sdk-task-memory-workflow-finite-data-durability
- audit.sdk-task-memory-workflow-finite-time-lifecycle
- audit.sdk-task-memory-workflow-finite-failure-concurrency
- audit.sdk-task-memory-workflow-finite-permission-external
- audit.sdk-task-memory-workflow-finite-contract-evidence
- audit.sdk-tools-bridge-finite-trigger-input
- audit.sdk-tools-bridge-finite-result-side-effect
- audit.sdk-tools-bridge-finite-state-authority
- audit.sdk-tools-bridge-finite-data-durability
- audit.sdk-tools-bridge-finite-time-lifecycle
- audit.sdk-tools-bridge-finite-failure-concurrency
- audit.sdk-tools-bridge-finite-permission-external
- audit.sdk-tools-bridge-finite-contract-evidence
- audit.sdk-integrations-finite-trigger-input
- audit.sdk-integrations-finite-result-side-effect
- audit.sdk-integrations-finite-state-authority
- audit.sdk-integrations-finite-data-durability
- audit.sdk-integrations-finite-time-lifecycle
- audit.sdk-integrations-finite-failure-concurrency
- audit.sdk-integrations-finite-permission-external
- audit.sdk-integrations-finite-contract-evidence
related_map_refs:
- map.sdk-product-boundary
scenarios:
  behavior-lifecycle-failure:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-integrations
    finding_refs:
    - finding.sdk-adapter-invalid-input
    - finding.sdk-delivery-exact-claim
    - finding.sdk-family-handle-argument-projection
    - finding.sdk-feature-failure-cause-fidelity
    - finding.sdk-live-agent-service-view
    - finding.sdk-lsp-response-fidelity
    - finding.sdk-mcp-batch-publication
    - finding.sdk-mcp-manager-owner
    - finding.sdk-trace-publication-quota
    review_scope: Entry, unique authority, normal lifecycle and named failure obligations
      have been inspected and linked. Mapped records the model, not bug-free/runtime/main
      closure; open Findings remain explicit.
  family-memory:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-memory
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing的14个接受field
      payload已有独立纯值投影证据，不包含远程Mock runtime。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-memory-permission-candidate
    - evidence.sdk-task-memory-workflow-finite-observation
    review_scope: Task/Memory/Workflow finite material observes its actual02059 runtime
      via linked Evidence/Audit; this map retains original observation for unrelated
      families; memory/checkpoint volatile, contextnative gap andphysicalB remain.
  family-workflow:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-workflow
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing的14个接受field
      payload已有独立纯值投影证据，不包含远程Mock runtime。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-workflow-successor-candidate
    - evidence.sdk-task-memory-workflow-finite-observation
    review_scope: Task/Memory/Workflow finite material observes its actual02059 runtime
      via linked Evidence/Audit; this map retains original observation for unrelated
      families; memory/checkpoint volatile, contextnative gap andphysicalB remain.
  family-delivery:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-delivery
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-state-trace-delivery-candidate
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-state:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-state
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-state-trace-delivery-candidate
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-trace:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-trace
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-state-trace-delivery-candidate
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-eval:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-eval
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-improve:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-improve
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-permission:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-permission
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-memory-permission-candidate
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-mcp:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-mcp
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-a2a:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-a2a
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-lsp:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-lsp
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-channels:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-channels
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-channel-message-handler-consumer-audit
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-telemetry:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-telemetry
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing的14个接受field
      payload已有独立纯值投影证据，不包含远程Mock runtime。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
  family-topology:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-topology
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-web:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-web
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-files:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-files
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-shell:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-shell
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-git:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-git
    unknown: 已实际Git有限8lens源码审发现a634 branch switch将分支名作路径恢复，可能覆盖同名dirty tracked文件；具体Finding
      open。原status/diff具名证据不覆盖此窗口，log/blame/commit/worktree更多消费补证仍限原接受范围。
    next_step: 沿同一framework GitTool修switch并在临时fixture验证分支/文件/refs副作用，精确published pin后SDK三语言消费；不靠SDK重写Git权威。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tool-behavior-candidate
    finding_refs:
    - finding.sdk-git-branch-switch-dirty-path-overwrite
  family-database:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-database
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tool-behavior-candidate
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-rag:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-rag
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-chart:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-chart
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tool-behavior-candidate
    - evidence.sdk-tool-input-validation-candidate
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-media:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-media
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tool-behavior-candidate
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-data:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-data
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tool-behavior-candidate
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-statistics:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-statistics
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-research:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-research
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tool-behavior-candidate
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Named builtin family source/authority/lifecycle/failure fields independently
      examined in exact group; see linked5bcb Evidence. Originalmap observation unchanged
      for unrelatedintegrations; no completeallcodec/futureprovider/main guarantee.
  family-content-guard:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-content-guard
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing的14个接受field
      payload已有独立纯值投影证据，不包含远程Mock runtime。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
  family-project-rules:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-project-rules
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing的14个接受field
      payload已有独立纯值投影证据，不包含远程Mock runtime。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
  family-testing:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-testing
    evidence_refs:
    - evidence.sdk-testing-value-candidate
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-structured-output:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-structured-output
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing的14个接受field
      payload已有独立纯值投影证据，不包含远程Mock runtime。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
  family-source-operation:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-source-operation
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-integrations-finite-observation
    review_scope: Specific accepted integration/registry-skill supporting role independently
      examined, original790source restoredbd8 and later A2A native scope separately
      proven. Not all sourceoperation methods or currentA scope/fullgenerator/main.
  family-invoke:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    behavior_refs:
    - behavior.sdk-family-invoke
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing的14个接受field
      payload已有独立纯值投影证据，不包含远程Mock runtime。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
  integration-finite-trigger-input:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    behavior_refs:
    - behavior.sdk-integrations
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    audit_refs:
    - audit.sdk-integrations-finite-trigger-input
    review_scope: Original independent reviewer finite accepted family/table scope
      plus laterbd8 A2A proof;5sB/12/currentA/platform/main excluded.
  integration-finite-result-side-effect:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    behavior_refs:
    - behavior.sdk-integrations
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    audit_refs:
    - audit.sdk-integrations-finite-result-side-effect
    review_scope: Original independent reviewer finite accepted family/table scope
      plus laterbd8 A2A proof;5sB/12/currentA/platform/main excluded.
  integration-finite-state-authority:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    behavior_refs:
    - behavior.sdk-integrations
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    audit_refs:
    - audit.sdk-integrations-finite-state-authority
    review_scope: Original independent reviewer finite accepted family/table scope
      plus laterbd8 A2A proof;5sB/12/currentA/platform/main excluded.
  integration-finite-data-durability:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    behavior_refs:
    - behavior.sdk-integrations
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    audit_refs:
    - audit.sdk-integrations-finite-data-durability
    review_scope: Original independent reviewer finite accepted family/table scope
      plus laterbd8 A2A proof;5sB/12/currentA/platform/main excluded.
  integration-finite-time-lifecycle:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    behavior_refs:
    - behavior.sdk-integrations
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    audit_refs:
    - audit.sdk-integrations-finite-time-lifecycle
    review_scope: Original independent reviewer finite accepted family/table scope
      plus laterbd8 A2A proof;5sB/12/currentA/platform/main excluded.
  integration-finite-failure-concurrency:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    behavior_refs:
    - behavior.sdk-integrations
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    audit_refs:
    - audit.sdk-integrations-finite-failure-concurrency
    review_scope: Original independent reviewer finite accepted family/table scope
      plus laterbd8 A2A proof;5sB/12/currentA/platform/main excluded.
  integration-finite-permission-external:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    behavior_refs:
    - behavior.sdk-integrations
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    audit_refs:
    - audit.sdk-integrations-finite-permission-external
    review_scope: Original independent reviewer finite accepted family/table scope
      plus laterbd8 A2A proof;5sB/12/currentA/platform/main excluded.
  integration-finite-contract-evidence:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/handles.rs
    behavior_refs:
    - behavior.sdk-integrations
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-integrations-finite-observation
    audit_refs:
    - audit.sdk-integrations-finite-contract-evidence
    review_scope: Original independent reviewer finite accepted family/table scope
      plus laterbd8 A2A proof;5sB/12/currentA/platform/main excluded.
---

# 集成能力与精确 source invoke

## 能力范围

CompiledOperationCatalog按确切accepted identity/signature/feature寻址，薄adapter调用live framework service。memory/workflow读取当前Agent服务，mixednamespace/配置错误在副作用前拒绝；Delivery携完整claim而不以id重建当前attempt；trace构造验证后发布，quota失败可复用。MCP config/file批量SDKalias原子发布保原best-effort native连接权威及exact Arc+Session；工具投影完整ToolResult/effects/artifact/canonical cause。LSP真实29request场景+manager生命周期有30观测/44原始请求；新诊断另42观测/42原始通知跨三语言全字段匹配，severity缺失与integer/text code无损、每URI错误/恢复/清空隔离、无URI走原runtime fatal/PID退出。两份证据各绑定fc63/4d43不同不可变Host，不互冒同一final输入。

## 入口与输出

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 行为关系

本能力经 official ACP/profile 或具体 catalog route 到 Host adapter，然后调用框架权威。跨能力调用与三语言消费见 docs/sdk/governance/capability-map.md。

## 状态与数据流

Framework 拥有 Run/Task/Store 的事实与终态；Host 只拥有寻址、可取消资源 owner 与协议发送状态；客户端确认已消费事实。

## 策略来源与优先级

用户八项治理目标与 AGENTS.md 约束行为；accepted external contract 约束兼容；compiled feature/canonical framework service 约束实现。完整 Rust inventory 不扩承诺。

## 生命周期与失败路径

CompiledOperationCatalog按确切accepted identity/signature/feature寻址，薄adapter调用live framework service。memory/workflow读取当前Agent服务，mixednamespace/配置错误在副作用前拒绝；Delivery携完整claim而不以id重建当前attempt；trace构造验证后发布，quota失败可复用。MCP config/file批量SDKalias原子发布保原best-effort native连接权威及exact Arc+Session；工具投影完整ToolResult/effects/artifact/canonical cause。LSP真实29request场景+manager生命周期有30观测/44原始请求；新诊断另42观测/42原始通知跨三语言全字段匹配，severity缺失与integer/text code无损、每URI错误/恢复/清空隔离、无URI走原runtime fatal/PID退出。两份证据各绑定fc63/4d43不同不可变Host，不互冒同一final输入。

原阶段缺口（历史）：Workflow48行覆盖17/24已接受操作，原剩7项已有0744后继真实消费者证据；Memory/Permission59行、State/Trace/Delivery9项、MCP4项、LSP29请求/42诊断分别有来源。Guard/Pivot后继189ff与Tool新validator源须匹配自己的运行/复审；Channel inbound合同缺口单列。

## 权限与敏感信息

本地 SDK 不引入线上权限门控。继续保持框架自动行为 permission authority，凭据不进入日志/合同/错误；资源 gate 仅防止错误owner和无意数据丢失。

## 用户侧投影

TypeScript/Python/Java 对已接受能力暴露惯用接口，handle/error/value 必须无损；不能用成功默认值或丢字段改变权威事实。

## 场景处置清单

主场景mapped只表示模型已展开；关联family Behavior仍needs_review，具体Evidence按支持关系与原epoch读取。剩余义务：Workflow48行覆盖17/24已接受操作，原剩7项已有0744后继真实消费者证据；Memory/Permission59行、State/Trace/Delivery9项、MCP4项、LSP29请求/42诊断分别有来源。Guard/Pivot后继189ff与Tool新validator源须匹配自己的运行/复审；Channel inbound合同缺口单列。

## 未展开项

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch与95ecpin保持原运行epoch；当前Host已pin published b909并移除patch，但最终nightlyprofiles/唯一合同生成与SDKwhole源码门禁仍待。frameworkmain交付证据不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

Workflow48行覆盖17/24已接受操作，原剩7项已有0744后继真实消费者证据；Memory/Permission59行、State/Trace/Delivery9项、MCP4项、LSP29请求/42诊断分别有来源。Guard/Pivot后继189ff与Tool新validator源须匹配自己的运行/复审；Channel inbound合同缺口单列。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，family needs_review和开放Finding保留，主场景mapped不扩运行证据。

## 建模完成与验证状态区别

组级主路径已按入口、唯一权威、生命周期和具名失败义务完成映射，故主场景mapped。family Behavior仍needs_review，已知open Finding与有限证据保留；mapped不等于verified、所有lens covered或最终交付。剩余事项采用明确合同/故障原因，不以无限未枚举组合拒绝建模。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## Task/Memory/Workflow后继有限关系

本Map仅新增memory/workflow两family的02059具名Evidence/Audit关系，其它integration族的原观察版本、未知与状态不变；不把本次作者源审扩大成全部MCP/LSP/Channel已验证。

## 内置十工具族后继有限关系

只对files/shell/web/database/rag/chart/media/data/statistics/research十targetscene追加5bcb共享Evidence/8lensAudit与mapped范围；此Map其它能力的原observation、不确定性与状态不变。schema/parser/Store等唯一frameworkauthority不重复，任意未接受sourceoperation不自动进入菜单。

## 原辅助测试出处

sdks/typescript/test/lsp-diagnostics-host.test.js, sdks/python/tests/test_lsp_diagnostics_host.py, sdks/java/src/test/java/com/echoagent/sdk/LspDiagnosticsHostTest.java 保原569与历史收据，不假bd8已经提交；其原字段义务保，具体新group17source refs真实Git可查。

## 当前Integration八视角范围

原12family与2support角色scope按bd8真实Git观察，8审计具名examined、Map入口mapped；先前Task/Tool组后继Audit各自版本不改，原family其余范围/unknown/开放Finding继续保。memoryvolatile/Delivery8opsnonDurable、State7/Trace17、A2Afuturegen非restart/localSSE非remotephysical/Testingvalue非Mock必须按具体表，不以count或Catalog闭合。

## 后继 Git 分支切换实际源码审结果

framework_final_review 新有界八视角原source审 C0/I1/M0；具体 finding.sdk-git-branch-switch-dirty-path-overwrite 保open。a634 checkout -- target 被解释为路径恢复，不是分支切换，真实反例尚待执行，不能把静态确定反例写成executed RED。原Git已采status/diff/branch指定refs证据及旧审限制保持，当前缺口由此具名实现义务承接，不再用whole/main待验解释全部未知。SDK仍原薄adapter；新框架修复尚未发布或消费验证，不引用其它builtin10审作本族通过。
