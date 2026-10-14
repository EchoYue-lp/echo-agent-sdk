---
schema_version: 1
id: finding.sdk-language-close-retry
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
- audit.sdk-language-client-lifecycle-time-lifecycle
- audit.sdk-language-client-lifecycle-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
verification_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
rereview_audit_refs:
- audit.sdk-language-client-lifecycle-time-lifecycle
- audit.sdk-language-client-lifecycle-failure-concurrency
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# close/unregister确认前清owner导致失败不可重试

## 触发条件与影响

当前源码路径：`sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

失败/并发close和callback保留回归已编写；真实Host副作用待验证。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

close/unregister确认前清owner导致失败不可重试。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 当前精确客户端子缺陷局部闭合

真实session/agent close以及registration/run失败重试后原callback继续live，不是仅nativeprocess closeFuture。保原owner直到确认、同ownerretry/coalescedwait；新TS initFailureCatch漏child另scope不被此局部闭合覆盖。 原independent实际核原repair与nativepostfix强assert后为closeeligible，具体sharedEvidence段落/对应Audit故障视角明确并非统一time-lifecycle泛借。原正文open/待测句保编写时历史，新具体Bug可resolved不等SDKmain；新negotiation、初始化异常清理、HostremoteCause或frameworkB5未因它关闭。没有外部Issue修改。
