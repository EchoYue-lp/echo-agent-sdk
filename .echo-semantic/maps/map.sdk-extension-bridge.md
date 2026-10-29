---
schema_version: 1
id: map.sdk-extension-bridge
kind: capability_map
title: 扩展桥
risk: high
observed_at: source:42415136ecd37de6bd39f3320bfe90663a7601f9e667238225568b220502a931
boundary_refs:
- boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-extension-bridge
- behavior.sdk-family-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-channel-message-handler-consumer-audit
- evidence.sdk-managed-retirement
- evidence.sdk-managed-supporting-operation-boundary
- evidence.sdk-extension-teardown-prefix-verification
- evidence.sdk-factory-child-settlement-verification
- evidence.sdk-tools-bridge-finite-observation
- evidence.sdk-python310-factory-timeout-closure
finding_refs:
- finding.sdk-managed-restart-passive-host-exit-timeout
- finding.sdk-extension-pending-stream-owner
- finding.sdk-factory-instance-routing
- finding.sdk-managed-store-adaptation
- finding.sdk-sandbox-cleanup-order
- finding.sdk-channel-message-handler-consumer
- finding.sdk-python-agent-component-stream-projection
- finding.sdk-sandbox-canonical-stream-routing
- finding.sdk-custom-agent-invocation-cancel-projection
- finding.sdk-extension-teardown-stream-prefix
- finding.sdk-task-resume-extension-invocation-conflict
- finding.sdk-factory-terminal-close-settlement
- finding.sdk-python310-factory-cleanup-timeout-classification
audit_refs:
- audit.sdk-managed-retirement-rereview
- audit.sdk-tools-bridge-finite-trigger-input
- audit.sdk-tools-bridge-finite-result-side-effect
- audit.sdk-tools-bridge-finite-state-authority
- audit.sdk-tools-bridge-finite-data-durability
- audit.sdk-tools-bridge-finite-time-lifecycle
- audit.sdk-tools-bridge-finite-failure-concurrency
- audit.sdk-tools-bridge-finite-permission-external
- audit.sdk-tools-bridge-finite-contract-evidence
- audit.sdk-python310-factory-timeout-closure
related_map_refs:
- map.sdk-product-boundary
scenarios:
  retained-consumer-values-and-acp-request:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/consumer_compatibility_tests.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-protocol/tests/consumer_compatibility_contract.rs
    - echo-sdk-protocol/tests/native_message_contract.rs
    - contracts/sdk/schema/echo-agent-extension-v1.schema.json
    - sdks/typescript/src/types.ts
    - sdks/typescript/test/consumer-compatibility-host.test.js
    - sdks/typescript/test/consumer-provider-http.test.js
    - sdks/python/tests/test_consumer_compatibility_host.py
    - sdks/python/tests/test_consumer_provider_http.py
    - sdks/java/src/test/java/com/echoagent/sdk/ConsumerCompatibilityHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ConsumerProviderHttpTest.java
    - sdks/shared/testing/consumer_compatibility_host.json
    - sdks/shared/testing/consumer_provider_http.py
    - scripts/check-language-sdks.sh
    - .github/workflows/rust-ci.yml
    - docs/adr/0043-native-message-sdk-adapter.md
    - docs/sdk/sdk-extension-bridge.md
    behavior_refs:
    - behavior.sdk-extension-bridge
    rule_refs:
    - rule.framework-runtime-authority
    unknown: Protocol原strict反例3通过/2失败保留；修后原Protocol227及Frame127d/SDK1290源的Host六控制通过。同14ec stdio Host的三语六whole-loop及十二HTTP组合已通过，Java为单方法内组合；完整语言/quickstarts/Parquet/fullfacade仍独立，正式pin与accepted不自动扩。
    next_step: 使用准确SDK冻结源与Root准许Framework组合，运行新ACP控制及原完整消费者；重型full facade/Parquet/producer只在Root独占窗口。
    review_scope: 沿Core唯一serde保既有ToolChoice合法值、strict独立可选字段和原生消息；真实chat/chat_stream原ACP路径不能以手工DTO往返替代，原确认字段拒绝guard保持。
  provider-response-observation-transport:
    status: needs_review
    source_refs:
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-protocol/src/schema.rs
    - echo-sdk-protocol/tests/provider_response_transport.rs
    - echo-sdk-protocol/tests/native_message_contract.rs
    - echo-sdk-host/src/core_profile/native_message.rs
    - echo-sdk-host/src/core_profile/wire.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/extension_bridge_message_tests.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    - contracts/sdk/schema/echo-agent-extension-v1.schema.json
    - contracts/sdk/fixtures/extension/v1/provider-response-minimal-valid.json
    - contracts/sdk/fixtures/extension/v1/provider-response-null-fields-valid.json
    - contracts/sdk/fixtures/extension/v1/provider-response-unknown-usage-valid.json
    - contracts/sdk/fixtures/extension/v1/provider-response-model-shape-invalid.json
    - contracts/sdk/fixtures/extension/v1/provider-response-usage-shape-invalid.json
    - contracts/sdk/fixtures/extension/v1/provider-response-extra-field-invalid.json
    - contracts/sdk/fixtures/extension/v1/message-provider-observation-valid.json
    - contracts/sdk/fixtures/extension/v1/llm-incomplete-bounded-failure-valid.json
    - sdks/typescript/src/types.ts
    - sdks/typescript/src/client.ts
    - sdks/typescript/test/typed-bridge.test.js
    - sdks/typescript/test/custom-agent-message-example.test.js
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/__init__.py
    - sdks/python/tests/test_custom_agent_message_example.py
    - sdks/java/src/main/java/com/echoagent/sdk/CallbackMessage.java
    - sdks/java/src/main/java/com/echoagent/sdk/LlmChatCall.java
    - sdks/java/src/test/java/com/echoagent/sdk/CallbackAndEffectsTest.java
    - docs/adr/0043-native-message-sdk-adapter.md
    - docs/sdk/native-messages.md
    - docs/sdk/sdk-extension-bridge.md
    - contracts/sdk/fixtures/extension/v1/request-tool-choice-function-valid.json
    - contracts/sdk/fixtures/extension/v1/request-tool-choice-null-valid.json
    - contracts/sdk/fixtures/extension/v1/request-tool-choice-mode-valid.json
    - contracts/sdk/fixtures/extension/v1/request-tool-choice-unknown-mode-invalid.json
    - contracts/sdk/fixtures/extension/v1/request-tool-choice-extra-invalid.json
    - contracts/sdk/fixtures/extension/v1/request-tool-choice-blank-name-invalid.json
    behavior_refs:
    - behavior.sdk-extension-bridge
    rule_refs:
    - rule.framework-runtime-authority
    unknown: Protocol56与三语request/资源TS29/PY13/Java18已通过；旧String-only Host8项只保原切点，当前Host绑定f327/2831与SDK1280源r04另取证。原编译失败保留，完整producer、正式新pin、冷恢复与完整SDK仍独立待验。
    next_step: 保留精确源与本片有限结果，待已审Git来源后由原完整producer列aggregate delta并完成完整消费者；不自动扩大accepted或重标历史Evidence。
    review_scope: 固定九观察、缺失/Null、嵌套Scalar、工具错误三值和canonical Empty无损转换；既有tool_choice字段按Core唯一serde保三模式与具名Function对象，不增独立accepted身份。SDK不解释终态、不创建身份、不暴露raw partial、不另建运行时权威。
  custom-agent-native-reverse-output:
    status: needs_review
    source_refs:
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-protocol/src/schema.rs
    - echo-sdk-protocol/tests/native_message_contract.rs
    - echo-sdk-protocol/tests/extension_contract.rs
    - contracts/sdk/schema/echo-agent-extension-v1.schema.json
    - contracts/sdk/fixtures/extension/v1/extension-stream-agent-final-chunk-invalid.json
    - contracts/sdk/fixtures/extension/v1/extension-stream-agent-final-chunk-valid.json
    - contracts/sdk/fixtures/extension/v1/extension-stream-agent-status-candidate-complete-invalid.json
    - contracts/sdk/fixtures/extension/v1/extension-stream-agent-status-settled-complete-valid.json
    - contracts/sdk/fixtures/extension/v1/extension-stream-agent-status-started-chunk-valid.json
    - contracts/sdk/fixtures/extension/v1/extension-stream-agent-status-wrong-phase-invalid.json
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/extension_bridge_message_tests.rs
    - echo-sdk-host/src/core_profile/native_message.rs
    - sdks/typescript/src/runtime_terminal.ts
    - sdks/typescript/examples/custom-agent-message.ts
    - sdks/typescript/test/runtime-terminal.test.js
    - sdks/typescript/test/native-producer-fixture.js
    - sdks/python/src/echo_agent_sdk/_runtime_terminal.py
    - sdks/python/examples/custom_agent_message.py
    - sdks/python/tests/test_runtime_terminal.py
    - sdks/python/tests/native_producer_fixture.py
    - sdks/java/src/main/java/com/echoagent/sdk/RuntimeTerminal.java
    - sdks/java/src/main/java/com/echoagent/sdk/RichMessageAgentExample.java
    - sdks/java/src/test/java/com/echoagent/sdk/RuntimeTerminalTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/NativeProducerFixture.java
    - docs/adr/0043-native-message-sdk-adapter.md
    - docs/sdk/native-messages.md
    behavior_refs:
    - behavior.sdk-extension-bridge
    rule_refs:
    - rule.framework-runtime-authority
    unknown: 有限冻结v03的ACP输出和同源Host三语消费已验证具名范围；正式Git pin、accepted聚合/退役实际差异、完整SDK/平台/外层Task与D1/D2尚未闭合，不能由生成fixture或局部绿色代替。
    next_step: 保留各工具epoch与原失败，用准确后继Framework来源完成正式producer和完整消费者验收；不把ACK、String、FinalAnswer或source EOF补为原生成功。
    review_scope: 六个fixture由唯一producer生成；FinalAnswer普通通知与原execution_status运输位置分列，原identity/phase/u64/reasoning值传。Java按demand等待原send，factory仅在原source真实EOF后close，不另建executor、store或状态权威。
  custom-agent-full-message-input:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/extension_bridge_message_tests.rs
    - echo-sdk-host/src/core_profile/wire.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-protocol/src/schema.rs
    - echo-sdk-protocol/tests/native_message_contract.rs
    - echo-sdk-protocol/tests/extension_contract.rs
    - sdks/typescript/src/types.ts
    - sdks/typescript/examples/custom-agent-message.ts
    - sdks/typescript/test/custom-agent-message-example.test.js
    - sdks/python/examples/custom_agent_message.py
    - sdks/python/tests/test_custom_agent_message_example.py
    - sdks/java/src/main/java/com/echoagent/sdk/RichMessageAgentExample.java
    - docs/adr/0043-native-message-sdk-adapter.md
    - docs/sdk/sdk-extension-bridge.md
    behavior_refs:
    - behavior.sdk-extension-bridge
    rule_refs:
    - rule.framework-runtime-authority
    unknown: 当前完整Message/history源码与语言mock已验证具名范围；Rust typed override回归、原schema生成、同源Host和真实factory外层消费者尚待执行，不能继承旧234或三语言资格。
    next_step: Root开放精确纯Protocol批次后运行原测试与唯一schema/语言资源生成入口，再于合格新Framework同源窗口验证Host及真实消费者；不手改生成JSON或占位pin。
    review_scope: 输入复用原LlmMessageWire和run_stream_scoped；Text/Parts/ResourceLink、role/name、工具身份、原生元数据和history保全。运行时FinalAnswer/Guard安全正文/回调及原owner保留，不把候选正文当结算。
  behavior-lifecycle-failure:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
    - sdks/shared/testing/managed_persistence_backend.py
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/LateFactoryHostTest.java
    behavior_refs:
    - behavior.sdk-extension-bridge
    finding_refs:
    - finding.sdk-extension-pending-stream-owner
    - finding.sdk-factory-instance-routing
    - finding.sdk-managed-store-adaptation
    - finding.sdk-sandbox-cleanup-order
    review_scope: Entry, unique authority, normal lifecycle and named failure obligations
      have been inspected and linked. Mapped records the model, not bug-free/runtime/main
      closure; open Findings remain explicit.
  family-bridge:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    behavior_refs:
    - behavior.sdk-family-bridge
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-tools-bridge-finite-observation
    review_scope: Six existing reverse roles finiteexamined, with knownphysicalB and
      widefeatureCauseFinding explicit; not full genericextensionverified.
  tools-reverse-finite-trigger-input:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    behavior_refs:
    - behavior.sdk-extension-bridge
    - behavior.sdk-family-bridge
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    audit_refs:
    - audit.sdk-tools-bridge-finite-trigger-input
    review_scope: finite originalcurrentTool builtin10/reverse6 sourceaudit; remote
      cause3validredgreen independentPASS; no new protocol or allmethodsfamilyclosure
  tools-reverse-finite-result-side-effect:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    behavior_refs:
    - behavior.sdk-extension-bridge
    - behavior.sdk-family-bridge
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    audit_refs:
    - audit.sdk-tools-bridge-finite-result-side-effect
    review_scope: finite originalcurrentTool builtin10/reverse6 sourceaudit; remote
      cause3validredgreen independentPASS; no new protocol or allmethodsfamilyclosure
  tools-reverse-finite-state-authority:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    behavior_refs:
    - behavior.sdk-extension-bridge
    - behavior.sdk-family-bridge
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    audit_refs:
    - audit.sdk-tools-bridge-finite-state-authority
    review_scope: finite originalcurrentTool builtin10/reverse6 sourceaudit; remote
      cause3validredgreen independentPASS; no new protocol or allmethodsfamilyclosure
  tools-reverse-finite-data-durability:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    behavior_refs:
    - behavior.sdk-extension-bridge
    - behavior.sdk-family-bridge
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    audit_refs:
    - audit.sdk-tools-bridge-finite-data-durability
    review_scope: finite originalcurrentTool builtin10/reverse6 sourceaudit; remote
      cause3validredgreen independentPASS; no new protocol or allmethodsfamilyclosure
  tools-reverse-finite-time-lifecycle:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    behavior_refs:
    - behavior.sdk-extension-bridge
    - behavior.sdk-family-bridge
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    audit_refs:
    - audit.sdk-tools-bridge-finite-time-lifecycle
    review_scope: finite originalcurrentTool builtin10/reverse6 sourceaudit; remote
      cause3validredgreen independentPASS; no new protocol or allmethodsfamilyclosure
  tools-reverse-finite-failure-concurrency:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    behavior_refs:
    - behavior.sdk-extension-bridge
    - behavior.sdk-family-bridge
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    audit_refs:
    - audit.sdk-tools-bridge-finite-failure-concurrency
    review_scope: finite originalcurrentTool builtin10/reverse6 sourceaudit; remote
      cause3validredgreen independentPASS; no new protocol or allmethodsfamilyclosure
  tools-reverse-finite-permission-external:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    behavior_refs:
    - behavior.sdk-extension-bridge
    - behavior.sdk-family-bridge
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    audit_refs:
    - audit.sdk-tools-bridge-finite-permission-external
    review_scope: finite originalcurrentTool builtin10/reverse6 sourceaudit; remote
      cause3validredgreen independentPASS; no new protocol or allmethodsfamilyclosure
  tools-reverse-finite-contract-evidence:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    behavior_refs:
    - behavior.sdk-extension-bridge
    - behavior.sdk-family-bridge
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    audit_refs:
    - audit.sdk-tools-bridge-finite-contract-evidence
    review_scope: finite originalcurrentTool builtin10/reverse6 sourceaudit; remote
      cause3validredgreen independentPASS; no new protocol or allmethodsfamilyclosure
