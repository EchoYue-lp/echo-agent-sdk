---
schema_version: 1
id: behavior.sdk-family-workflow
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
- evidence.sdk-workflow-successor-candidate
- evidence.sdk-llm-workflow-stream
- evidence.sdk-python-typed-component-stream-verification
finding_refs:
- finding.sdk-python-agent-component-stream-projection
---

# SDK family workflow

## 重要承诺

对已接受 `workflow` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/workflow.rs::dispatch` → framework declarative graph/engine；
  extension workflow 与 facade stream 用同一 graph/stream adapter。
- **权威/状态**：framework `workflow::Graph`、`SharedState`、`CheckpointStore`、cancel
  token；Host graph/state/stream owner。build → run → interrupt/checkpoint → exact resume/
  restore/branch → terminal/close。
- **正常/失败**：condition path、pending node、state revision/identity、branch/tag/load；
  invalid graph/checkpoint、cancel-vs-next、same-owner wrong-kind、Session close、stream error。
- **已有断言**：Core declarative graph 5 nodes/4 edges、conditional yes/no、checkpoint、state
  get/set/keys、cancel then framework error、cross owner；Facade unified stream/backpressure/
  connection teardown；Bridge workflow-agent component stream。
- **最低充分证据**：resume_exact/resume_with_state/restore/branch/tag 的完整 state/checkpoint
  字段比对，cancel/fault 后 graph terminal 唯一；stream next 并发/slow consumer/close retry
  不漏 NodeError/Completed，三语言同场景且重启后 checkpoint 身份不被默认替换。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

真实五节点四边条件图、checkpoint/state 操作；cancel 返回规范 Cancelled/cause 并在 stream failed terminal 保留；统一资源 owner/stream/backpressure/close 和反向 workflow-agent 消费实际执行。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#workflow_family_runs_declarative_graphs_over_the_framework_engine`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_cancel_preserves_canonical_failure_in_rpc_and_stream_terminal`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_stream_uses_unified_handle_backpressure_and_close`；`echo-sdk-host/tests/extension_bridge_e2e.rs#workflow_agent_component_stream_flows_through_facade_pull_stream`。

仍缺：resume_exact/restore/branch/tag 所有 state/checkpoint 字段及三语言重启共同场景未完成。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Current Workflow producer/run-graph context/cache authority changed after frozen6711 input; old418/TS240/Python300/Java264 do not certify it. Separate successor execution/review receipt is pending binding; no retained runner may ignore supplied graph/state.

## Workflow后继具名运行证据

正式48row共享语料已对f5c72源码/cb9Host执行并三语言精确匹配；17/24accepted操作、11expectederror及完整checkpoint/state/lineage断言见evidence.sdk-workflow-successor-candidate。其余七accepted操作和全套最终门禁不继承旧6711通过，仍needs_review。

## 本次有限审查单元

行为/生命周期单元：graph/state/checkpoint object、resume/branch/lineage与framework claim权威。

剩余处置：原48行17/24与后继LLM/Workflow三组分别归档；剩7个accepted Workflow verbs已真实可达，流finally/owner/cancel有0744有界证据。Python typed AgentComponentCall.stream丢字段精确Finding已新5green与独立PASS本地resolved；最终字段生成/epoch与合同一致性另验。

本项建模阈值是已接受入口、唯一权威、生命周期及具名失败可定位，直接证据或明确开放原因可复核；mapped不代表verified，不要求未定义的任意组合。
