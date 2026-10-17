---
schema_version: 1
id: evidence.sdk-rag-unicode-character-count-candidate
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- echo-sdk-host/src/core_profile/facade/tools.rs
- docs/sdk/governance/evidence/rag-project-rules/receipt.json
- docs/sdk/governance/evidence/rag-project-rules/README.md
- echo-sdk-host/src/core_profile/facade/tools.rs#RagChunkDocumentTool
- sdks/shared/testing/rag_project_rules_fixture.py
- sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
- docs/sdk/governance/evidence/rag-project-rules-scalar-delta/published-framework-source/echo-tools/src/rag.rs#let
  char_count = chunk.chars().count()
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
- docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
- docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
- docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
- docs/sdk/governance/evidence/sdk-language-factory-final/SHA256SUMS.json
supports: []
limitations:
- Open candidate until exactrepair/verify/rereview receipts are bound
- Currentold0435 focused results do not prove newD/SDKsource
- No new user productdecision or extraacceptedpublicidentity
---

# rag-unicode-character-count-candidate

## 支持的结论

原13byte中文emoji被报char_count13而标量4；preview70中文截断单位错误。框架D两个字段已oldred2→green2且sourceindependentPASS，当前0435原native不覆盖D；最终Dpublishedpin+新consumer待归档。 原事实由root/所属lane具名报告与所引归档定位，不伪造本模型lane运行。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Open candidate until exactrepair/verify/rereview receipts are bound；Currentold0435 focused results do not prove newD/SDKsource；No new user productdecision or extraacceptedpublicidentity。

## 本次 current_source_rereview

这是独立reader现在实际重新读取源码/原故障假设后产生的新业务复审，未把原CodeAudit降为receipt审计。结论回收观测时间UTC `2026-10-13T23:17:58.622944+00:00`（不是原运行时间或原review开始时间），reader `sdk_capability_acceptance_audit`，当前SDK候选Git `56929154c1851473e9e4837901b610cecf001b2d`（source17cf）。

actual新当前源码业务复审PASS0：frameworkb909Ragd7/Hosttools77，Scalar4/4/3及70/200/201 preview/truncated、32calls/13realembeddingPOST/orderedrank/sharedinput原strongoracle都读；JavaNormalizeonly/testroot变化核为非行为改变，不能旧0435bytecount当正确。

真实当前SDK source_refs在上方追加，逐569Gitblob读取/currentSHA完全一致/锚点存在；新14Rust与bb4f完整三语632input/54build实际scope对应。原历史审查正文、原source marker `source:pending`、旧inputHash/raw/time不变，原whole3ea/9e缺失仍不造整树；这个新观察不宣称旧运行发生Git569。当前变化的调用方/feature/owner路径已在本次独立源码复审核，不由只存在锚点或count推PASS。

Framework源为独立publishedGit b909，SDKtree不是frameworktree；引用SDK内已有framepin/冻结源码/provenance与实际代码原Gitblob：`b9099558620f5f9693c736c85c214db14f62a8cb:echo-tools/src/rag.rs` SHA`d7dacc2133e935b982237807cbb13b7b3de7d11cbdb85d4662793f357bd77f6f`。不把跨仓路径写成SDK源码引用。

原支持范围、旧failure未知与limits保持；只追加这五故障的当前有界afterproof，特别ordinaryTools这里只承接Clinical，不扩大另外四族新复审。12acceptedsource迁移、5秒物理结算、依赖50warning规则仍未人批，Linux/Windows/main/fullgeneration另验；statusexamined不表示全无缺陷，41/coverage/closure/Finding状态不自动改变。