---

# 扩展桥

## 能力范围

extension register/unregister经descriptor/capability准入，反向call复用官方connection lease/deadline；Tool/LLM/Store/hook/callback由framework消费并决定terminal/commit。initial response前RAII stream reservation避免超时泄漏；cleanup/interrupted/terminal callback使用connection作用域，不因Run取消丢清理。Factory先reserve官方child handle及factory_origin，再publish/rollback，同Factory可创建独立Session实例。managed11 typed DTO/TaskStore execution-CAS显式opt-in，真实deadline-before-work、lost ACK/断连重启同receipt保留commit权威。Bridge34候选通过，只证明具名调用；Java2bad cleanup/failed-future owner有自身264项与独立复审记录，但当前能力未列场景仍不算闭合。

## 入口与输出

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 行为关系

本能力经 official ACP/profile 或具体 catalog route 到 Host adapter，然后调用框架权威。跨能力调用与三语言消费见 docs/sdk/governance/capability-map.md。

## 状态与数据流

Framework 拥有 Run/Task/Store 的事实与终态；Host 只拥有寻址、可取消资源 owner 与协议发送状态；客户端确认已消费事实。

## 策略来源与优先级

用户八项治理目标与 AGENTS.md 约束行为；accepted external contract 约束兼容；compiled feature/canonical framework service 约束实现。完整 Rust inventory 不扩承诺。

