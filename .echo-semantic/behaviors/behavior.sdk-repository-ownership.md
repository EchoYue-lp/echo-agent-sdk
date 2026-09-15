---
schema_version: 1
id: behavior.sdk-repository-ownership
kind: behavior
status: verified
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus: [data_durability, contract_evidence]
boundary: boundary.sdk-product
observed_at: source:d66d41ae73c5e05d8bc1781eb8f457abe1f0e2730a8e3b40f5af32a500a7616f
code_refs: [AGENTS.md, MIGRATION-SOURCE.md, docs/adr/0001-sdk-repository-boundary.md]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity, evidence.sdk-framework-pin]
finding_refs: []
---

# SDK repository ownership

## 重要承诺

SDK Host、protocol、合同、三语言源码、SDK 文档、生成器与 CI 只在本仓长期维护。

## 当前行为

Source import 与 continuity 已承接完整 SDK 产品和历史；clean pin 已完成 protocol 纯化、Host
精确 revision、accepted contract 分离和三语言独立门禁。

## 期望行为

Framework 只保留运行时权威与指向本仓的边界链接，不维护第二份可编辑 SDK 产品。

## 触发、结果与副作用

SDK 变更在本仓独立构建和验证；framework 依赖升级是显式 SDK commit，完整 Rust inventory
只提供非阻断漂移遥测。

## 失败、重试与恢复

源差异、不可恢复 revision 或未推送 commit 会阻止 framework 删除和 superproject gitlink 更新。

## 证据

仓库规则、迁移说明、ADR 和 Git ancestry 共同证明 ownership。

## 裁决记录

用户确认 SDK 是依赖 echo-agent 的独立产品，并作为 lp-agent 第四个 submodule。
