---
schema_version: 1
id: behavior.sdk-family-subagent
kind: behavior
status: needs_review
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
- data_durability
boundary: boundary.sdk-task-subagent
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
code_refs:
- echo-sdk-host/src/core_profile/facade/task.rs
- echo-sdk-host/src/core_profile/facade/task_runtime.rs
- echo-sdk-host/src/core_profile/facade/subagent.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-protocol/src/methods.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-task-subagent-candidate
- evidence.sdk-stream-cancellation-routing-verification
finding_refs:
- finding.sdk-task-resume-extension-invocation-conflict
---

# SDK family subagent

## 重要承诺

对已接受 `subagent` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：typed dispatch/await/control → `facade/subagent.rs`，由 framework
  `SubagentExecutor` 建立真实 attempt；extension custom agent/factory 也是其 adapter。
- **权威/状态**：framework Subagent run/result/attempt receipt；Host handle owner/generation。
  admit → spawn → publish → await/control → settled → close。短 await 不应丢结果。
- **正常/失败**：unknown attempt、重复 cancel、spawn publication quota failure、reused ID/
  generation、Session close 竞态；result/dispatch receipt 不能仅保留文本。
- **已有断言**：Core `subagent_short_await_preserves_result_and_dispatch_receipt`、
  `subagent_family_dispatches_and_controls_live_attempts`；Bridge custom/factory 消费 tests。
- **最低充分证据**：真实 Host short await 后第二 await 得完整 result，control 绑定 exact
  attempt，multi-Session isolation、quota rollback、callback failure/cancel/close；三语言
  相同 context/result/receipt round-trip，不用 framework-only 测试替代 SDK 转换。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

short await 后再次 await 仍得到完整结果与 dispatch receipt；真实 dispatch/control 触达活 attempt；同 Factory 跨 Session 并行创建不同子实例。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#subagent_short_await_preserves_result_and_dispatch_receipt`；`echo-sdk-host/tests/core_profile_e2e.rs#subagent_family_dispatches_and_controls_live_attempts`；`echo-sdk-host/tests/extension_bridge_e2e.rs#same_factory_constructs_independent_instances_across_concurrent_sessions`。

仍缺：全部 context DTO 三语言字段级 round-trip、quota rollback 与 exact attempt 竞争反例仍缺。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Four temporary shared true-Host scenarios per language are prepared/executed against6711: exact pause/cancel sibling isolation, persisted dependency result reuse, stale revision/duplicate execute, idempotent short/full await, CRUD-only preflight. They remain temporary inputs and do not prove complete context/failure/restart coverage.

正式承接增量：cb9Host上的公开正式三语言各4test已通过，来源/日志/XML按evidence.sdk-task-subagent-candidate正式proof限定；原临时输入不再被误说为未采用，但完整family仍needs_review。

## 本次有限审查单元

行为/生命周期单元：dispatch/await/control、同一幂等receipt和单collector结果。

剩余处置：四场景覆盖短await/重复await/约束；过程不可传parent_context明确拒，完整context义务按已接受字段列账。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
