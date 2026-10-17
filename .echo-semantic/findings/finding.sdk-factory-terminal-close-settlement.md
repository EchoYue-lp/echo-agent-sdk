---
schema_version: 1
id: finding.sdk-factory-terminal-close-settlement
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- state_authority
- contract_evidence
boundary_ref: boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-family-bridge
- behavior.sdk-family-eval
- behavior.sdk-family-improve
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-rust-workspace-factory-gate-failure
- evidence.sdk-factory-child-settlement-repair
- evidence.sdk-factory-child-settlement-verification
audit_refs:
- audit.sdk-factory-child-settlement-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-factory-child-settlement-repair
verification_evidence_refs:
- evidence.sdk-factory-child-settlement-verification
rereview_audit_refs:
- audit.sdk-factory-child-settlement-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Factory终态后关闭与实例结算门禁失败

## 问题

Rust官方全workspace当前两个真实Factory E2E失败：lazyEval/Improve factory unregister报仍有liveCustomAgents，同Factory跨concurrentSessiontypedresults失败。

## 触发条件与影响

原actualRPC response与remoteTerminal可能已经到达，但consumer见终态后的AgentClose与异步trackerrelease/原exclusivelease竞态尚待验证；close_result OnceCell缓存Err让childlive也是hypothesis，不已确认结论。完整失败日志保持原代码/断言。

## 证据

Hostlib167/core31green、bridge32pass2fail/workspaceexit101/完整25123bytes，前4matrix格式lint0、后9未执行。原632输入wholelanguage成功不作为新bridge源码通过。

## 处理记录

具体Factoryowner正soloCargo复现，未闭合；不能先改原twofact物理结算authority、弱化exclusive准入或凭测试名猜共享lock。待真实根因/红绿/独立复审三链后resolved，无新产品scope。

## 当前根因与精确闭合

原hypothesis已被twofact真实unit/Factoryactualbefore5/并行close-retry强反例确认，当前8e7源码5/0、171lib/34bridge、严格lint及独立PASS三链齐备，specificFactory2bug本地resolved。前初matrix失败和假设保历史，不用old632languagewhole冒新bridgepass；pendinganswer/terminal/wronggen占用保持，原physical5sframeworkB未选择，不能由remoteEOF泛推physicalsettled。
