---
schema_version: 1
id: audit.sdk-computation-50-rereview
kind: audit
boundary_ref: boundary.sdk-integrations
lens: contract_evidence
freshness: examined
revision: 56929154c1851473e9e4837901b610cecf001b2d
finding_refs:
- finding.sdk-chart-dimension-integer-loss
challenges:
  actual-source-and-consumer-boundary:
    revision: 56929154c1851473e9e4837901b610cecf001b2d
    source_refs:
    - docs/sdk/governance/evidence/sdk-successor-model-reviews/computation-50.json
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - sdks/shared/testing/computation_fixture.py
    - docs/sdk/governance/evidence/task-context-consumption/rust-gates-reviewed-final/final-receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-before.json
    - docs/sdk/governance/evidence/sdk-language-factory-final/source-after.json
    evidence_refs:
    - evidence.sdk-computation-50
    - evidence.sdk-chart-dimension-integer-loss-current-source
  named-settlement-family-invoke:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    evidence_refs:
    - evidence.sdk-computation-50
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
---

# computation-50-rereview

## 审查范围

sdk_capability_acceptance_audit原独立limitedpass0，由parent回收：50archive SHA c8614eac、12compiledinputs、三语言各1test/50nativecalls/19typederrors，statistics/structured/content_guard/chart/data/media直接六family；8个保护文件每调用原字节未变，不表示files API全测。

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

当前SDK真实Git`56929154c1851473e9e4837901b610cecf001b2d`/source17cf，独立reader`current_tool_boundary_review`确实重新读取原业务故障假设、当前路径/变化owner/cfg/aliases和现14/newbb4f强场景，消息回收UTC`2026-10-14T00:39:15.355735+00:00`不是原运行时间。结果：**BLOCK / Important1 Chart整数丢失；原50有限场景仍PASS**。原CodeAudit仍业务角色，不降为receiptAudit；原正文/历史sourcemarker`source:pending`/旧rawepoch保。

本次已检查：Truecomputedvalues 50row/19typederrors/9oracles,UTF8 guard spans+structuredvalidate optional/no extraAgentrepairconfig；Pivot null label collision expliciterror/nullindexnotlost/unknownaggregate rejects/defaultsum correct;8protectedfiles actualbyte effects。当前函数定位：dispatch_tool；run_bound_tool；dispatch_content_guard。实际afterproof范围：Newbb4f ComputationHostTest/TS/PY one50row group each;actualPolarsunordered normalization limited,not filesAPIfullscope。

当前b909 Chart`echo-tools/src/chart.rs` SHA`cc9ea45f5a7fd942b923c81156314a359069da96757435d3e9ef7a41e94a3c47` width/height u64→u32 wrap/negativefallback，独立新I1绑定`finding.sdk-chart-dimension-integer-loss`仍open；不会将原50PASS或未publishedN修复当此整数义务通过，现SDK569/b909仍BLOCK，不新增宽度近似接受/default规则。

上方追加source_refs逐569Gitblob=currentfileSHA精确，现14/newbb4f632+54 sourcecut实际范围匹配。原FaultHypothesis文本保留并在本次业务代码复核，源码变化闭包按原scope检查；statusexamined不等全无缺陷，supports/limits/41coverage/closure不提升。Framework业务源属于独立publishedGitb909，对native工具用既有pin/buildmanifest定位，不将framework文件伪置SDKtree。原历史整仓3ea/9e缺失仍未知，不把原run重新标为569。full28gen/三兼容人选/LinuxWindows/main仍分开。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-family-invoke`：有限named单元结算；证据`evidence.sdk-computation-50`、`evidence.sdk-candidate-behavior-verification`、`evidence.sdk-parent-driver-candidate-verification`。
