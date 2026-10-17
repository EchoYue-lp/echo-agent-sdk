---
schema_version: 1
id: finding.sdk-managed-store-adaptation
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- contract_evidence
- data_durability
boundary_ref: boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-extension-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-managed-retirement
- evidence.sdk-managed-supporting-operation-boundary
- evidence.sdk-tools-bridge-finite-observation
audit_refs:
- audit.sdk-tools-bridge-finite-contract-evidence
- audit.sdk-managed-retirement-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
- evidence.sdk-managed-retirement
- evidence.sdk-managed-supporting-operation-boundary
verification_evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
- evidence.sdk-managed-retirement
- evidence.sdk-managed-supporting-operation-boundary
rereview_audit_refs:
- audit.sdk-tools-bridge-finite-contract-evidence
- audit.sdk-managed-retirement-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 目标managedStore capability/deadline/revision/receipt未承接

## 触发条件与影响

当前源码路径：`echo-sdk-protocol/src/managed.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

11生产callercontextops typed DTO已补；expired无commit、unknown outcome及同receipt恢复待验证。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

目标managedStore capability/deadline/revision/receipt未承接。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 有界义务分解

已具直接证据：11个已接受callercontextDTO字段转换、executionCAS能力声明、绝对deadline前无mutation、commit后lostACK/outcomeUnknown、restart后同receipt去重。对应bridge34/managed真实三语言既有收据，各自epoch保留。

原字段转换、deadline/OutcomeUnknown/同receipt恢复缺陷需以其精确repair/verification/独立scope引用结账；不能要求没有acceptedforward的三逆DTO强造consumer。三逆DTO保schema/adapter与Ruststabledeletecaller，当前兼容delete明确拒managed、clear仅single-generationretire；这一范围取证已完成，最后全生成/pin是交付边界。

## 已审的三逆操作处置

新增staticEvidence将原“剩余3consumer必须全部跑”改为准确合同边界：不撤销accepted逆DTO，不暗接受新forwardAPI，Rust/Host调用原frameworkstabledelete。原广义缺陷状态不由静态取证自动resolved；缺原精确修复/失败签名复审refs需归档，不把无APIconsumer当新增产品裁决。

## 具体本地修复、验证与复审已完成

原 managed capability/deadline/revision/receipt 适配义务结合既有 managed-retirement 独审与 supporting-operation-boundary：missing capability 先于 mutation 拒绝，deadline 过期不 commit，commit 后丢 ACK 保 OutcomeUnknown，disconnect/restart 使用同 exact receipt，typed checkpoint 大 revision 与 subsecond 字段无损；具名 tests 在最新原05真实通过。第八个 native consumer 与 backend20 证据保原 epoch，不宣全部11 forward；三逆 DTO 保 schema/RustHost supporting 边界，无已接受 forward 不新造 API。
真实 current_tool_boundary_review 原 source review 与本次具体 closure 复核支持三链局部 resolved。原正文等待语句是原阶段历史，保原字节；不以 SDK main 未交付长期假 open。B/12/宽资源/平台/main 仍各自独立。
