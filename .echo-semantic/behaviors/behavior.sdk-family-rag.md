---
schema_version: 1
id: behavior.sdk-family-rag
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
- evidence.sdk-rag-project-rules
- evidence.sdk-rag-scalar-verification
finding_refs:
- finding.sdk-rag-unicode-character-count
---

# SDK family rag

## 重要承诺

对已接受 `rag` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/tools.rs::RagToolSet` 为每 Session 创建 framework RagStore/
  HttpEmbedder；index/search 共用 set，chunk tool 是独立 leaf。
- **权威/状态**：framework vector store/embedder；Host 只拥有 Session instance reference。
  configure → chunk/index → search → Session close drop，不能每次调用新建空 store。
- **正常/失败**：index 后 search 命中、Session isolation；缺 key/invalid config、embedding
  provider fault、cancel/large document/close，明确 FeatureUnavailable/InvalidConfig。
- **已有断言**：Core chunk success；adapter unit 使用 TestEmbedder index/search 命中，
  第二 RagToolSet 不命中。TestEmbedder 不证明真实 Host HttpEmbedder 配置与网络路径。
- **最低充分证据**：loopback embedding provider 真实 Host index→search、两 Session
  isolation、missing env/bad endpoint/provider error/cancel，无半提交索引；三语言命中
  score/source/chunk metadata 与字符边界、清理和 typed error，不能以 chunk 成功覆盖 index。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

真实 Host chunk工具success；TestEmbedder unit中同一Session工具集index/search命中、另一工具集不命中。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#remaining_tool_families_execute_framework_tools_locally`；`echo-sdk-host/src/core_profile/facade/tools.rs#rag_index_and_search_share_one_session_store`。

仍缺：真实 Host HttpEmbedder网络路径/索引半提交/错误取消与三语言score/metadata缺口；TestEmbedder不是网络证据。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## Tool validator变化限制

当前SDK direct Tool路径在旧6711以后改变，改用同一framework validator并拒绝超额arity。旧候选运行仅保留在原来源历史审计，不证明新生产路径的完整行为；需结合新增validator具体反例与最终当前门禁，状态不提升。

## 本次有限审查单元

行为/生命周期单元：同Sessionindex/search共享store/embedding与hitmetadata。

剩余处置：TestEmbedder仅unit无网络保证；明确HttpEmbedder公开接受路径消费者证据缺。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
