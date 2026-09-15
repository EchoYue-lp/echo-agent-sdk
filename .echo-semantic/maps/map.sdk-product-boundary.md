---
schema_version: 1
id: map.sdk-product-boundary
kind: capability_map
title: 独立 SDK 产品边界
risk: high
observed_at: source:dfb25170ad6ddac42bf3294b902a2815a34970ae1fb74dec4e905dd386b9e80c
boundary_refs: [boundary.sdk-product]
behavior_refs: [behavior.sdk-repository-ownership]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity]
finding_refs: []
audit_refs: []
related_map_refs: []
scenarios:
  source-history:
    status: mapped
    source_refs: [MIGRATION-SOURCE.md, docs/adr/0001-sdk-repository-boundary.md]
    behavior_refs: [behavior.sdk-repository-ownership]
    evidence_refs: [evidence.sdk-source-continuity]
  protocol-and-host:
    status: needs_review
    source_refs: [echo-sdk-protocol/src/lib.rs, echo-sdk-host/src/lib.rs]
    unknown: protocol dependency purification and exact framework pin are not completed in the source-import checkpoint
    next_step: execute the subsequent SDK convergence outcome after framework extraction
  accepted-contract:
    status: needs_review
    source_refs: [contracts/sdk/parity-manifest.json, contracts/sdk/source-contract.json]
    unknown: accepted external contract and full Rust inventory are not yet separated into blocking and telemetry artifacts
    next_step: rebuild the contract gate in the independent SDK repository
  language-clients:
    status: needs_review
    source_refs: [sdks/typescript/src/index.ts, sdks/python/src/echo_agent_sdk/__init__.py, sdks/java/pom.xml]
    unknown: independent TypeScript Python and Java gates have not yet been rerun against the frozen framework revision
    next_step: run the three language suites and quickstarts after contract regeneration
---

# 独立 SDK 产品边界

## 能力范围

覆盖 SDK source history、Rust protocol/Host、合同与 TypeScript、Python、Java clients。

## 入口与输出

入口是 ACP standard 与 `_echo_agent/*` extension；输出是 typed response、event、stream、handle 和稳定错误。

## 行为关系

SDK 只投影 framework runtime；source continuity 不改变运行行为。

## 状态与数据流

Framework 拥有 Agent/Run/Task/Subagent 与 terminal 状态；Host 只持有协议寻址和资源生命周期。

## 策略来源与优先级

本仓 ADR、accepted contract 和精确 framework dependency 依次约束 repository、wire 与 build 行为。

## 生命周期与失败路径

当前是 source-import checkpoint；缺失 pin、漂移或语言失败必须阻止后续 runnable 声明。

## 权限与敏感信息

SDK 不新增线上权限模型，凭据不得写入合同、日志或迁移证据。

## 用户侧投影

三语言保留惯用 API，同时不得复制 framework 的状态机和恢复权威。

## 场景处置清单

历史与 ownership 已映射；protocol、contract 和三语言独立门禁保持 needs_review。

## 未展开项

完整 capability 分类和生命周期将在后续 SDK 收敛阶段展开。
