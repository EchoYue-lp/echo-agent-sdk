---
schema_version: 1
id: finding.sdk-task-resume-extension-invocation-conflict
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: failure_concurrency
focus:
- time_lifecycle
- state_authority
- contract_evidence
boundary_ref: boundary.sdk-task-subagent
behavior_refs:
- behavior.sdk-family-task
- behavior.sdk-family-subagent
- behavior.sdk-family-bridge
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-language-current-first-gate-failure
- evidence.sdk-stream-cancellation-routing-verification
audit_refs:
- audit.sdk-stream-cancellation-routing-rereview
decision_refs: []
repair_evidence_refs:
- evidence.sdk-stream-cancellation-routing-repair
verification_evidence_refs:
- evidence.sdk-stream-cancellation-routing-verification
rereview_audit_refs:
- audit.sdk-stream-cancellation-routing-rereview
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# Task pause恢复A2在extension准入冲突

## 问题

官方全语言门禁中exactpause场景原TaskStore12CAS记录A恢复pending/running后failed extension_conflict/exclusiveinvocation，真实callback没有A2。

## 触发条件与影响

A paused→resume，B/C已completed并保持canonicalresult，D仍pending，现不可宣同原ownerresume可用。immutable8d819/bridge43ec/mod7f1为actualepoch。

## 证据

原完整日志/runnerreceipt/TaskStore/配置已原字节归档。TS287中285pass2fail0skip，PythonJava尚未执行；另OS35sharedroot失败不是此根因。

## 处理记录

bridgeowner正在debug，尚不裁责任为SDK或framework，也不将失败归纯fixturetimeout。只登记Task/extension生命周期具体边界；原EOF有限resolved不回滚/不扩大证明。待actualrootcause/反例/修复/验证/复审三链齐备才closed，无新产品scope。

## 已闭合的客户端子根因

setupACK后过早删原invocation取消地址已3client真实红绿、9source/27archive、TS19PY39J27与独立PASS三链修复，记录该子根因有限resolved，不建立重复全familyFinding。原whole287/285/2失败与210file真实timeline保。

## 仍开放的Framework物理结算义务

另原outerselect提前Drop与5秒强abort物理结算由frameworkDowner独立处理；真实5.10秒claimcleared/Cancelled但originalproducer未停止仍open。协议取消地址到达不证明物理工作停止，不能以nativecooperative绿或本client三链关闭这个宽TaskFinding。framework proposed design docs/supreme/specs/2026-10-14-subagent-physical-settlement/design.md（三语言不持第二scheduler）等待独立设计review/必要人兼容选择；原诊断私Git286715f9/ref diagnostics/subagent-stream-grace-held可恢复。SDK只引用该上游链，不独自风险接受或更改5秒policy。
