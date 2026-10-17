---
schema_version: 1
id: map.sdk-events-recovery
kind: capability_map
title: 事件、ACK、replay、持久化
risk: high
observed_at: cd5b78467f77ac28919cf9dda6877662bed744f4
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
- evidence.sdk-events-resources-finite-observation
finding_refs:
- finding.sdk-ack-submission-consumption
- finding.sdk-gap-ack-replay-watermark
- finding.sdk-language-consumption-ack
- finding.sdk-gap-duplicate-ack-successor-observation
audit_refs:
- audit.sdk-ack-recovery-rereview
- audit.sdk-gap-watermark-host-rereview
- audit.sdk-events-resources-finite-trigger-input
- audit.sdk-events-resources-finite-result-side-effect
- audit.sdk-events-resources-finite-state-authority
- audit.sdk-events-resources-finite-data-durability
- audit.sdk-events-resources-finite-time-lifecycle
- audit.sdk-events-resources-finite-failure-concurrency
- audit.sdk-events-resources-finite-permission-external
- audit.sdk-events-resources-finite-contract-evidence
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
    status: mapped
    source_refs:
    - echo-sdk-host/src/core_profile/events.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-protocol/src/event.rs
    - echo-sdk-host/tests/core_profile_e2e.rs
    behavior_refs:
    - behavior.sdk-family-event-replay
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-events-resources-finite-observation
    review_scope: Accepted event-replay behavior/uniqueauthority/lifecycle/failure
      covered by named source review and original/new consumer proof. Fullgeneration/main
      and physicalB remain explicit, no infinite combination requirement.
  finite-events-resources-trigger-input:
    status: mapped
    source_refs: &id001
    - echo-sdk-host/src/core_profile/events.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/wire.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/bounded_stdio.rs
    - echo-sdk-host/src/config.rs
    - echo-sdk-protocol/src/event.rs
    behavior_refs:
    - behavior.sdk-events-recovery
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-events-resources-finite-observation
    audit_refs:
    - audit.sdk-events-resources-finite-trigger-input
    review_scope: Actual original69b source audit; unchanged source restored in cd5;
      finite livejournal native errors/isolation/restore added. Known physicalB and
      current mutableA excluded.
  finite-events-resources-result-side-effect:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-events-recovery
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-events-resources-finite-observation
    audit_refs:
    - audit.sdk-events-resources-finite-result-side-effect
    review_scope: Actual original69b source audit; unchanged source restored in cd5;
      finite livejournal native errors/isolation/restore added. Known physicalB and
      current mutableA excluded.
  finite-events-resources-state-authority:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-events-recovery
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-events-resources-finite-observation
    audit_refs:
    - audit.sdk-events-resources-finite-state-authority
    review_scope: Actual original69b source audit; unchanged source restored in cd5;
      finite livejournal native errors/isolation/restore added. Known physicalB and
      current mutableA excluded.
  finite-events-resources-data-durability:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-events-recovery
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-events-resources-finite-observation
    audit_refs:
    - audit.sdk-events-resources-finite-data-durability
    review_scope: Actual original69b source audit; unchanged source restored in cd5;
      finite livejournal native errors/isolation/restore added. Known physicalB and
      current mutableA excluded.
  finite-events-resources-time-lifecycle:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-events-recovery
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-events-resources-finite-observation
    audit_refs:
    - audit.sdk-events-resources-finite-time-lifecycle
    review_scope: Actual original69b source audit; unchanged source restored in cd5;
      finite livejournal native errors/isolation/restore added. Known physicalB and
      current mutableA excluded.
  finite-events-resources-failure-concurrency:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-events-recovery
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-events-resources-finite-observation
    audit_refs:
    - audit.sdk-events-resources-finite-failure-concurrency
    review_scope: Actual original69b source audit; unchanged source restored in cd5;
      finite livejournal native errors/isolation/restore added. Known physicalB and
      current mutableA excluded.
  finite-events-resources-permission-external:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-events-recovery
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-events-resources-finite-observation
    audit_refs:
    - audit.sdk-events-resources-finite-permission-external
    review_scope: Actual original69b source audit; unchanged source restored in cd5;
      finite livejournal native errors/isolation/restore added. Known physicalB and
      current mutableA excluded.
  finite-events-resources-contract-evidence:
    status: mapped
    source_refs: *id001
    behavior_refs:
    - behavior.sdk-events-recovery
    rule_refs:
    - rule.framework-runtime-authority
    evidence_refs:
    - evidence.sdk-events-resources-finite-observation
    audit_refs:
    - audit.sdk-events-resources-finite-contract-evidence
    review_scope: Actual original69b source audit; unchanged source restored in cd5;
      finite livejournal native errors/isolation/restore added. Known physicalB and
      current mutableA excluded.
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

