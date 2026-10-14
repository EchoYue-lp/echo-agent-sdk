---
schema_version: 1
id: behavior.sdk-family-invoke
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
- evidence.sdk-computation-50
finding_refs:
- finding.sdk-accepted-source-continuity
---

# SDK family invoke

## 重要承诺

对已接受 `invoke` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`_echo_agent/facade/invoke` → `facade/mod.rs` exact registry；
  `facade.resource.close` → `FacadeState::close_resource`，所有 resource/stream 共用 handles。
- **权威/状态**：catalog 仅 addressing/signature/feature authority；HandleRegistry 管
  owner/generation，底层 framework instance 决定业务 close/cancel 是否成功。
  admit → dispatch → typed value/error；closing → awaited cleanup → unregister。
- **正常/失败**：canonical/family route 一致、resource close idempotency；wrong signature/
  unknown method/feature、wrong owner/kind、fallible cleanup、重复 close、cancel/EOF。
- **已有断言**：Core admission/unknown/signature/feature、resource close/repeated false；
  Facade quota/backpressure/cascade，integrations mock close debt retry。
- **最低充分证据**：每个 resource kind publication quota rollback、失败 close保留可重试
  owner、成功 close 后 exact handle invalid 且 slot 复用；wrong same-owner kind 不误关对象。
  三语言显式 error envelope/handle、no-value rejection、Client close/cancel 不推断终态。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

真实unknown/signature/feature admission、resource配额/backpressure/cascade指定断言；family参数保留原opaque handle而非投影为enum；resourceclose失败保留owner的指定unit已执行。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#facade_resource_limits_match_the_advertised_bound`；`echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address`。

仍缺：每个resource kind publication/closefault/竞态与三语言invalidowner/kind组合仍需反例与正式pin复审。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：catalog准入、signature/feature/owner/generation与薄adapter。

剩余处置：typedHandle正常回传真实红绿有，后继callsite需独立限定审计，nullinputreachability不证明所有操作结果。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
