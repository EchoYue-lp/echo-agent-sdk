---
schema_version: 1
id: behavior.sdk-events-recovery
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
boundary: boundary.sdk-events-recovery
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
code_refs:
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
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-ack-recovery-verification
- evidence.sdk-gap-watermark-host-verification
finding_refs:
- finding.sdk-ack-submission-consumption
- finding.sdk-gap-ack-replay-watermark
- finding.sdk-language-consumption-ack
---

# 事件、ACK、replay、持久化

## 重要承诺

本能力的行为、唯一权威、生命周期、失败路径与无损合同必须有可复核证据。

## 当前行为

framework EventJournal唯一拥有committed envelope、journal identity和terminal；StreamDelivery只管理发送窗口与cursor。#120候选修复把snapshot/page已消费ACK的coverage推进到所覆盖水位，再续发严格更大的sequence；成功enqueue后才授权ACK，重复/未来ACK不退回水位。12个event unit包含真实segmented journal轮转/prune、oversized gap、single/multipage coverage、空future page及publish失败不扩大授权。真实三语言count/oversized、延迟external replay ACK两窗口、terminal replay ACK自然EOF均执行，并比较journal union/event_id/唯一terminal；不是旧tiny-window ACK后即退出。

## 期望行为

保持框架业务权威、Host寻址与资源ownership、三语言无损投影；所有已确认缺陷修复并取得最新反例验证。

## 触发、结果与副作用

源码入口与实际注册/分派路径由本 Map 的 source_refs 定位。输出为 typed response/error、framework committed event 或 generation-fenced handle；所有动态入口仍需逐场景验证。

## 失败、重试与恢复

framework EventJournal唯一拥有committed envelope、journal identity和terminal；StreamDelivery只管理发送窗口与cursor。#120候选修复把snapshot/page已消费ACK的coverage推进到所覆盖水位，再续发严格更大的sequence；成功enqueue后才授权ACK，重复/未来ACK不退回水位。12个event unit包含真实segmented journal轮转/prune、oversized gap、single/multipage coverage、空future page及publish失败不扩大授权。真实三语言count/oversized、延迟external replay ACK两窗口、terminal replay ACK自然EOF均执行，并比较journal union/event_id/唯一terminal；不是旧tiny-window ACK后即退出。

仍缺：journal identity替换/错误generation、正式pin重启replay及未列出的partial-write/terminal竞争组合仍待；不能从指定真实prune或enqueue失败推广所有持久化故障。#120 Finding仍open，需最终修复/验证/复审及精确main归档。

## 证据

具名候选运行范围由本组Map关联的child family Behaviors及其 `evidence.sdk-candidate-behavior-verification` 定位；该Evidence只supports具体family，不把其引用冒充本组完整通过。

所有状态保持 needs_review。具名断言的候选通过不证明能力全覆盖：SDK manifest 固定95ec66c6，八crate临时patch组合Task/ACP/LSP源码用于本地验证；最终Channel后继published revision、移除patch后的独立构建、精确main和远端CI仍待。完整Rust inventory仅遥测，最终accepted字段/退役必须由唯一生成器明确裁决。

journal identity替换/错误generation、正式pin重启replay及未列出的partial-write/terminal竞争组合仍待；不能从指定真实prune或enqueue失败推广所有持久化故障。#120 Finding仍open，需最终修复/验证/复审及精确main归档。

## 裁决记录

用户明确要求完成八项治理，三语言范围只以明确接受合同为准；已授权子智能体执行，主线程负责协调及验收。

## 当前观察、归档和后继边界

原fc63证据已保存为仓库内不可变历史收据，原观察epoch不可恢复为Git时不补造提交。原6711证据归档支持具名局部断言；原27个历史输入中tools.rs已变，具名测试body保持可对照；55个6711运行输入中tools.rs和Workflow已变更。新Workflow语义或其它后继场景必须绑定自己的来源/日志；不把旧整套418/TS240/Python300/Java264冒充最终当前通过。支持见历史审计Evidence及evidence.sdk-parent-driver-candidate-verification的限制，所有needs_review/开放Finding保留。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
