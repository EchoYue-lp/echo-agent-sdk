---
schema_version: 1
id: audit.sdk-managed-retirement-rereview
kind: audit
boundary_ref: boundary.sdk-extension-bridge
lens: contract_evidence
freshness: examined
revision: 56929154c1851473e9e4837901b610cecf001b2d
finding_refs:
- finding.sdk-managed-restart-passive-host-exit-timeout
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
  managed-passive-exit-unknown-frontier:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
    evidence_refs:
    - evidence.sdk-managed-retirement
    - evidence.sdk-cancellation-observation
  managed-first-linux-pass-without-production-repair:
    revision: e58a3e0c07cf0b6b24d570df07063301a888ddc6
    source_refs:
    - echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
    - echo-sdk-host/src/main.rs
    - echo-sdk-host/src/bounded_stdio.rs
    - echo-sdk-protocol/src/methods.rs
    - Cargo.lock
    - .github/workflows/rust-ci.yml
    evidence_refs:
    - evidence.sdk-cancellation-observation
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


## 2026-10-20 被动退出未知义务的明确关联

本Audit的原managed数据退役/receipt消费PASS不等首Host自然被动退出证明。现明确关联finding.sdk-managed-restart-passive-host-exit-timeout，保其implementation_bug/open及repair三链空；历史0ebe/F00782原Linux10秒timeout根因未知，同F后继Linuxbridge28/0/0与208a本机whole05/12通过只为各自回归，不冒因果repair。当前04eb原测试保10秒被动wait及已有phase诊断，无删断言/kill/retry-as-pass，也未新增CI。引用的旧managed Evidence仅其数据退役范围，cancellation Evidence明确被动退出未知，两者不能相互代替。

最终精SDKmain Linux原sdk-host整bridge及原日志会提供当前回归；即使通过，仍须分别记当前回归和历史根因未知，不凭之resolved或填虚构repair。Root只在全部可执行工作完成后处理准确残余裁决；risk_accepted必须有人的decision_refs。主header revision/freshness与原challenge原件不重标，当前只新增这一具体unknown frontier。


## 2026-10-20 原被动退出超时的同源PASS反证复核

只关联finding.sdk-managed-restart-passive-host-exit-timeout的unknown，不借原managed数据退役/receipt消费PASS当Host被动退出repair。失败0ebe→首次Linux成功精e58a3e0c07cf0b6b24d570df07063301a888ddc6时，Host生产src树b601874d724c56f68c1d22cccc1fb814bb86f3f0、Protocol src树6fff1a9cd6b8bfd8167b88dc4b1c6b24c717c330、锁blob017220fd23f465cbdbbced2c87da03a01e7050d8和CI blob96ae6cbc5bd46f712a55096d74040ca8d42c0140逐字同，依赖F00782/ACP2.1/Tokio1.53.1同；仅原10秒失败后诊断增量。原日志SHA各自独核及该实际Git比较支持“没有首次PASS生产repair可归因”，不支持原卡点已知或已修。

原首wait10s阶段缺Host EOF/cleanup callback Run/runtime退出现场；Responder/sender与transport stdin所有权链未证确定自持环，EOF pendingError/cancel释放原guard/lease到F drain的既有路径不能证明失败当次已走完。当前main/boundedstdio同旧blob，后Factory/CustomAgent修复或a17当前PASS不得借作此因果。保持Finding OPEN/typeimplementation_bug/repair空和原测试10s/phase诊断；最终main Linux原回归只补当前资格，历史unknown需人最后明确裁决或后继真实RCA，不在本Audit伪riskaccept/newCI。

主header revision/freshness和原挑战原件不变；新challenge精指已有必要e58源码tag，是历史反证复核，不把cancellation/managed其它Evidence外推为本缺陷修复或当前全SDK无泄漏。
