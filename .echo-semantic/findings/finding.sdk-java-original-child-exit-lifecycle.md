---
schema_version: 1
id: finding.sdk-java-original-child-exit-lifecycle
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: time_lifecycle
focus:
- failure_concurrency
- contract_evidence
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-java-host-early-exit-repair
- evidence.sdk-java-host-early-exit-verification
audit_refs:
- audit.sdk-java-host-early-exit-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-java-host-early-exit-repair
verification_evidence_refs:
- evidence.sdk-java-host-early-exit-verification
rereview_audit_refs:
- audit.sdk-java-host-early-exit-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Java原Host退出与late/closingsubscriber结算

## 问题

原实际child退出但officialinitialize仍pending；existing/late/closing期间新publisher未收到明确终态。

## 触发条件与影响

原child早status7退出；实际transportclose/heldcleanup和lateevent/update订阅窗口。

## 证据

sourceb65dea/test02ac exact52payload/59组合7XML0skip independentPASS0C0I0M；原真实失败与55/57epochs完整保。

## 处理记录

三链精确本地resolved。officialoriginalProcess唯一owner，私有exitobserver不新状态权威。purestdoutEOF childalive无publicofficialhook未测，不能claim所有EOF闭合；wholelanguage/完整generatedcontract/SDKmain另验。
