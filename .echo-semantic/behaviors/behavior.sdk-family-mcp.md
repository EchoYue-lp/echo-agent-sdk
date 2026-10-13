---
schema_version: 1
id: behavior.sdk-family-mcp
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
- evidence.sdk-mcp-channel-owned-candidate
finding_refs:
- finding.sdk-mcp-batch-publication
---

# SDK family mcp

## 重要承诺

对已接受 `mcp` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/integrations.rs::dispatch` → `McpManager`/`McpClient`；source
  exact constructors/tool adapters 同服务，`IntegrationResources` 保存 preparation/cleanup debt。
- **权威/状态**：framework manager/client/transport；Host manager-to-derived-client/tool
  关联及 handles。prepare/connect → publish → tools/resources/prompts calls → disconnect/
  close_all → awaited cleanup。同名 server 在不同 manager 中不是同一 owner。
- **正常/失败**：config/reconcile/get_clients/tool call、server metadata；connect/close failure、
  cancelled preparation、quota publication rollback、manager close 后 derived handles 失效。
- **已有断言**：Core fresh manager/list、connect fault；Bridge MCP quota publication 失败
  等待 transport close；integrations unit 用 CleanupTransport 覆盖部分 debt retry；Facade
  Agent cascade same-name isolation。mock transport unit 不证明 native subprocess 生命周期。
- **最低充分证据**：真实 stdio/HTTP fixture server connect/call/resources/prompts/close，
  两 manager 同名隔离、derived client/tool 失效；close fault 后可重试 debt、取消 publication
  不遗留进程/slot；三语言 typed metadata/error/callback，feature disabled 单独验收。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

config/file 两真实 stdio batch 在 quota1 拒绝整批 SDK alias，不发布无返回 prefix；已成功连接仍由 Agent 管理，best-effort accepted 行为保留，既有 alias 与同名其他 manager 不被误关；close debt unit 与 transport publication quota 场景实际执行。

实际来源：`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_quota_failure_does_not_publish_unreturned_prefix`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_file_batch_quota_failure_does_not_publish_unreturned_prefix`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_preserves_best_effort_and_existing_alias`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources`。

当前有界后继：MCP/Channel五场景每语言真实通过，来源与完整局部断言见evidence.sdk-mcp-channel-owned-candidate。独立后继复审、最终pin/main与具体开放Finding仍待；此证据不扩大accepted操作。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：framework client/manager/tools/resources/prompts与注册transport真实关闭。

剩余处置：正式4scene支持exactPID/内容/失败重试；新ownedChannel不冒MCP全覆，pending close cancel明确审计。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。
