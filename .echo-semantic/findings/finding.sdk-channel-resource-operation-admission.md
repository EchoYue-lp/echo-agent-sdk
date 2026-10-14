---
schema_version: 1
id: finding.sdk-channel-resource-operation-admission
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- state_authority
- time_lifecycle
- contract_evidence
boundary_ref: boundary.sdk-resources-streams
behavior_refs:
- behavior.sdk-family-channels
- behavior.sdk-resources-streams
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-channel-resource-admission
audit_refs:
- audit.sdk-channel-resource-admission-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-channel-resource-admission
verification_evidence_refs:
- evidence.sdk-channel-resource-admission
rereview_audit_refs:
- audit.sdk-channel-resource-admission-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Captured Channel operation越过resource关闭

## 问题

旧capturedstart在resource tombstone之后仍启动原plugin，假closed而Running真实反例。

## 触发条件与影响

旧operation捕获原entry后与资源close竞争；真正frameworkplugin/nativeSessionruntime组合观察到墓碑后Running。

## 证据

0744epoch原red1→3实际units/Hostlib140/三语言各5；integ20c908/mod51a6/handles4562及127raw匹配，原host_actor_delta_review独立PASS/action0。

## 处理记录

同原resource RWauthority七ops读permit贯穿await，close写permit；wait后Registry再核、metadata不持permit允许重入。该精确bug本地resolved，后继approvedforward与全部cleanup owner不继承结论，最终pin/main独立。
