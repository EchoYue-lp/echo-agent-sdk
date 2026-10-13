---
schema_version: 1
id: behavior.sdk-family-source-operation
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
finding_refs: []
---

# SDK family source_operation

## 重要承诺

对已接受 `source_operation` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/registry.rs` exact accepted identity/digest →
  `_echo_agent/facade/invoke` → `facade/source_operations.rs::dispatch` 或同一 family adapter。
  ReactAgent/Agent service、SkillRegistry、file/tokenizer/guard/store 等由 receiver 调用。
- **权威/状态**：所调用 framework 对象及上述各 family 权威；Host resource只保存被
  寻址实例。source setter/getter 必须作用于当前 live Agent，而非过时 definition clone。
- **正常/失败**：exact receiver/arguments、live prompt/skills/services、constructor resource
  字段、optional、same-owner wrong-kind、bad signature/identity/feature；不能字符串猜 route。
- **已有断言**：Core admission live prompt/max_iterations/skills/permission/services、
  stateful resource/file authority/turn receipt；canonical sentinel 仅 reachability。
- **最低充分证据**：accepted route 按产品能力归组对齐本矩阵，逐 stateful setter→live
  consumer→getter/side effect、constructor→合法 method→close；所有转换字段 round-trip，
  非法 receiver/signature 无 mutation。source route 总数不成为项目完成分母。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

指定liveAgent setter/getter、resource方法/close、文件权威和receipt投影实际执行；全canonical null-input哨兵只支持adapter reachability。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`；`echo-sdk-host/tests/core_profile_e2e.rs#stateful_source_resources_preserve_owner_state_and_close`；`echo-sdk-host/tests/core_profile_e2e.rs#source_files_and_skill_registry_preserve_rust_authority`；`echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered`。

仍缺：accepted setter→真实consumer、所有sourceconstructor→合法method→close及各字段无损仍需逐能力归组，不以identity数量完成。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：exactaccepted definingidentity/signature→现framework服务/资源。

剩余处置：不是每Rustpubidentity三语言；逐能力sourceconstruct/method/close的明确义务由菜单/Findings跟踪。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
