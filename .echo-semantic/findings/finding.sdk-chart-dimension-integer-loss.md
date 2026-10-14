---
schema_version: 1
id: finding.sdk-chart-dimension-integer-loss
kind: finding
type: implementation_bug
status: resolved
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
- evidence.sdk-chart-dimension-published-after-repair
- evidence.sdk-chart-dimension-published-after-verification
audit_refs:
- audit.sdk-computation-50-rereview
- audit.sdk-chart-dimension-published-after-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-chart-dimension-published-after-repair
verification_evidence_refs:
- evidence.sdk-chart-dimension-published-after-verification
rereview_audit_refs:
- audit.sdk-chart-dimension-published-after-rereview
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

## 当前发布源after的具体闭合

原b909/569整数BLOCK依旧是原审查正确结论。后来frameworkbe5已正式main/7PR7mainCI、SDK八精准sourceGitpin无patch并以bcb真实3native同consumer11cases观察before4failure/语种→after0；checkedU32/schema边界/缺None默认/max0/Unicode/错误完整字段与8protected无effect，68raw/55build/627source以及独立source+afterPASS三链令本具体Finding局部resolved，不等SDKmain。新afterEvidence/Audit观测暂pending待全61源freeze/newprivateGit，不伪9db包含新packet；原50cases有限PASS、computationb909I1BLOCK历史保，61测试在跑不改成绿。5sB/full12gen仍pending，整个支持族未自动verified。