## 生命周期与失败路径

extension register/unregister经descriptor/capability准入，反向call复用官方connection lease/deadline；Tool/LLM/Store/hook/callback由framework消费并决定terminal/commit。initial response前RAII stream reservation避免超时泄漏；cleanup/interrupted/terminal callback使用connection作用域，不因Run取消丢清理。Factory先reserve官方child handle及factory_origin，再publish/rollback，同Factory可创建独立Session实例。managed11 typed DTO/TaskStore execution-CAS显式opt-in，真实deadline-before-work、lost ACK/断连重启同receipt保留commit权威。Bridge34候选通过，只证明具名调用；Java2bad cleanup/failed-future owner有自身264项与独立复审记录，但当前能力未列场景仍不算闭合。

原阶段缺口（历史）：managed Store的11个DTO与deadline/结果恢复已有有界证据，剩余真实caller/operation审计仍明确open；Channel MessageHandler原缺口已用户批准保承诺与forward范围；当前设计/实现未完。reverse mailbox/child factory/取消重入按具名Finding审查，不用任意组合扩大门槛。

## 权限与敏感信息

本地 SDK 不引入线上权限门控。继续保持框架自动行为 permission authority，凭据不进入日志/合同/错误；资源 gate 仅防止错误owner和无意数据丢失。

