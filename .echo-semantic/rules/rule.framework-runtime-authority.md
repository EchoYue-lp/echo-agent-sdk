---
schema_version: 1
id: rule.framework-runtime-authority
kind: rule
status: verified
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus: [time_lifecycle, failure_concurrency, contract_evidence]
observed_at: source:dfb25170ad6ddac42bf3294b902a2815a34970ae1fb74dec4e905dd386b9e80c
behavior_refs: [behavior.sdk-repository-ownership]
code_refs: [AGENTS.md, echo-sdk-host/src/lib.rs, echo-sdk-protocol/src/lib.rs, docs/adr/0001-sdk-repository-boundary.md]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity]
finding_refs: []
---

# Framework runtime authority

## 不变量或唯一权威

`echo_agent` 唯一拥有 Agent、Run、Task、Subagent、event、retry、cancel、recovery、persistence 和 terminal 语义。

## 适用行为

适用于 protocol DTO、Host adapter、ACP transport 和三语言 client。

## 当前实现

Host 消费 framework 服务并做协议转换；protocol 和 clients 不建立第二套 runtime authority。

## 期望行为

Protocol 最终只依赖 wire/schema 库，Host 精确依赖 framework revision，所有 adapter 转换保持字段级无损。

## 证据

AGENTS、SDK boundary ADR、Host 与 protocol 入口共同限定依赖方向。

## 裁决记录

用户确认 protocol 纯化和 accepted external contract 边界；source continuity 不改变这些长期决策。
