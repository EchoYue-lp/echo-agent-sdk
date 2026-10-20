---
schema_version: 1
id: audit.sdk-ack-recovery-rereview
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: failure_concurrency
freshness: examined
revision: 56929154c1851473e9e4837901b610cecf001b2d
finding_refs:
- finding.sdk-ack-submission-consumption
challenges:
  actual-source-and-consumer-boundary:
    revision: 56929154c1851473e9e4837901b610cecf001b2d
    source_refs:
    - docs/sdk/governance/evidence/sdk-ack-recovery-windows/review-final-v4-source-manifest.json
    - docs/sdk/governance/evidence/sdk-ack-recovery-windows/evidence-sha256.json
    - docs/sdk/governance/evidence/sdk-ack-recovery-windows/REVIEW-DELTA.md
    - docs/sdk/governance/evidence/sdk-successor-model-reviews/ack-exact-slice.json
    - sdks/typescript/src/client.ts
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/java/src/main/java/com/echoagent/sdk/BoundedPublisher.java
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
    evidence_refs:
    - evidence.sdk-ack-recovery-repair
    - evidence.sdk-ack-recovery-verification
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

# ack-recovery-rereview

## 审查范围

framework_final_review已完成ACKv4 exactslice PASS/action0，原Important2与fixtureMinor全部闭合；原输入/失败尝试分别归档。仅本scope，不扩任意journal failure/后继Channel路径。

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

当前SDK真实Git`56929154c1851473e9e4837901b610cecf001b2d`/source17cf，独立reader`framework_final_review`确实重新读取原业务故障假设、当前路径/变化owner/cfg/aliases和现14/newbb4f强场景，消息回收UTC`2026-10-14T00:39:15.355735+00:00`不是原运行时间。结果：**PASS / 0C0I0M（本次有界源码业务复审）**。原CodeAudit仍业务角色，不降为receiptAudit；原正文/历史sourcemarker`source:pending`/旧rawepoch保。

本次已检查：Retentiongap requestcursor→snapshot connectedclosure / ordinarypages merge orderindependent;empty/disconnected/interior cursors never authorize；Actualliveinclusive512ranges do not fill pendingjump gaps; pendingACKforwardgap can arrive but never commits/discards before officialsuccess; failedsendqueue retained。当前函数定位：observeReplay；acknowledge；_replay_ranges；pending_ack_sequences；observeReplay；replayCoversCursor；public CompletionStage<Void> notify；publishGap；publishEvent。实际afterproof范围：AckSubmissionTest / TS12/PY23/J18 cases;newfull publicgap andheldCompletion routing, not physicalprune3native。

上方追加source_refs逐569Gitblob=currentfileSHA精确，现14/newbb4f632+54 sourcecut实际范围匹配。原FaultHypothesis文本保留并在本次业务代码复核，源码变化闭包按原scope检查；statusexamined不等全无缺陷，supports/limits/41coverage/closure不提升。Framework业务源属于独立publishedGitb909，对native工具用既有pin/buildmanifest定位，不将framework文件伪置SDKtree。原历史整仓3ea/9e缺失仍未知，不把原run重新标为569。full28gen/三兼容人选/LinuxWindows/main仍分开。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-events-recovery`：有限named单元结算；证据`evidence.sdk-ack-recovery-verification`、`evidence.sdk-gap-watermark-host-verification`、`evidence.sdk-events-resources-finite-observation`。
- `behavior.sdk-family-event-replay`：有限named单元结算；证据`evidence.sdk-ack-recovery-verification`、`evidence.sdk-gap-watermark-host-verification`、`evidence.sdk-events-resources-finite-observation`。
