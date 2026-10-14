---
schema_version: 1
id: behavior.sdk-family-channels
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
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
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
- evidence.sdk-channel-message-handler-consumer-audit
- evidence.sdk-mcp-channel-owned-candidate
- evidence.sdk-channel-resource-admission
- evidence.sdk-channel-cooperative-two
- evidence.sdk-extension-teardown-prefix-verification
finding_refs:
- finding.sdk-channel-message-handler-consumer
- finding.sdk-channel-resource-operation-admission
- finding.sdk-channel-external-callback-cooperative-close
- finding.sdk-extension-teardown-stream-prefix
---

# SDK family channels

## 重要承诺

对已接受 `channels` 能力明确入口、唯一权威、状态/生命周期、失败处置和直接行为证据；未接受 Rust identity 不进入承诺。

## 当前行为

- **入口/可达**：`facade/integrations.rs::dispatch_channels`，channels + bridge 编译后
  manager 注册 extension plugin，framework `ChannelManager` 消费 message handler。
- **权威/状态**：framework channel/plugin connection 与 message delivery；Host proxy
  lease/resource/handler address。register → start → receive/send/health → stop → close。
- **正常/失败**：plugin message/attachments/result，stop_exact/list/ids；start/stop/send
  callback error、disconnect、feature/bridge absent、publication failure 和 unregister race。
- **已有断言**：Bridge `channel_plugin_facade_round_trip` 和 channel feature requirement；
  Core admission create resource/MethodNotFound。仅 manager.open 不证明 receive/send。
- **最低充分证据**：真实 Host plugin start/send/receive/hook/attachments，stop 与 pending
  callback 结算、stop_exact 不影响 sibling manager、重入/error/disconnect 后 resource
  可重试/无 leak；三语言 message identity 与 delivery outcome 一致。

## 期望行为

上述正常/失败场景须保持接受范围、完整转换和唯一权威，并在最终源码/pin证据及独立复审后闭合。

## 触发、结果与副作用

按上方具体入口和正常/失败记录检查真实主路径。可达性或catalog状态不能证明副作用、错误或结算。

## 失败、重试与恢复

按上方生命周期和最低充分证据检查取消、超时、错误、释放和恢复；不默认成功，不以其它能力族测试替代。

## 证据

阶段：`candidate_runtime_verified`；语义状态仍为 `needs_review`。

真实 plugin 经 facade manager 注册、start/send/stop 的指定输入/输出与 feature requirement 断言通过；未以 manager.open 代替插件消费。

实际来源：`echo-sdk-host/tests/extension_bridge_e2e.rs#channel_plugin_facade_round_trip`。

当前有界后继：MCP/Channel五场景每语言真实通过，来源与完整局部断言见evidence.sdk-mcp-channel-owned-candidate。独立后继复审、最终pin/main与具体开放Finding仍待；此证据不扩大accepted操作。

## 裁决记录

用户要求SDK全能力行为治理，以明确接受跨语言合同为兼容边界；完整Rust inventory仅遥测。

## 当前观察与历史收据边界

Registered plugin start/send/stop have named true-Host evidence. Accepted MessageHandler handle/handle_stream/reply consumer remains unreachable through the stored handler/handler_id path; new open Finding tracks it. No inbound success is inferred.

## Channel inbound consumer obligation

Accepted MessageHandler handle/handle_stream/reply has a current static consumer
gap. Registration/start/send/stop receipts do not close it; no unaccepted receive
wire is invented. See finding.sdk-channel-message-handler-consumer and its source
audit Evidence.

## 本次有限审查单元

行为/生命周期单元：plugin register/start/send/stop与框架ownedentry的生命周期。

剩余处置：0744 resourceadmission capturedstart真red1→3units/Hostlib140/三语各5及独立pass保存该epoch；用户已批准handle/handle_stream/reply及必要streamcontrols，ADR0035设计/实现/验证待完成，不再等待产品选择。

本项建模阈值是已接受入口、唯一权威、生命周期及具名失败可定位，直接证据或明确开放原因可复核；mapped不代表verified，不要求未定义的任意组合。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
