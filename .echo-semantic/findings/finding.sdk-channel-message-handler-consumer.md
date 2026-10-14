---
schema_version: 1
id: finding.sdk-channel-message-handler-consumer
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: contract_evidence
focus:
- trigger_input
- state_authority
- time_lifecycle
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-channels
- behavior.sdk-extension-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-channel-message-handler-consumer-audit
- evidence.sdk-channel-forward-human-scope
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Accepted Channel MessageHandler inbound consumer is missing

## 问题

The accepted MessageHandler handle/handle_stream/reply bridge has typed proxy
implementations but no actual inbound consumer from the registered language
ChannelPlugin. start stores a handler and passes a plain handler_id; no native
consume path calls the stored handler. Reverse extension/invoke cannot be
reinterpreted as an SDK-to-Host receive operation.

## 触发条件与影响

A language plugin attempts to deliver an inbound message through its declared
handler after successful registration/start. Existing send/stop success cannot
prove input reaches the accepted handler or its stream/reply lifecycle. Creating
a new unaccepted wire receive verb would mask the obligation instead of proving
current contract behavior.

## 证据

Source audit Evidence records the actual registration, start storage and reverse
request direction. This is static path evidence, not a runtime failure trace.

## 处理记录

Status open. Determine the exact accepted input/consumer contract before any
API implementation. Real shared inbound/stream/reply scenarios and independent
review remain required. Other channel plugin start/send/stop receipts are
preserved within their original scope. No external Issue is created/closed here.

## 已批准的处置

用户选择保留原承诺并批准三个forward操作及流控制范围；ADR0035设计/实现进行中。原缺口保持open至真实消费、取消/owner/stream结算、最终生成与复审完成，不再等待产品选择。semantic0.5无Decision对象类型，使用人的原答复Evidence定位，不造decisionRef或riskAccepted。
