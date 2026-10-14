---
schema_version: 1
id: finding.sdk-accepted-source-continuity
kind: finding
type: intent_gap
status: open
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- contract_evidence
boundary_ref: boundary.sdk-product
behavior_refs:
- behavior.sdk-family-value
- behavior.sdk-family-invoke
- behavior.sdk-family-task
- behavior.sdk-repository-ownership
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-accepted-source-continuity-gate
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 明确accepted源/kind连续性阻写

## 问题

b909全28profile producer正确拒12旧canonical source/kind缺失：2obsoleteSnapshotownerfield、7Securitycanonical moved/rootalias仍保、3TaskContextpublicfield转getter。不能把根alias/无SDKconsumer/Catalogdone当自动retire或完整Rustinventoryparity目标。

## 触发条件与影响

完整update读取旧acceptedseed，membership由exactsource/kind唯一判断；同名getter与field不同key。正确exit1/noartifactswrite，不得旁路门禁默认丢旧承诺。

## 证据

原12closure保旧entry/signature/classification/route/28profiles/loghash。proposed文稿仅推荐一次有限兼容选择，尚未humanapproved；无新产品API/第二authority。

## 处理记录

等待根完整审阅+一次人的兼容裁决，协议owner唯一exactmigrationtable/ADR随后执行；本Finding保持open。既往sourcecontinuitymerge历史另对象，不借旧wave2closure关闭此次source/kind迁移。

## Task三字段证据界限

现methods.rs无完整TaskSubagentContext三字段专用reverseDTO。真实Hostrun_id()→loadsnapshot、waived_dependency_ids()→依赖skip提示、delegation_policy()→DispatchRequest；Task三语言四场景不能证明完整context值直接给CustomAgent。拟保这些原生语义现薄映射/getterRustHostonly，授权身份迁移后定向补现路径字段消费，不新增wire/API。2nonSerdeSnapshot Rustowner字段的旧wire_value身份仍需人明确退役决定，不能无consumer自动删。
