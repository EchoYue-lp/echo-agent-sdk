---
schema_version: 1
id: audit.sdk-historical-proof-storage-rereview
kind: audit
boundary_ref: boundary.sdk-product
lens: contract_evidence
freshness: examined
revision: 019e700279677a2eb77e265b5038d9d581bc22da
finding_refs:
- finding.sdk-historical-proof-storage-migration
challenges:
  historical_bytes_roles_and_owner_obligations:
    revision: 019e700279677a2eb77e265b5038d9d581bc22da
    source_refs:
    - .github/workflows/rust-ci.yml
    - Cargo.lock
    - Cargo.toml
    - contracts/sdk/accepted-external-contract.json
    - contracts/sdk/accepted-external-contract.schema.json
    - contracts/sdk/accepted-facade-operation-catalog.json
    - contracts/sdk/facade-operation-catalog.json
    - contracts/sdk/inventory-telemetry.json
    - contracts/sdk/parity-manifest.json
    - contracts/sdk/public-api.txt
    - contracts/sdk/schema/echo-agent-extension-v1.schema.json
    - contracts/sdk/source-contract.json
    - deny.toml
    - docs/adr/0038-sdk-historical-proof-storage.md
    - docs/sdk/governance/semantic-history.md
    - echo-sdk-host/Cargo.toml
    - echo-sdk-host/src/bounded_stdio.rs
    - echo-sdk-host/src/config.rs
    - echo-sdk-host/src/core_profile/events.rs
    - echo-sdk-host/src/core_profile/extension_bridge.rs
    - echo-sdk-host/src/core_profile/facade/integrations.rs
    - echo-sdk-host/src/core_profile/facade/memory.rs
    - echo-sdk-host/src/core_profile/facade/mod.rs
    - echo-sdk-host/src/core_profile/facade/observability.rs
    - echo-sdk-host/src/core_profile/facade/permission.rs
    - echo-sdk-host/src/core_profile/facade/registry.rs
    - echo-sdk-host/src/core_profile/facade/source_operations.rs
    - echo-sdk-host/src/core_profile/facade/state_delivery.rs
    - echo-sdk-host/src/core_profile/facade/stream.rs
    - echo-sdk-host/src/core_profile/facade/subagent.rs
    - echo-sdk-host/src/core_profile/facade/task.rs
    - echo-sdk-host/src/core_profile/facade/task_runtime.rs
    - echo-sdk-host/src/core_profile/facade/telemetry.rs
    - echo-sdk-host/src/core_profile/facade/tools.rs
    - echo-sdk-host/src/core_profile/facade/workflow.rs
    - echo-sdk-host/src/core_profile/handler.rs
    - echo-sdk-host/src/core_profile/handles.rs
    - echo-sdk-host/src/core_profile/mod.rs
    - echo-sdk-host/src/core_profile/persistence.rs
    - echo-sdk-host/src/core_profile/state.rs
    - echo-sdk-host/src/core_profile/wire.rs
    - echo-sdk-host/src/factory.rs
    - echo-sdk-host/src/features.rs
    - echo-sdk-host/src/lib.rs
    - echo-sdk-host/src/main.rs
    - echo-sdk-host/tests/core_profile_e2e.rs
    - echo-sdk-host/tests/extension_bridge_e2e.rs
    - echo-sdk-host/tests/facade_feature_adapters_e2e.rs
    - echo-sdk-host/tests/stdio_e2e.rs
    - echo-sdk-host/tests/tool_behavior_governance_e2e.rs
    - echo-sdk-protocol/Cargo.toml
    - echo-sdk-protocol/src/bin/export_schema.rs
    - echo-sdk-protocol/src/capability.rs
    - echo-sdk-protocol/src/error.rs
    - echo-sdk-protocol/src/event.rs
    - echo-sdk-protocol/src/facade.rs
    - echo-sdk-protocol/src/inventory.rs
    - echo-sdk-protocol/src/managed.rs
    - echo-sdk-protocol/src/methods.rs
    - echo-sdk-protocol/src/scalar.rs
    - echo-sdk-protocol/src/schema.rs
    - echo-sdk-protocol/tests/core_rpc_contract.rs
    - echo-sdk-protocol/tests/facade_inventory.rs
    - scripts/check-language-sdks.sh
    - scripts/check-sdk-contracts.sh
    - scripts/check-sdk-inventory-telemetry.sh
    - scripts/export-language-sdk-catalog.sh
    - sdks/java/src/main/java/com/echoagent/sdk/BoundedPublisher.java
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
    - sdks/java/src/main/java/com/echoagent/sdk/EchoAgentException.java
    - sdks/java/src/main/java/com/echoagent/sdk/EventIdentity.java
    - sdks/java/src/main/java/com/echoagent/sdk/ExtensionRegistration.java
    - sdks/java/src/main/java/com/echoagent/sdk/JsonSupport.java
    - sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java
    - sdks/java/src/main/java/com/echoagent/sdk/WireValues.java
    - sdks/java/src/test/java/com/echoagent/sdk/AckSubmissionTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/EvalImproveTopologyHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/EventIdentityTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ExternalAckHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/FacadeParityTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryCleanupLifecycleTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/FactoryHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/FamilyRouteHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/GitToolHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/HostEarlyExitTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/IntrinsicToolValueTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/McpChannelHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/MemoryPermissionHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ProtocolHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ProtocolIntegrityTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ReplayTerminalHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/SecretScanHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/StateTraceDeliveryHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/TaskSubagentHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/TelemetryHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/TestingContractTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ToolBehaviorsHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
    - sdks/java/src/test/java/com/echoagent/sdk/WorkflowCheckpointHostTest.java
    - sdks/python/src/echo_agent_sdk/client.py
    - sdks/python/src/echo_agent_sdk/event_identity_values.py
    - sdks/python/src/echo_agent_sdk/execution_usage_values.py
    - sdks/python/src/echo_agent_sdk/wire.py
    - sdks/python/tests/test_ack_submission.py
    - sdks/python/tests/test_catalog.py
    - sdks/python/tests/test_client_close_callbacks.py
    - sdks/python/tests/test_event_identity_values.py
    - sdks/python/tests/test_factory_host.py
    - sdks/python/tests/test_family_route_host.py
    - sdks/python/tests/test_git_tool_host.py
    - sdks/python/tests/test_intrinsic_tool_values.py
    - sdks/python/tests/test_llm_workflow_stream_host.py
    - sdks/python/tests/test_mcp_channel_host.py
    - sdks/python/tests/test_memory_permission_host.py
    - sdks/python/tests/test_protocol_host.py
    - sdks/python/tests/test_protocol_integrity.py
    - sdks/python/tests/test_secret_scan_host.py
    - sdks/python/tests/test_state_trace_delivery_host.py
    - sdks/python/tests/test_task_subagent_host.py
    - sdks/python/tests/test_testing_contract.py
    - sdks/python/tests/test_tool_behaviors.py
    - sdks/python/tests/test_tool_value_effect_host.py
    - sdks/python/tests/test_wire.py
    - sdks/python/tests/test_workflow_checkpoint_host.py
    - sdks/shared/contract-digests.json
    - sdks/shared/facade-operation-catalog.json
    - sdks/shared/fixtures/native-projection-invalid.json
    - sdks/shared/fixtures/testing-contract-values.json
    - sdks/shared/testing/computation_fixture.py
    - sdks/shared/testing/eval_improve_topology_fixture.py
    - sdks/shared/testing/git_tool_host_fixture.py
    - sdks/shared/testing/llm_workflow_stream_fixture.py
    - sdks/shared/testing/memory_permission_fixture.py
    - sdks/shared/testing/rag_project_rules_fixture.py
    - sdks/shared/testing/task_subagent_fixture.py
    - sdks/shared/testing/tool_behavior_fixture.py
    - sdks/shared/testing/tool_value_effect_fixture.py
    - sdks/shared/testing/workflow_checkpoint_fixture.py
    - sdks/shared/toolchain.json
    - sdks/typescript/src/client.ts
    - sdks/typescript/src/errors.ts
    - sdks/typescript/src/event_identity_values.ts
    - sdks/typescript/src/wire.ts
    - sdks/typescript/test/ack-submission.test.js
    - sdks/typescript/test/catalog.test.js
    - sdks/typescript/test/event-identity-values.test.js
    - sdks/typescript/test/factory-host.test.js
    - sdks/typescript/test/family-route-host.test.js
    - sdks/typescript/test/git-tool-host.test.js
    - sdks/typescript/test/host.test.js
    - sdks/typescript/test/intrinsic-tool-values.test.js
    - sdks/typescript/test/mcp-channel-host.test.js
    - sdks/typescript/test/memory-permission-host.test.js
    - sdks/typescript/test/protocol-host.test.js
    - sdks/typescript/test/protocol-integrity.test.js
    - sdks/typescript/test/secret-scan-host.test.js
    - sdks/typescript/test/state-trace-delivery-host.test.js
    - sdks/typescript/test/task-subagent-host.test.js
    - sdks/typescript/test/testing-contract.test.js
    - sdks/typescript/test/tool-behaviors.test.js
    - sdks/typescript/test/wire.test.js
    - sdks/typescript/test/workflow-checkpoint-host.test.js
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
  exact-main-1e1-delivery-qualification:
    revision: 1e1c4f03f670561d6ca804a56d35967eeea191ce
    source_refs:
    - docs/sdk/governance/semantic-history.md
    - docs/adr/0038-sdk-historical-proof-storage.md
    - MIGRATION-SOURCE.md
    - .github/workflows/rust-ci.yml
    evidence_refs:
    - evidence.sdk-historical-proof-storage-equivalence
