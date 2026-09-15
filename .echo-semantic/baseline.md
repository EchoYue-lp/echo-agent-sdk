---
schema_version: 1
id: baseline.repository
kind: baseline
source_snapshot:
  base_revision: ea21dfc58fa576296a0b1d0d3267c9632d84f0ae
  content_digest: dfb25170ad6ddac42bf3294b902a2815a34970ae1fb74dec4e905dd386b9e80c
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

当前 bootstrap 快照绑定 source-import 分支和排除本目录后的源码摘要。

## 仓库区域

Rust protocol/Host、合同和三语言 SDK 是生产范围；文档、CI、脚本和根配置是支撑范围。

## 能力图与边界

当前只建立 SDK 产品 ownership 边界；后续 protocol、Host lifecycle、external contract 与各语言
投影会在独立收敛阶段展开。

## 覆盖网格

库存和行为模型保持 open，尚未声称八个风险视角全覆盖。

## 未知与缺口

Protocol 仍依赖 framework core，CI 与文档仍是 source-import 状态，动态入口见 Discovery。

## 闭合结论

该基线可用于迁移预检和严格源码快照校验，但不代表 SDK 已可独立运行或兼容合同已闭合。
