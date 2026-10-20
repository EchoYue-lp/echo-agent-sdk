---
{
  "schema_version": 1,
  "id": "finding.sdk-sandbox-cleanup-order",
  "kind": "finding",
  "type": "implementation_bug",
  "status": "resolved",
  "severity": "high",
  "primary_focus": "failure_concurrency",
  "focus": [
    "time_lifecycle",
    "contract_evidence",
    "data_durability"
  ],
  "boundary_ref": "boundary.sdk-extension-bridge",
  "behavior_refs": [
    "behavior.sdk-extension-bridge"
  ],
  "rule_refs": [
    "rule.framework-runtime-authority"
  ],
  "evidence_refs": [
    "evidence.sdk-cancellation-observation"
  ],
  "audit_refs": [
    "audit.sdk-cancellation-observation-rereview"
  ],
  "decision_refs": [],
  "repair_evidence_refs": [
    "evidence.sdk-cancellation-observation"
  ],
  "verification_evidence_refs": [
    "evidence.sdk-cancellation-observation"
  ],
  "rereview_audit_refs": [
    "audit.sdk-cancellation-observation-rereview"
  ],
  "discovered_at": "30d7e219cc7d3aef0f75a19217c82898c363f50d"
}
---

# cleanup复用已取消Run token不能证明先于terminal完成

## 触发条件与影响

当前源码路径：`echo-sdk-host/src/core_profile/extension_bridge.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

connection scoped boundedcleanup与迟延副作用标记反例已编写待执行。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

cleanup复用已取消Run token不能证明先于terminal完成。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 2026-10-19：当前具名缺陷本地闭合

原cleanup复用已取消Run token已由公开69b6c17f8eb766fe8cdd0e2c878369f1bed268d6采用connection-scoped bounded cleanup修复；当前原Sandbox test实收cancel后held无terminal/cleanup0，再同原Responder真实Cancelled，原250ms cleanup与10s finalWait/cleanup1/unregister通过。旧fixture永不回答原callback而期待terminal是oracle缺物理事实，不能充作该生产修复before。当前完整Rust bridge35、同新Host三语Tool/Sandbox消费与原独立复审支持既有三链的有限resolved；不把harness kill当自然退出或foreign receipt。
