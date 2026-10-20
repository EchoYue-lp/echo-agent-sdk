---
schema_version: 1
id: audit.sdk-gap-watermark-host-rereview
kind: audit
boundary_ref: boundary.sdk-events-recovery
lens: failure_concurrency
freshness: examined
revision: 56929154c1851473e9e4837901b610cecf001b2d
finding_refs:
- finding.sdk-gap-ack-replay-watermark
challenges:
  actual-source-and-consumer-boundary:
    revision: 56929154c1851473e9e4837901b610cecf001b2d
    source_refs:
    - docs/sdk/governance/evidence/sdk-gap-watermark-independent-review/receipt.json
    - docs/sdk/governance/evidence/sdk-gap-watermark-independent-review/original-test-all-repaired.log
    - docs/sdk/governance/evidence/sdk-gap-watermark-host-red/receipt.json
    - docs/sdk/governance/evidence/sdk-gap-watermark-host-red/SHA256SUMS.json
    - docs/sdk/governance/evidence/sdk-gap-watermark-host-red/source-manifest.json
    - docs/sdk/governance/evidence/sdk-gap-watermark-host-red/events-fixed.rs
    - docs/sdk/governance/evidence/sdk-gap-watermark-host-red/events-regressed.rs
    - docs/sdk/governance/evidence/sdk-gap-watermark-host-red/red-host.log
    - docs/sdk/governance/evidence/sdk-gap-watermark-host-red/green-host.log
    - echo-sdk-host/src/core_profile/events.rs#fn acknowledge
    - echo-sdk-host/src/core_profile/events.rs#fn publish_replay
    - echo-sdk-host/src/core_profile/events.rs#fn resume_after
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/tests/core_profile_e2e.rs#ack_window_snapshot_ack_resumes_strictly_after_watermark
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/SHA256SUMS.json
    evidence_refs:
    - evidence.sdk-gap-watermark-host-repair
    - evidence.sdk-gap-watermark-host-verification
  named-settlement-events-recovery:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
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
    evidence_refs:
    - evidence.sdk-ack-recovery-verification
    - evidence.sdk-gap-watermark-host-verification
    - evidence.sdk-events-resources-finite-observation
  named-settlement-family-event-replay:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/events.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-protocol/src/event.rs
    - echo-sdk-host/tests/core_profile_e2e.rs
    evidence_refs:
    - evidence.sdk-ack-recovery-verification
    - evidence.sdk-gap-watermark-host-verification
    - evidence.sdk-events-resources-finite-observation
---

# gap-watermark-host-rereview

## 审查范围

host_actor_delta_review实际完整events/sourcecallers originalPASS/action0，12unit与4Hostcaller原green；新27档/两个tar216+552hash/gitshow30d/singleline/recoveredtest红绿纯证据复核实际PASS。所有当前events e4hash精确，不借clientonlyreview。

## 已检查故障假设

旧wholecounts覆盖后继源码、typedwrapper丢字段、默认成功掩失败、callback mock替真实Host、跨epoch错误借用。

## 实际实现路径与证据

具名源码、原receipt与rawhash按引用读取；scope限定结论不扩为全部SDK审查。

## 问题记录

见独立原结论及关联Finding；不同缺陷不继承此pass。

## 残余风险

当前最终合同生成、无patchpin与whole门禁/main单独待验。

## 未检查项

scope外操作、任意组合与后继Channelforward新实现。

## 本次 current_source_rereview

这是独立reader现在实际重新读取源码/原故障假设后产生的新业务复审，未把原CodeAudit降为receipt审计。结论回收观测时间UTC `2026-10-13T23:17:58.622944+00:00`（不是原运行时间或原review开始时间），reader `framework_final_review`，当前SDK候选Git `56929154c1851473e9e4837901b610cecf001b2d`（source17cf）。

actual新当前源码业务复审PASS0：eventse4/handlerfa6/coreE2Ebe8，12strongunits与single/multi replaypage/current成功publication/失败无ACK授权/closingcaller均实际读，水位max与原唯一journal送事实保持。原snapshot2live2单变量旧机制反例不变成旧30dwholebuild；当前14/newbb4f语言覆盖的具体case已核。

真实当前SDK source_refs在上方追加，逐569Gitblob读取/currentSHA完全一致/锚点存在；新14Rust与bb4f完整三语632input/54build实际scope对应。原历史审查正文、原source marker `source:pending`、旧inputHash/raw/time不变，原whole3ea/9e缺失仍不造整树；这个新观察不宣称旧运行发生Git569。当前变化的调用方/feature/owner路径已在本次独立源码复审核，不由只存在锚点或count推PASS。

原支持范围、旧failure未知与limits保持；只追加这五故障的当前有界afterproof，特别ordinaryTools这里只承接Clinical，不扩大另外四族新复审。12acceptedsource迁移、5秒物理结算、依赖50warning规则仍未人批，Linux/Windows/main/fullgeneration另验；statusexamined不表示全无缺陷，41/coverage/closure/Finding状态不自动改变。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-events-recovery`：有限named单元结算；证据`evidence.sdk-ack-recovery-verification`、`evidence.sdk-gap-watermark-host-verification`、`evidence.sdk-events-resources-finite-observation`。
- `behavior.sdk-family-event-replay`：有限named单元结算；证据`evidence.sdk-ack-recovery-verification`、`evidence.sdk-gap-watermark-host-verification`、`evidence.sdk-events-resources-finite-observation`。
