---
schema_version: 1
id: finding.sdk-gap-ack-replay-watermark
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-events-recovery
behavior_refs:
- behavior.sdk-events-recovery
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-ack-recovery-verification
- evidence.sdk-gap-watermark-host-verification
audit_refs: &id001
- audit.sdk-gap-watermark-host-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-gap-watermark-host-repair
verification_evidence_refs:
- evidence.sdk-gap-watermark-host-verification
rereview_audit_refs: *id001
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# ACK snapshot watermark后重发旧事件

## 问题

历史30d StreamDelivery ACK后仍使用last_sent而未推进已消费snapshot水位；旧问题对应framework Issue120。

## 触发条件与影响

snapshot2 ACK后旧水位机制真实Hostlive2重发覆盖事件。新LP隔离实验保持当前其它修复，仅删除ACKmax一行、同fixture原断言fail；恢复后live3/gapfrom4成功。

## 证据

repair/verification保存27档原byte/hash、216SDK/552framework可恢复输入、实际red1fail101→green1pass0ignored同fixture。原完整events960行e4/sourcecallers independentPASS及12eventsunit/4Host/恢复log独立；新archive两个tar逐成员、gitshow30d、singleline差异均独立复核PASS/action0。不宣称完整30dSDK构建或三语言oldHost红。

## 处理记录

精确本地watermark缺陷resolved，三链引用齐备。e4源当前匹配；ACKclientv4其它缺陷使用自己的独立scope。最终frameworkChannelpublishedpin/SDK完整源生成与三语言重验、main和外部Issue关闭独立待验，不用main未交付单独阻止局部bug闭合。

## 历史关联与不可恢复叙述

原对象来自framework同名Finding，Issue https://github.com/EchoYue-lp/echo-agent/issues/120 。过去叙述曾称Rustcount/oversized三语言directred，但原日志未找到，不作确认计数或闭合依据；保原历史pointer，实际确认反例只为本次snapshot2/live2 singleline机制，原历史源码与新实验边界完整归档。没有本lane GitHubIssue状态修改。
