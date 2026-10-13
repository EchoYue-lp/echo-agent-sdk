---
schema_version: 1
id: finding.sdk-dependency-warning-policy-conflict
kind: finding
type: authority_conflict
status: open
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- contract_evidence
boundary_ref: boundary.sdk-product
behavior_refs:
- behavior.sdk-repository-ownership
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-current-dependency-tool-results
- evidence.sdk-dependency-after-feature-tool-results
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 依赖重复警告配置与零警告合并规则冲突

## 问题

SDK AGENTS要求所有适用门禁零警告，但原deny.toml multiple-versions=warn在当前allfeatureslockedgraph产生50warningduplicate。两个工具exit0不能自动覆盖人定仓库规则。

## 触发条件与影响

当前7inputhash一致/b909658lockeddeps，freshRustSec官方6aed846两DBclean/1310advisory；auditdenywarnings0warning、deny4checksok/exit0但50duplicatewarning。原4exceptions未增、非duplicate其它category0，不是新的漏洞豁免。

## 证据

原20档hash/tool命令/退出/完整log/前后源输入逐核，独立reader明确tool事实通过但SDK合并规则阻塞。proposal文稿Statusproposed尚未人答，不是获准例外或RiskAccepted。

## 处理记录

保open至根具体规则差异审阅及人的选择/正式规则一致化与复验。不得以policy旧warn自行overrideAGENTS、不隐藏50warning、不擅修插件或ignore；不重复12source/5s选择、不加无关安全门控。Factory新Rust2fail仍另specificscope待verify/review，不能依赖exit0关全门禁。

## 新cut复验仍未形成规则豁免

afterfeature19包audit/denyactual0/fresh6aed两DB/7inputs一致，50groupcrate+actualversions与proposal完全同，但人的规则选择尚未答，mergeBLOCK维持。独立新包reader尚读，不能工具0关此Finding。
