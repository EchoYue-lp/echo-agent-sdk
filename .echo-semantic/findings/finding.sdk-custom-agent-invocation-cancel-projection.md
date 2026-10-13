---
schema_version: 1
id: finding.sdk-custom-agent-invocation-cancel-projection
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- time_lifecycle
- failure_concurrency
boundary_ref: boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-family-eval
- behavior.sdk-family-improve
- behavior.sdk-family-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-custom-agent-invocation-cancel-projection-candidate
- evidence.sdk-eval-improve-topology
audit_refs:
- audit.sdk-custom-agent-invocation-cancel-projection-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-eval-improve-topology
verification_evidence_refs:
- evidence.sdk-eval-improve-topology
rereview_audit_refs:
- audit.sdk-custom-agent-invocation-cancel-projection-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# CustomAgent丢Eval invocation取消上下文

## 问题

Evaltimeout1秒原nativePY10秒/TSJava13秒grace且callback未收到原token；proxy普通run_stream忽略已有traitcontext。现窄override复用frameworkinvocation取消/workspace、实际answer+declaredterminal双条件，Hostlib158已有绿；新三native证明待，不新helper自动接受。

## 触发条件与影响

实际consumer路径与原raw回收在所引Evidence；未知/非法值不静默默认，适配边界不新增第二业务权威。

## 证据

原真实反例及已执行局部候选有自己的epoch；当时scope三链未齐保持open；当前精确本地闭合见后节，不借别处whole绿。

## 处理记录

用户已授权当前合同修复，无新产品裁决。仅生产owner切片修，模型lane不改业务/生成物；最终生成/pin/source三语/独立复审另验。

## 本地精确闭合

原真实before反例→该不可变Host强native行为证据及独立boundedPASS三链齐备，本缺陷resolved。原文准备态是历史切点；最终newpin/gen/allSDKgate另验，当前其它bridge/teardownPrefix不继承此结论。
