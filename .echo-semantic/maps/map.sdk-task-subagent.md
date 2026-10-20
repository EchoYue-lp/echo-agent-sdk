---
schema_version: 1
id: map.sdk-task-subagent
kind: capability_map
title: Task 与 Subagent
risk: high
observed_at: 02059dd3014633a23c188e30e73b0634a8c5621d
boundary_refs:
- boundary.sdk-task-subagent
behavior_refs:
- behavior.sdk-task-subagent
- behavior.sdk-family-task
- behavior.sdk-family-subagent
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-stream-cancellation-routing-verification
- evidence.sdk-task-memory-workflow-finite-observation
finding_refs:
- finding.sdk-subagent-await-result
- finding.sdk-task-control-locality
- finding.sdk-task-runtime-store-authority
- finding.sdk-task-resume-extension-invocation-conflict
audit_refs:
- audit.sdk-task-memory-workflow-finite-trigger-input
- audit.sdk-task-memory-workflow-finite-result-side-effect
- audit.sdk-task-memory-workflow-finite-state-authority
- audit.sdk-task-memory-workflow-finite-data-durability
- audit.sdk-task-memory-workflow-finite-time-lifecycle
- audit.sdk-task-memory-workflow-finite-failure-concurrency
- audit.sdk-task-memory-workflow-finite-permission-external
- audit.sdk-task-memory-workflow-finite-contract-evidence
related_map_refs:
- map.sdk-product-boundary
scenarios:
  behavior-lifecycle-failure:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs#claim_task
    - echo-sdk-host/src/core_profile/facade/subagent.rs#collect_result
    - echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues
    behavior_refs:
    - behavior.sdk-task-subagent
    finding_refs:
    - finding.sdk-subagent-await-result
    - finding.sdk-task-control-locality
    - finding.sdk-task-runtime-store-authority
    review_scope: Entry, unique authority, normal lifecycle and named failure obligations
      have been inspected and linked. Mapped records the model, not bug-free/runtime/main
      closure; open Findings remain explicit.
  family-task:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    behavior_refs:
    - behavior.sdk-family-task
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-task-subagent-candidate
    - evidence.sdk-task-memory-workflow-finite-observation
    review_scope: Accepted family lifecycle, exact CAS/claim/result and named failures
      examined; knownphysicalB and12sourceidentity migration remain explicit.
  family-subagent:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    behavior_refs:
    - behavior.sdk-family-subagent
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-task-subagent-candidate
    - evidence.sdk-task-memory-workflow-finite-observation
    review_scope: Accepted family lifecycle, exact CAS/claim/result and named failures
      examined; knownphysicalB and12sourceidentity migration remain explicit.
  task-memory-workflow-finite-trigger-input:
    status: mapped
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
    behavior_refs:
    - behavior.sdk-task-subagent
    - behavior.sdk-family-task
    - behavior.sdk-family-subagent
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    audit_refs:
    - audit.sdk-task-memory-workflow-finite-trigger-input
    review_scope: Original69b taskauthority source restored02059; actual author finite8lens,
      currentA excluded; original790 consumerproof and4getter scope retained.
  task-memory-workflow-finite-result-side-effect:
    status: mapped
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
    behavior_refs:
    - behavior.sdk-task-subagent
    - behavior.sdk-family-task
    - behavior.sdk-family-subagent
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    audit_refs:
    - audit.sdk-task-memory-workflow-finite-result-side-effect
    review_scope: Original69b taskauthority source restored02059; actual author finite8lens,
      currentA excluded; original790 consumerproof and4getter scope retained.
  task-memory-workflow-finite-state-authority:
    status: mapped
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
    behavior_refs:
    - behavior.sdk-task-subagent
    - behavior.sdk-family-task
    - behavior.sdk-family-subagent
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    audit_refs:
    - audit.sdk-task-memory-workflow-finite-state-authority
    review_scope: Original69b taskauthority source restored02059; actual author finite8lens,
      currentA excluded; original790 consumerproof and4getter scope retained.
  task-memory-workflow-finite-data-durability:
    status: mapped
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
    behavior_refs:
    - behavior.sdk-task-subagent
    - behavior.sdk-family-task
    - behavior.sdk-family-subagent
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    audit_refs:
    - audit.sdk-task-memory-workflow-finite-data-durability
    review_scope: Original69b taskauthority source restored02059; actual author finite8lens,
      currentA excluded; original790 consumerproof and4getter scope retained.
  task-memory-workflow-finite-time-lifecycle:
    status: mapped
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
    behavior_refs:
    - behavior.sdk-task-subagent
    - behavior.sdk-family-task
    - behavior.sdk-family-subagent
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    audit_refs:
    - audit.sdk-task-memory-workflow-finite-time-lifecycle
    review_scope: Original69b taskauthority source restored02059; actual author finite8lens,
      currentA excluded; original790 consumerproof and4getter scope retained.
  task-memory-workflow-finite-failure-concurrency:
    status: mapped
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
    behavior_refs:
    - behavior.sdk-task-subagent
    - behavior.sdk-family-task
    - behavior.sdk-family-subagent
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    audit_refs:
    - audit.sdk-task-memory-workflow-finite-failure-concurrency
    review_scope: Original69b taskauthority source restored02059; actual author finite8lens,
      currentA excluded; original790 consumerproof and4getter scope retained.
  task-memory-workflow-finite-permission-external:
    status: mapped
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
    behavior_refs:
    - behavior.sdk-task-subagent
    - behavior.sdk-family-task
    - behavior.sdk-family-subagent
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    audit_refs:
    - audit.sdk-task-memory-workflow-finite-permission-external
    review_scope: Original69b taskauthority source restored02059; actual author finite8lens,
      currentA excluded; original790 consumerproof and4getter scope retained.
  task-memory-workflow-finite-contract-evidence:
    status: mapped
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
    behavior_refs:
    - behavior.sdk-task-subagent
    - behavior.sdk-family-task
    - behavior.sdk-family-subagent
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-task-memory-workflow-finite-observation
    audit_refs:
    - audit.sdk-task-memory-workflow-finite-contract-evidence
    review_scope: Original69b taskauthority source restored02059; actual author finite8lens,
      currentA excluded; original790 consumerproof and4getter scope retained.
