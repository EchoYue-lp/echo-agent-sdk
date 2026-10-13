---
schema_version: 1
id: map.sdk-extension-bridge
kind: capability_map
title: 扩展桥
risk: high
observed_at: source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404
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
finding_refs:
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
audit_refs:
- audit.sdk-managed-retirement-rereview
related_map_refs:
- map.sdk-product-boundary
scenarios:
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
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    behavior_refs:
    - behavior.sdk-family-bridge
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing仅声明库存无运行supports。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
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

仍缺：managed Store的11个DTO与deadline/结果恢复已有有界证据，剩余真实caller/operation审计仍明确open；Channel MessageHandler原缺口已用户批准保承诺与forward范围；当前设计/实现未完。reverse mailbox/child factory/取消重入按具名Finding审查，不用任意组合扩大门槛。

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
