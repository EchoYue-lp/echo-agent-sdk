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
- finding.sdk-managed-store-adaptation
- finding.sdk-media-accepted-tool-name-routing
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
  specific-managed-store-adaptation:
    revision: 622d7931e01556161441d22de0697f6f35f24f17
    source_refs:
    - echo-sdk-protocol/src/managed.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs#missing_managed_capability_fails_before_store_mutation
    - echo-sdk-host/tests/extension_bridge_e2e.rs#managed_persistence_deadline_and_lost_ack_preserve_commit_authority
    - echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
    - echo-sdk-host/src/core_profile/wire.rs#managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/original/05-workspace-tests.log
    - docs/sdk/governance/evidence/sdk-final-idempotency-rust-gates/combined-receipt.json
    evidence_refs:
    - evidence.sdk-tools-bridge-finite-observation
    - evidence.sdk-managed-retirement
    - evidence.sdk-managed-supporting-operation-boundary
  named-settlement-extension-bridge:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt
    - sdks/shared/testing/managed_persistence_backend.py
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/LateFactoryHostTest.java
    evidence_refs:
    - evidence.sdk-channel-message-handler-consumer-audit
    - evidence.sdk-managed-retirement
    - evidence.sdk-llm-workflow-stream
    - evidence.sdk-python-typed-component-stream-verification
    - evidence.sdk-managed-supporting-operation-boundary
    - evidence.sdk-extension-teardown-prefix-verification
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-bridge:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-protocol/src/methods.rs
    evidence_refs:
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-parent-driver-candidate-verification
    - evidence.sdk-managed-retirement
    - evidence.sdk-llm-workflow-stream
    - evidence.sdk-python-typed-component-stream-verification
    - evidence.sdk-managed-supporting-operation-boundary
    - evidence.sdk-eval-improve-topology
    - evidence.sdk-search-sandbox-06bb
    - evidence.sdk-channel-cooperative-two
    - evidence.sdk-extension-teardown-prefix-verification
    - evidence.sdk-stream-cancellation-routing-verification
    - evidence.sdk-factory-child-settlement-verification
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-chart:
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
    - evidence.sdk-tool-input-validation-candidate
    - evidence.sdk-be5-shared-chart61-ci
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-data:
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
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-database:
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
    - evidence.sdk-tool-behavior-candidate
    - evidence.sdk-ordinary-tools-b909-verification
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-files:
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
    - evidence.sdk-ordinary-tools-b909-verification
    - evidence.sdk-tool-value-natural-eof-after-candidate
    - evidence.sdk-tools-bridge-finite-observation
    - evidence.sdk-integrations-finite-observation
  named-settlement-family-media:
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
    - evidence.sdk-candidate-behavior-verification
    - evidence.sdk-tool-behavior-candidate
    - evidence.sdk-computation-50
    - evidence.sdk-tools-bridge-finite-observation
    - evidence.sdk-media-accepted-route-repair-verification
  named-settlement-family-rag:
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
    - evidence.sdk-rag-project-rules
    - evidence.sdk-rag-scalar-verification
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-research:
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
    - evidence.sdk-ordinary-tools-b909-verification
    - evidence.sdk-tool-value-natural-eof-after-candidate
    - evidence.sdk-external-timeout-oracle
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-shell:
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
    - evidence.sdk-search-sandbox-06bb
    - evidence.sdk-sandbox-natural-eof-verification
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-statistics:
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
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-family-web:
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
    - evidence.sdk-search-sandbox-06bb
    - evidence.sdk-ordinary-tools-b909-verification
    - evidence.sdk-tool-value-natural-eof-after-candidate
    - evidence.sdk-tools-bridge-finite-observation
  named-settlement-media-two-alias-repair:
    revision: 04eb6ad50982656b5a8b8ea649619b6a383065ff
    source_refs:
    - echo-sdk-host/src/core_profile/facade/tools.rs#dispatch_tool
    - sdks/shared/testing/tool_behavior_fixture.py#verify_media
    - sdks/typescript/test/tool-behaviors.test.js
    - sdks/python/tests/test_tool_behaviors.py
    - sdks/java/src/test/java/com/echoagent/sdk/ToolBehaviorsHostTest.java
    evidence_refs:
    - evidence.sdk-media-accepted-route-repair-verification
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