---
# 历史证明存储独立有界复审

## 审查范围
仅本4365阶段证明必要性、62个非文稿强角色、原owner历史保全、924指纹映射；不重跑SDK业务/Host，不替全部最终门禁。

## 已检查故障假设
误删真实fixture/consumer、旧raw失去公开恢复、protocol历史演进假等价、XML假重跑、强角色降为doc、单选collision变体、旧business/status/supports/限制或正文丢失、多余branch恢复依赖。

## 实际实现路径与证据
独立sdk_archive_role_review实际读dd0ce/C92ef8、两actual表及所有oldowner结构/必要性；14protocol47test1prod具名承接真实，4365父与公开0ebe全blob/mode同、98目标可恢复、924FP/34collision124variant完整。实际结论C0/I0/M0；current locator统一fullfile只避免自造anchor碰撞，旧raw/anchor/FP不动。真实消息是复审依据，不制造reviewJSON。

## 问题记录
原草案Eq unknown/authority草案不是可用门禁。现仅按真实已核storage事实及本批准范围完成三链，不把有界审写为全ordinary/continuity PASS。

## 残余风险
完整strict/原父delete/30d/continuity仍待actual；Root未准Sapply。四原业务open、Host未知/物理B/平台/main及所有旧失败范围保。

## 未检查项
新remoteSDK CI、main merge、所有SDK能力新执行；工具action精pin有公开工具mainCI依据但SDK job尚未新运行。


