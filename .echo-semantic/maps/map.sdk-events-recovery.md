---
schema_version: 1
id: map.sdk-events-recovery
kind: capability_map
title: 事件、ACK、replay、持久化
risk: high
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
boundary_refs:
- boundary.sdk-events-recovery
behavior_refs:
- behavior.sdk-events-recovery
- behavior.sdk-family-event-replay
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-ack-recovery-verification
- evidence.sdk-gap-watermark-host-verification
finding_refs:
- finding.sdk-ack-submission-consumption
- finding.sdk-gap-ack-replay-watermark
- finding.sdk-language-consumption-ack
audit_refs:
- audit.sdk-ack-recovery-rereview
- audit.sdk-gap-watermark-host-rereview
related_map_refs:
- map.sdk-product-boundary
scenarios:
  behavior-lifecycle-failure:
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/events.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/wire.rs
    - echo-sdk-protocol/src/event.rs
    - echo-sdk-host/src/core_profile/events.rs#real_journal_retention_page_authorizes_only_its_returned_watermark
    - echo-sdk-host/src/core_profile/events.rs#replay_publication_failure_and_empty_future_page_grant_no_ack_coverage
    - echo-sdk-host/tests/core_profile_e2e.rs#consumed_multiple_replay_page_ack_resumes_live_after_page_cursor
    - sdks/shared/testing/gap_ack_fixture.py
    - sdks/typescript/test/gap-ack-host.test.js
    - sdks/python/tests/test_gap_ack_host.py
    - sdks/java/src/test/java/com/echoagent/sdk/ExternalAckHostTest.java
    behavior_refs:
    - behavior.sdk-events-recovery
    finding_refs:
    - finding.sdk-ack-submission-consumption
    - finding.sdk-gap-ack-replay-watermark
    - finding.sdk-language-consumption-ack
    review_scope: Entry, unique authority, normal lifecycle and named failure obligations
      have been inspected and linked. Mapped records the model, not bug-free/runtime/main
      closure; open Findings remain explicit.
  family-event-replay:
    status: needs_review
    source_refs:
    - echo-sdk-host/src/core_profile/events.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-protocol/src/event.rs
    - echo-sdk-host/tests/core_profile_e2e.rs
    behavior_refs:
    - behavior.sdk-family-event-replay
    unknown: 该family已有具名候选断言；完整最低充分行为及最终published pin/source、独立复审仍未闭合。testing仅声明库存无运行supports。
    next_step: 沿关联family Behavior的最低充分证据补未列场景；最终源码/二进制/合同冻结后独立复审及精确main对账，不重跑或抹除已有反例。
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
---

# 事件、ACK、replay、持久化

## 能力范围

framework EventJournal唯一拥有committed envelope、journal identity和terminal；StreamDelivery只管理发送窗口与cursor。#120候选修复把snapshot/page已消费ACK的coverage推进到所覆盖水位，再续发严格更大的sequence；成功enqueue后才授权ACK，重复/未来ACK不退回水位。12个event unit包含真实segmented journal轮转/prune、oversized gap、single/multipage coverage、空future page及publish失败不扩大授权。真实三语言count/oversized、延迟external replay ACK两窗口、terminal replay ACK自然EOF均执行，并比较journal union/event_id/唯一terminal；不是旧tiny-window ACK后即退出。

## 入口与输出

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 行为关系

本能力经 official ACP/profile 或具体 catalog route 到 Host adapter，然后调用框架权威。跨能力调用与三语言消费见 docs/sdk/governance/capability-map.md。

## 状态与数据流

Framework 拥有 Run/Task/Store 的事实与终态；Host 只拥有寻址、可取消资源 owner 与协议发送状态；客户端确认已消费事实。事件 ACK cursor 不是 Journal 水位。

## 策略来源与优先级

用户八项治理目标与 AGENTS.md 约束行为；accepted external contract 约束兼容；compiled feature/canonical framework service 约束实现。完整 Rust inventory 不扩承诺。

## 生命周期与失败路径

framework EventJournal唯一拥有committed envelope、journal identity和terminal；StreamDelivery只管理发送窗口与cursor。#120候选修复把snapshot/page已消费ACK的coverage推进到所覆盖水位，再续发严格更大的sequence；成功enqueue后才授权ACK，重复/未来ACK不退回水位。12个event unit包含真实segmented journal轮转/prune、oversized gap、single/multipage coverage、空future page及publish失败不扩大授权。真实三语言count/oversized、延迟external replay ACK两窗口、terminal replay ACK自然EOF均执行，并比较journal union/event_id/唯一terminal；不是旧tiny-window ACK后即退出。

仍缺：#120新增retention replay coverage ACK与pending ACK后第一oversized gap已发现真实反例，正由ACK专项验证；journal换代/generation及发送失败沿具名合同检查。旧page ACK绿色不关闭新窗口，最终源复审仍待。

## 权限与敏感信息

本地 SDK 不引入线上权限门控。继续保持框架自动行为 permission authority，凭据不进入日志/合同/错误；资源 gate 仅防止错误owner和无意数据丢失。

## 用户侧投影

TypeScript/Python/Java 对已接受能力暴露惯用接口，handle/error/value 必须无损；不能用成功默认值或丢字段改变权威事实。

## 场景处置清单

主场景mapped只表示模型已展开；关联family Behavior仍needs_review，具体Evidence按支持关系与原epoch读取。剩余义务：#120新增retention replay coverage ACK与pending ACK后第一oversized gap已发现真实反例，正由ACK专项验证；journal换代/generation及发送失败沿具名合同检查。旧page ACK绿色不关闭新窗口，最终源复审仍待。

## 未展开项

组级主路径mapped表示入口、唯一权威、生命周期与具名失败义务已建模；41个family Behavior仍needs_review，开放Finding及最终交付分别保留。旧候选的八crate临时patch与95ecpin保持原运行epoch；当前Host已pin published b909并移除patch，但最终nightlyprofiles/唯一合同生成与SDKwhole源码门禁仍待。frameworkmain交付证据不替SDK精确main/CI。完整Rust inventory仅遥测；accepted字段必须由唯一生成器明确裁决。

#120新增retention replay coverage ACK与pending ACK后第一oversized gap已发现真实反例，正由ACK专项验证；journal换代/generation及发送失败沿具名合同检查。旧page ACK绿色不关闭新窗口，最终源复审仍待。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，family needs_review和开放Finding保留，主场景mapped不扩运行证据。

## 建模完成与验证状态区别

组级主路径已按入口、唯一权威、生命周期和具名失败义务完成映射，故主场景mapped。family Behavior仍needs_review，已知open Finding与有限证据保留；mapped不等于verified、所有lens covered或最终交付。剩余事项采用明确合同/故障原因，不以无限未枚举组合拒绝建模。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
