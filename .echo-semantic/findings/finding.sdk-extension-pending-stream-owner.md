---
schema_version: 1
id: finding.sdk-extension-pending-stream-owner
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
- evidence.sdk-tools-bridge-finite-observation
audit_refs:
- audit.sdk-tools-bridge-finite-time-lifecycle
decision_refs: []
repair_evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
verification_evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
rereview_audit_refs:
- audit.sdk-tools-bridge-finite-time-lifecycle
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 初始流应答被外层Tool timeout drop漏handle和callbackguard

## 触发条件与影响

当前源码路径：`echo-sdk-host/src/core_profile/extension_bridge.rs`。具体触发序列与审查记录见 `docs/sdk/governance/finding-ledger.md`。

## 证据

真实framework ToolManager drop可达；RAII与真实Host低配额回归待验证。

本 Finding 保持 open；静态取证、代码修补或旧 pin focused 通过不能替代最终快照验证与独立复审。
## 问题

初始流应答被外层Tool timeout drop漏handle和callbackguard。

## 处理记录

修复与最新直接验证进行中；尚未完成最终精确 snapshot 独立复审，保持 open。

## 具体本地修复、验证与复审已完成

原初始应答被外层 ToolManager timeout/drop 的缺陷由原 PendingStream RAII 守护 sink、已发 stream 地址和 callback/reentry guard。具名 unit 实际 drop 后资源释放，两个真实 Host 回归分别 outer timeout before ACK 与 cancelled initial response，保原 Session owner/低配额再入强断言；最新正式原05逐名已执行通过。不是只借 teardown prefix 修复或整体测试数。
真实 current_tool_boundary_review 原 source review 与本次具体 closure 复核支持三链局部 resolved。原正文等待语句是原阶段历史，保原字节；不以 SDK main 未交付长期假 open。B/12/宽资源/平台/main 仍各自独立。
