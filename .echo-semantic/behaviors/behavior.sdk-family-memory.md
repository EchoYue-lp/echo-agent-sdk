---
schema_version: 1
id: behavior.sdk-family-memory
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
- evidence.sdk-memory-permission-candidate
finding_refs: []
---

# SDK family memory

## 重要承诺

对已接受 `memory` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/memory.rs::dispatch`，memory.store 路径取 live Session Agent
  Store；resource 路径从 `source_operations.rs::MemoryStoreAuthority` 及统一 registry 取实例。
- **权威/状态**：framework `memory::Store` 及其 namespace/provenance/expiry；Host 只
  保存 resource owner。put/get/search/list/delete；resource open → consume → close。
- **正常/失败**：put 后真实 agent/tool recall 同一数据；namespace、search mode、page/
  truncated、expiry/dedup；file I/O 或 extension Store failure 应传播，不能假 put 成功。
- **已有断言**：Core admission test put/get/delete/unknown；stateful-source resource get
  key/value、cross-owner/close；adapter unit 覆盖部分 search/resource 操作。未证明真实
  Session agent recall 与外部 resource store 相同，更未覆盖全部 provenance/expiry。
- **最低充分证据**：真实 Host memory.store 写后由 live Agent memory tool 读取；多 Session
  namespace isolation，resource search_with/prune/dedup 与显式 expiry/provenance；file/managed
  Store fault、publication/close 重试，三语言完整 item 字段和 large timestamps round-trip。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

真实 family put/get/delete；adapter 反例证明 mixed namespace 不写入、替换 Store 后读当前 live authority，unsupported semantic search 保留 Memory cause 且原值不变；三语言 smoke 实际 put/get。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`；`echo-sdk-host/src/core_profile/facade/memory.rs#malformed_namespace_never_writes_and_replacement_store_is_live_authority`；`echo-sdk-host/src/core_profile/facade/memory.rs#unsupported_store_operation_keeps_memory_cause_without_mutation`。

仍缺：recall tool 与写入同实例、provenance/expiry/search/prune/dedup 与多 Session 隔离仍缺充分 Host 行为。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Memory/Permission后继消费者范围

三语言临时原生consumer各59证据记录（52operation结果/2typed失败/11Store回调及model/journal/真正Run副作用）已有共同对照；不是59独立test。正式TS/Python单场景执行与临时Java原来源分开归档。30分钟TTL过期和未接受approval_receipt仍明确缺口，不造短TTL API，状态不提升。

## 本次有限审查单元

行为/生命周期单元：live Store权威、namespace/CRUD/search/prune/dedup与Active recall。

剩余处置：正式59rows支持Store切换/recall/真实effect；没有任意数值nativeStore默认类型保证。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
