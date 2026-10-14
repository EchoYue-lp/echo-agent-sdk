---
schema_version: 1
id: audit.sdk-tools-bridge-finite-contract-evidence
kind: audit
boundary_ref: boundary.sdk-extension-bridge
lens: contract_evidence
freshness: examined
revision: 5bcbad2437e5c19f61101883044c4f67944cb97c
finding_refs:
- finding.sdk-feature-failure-cause-fidelity
challenges:
  builtin-tool-and-reverse-bridge-obligation:
    revision: 5bcbad2437e5c19f61101883044c4f67944cb97c
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    - docs/sdk/governance/evidence/remote-stream-failure-domain/manifest.json
    - docs/sdk/governance/evidence/remote-stream-failure-domain/README.md
    - docs/sdk/governance/evidence/remote-stream-failure-domain/before-valid-descriptor-run.json
    - docs/sdk/governance/evidence/remote-stream-failure-domain/after-run.json
    - docs/sdk/governance/evidence/remote-stream-failure-domain/production-only.patch
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/receipt.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-before.json
    - docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/inputs-after.json
    - docs/sdk/governance/evidence/sdk-chart-published-pin/ci-regression/regression-review-manifest.json
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
  original-three-feature-cause-domains:
    revision: 622d7931e01556161441d22de0697f6f35f24f17
    source_refs:
    - echo-sdk-host/src/core_profile/facade/memory.rs#unsupported_store_operation_keeps_memory_cause_without_mutation
    - echo-sdk-host/src/core_profile/facade/source_operations.rs#factory_failure_retains_canonical_cause_in_real_eval_result
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_cancel_preserves_canonical_failure_in_rpc_and_stream_terminal
    - echo-sdk-host/src/core_profile/wire.rs
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/continuation/13-ci-facade-e2e.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
---

# Tools/Bridge contract_evidence 有限审查

## 审查范围

本session实际currentTool原source审查的builtin10与reverse6role回收，原69b运行文件在5bcb真实Git相同；不是作者新审或全部identity承诺。各familyscope表见sharedEvidence，不把Chart61统计数累为其它能力。

## 已检查故障假设

具名builtin10fam与reverse Tool/LLM/Store/Hook/AgentCallback/Intervention roles实际入口和typed字段；remoteTool取消/timeout使用原ToolFailure，LLM取消原AgentError，不能lossy Other/default。

## 实际实现路径与证据

原工具49/61、Rag32、FileAuthority10等具名字段/文件SHA/真实PID/errorcase支持原业务；Tool/LLMremotecause另合法3red→3green官方注册consumer强断言。code/packet在真实Git5bcb，旧whole790不冒修复后新whole。原sourceindep和本材料质控分别记角色。

## 问题记录

两个原源码Important的remote子根因已具真实反例修复和独立review，scope内finitePASS。宽featurecauseFinding其它Workflow/Memory/Eval仍原未决，不能通过单scope压成全resolved或all-familyverified。

## 残余风险

已接受角色之外任意DTO/constructor无新API保证；network时点、内存非crossprocess、5秒B/12sourcekind/新A后最终wholemain分别保。暂无本组新增确定源码阻断。

## 未检查项

任意codec/network故障重试可靠导出或强kill后远端物理工作已停止，不由本scope强推；原callback所有秘密过滤也未承诺。

## 原Workflow/Memory/Eval三域新实际独审

后继frameworkFinal独立有限源码审真实PASS C0/I0/M0明确补齐原Workflow/Memory/Eval三个错误域：Memory1532的liveStore.search_with→framework_error/semanticunsupported测试396，category=memory/terminalfailed/code=memory/retryfalse/原operation，旧namespacevalue/len1保无mutation；Eval91bd原run_all factoryErr由FailedEvalAgent第一次取Mutex<Option<originalReactError>>而非displaytext重造，真实EvalRunner对Cancelled/Timeout/Llm429 FailedResult保原violation/清空workspace（不伪每项RPCthrow或三native全部错误域已执行）；Workflow f631 graph.run和startup/stream每eventErr原canonicalconverter，实际Host2190取消RPC完整Cancelled/Never/operation/causequartet，stream failed variant仍Cancelledcause非Complete。Memory/Eval/wire当前和622d相同；workflow当前90e6仅cfgtest LLMA→B新增，生产prefix/原错误路径不变，未在这次审/未终态测试不借成果。原最新finalRust05/13日志逐具名实际ok。 该newchallenge绑定真实622d原错误路径，主5bcb原remote/工具审查revision不变；不会用sharedEvidence名字偷换scope。原宣其它三域未决是早期审查历史，真正补审后此精确root可closed。
