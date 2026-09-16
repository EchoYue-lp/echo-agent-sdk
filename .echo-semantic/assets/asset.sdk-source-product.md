---
schema_version: 1
id: asset.sdk-source-product
kind: asset
title: 独立 echo-agent SDK 产品
asset_type: protocol
status: active
risk: high
observed_at: source:4a4b9005728bf4b6ca1298ed3d92d9b9eab0d20416602abf075116a3c293e6db
boundary_refs: [boundary.sdk-product]
code_refs: [echo-sdk-protocol/src/lib.rs, echo-sdk-host/src/lib.rs, contracts/sdk/accepted-external-contract.json, contracts/sdk/source-contract.json, contracts/sdk/inventory-telemetry.json, sdks/typescript/src/index.ts, sdks/python/src/echo_agent_sdk/__init__.py, sdks/java/pom.xml, docs/sdk/README.md, scripts/check-sdk-contracts.sh, scripts/check-sdk-inventory-telemetry.sh, scripts/check-language-sdks.sh, scripts/export-language-sdk-catalog.sh]
consumer_refs: [README.md, README.zh.md, .github/workflows/rust-ci.yml]
behavior_refs: [behavior.sdk-repository-ownership]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity, evidence.sdk-framework-pin, evidence.sdk-wave2-cutover-repair]
finding_refs: [finding.sdk-wave2-cutover-integrity]
candidate_refs: []
---

# 独立 echo-agent SDK 产品

## 资产身份

SDK protocol、Host、合同、三语言 client、文档、生成器与 CI 的聚合产品资产。

## 来源与消费者

来源是 echo-agent 的 SDK-owned 历史和冻结源码；消费者包括 SDK users、framework compatibility 检查和 lp-agent superproject。

## 生命周期

Source continuity、framework 精确 pin、protocol/contract/Host/language 收敛与 Wave 2 inventory 接收均已映射，后续由独立仓版本与发布流程治理。

## 候选关系

没有第二个可编辑产品 owner；framework ACP adapter 是依赖，不是本资产的替代实现。

## 未知与限制

当前源码交付已通过独立构建和三语言门禁；完整 inventory 不构成跨语言 parity 承诺，发布制品仍未交付。
