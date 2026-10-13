---
schema_version: 1
id: behavior.sdk-family-state
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
- evidence.sdk-managed-retirement
- evidence.sdk-managed-supporting-operation-boundary
finding_refs: []
---

# SDK family state

## 重要承诺

对已接受 `state` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/state_delivery.rs::dispatch_state` 直接使用 Host shared
  framework `FileRuntimeStateStore`；Agent/Session runtime 保存应访问同一 service。
- **权威/状态**：framework runtime/checkpoint store；Host 不维护第二 checkpoint JSON。
  save → get/list → runtime/scope clear，commit/identity/generation 保留。
- **正常/失败**：完整 checkpoint save/get、一致 scope clear；invalid scope/revision、
  partial write/corrupt file、并发 save/clear、重启/outcome unknown 不能吞错。
- **已有断言**：Core save ok/list 一个 runtime/get/clear 与 unknown checkpoint operation；
  Bridge managed persistence tests 和 Wire field tests 是其它指定边界，不等同文件路径证明。
- **最低充分证据**：真实 Run checkpoint 与 facade read 同一字段及 journal identity，
  clear scope/重启隔离、file fault 与 atomicity，三语言 accepted checkpoints/commit receipts
  大 revision/subsecond/optional round-trip；不能只断言 save 的 `ok=true`。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

Host checkpoint facade 的 save/list/get/clear 指定断言；真实 managed 事务在 commit 后断连/重启保持 pending projection、AlreadyApplied 复用同 receipt，三语言比较副作用 ordinal 不重复。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#state_delivery_and_trace_families_use_framework_services`；`echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt`；`sdks/typescript/test/managed-host.test.js`。

仍缺：文件后端所有字段、scope clear/I/O fault/atomicity 未由 managed backend 证据替代；11 managed typed calls 不等于全部实际操作。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## State/Trace/Delivery后继候选

三语言各三场景在cb9执行，九个raw结果/指定DTO及侧作用已对账；对应部分见evidence.sdk-state-trace-delivery-candidate。原执行输入保留，新required Host空白前置条件diff明确，不将内存Delivery恢复当跨进程durable证明，也不外推全局clear或所有Trace事件。状态仍needs_review。

## 本次有限审查单元

行为/生命周期单元：FileRuntimeStateStore checkpoint/scopes/owner索引与cold恢复。

剩余处置：三语言9scene中state部分支持scope隔离/冷恢复；clear不宣全局all-or-none。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
