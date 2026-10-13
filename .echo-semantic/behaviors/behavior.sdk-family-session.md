---
schema_version: 1
id: behavior.sdk-family-session
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
boundary: boundary.sdk-agent-session-run
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
code_refs:
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/state.rs
- echo-sdk-host/src/core_profile/persistence.rs
- echo-sdk-host/src/factory.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
finding_refs: []
---

# SDK family session

## 重要承诺

对已接受 `session` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：typed `session/create|load|close` → `handler.rs`、`factory.rs`；标准 ACP
  Session 与扩展 handle 通过同一 connection services 桥接，见 Core standard-prompt 测试。
- **权威/状态**：framework live Session Agent、conversation/checkpoint store；Host Session
  handle/recovery mapping。create/load → live → closing → cleanup settled → release；重启新 generation。
- **正常/失败**：cwd、history、memory/task/services 绑定准确 Session；未知/跨 owner Session、
  load 损坏/缺失持久化、create publication 失败、close 超时/重试/并发不能返回假成功。
- **已有断言**：Core `extended_standard_prompt_bridges_shared_core_handles` 与 restart
  tests；三语言 protocol-host 两 Session ID 不同，关闭 one 后 two 的 Agent name 调用成功。
- **最低充分证据**：相同 Agent 下两个 Session 写 memory/执行 Task、关闭其中一个、重启
  load history；比对隔离状态、旧 generation rejection、close debt retry 和资源配额复用。
  Agent name 成功只证明 sibling Session 仍可访问，不证明所有核心状态隔离。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

标准 prompt 与扩展 Session 共享对象；历史 native turn7 后 create/load 都精确分配 turn8；畸形身份报 create/load 实际 operation，重复失败无句柄配额泄漏。禁用 reservation 的真实反例两路径均产生 turn1。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles`；`echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run`；`echo-sdk-host/tests/core_profile_e2e.rs#session_load_reserves_persisted_turn_identity_before_new_run`；`echo-sdk-host/tests/core_profile_e2e.rs#malformed_recovered_identity_rejects_actual_method_without_handle_publication`。

仍缺：两 Session 的全部 memory/Task 状态隔离、持久化 I/O fault 和 close debt 的三语言共同场景仍未穷尽。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：create/load/标准prompt共用framework Session、原生turn恢复identity与同owner清理。

剩余处置：持久化错误和Sessionclosing回调重入的新明确窗口；不外推任意resource组合。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
