---
{
  "schema_version": 1,
  "id": "finding.sdk-inventory-acceptance-authority",
  "kind": "finding",
  "type": "authority_conflict",
  "status": "open",
  "severity": "high",
  "primary_focus": "contract_evidence",
  "focus": [
    "state_authority",
    "failure_concurrency",
    "time_lifecycle"
  ],
  "boundary_ref": "boundary.sdk-language-clients",
  "behavior_refs": [
    "behavior.sdk-language-clients"
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

# 生成分类擅自扩大 accepted identity 合同

## 问题

完整库存与 catalog 自动接受新增可路由身份；唯一 generator 必须以既有明确 accepted identity + kind 为来源，新身份只遥测。候选及反例验证进行中。

## 触发条件与影响

实际主路径 `echo-sdk-protocol/src/inventory.rs`；具体证据范围与处理历史见 `docs/sdk/governance/finding-ledger.md`。

## 证据

修复反例与 focused 命令仍需绑定最终 source/revision 和独立复审；不把候选、局部绿色或 catalog 标记当全能力闭合。

## 处理记录

本 Finding 保持 open。最终 repair、verification、rereview 与 consumer 证据闭合后再变更状态；不影响历史外部 Issue 状态。
