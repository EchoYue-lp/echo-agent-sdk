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
observed_at: source:18de34be70afdcf30950277f1bc16142a058e3d4da6905fa4e1eee36f2a7663b
code_refs: [AGENTS.md, MIGRATION-SOURCE.md, docs/adr/0001-sdk-repository-boundary.md]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity, evidence.sdk-framework-pin, evidence.sdk-wave2-cutover-repair, evidence.sdk-wave2-cutover-final-verification, evidence.sdk-compatibility-gates]
finding_refs: [finding.sdk-wave2-cutover-integrity]
---

# SDK repository ownership

## 重要承诺

SDK Host、protocol、合同、三语言源码、SDK 文档、生成器与 CI 只在本仓长期维护。

## 当前行为

Source import 与 continuity 已承接完整 SDK 产品和历史；已合并的 PR #2 恢复被 squash 丢失的
双父 ancestry，并把 Host pin、accepted contract、inventory telemetry 和三语言 catalog
同步到 framework Wave 2 最终 main revision。最终本地门禁、SDK 8/8 CI 与 merge ancestry 已验证。

## 期望行为

Framework 只保留运行时权威与指向本仓的边界链接，不维护第二份可编辑 SDK 产品。

## 触发、结果与副作用

SDK 变更在本仓独立构建和验证；framework 依赖升级是显式 SDK commit，完整 Rust inventory
只提供非阻断漂移遥测。
三语言 catalog 测试直接读取 accepted artifact；完整 inventory 的 scope 数量和 intrinsic
membership 只打印遥测，分类样例不要求未接受的 identity 必须存在。

## 失败、重试与恢复

源差异、不可恢复 revision 或未推送 commit 会阻止 framework 删除和 superproject gitlink 更新。

## 证据

仓库规则、迁移说明、ADR 和 Git ancestry 共同证明 ownership。

## 裁决记录

用户确认 SDK 是依赖 echo-agent 的独立产品，并作为 lp-agent 第四个 submodule。
