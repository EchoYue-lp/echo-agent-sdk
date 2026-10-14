---
schema_version: 1
id: audit.sdk-clinical-trials-total-count-request-rereview
kind: audit
boundary_ref: boundary.sdk-integrations
lens: contract_evidence
freshness: examined
revision: 56929154c1851473e9e4837901b610cecf001b2d
finding_refs:
- finding.sdk-clinical-trials-total-count-request
challenges:
  actual-source-and-consumer-boundary:
    revision: 56929154c1851473e9e4837901b610cecf001b2d
    source_refs:
    - docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/manifest.json
    - docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/SHA256SUMS.json
    - docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/tool-comparison.json
    - docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/host-build-receipt.json
    - docs/sdk/governance/evidence/sdk-successor-model-reviews/ordinary-tools-b909.json
    - docs/sdk/governance/evidence/tool-value-effects/README.md
    - echo-sdk-host/src/core_profile/facade/tools.rs#ClinicalTrialsSearchTool
    - sdks/shared/testing/tool_value_effect_fixture.py
    - docs/sdk/governance/evidence/sdk-count-authority-calibration/framework-main-b909-acceptance.json
    - docs/sdk/governance/evidence/sdk-successor-model-reviews/framework-database-rag-final-gates.json
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/source-build-manifest.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/SHA256SUMS.json
    evidence_refs:
    - evidence.sdk-ordinary-tools-b909-repair
    - evidence.sdk-ordinary-tools-b909-verification
---

# clinical-trials-total-count-request-rereview

## 审查范围

sdk_capability_acceptance_audit actual source/binary/75archive/15executed inputs + ordinary3native49rows independentfinitePASS0 exactfield/effect repair; Sandboxrace/fullSDK excluded.

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

actual新当前源码业务复审PASS0：publishedframeworkb909Clinical6b实际构造countTotal=true官方GET、fallibleas_u64缺失/负值/wrongtype明确Err不默认0，actualprovider total>=returned非固定386；Root14+bb4f新强ordinarytools/publiccaller已核。只复审原Clinical字段故障，不把ordinaryTools另外四族沿此audit冒新复审。

真实当前SDK source_refs在上方追加，逐569Gitblob读取/currentSHA完全一致/锚点存在；新14Rust与bb4f完整三语632input/54build实际scope对应。原历史审查正文、原source marker `source:pending`、旧inputHash/raw/time不变，原whole3ea/9e缺失仍不造整树；这个新观察不宣称旧运行发生Git569。当前变化的调用方/feature/owner路径已在本次独立源码复审核，不由只存在锚点或count推PASS。

Framework源为独立publishedGit b909，SDKtree不是frameworktree；引用SDK内已有framepin/冻结源码/provenance与实际代码原Gitblob：`b9099558620f5f9693c736c85c214db14f62a8cb:echo-tools/src/research/clinical_trials.rs` SHA`6bfe4c12a2ad77022c9426b9bf21b5e0ecad68c4df7fd1d7efe1ad28759ccd7d`。不把跨仓路径写成SDK源码引用。

原支持范围、旧failure未知与limits保持；只追加这五故障的当前有界afterproof，特别ordinaryTools这里只承接Clinical，不扩大另外四族新复审。12acceptedsource迁移、5秒物理结算、依赖50warning规则仍未人批，Linux/Windows/main/fullgeneration另验；statusexamined不表示全无缺陷，41/coverage/closure/Finding状态不自动改变。