## 用户侧投影

TypeScript/Python/Java 对已接受能力暴露惯用接口，handle/error/value 必须无损；不能用成功默认值或丢字段改变权威事实。

## 场景处置清单

主场景mapped只表示模型已展开；关联family Behavior仍needs_review，具体Evidence按支持关系与原epoch读取。剩余义务：managed Store的11个DTO与deadline/结果恢复已有有界证据，剩余真实caller/operation审计仍明确open；Channel MessageHandler原缺口已用户批准保承诺与forward范围；当前设计/实现未完。reverse mailbox/child factory/取消重入按具名Finding审查，不用任意组合扩大门槛。

## 未展开项

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch与95ecpin保持原运行epoch；当前Host已pin published b909并移除patch，但最终nightlyprofiles/唯一合同生成与SDKwhole源码门禁仍待。frameworkmain交付证据不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

managed Store的11个DTO与deadline/结果恢复已有有界证据，剩余真实caller/operation审计仍明确open；Channel MessageHandler原缺口已用户批准保承诺与forward范围；当前设计/实现未完。reverse mailbox/child factory/取消重入按具名Finding审查，不用任意组合扩大门槛。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，family needs_review和开放Finding保留，主场景mapped不扩运行证据。

## 建模完成与验证状态区别

组级主路径已按入口、唯一权威、生命周期和具名失败义务完成映射，故主场景mapped。family Behavior仍needs_review，已知open Finding与有限证据保留；mapped不等于verified、所有lens covered或最终交付。剩余事项采用明确合同/故障原因，不以无限未枚举组合拒绝建模。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。

