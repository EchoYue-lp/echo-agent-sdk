---
{
  "schema_version": 1,
  "id": "finding.sdk-lsp-response-fidelity",
  "kind": "finding",
  "type": "implementation_bug",
  "status": "open",
  "severity": "high",
  "primary_focus": "contract_evidence",
  "focus": [
    "state_authority",
    "failure_concurrency",
    "time_lifecycle"
  ],
  "boundary_ref": "boundary.sdk-integrations",
  "behavior_refs": [
    "behavior.sdk-integrations"
  ],
  "rule_refs": [
    "rule.framework-runtime-authority"
  ],
  "evidence_refs": [],
  "audit_refs": [],
  "decision_refs": [],
  "repair_evidence_refs": [],
  "verification_evidence_refs": [],
  "rereview_audit_refs": [],
  "discovered_at": "30d7e219cc7d3aef0f75a19217c82898c363f50d"
}
---

# 上游 LSP 响应投影丢字段或默空

## 问题

framework hover range、合法 nullable result、Completion optional kind 和 malformed 响应解析失真。独立 framework 切片已有红绿，复审新增 signature/tuple 对象缺口，SDK 必须保持薄适配并补最终 consumer 证据。

## 触发条件与影响

实际主路径 `echo-sdk-host/src/core_profile/facade/integrations.rs`；具体证据范围与处理历史见 `docs/sdk/governance/finding-ledger.md`。

## 证据

修复反例与 focused 命令仍需绑定最终 source/revision 和独立复审；不把候选、局部绿色或 catalog 标记当全能力闭合。

## 处理记录

本 Finding 保持 open。最终 repair、verification、rereview 与 consumer 证据闭合后再变更状态；不影响历史外部 Issue 状态。
