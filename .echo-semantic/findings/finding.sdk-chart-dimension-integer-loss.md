---
schema_version: 1
id: finding.sdk-chart-dimension-integer-loss
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: contract_evidence
focus:
- contract_evidence
- state_authority
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-chart
- behavior.sdk-family-value
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-chart-dimension-integer-loss-current-source
audit_refs:
- audit.sdk-computation-50-rereview
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Chart宽高整数静默丢失

## 问题

当前publishedframeworkb909 GenerateChartTool::execute将width/height的as_u64结果unwrap_or(600/400)后as u32，schemainteger无u32 bounds。4294967296→0、4294967297→1，negative被静默default，违反明确整数无损义务。

## 触发条件与影响

SDK候选569/source17cf、Hostbb4f仍调用b909有缺陷图表工具；Host薄dispatch原frameworkschema验证不修这些无bounds值，不能静默约窄或填默认。

## 证据

current_tool_boundary_review本次实际只读source确认Important1/BLOCK；b909chartSHA cc9ea45f…107–116及原parametersschema可复核。这里是确定的静态sourcecounter，不称SDKruntime已executedRED。原50case有限通过不包括这些宽高边界，不可用它关整数义务。

## 处理记录

Root已授权框架窄修，Nafter23/3508/6focused/source22afe独审通过但尚未published/pin，当前SDK仍b909，所以本Finding保持open。修复属于已接受无损合同，不需新产品延期/风险接受；不借Nsource302abc当SDK569或原source17cf，后续真实publishedpin/nativeedgeconsumer及独立复审另验。12canonical迁移/5秒B/dep50选择仍各pending。