## 当前有界Tools与reverse角色

sharedEvidence/8Audit对应真实5bcb与原runtime69b；原failurecause两个remote子scope已sourceconsumer修复，不推全Workflow/Memory/Eval。框架仍唯一schema/ToolFailure/LLMterminal authority，Hostthin。各旧原packet与after3focused本体保持，原whole790不冒afterpatch新whole；状态mapped仅建模examined。


## Python3.10 Factory cleanup typed超时局部闭合

新具名Finding finding.sdk-python310-factory-cleanup-timeout-classification承接受支持3.10真实wait_for异常被builtin捕获漏掉而返回extension_failed的单行生产缺陷。修后asyncio.TimeoutError恢复原extension_timeout/after_delay，shield/pendingchild owner/retry和cleanup权威不变；原测试不改、两版本各31及第四whole按各sourcecut读取，三链由上述specificEvidence/Audit承接。原map观察/场景source_refs与所有旧义务保，不因本局部closed提升Bridge/LanguageBehavior或B物理/12连续性。SDKmain/平台另验，不新增状态/contract/API。


## Managed restart被动Host退出的具体未决

新增 finding.sdk-managed-restart-passive-host-exit-timeout 只定位原0ebe第一Host child.wait10s Elapsed，restart与后续receipt断言未达。入口本已在此Map场景sourceRefs中；场景mapped不代表该失败已修。原scenes/status/观察不改，新诊断未提交不假afterGit、不推数据丢失或B5秒policy。


## 2026-10-28：完整Message输入的当前依据与待验

新增场景只关联本次CustomAgent完整输入源码、三语言示例与mock消费者，使用原Framework
runtime权威和SDK薄转换。现有LlmMessageWire、message_wire/from_wire与run_stream_scoped
承担同一输入；role/name、Parts/ResourceLink meta、工具身份、原生块/批准投影/状态和
invocation.history不降为裸字符串，默认/显式invocation路径保持原owner和取消信号。

当前具名语言mock TS44/Python40/Java8通过、源码静态独审通过；Rust、schema/package生成、
同源Host/真实FactoryAdapter外层仍未验，故新增场景保持needs_review。原观察5bcb与旧场景
及历史Evidence可由WIP6cb恢复；本source观察更新只定位当前新增依据，不提升它们的旧
验证状态或将raw旧票改称当前。runtime FinalAnswer/回调/Guard安全正文/校验收尾均保留。
