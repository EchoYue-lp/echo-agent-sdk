---
schema_version: 1
id: discovery.sdk-bootstrap
kind: discovery
source_snapshot:
  base_revision: ea21dfc58fa576296a0b1d0d3267c9632d84f0ae
  content_digest: dfb25170ad6ddac42bf3294b902a2815a34970ae1fb74dec4e905dd386b9e80c
scope: independent SDK source import and repository ownership boundary
inspected_paths: [Cargo.toml, echo-sdk-protocol, echo-sdk-host, contracts/sdk, sdks, docs/sdk, scripts, MIGRATION-SOURCE.md, docs/adr]
candidate_refs: [map.sdk-product-boundary, asset.sdk-source-product]
unresolved: [echo-sdk-host/src/core_profile/facade/source_operations.rs, echo-sdk-protocol/src/bin/export_schema.rs, echo-sdk-protocol/src/facade.rs, echo-sdk-protocol/tests/facade_inventory.rs, sdks/python/src/echo_agent_sdk/client.py, sdks/python/src/echo_agent_sdk/errors.py, sdks/python/src/echo_agent_sdk/event_identity_values.py, sdks/typescript/src/thinking_profile_values.ts, sdks/typescript/test/intrinsic-tool-values.test.js]
---

# SDK bootstrap discovery

## 扫描范围

扫描 source-import workspace、协议/Host 入口、合同、三语言源码、文档、脚本与 repository ADR。

## 候选事实

仓库已具备单一 SDK 产品边界；protocol purity、external contract 分离和独立语言门禁尚待展开。

## 归并结果

当前聚合为一个产品 Asset、一个 ownership Behavior、一个 framework authority Rule 和一张开放 Capability Map。

## 未决项

列出的动态入口需要在 sdk-clean-framework-pin 阶段结合真实调用路径、生成器与测试进一步建模。
