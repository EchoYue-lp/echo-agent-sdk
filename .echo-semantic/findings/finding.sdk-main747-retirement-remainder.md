---
schema_version: 1
id: finding.sdk-main747-retirement-remainder
kind: finding
type: consolidation_candidate
status: resolved
severity: high
primary_focus: contract_evidence
focus: [contract_evidence, state_authority]
boundary_ref: boundary.sdk-integrations
behavior_refs: [behavior.sdk-family-a2a, behavior.sdk-family-lsp]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-main747-retained-retirement-boundary]
audit_refs: [audit.sdk-main747-retirement-remainder-rereview]
decision_refs: []
repair_evidence_refs: [evidence.sdk-main747-retained-retirement-boundary]
verification_evidence_refs: [evidence.sdk-main747-retained-retirement-boundary]
rereview_audit_refs: [audit.sdk-main747-retirement-remainder-rereview]
discovered_at: 747be937226ce379d0abc04e0f3ba52029da5ed1
decision: retire
candidate_asset_refs: [asset.sdk-retired-a2a-lsp-consumer-sources, asset.sdk-source-product]
canonical_asset_ref: asset.sdk-source-product
replacement_refs: [asset.sdk-source-product]
rollback_ref: Restore these seven paths and direct exports/tests from main747 on a new task branch; preserve archived43dd and stash26d22f23; never rewrite published main.
delete_paths:
- sdks/typescript/src/jwt_values.ts
- sdks/python/src/echo_agent_sdk/jwt_values.py
- sdks/java/src/main/java/com/echoagent/sdk/JwtClaims.java
- sdks/java/src/main/java/com/echoagent/sdk/JwtConfig.java
- sdks/shared/testing/a2a_host_fixture.py
- sdks/shared/testing/lsp_diagnostic_server.py
- sdks/shared/testing/lsp_fault_server.py
---

# 当前main747剩余七路径的具名退役

## 问题

这七路径属于原批准29路径的剩余main差异；旧Finding的before525不能承接当前before747。新Finding只管这七路径，不改旧Finding状态。

## 触发条件与影响

四公开JWT构造文件及三独立fixture被移除，这是有意改变。目标accepted准入两端均0、两原owner同字节及保留suite字面调用相同，是有限保留对照；不能填补before动态或外部消费者未知。

## 处理记录

复用ADR0042、两原Asset与当前Host/语言入口，无第二store/executor/状态权威。当前完整验证按其实际原件保留。新Finding在独立审查前保持open。

## 证据

关联Evidence列精确base/current摘要、7路径、3个有限matched场景以及全部changed/unknown限制。等待实际rereview裁定，不为预检通过提前标resolved。


## 2026-11-09 当前处理状态

独立复审实际PASS C0/I0/M0后登记关联Audit，仅本次7路径有限退役Finding由open转resolved。原before动态/f5/外部动态未知未关闭；旧Map/Finding/closure未改变。原producer和strict尚须实际执行，resolved本身不代表门禁通过。
