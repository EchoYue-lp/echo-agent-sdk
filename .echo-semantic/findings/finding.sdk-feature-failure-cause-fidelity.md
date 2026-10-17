---
schema_version: 1
id: finding.sdk-feature-failure-cause-fidelity
kind: finding
type: implementation_bug
status: resolved
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
repair_evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
verification_evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
rereview_audit_refs:
- audit.sdk-tools-bridge-finite-contract-evidence
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

## 原三域与remote子域具体本地闭合

后继frameworkFinal独立有限源码审真实PASS C0/I0/M0明确补齐原Workflow/Memory/Eval三个错误域：Memory1532的liveStore.search_with→framework_error/semanticunsupported测试396，category=memory/terminalfailed/code=memory/retryfalse/原operation，旧namespacevalue/len1保无mutation；Eval91bd原run_all factoryErr由FailedEvalAgent第一次取Mutex<Option<originalReactError>>而非displaytext重造，真实EvalRunner对Cancelled/Timeout/Llm429 FailedResult保原violation/清空workspace（不伪每项RPCthrow或三native全部错误域已执行）；Workflow f631 graph.run和startup/stream每eventErr原canonicalconverter，实际Host2190取消RPC完整Cancelled/Never/operation/causequartet，stream failed variant仍Cancelledcause非Complete。Memory/Eval/wire当前和622d相同；workflow当前90e6仅cfgtest LLMA→B新增，生产prefix/原错误路径不变，未在这次审/未终态测试不借成果。原最新finalRust05/13日志逐具名实际ok。 合原Tool/LLM两个remote原因root已经真3红绿/独审三链，原Finding所列Workflowcancel/Memoryunsupported/EvalfactoryFailedcause已有准确修复、运行strongassert与这次明确source独审，故局部resolved，不用未SDKmain或旧sharedEvidence当时三域待审文字长期假open。B5physical/12sourcekind/其他资源/完整协议generator/source平台/main均另义务；不扩所有未列cause类型或details新DTO。
