---
schema_version: 1
id: behavior.sdk-family-a2a
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
finding_refs: []
---

# SDK family a2a

## 重要承诺

对已接受 `a2a` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/integrations.rs::dispatch` → framework `a2a::A2AClient`；task
  SSE stream 经 `facade/stream.rs`，统一 owner/window 而非第二 A2A task authority。
- **权威/状态**：远端 A2A task、framework client/protocol；Host client/stream handles。
  open/discover → send/get/cancel 或 stream → terminal/close；远端 cancel acknowledgement
  与本地 stream close 必须区分。
- **正常/失败**：task/update/artifact metadata、stream order；closed-port discovery、HTTP/
  malformed SSE、timeout/cancel/disconnect、missing terminal、quota publish rollback。
- **已有断言**：Core client handle 与 closed-port FrameworkError；Facade
  `a2a_sse_stream_delivers_items_then_terminal` 检查 item/terminal；此为 loopback fixture，
  不证明全部 accepted task cancel/recovery 或外部互操作。
- **最低充分证据**：fixture 覆盖 discover/send/get/cancel、SSE fail/no-terminal/reconnect，
  并发 next、Session close 回收；三语言逐字段保留 task/artifact/status/remote error，
  明确不支持的恢复边界，不能把本地 close 等同远端已取消。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

closed-port discovery 返回 FrameworkError；loopback SSE 产生 items 后唯一 terminal，并使用统一 stream。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#integration_families_use_framework_managers_and_clients`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#a2a_sse_stream_delivers_items_then_terminal`。

仍缺：discover/send/get/cancel 的完整远端 DTO、断线恢复和三语言同场景未覆盖，不推断远端取消。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：现accepteddiscovery/request/SSE与共享stream transport。

剩余处置：closedport/SSE具名证据；远端取消恢复具体合同只按已有API，不猜remoteeffect。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
