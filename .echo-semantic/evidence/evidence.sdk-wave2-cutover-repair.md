---
schema_version: 1
id: evidence.sdk-wave2-cutover-repair
kind: evidence
observed_at: source:4a4b9005728bf4b6ca1298ed3d92d9b9eab0d20416602abf075116a3c293e6db
source_refs:
  - AGENTS.md
  - Cargo.lock
  - MIGRATION-SOURCE.md
  - README.md
  - README.zh.md
  - contracts/sdk/accepted-external-contract.json
  - contracts/sdk/accepted-facade-operation-catalog.json
  - contracts/sdk/facade-operation-catalog.json
  - contracts/sdk/inventory-telemetry.json
  - contracts/sdk/parity-manifest.json
  - contracts/sdk/public-api.txt
  - contracts/sdk/source-contract.json
  - docs/adr/0001-sdk-repository-boundary.md
  - docs/adr/0031-sdk-identity-governance-scope.md
  - docs/adr/0032-sdk-contract-scope-classification.md
  - docs/sdk/README.md
  - docs/sdk/acp-agent-adapter.md
  - echo-sdk-host/Cargo.toml
  - echo-sdk-protocol/src/facade.rs
  - echo-sdk-protocol/tests/facade_inventory.rs
  - scripts/check-sdk-contracts.sh
  - sdks/java/src/test/java/com/echoagent/sdk/FacadeParityTest.java
  - sdks/python/tests/test_catalog.py
  - sdks/shared/contract-digests.json
  - sdks/shared/facade-operation-catalog.json
  - sdks/typescript/test/catalog.test.js
supports: [behavior.sdk-repository-ownership, rule.framework-runtime-authority, asset.sdk-source-product]
limitations:
  - final local gates, independent rereview, remote CI, and post-merge ancestry verification remain pending
  - the SDK merge commit and remote CI remain required before the framework extraction Issue can close
---

# SDK Wave 2 cutover repair evidence

## 支持的结论

Merge commit `7beee9d416ac304c3cec6df6646f558496ab577d` 同时保留 SDK main 与 clean-pin
lineage，`b80cf06`、`89d18f0`、`6f743d1`、`863cd5b` 均为当前修复分支祖先。

Host manifest、lockfile、provenance gate 和正式文档精确固定已合并 framework main revision
`27c7701e1eb116db1076da7f84bb68898544a44c`。唯一 generator 产生 9,724 项 canonical
inventory，scope 为 5,622 external、1,774 Host/Rust-only、790 language intrinsic、90
internal helper 和 1,448 deferred；11 个 Journal identity 与 2 个签名变化均来自该最终 revision。

## 来源与范围

修复只更新 framework build provenance、完整 Rust inventory telemetry、由 framework public
value 进入的 accepted contract 投影、三语言 catalog 断言和迁移文档。Protocol 继续不依赖
framework，Host 继续是唯一 framework conversion boundary，没有新增 runtime 状态权威或 wire method。

## 已知缺口

Focused protocol inventory 76 项和 Host `sdk-facade-all` check 已通过；完整本地门禁、独立复审、
远端 CI 与 SDK main merge ancestry 仍待后续 Evidence 对账。