## finding.sdk-managed-store-adaptation 原故障义务准确复核

原 managed capability/deadline/revision/receipt 适配义务结合既有 managed-retirement 独审与 supporting-operation-boundary：missing capability 先于 mutation 拒绝，deadline 过期不 commit，commit 后丢 ACK 保 OutcomeUnknown，disconnect/restart 使用同 exact receipt，typed checkpoint 大 revision 与 subsecond 字段无损；具名 tests 在最新原05真实通过。第八个 native consumer 与 backend20 证据保原 epoch，不宣全部11 forward；三逆 DTO 保 schema/RustHost supporting 边界，无已接受 forward 不新造 API。
current_tool_boundary_review 本次真实有限审查明确该原缺陷 closeeligible；本新 challenge 保主5bcb/旧审正文及已知范围，不借宽 tools family PASS。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-extension-bridge`：仅named子链/实际after，整体仍hold；证据`evidence.sdk-channel-message-handler-consumer-audit`、`evidence.sdk-managed-retirement`、`evidence.sdk-llm-workflow-stream`、`evidence.sdk-python-typed-component-stream-verification`、`evidence.sdk-managed-supporting-operation-boundary`、`evidence.sdk-extension-teardown-prefix-verification`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-bridge`：仅named子链/实际after，整体仍hold；证据`evidence.sdk-candidate-behavior-verification`、`evidence.sdk-parent-driver-candidate-verification`、`evidence.sdk-managed-retirement`、`evidence.sdk-llm-workflow-stream`、`evidence.sdk-python-typed-component-stream-verification`、`evidence.sdk-managed-supporting-operation-boundary`、`evidence.sdk-eval-improve-topology`、`evidence.sdk-search-sandbox-06bb`、`evidence.sdk-channel-cooperative-two`、`evidence.sdk-extension-teardown-prefix-verification`、`evidence.sdk-stream-cancellation-routing-verification`、`evidence.sdk-factory-child-settlement-verification`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-chart`：有限named单元结算；证据`evidence.sdk-tool-input-validation-candidate`、`evidence.sdk-be5-shared-chart61-ci`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-data`：有限named单元结算；证据`evidence.sdk-computation-50`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-database`：有限named单元结算；证据`evidence.sdk-tool-behavior-candidate`、`evidence.sdk-ordinary-tools-b909-verification`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-files`：有限named单元结算；证据`evidence.sdk-ordinary-tools-b909-verification`、`evidence.sdk-tool-value-natural-eof-after-candidate`、`evidence.sdk-tools-bridge-finite-observation`、`evidence.sdk-integrations-finite-observation`。
- `behavior.sdk-family-media`：仅named子链/实际after，整体仍hold；证据`evidence.sdk-candidate-behavior-verification`、`evidence.sdk-tool-behavior-candidate`、`evidence.sdk-computation-50`、`evidence.sdk-tools-bridge-finite-observation`、`evidence.sdk-media-accepted-route-repair-verification`。
- `behavior.sdk-family-rag`：有限named单元结算；证据`evidence.sdk-rag-project-rules`、`evidence.sdk-rag-scalar-verification`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-research`：有限named单元结算；证据`evidence.sdk-ordinary-tools-b909-verification`、`evidence.sdk-tool-value-natural-eof-after-candidate`、`evidence.sdk-external-timeout-oracle`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-shell`：有限named单元结算；证据`evidence.sdk-search-sandbox-06bb`、`evidence.sdk-sandbox-natural-eof-verification`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-statistics`：有限named单元结算；证据`evidence.sdk-computation-50`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-web`：有限named单元结算；证据`evidence.sdk-search-sandbox-06bb`、`evidence.sdk-ordinary-tools-b909-verification`、`evidence.sdk-tool-value-natural-eof-after-candidate`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-media`：仅named子链/实际after，整体仍hold；证据`evidence.sdk-media-accepted-route-repair-verification`。
