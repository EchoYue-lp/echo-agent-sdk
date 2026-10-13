---
schema_version: 1
id: behavior.sdk-family-improve
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
- evidence.sdk-eval-improve-topology
- evidence.sdk-factory-child-settlement-verification
finding_refs:
- finding.sdk-custom-agent-invocation-cancel-projection
- finding.sdk-factory-terminal-close-settlement
---

# SDK family improve

## 重要承诺

对已接受 `improve` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/observability.rs::dispatch_improve` 和 source improve loop/
  trajectory saver route；agent factory 反向调用来自 bridge，不在 Host 重建优化执行器。
- **权威/状态**：framework ShareGPT exporter、RunAnalyzer、source loop；Host resource/
  callback ownership。leaf export/analyze 与 stateful loop/saver 的生命周期分别验收。
- **正常/失败**：trajectory messages、critique 与 run identity；improve 编译但 eval 不在时
  analyze 应明确 FeatureUnavailable；factory failure/cancel/timeout/save I/O 不能默认成功。
- **已有断言**：Core ShareGPT messages 为数组的形状断言、critique run_id/success；Bridge factory lazy invocation。
- **最低充分证据**：真实 source loop/result/stop/cancel 和 trajectory save/reopen，保留
  critique 指标/失败原因、factory instance owner cleanup；独立 feature build 下 analyze
  缺 eval 的错误 detail；三语言同一 run 分析结果，不能靠 loop registration 即认定已执行。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

ShareGPT messages 为数组的形状、critique run_id/success、factory lazy invocation 指定断言实际执行；未验证导出内容逐项正确。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#eval_and_improve_families_use_the_framework_analyzers`；`echo-sdk-host/tests/extension_bridge_e2e.rs#eval_and_improve_source_adapters_invoke_agent_factory_lazily`。

仍缺：source loop stop/cancel、trajectory save/reopen、缺 eval feature 的 standalone 行为矩阵未验证。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：已接受trajectory/critic/eval组合与原ShareGPT导出字段。

剩余处置：现有输出shape不替内容逐字段正确；明确acceptedloop/stop/save消费者剩余证据。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
