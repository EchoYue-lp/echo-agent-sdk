---
schema_version: 1
id: behavior.sdk-family-standard
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
boundary: boundary.sdk-acp
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
code_refs:
- echo-sdk-host/src/lib.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-host/src/features.rs
- echo-sdk-host/src/core_profile/mod.rs
- echo-sdk-host/src/core_profile/state.rs
- echo-sdk-protocol/src/capability.rs
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-candidate-behavior-verification
- evidence.sdk-parent-driver-candidate-verification
finding_refs: []
---

# SDK family standard

## 重要承诺

对已接受 `standard` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：official ACP v1 methods → `run_stdio`/framework `AcpAgentAdapter`，
  可无 Echo profile 使用；不靠 accepted facade catalog 将标准方法重新封装。
- **权威/状态**：framework standard Session/prompt/cancel，official schema 定义协商及
  wire shape；连接 initialize → Session → prompt/update → stop/cancel → EOF。
- **正常/失败**：标准 prompt/update 与扩展 Run 共享事实；版本/非法 params/无 profile、
  EOF、配置准备失败、stdout 污染不能被扩展成功用例遮盖。
- **已有断言**：Stdio standard prompt、parked-model cancel、EOF no noise、invalid config
  no secret；Protocol ACP baseline 检查 v1 constants、lock/checksum、unstable exclusions。
- **最低充分证据**：最终 binary 上普通 ACP client 无 profile 的正常 prompt/cancel/EOF，
  与 extended standard prompt 共享 Session/Run 比对；初始化错误、unsupported version
  和 startup cleanup 的真实行为。baseline constants 不是协议执行证据。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

extended standard session/prompt 的 end_turn、session/update 与 Session identity 同步；三语言真实 Host smoke/examples 亦执行标准 prompt。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles`；`sdks/typescript/test/host.test.js`。

仍缺：无 profile 的独立 stdio_e2e suite 不在此 Host136/31/34/12 收据内；unsupported version/startup cleanup 与 plain ACP cancel/EOF 最终运行仍缺。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：标准ACP initialize/session/prompt/cancel/EOF与可选扩展保持角色分离。

剩余处置：标准stdio4有实际证据；最终当前pin完整stdio/Profile测试与平台signal待交付。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
