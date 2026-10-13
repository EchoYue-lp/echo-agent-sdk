---
schema_version: 1
id: behavior.sdk-family-statistics
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
- evidence.sdk-computation-45-candidate
- evidence.sdk-computation-50
finding_refs: []
---

# SDK family statistics

## 重要承诺

对已接受 `statistics` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/tools.rs` → framework ExploratoryStatisticsTool，columns 与
  data_path 按 tool schema（不能假定 positional 顺序与 Rust 声明一致）。
- **权威/状态**：framework descriptive statistical computation；leaf result，不能由
  SDK 追加推断性结论或改写 accepted method semantics。
- **正常/失败**：统计样本、缺失/常量/空列、columns optional、missing file/invalid data。
- **已有断言**：Core success 且 output 含 `"inference":false`；未断言均值/计数等数值。
- **最低充分证据**：固定样本精确 descriptive values、empty/missing/constant/invalid column
  结果与错误，optional omission/null 和 float/nonfinite 边界；三语言结构与 inference=false，
  禁止把全文 substring 当作全部 statistical result 证明。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

exploratory statistics真实success且output含inference=false；未断言均值/count精确数。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#remaining_tool_families_execute_framework_tools_locally`。

仍缺：固定样本所有数值/empty/constant/nonfinite边界和三语言结果比较仍缺。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Tool validator变化限制

当前SDK direct Tool路径在旧6711以后改变，改用同一framework validator并拒绝超额arity。旧候选运行仅保留在原来源历史审计，不证明新生产路径的完整行为；需结合新增validator具体反例与最终当前门禁，状态不提升。

## 本次有限审查单元

行为/生命周期单元：指定descriptive/group/统计数值DTO与非法输入错误。

剩余处置：新7bbd/50各language50calls/19errors/9oracle支持指定数值DTO；旧64c7/45独立历史，不冒finalpin。

本项建模阈值是已接受入口、唯一权威、生命周期及具名失败可定位，直接证据或明确开放原因可复核；mapped不代表verified，不要求未定义的任意组合。
