---
schema_version: 1
id: finding.sdk-language-external-ack-projection-admission-ordering
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- data_durability
- contract_evidence
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-language-client-lifecycle-current
audit_refs:
- audit.sdk-language-client-lifecycle-failure-concurrency
decision_refs: []
repair_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
verification_evidence_refs:
- evidence.sdk-language-client-lifecycle-current
rereview_audit_refs:
- audit.sdk-language-client-lifecycle-failure-concurrency
discovered_at: 26c781e66260cca2851658f4784193df5980dc22
---

# 三语言external ACK投影与满队列事件准入竞态

## 问题

ACK frame已被peer看见，原发送completion或成功后的本地退休投影尚未完成时，合法下一event按仍占满的旧data queue准入，导致队列永久失败及后续投递中断。TS保持原backpressure，PY/Java保持原event_gap；不是Host journal数据被删除、fixture oracle或新的容量政策。

## 触发条件与影响

capacity1已有data1和gap2；peer见ACK2后投递event3，本地仍未退休旧prefix。成功ACK后来把消费水位推进至2，也无法恢复已失败queue/event3。Java原26c Linux window1和受控真实Host/官方transport机制反例、TS/PY原4256官方mechanism counter分别保存；后两者不是物理Host before。不得增加容量、提前消费或删除原续流断言规避。

## 证据

复用 evidence.sdk-language-client-lifecycle-current 的本次before/after段与 audit.sdk-language-client-lifecycle-failure-concurrency 的具名challenge，保存原typed错误、精确源/结果SHA/官方job及Admin定位。旧 finding.sdk-language-consumption-ack 闭合的是iterator/gap自动ACK，不用它证明本竞态after，也不重开#120原framework事件权威。

## 处理记录

Java2e0与TS/PY69cb已保存修复并分别具实际受控before→after、有限真实Host窗口及成功/失败/取消/关闭回归；FrameFinal(Java)、SDKcap(TS/PY postMinor)已实际源码/after有限PASS。具体执行时2e0dirty与后保存69cb、版本/源输入及原失败限制见复用Evidence。仅此已实证竞态局部resolved；总transport内存不宣有界，新Linux/完整语言/原七open/B3/12/合同13/Hostcause/continuity/payload/main独立。无新publicAPI、Host权威、wire、容量或accepted合同；不改变原七个open状态。
