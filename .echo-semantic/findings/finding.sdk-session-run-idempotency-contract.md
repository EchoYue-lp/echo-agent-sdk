---
schema_version: 1
id: finding.sdk-session-run-idempotency-contract
kind: finding
type: intent_gap
status: open
severity: high
primary_focus: contract_evidence
focus:
- trigger_input
- state_authority
- failure_concurrency
- result_side_effect
boundary_ref: boundary.sdk-agent-session-run
behavior_refs:
- behavior.sdk-agent-session-run
rule_refs:
- rule.framework-runtime-authority
evidence_refs: []
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 76601b242d7c354352f3b5c7d4bf099873746b14
---

# Session / Run 已接受 idempotency_id 未消费

## 问题

级别：Important（机器 severity high）；契约缺口使用合同允许的 `intent_gap`，不是未经定义的 `contract_gap` 类型。

## 触发条件与影响

SessionCreateRequest 与 RunStartRequest 接受并校验 idempotency_id，而 handler 未读取。真实 Host193e49 的四个有界反例已证：同 key 重复 Session 创建、changed cwd 无冲突、关闭后又创建；Run completed 后同 key 重复执行、changed input 无冲突、真实模型回调重复；并发 duplicate 仅受 active Prompt Turn 拒绝，不重放原响应。响应丢失重试可能导致重复资源或执行副作用。

## 证据

事实包：`docs/sdk/governance/evidence/session-run-idempotency-counterexample/`；原 facts SHA `d1658d4c77694b7f160d2c51874a72a4570cdf649e16a9c212de71d1bb2c0a8e`，不可变 Host SHA `193e49b8557db9b746a1cc8c6f5cf30bb6d3154a630562f7ef8551b949e878d1`。discovered_at 仅记录发现时实际 SDK checkout；不称其含完整 composed runtime。生产源码 SHA 由 facts/summary 固定。首脚本误发不存在 run/close 的失败单独保留，不归为产品缺陷。

直接入口：`echo-sdk-protocol/src/methods.rs:362,651`；`echo-sdk-host/src/core_profile/handler.rs:262,865`。现有 Agent create 在 `handler.rs:118` 与 `handles.rs:364` 明确 key/fingerprint/closed 行为；只能据此建议复用，不自动定义另外两个操作的公共合同。

## 处理记录

裁决材料：`docs/sdk/governance/session-run-idempotency-proposal.md`，用户已明确选择 A，状态 Approved；正式权威见 ADR0037。批准保留既有参数，在当前连接/generation/操作/原 owner scope 内 same key+canonical payload 重放原 publication、changed payload typed conflict、真正关闭后 typed closed_handle 不复活；备选明确延期并副作用前拒绝参数。跨连接/重启 durable 幂等不在推荐承诺内；框架仍唯一执行/终态权威，Host 仅 publication mapping。

本 Finding 保持 open。用户已批准 A 的期望，当前原 Host Registry/结算 owner 实现候选与直接反例验证进行中；尚需独立源码复审、三语言同场景证据和交付对账，不把 collector exit0 当行为修复通过。12source 迁移及5秒物理结算属于其它未决单元，此项不替代或催促其裁决。
