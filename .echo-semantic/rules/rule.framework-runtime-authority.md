---
schema_version: 1
id: rule.framework-runtime-authority
kind: rule
status: verified
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus: [time_lifecycle, failure_concurrency, contract_evidence]
observed_at: source:d66d41ae73c5e05d8bc1781eb8f457abe1f0e2730a8e3b40f5af32a500a7616f
behavior_refs: [behavior.sdk-repository-ownership]
code_refs: [AGENTS.md, echo-sdk-host/src/lib.rs, echo-sdk-protocol/src/lib.rs, docs/adr/0001-sdk-repository-boundary.md]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity, evidence.sdk-framework-pin]
finding_refs: []
---

# Framework runtime authority

## 不变量或唯一权威

`echo_agent` 唯一拥有 Agent、Run、Task、Subagent、event、retry、cancel、recovery、persistence 和 terminal 语义。

## 适用行为

适用于 protocol DTO、Host adapter、ACP transport 和三语言 client。

## 当前实现

Host 消费精确 pin 的 framework 服务并做协议转换；protocol 不依赖任何 echo framework crate，
clients 不建立第二套 runtime authority。

## 期望行为

Protocol 只依赖 wire/schema 库，Host 精确依赖 framework revision，所有 adapter 转换保持字段级无损；
协商只使用 protocol version、accepted contract digests、capability 与 compiled feature。

## 证据

AGENTS、SDK boundary ADR、Host 与 protocol 入口共同限定依赖方向。

## 裁决记录

用户确认 protocol 纯化和 accepted external contract 边界；source continuity 不改变这些长期决策。
