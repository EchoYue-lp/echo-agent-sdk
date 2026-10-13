---
schema_version: 1
id: audit.sdk-clinical-trials-total-count-request-rereview
kind: audit
boundary_ref: boundary.sdk-integrations
lens: contract_evidence
freshness: examined
revision: source:pending
finding_refs:
- finding.sdk-clinical-trials-total-count-request
challenges:
  actual-source-and-consumer-boundary:
    revision: source:pending
    source_refs:
    - docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/manifest.json
    - docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/SHA256SUMS.json
    - docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/tool-comparison.json
    - docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/host-build-receipt.json
    - docs/sdk/governance/evidence/sdk-successor-model-reviews/ordinary-tools-b909.json
    - docs/sdk/governance/evidence/tool-value-effects/README.md
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
