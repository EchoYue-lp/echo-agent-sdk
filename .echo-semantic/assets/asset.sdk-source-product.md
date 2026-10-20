---
schema_version: 1
id: asset.sdk-source-product
kind: asset
title: 独立 echo-agent SDK 产品
asset_type: protocol
status: active
risk: high
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
boundary_refs:
- boundary.sdk-product
code_refs:
- echo-sdk-protocol/src/lib.rs
- echo-sdk-host/src/lib.rs
- contracts/sdk/accepted-external-contract.json
- contracts/sdk/source-contract.json
- contracts/sdk/inventory-telemetry.json
- sdks/typescript/src/index.ts
- sdks/python/src/echo_agent_sdk/__init__.py
- sdks/java/pom.xml
- docs/sdk/README.md
- scripts/check-sdk-contracts.sh
- scripts/check-sdk-inventory-telemetry.sh
- scripts/check-language-sdks.sh
- scripts/export-language-sdk-catalog.sh
consumer_refs:
- README.md
- README.zh.md
- .github/workflows/rust-ci.yml
behavior_refs:
- behavior.sdk-repository-ownership
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-source-import
- evidence.sdk-source-continuity
- evidence.sdk-framework-pin
- evidence.sdk-wave2-cutover-repair
- evidence.sdk-wave2-cutover-final-verification
- evidence.sdk-compatibility-gates
finding_refs:
- finding.sdk-wave2-cutover-integrity
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

## 当前治理快照限制

上述已验证行为绑定历史 `30d7e219`。本轮显式 framework 升级、扩展桥/Task/事件/资源修复及三语言变化仍在实现和反例验证中；当前源码摘要、合同生成、独立复审和 main 验收完成前，不恢复 verified/closure 声明。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
