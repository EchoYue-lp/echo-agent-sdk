---
schema_version: 1
id: finding.sdk-inventory-acceptance-authority
kind: finding
type: authority_conflict
status: resolved
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- failure_concurrency
- time_lifecycle
boundary_ref: boundary.sdk-language-clients
behavior_refs:
- behavior.sdk-language-clients
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-protocol-finite-observation
audit_refs:
- audit.sdk-protocol-finite-contract-evidence
decision_refs: []
repair_evidence_refs:
- evidence.sdk-protocol-finite-observation
verification_evidence_refs:
- evidence.sdk-protocol-finite-observation
rereview_audit_refs:
- audit.sdk-protocol-finite-contract-evidence
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 生成分类擅自扩大 accepted identity 合同

## 问题

完整库存与 catalog 自动接受新增可路由身份；唯一 generator 必须以既有明确 accepted identity + kind 为来源，新身份只遥测。候选及反例验证进行中。

## 触发条件与影响

实际主路径 `echo-sdk-protocol/src/inventory.rs`；具体证据范围与处理历史见 `docs/sdk/governance/finding-ledger.md`。

## 证据

修复反例与 focused 命令仍需绑定最终 source/revision 和独立复审；不把候选、局部绿色或 catalog 标记当全能力闭合。

## 处理记录

本 Finding 保持 open。最终 repair、verification、rereview 与 consumer 证据闭合后再变更状态；不影响历史外部 Issue 状态。

## 原自动扩大接受范围逻辑的具体本地闭合

本次 sdk_capability_acceptance_audit 确实读取 inventory、export_schema、facade_inventory 的原接受权威故障与修复路径，有限源码复审 C0/I0/M0。三个修复源在真实85400与622d及本次当前逐字节相同。manifest_entries 的新 routable 身份默认 Deferred / sdk_scope=false；apply_accepted_source_scope 以既有 seed 的 defining source + ItemKind 精确匹配，不由 signature、alias、路由或数量推接受。非canonical/duplicate seed、missing/wrongkind、unknown add/invalid retire 都在 mutation 前明确拒绝。producer 在文件写入前 apply，catalog_documents 仅 canonical accepted，acceptance delta 要已批准的精确 ADR 与 full-update。

facade_inventory 八个具名真实 fixture 覆盖新可路由项不接受、alias/signature 漂移不改变既有 source/kind、missing atomic/精确退役、新 Task result 不接受兄弟字段、RustHost recovery authority 不暗升语言 API、seed 精确重现、invalid delta 原子拒绝、收集全部 missing 且 partial retire 不掩余项；inventory/new public invoke 与 export 三个反例断言也实际在正式05/08通过。没有原旧产品执行红日志就不写 executed RED；meaningful postfix 场景与本次真实源码独审支持原自动扩大身份的具体逻辑缺陷本地 resolved。

seed 未在此改动，完整28 profiles 的12 missing source 按原明确合同正确阻写，旧身份是否迁移/退役仍待用户。该正确阻断不是旧自动接受 bug 仍未修，也不是 full generator 已通过。库存量不成为能力完成分母，不自动接受 API、关闭全部模型或宣称 SDK main 完成。
原正文“候选进行中/待最终引用”是历史阶段，保原字节；本次 tri-chain 仅结清这个实际 classifier 权威冲突，source continuity 的12项 Finding 保 open。
