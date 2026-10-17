---
schema_version: 1
id: finding.sdk-factory-instance-routing
kind: finding
type: implementation_bug
status: resolved
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
- evidence.sdk-language-client-lifecycle-current
audit_refs:
- audit.sdk-language-client-lifecycle-state-authority
- audit.sdk-language-client-lifecycle-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
verification_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
rereview_audit_refs:
- audit.sdk-language-client-lifecycle-state-authority
- audit.sdk-language-client-lifecycle-failure-concurrency
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

## 当前精确客户端子缺陷局部闭合

三native5 distinct child/same descriptor、parallel closeA/B live/parent再构造、PY invalid factoryorigin stalegeneration零handler effect。Hostfactory_origin exactrouting与真实callback匹配，不用managedstore/LlmGraph代。 原independent实际核原repair与nativepostfix强assert后为closeeligible，具体sharedEvidence段落/对应Audit故障视角明确并非统一time-lifecycle泛借。原正文open/待测句保编写时历史，新具体Bug可resolved不等SDKmain；新negotiation、初始化异常清理、HostremoteCause或frameworkB5未因它关闭。没有外部Issue修改。
