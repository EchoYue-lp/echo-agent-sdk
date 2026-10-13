---
schema_version: 1
id: behavior.sdk-family-chart
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
- evidence.sdk-tool-behavior-candidate
- evidence.sdk-tool-input-validation-candidate
- evidence.sdk-computation-45-candidate
- evidence.sdk-computation-50
finding_refs:
- finding.sdk-tool-input-validator-bypass
---

# SDK family chart

## 重要承诺

对已接受 `chart` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/tools.rs` → framework GenerateChartTool，输入按 tool schema 映射。
- **权威/状态**：framework chart spec/result/artifact 与必要输出文件；leaf 调用不拥有
  长期 Run。validate data/options → generate → effect/artifact result。
- **正常/失败**：accepted chart types、optional color、Unicode/title、data values；invalid
  type/field/schema、oversized data/artifact write fault，不能返回默认空 spec。
- **已有断言**：bar完整normalized Vega-Lite schema/mark/title/data/encoding/width/height，中文emoji和color=null omission；missing data返回framework_error及完整canonical detail。
- **最低充分证据**：精确 spec schema/type/title/fields/data，合法 optional omission 与
  null，失败 code/full ToolResult，artifact 引用若存在则核对 bytes/path；三语言结构一致，
  明确 spec generation 与 raster rendering 的证据边界。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

bar完整normalized Vega-Lite schema/mark/title/data/encoding/width/height，中文emoji和color=null omission；missing data返回framework_error及完整canonical detail。

实际来源：`evidence.sdk-tool-behavior-candidate`，绑定当前仍相同的具名测试文件与实测Host4d436；
28次工具调用+1Session close、10typed RPC error及database success=false拒写分别核对。
reported effects empty只表示已报告事实，实际文件/ref变更另行验证。GET-only记录不证明
无POST，invalid branch没有repo A全部refs前后断言。

仍缺：其它accepted chart type、invalid type/field/large数据、HTML/raster与artifact write fault仍缺；本组仅spec生成。 正式pin、最后变更后的独立复审与精确main仍待。
归档观察当前源摘要不重绑定旧whole运行；不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Tool validator变化限制

当前SDK direct Tool路径在旧6711以后改变，改用同一framework validator并拒绝超额arity。旧候选运行仅保留在原来源历史审计，不证明新生产路径的完整行为；需结合新增validator具体反例与最终当前门禁，状态不提升。

## 本次有限审查单元

行为/生命周期单元：Vega-Lite schema/value/label/UTF8和framework唯一inputvalidator。

剩余处置：旧64c7/45保历史；新7bbd/50及独立limitedpass覆盖指定chart value与失败cause，文件effects不等files API。新Toolvalidator/customTool执行计数源按自己的units/复审限定，最终生成/全门禁另验。

本项建模阈值是已接受入口、唯一权威、生命周期及具名失败可定位，直接证据或明确开放原因可复核；mapped不代表verified，不要求未定义的任意组合。
