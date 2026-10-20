---
schema_version: 1
id: finding.sdk-ack-submission-consumption
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- failure_concurrency
- time_lifecycle
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-ack-recovery-repair
- evidence.sdk-ack-recovery-verification
audit_refs:
- audit.sdk-ack-recovery-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-ack-recovery-repair
verification_evidence_refs:
- evidence.sdk-ack-recovery-verification
rereview_audit_refs:
- audit.sdk-ack-recovery-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# ACK提交前消费的精确缺陷

## 问题

旧客户端失败/callercancel/pending并发提前剪prefix或提交水位，错误丢消费事实。

## 触发条件与影响

三语言官方ACP实际提交/失败，pending恢复后live/gap与缺失interior误授权；具体输入、oldred/currentgreen分别保原epoch。

## 证据

v4同最终fixture真实4行为fail→23PythonACKgreen；三语言combined65/53/35及真实Hostheldcompletion六记录、独立Important2/fixtureMinor pass0由repair/verification/rereview引用定位。物理prune非三语言E2E，不伪称。

## 处理记录

该本地ACK提交消费缺陷resolved；仅已提交成功才消费，pending仅容后续不授权空洞，失败保原prefix。Hostjournal大scope、最终publishedpin/globalgate/main和外部Issue120独立，不自动关闭其它Finding。
