---
schema_version: 1
id: behavior.sdk-family-eval
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
boundary: boundary.sdk-integrations
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
code_refs:
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/registry.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/memory.rs
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/src/core_profile/facade/state_delivery.rs
- echo-sdk-host/src/core_profile/facade/tools.rs
- echo-sdk-host/src/core_profile/facade/observability.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
- evidence.sdk-eval-improve-topology
- evidence.sdk-factory-child-settlement-verification
finding_refs:
- finding.sdk-custom-agent-invocation-cancel-projection
- finding.sdk-factory-terminal-close-settlement
---

# SDK family eval

## 重要承诺

对已接受 `eval` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/observability.rs::dispatch_eval` 的 constraints/report；accepted
  source runner/grader route 在 `source_operations.rs`，通过 extension factory 真正消费 agent。
- **权威/状态**：framework eval constraint evaluator/report/runner/grader；Host runner
  resource 和 factory address。report leaf 无长期状态；runner 才有 invocation 生命周期。
- **正常/失败**：criteria results/counts、factory lazy create、per-case outcomes；malformed
  run/result、grader error、deadline/cancel、factory release/quota/feature disabled。
- **已有断言**：Core constraints 与 report total=2/passed=1/failed=1、unknown op；Bridge
  eval/improve source factory lazy invocation。报表计数不证明 runner 取消与清理。
- **最低充分证据**：所有 accepted runner/grader verbs 的真实输入及 result fields，factory
  instance 隔离，one failing grader/timeout/disconnect 后 cleanup 和 case outcome，不丢
  metric/error；三语言 DTO/feature error 一致，禁止把仅统计 report 当作 runner 覆盖。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

report total2/pass1/fail1 与 constraints；真实 EvalRunner lazy factory 子实例消费已执行，factory failure 在 case 结果保留 canonical cause；重型 Factory 两场景不再 stack overflow。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#eval_and_improve_families_use_the_framework_analyzers`；`echo-sdk-host/tests/extension_bridge_e2e.rs#eval_and_improve_source_adapters_invoke_agent_factory_lazily`；`echo-sdk-host/src/core_profile/facade/source_operations.rs#factory_failure_retains_canonical_cause_in_real_eval_result`。

仍缺：全部 grader/metric/error 字段、timeout/disconnect/cancel 的 runner cleanup 尚未穷尽。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：真实runner案例/criterion/grader/factory lazy实例与错误cause。

剩余处置：当前case report/factory路径已证；具体accepted超时/取消owner与remaining字段需精确复审，非全部任意metric。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
