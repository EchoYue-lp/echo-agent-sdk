---
schema_version: 1
id: audit.sdk-wave2-cutover-final-rereview
kind: audit
boundary_ref: boundary.sdk-product
lens: contract_evidence
freshness: examined
revision: 146f69a923e7df02417528c3dce6533312be85e1
finding_refs: [finding.sdk-wave2-cutover-integrity]
challenges:
  preserved-source-ancestry:
    revision: 146f69a923e7df02417528c3dce6533312be85e1
    source_refs: [MIGRATION-SOURCE.md, docs/adr/0001-sdk-repository-boundary.md]
    evidence_refs: [evidence.sdk-source-continuity, evidence.sdk-wave2-cutover-final-verification]
  final-framework-pin:
    revision: 146f69a923e7df02417528c3dce6533312be85e1
    source_refs: [Cargo.lock, echo-sdk-host/Cargo.toml, scripts/check-sdk-contracts.sh]
    evidence_refs: [evidence.sdk-wave2-cutover-final-verification]
  wave2-contract-payload:
    revision: 146f69a923e7df02417528c3dce6533312be85e1
    source_refs: [contracts/sdk/inventory-telemetry.json, contracts/sdk/source-contract.json, echo-sdk-protocol/src/facade.rs]
    evidence_refs: [evidence.sdk-wave2-cutover-repair, evidence.sdk-wave2-cutover-final-verification]
  exact-main-1e1-delivery-qualification:
    revision: 1e1c4f03f670561d6ca804a56d35967eeea191ce
    source_refs:
    - MIGRATION-SOURCE.md
    - Cargo.lock
    - echo-sdk-host/Cargo.toml
    - contracts/sdk/source-contract.json
    - docs/sdk/governance/semantic-history.md
    - .github/workflows/rust-ci.yml
    evidence_refs:
    - evidence.sdk-source-continuity
    - evidence.sdk-wave2-cutover-final-verification
---

# SDK Wave 2 cutover final re-review

## 审查范围

复审 squash 恢复路径、双父 ancestry、最终 framework pin、Journal identity Wave 2 payload、
protocol/Host 单向依赖、三语言合同、远端平台信号和 Issue #122 closure。

## 已检查故障假设

- SDK main 只保存 squash tree 而不保存 filtered history；
- Host 或 lockfile 仍 pin 已删除的 candidate branch；
- generator 没有吸收 Journal identity 与签名变化；
- accepted contract、language catalogs 或 telemetry 计数互相漂移；
- merge 后 source branch 删除导致 ancestry 不可恢复。

## 实际实现路径与证据

SDK main `146f69a9` 的两个父线分别是先前 main `863cd5b` 与 cutover head `c8941c6`；后者包含
`7beee9d` 双父 merge 和完整 clean-pin/source-continuity lineage。Host/lockfile 精确 pin
framework main `27c7701e`，generator、合同、Host E2E、三语言与 8/8 CI 结果一致。

## 问题记录

最终复审未发现 Finding 范围内的新反例。历史 repair Evidence 保留 candidate 阶段事实，最终
verification Evidence 与本 Audit 负责合并后闭合。

## 残余风险

完整 SDK capability inventory 与生命周期基线仍为 open，不影响本 Finding 的 repository cutover
与 provenance closure；未来 framework pin 需要独立 inventory review。

## 未检查项

未检查 binary/registry 发布、未来 framework revision 或仓库外未声明 consumer。


## 2026-10-20 精确main交付后继

主header revision/freshness和全部原challenge保持原观察，本具名后继只绑定精SDKmain 1e1c4f03f670561d6ca804a56d35967eeea191ce。PR5已MERGED，GitHub签名verified/valid；原push CI [39394908918](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39394908918)确实执行该head，9个job均自然SUCCESS。

原146f69a9双父cutover、27c7701e提取pin和当期八CI保持历史观察；不把后继main伪装为原迁移重新执行。这次PR5按已审736131b5e6124afb22e880067823fda7c9b411b8精tree squash，main单父30d7e219，main tree98c4294c0f009a0f0e993914e949942087b1d5f7与审查任务tree逐字相同。当前Host产品目标是framework356，后继CLI专用Builder采用工作不改变SDKpin或accepted合同。新空仓只从GitHubmain与35个必要公开annotated证据标签fullfetch，固定c0bc严格校验通过，证明本次squash后的恢复不依赖任务分支、本地对象或私有candidate；原迁移Finding历史三链不改、不新增迁移任务。
