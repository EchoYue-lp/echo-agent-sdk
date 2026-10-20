---
schema_version: 1
id: evidence.sdk-rag-scalar-repair
kind: evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
source_refs:
- docs/sdk/governance/evidence/rag-project-rules-scalar-delta/after-receipt.json
- docs/sdk/governance/evidence/rag-project-rules-scalar-delta/baseline-continuity.json
- docs/sdk/governance/evidence/rag-project-rules-scalar-delta/comparison.json
- docs/sdk/governance/evidence/rag-project-rules-scalar-delta/SHA256SUMS.json
- docs/sdk/governance/evidence/rag-project-rules-scalar-delta/README.md
- docs/sdk/governance/evidence/sdk-successor-model-reviews/rag-scalar-b909.json
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
supports:
- behavior.sdk-family-rag
limitations:
- Specific Unicodecount/preview bug only
- Old0435bytecount receipt retained wrongfield observation, never acceptedcorrect
- No crashdurability/finalacceptedsourcecontinuity/globalSDK closure
---

# rag-scalar-repair

## 支持的结论

框架D/B909 Unicode标量char_count和preview截断两字段保原索引/searchauthority：原byte13/12/9错误→scalar4/4/3，70中文/200/201边界各native自己的oracle完全一致。真实Gitb909/noPatch ee9ff原生每语32calls/13embeddingPOST/1test0skip，其他原28结果与旧29语料相同，不增加embedding调用。65档逐SHA匹配、旧76 immutable，3真正embedding serverPID退出。独立source/native限定PASS/action0。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Specific Unicodecount/preview bug only；Old0435bytecount receipt retained wrongfield observation, never acceptedcorrect；No crashdurability/finalacceptedsourcecontinuity/globalSDK closure。

## 对原保存收据的当前审计观察

本对象observed_at现在指真正当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`，只定位已经保存的收据、输入源码副本与manifest的审计观察。原观测字段为`source:pending`；它若是source摘要或pending，均不被伪装为历史执行当时的可恢复整仓Git。当前观察不宣称原运行发生于Git569、不重跑或升级原runtimeepoch。

全部上列source_refs均从该Gitblob实际读取，与当前archive原字节SHA精确相同，原锚点存在；原日志时间、HostbinarySHA、sourcefreeze、错误/通过计数及正文来源保持。原body的epoch/signature/cause限制与supports均为原Behavior义务的历史有界支持，不能由此宣布当前源码verified；旧3ea/9e没有完整Git时仅有原保存收据可恢复，整仓原epoch仍不可恢复，不补造树。

原保存artifact中的具名元数据（完整值仍见原JSON，不改变）：`framework_revision=b9099558620f5f9693c736c85c214db14f62a8cb`；`host_sha256=ee9ff37598f4061b0cc33a79d9c58e1ad4481d6592607fb308a28b7b425fc621`；`source_contract=3cd835dbf40abf1d0aec04e02f3b51411631fcaa50ba7e5f0f2d2d2f500ed9a6`。

本次只附加观察说明；原正文/supports/limitations保留，原rawReceipt/ArchiveChecksum/运行时间未写入，任何未决unknown/失败仍不自动resolved。源码业务Audit不是此收据观察，另队列保持原revision与复审义务，不能偷换类型或scope。

## 本次 current_source_rereview

这是独立reader现在实际重新读取源码/原故障假设后产生的新业务复审，未把原CodeAudit降为receipt审计。结论回收观测时间UTC `2026-10-13T23:17:58.622944+00:00`（不是原运行时间或原review开始时间），reader `sdk_capability_acceptance_audit`，当前SDK候选Git `56929154c1851473e9e4837901b610cecf001b2d`（source17cf）。

actual新当前源码业务复审PASS0：frameworkb909Ragd7/Hosttools77，Scalar4/4/3及70/200/201 preview/truncated、32calls/13realembeddingPOST/orderedrank/sharedinput原strongoracle都读；JavaNormalizeonly/testroot变化核为非行为改变，不能旧0435bytecount当正确。

真实当前SDK source_refs在上方追加，逐569Gitblob读取/currentSHA完全一致/锚点存在；新14Rust与bb4f完整三语632input/54build实际scope对应。原历史审查正文、原source marker `56929154c1851473e9e4837901b610cecf001b2d`、旧inputHash/raw/time不变，原whole3ea/9e缺失仍不造整树；这个新观察不宣称旧运行发生Git569。当前变化的调用方/feature/owner路径已在本次独立源码复审核，不由只存在锚点或count推PASS。

Framework源为独立publishedGit b909，SDKtree不是frameworktree；引用SDK内已有framepin/冻结源码/provenance与实际代码原Gitblob：`b9099558620f5f9693c736c85c214db14f62a8cb:echo-tools/src/rag.rs` SHA`d7dacc2133e935b982237807cbb13b7b3de7d11cbdb85d4662793f357bd77f6f`。不把跨仓路径写成SDK源码引用。

原支持范围、旧failure未知与limits保持；只追加这五故障的当前有界afterproof，特别ordinaryTools这里只承接Clinical，不扩大另外四族新复审。12acceptedsource迁移、5秒物理结算、依赖50warning规则仍未人批，Linux/Windows/main/fullgeneration另验；statusexamined不表示全无缺陷，41/coverage/closure/Finding状态不自动改变。
