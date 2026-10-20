---
schema_version: 1
id: audit.sdk-dependency-duplicate-policy-rereview
kind: audit
boundary_ref: boundary.sdk-product
lens: contract_evidence
freshness: examined
revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
finding_refs:
- finding.sdk-dependency-warning-policy-conflict
challenges:
  bounded_duplicate_rule_implementation:
    revision: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
    source_refs:
    - AGENTS.md
    - deny.toml
    - docs/sdk/governance/dependency-warning-policy-proposal.md
    - docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/baseline-comparison.json
    - docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/audit-receipt.json
    - docs/sdk/governance/evidence/sdk-current-dependency-policy-after-feature-wiring/deny-receipt.json
    evidence_refs:
    - evidence.sdk-dependency-duplicate-human-decision
    - evidence.sdk-dependency-duplicate-policy-implementation
    - evidence.sdk-dependency-duplicate-policy-verification
  exact-main-1e1-delivery-qualification:
    revision: 1e1c4f03f670561d6ca804a56d35967eeea191ce
    source_refs:
    - AGENTS.md
    - Cargo.lock
    - deny.toml
    - .github/workflows/rust-ci.yml
    - docs/sdk/governance/dependency-warning-policy-proposal.md
    evidence_refs:
    - evidence.sdk-dependency-duplicate-human-decision
    - evidence.sdk-dependency-duplicate-policy-implementation
    - evidence.sdk-dependency-duplicate-policy-verification
---

# 有界duplicate规则实施复审

## 审查范围

sdk_capability_acceptance_audit实际source规则复审PASS0C0I0M由root原消息回收：SAGENT/Top同步归属/acceptedpolicy1bd944/old4c940/用户答date/50exacttable/locka42/原19raw noextraexceptions。

## 已检查故障假设

警告条数替代crate+版本身份、自动未来增量豁免、duplicate例外扩compiler/advisory/license/source、工具exit0替零warning、把人的特定决定当其它scope批准。

## 实际实现路径与证据

只有SDK AGENTS限定已受审表及源cut，完整输出保，未来pin/lock/features/policy须fresh工具验证+复审；原tool实际0与50warning都保，不隐藏或加4ignore。

## 问题记录

这项精确规则authorityConflict本地三链一致，可resolved，不是漏洞RiskAccepted。

## 残余风险

其它12source/B5/外部交付及全SDKgen/main另验。

## 未检查项

未来依赖图/新versiongroup以及非duplicate工具政策。


## 2026-10-20 精确main交付后继

主header revision/freshness和全部原challenge保持原观察，本具名后继只绑定精SDKmain 1e1c4f03f670561d6ca804a56d35967eeea191ce。PR5已MERGED，GitHub签名verified/valid；原push CI [39394908918](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39394908918)确实执行该head，9个job均自然SUCCESS。

精main原dependency-policy job118535120314的完整输出逐warning块提取crate及全部版本：49组与本次356锁已审49组完全相等，相对原人批准50组仅zip组已不存在，没有新增crate、版本或警告类别。Cargo.lock SHA25658599a9f95c7b524fe63c97a560d2bac235539570b2ed9ed6165586c793444a3，deny.toml SHA256b18307d3e0afa05d0c1ad32004b59cb0a7fe62136e90037bec17738545b3013f；audit和deny四类检查正常结束，warning类别仅duplicate，完整raw保留。精确组比较证明3981B/SHA256e07d436a2ef7344c3d151d515ea056a99effd28ab8bb7e522800af5e77c0d2bf。继续仅SDK精确锁版本组例外；不扩展到compiler、其它scanner或CLI后继依赖告警，也不授权未来版本增量。
