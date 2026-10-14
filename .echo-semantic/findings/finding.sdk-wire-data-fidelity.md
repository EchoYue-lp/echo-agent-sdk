---
{
  "schema_version": 1,
  "id": "finding.sdk-wire-data-fidelity",
  "kind": "finding",
  "type": "implementation_bug",
  "status": "open",
  "severity": "high",
  "primary_focus": "failure_concurrency",
  "focus": [
    "time_lifecycle",
    "contract_evidence",
    "data_durability"
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

# 特殊Map键、重复/nonstring键和畸形值被丢字段/默认空

## 触发条件与影响

当前源码路径：`sdks/typescript/src/wire.ts`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

TS __proto__与Python丢键已runtimeprobe复现；共享malformed corpus三语言red→green。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

特殊Map键、重复/nonstring键和畸形值被丢字段/默认空。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。
