---
schema_version: 1
id: evidence.sdk-source-import
kind: evidence
observed_at: ea21dfc58fa576296a0b1d0d3267c9632d84f0ae
source_refs: [MIGRATION-SOURCE.md, Cargo.toml, echo-sdk-protocol/Cargo.toml, echo-sdk-host/Cargo.toml]
supports: [behavior.sdk-repository-ownership, rule.framework-runtime-authority, asset.sdk-source-product]
limitations: ["当前只证明 source-import 结构和 ownership；framework c5f7688 连续性、protocol 纯化与完整 SDK 门禁尚未完成。"]
---

# SDK source import evidence

## 支持的结论

当前仓库在 `ea21dfc58fa576296a0b1d0d3267c9632d84f0ae` 保存 source-import checkpoint，
并以本仓 Cargo workspace、迁移说明和 ADR 证明独立产品结构。

## 来源与范围

来源包括迁移说明、根 workspace 和两份 Rust crate manifest。

## 已知缺口

后续 Evidence 将补充过滤历史、冻结源树对账和远端 commit；本证据不覆盖独立构建或合同闭合。
