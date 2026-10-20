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
- finding.sdk-live-agent-service-view
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
  specific-live-agent-service-replacement:
    revision: b7e6c68fbaa91cc97ad5fef4ff3d5a61b187345a
    source_refs:
    - echo-sdk-host/src/core_profile/facade/memory.rs#malformed_namespace_never_writes_and_replacement_store_is_live_authority
    - echo-sdk-host/src/core_profile/facade/workflow.rs#agent_node_uses_replaced_live_llm_after_session_services_capture
    - echo-sdk-host/src/factory.rs
    - docs/sdk/governance/evidence/live-service-llm-proof/receipt.json
    - docs/sdk/governance/evidence/live-service-llm-proof/independent-review.json
    - docs/sdk/governance/evidence/live-service-llm-proof/manifest.json
    - docs/sdk/governance/evidence/live-service-llm-proof/focused.log
    - docs/sdk/governance/evidence/live-service-llm-proof/host-lib.log
    - docs/sdk/governance/evidence/live-service-llm-proof/historical-30d/echo-sdk-host/src/core_profile/facade/workflow.rs
    - docs/sdk/governance/evidence/live-service-llm-proof/historical-30d/echo-sdk-host/src/factory.rs
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
  named-settlement-family-memory:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
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
    evidence_refs:
    - evidence.sdk-memory-permission-candidate
    - evidence.sdk-task-memory-workflow-finite-observation
  named-settlement-family-subagent:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    evidence_refs:
    - evidence.sdk-task-subagent-candidate
    - evidence.sdk-stream-cancellation-routing-verification
    - evidence.sdk-task-memory-workflow-finite-observation
  named-settlement-family-task:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    evidence_refs:
    - evidence.sdk-task-subagent-candidate
    - evidence.sdk-task-context-direct-getter-consumption
    - evidence.sdk-task-memory-workflow-finite-observation
  named-settlement-family-workflow:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
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
    evidence_refs:
    - evidence.sdk-workflow-successor-candidate
    - evidence.sdk-llm-workflow-stream
    - evidence.sdk-python-typed-component-stream-verification
    - evidence.sdk-task-memory-workflow-finite-observation
  named-settlement-task-subagent:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs#claim_task
    - echo-sdk-host/src/core_profile/facade/subagent.rs#collect_result
    - echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    - evidence.sdk-model-read-retained-verification
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

## Live Agent replacement 的实际独立复审

本次补齐原 Finding 的两个具体 live authority 义务。Memory 原具名测试在同 Agent 的 Store A→B 后实际读写 B，错误 namespace 零写，旧值与计数强断言已执行。Workflow 新具名测试先由原 SessionAuthorityServices 捕获同一个 AgentHandle，再通过既有公开 set_llm_client 将 A 替换成 B，随后经真实 Host graph.build 与框架 AgentNode 执行。A 的真实 stream 请求数为 0，B 为 1；完整 ChatRequest 保留原 task 与 Unicode system/user，completed graph state 输出 B 的 Unicode 回答与原任务、唯一路径。该证据不是 pure router 的 callback0，也不是 getter 观察。

修复生产路径是原 live Agent 服务读取；workflow 新 d516 文件只增加 cfg(test)，生产 prefix 9ae6 与原修复源一致。b7e6 是该代码与正式包的真实可恢复 Git 观察，不表示早期 Memory 命令重新执行。原 30d captured client/config 源已冻结，作为旧缺陷源码证据，未造旧产品执行红。两次新 fixture 错误（缺合法 owner、读错 state JSON 路径）明确保留。focused 1、Host lib 187/0 ignored、严格 Clippy 和 fmt/check 实际通过；sdk_capability_acceptance_audit 已独立读源码、真实断言和正式包，PASS C0/I0/M0。

此处仅证明替换后新建 graph 使用当前 provider，不承诺已建 graph 中途热换，不新增 SDK setter、RPC 或跨进程持久化。Memory 与 Workflow 原具体 captured authority 缺陷三链足够本地 resolved；B 物理结算、12 身份处置、平台与 SDK main 均独立。
主 02059 审查与原 challenge 保持；本 challenge 使用后继实际 source review 和具名消费断言，不借整个 group 审查代替新 LLM 证明。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-family-memory`：有限named单元结算；证据`evidence.sdk-memory-permission-candidate`、`evidence.sdk-task-memory-workflow-finite-observation`。
- `behavior.sdk-family-subagent`：有限named单元结算；证据`evidence.sdk-task-subagent-candidate`、`evidence.sdk-stream-cancellation-routing-verification`、`evidence.sdk-task-memory-workflow-finite-observation`。
- `behavior.sdk-family-task`：有限named单元结算；证据`evidence.sdk-task-subagent-candidate`、`evidence.sdk-task-context-direct-getter-consumption`、`evidence.sdk-task-memory-workflow-finite-observation`。
- `behavior.sdk-family-workflow`：有限named单元结算；证据`evidence.sdk-workflow-successor-candidate`、`evidence.sdk-llm-workflow-stream`、`evidence.sdk-python-typed-component-stream-verification`、`evidence.sdk-task-memory-workflow-finite-observation`。
- `behavior.sdk-task-subagent`：有限named单元结算；证据`evidence.sdk-task-memory-workflow-finite-observation`、`evidence.sdk-model-read-retained-verification`。
