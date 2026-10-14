---
schema_version: 1
id: map.sdk-agent-session-run
kind: capability_map
title: Agent、Session、Run
risk: high
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
boundary_refs:
- boundary.sdk-agent-session-run
behavior_refs:
- behavior.sdk-agent-session-run
- behavior.sdk-family-agent-lifecycle
- behavior.sdk-family-session
- behavior.sdk-family-run
rule_refs:
- rule.framework-runtime-authority
evidence_refs: []
finding_refs:
- finding.sdk-agent-close-cascade
- finding.sdk-cleanup-retry-owner
audit_refs: []
related_map_refs:
- map.sdk-product-boundary
scenarios:
  behavior-lifecycle-failure:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle
    - echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run
    - sdks/python/tests/test_parallel_sessions_host.py
    behavior_refs:
    - behavior.sdk-agent-session-run
    finding_refs:
    - finding.sdk-agent-close-cascade
    - finding.sdk-cleanup-retry-owner
    review_scope: Entry, unique authority, normal lifecycle and named failure obligations
      have been inspected and linked. Mapped records the model, not bug-free/runtime/main
      closure; open Findings remain explicit.
  family-agent-lifecycle:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/factory.rs
    behavior_refs:
    - behavior.sdk-family-agent-lifecycle
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing仅声明库存无运行supports。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
  family-session:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/factory.rs
    behavior_refs:
    - behavior.sdk-family-session
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing仅声明库存无运行supports。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
  family-run:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/factory.rs
    behavior_refs:
    - behavior.sdk-family-run
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing仅声明库存无运行supports。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
---

# Agent、Session、Run

## 能力范围

Agent/Session/Run handler复用framework规范执行、receipt与取消。HandleRegistry唯一管理owner/generation/idempotency/配额；创建及关闭admission、durable receipt slot、失败写入重试和Agent cascade已有候选反例。Session create/load读取持久原生身份并调用framework reserve_recovered_turn_ids，turn7后分配turn8；禁用同一调用得到turn1的两路径红证据已取得。活Run关闭保持同一个官方actor，不以watcher替代join/receipt。真实kill恢复Interrupted；并行Session取消A不改变B。

## 入口与输出

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 行为关系

本能力经 official ACP/profile 或具体 catalog route 到 Host adapter，然后调用框架权威。跨能力调用与三语言消费见 docs/sdk/governance/capability-map.md。

## 状态与数据流

Framework 拥有 Run/Task/Store 的事实与终态；Host 只拥有寻址、可取消资源 owner 与协议发送状态；客户端确认已消费事实。

## 策略来源与优先级

用户八项治理目标与 AGENTS.md 约束行为；accepted external contract 约束兼容；compiled feature/canonical framework service 约束实现。完整 Rust inventory 不扩承诺。

## 生命周期与失败路径

Agent/Session/Run handler复用framework规范执行、receipt与取消。HandleRegistry唯一管理owner/generation/idempotency/配额；创建及关闭admission、durable receipt slot、失败写入重试和Agent cascade已有候选反例。Session create/load读取持久原生身份并调用framework reserve_recovered_turn_ids，turn7后分配turn8；禁用同一调用得到turn1的两路径红证据已取得。活Run关闭保持同一个官方actor，不以watcher替代join/receipt。真实kill恢复Interrupted；并行Session取消A不改变B。

仍缺：Agent cascade/cleanup retry仍由具名Finding承接；Session closing读取准入与owned Channel回调新窗口须各自最终反例/复审。framework parent-drop修复使用自己的后继证据，旧23项通过不替后继。

## 权限与敏感信息

本地 SDK 不引入线上权限门控。继续保持框架自动行为 permission authority，凭据不进入日志/合同/错误；资源 gate 仅防止错误owner和无意数据丢失。

## 用户侧投影

TypeScript/Python/Java 对已接受能力暴露惯用接口，handle/error/value 必须无损；不能用成功默认值或丢字段改变权威事实。

## 场景处置清单

主场景mapped只表示模型已展开；关联family Behavior仍needs_review，具体Evidence按支持关系与原epoch读取。剩余义务：Agent cascade/cleanup retry仍由具名Finding承接；Session closing读取准入与owned Channel回调新窗口须各自最终反例/复审。framework parent-drop修复使用自己的后继证据，旧23项通过不替后继。

## 未展开项

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch与95ecpin保持原运行epoch；当前Host已pin published b909并移除patch，但最终nightlyprofiles/唯一合同生成与SDKwhole源码门禁仍待。frameworkmain交付证据不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

Agent cascade/cleanup retry仍由具名Finding承接；Session closing读取准入与owned Channel回调新窗口须各自最终反例/复审。framework parent-drop修复使用自己的后继证据，旧23项通过不替后继。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，family needs_review和开放Finding保留，主场景mapped不扩运行证据。

## 建模完成与验证状态区别

组级主路径已按入口、唯一权威、生命周期和具名失败义务完成映射，故主场景mapped。family Behavior仍needs_review，已知open Finding与有限证据保留；mapped不等于verified、所有lens covered或最终交付。剩余事项采用明确合同/故障原因，不以无限未枚举组合拒绝建模。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
