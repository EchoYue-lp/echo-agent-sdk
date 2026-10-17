---
schema_version: 1
id: finding.sdk-clinical-trials-total-count-request
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- time_lifecycle
- failure_concurrency
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-research
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-clinical-trials-total-count-request-candidate
- evidence.sdk-database-rag-clinical-framework-candidate
- evidence.sdk-ordinary-tools-b909-repair
- evidence.sdk-ordinary-tools-b909-verification
audit_refs:
- audit.sdk-clinical-trials-total-count-request-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-ordinary-tools-b909-repair
verification_evidence_refs:
- evidence.sdk-ordinary-tools-b909-verification
rereview_audit_refs:
- audit.sdk-clinical-trials-total-count-request-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# ClinicalTrials缺totalCount被填0

## 问题

同官方GET默认没有totalCount而返回1study被映射total0；仅countTotal=true得386。根D窄修准备，不将unknown填0或改nullablecontract，不新增publicAPI；新actualtarget验证待。

## 触发条件与影响

实际consumer路径与原raw回收在所引Evidence；未知/非法值不静默默认，适配边界不新增第二业务权威。

## 证据

原真实反例及已执行局部候选有自己的epoch；最终scope三链未齐保持open，不借别处whole绿。

## 处理记录

用户已授权当前合同修复，无新产品裁决。仅生产owner切片修，模型lane不改业务/生成物；最终生成/pin/source三语/独立复审另验。

## Framework D后继切点

框架3source修复与23wholecommands已独立scope/receiptPASS，3504/0/3optin、978source1d52/Git848fa保候选。SDK旧Host结果不证明D；新RAG32calls仅prepared、DB/Clinical正确值native待Dpublished/新bin。此SDK跨仓Finding暂保open待精确运行后继，不伪wholeSDK绿色。

## 本地精确字段/能力闭合

原actualbeforefailure与b909/noPatch ee9ff after普通五族strongoracle/native3×49、75archive/15input、independentfinitePASS三链齐备，该具体bugresolved。上方sourceprepared/新bin待为原历史阶段；当前Sandbox自然EOFbufferrace仍独立open，整TSclass2/3不能改全绿。acceptedsource12人的兼容选择未答不写seed；最终contract/main另验。
