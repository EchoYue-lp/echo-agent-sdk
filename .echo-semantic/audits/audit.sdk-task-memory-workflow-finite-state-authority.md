---
schema_version: 1
id: audit.sdk-task-memory-workflow-finite-state-authority
kind: audit
boundary_ref: boundary.sdk-task-subagent
lens: state_authority
freshness: examined
revision: 02059dd3014633a23c188e30e73b0634a8c5621d
finding_refs:
- finding.sdk-task-resume-extension-invocation-conflict
- finding.sdk-task-control-locality
- finding.sdk-task-runtime-store-authority
challenges:
  task-memory-workflow-accepted-obligation:
    revision: 02059dd3014633a23c188e30e73b0634a8c5621d
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/tests/core_profile_e2e.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TaskSubagentHostTest.xml
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.WorkflowCheckpointHostTest.xml
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.MemoryPermissionHostTest.xml
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.LlmWorkflowStreamHostTest.xml
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
  specific-task-control-locality:
    revision: 622d7931e01556161441d22de0697f6f35f24f17
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs#request_live_interrupt
    - echo-sdk-host/tests/core_profile_e2e.rs#task_pause_is_exact_and_resume_reuses_canonical_dependency_result
    - echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/step10-import-fix/10-ci-core-profile-after.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TaskSubagentHostTest.xml
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
  specific-task-runtime-store-authority:
    revision: 622d7931e01556161441d22de0697f6f35f24f17
    source_refs:
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/tests/core_profile_e2e.rs#task_rpc_shares_the_session_task_authority
    - echo-sdk-host/tests/core_profile_e2e.rs#task_execute_and_control_settle_through_the_runtime
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/step10-import-fix/10-ci-core-profile-after.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/java-surefire/TEST-com.echoagent.sdk.TaskSubagentHostTest.xml
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
---

# Task/Memory/Workflow state_authority 有限审查

## 审查范围

回收 `/root/cli_framework_consumer_compatibility` 本session原真实sourceaudit文本，不称模型作者新review。原源码69b在02059仍相同，当前A源码/Host幂等另slice；Task/Subagent/memory/workflow四现family。

## 已检查故障假设

TaskRun graph/revision/claim/terminal唯一framework服务，Hostthin；每次memory调用当前Agentstore；Workflow graph/checkpoint原owner；context无第二store/状态。

## 实际实现路径与证据

原TaskRuntime/subagent/memory/workflow/selector/sourceoperation与四getter Hosttests具体路径在Evidence中，nativewhole790及四Java具名XML保完整结果。短await/同claim、canonicaldeps、liveStore/noeffect、Workflowclaim状态都有具名来源；原trace/control图不是第二authority。

## 问题记录

该原作者审查finiteexamined未新增已证实源码阻断；已有Taskphysical5s Finding保open。后续材料独立review不是把作者冒为independent source reviewer，原各slice独审范围保持。

## 残余风险

ContextInheritance纯值不证明实际history/memory继承E2E，compressor消费不泛推全hook路径；内存不承诺跨processdurable；12身份/5秒B/main未决。任务context native只runid，四getter另Host组合，不伪三字段reverseDTO。

## 未检查项

最新approved意图Acaller幂等、任意未接受sourceoperation或所有Subagent强停物理工作；不以未定义穷尽组合拉大本slice。

## task-control-locality原局部缺陷准确复核

原rootcancel全图误推改为exactattempt/controlclaim；pauseA/BRunning/Cpending→B/C完成canonicalB后resumeA2/D1，cancelA兄弟继续，3native4完整输出/次数；Bphysical独立。原真实来源修复/强断言/本session实际有限scope独ref复核支持，仅此root不物理B。newchallenge真实622d，原主02059audit/time/limits不改，knownliveLLM缺口不借Task场景解决。

## task-runtime-store-authority原局部缺陷准确复核

SessionServices取liveAgenttaskrevisionservice同Arc，CRUD/runtime同framework service/store无fallback双图；nativeCRUD-onlyexecute featureUnavailable agentcalls0拒前effect/canonicalCAS4scene真实。原真实来源修复/强断言/本session实际有限scope独ref复核支持，仅此root不物理B。newchallenge真实622d，原主02059audit/time/limits不改，knownliveLLM缺口不借Task场景解决。
