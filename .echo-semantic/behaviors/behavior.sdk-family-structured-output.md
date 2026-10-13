---
schema_version: 1
id: behavior.sdk-family-structured-output
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
- evidence.sdk-computation-45-candidate
- evidence.sdk-computation-50
finding_refs: []
---

# SDK family structured_output

## 重要承诺

对已接受 `structured_output` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/structured_output.rs::validate` 由 dedicated method/accepted
  route 调用；此路径只检验 schema/sample，不执行 Agent。
- **权威/状态**：Host使用jsonschema validator判定schema/sample；当前AgentConfigWire/AgentCreate
  合同没有output schema/repair配置。accepted仅validate，不承诺Run structured输出或repair policy。
- **正常/失败**：schema compile、sample valid/errors（有界）、absent sample、too many
  args、schema byte bound、invalid wire；valid schema 不等于 Agent 最终输出合格。
- **已有断言**：Core admission 中 valid sample 的 valid=true、invalid request typed error；
  未完整证明schema over-limit/error paths。
- **最低充分证据**：false sample 的 exact errors/path、schema compile failure、payload
  bound、optional sample区别；三语言原样保留validate结果与错误。Run最终输出/repair不属于
  此已接受合同，不能作为新增验收目标。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

合法sample valid=true和非法请求typederror在真实Host执行。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed`。

仍缺：false样本errors/path、schema超限/compilefailure及三语言validate结果缺口；无Run repair承诺。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：已接受schema/value validate与明确错误path。

剩余处置：已接受validate的合法/非法schema、missing/null和payload/error限额由50共同语料证明；不新增Agent repair配置，最终accepted输出字段生成需匹配。

本项建模阈值是已接受入口、唯一权威、生命周期及具名失败可定位，直接证据或明确开放原因可复核；mapped不代表verified，不要求未定义的任意组合。
