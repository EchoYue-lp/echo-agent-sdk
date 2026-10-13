---
schema_version: 1
id: behavior.sdk-family-trace
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

# SDK family trace

## 重要承诺

对已接受 `trace` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/observability.rs::dispatch_trace` 创建 framework RunStore；
  `source_operations.rs` 的 trace resource verbs 复用实例。
- **权威/状态**：framework `trace::RunStore`，memory 或 `JsonlRunStore`；Host store handle。
  open → save/load/list → close；list page/truncation 和 absent run 是合同结果。
- **正常/失败**：完整 Run/RunSummary round-trip，unknown mode、missing/invalid path、quota、
  JSONL write/corruption、Session close；未发布 store 不应耗用 slot。
- **已有断言**：Core trace memory save/load/list_recent；adapter
  `failed_trace_open_does_not_consume_the_only_resource_slot` 断言非法 open 后可复用配额。
- **最低充分证据**：JSONL path、persist/reopen、full Run metadata/events/usage 字段，
  malformed run、I/O fault、retention/page boundaries、quota failure 后资源计数、close retry；
  live framework collector 与 SDK store 是否同实例须按 route 明确，不假定所有 trace 自动写入。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

内存 trace save/load/list_recent；非法构造先校验后发布，不占唯一 resource slot，后续合法 open 成功。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#state_delivery_and_trace_families_use_framework_services`；`echo-sdk-host/src/core_profile/facade/observability.rs#failed_trace_open_does_not_consume_the_only_resource_slot`。

仍缺：JSONL reopen/retention/page/full events/usage 和 I/O fault 缺行为证据；不假定 facade trace 自动绑定 live collector。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## State/Trace/Delivery后继候选

三语言各三场景在cb9执行，九个raw结果/指定DTO及侧作用已对账；对应部分见evidence.sdk-state-trace-delivery-candidate。原执行输入保留，新required Host空白前置条件diff明确，不将内存Delivery恢复当跨进程durable证明，也不外推全局clear或所有Trace事件。状态仍needs_review。

## 本次有限审查单元

行为/生命周期单元：显式collector/fileJSONL、完整指定events/usage与sanitizer。

剩余处置：三语言JSONL原字节cold读取；未声明所有secret自动redaction或其它backend完整保留合同。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
