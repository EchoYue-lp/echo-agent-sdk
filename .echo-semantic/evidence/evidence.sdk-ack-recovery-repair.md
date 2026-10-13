---
schema_version: 1
id: evidence.sdk-ack-recovery-repair
kind: evidence
observed_at: source:pending
source_refs:
- docs/sdk/governance/evidence/sdk-ack-recovery-windows/review-final-v4-source-manifest.json
- docs/sdk/governance/evidence/sdk-ack-recovery-windows/evidence-sha256.json
- docs/sdk/governance/evidence/sdk-ack-recovery-windows/REVIEW-DELTA.md
- docs/sdk/governance/evidence/sdk-successor-model-reviews/ack-exact-slice.json
supports:
- behavior.sdk-events-recovery
- behavior.sdk-family-event-replay
- behavior.sdk-language-clients
limitations:
- Originalc52 focused scope only; no finalglobal runtime promotion
- Native physicalprune not exposed byacceptedAPI; only Rustphysicalunit + officialtransport
  valid retention
- ExternalIssue120 closes only after exactmain delivery
---

# ack-recovery-repair

## 支持的结论

既有Host恢复发送权威和官方client真实消费事实分离。retention snapshot保原请求cursor连接、overlap/touch普通pages合并，空page不产生coverage；乱序/重复end不能丢真实连接。仅成功官方ACK提交消费；pendingACK可容规范后续gap，失败不剪未消费prefix。实际liveinterval只覆盖真正收到sequence，不以lastLive最大值授权缺失gapinterior。原v3用同最终fixture四行为fail；修后23PythonACK/TS12/Java18，combined65/53/35零失败/skip；真实Hostheldcompletion success/failure六记录保留。独立exactslice Important2和fixtureMinor PASS/action0归档。

## 来源与范围

原归档receipt/输入hash/二进制保持不变。当前观察只绑定模型与可读取归档，不把原epoch冒称最终current全套通过。

## 已知缺口

Originalc52 focused scope only; no finalglobal runtime promotion；Native physicalprune not exposed byacceptedAPI; only Rustphysicalunit + officialtransport valid retention；ExternalIssue120 closes only after exactmain delivery。
