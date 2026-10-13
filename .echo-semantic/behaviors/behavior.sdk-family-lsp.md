---
schema_version: 1
id: behavior.sdk-family-lsp
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

# SDK family lsp

## 重要承诺

对已接受 `lsp` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/integrations.rs::dispatch` → framework `LspManager`/
  `StdioLspClient`；derived client 经统一 registry，manager/子 client 关联需检查。
- **权威/状态**：framework server process/initialize/document version/diagnostic state；
  Host manager/client owner。config/root → start/initialize → document/RPC → shutdown/restart。
- **正常/失败**：load_config/start/restart/get_client、did_open/change/save/close、诊断及
  definition/reference/hover/completion；bad config/server exit/stale doc/close failure。
- **已有断言**：Core fresh manager status/no running servers，仅此不能证明 LSP process
  或 request response。当前已接受 client methods 必须逐行为补场景。
- **最低充分证据**：可控 stdio LSP fixture 的 initialize → UTF-16/Unicode document →
  did_change/version → response/diagnostic → restart/shutdown，错误 RPC 与 server crash，
  derived handle close/quota/owner/generation；三语言 positions/ranges/null/arrays 无损。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

同一真实 stdio fixture 的29 case+1 lifecycle 共30观察/44 text-document calls，三语言返回值/typed errors/request trace 哈希一致；malformed response 明确失败后后续请求可用，shutdown 进程已退出，旧 client 不能复活。

实际来源：`sdks/typescript/test/lsp-host.test.js`；`sdks/python/tests/test_lsp_host.py`；`sdks/java/src/test/java/com/echoagent/sdk/LspHostTest.java`。

仍缺：diagnostics push 的4项新增用例与新 framework 修复正在独立工作，未包括本冻结三语言收据；UTF-16版本/change/restart/owner竞态完整矩阵仍缺。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Request29+managerclose and diagnostic42-observation shared corpora were actually executed; integer/text code and optional severity preserve facts. Explicit final accepted supporting-value generation and unchanged final published revision remain pending.

## 本次有限审查单元

行为/生命周期单元：framework唯一native parser，nullable/range/completion/diagnostic replacement和processowner。

剩余处置：29request+42diag精确支持；新DiagnosticCode supportingaccepted字段生成待最终rev，不复制SDK parser。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
