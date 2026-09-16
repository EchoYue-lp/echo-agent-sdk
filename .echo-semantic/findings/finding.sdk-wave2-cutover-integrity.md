---
schema_version: 1
id: finding.sdk-wave2-cutover-integrity
kind: finding
type: evidence_gap
status: open
severity: high
primary_focus: contract_evidence
focus: [data_durability, state_authority, time_lifecycle]
boundary_ref: boundary.sdk-product
behavior_refs: [behavior.sdk-repository-ownership]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-source-continuity, evidence.sdk-framework-pin, evidence.sdk-wave2-cutover-repair]
audit_refs: []
decision_refs: []
repair_evidence_refs: [evidence.sdk-wave2-cutover-repair]
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 863cd5b34b516fa4a0021fe26f43b06c64c7c0b7
---

# SDK Wave 2 cutover integrity

## 外部 Issue

Framework GitHub Issue: https://github.com/EchoYue-lp/echo-agent/issues/122

## 问题

SDK PR #1 被 squash 合入后，双父 source-continuity merge 不再是 SDK main 的祖先；同时 main
仍固定旧 framework revision `1754877996778afac4e4db77ce37c330496760ea`，只保存 9,713
项 inventory，缺少 framework Wave 2 的 11 个 Journal identity 与 2 个签名变化。

## 触发条件与影响

若 framework 先删除 SDK-owned 路径，独立 SDK main 将同时失去可验证的迁移谱系和最新
accepted contract/inventory payload，无法证明删除前后的来源连续性与 framework compatibility。

## 证据

`MIGRATION-SOURCE.md`、`Cargo.lock`、`echo-sdk-host/Cargo.toml`、
`contracts/sdk/inventory-telemetry.json` 和 Git ancestry 展示旧 pin、9,713 payload 与 squash 缺口。

## 处理记录

修复分支以 merge commit 恢复 `b80cf06`、`89d18f0`、`6f743d1` 和 SDK main 的共同 ancestry，
并使用唯一 generator 把 Host pin、accepted contract、inventory telemetry、三语言 catalog 与文档
同步到 framework 最终 main revision `27c7701e1eb116db1076da7f84bb68898544a44c`。本 Finding 保持 open，
等待完整本地门禁、独立复审、远端 CI 与 merge ancestry 对账。
