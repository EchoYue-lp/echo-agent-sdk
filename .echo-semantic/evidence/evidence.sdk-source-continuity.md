---
schema_version: 1
id: evidence.sdk-source-continuity
kind: evidence
observed_at: source:4b0bfff26c8072e1ab77b20c2c458d10e49ea91fe1896a1b3a8fe64db6d9da8f
source_refs: [MIGRATION-SOURCE.md, docs/adr/0001-sdk-repository-boundary.md, docs/adr/0028-source-first-multilanguage-sdk-runtime.md, docs/adr/0031-sdk-identity-governance-scope.md, docs/adr/0032-sdk-contract-scope-classification.md, contracts/sdk/source-contract.json, echo-sdk-protocol/src/facade.rs, echo-sdk-host/tests/extension_bridge_e2e.rs, sdks/shared/contract-digests.json]
supports: [behavior.sdk-repository-ownership, rule.framework-runtime-authority, asset.sdk-source-product]
limitations: ["本证据覆盖 source continuity 与文档 ownership，不证明 protocol 已纯化、accepted contract 已分离或完整独立 SDK 门禁已通过。"]
---

# SDK source continuity evidence

## 支持的结论

过滤后的 framework SDK history tip 是 `28d6709c93a7809c1eb3ba744d3e4cc89b0332f7`，
对应原 framework 冻结 revision `c5f7688212d45d5bdcdbf60342605e8bfb176cae` 的
SDK-owned 路径。任务分支通过未提交双根 merge 保留该父线与 `ea21dfc` source-import 父线。

源 revision 的 SDK-owned tree 与当前仓均有 602 个文件。Contracts、三语言、scripts、
Host/Protocol（排除各自 Cargo manifest）逐目录一致；初始 import 到冻结 tree 的实际工作树
差异为 13 个文件。两份 Cargo manifest 保留 Git dependency 适配，ACP adapter 文档将相邻
framework example 改为 `c5f7688` 的不可变外链。

## 来源与范围

来源包括 Git filtered history、`c5f7688` archive、逐目录对账、迁移说明、三份 SDK ADR、
source-first design 和 SDK 文档链接检查。

## 验证结果

过滤历史检查确认 31 个 commit、618 个文件和零越界路径；SDK tree 对账确认源/目标均为
602 个文件，排除三处已记录适配后零缺失、零额外、零 blob/mode mismatch。40 个 Markdown
本地链接、Cargo metadata、Rustfmt、全部迁入 JSON、三份 shell 脚本语法和 Java POM XML
检查均为零退出。

## 已知缺口

最终 merge commit 与远端 SHA 在本 Evidence 所属提交完成后由 Git 对账证明。Protocol purity、
framework pin、合同分离、Host E2E 与三语言完整门禁属于后续 SDK 收敛结果。
