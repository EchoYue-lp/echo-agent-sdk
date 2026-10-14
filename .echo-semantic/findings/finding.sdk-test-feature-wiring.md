---
schema_version: 1
id: finding.sdk-test-feature-wiring
kind: finding
type: implementation_bug
status: resolved
severity: medium
primary_focus: contract_evidence
focus:
- contract_evidence
boundary_ref: boundary.sdk-product
behavior_refs:
- behavior.sdk-repository-ownership
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-feature-wiring-repair
- evidence.sdk-feature-wiring-verification
audit_refs:
- audit.sdk-feature-wiring-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-feature-wiring-repair
verification_evidence_refs:
- evidence.sdk-feature-wiring-verification
rereview_audit_refs:
- audit.sdk-feature-wiring-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 测试依赖与feature可用性接线

## 问题

default/core/bridge/noBridge各真实feature组合有缺测试依赖、import/cfg死代码/unused警告，不是运行时能力缺失。

## 触发条件与影响

官方实际组合编译/测试失败或-Wdenied；旧stderr/raw全部保，不能以强启所有features/allow或关tests绕过。

## 证据

6exactfilespairs与所有manifestreceiptsSHA匹配，各samefeaturefocusedgate0warnings与独立PASS三链齐备。Lock/bridge/TaskContext生产prefix不变。

## 处理记录

本地testcompile/cfg具体bugresolved，successivefeatureepochs记录明确；最终reviewed14Rustmatrix正在运行，sourcefreeze/newHostlanguage/main仍待，不自动全family/lensverified。
