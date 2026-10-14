---
schema_version: 1
id: finding.sdk-rag-unicode-character-count
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
- behavior.sdk-family-rag
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-rag-unicode-character-count-candidate
- evidence.sdk-database-rag-clinical-framework-candidate
- evidence.sdk-rag-scalar-repair
- evidence.sdk-rag-scalar-verification
audit_refs:
- audit.sdk-rag-scalar-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-rag-scalar-repair
verification_evidence_refs:
- evidence.sdk-rag-scalar-verification
rereview_audit_refs:
- audit.sdk-rag-scalar-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# RAG字符统计与预览使用字节

## 问题

原13byte中文emoji被报char_count13而标量4；preview70中文截断单位错误。框架D两个字段已oldred2→green2且sourceindependentPASS，当前0435原native不覆盖D；最终Dpublishedpin+新consumer待归档。

## 触发条件与影响

实际consumer路径与原raw回收在所引Evidence；未知/非法值不静默默认，适配边界不新增第二业务权威。

## 证据

原真实反例及已执行局部候选有自己的epoch；最终scope三链未齐保持open，不借别处whole绿。

## 处理记录

用户已授权当前合同修复，无新产品裁决。仅生产owner切片修，模型lane不改业务/生成物；最终生成/pin/source三语/独立复审另验。

## Framework D后继切点

框架3source修复与23wholecommands已独立scope/receiptPASS，3504/0/3optin、978source1d52/Git848fa保候选。SDK旧Host结果不证明D；新RAG32calls仅prepared、DB/Clinical正确值native待Dpublished/新bin。此SDK跨仓Finding暂保open待精确运行后继，不伪wholeSDK绿色。

## 本地精确闭合

publishedb909/noPatch ee9ff三原生scalar32call/13POST及65archive/source限定独立PASS三链齐备，本缺陷resolved；旧0435 byteoracle保持错误观察不充正确值。旧prepared阶段是历史，当前只证明RAG scalar修复，不借为wholeSDK/finalcontract连续性完成。
