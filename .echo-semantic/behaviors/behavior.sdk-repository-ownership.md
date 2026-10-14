---
schema_version: 1
id: behavior.sdk-repository-ownership
kind: behavior
status: needs_review
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- data_durability
- contract_evidence
boundary: boundary.sdk-product
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
code_refs:
- AGENTS.md
- MIGRATION-SOURCE.md
- docs/adr/0001-sdk-repository-boundary.md
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-source-import
- evidence.sdk-source-continuity
- evidence.sdk-framework-pin
- evidence.sdk-wave2-cutover-repair
- evidence.sdk-wave2-cutover-final-verification
- evidence.sdk-compatibility-gates
- evidence.sdk-feature-wiring-verification
- evidence.sdk-current-reviewed-rust-gates
finding_refs:
- finding.sdk-wave2-cutover-integrity
- finding.sdk-accepted-source-continuity
- finding.sdk-dependency-warning-policy-conflict
- finding.sdk-test-feature-wiring
---

# SDK repository ownership

## 重要承诺

SDK Host、protocol、合同、三语言源码、SDK 文档、生成器与 CI 只在本仓长期维护。

## 当前行为

Source import 与 continuity 已承接完整 SDK 产品和历史；已合并的 PR #2 恢复被 squash 丢失的
双父 ancestry，并把 Host pin、accepted contract、inventory telemetry 和三语言 catalog
同步到 framework Wave 2 最终 main revision。最终本地门禁、SDK 8/8 CI 与 merge ancestry 已验证。

## 期望行为

Framework 只保留运行时权威与指向本仓的边界链接，不维护第二份可编辑 SDK 产品。

## 触发、结果与副作用

SDK 变更在本仓独立构建和验证；framework 依赖升级是显式 SDK commit，完整 Rust inventory
只提供非阻断漂移遥测。
三语言 catalog 测试直接读取 accepted artifact；完整 inventory 的 scope 数量和 intrinsic
membership 只打印遥测，分类样例不要求未接受的 identity 必须存在。

## 失败、重试与恢复

源差异、不可恢复 revision 或未推送 commit 会阻止 framework 删除和 superproject gitlink 更新。

## 证据

仓库规则、迁移说明、ADR 和 Git ancestry 共同证明 ownership。

## 裁决记录

用户确认 SDK 是依赖 echo-agent 的独立产品，并作为 lp-agent 第四个 submodule。

## 当前治理快照限制

上述已验证行为绑定历史 `30d7e219`。本轮显式 framework 升级、扩展桥/Task/事件/资源修复及三语言变化仍在实现和反例验证中；当前源码摘要、合同生成、独立复审和 main 验收完成前，不恢复 verified/closure 声明。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
