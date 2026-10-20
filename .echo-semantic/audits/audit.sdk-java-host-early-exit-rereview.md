---
schema_version: 1
id: audit.sdk-java-host-early-exit-rereview
kind: audit
boundary_ref: boundary.sdk-language-clients
lens: time_lifecycle
freshness: examined
revision: 56929154c1851473e9e4837901b610cecf001b2d
finding_refs:
- finding.sdk-java-original-child-exit-lifecycle
challenges:
  actual-source-and-consumer-boundary:
    revision: 56929154c1851473e9e4837901b610cecf001b2d
    source_refs:
    - docs/sdk/governance/evidence/sdk-java-host-early-exit/README.md
    - docs/sdk/governance/evidence/sdk-java-host-early-exit/evidence-sha256.json
    - docs/sdk/governance/evidence/sdk-java-host-early-exit/closing-final-source-class-manifest.json
    - docs/sdk/governance/evidence/sdk-java-host-early-exit/real-child-before.log
    - docs/sdk/governance/evidence/sdk-java-host-early-exit/java-lifecycle-focused-closing-final.log
    - docs/sdk/governance/evidence/sdk-successor-model-reviews/java-original-host-exit.json
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java#class OwnedStdioTransport
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java#awaitForExit
    - sdks/java/src/test/java/com/echoagent/sdk/HostEarlyExitTest.java#newEventOrUpdateSubscriberDuringOwnedCleanupGetsNaturalCompletion
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/SHA256SUMS.json
    evidence_refs:
    - evidence.sdk-java-host-early-exit-repair
    - evidence.sdk-java-host-early-exit-verification
---

# java-host-early-exit-rereview

## 审查范围

sdkcapability actual59combination7XML/52payloads/sourceb65dea,test02ac finitePASS0C0I0M. Officialtransportartifact0.17 verified; livechildEOF excluded.

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

这是独立reader现在实际重新读取源码/原故障假设后产生的新业务复审，未把原CodeAudit降为receipt审计。结论回收观测时间UTC `2026-10-13T23:17:58.622944+00:00`（不是原运行时间或原review开始时间），reader `sdk_capability_acceptance_audit`，当前SDK候选Git `56929154c1851473e9e4837901b610cecf001b2d`（source17cf）。

actual新当前源码业务复审PASS0：clientb65/test02ac九currentcases且newJava320实际XML/源码/参数均核，official原Process/retainedfirstfailure/failedclose/latefeeds/closingnewpublisher/slots保持原owner，final59原faults不放宽。purestdoutEOFchildalive无publichook仍未证明；旧55/57epoch不重标。

真实当前SDK source_refs在上方追加，逐569Gitblob读取/currentSHA完全一致/锚点存在；新14Rust与bb4f完整三语632input/54build实际scope对应。原历史审查正文、原source marker `source:pending`、旧inputHash/raw/time不变，原whole3ea/9e缺失仍不造整树；这个新观察不宣称旧运行发生Git569。当前变化的调用方/feature/owner路径已在本次独立源码复审核，不由只存在锚点或count推PASS。

原支持范围、旧failure未知与limits保持；只追加这五故障的当前有界afterproof，特别ordinaryTools这里只承接Clinical，不扩大另外四族新复审。12acceptedsource迁移、5秒物理结算、依赖50warning规则仍未人批，Linux/Windows/main/fullgeneration另验；statusexamined不表示全无缺陷，41/coverage/closure/Finding状态不自动改变。
