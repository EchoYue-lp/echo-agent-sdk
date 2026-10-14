---
schema_version: 1
id: finding.sdk-feature-failure-cause-fidelity
kind: finding
type: implementation_bug
status: open
severity: high
primary_focus: contract_evidence
focus:
- state_authority
- failure_concurrency
- time_lifecycle
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-integrations
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
audit_refs:
- audit.sdk-tools-bridge-finite-contract-evidence
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 30d7e219cc7d3aef0f75a19217c82898c363f50d
---

# 框架 feature 错误原因被 adapter 简化

## 问题

Workflow cancel、Memory unsupported、Eval Factory failed cause 在 typed RPC/stream 中丢失，需复用 canonical 转换；当前三条真实红绿已取得，最终快照复审与三语言证据未闭合。

## 触发条件与影响

实际主路径 `echo-sdk-host/src/core_profile/facade/workflow.rs`；具体证据范围与处理历史见 `docs/sdk/governance/finding-ledger.md`。

## 证据

修复反例与 focused 命令仍需绑定最终 source/revision 和独立复审；不把候选、局部绿色或 catalog 标记当全能力闭合。

## 处理记录

本 Finding 保持 open。最终 repair、verification、rereview 与 consumer 证据闭合后再变更状态；不影响历史外部 Issue 状态。

## Tool/LLM remote stream 子范围局部闭合

后继69b/c8dc runtime真实Git及5bcb原取证恢复：Tool远端Cancelled/Failed(extension_timeout)原Permanent/Stop/None错误由合法descriptor同test3red→after3green修复，三个risk的分类/恢复/副作用按frameworkToolFailure::from_error保持；Llm原callerToken未取消的remoteCancelled由Other/Failed变原AgentCancelled。实际官方ACP注册proxy消费/ToolManager Complete，非镜像mapper或本地token取消；原lease0、scopeClippy/fmt/独立source及材料review有限PASS。该specific子根因可局部resolved，不为它新增重复宽Finding；本对象整体仍open，因为原Workflow/Memory/Eval宽错误义务各仍需自己证据链，不能借这个3case全部关闭。
