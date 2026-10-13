---
schema_version: 1
id: behavior.sdk-family-delivery
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
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
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
- evidence.sdk-state-trace-delivery-candidate
finding_refs: []
---

# SDK family delivery

## 重要承诺

对已接受 `delivery` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/state_delivery.rs::dispatch_delivery`，ledger.open 后 resource
  存储真实 framework `DeliveryLedger`；enqueue/claim/transition/defer/settle/recover 操作它。
- **权威/状态**：每个 ledger 的 framework journal/checkpoint 与 exact delivery claim；
  Host 只标 owner/address。pending → claimed → dispatched/deferred/settled，revision/CAS
  必须携带，不以 delivery ID 单独代替 claim。
- **正常/失败**：同 claim 转移、recovery snapshot；stale/wrong claim、duplicate settle、
  concurrent claim、journal/commit failure/outcome unknown、close 不能返回虚构成功。
- **已有断言**：Core state-delivery test open/enqueue/claim/transition/settle/snapshot/recover；
  是 memory ledger 路径，不自动证明 durable crash recovery 或 managed CAS。
- **最低充分证据**：并发 claim 唯一、旧 claim transition rejection 且 snapshot unchanged，
  defer/recover/settle 字段及副作用 exact，真实 Host 三语言 CAS/receipt round-trip；durable
  journal fault/lost ACK/restart 如属 accepted route 必须证明，不从 memory test 推导。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

内存 ledger open/enqueue/claim/transition/settle/recover/snapshot；完整 claim 转换拒绝 id-only/旧 attempt，不重建当前 claim 冒充旧调用。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#state_delivery_and_trace_families_use_framework_services`；`echo-sdk-host/src/core_profile/facade/state_delivery.rs#exact_claim_roundtrip_rejects_delayed_attempt_and_id_only`。

仍缺：durable crash/lost ACK、并发 claim 唯一和三语言完整 CAS receipt 对账仍缺；内存路径不是 durable delivery 证明。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## State/Trace/Delivery后继候选

三语言各三场景在cb9执行，九个raw结果/指定DTO及侧作用已对账；对应部分见evidence.sdk-state-trace-delivery-candidate。原执行输入保留，新required Host空白前置条件diff明确，不将内存Delivery恢复当跨进程durable证明，也不外推全局clear或所有Trace事件。状态仍needs_review。

## 本次有限审查单元

行为/生命周期单元：完整exactclaim与唯一activeattempt/effect/settlement。

剩余处置：三语言内存ledger对账支持；该acceptedopen是memory，不追加未承诺的跨进程durability门槛。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
