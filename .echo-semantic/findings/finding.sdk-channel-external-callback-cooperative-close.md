---
schema_version: 1
id: finding.sdk-channel-external-callback-cooperative-close
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- time_lifecycle
- failure_concurrency
boundary_ref: boundary.sdk-resources-streams
behavior_refs:
- behavior.sdk-family-channels
- behavior.sdk-family-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-channel-external-callback-cooperative-close-candidate
- evidence.sdk-channel-cooperative-two
audit_refs:
- audit.sdk-channel-external-callback-cooperative-close-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-channel-cooperative-two
verification_evidence_refs:
- evidence.sdk-channel-cooperative-two
rereview_audit_refs:
- audit.sdk-channel-external-callback-cooperative-close-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Channel提前close而外部callback未结算

## 问题

0435公开heldclose返回true时finally0，坏chunk先failed但callback未结束。新source取消exactinvocation并drain至真实EOF，不localabort外部driver；无terminal明确failed，actualanswer停止deadline。10units/Hostlib158当前绿为候选，strongnative新bin及独立scope待。

## 触发条件与影响

实际consumer路径与原raw回收在所引Evidence；未知/非法值不静默默认，适配边界不新增第二业务权威。

## 证据

原真实反例及已执行局部候选有自己的epoch；当时scope三链未齐保持open；当前精确本地闭合见后节，不借别处whole绿。

## 处理记录

用户已授权当前合同修复，无新产品裁决。仅生产owner切片修，模型lane不改业务/生成物；最终生成/pin/source三语/独立复审另验。

## 本地精确闭合

原真实before反例→该不可变Host强native行为证据及独立boundedPASS三链齐备，本缺陷resolved。原文准备态是历史切点；最终newpin/gen/allSDKgate另验，当前其它bridge/teardownPrefix不继承此结论。
