---
schema_version: 1
id: finding.sdk-language-consumption-ack
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
- audit.sdk-language-client-lifecycle-data-durability
- audit.sdk-language-client-lifecycle-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
verification_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
rereview_audit_refs:
- audit.sdk-language-client-lifecycle-data-durability
- audit.sdk-language-client-lifecycle-failure-concurrency
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# iterator等待next才ACK阻塞且gap未消费即autoACK

## 触发条件与影响

当前源码路径：`sdks/python/src/echo_agent_sdk/client.py`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

自然消费、显式replayACK三语言旧pin gap场景绿；target与consumer异常ACK待审。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

iterator等待next才ACK阻塞且gap未消费即autoACK。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 当前精确客户端子缺陷局部闭合

原officialwriter completion提交消费而非fetch/nextpending就消费；gap不autoACK、显式snapshot/replaycoverage，failed/cancelledpending不commit、terminalcoverage自然EOF。ACKv4原独review+18/4/1强assert不是counts单证。 原independent实际核原repair与nativepostfix强assert后为closeeligible，具体sharedEvidence段落/对应Audit故障视角明确并非统一time-lifecycle泛借。原正文open/待测句保编写时历史，新具体Bug可resolved不等SDKmain；新negotiation、初始化异常清理、HostremoteCause或frameworkB5未因它关闭。没有外部Issue修改。
