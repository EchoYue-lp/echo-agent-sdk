---
schema_version: 1
id: evidence.sdk-framework-pin
kind: evidence
observed_at: 6f743d1f37b604dbeb315c75b9e608a5254dcfe2
source_refs:
  - .github/workflows/rust-ci.yml
  - AGENTS.md
  - Cargo.lock
  - deny.toml
  - MIGRATION-SOURCE.md
  - README.md
  - README.zh.md
  - contracts/sdk/accepted-external-contract.json
  - contracts/sdk/accepted-external-contract.schema.json
  - contracts/sdk/accepted-facade-operation-catalog.json
  - contracts/sdk/inventory-telemetry.json
  - contracts/sdk/parity-manifest.json
  - contracts/sdk/public-api.txt
  - contracts/sdk/schema/echo-agent-extension-v1.schema.json
  - contracts/sdk/source-contract.json
  - docs/adr/0001-sdk-repository-boundary.md
  - docs/adr/0032-sdk-contract-scope-classification.md
  - docs/sdk/README.md
  - docs/sdk/protocol.md
  - docs/sdk/sdk-core-profile.md
  - echo-sdk-host/Cargo.toml
  - echo-sdk-host/src/core_profile/facade/source_operations.rs
  - echo-sdk-host/src/core_profile/wire.rs
  - echo-sdk-host/src/features.rs
  - echo-sdk-protocol/Cargo.toml
  - echo-sdk-protocol/src/bin/export_schema.rs
  - echo-sdk-protocol/src/capability.rs
  - echo-sdk-protocol/src/error.rs
  - echo-sdk-protocol/src/event.rs
  - echo-sdk-protocol/src/facade.rs
  - echo-sdk-protocol/src/inventory.rs
  - echo-sdk-protocol/src/methods.rs
  - echo-sdk-protocol/src/schema.rs
  - echo-sdk-protocol/tests/facade_inventory.rs
  - scripts/check-language-sdks.sh
  - scripts/check-sdk-contracts.sh
  - scripts/check-sdk-inventory-telemetry.sh
  - scripts/export-language-sdk-catalog.sh
  - sdks/shared/README.md
  - sdks/shared/contract-digests.json
  - sdks/shared/facade-operation-catalog.json
supports: [behavior.sdk-repository-ownership, rule.framework-runtime-authority, asset.sdk-source-product]
limitations: ["验证在 macOS 本地完成；Windows Host 编译与 Linux job 由独立 SDK CI 继续提供平台信号。"]
command_results:
  - { command: "cargo fmt --all -- --check", exit_code: 0 }
  - { command: "cargo check --workspace --all-features --locked", exit_code: 0 }
  - { command: "cargo test -p echo-sdk-protocol --locked", exit_code: 0 }
  - { command: "cargo test -p echo-sdk-host --features sdk-core-profile --lib --tests --locked", exit_code: 0 }
  - { command: "cargo test -p echo-sdk-host --features sdk-extension-bridge --locked --test extension_bridge_e2e", exit_code: 0 }
  - { command: "cargo test -p echo-sdk-host --features sdk-facade-all --locked --test facade_feature_adapters_e2e --test core_profile_e2e", exit_code: 0 }
  - { command: "./scripts/check-sdk-contracts.sh", exit_code: 0 }
  - { command: "./scripts/check-sdk-inventory-telemetry.sh", exit_code: 0 }
  - { command: "./scripts/check-language-sdks.sh", exit_code: 0 }
  - { command: "cargo audit --deny warnings", exit_code: 0 }
  - { command: "cargo deny check all", exit_code: 0 }
---

# SDK clean framework pin evidence

## 支持的结论

`echo-sdk-protocol` 的实际 Cargo dependency graph 不包含 `echo_agent`、`echo_core` 或其它
framework crate。Framework EventEnvelope/AgentFailure 转换集中在 Host adapter；Host 和
`Cargo.lock` 精确解析已推送 revision
`1754877996778afac4e4db77ce37c330496760ea`。

阻断兼容面由 `accepted-external-contract.json` 和
`accepted-facade-operation-catalog.json` 组成，`source-contract.json` 只摘要这两个输入。
完整 Rust public inventory、full parity manifest、Host catalog、Cargo.lock 和 framework
revision 不参与 runtime digest；`inventory-telemetry.json` 记录其来源与 scope counts。

## 运行时与语言验证

Protocol unit/contract/schema/fixture tests、Host standard/core/extension/facade E2E、TypeScript、
Python、Java 测试与三份真实 Host quickstart 均通过。Host 继续调用 framework 的 Agent、Run、
Task、Subagent、cancel、recovery 和 terminal 权威，没有引入平行状态机。

## 来源与范围

证据来源包括精确 Git dependency metadata、protocol Cargo tree、确定性合同生成、Host 三组
E2E、三语言测试与真实 Host quickstart，以及 SDK 文档和 CI 配置。

## 已知缺口

Accepted artifact、extension schema、fixture、protocol DAG 或 framework provenance 漂移会让
blocking gate 失败；只影响非 external Rust identity 的变化仅出现在独立 telemetry 报告中。
本证据不证明尚未运行的 Windows/Linux CI 环境结果，也不代表已经发布二进制或 registry 包。
依赖审计使用修复后的 `rustls` 版本与显式允许的 framework Git 源；cargo-deny 的重复版本
提示属于现有依赖图告警，不改变审计退出结果。
