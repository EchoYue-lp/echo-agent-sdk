---
schema_version: 1
id: finding.sdk-wire-data-fidelity
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-language-client-lifecycle-current
audit_refs:
- audit.sdk-language-client-lifecycle-contract-evidence
- audit.sdk-language-client-lifecycle-result-side-effect
decision_refs: []
repair_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
verification_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
rereview_audit_refs:
- audit.sdk-language-client-lifecycle-contract-evidence
- audit.sdk-language-client-lifecycle-result-side-effect
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 特殊Map键、重复/nonstring键和畸形值被丢字段/默认空

## 触发条件与影响

当前源码路径：`sdks/typescript/src/wire.ts`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

TS __proto__与Python丢键已runtimeprobe复现；共享malformed corpus三语言red→green。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

特殊Map键、重复/nonstring键和畸形值被丢字段/默认空。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 当前精确客户端子缺陷局部闭合

原13sharedinvalidcorpus三语严格拒nonstring/duplicate/missing/unknownmalformed；actualHost memory特殊__proto__/constructor+中文emoji原值exact不defaultempty。wire封闭代数与prototype安全，不把Java新advertarray静默drop当该closedwire已修。 原independent实际核原repair与nativepostfix强assert后为closeeligible，具体sharedEvidence段落/对应Audit故障视角明确并非统一time-lifecycle泛借。原正文open/待测句保编写时历史，新具体Bug可resolved不等SDKmain；新negotiation、初始化异常清理、HostremoteCause或frameworkB5未因它关闭。没有外部Issue修改。
