---
schema_version: 1
id: finding.sdk-cleanup-retry-owner
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-resources-streams
behavior_refs:
- behavior.sdk-resources-streams
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-cancellation-observation
- evidence.sdk-rust-workspace-factory-gate-failure
- evidence.sdk-factory-child-settlement-verification
audit_refs:
- audit.sdk-cancellation-observation-rereview
- audit.sdk-factory-child-settlement-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-cancellation-observation
- evidence.sdk-factory-child-settlement-repair
verification_evidence_refs:
- evidence.sdk-cancellation-observation
- evidence.sdk-factory-child-settlement-verification
rereview_audit_refs:
- audit.sdk-cancellation-observation-rereview
- audit.sdk-factory-child-settlement-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 关闭前 remove/tombstone 丢失取消或失败后的重试 owner

## 触发条件与影响

当前源码路径：`echo-sdk-host/src/core_profile/facade/mod.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

源码确定；cleanup cancellation/EOF/retry owner 反例待执行。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

关闭前 remove/tombstone 丢失取消或失败后的重试 owner。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 当前Rust门禁的有限后继

当前workspace真实两个Factoryfailure由finding.sdk-factory-terminal-close-settlement承接，日志32pass2fail/exit101原byte保。Trackerrelease/close_resultErrcache只是初hypothesis，不从构造或并发测试名推单一共享锁。原有限identity/cleanup修复epoch继续保，当前source修复与全matrix续跑尚未通过。

## 已闭合Factory子scope

原FactoryInstanceLease failedcloseErr永久缓存已实际单变量红→同ownerparallelretry成功/noextraRPC/parentunregister与独立源PASS齐备，Factorycleanupretry子scope有限resolved。此宽Finding其余resource/Python/Sandbox/connectionowner义务不由这一局部自动闭合；statusopen保持，scope精确见FactorysettlementEvidence。

## callback观察与原物理回收子scope

当前原invoke/ACP反例分别证明observer先unwind会提前回收、错误ACK会保资源却错误结束source；两处最小修复后，同不可变testbinary七个完整selector实际全通过。普通typed/raw final Error在首次错误可见时已释放guard/lease，合法foreign观察精确保身份；独立局部复审pass。修复、验证与复审三链见新增Evidence/Audit；它们不关闭宽Finding其余义务，不证明HostRun已结算或SDK最终交付。
