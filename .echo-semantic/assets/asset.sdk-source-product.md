---
schema_version: 1
id: asset.sdk-source-product
kind: asset
title: 独立 echo-agent SDK 产品
asset_type: protocol
status: active
risk: high
observed_at: source:dfb25170ad6ddac42bf3294b902a2815a34970ae1fb74dec4e905dd386b9e80c
boundary_refs: [boundary.sdk-product]
code_refs: [echo-sdk-protocol/src/lib.rs, echo-sdk-host/src/lib.rs, contracts/sdk/source-contract.json, sdks/typescript/src/index.ts, sdks/python/src/echo_agent_sdk/__init__.py, sdks/java/pom.xml, docs/sdk/README.md, scripts/check-sdk-contracts.sh, scripts/check-language-sdks.sh, scripts/export-language-sdk-catalog.sh]
consumer_refs: [README.md, README.zh.md, .github/workflows/rust-ci.yml]
behavior_refs: [behavior.sdk-repository-ownership]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity]
finding_refs: []
candidate_refs: []
---

# 独立 echo-agent SDK 产品

## 资产身份

SDK protocol、Host、合同、三语言 client、文档、生成器与 CI 的聚合产品资产。

## 来源与消费者

来源是 echo-agent 的 SDK-owned 历史和冻结源码；消费者包括 SDK users、framework compatibility 检查和 lp-agent superproject。

## 生命周期

Source continuity 完成后进入独立 protocol/contract/Host/language 收敛，再由独立版本发布。

## 候选关系

没有第二个可编辑产品 owner；framework ACP adapter 是依赖，不是本资产的替代实现。

## 未知与限制

当前仍是 source-import checkpoint，不声明 runnable、parity complete 或 release readiness。
