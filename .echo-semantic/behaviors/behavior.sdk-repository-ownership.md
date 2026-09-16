---
schema_version: 1
id: behavior.sdk-repository-ownership
kind: behavior
status: stale
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus: [data_durability, contract_evidence]
boundary: boundary.sdk-product
observed_at: source:4a4b9005728bf4b6ca1298ed3d92d9b9eab0d20416602abf075116a3c293e6db
code_refs: [AGENTS.md, MIGRATION-SOURCE.md, docs/adr/0001-sdk-repository-boundary.md]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity, evidence.sdk-framework-pin, evidence.sdk-wave2-cutover-repair]
finding_refs: [finding.sdk-wave2-cutover-integrity]
---

# SDK repository ownership

## 重要承诺

SDK Host、protocol、合同、三语言源码、SDK 文档、生成器与 CI 只在本仓长期维护。

## 当前行为

Source import 与 continuity 已承接完整 SDK 产品和历史；当前修复分支恢复被 squash 丢失的
双父 ancestry，并把 Host pin、accepted contract、inventory telemetry 和三语言 catalog
同步到 framework Wave 2 最终 main revision。最终门禁与 SDK merge ancestry 尚待验证，因此本 Behavior 暂为 stale。

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
