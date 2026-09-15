---
schema_version: 1
id: baseline.repository
kind: baseline
source_snapshot:
  base_revision: ea21dfc58fa576296a0b1d0d3267c9632d84f0ae
  content_digest: 4b0bfff26c8072e1ab77b20c2c458d10e49ea91fe1896a1b3a8fe64db6d9da8f
inventory_closure: open
behavior_model_closure: open
map_refs: [map.sdk-product-boundary]
regions:
  - { path: .cargo, status: supporting }
  - { path: .github, status: supporting }
  - { path: .gitignore, status: supporting }
  - { path: AGENTS.md, status: supporting }
  - { path: Cargo.lock, status: supporting }
  - { path: Cargo.toml, status: in_scope }
  - { path: LICENSE, status: supporting }
  - { path: MIGRATION-SOURCE.md, status: supporting }
  - { path: README.md, status: supporting }
  - { path: README.zh.md, status: supporting }
  - { path: contracts, status: in_scope }
  - { path: deny.toml, status: supporting }
  - { path: docs, status: supporting }
  - { path: echo-sdk-host, status: in_scope }
  - { path: echo-sdk-protocol, status: in_scope }
  - { path: rust-toolchain.toml, status: supporting }
  - { path: scripts, status: supporting }
  - { path: sdks, status: in_scope }
boundaries:
  - { id: boundary.sdk-product, map_ref: map.sdk-product-boundary, risk: high }
coverage: []
---

# echo-agent-sdk 语义基线

## 源码快照

当前快照绑定 SDK clean-pin 工作树和排除本目录后的源码摘要。

## 仓库区域

Rust protocol/Host、合同和三语言 SDK 是生产范围；文档、CI、脚本和根配置是支撑范围。

## 能力图与边界

SDK 产品 ownership、protocol/Host 依赖边界、accepted external contract 与三语言投影已映射；
更细粒度 capability/lifecycle 治理仍按 open 基线逐步展开。

## 覆盖网格

库存和行为模型保持 open，尚未声称八个风险视角全覆盖。

## 未知与缺口

完整 capability inventory 和八视角覆盖尚未闭合，动态入口见 Discovery；这不影响本次
protocol purity、framework pin 与 contract boundary 的已验证结论。

## 闭合结论

该基线可用于迁移预检和严格源码快照校验；独立 Host、合同与三语言门禁由
`evidence.sdk-framework-pin` 证明，发布状态仍由后续仓库交付决定。