---

# Task 与 Subagent

## 能力范围

factory中的同一个TaskRevisionService供应CRUD及FacadeTaskController；controller仅转换metadata、claim/dispatch并调用framework服务，不拥有frontier或第二DAG validator。execution CAS preflight在副作用前拒CRUD-only后端；exact pause/cancel绑定canonical attempt，独立sibling继续，Completed result含absent/null区别。Subagent单collector保留join结果，短await超时后可再次await，不提前耗尽句柄；runtime失败经task/list保留origin_operation/cause。Hostcore31/bridge34中这些具名场景已执行。

## 入口与输出

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 行为关系

本能力经 official ACP/profile 或具体 catalog route 到 Host adapter，然后调用框架权威。跨能力调用与三语言消费见 docs/sdk/governance/capability-map.md。

## 状态与数据流

Framework 拥有 Run/Task/Store 的事实与终态；Host 只拥有寻址、可取消资源 owner 与协议发送状态；客户端确认已消费事实。

## 策略来源与优先级

用户八项治理目标与 AGENTS.md 约束行为；accepted external contract 约束兼容；compiled feature/canonical framework service 约束实现。完整 Rust inventory 不扩承诺。

## 生命周期与失败路径

factory中的同一个TaskRevisionService供应CRUD及FacadeTaskController；controller仅转换metadata、claim/dispatch并调用framework服务，不拥有frontier或第二DAG validator。execution CAS preflight在副作用前拒CRUD-only后端；exact pause/cancel绑定canonical attempt，独立sibling继续，Completed result含absent/null区别。Subagent单collector保留join结果，短await超时后可再次await，不提前耗尽句柄；runtime失败经task/list保留origin_operation/cause。Hostcore31/bridge34中这些具名场景已执行。

原569阶段缺口（历史，当前有限范围见后继段）：正式三语言各四项DAG/control/短await场景已有证据；canonical result/执行次数增强与callback结算后继源独立归档。剩余已接受claim/context字段和framework parent-drop修复按各自具名义务对账，不扩大Task API。

## 权限与敏感信息

本地 SDK 不引入线上权限门控。继续保持框架自动行为 permission authority，凭据不进入日志/合同/错误；资源 gate 仅防止错误owner和无意数据丢失。

## 用户侧投影

TypeScript/Python/Java 对已接受能力暴露惯用接口，handle/error/value 必须无损；不能用成功默认值或丢字段改变权威事实。

## 场景处置清单

主场景mapped只表示模型已展开；关联family Behavior仍needs_review，具体Evidence按支持关系与原epoch读取。剩余义务：正式三语言各四项DAG/control/短await场景已有证据；canonical result/执行次数增强与callback结算后继源独立归档。剩余已接受claim/context字段和framework parent-drop修复按各自具名义务对账，不扩大Task API。

## 未展开项

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch与95ecpin保持原运行epoch；当前Host已pin published b909并移除patch，但最终nightlyprofiles/唯一合同生成与SDKwhole源码门禁仍待。frameworkmain交付证据不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

正式三语言各四项DAG/control/短await场景已有证据；canonical result/执行次数增强与callback结算后继源独立归档。剩余已接受claim/context字段和framework parent-drop修复按各自具名义务对账，不扩大Task API。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，family needs_review和开放Finding保留，主场景mapped不扩运行证据。

## 建模完成与验证状态区别

组级主路径已按入口、唯一权威、生命周期和具名失败义务完成映射，故主场景mapped。family Behavior仍needs_review，已知open Finding与有限证据保留；mapped不等于verified、所有lens covered或最终交付。剩余事项采用明确合同/故障原因，不以无限未枚举组合拒绝建模。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前Task/Memory/Workflow具名审查

当前观测恢复真实02059，所选TaskRuntime/Subagent/memory/workflow/selector/sourceoperations等10文件与原审69b逐blob一致；currentfactory/coretests因意图A变化，原源定位不冒新A被审。CLI原实际8lens作者sourceaudit有限examined、无新增已证实缺陷，后经本材料独立质控；作者身份与独立review角色不混同。新sharedEvidence和8窄Audit按四acceptedfamily与原唯一authority明确scope，没有near-synonym contextfamily。

原正式a634/790whole TaskJ4/WFcheckpointJ1/MemoryJ1/LLMWFJ3及三语强断言支持exactCAS、canonicaldeps、短await不耗结果、liveStore、checkpoint/claimack与streamcancel具名路径。Taskcontext四Host getter消费为runid/waiver/nonwaivedmissingresult/nondefaultpolicy；三nativeCallback仅runid来自SessionACP，不宣完整3fieldreverseDTO。ContextInheritance纯值不是livehistory/memory继承E2E；compressor真实注册消费不推全部三nativehook重入取消，这一具体支持性缺口保留。

Behavior保持needs_review/场景mapped只表有限审查闭包。内存Store/checkpoint无额外crossprocessdurability保证，5秒physicalB/wideTaskFinding仍open、12sourceidentity仍待人，当前A已有批准但其实现后原生/全gate另切点。scope外未接受sourceoperation/构造器不纳新增义务，也不以‘所有组合未穷尽’否定已审能力路径。
