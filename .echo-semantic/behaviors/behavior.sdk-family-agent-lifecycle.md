---
schema_version: 1
id: behavior.sdk-family-agent-lifecycle
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
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
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

# SDK family agent_lifecycle

## 重要承诺

对已接受 `agent_lifecycle` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：typed `agent/create|describe|close` → `handler.rs` → `factory.rs`、
  `handles.rs`；prepared definition 被真实 Session factory 消费，不能只看 create。
- **权威/状态**：framework Session Agent/connection services 决定运行；Host Agent handle
  只定位 definition 并拥有子 Session/resource owner。create → published → closing → released；
  关闭失败须保留可重试 owner，不能提前删掉活运行。
- **正常/失败**：配置与 idempotency 一致时复用规定对象；冲突、quota、Session 创建中途失败、
  Agent close 与 start 并发、重复 close/EOF 必须显式结算。无关 Agent 不被 cascade。
- **已有断言**：Core `valid_hello_completes_full_core_lifecycle`；Facade
  `agent_cascade_preserves_same_named_other_manager_and_releases_resources` 有 same-name
  隔离及 quota reuse 断言；Languages protocol-host 有 Agent close transport 错误后重试。
- **最低充分证据**：真实 Host 两 Agent/多 Session，阻塞 Run 与 MCP/stream 下 close，
  注入底层 close 失败后重试；断言旧 owner 资源不可用、其它 Agent 继续可用、进程/slot
  结算及 idempotency 冲突无额外创建。代理合成 close 错误不证明 Host 自身 cleanup。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

真实 create/describe/close 和重复关闭、Agent cascade 后资源配额复用及两个同名独立 MCP manager 隔离；活 Run 的同一连接 actor 保持到清理与规范 receipt 结算。

实际来源：`echo-sdk-host/tests/core_profile_e2e.rs#valid_hello_completes_full_core_lifecycle`；`echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources`；`echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle`。

仍缺：多 Agent 创建/关闭并发及全部资源种类 publication fault 尚无逐种语言场景。 published framework pin、最后变更后的独立复审和精确main验收
仍待；本证据不证明未列出的生命周期、字段或错误组合，也不自动扩大accepted合同。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Candidate assertion evidence is now preserved as original receipt/source fingerprints and checked against inspected current paths. It supports only named partial cases; current observation binding neither reruns old commands nor upgrades family status.

## 本次有限审查单元

行为/生命周期单元：AgentDefinition/Session创建与Agent cascade，统一Registry owner/generation与关闭准入。

剩余处置：并发publication/close已知owner义务由agent-close-cascade与cleanup-retry-owner承接；最终当前组合复审。

本项建模阈值是已接受入口、唯一状态权威、正常生命周期与具名失败/恢复路径可定位，并有直接证据或明确开放原因；不要求所有未定义组合全测，也不由绿色测试自动verified。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