原569阶段缺口（历史，当前见后继有限审查）：#120新增retention replay coverage ACK与pending ACK后第一oversized gap已发现真实反例，正由ACK专项验证；journal换代/generation及发送失败沿具名合同检查。旧page ACK绿色不关闭新窗口，最终源复审仍待。

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

## 原历史测试出处

56929154c1851473e9e4837901b610cecf001b2d 中保留 sdks/java/src/test/java/com/echoagent/sdk/ExternalAckHostTest.java。这些原测试和具名义务仍属于其已保存源码/收据，不伪装成cd5包含的源码；本次group审查由真实runtime69b、正式cd5证明和各原slice审查支持。

## 当前有界审查与明确剩余

本对象观察绑定真实cd5，所列原运行源码69b在cd5逐Gitblob相同；新Session/RunA实施已变handler/handles/state，不能以本观察宣称最新mutable行为已验证。八份audit.sdk-events-resources-finite-*记录ACK原作者实际只读审查；作者身份不冒独立review，Host events/stream/client各原独立scope和新liveJournal当前Tool独审范围明确保留。

原#120、ACK消费与stream具体缺陷三链resolved继续有效，不重复造关闭对象或操作外部Issue。新增liveJournal场景证明settledRun且Host/connection仍活时目录换代两次明确replay_unavailable/after_delay、原Runreceipt/peerSession完整page不变，restore同inode/原两fileSHA后完整page相同；三语言12/12/9零skip。它是既有正确失败路径补证，不是生产redgreen；不扩为activewritepartialcrash或任意网络故障。

已接受event-replay及共享资源stream的入口、单一authority、正常生命周期/具名错误和恢复已有finiteexamined闭包；映射仅boundedmodel，Behavior保持needs_review，八个全局风险和baseline不因此全covered。宽cleanup与5秒B保持原Finding理由，12身份/完整生成/平台/main独立；其它resourcefamily/sourceoperation未审方法不从这条共享owner路径泛推全部无损。


## 后继722 Linux CI局部开放记录

`finding.sdk-gap-duplicate-ack-successor-observation` 当前open：现有Python duplicate ACK测试的观察基线未等前次合法ACK产生的paused successor gap，Linux真实4!=3及Root受控同Hostgap4 beforeduplicate因果属于测试oracle；不是重复ACK产品效应/序号权威变化，原已闭水位/消费义务不重开。

本记录发现观察基准722及PRmerge5d407仅定位后继测试/CI材料，Map原observed_at、所有旧scene/source_refs/status、行为义务和原Evidence审查仍属原epoch；新增只本owner的Finding引用，不复制到所有Map，不提升或失效全8视角/41族。原日志目前Admin完整Hash定位，正式Git归档后再补当期来源，不伪旧Git含未来raw；旧56/4是原切点，安装这三新具体缺陷后56resolved/7open，B3共享物理B与12连续性不改变。


## 正式CI保存材料的当期观察（f68）

本段当期观察Git `f68cb49bf421ecdb71dfa4775495c363e31b3324`仅定位正式保存的722运行材料，不把运行时间或源码测试执行标成f68。原发现点`722d09b1a0837f170d4af19cbc3e417d2131b5f3`、PR实际merge5d407、候选所有原Map观察/scenes/status/锚点与对应测试源码SHA保持。正式来源逐路径与Hash如下，可从该Git读取：

- `docs/sdk/governance/evidence/sdk-722-ci-results/README.md`，Git f68 SHA256 `dcd6a9e4922a05527b189a2c03108a3ac73936d5107d022600dada1e570f0b27`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/manifest.json`，Git f68 SHA256 `4eb02f38b55191bfa761cd892b667733dcb26d4462eb655cfd2a22cbd379443e`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/SHA256SUMS.json`，Git f68 SHA256 `c22f92784ad1874ffcb0bb70ad312965e50bda78dcd69871230b2073006f1e44`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/jobs/116172597308.log`，Git f68 SHA256 `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`。

正式完整八job范围为5success/3failure；TS336及quickstart通过，Python413通过/2失败/18subtests，Java未达。两个合同job原13连续性阻断与旧证明分别保留，不从CI fail泛化未知生产缺陷。PID原Linux[]唯一cause仍unknown，Darwin2.5925s排队原反例不是该CI唯一cause。bootstrap单input实施只原配置/顺序依据，新remote error结算尚未证明，不提前resolved。原四已闭Mac缺陷不重开、B/12未决保持。

原上文“第三repo工具交付权限待批准”属于候选准备时历史；Root后续已获得第三repo工具修复交付授权，实际工具修补/源码SHA/发布main与新CIjob结果尚未回报，本候选不冒已交付或官方原30d门禁已0。
