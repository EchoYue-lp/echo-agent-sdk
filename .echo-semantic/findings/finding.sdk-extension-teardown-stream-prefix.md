---
schema_version: 1
id: finding.sdk-extension-teardown-stream-prefix
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: failure_concurrency
focus:
- contract_evidence
- time_lifecycle
boundary_ref: boundary.sdk-extension-bridge
behavior_refs:
- behavior.sdk-family-bridge
- behavior.sdk-family-channels
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-extension-teardown-stream-prefix-candidate
- evidence.sdk-extension-teardown-prefix-repair
- evidence.sdk-extension-teardown-prefix-verification
audit_refs:
- audit.sdk-extension-teardown-prefix-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-extension-teardown-prefix-repair
verification_evidence_refs:
- evidence.sdk-extension-teardown-prefix-verification
rereview_audit_refs:
- audit.sdk-extension-teardown-prefix-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Teardown保原issuedstream前缀

## 问题

旧closingadmissionterminal-only过滤丢Chunk1却允许Complete2，被原remoteledger按非连续sequence拒绝。

## 触发条件与影响

真实官方ACP初始化、原ExtendedCloseOwner和同Profile/Registry关闭路径，原issuedstream Chunk1Complete2，owner/gen/bytes必须完整且仅沿原sinkauthority。

## 证据

旧过滤同case实际红1→fa6保前缀绿1。额外actualreturned missingterminalguard唯一删除真实挂10.26秒红，restoremtime首次旧artifact失败分类stale_build_cache，freshCompiled48.19后同after绿1(.32)。17counterbytes/SHA、bridge43ec/handlerfa6独立ActorfinalfinitePASS/action0。

## 处理记录

repair/verification/rereview三引用齐备，精确本地I2prefixbugresolved。初fixturecloseowner错误、终止compile和staleartifact都保原attempt不当行为绿。原四Custom路径有限通过不代表最新wholeHost162/新bin/finalpin；最终生成、wholeSDK匹配源及main仍另验。
