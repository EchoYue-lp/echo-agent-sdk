---
schema_version: 1
id: finding.sdk-trace-publication-quota
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-integrations
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-integrations-finite-observation
audit_refs:
- audit.sdk-integrations-finite-trigger-input
- audit.sdk-integrations-finite-result-side-effect
decision_refs: []
repair_evidence_refs:
- evidence.sdk-integrations-finite-observation
verification_evidence_refs:
- evidence.sdk-integrations-finite-observation
rereview_audit_refs:
- audit.sdk-integrations-finite-trigger-input
- audit.sdk-integrations-finite-result-side-effect
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Trace constructor失败先发布不可回收resource占配额

## 触发条件与影响

当前源码路径：`echo-sdk-host/src/core_profile/facade/observability.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

源码确定；quota1失败后合法创建回归已编写待执行。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

Trace constructor失败先发布不可回收resource占配额。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 具体原缺陷本地闭合

maxresources1 unknownmode/jsonlmissingpath/typedwrongpath拒后真正memoryopen成功storeslen1(Registry)，原constructor在register前故不占slot；原05log78ok。不伪IOpermissionfault红测。 原修复/准确postfixstrongcase及本session原source+closure复核三链齐，具体Bug局部resolved；旧正文待测/待main文字保作历史。当前共享Evidence包含本rootrepair与verification，Audit按具体视角/role不是泛用PASS。无需等待SDKmain才承认原Bug已解决，但平台/fullgenerator/12/5秒B/widecleanup仍独立，不外部Issueclose；未承诺网络/全物理工作保证不扩。
