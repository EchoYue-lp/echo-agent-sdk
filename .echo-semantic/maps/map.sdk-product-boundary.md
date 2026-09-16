---
schema_version: 1
id: map.sdk-product-boundary
kind: capability_map
title: 独立 SDK 产品边界
risk: high
observed_at: source:4a4b9005728bf4b6ca1298ed3d92d9b9eab0d20416602abf075116a3c293e6db
boundary_refs: [boundary.sdk-product]
behavior_refs: [behavior.sdk-repository-ownership]
rule_refs: [rule.framework-runtime-authority]
evidence_refs: [evidence.sdk-source-import, evidence.sdk-source-continuity, evidence.sdk-framework-pin, evidence.sdk-wave2-cutover-repair, evidence.sdk-wave2-cutover-final-verification]
finding_refs: [finding.sdk-wave2-cutover-integrity]
audit_refs: [audit.sdk-wave2-cutover-final-rereview]
related_map_refs: []
scenarios:
  source-history:
    status: mapped
    source_refs: [MIGRATION-SOURCE.md, docs/adr/0001-sdk-repository-boundary.md]
    behavior_refs: [behavior.sdk-repository-ownership]
    evidence_refs: [evidence.sdk-source-continuity, evidence.sdk-wave2-cutover-repair]
    finding_refs: [finding.sdk-wave2-cutover-integrity]
  protocol-and-host:
    status: mapped
    source_refs: [echo-sdk-protocol/Cargo.toml, echo-sdk-host/Cargo.toml, echo-sdk-host/src/core_profile/wire.rs]
    behavior_refs: [behavior.sdk-repository-ownership]
    evidence_refs: [evidence.sdk-framework-pin, evidence.sdk-wave2-cutover-repair]
    finding_refs: [finding.sdk-wave2-cutover-integrity]
  accepted-contract:
    status: mapped
    source_refs: [contracts/sdk/accepted-external-contract.json, contracts/sdk/source-contract.json, contracts/sdk/inventory-telemetry.json]
    behavior_refs: [behavior.sdk-repository-ownership]
    evidence_refs: [evidence.sdk-framework-pin, evidence.sdk-wave2-cutover-repair]
    finding_refs: [finding.sdk-wave2-cutover-integrity]
  language-clients:
    status: mapped
    source_refs: [sdks/shared/contract-digests.json, sdks/typescript/src/index.ts, sdks/python/src/echo_agent_sdk/__init__.py, sdks/java/pom.xml]
    behavior_refs: [behavior.sdk-repository-ownership]
    evidence_refs: [evidence.sdk-framework-pin, evidence.sdk-wave2-cutover-repair]
    finding_refs: [finding.sdk-wave2-cutover-integrity]
  wave2-cutover:
    status: mapped
    source_refs: [Cargo.lock, echo-sdk-host/Cargo.toml, echo-sdk-protocol/src/facade.rs, contracts/sdk/inventory-telemetry.json, sdks/shared/contract-digests.json]
    behavior_refs: [behavior.sdk-repository-ownership]
    rule_refs: [rule.framework-runtime-authority]
    evidence_refs: [evidence.sdk-wave2-cutover-repair]
    finding_refs: [finding.sdk-wave2-cutover-integrity]
---

# 独立 SDK 产品边界

## 能力范围

覆盖 SDK source history、Rust protocol/Host、合同与 TypeScript、Python、Java clients。

## 入口与输出

入口是 ACP standard 与 `_echo_agent/*` extension；输出是 typed response、event、stream、handle 和稳定错误。

## 行为关系

SDK 只投影 framework runtime；clean pin 将 framework 转换集中在 Host，不改变运行权威。

## 状态与数据流

Framework 拥有 Agent/Run/Task/Subagent 与 terminal 状态；Host 只持有协议寻址和资源生命周期。

## 策略来源与优先级

本仓 ADR、accepted contract 和精确 framework dependency 依次约束 repository、wire 与 build 行为。

## 生命周期与失败路径

Protocol DAG、framework pin、accepted contract、Host E2E 或语言 gate 失败会阻止交付声明；
完整 Rust inventory 漂移只形成遥测信号。

## 权限与敏感信息

SDK 不新增线上权限模型，凭据不得写入合同、日志或迁移证据。

## 用户侧投影

三语言保留惯用 API，同时不得复制 framework 的状态机和恢复权威。

## 场景处置清单

历史、ownership、protocol/Host、accepted contract 和三语言独立门禁均已映射。

## 未展开项

完整 capability 分类和生命周期将在后续 SDK 收敛阶段展开。
