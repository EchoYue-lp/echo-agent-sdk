---
schema_version: 1
id: audit.sdk-managed-retirement-rereview
kind: audit
boundary_ref: boundary.sdk-extension-bridge
lens: contract_evidence
freshness: examined
revision: 56929154c1851473e9e4837901b610cecf001b2d
finding_refs: []
challenges:
  actual-source-and-consumer-boundary:
    revision: 56929154c1851473e9e4837901b610cecf001b2d
    source_refs:
    - docs/sdk/governance/evidence/sdk-successor-model-reviews/managed-retirement.json
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
    evidence_refs:
    - evidence.sdk-managed-retirement
---

# managed-retirement-rereview

## 审查范围

sdk_capability_acceptance_audit原独立limitedpass0，由parent回收：三语言各2场景与backend20（13原+7退役）；当前真实forward clear/retire消费及Unsupported无突变检查，只新增第八个实消费者。

## 已检查故障假设

旧wholecounts覆盖后继源码、typedwrapper丢字段、默认成功掩失败、callback mock替真实Host、跨epoch错误借用。

## 实际实现路径与证据

具名源码、原receipt与rawhash按引用读取；scope限定结论不扩为全部SDK审查。

## 问题记录

见独立原结论及关联Finding；不同缺陷不继承此pass。

## 残余风险

当前最终合同生成、无patchpin与whole门禁/main单独待验。

## 未检查项

scope外操作、任意组合与后继Channelforward新实现。

## 本次 current_source_rereview

当前SDK真实Git`56929154c1851473e9e4837901b610cecf001b2d`/source17cf，独立reader`sdk_capability_acceptance_audit`确实重新读取原业务故障假设、当前路径/变化owner/cfg/aliases和现14/newbb4f强场景，消息回收UTC`2026-10-14T00:39:15.355735+00:00`不是原运行时间。结果：**PASS / 0C0I0M（本次有界源码业务复审）**。原CodeAudit仍业务角色，不降为receiptAudit；原正文/历史sourcemarker`source:pending`/旧rawepoch保。

本次已检查：Originalcanonicalclear leads actual eighthreverseconsumer/CASretirement once;Unsupported rejects mutation;lostACK/restart sameexactreceipt；Scope-retirement/delete3reverseDTO support vsnoacceptedforwardconsumer remainsRustHostboundary not newSDKAPI;explicit stablecaller identity。当前函数定位：retire_runtime_generation_with_context；begin_scope_retirement_with_context；continue_scope_retirement_with_context；clear_persisted_runtime_incarnation；delete_persisted_conversation。实际afterproof范围：Newbb4f ManagedRetirementHostTest 2each +20backend/Taskstore executionCAS;not all11forwardoperations。

上方追加source_refs逐569Gitblob=currentfileSHA精确，现14/newbb4f632+54 sourcecut实际范围匹配。原FaultHypothesis文本保留并在本次业务代码复核，源码变化闭包按原scope检查；statusexamined不等全无缺陷，supports/limits/41coverage/closure不提升。Framework业务源属于独立publishedGitb909，对native工具用既有pin/buildmanifest定位，不将framework文件伪置SDKtree。原历史整仓3ea/9e缺失仍未知，不把原run重新标为569。full28gen/三兼容人选/LinuxWindows/main仍分开。
