---
schema_version: 1
id: finding.sdk-python-agent-component-stream-projection
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- time_lifecycle
boundary_ref: boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-family-bridge
- behavior.sdk-family-workflow
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-python-agent-component-stream-projection
- evidence.sdk-python-typed-component-stream-repair
- evidence.sdk-python-typed-component-stream-verification
audit_refs: &id001
- audit.sdk-python-typed-component-stream-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-python-typed-component-stream-repair
verification_evidence_refs:
- evidence.sdk-python-typed-component-stream-verification
rereview_audit_refs: *id001
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Python typed AgentComponentCall丢已接受Host stream

## 问题

原_typed_call校验合法stream，却AgentComponentCall.from_payload仅投影component/request/operation/extension/invocation_id/deadline，handler拿不到Hostissuedstream用于公开stream_writer。README明确typedWorkflow/Sandboxstreamvariant，raw register workaround不修typedhelper。

## 触发条件与影响

合法agent_component_call_stream/workflow_run_stream payload解码成功但无stream/raw属性，静默丢已接受字段。

## 证据

sdk_capability_acceptance_audit实际只读反例由父派窄修；18archive原red与0744新5green/related41/style0及独立PASS已归档，该具体本地缺陷resolved。

## 处理记录

复用既有typedcall结构补optionalstream与严格decoder，无新wire/Rustidentity接受。原0744/26raw冻结仍独立；新typedhelper自己的5green证明该具体修复，不宣wholeSDK通过。

## 本地闭合范围

repair/verification/rereview三套引用齐备；保原noHostfullsuite3fail及ACKfixture后继另scope，最终pin/currentgen/whole门禁/main独立待验。
