---
schema_version: 1
id: audit.sdk-tools-bridge-finite-permission-external
kind: audit
boundary_ref: boundary.sdk-extension-bridge
lens: permission_external
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
---

# Tools/Bridge permission_external 有限审查

## 审查范围

本session实际currentTool原source审查的builtin10与reverse6role回收，原69b运行文件在5bcb真实Git相同；不是作者新审或全部identity承诺。各familyscope表见sharedEvidence，不把Chart61统计数累为其它能力。

## 已检查故障假设

原descriptorrisk/hookallowdeny/PermissionService与本地文件规则，SDK不新线上门控；外部provider只原时点成功/429/503/30sdeadline，不保证未来可用。

## 实际实现路径与证据

原工具49/61、Rag32、FileAuthority10等具名字段/文件SHA/真实PID/errorcase支持原业务；Tool/LLMremotecause另合法3red→3green官方注册consumer强断言。code/packet在真实Git5bcb，旧whole790不冒修复后新whole。原sourceindep和本材料质控分别记角色。

## 问题记录

两个原源码Important的remote子根因已具真实反例修复和独立review，scope内finitePASS。宽featurecauseFinding其它Workflow/Memory/Eval仍原未决，不能通过单scope压成全resolved或all-familyverified。

## 残余风险

已接受角色之外任意DTO/constructor无新API保证；network时点、内存非crossprocess、5秒B/12sourcekind/新A后最终wholemain分别保。暂无本组新增确定源码阻断。

## 未检查项

任意codec/network故障重试可靠导出或强kill后远端物理工作已停止，不由本scope强推；原callback所有秘密过滤也未承诺。


## 2026-10-20 具名后继的有界结算

主header revision/freshness及原挑战仍保原审查cut；这里只记录下列named单元，原17be/0bbc读审与04eb受控media增量分开，旧Evidence运行不重标。任何一个row不能推整八风险组或整composite资格。

- `behavior.sdk-extension-bridge`：仅named子链/实际after，整体仍hold；证据`evidence.sdk-channel-message-handler-consumer-audit`、`evidence.sdk-managed-retirement`、`evidence.sdk-llm-workflow-stream`、`evidence.sdk-python-typed-component-stream-verification`、`evidence.sdk-managed-supporting-operation-boundary`、`evidence.sdk-extension-teardown-prefix-verification`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-bridge`：仅named子链/实际after，整体仍hold；证据`evidence.sdk-candidate-behavior-verification`、`evidence.sdk-parent-driver-candidate-verification`、`evidence.sdk-managed-retirement`、`evidence.sdk-llm-workflow-stream`、`evidence.sdk-python-typed-component-stream-verification`、`evidence.sdk-managed-supporting-operation-boundary`、`evidence.sdk-eval-improve-topology`、`evidence.sdk-search-sandbox-06bb`、`evidence.sdk-channel-cooperative-two`、`evidence.sdk-extension-teardown-prefix-verification`、`evidence.sdk-stream-cancellation-routing-verification`、`evidence.sdk-factory-child-settlement-verification`、`evidence.sdk-tools-bridge-finite-observation`。
- `behavior.sdk-family-media`：仅named子链/实际after，整体仍hold；证据`evidence.sdk-candidate-behavior-verification`、`evidence.sdk-tool-behavior-candidate`、`evidence.sdk-computation-50`、`evidence.sdk-tools-bridge-finite-observation`、`evidence.sdk-media-accepted-route-repair-verification`。
