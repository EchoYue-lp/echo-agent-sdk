---
schema_version: 1
id: audit.sdk-chart-dimension-published-after-rereview
kind: audit
boundary_ref: boundary.sdk-integrations
lens: contract_evidence
freshness: examined
revision: source:pending
finding_refs:
- finding.sdk-chart-dimension-integer-loss
challenges:
  dimension_checked_integer_and_original_schema_route:
    revision: source:pending
    source_refs:
    - docs/sdk/governance/evidence/sdk-chart-published-pin/README.md
    - docs/sdk/governance/evidence/sdk-chart-published-pin/final-native-receipt.json
    - docs/sdk/governance/evidence/sdk-chart-published-pin/evidence-sha256.json
    - docs/sdk/governance/evidence/sdk-chart-published-pin/source-build-manifest.json
    - docs/sdk/governance/evidence/sdk-chart-published-pin/published-pin-provenance.json
    - docs/sdk/governance/evidence/sdk-chart-published-pin/native/before/before-counterexample-run.json
    - docs/sdk/governance/evidence/sdk-chart-published-pin/native/after/after-boundary-validation-run.json
    - docs/sdk/governance/evidence/sdk-chart-published-pin/native/after/comparison.json
    evidence_refs:
    - evidence.sdk-chart-dimension-published-after-repair
    - evidence.sdk-chart-dimension-published-after-verification
---

# Chart已发布精确整数after复审

## 审查范围

current_tool_boundary_review真实源码+SDKactual11delta审查PASS0C0I0M由root回收：be5checkedu32/schema/ToolManager/b909be5生产仅Chart变化/registrymenu等价、8locksGitpin/noPatch、55build/627runtime inputs、3consumerbefore4failureafter0强rawnormalized/8fileunchanged68packet。

## 已检查故障假设

越界asU32静默wrap/negative默认/类型错填默认，normalabsentdefault语义改变，schema与Toolexecute无损拒绝不同步，CLI/SDK薄adapter另parser或wildidentity接受。

## 实际实现路径与证据

由实际publishedpin/frameworkGitbe5源码与SDKoriginalroute/sourceBuild/同wire原生对照恢复；工作树bodyfreeze未最终，所以本Auditpending等待新Git，不伪old9db含新be5。

## 问题记录

具体dimensionI1三链闭合，不要求SDKmain先交付再关闭本地已修bug。

## 残余风险

shared61尚在运行/最后wholegates/accepted12source生成人未选/5sB另缺。

## 未检查项

未列任意chart维度产品语义/各imagebackend及全族所有字段。
