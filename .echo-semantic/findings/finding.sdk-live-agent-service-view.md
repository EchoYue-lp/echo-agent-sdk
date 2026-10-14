---
{
  "schema_version": 1,
  "id": "finding.sdk-live-agent-service-view",
  "kind": "finding",
  "type": "authority_conflict",
  "status": "open",
  "severity": "high",
  "primary_focus": "state_authority",
  "focus": [
    "time_lifecycle",
    "contract_evidence",
    "data_durability"
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

# set_store/set_llm_client后family继续使用旧captured authority

## 触发条件与影响

当前源码路径：`echo-sdk-host/src/core_profile/facade/memory.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

改live Agent读取；Store A→B/Workflow LLM回归待执行。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

set_store/set_llm_client后family继续使用旧captured authority。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。
