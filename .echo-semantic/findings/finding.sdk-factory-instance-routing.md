---
schema_version: 1
id: finding.sdk-factory-instance-routing
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-extension-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-rust-workspace-factory-gate-failure
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Factory child新handle在三语言没有精确路由关联

## 触发条件与影响

当前源码路径：`echo-sdk-protocol/src/methods.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

采用Host childhandle唯一identity+typedfactory_origin，不做unknownfallback；三语言真实factory生命周期待验证。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

Factory child新handle在三语言没有精确路由关联。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 当前Rust门禁的有限后继

当前workspace真实两个Factoryfailure由finding.sdk-factory-terminal-close-settlement承接，日志32pass2fail/exit101原byte保。Trackerrelease/close_resultErrcache只是初hypothesis，不从构造或并发测试名推单一共享锁。原有限identity/cleanup修复epoch继续保，当前source修复与全matrix续跑尚未通过。