## 2026-10-20 精确main交付后继

主header revision/freshness和全部原challenge保持原观察，本具名后继只绑定精SDKmain 1e1c4f03f670561d6ca804a56d35967eeea191ce。PR5已MERGED，GitHub签名verified/valid；原push CI [39394908918](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39394908918)确实执行该head，9个job均自然SUCCESS。

新的空仓sdk-exact-main-cold-1e1c4f03仅使用GitHuborigin：main与预检逐来源解析后必要35个公开annotatedtags；无alternates、shallow、task/PR refs、本地object导入或replace refs。恢复精main/tree/parent后，固定可信c0bc306bd36d2b1b76def2b6c7308a07dda4f54b校验器以--strict-snapshot --base30d7e219cc7d3aef0f75a19217c82898c363f50d --require-change-evidence实际退出0。必要标签也恢复其祖先来源，不要求每个祖先另立同名tag；未新增私有或无必要milestone标签。ignored原始cold日志230B/SHA2566b40ca6ffad467dc89bb6b8a7b35b0e0f52fe57d19d19e51dd8c5e53eef30c65，cold证明4217B/SHA256e9af152c88cc4f16840472226083cf6f475631046bd1a06d7dde6e889636e29e。这结算当前来源/恢复资格，不把旧raw当当前运行，也不替原生安装缓存或Hook/Stop验收。
