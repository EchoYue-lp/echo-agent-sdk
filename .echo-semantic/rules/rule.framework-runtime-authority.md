---
schema_version: 1
id: rule.framework-runtime-authority
kind: rule
status: needs_review
expectation: human_confirmed
risk: high
primary_focus: state_authority
focus:
- time_lifecycle
- failure_concurrency
- contract_evidence
observed_at: 56929154c1851473e9e4837901b610cecf001b2d
behavior_refs:
- behavior.sdk-repository-ownership
code_refs:
- AGENTS.md
- echo-sdk-host/src/lib.rs
- echo-sdk-protocol/src/lib.rs
- docs/adr/0001-sdk-repository-boundary.md
- echo-sdk-protocol/src/bin/export_schema.rs
- echo-sdk-protocol/src/managed.rs
- echo-sdk-protocol/src/task_store.rs
- sdks/python/src/echo_agent_sdk/managed.py
- sdks/typescript/src/managed.ts
- sdks/java/src/main/java/com/echoagent/sdk/ManagedPersistence.java
- sdks/java/src/main/java/com/echoagent/sdk/TaskStore.java
- echo-sdk-host/src/core_profile/facade/workflow.rs
- echo-sdk-host/src/core_profile/events.rs
- echo-sdk-host/src/core_profile/extension_bridge.rs
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/memory.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
- echo-sdk-host/src/core_profile/facade/observability.rs
- echo-sdk-host/src/core_profile/facade/source_operations.rs
- echo-sdk-host/src/core_profile/facade/state_delivery.rs
- echo-sdk-host/src/core_profile/facade/stream.rs
- echo-sdk-host/src/core_profile/facade/subagent.rs
- echo-sdk-host/src/core_profile/facade/task.rs
- echo-sdk-host/src/core_profile/facade/task_runtime.rs
- echo-sdk-host/src/core_profile/facade/telemetry.rs
- echo-sdk-host/src/core_profile/facade/tools.rs
- echo-sdk-host/src/core_profile/handler.rs
- echo-sdk-host/src/core_profile/handles.rs
- echo-sdk-host/src/core_profile/mod.rs
- echo-sdk-host/src/core_profile/state.rs
- echo-sdk-host/src/core_profile/wire.rs
- echo-sdk-host/src/factory.rs
- echo-sdk-protocol/src/catalog.rs
- echo-sdk-protocol/src/facade.rs
- echo-sdk-protocol/src/inventory.rs
- echo-sdk-protocol/src/methods.rs
- echo-sdk-protocol/src/schema.rs
- sdks/java/src/main/java/com/echoagent/sdk/AgentComponentCall.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentComponentDescriptor.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentComponentRequest.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentComponentResult.java
- sdks/java/src/main/java/com/echoagent/sdk/AgentHandle.java
- sdks/java/src/main/java/com/echoagent/sdk/BoundedPublisher.java
- sdks/java/src/main/java/com/echoagent/sdk/CallbackCall.java
- sdks/java/src/main/java/com/echoagent/sdk/CallbackDescriptor.java
- sdks/java/src/main/java/com/echoagent/sdk/CallbackHandler.java
- sdks/java/src/main/java/com/echoagent/sdk/CallbackMessage.java
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- sdks/java/src/main/java/com/echoagent/sdk/EchoAgentException.java
- sdks/java/src/main/java/com/echoagent/sdk/ExtensionRegistration.java
- sdks/java/src/main/java/com/echoagent/sdk/JsonSupport.java
- sdks/java/src/main/java/com/echoagent/sdk/ReplayResponse.java
- sdks/java/src/main/java/com/echoagent/sdk/RunHandle.java
- sdks/java/src/main/java/com/echoagent/sdk/SessionHandle.java
- sdks/java/src/main/java/com/echoagent/sdk/ToolEffect.java
- sdks/java/src/main/java/com/echoagent/sdk/ToolResult.java
- sdks/java/src/main/java/com/echoagent/sdk/WireHandle.java
- sdks/python/src/echo_agent_sdk/__init__.py
- sdks/python/src/echo_agent_sdk/callbacks.py
- sdks/python/src/echo_agent_sdk/client.py
- sdks/python/src/echo_agent_sdk/errors.py
- sdks/python/src/echo_agent_sdk/intrinsics.py
- sdks/python/src/echo_agent_sdk/wire.py
- sdks/typescript/src/client.ts
- sdks/typescript/src/errors.ts
- sdks/typescript/src/helpers.ts
- sdks/typescript/src/index.ts
- sdks/typescript/src/types.ts
- sdks/typescript/src/wire.ts
- contracts/sdk/accepted-facade-operation-catalog.json
- contracts/sdk/facade-operation-catalog.json
- contracts/sdk/fixtures/extension/v1/delivery-complete-claim-valid.json
- contracts/sdk/fixtures/extension/v1/delivery-id-only-claim-invalid.json
- contracts/sdk/fixtures/extension/v1/extension-callback-interrupted-canonical-id-valid.json
- contracts/sdk/fixtures/extension/v1/extension-factory-instance-origin-valid.json
- contracts/sdk/fixtures/extension/v1/factory-create-missing-instance-invalid.json
- contracts/sdk/fixtures/extension/v1/factory-create-reserved-instance-valid.json
- contracts/sdk/fixtures/extension/v1/managed-deadline-context-i64-valid.json
- contracts/sdk/fixtures/extension/v1/managed-deadline-context-number-invalid.json
- contracts/sdk/fixtures/extension/v1/managed-projection-authority-u64-valid.json
- contracts/sdk/fixtures/extension/v1/task-execution-claim-result-null-valid.json
- contracts/sdk/fixtures/extension/v1/task-execution-unsafe-claim-revision-invalid.json
- contracts/sdk/fixtures/extension/v1/task-store-execution-cas-opt-in-valid.json
- contracts/sdk/fixtures/extension/v1/task-store-execution-cas-string-invalid.json
- contracts/sdk/fixtures/extension/v1/tool-result-authoritative-effects-valid.json
- contracts/sdk/fixtures/extension/v1/tool-result-unknown-effect-invalid.json
- contracts/sdk/fixtures/extension/v1/transcript-projection-settlement-observation-valid.json
- contracts/sdk/schema/echo-agent-extension-v1.schema.json
- docs/sdk/governance/evidence/sdk-parent-driver-20261013/current-production-observation.json
- echo-sdk-protocol/tests/extension_contract.rs
- sdks/java/src/test/java/com/echoagent/sdk/WorkflowCheckpointHostTest.java
- sdks/python/tests/protocol_host_proxy.py
- sdks/python/tests/test_ack_submission.py
- sdks/python/tests/test_client_close_callbacks.py
- sdks/python/tests/test_factory_host.py
- sdks/python/tests/test_managed_host.py
- sdks/python/tests/test_managed_values.py
- sdks/python/tests/test_protocol_host.py
- sdks/python/tests/test_workflow_checkpoint_host.py
- sdks/shared/testing/gap_ack_host_proxy.py
- sdks/shared/testing/lsp_diagnostic_server.py
- sdks/shared/testing/lsp_fault_server.py
- sdks/shared/testing/parallel_session_fixture.py
- sdks/shared/testing/test_managed_persistence_backend.py
- sdks/shared/testing/workflow_checkpoint_fixture.py
- sdks/shared/testing/tool_value_effect_fixture.py
- sdks/typescript/test/tool-value-effect-host.test.js
- sdks/python/tests/test_tool_value_effect_host.py
- sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
- sdks/shared/testing/rag_project_rules_fixture.py
- sdks/typescript/test/rag-project-rules-host.test.js
- sdks/python/tests/test_rag_project_rules_host.py
- sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
- sdks/shared/testing/eval_improve_topology_fixture.py
- sdks/typescript/test/eval-improve-topology-host.test.js
- sdks/python/tests/test_eval_improve_topology_host.py
- sdks/java/src/test/java/com/echoagent/sdk/EvalImproveTopologyHostTest.java
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/python/tests/test_tool_value_effect_host.py
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/python/src/echo_agent_sdk/catalog.py
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/python/src/echo_agent_sdk/client.py
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/python/src/echo_agent_sdk/wire.py
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/typescript/test/tool-value-effect-host.test.js
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/typescript/dist/client.js
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/typescript/dist/catalog.js
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/typescript/dist/wire.js
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/java/src/main/java/com/echoagent/sdk/JsonSupport.java
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/java/src/main/java/com/echoagent/sdk/FacadeCatalog.java
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/shared/testing/tool_behavior_fixture.py
- docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/shared/testing/tool_value_effect_fixture.py
- docs/sdk/governance/evidence/host-fixture-isolation/after-source/family-route-host.test.js
- docs/sdk/governance/evidence/host-fixture-isolation/after-source/host.test.js
- docs/sdk/governance/evidence/host-fixture-isolation/before-source/family-route-host.test.js
- docs/sdk/governance/evidence/host-fixture-isolation/before-source/host.test.js
- docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/EvalImproveTopologyHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/after/sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/EvalImproveTopologyHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/RagProjectRulesHostTest.java
- docs/sdk/governance/evidence/sdk-java-default-root-repair/before/sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java
- docs/sdk/governance/evidence/sdk-java-host-early-exit/EarlyExitProbe.java
- docs/sdk/governance/evidence/sdk-java-host-early-exit/before-closing-subscriber-EchoAgentClient.java
- docs/sdk/governance/evidence/sdk-java-host-early-exit/before-late-subscribe-EchoAgentClient.java
- docs/sdk/governance/evidence/sdk-java-host-early-exit/before/EchoAgentClient.java
- docs/sdk/governance/evidence/sdk-python-test-entry/check-language-sdks.after.sh
- docs/sdk/governance/evidence/sdk-python-test-entry/check-language-sdks.before.sh
- scripts/check-language-sdks.sh
- sdks/java/src/test/java/com/echoagent/sdk/ChannelHandlerForwardHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/HostEarlyExitTest.java
- sdks/java/src/test/java/com/echoagent/sdk/LlmWorkflowStreamHostTest.java
- sdks/java/src/test/java/com/echoagent/sdk/ManagedRetirementHostTest.java
- sdks/java/pom.xml
- docs/sdk/governance/evidence/sdk-language-test-nop-complete/executed-source/sdks/java/pom.xml
- deny.toml
- docs/sdk/governance/evidence/factory-child-settlement/before-regression-bridge.rs
- docs/sdk/governance/evidence/factory-child-settlement/after-bridge.rs
- docs/sdk/governance/evidence/factory-child-settlement/before-e2e.rs
- docs/sdk/governance/evidence/factory-child-settlement/before-retry-fixed-bridge.rs
- docs/sdk/governance/evidence/factory-child-settlement/ordering-fixed-bridge.rs
- docs/sdk/governance/evidence/factory-child-settlement/before-bridge.rs
- docs/sdk/governance/evidence/factory-child-settlement/after-e2e.rs
- echo-sdk-host/Cargo.toml
- docs/sdk/governance/evidence/task-context-consumption/before-dev-test-dependencies-Cargo.toml
- docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/Cargo.toml
- docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/src/core_profile/handles.rs
- docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/src/core_profile/handles.rs
- docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/src/core_profile/wire.rs
- docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/src/core_profile/wire.rs
- docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/src/core_profile/facade/source_operations.rs
- docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/src/core_profile/facade/source_operations.rs
- docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/src/core_profile/facade/stream.rs
- docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/src/core_profile/facade/stream.rs
- echo-sdk-host/tests/core_profile_e2e.rs
- docs/sdk/governance/evidence/task-context-consumption/before-feature-wiring/echo-sdk-host/tests/core_profile_e2e.rs
- docs/sdk/governance/evidence/task-context-consumption/feature-wiring-current/echo-sdk-host/tests/core_profile_e2e.rs
- contracts/sdk/fixtures/extension/v1/channel-forward-handle-stream-valid.json
- contracts/sdk/fixtures/extension/v1/channel-forward-handle-valid.json
- contracts/sdk/fixtures/extension/v1/channel-forward-reply-valid.json
- contracts/sdk/fixtures/extension/v1/channel-forward-stream-cancel-valid.json
- contracts/sdk/fixtures/extension/v1/channel-forward-stream-close-valid.json
- contracts/sdk/fixtures/extension/v1/channel-forward-stream-next-valid.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/contracts/sdk/accepted-facade-operation-catalog.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/contracts/sdk/facade-operation-catalog.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/contracts/sdk/schema/echo-agent-extension-v1.schema.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/contracts/sdk/source-contract.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/echo-sdk-host/src/core_profile/extension_bridge.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/echo-sdk-protocol/src/facade.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/echo-sdk-protocol/src/methods.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/combined-callback-ownership-93de/extension_bridge.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/counters/before-slow-source.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/counters/green-bridge-with-slow-case.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/counters/old-common-after-source.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/counters/old-common-before-source.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/late-setup-after-source.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/late-setup-before-source.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/contracts/sdk/accepted-facade-operation-catalog.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/contracts/sdk/facade-operation-catalog.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/contracts/sdk/schema/echo-agent-extension-v1.schema.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/contracts/sdk/source-contract.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/echo-sdk-host/src/core_profile/extension_bridge.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/echo-sdk-protocol/src/facade.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/echo-sdk-protocol/src/methods.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/contracts/sdk/accepted-facade-operation-catalog.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/contracts/sdk/facade-operation-catalog.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/contracts/sdk/schema/echo-agent-extension-v1.schema.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/contracts/sdk/source-contract.json
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/echo-sdk-host/src/core_profile/extension_bridge.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/echo-sdk-protocol/src/facade.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/echo-sdk-protocol/src/methods.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/missing-terminal-current-after.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/missing-terminal-current-before.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-after-source.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-before-source.rs
- docs/sdk/governance/evidence/channel-bridge-callback-settlement/raw/extension_bridge.current.rs
- docs/sdk/governance/evidence/channel-handler-forward-candidate/stream-cancel-before/handles.rs
- docs/sdk/governance/evidence/rag-project-rules-scalar-delta/published-framework-source/echo-tools/src/rag.rs
- docs/sdk/governance/evidence/rag-project-rules/sources/echo-sdk-host/src/core_profile/facade/source_operations.rs
- docs/sdk/governance/evidence/rag-project-rules/sources/sdks/python/src/echo_agent_sdk/client.py
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/README.md
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/candidate-manifest.json
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/extension-contract-tests.log
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/language-catalog-check.log
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/language-catalog-update.log
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/protocol-focused-retry.log
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/protocol-focused.log
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/source-pre-generation-manifest.json
- docs/sdk/governance/evidence/sdk-channel-forward-protocol/wire-candidate-producer.log
- docs/sdk/governance/evidence/sdk-final-framework-contract/PREPARE.md
- docs/sdk/governance/evidence/sdk-final-framework-contract/PROPOSED-ACCEPTED-SOURCE-MIGRATION.md
- docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/.cargo/config.toml
- docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/Cargo.lock
- docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/contracts/sdk/accepted-external-contract.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/contracts/sdk/parity-manifest.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/docs/adr/0034-sdk-governance-authority-and-lossless-adaptation.md
- docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/echo-sdk-host/Cargo.toml
- docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/scripts/check-sdk-contracts.sh
- docs/sdk/governance/evidence/sdk-final-framework-contract/diagnostic-current-rustdoc-observation.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-locked.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-locked.log
- docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-minimal-locked.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-minimal-locked.log
- docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-unlocked.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-unlocked.log
- docs/sdk/governance/evidence/sdk-final-framework-contract/full-extraction-first-continuity-failure.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/full-update-complete-missing-report.log
- docs/sdk/governance/evidence/sdk-final-framework-contract/full-update-initial-command.txt
- docs/sdk/governance/evidence/sdk-final-framework-contract/full-update-initial.log
- docs/sdk/governance/evidence/sdk-final-framework-contract/lock-exact-delta.patch
- docs/sdk/governance/evidence/sdk-final-framework-contract/lock-framework-update.log
- docs/sdk/governance/evidence/sdk-final-framework-contract/lock-package-delta.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/lock-quinn-proto-update.log
- docs/sdk/governance/evidence/sdk-final-framework-contract/preparation-source-manifest.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/published-pin-provenance.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/security-current-rustdoc-observation.json
- docs/sdk/governance/evidence/sdk-final-framework-contract/snapshot-current-rustdoc-observation.json
- docs/sdk/governance/evidence/sdk-language-factory-final/executed-source/echo-sdk-host/src/core_profile/extension_bridge.rs
- docs/sdk/governance/evidence/sdk-language-factory-final/executed-source/echo-sdk-host/src/core_profile/handles.rs
- docs/sdk/governance/evidence/sdk-stream-cancellation-routing/before/sdks/python/src/echo_agent_sdk/client.py
- docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/diagnostic-client/dist/a2a.d.ts
- docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/diagnostic-client/dist/managed.d.ts
- docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/diagnostic-client/dist/steering_values.d.ts
- docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/source/echo-sdk-host/src/core_profile/extension_bridge.rs
- docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/source/sdks/python/src/echo_agent_sdk/client.py
- docs/sdk/governance/evidence/tool-value-effects/fused-06bb/executed-source/sdks/python/src/echo_agent_sdk/client.py
- docs/sdk/governance/evidence/tool-value-effects/prepared-source-epoch/sdks/python/src/echo_agent_sdk/client.py
- docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/executed-source/sdks/python/src/echo_agent_sdk/client.py
- sdks/java/src/test/java/com/echoagent/sdk/StreamCancellationRoutesTest.java
- sdks/python/tests/test_channel_handler_forward_host.py
- sdks/python/tests/test_llm_workflow_stream_host.py
- sdks/python/tests/test_managed_retirement_host.py
- sdks/python/tests/test_stream_cancellation_route_host.py
- sdks/python/tests/test_stream_cancellation_routes.py
- sdks/shared/testing/compare_channel_forward_results.py
- sdks/shared/testing/llm_workflow_stream_fixture.py
- sdks/shared/testing/test_managed_retirement_backend.py
evidence_refs:
- evidence.sdk-source-import
- evidence.sdk-source-continuity
- evidence.sdk-framework-pin
- evidence.sdk-wave2-cutover-repair
- evidence.sdk-wave2-cutover-final-verification
- evidence.sdk-compatibility-gates
- evidence.sdk-deferred-product-disposition
- evidence.sdk-managed-supporting-operation-boundary
- evidence.sdk-channel-forward-human-scope
- evidence.sdk-extension-teardown-prefix-verification
- evidence.sdk-rag-scalar-verification
- evidence.sdk-rag-project-rules
- evidence.sdk-accepted-source-continuity-gate
- evidence.sdk-official-factory-final-language-gate
- evidence.sdk-stream-cancellation-routing-verification
- evidence.sdk-language-current-first-gate-failure
- evidence.sdk-ordinary-tools-b909-verification
- evidence.sdk-llm-workflow-stream
- evidence.sdk-managed-retirement
finding_refs:
- finding.sdk-wave2-cutover-integrity
- finding.sdk-accepted-source-continuity
- finding.sdk-dependency-warning-policy-conflict
---

# Framework runtime authority

## 不变量或唯一权威

`echo_agent` 唯一拥有 Agent、Run、Task、Subagent、event、retry、cancel、recovery、persistence 和 terminal 语义。

## 适用行为

适用于 protocol DTO、Host adapter、ACP transport 和三语言 client。

## 当前实现

Host 消费精确 pin 的 framework 服务并做协议转换；protocol 不依赖任何 echo framework crate，
clients 不建立第二套 runtime authority。

## 期望行为

Protocol 只依赖 wire/schema 库，Host 精确依赖 framework revision，所有 adapter 转换保持字段级无损；
协商只使用 protocol version、accepted contract digests、capability 与 compiled feature。

## 证据

AGENTS、SDK boundary ADR、Host 与 protocol 入口共同限定依赖方向。

## 裁决记录

用户确认 protocol 纯化和 accepted external contract 边界；source continuity 不改变这些长期决策。

## 当前治理快照限制

上述已验证行为绑定历史 `30d7e219`。本轮显式 framework 升级、扩展桥/Task/事件/资源修复及三语言变化仍在实现和反例验证中；当前源码摘要、合同生成、独立复审和 main 验收完成前，不恢复 verified/closure 声明。

## 当前后继适配边界

Task execution与managed持久化DTO只传权威claim/revision/commit/result；Host adapter不拥有第二调度或状态机。accepted scope仅由唯一生成器显式裁决，export rustdoc目录遵CARGO_TARGET_DIR而不改接受policy。Workflow传入graph/state的实际消费及Channel inbound consumer尚有独立义务，测试总数不关闭这些路径。

## 新协议及消费者路径依据

新增fixture/schema/catalog与consumer测试路径按其实际文件声明盘点：managed/TaskCAS/result、callback/canonicalidentity、factoryreservation、ToolEffect及事件ACK格式只承接既有接受行为，不添加第二authority。测试/proxy是具名反例消费者，不作为生产权威。代码引用仅定位源与治理约束，运行支持仍由各自Evidence原输入/日志限定；不能据机器路径检查宣布业务通过。

## ACK反例源码归档边界

- `docs/sdk/governance/evidence/sdk-ack-recovery-windows/before/sdks/python/src/echo_agent_sdk/client.py`保存原3abae766客户端输入，用于retention coverage与pending ACK反例；这是明确历史before源码，不注册到生产客户端或形成第二cursor权威。
- `docs/sdk/governance/evidence/sdk-ack-recovery-windows/review-before-v3/sdks/python/src/echo_agent_sdk/client.py`保存35ac584b复审前阶段输入；与原before、修后current分别保留。它只解释复审反例所执行的源，不表示最终修复或运行通过。

上述归档路径包含历史公开定义，机器会识别publicApi/状态代码，但它们的生命周期是不可变取证文件，不是新的accepted接口/dispatcher/runtime。ACK实际消费与失败rollback仍只由正式语言客户端官方请求提交事实管理；具体红绿/复审由ACK专项Evidence限定，不能仅因这些源路径声明而关闭#120。

## 新增计算消费者与Channel反例取证依据

- `sdks/shared/testing/computation_fixture.py`声明固定操作/输入，创建CSV/导出sentinel并对原始结果做数值、UTF-8跨度、typederror与原字节oracle；它不发送ACP请求，不执行Guard/Pivot，也不持有生产数据权威。新增null label/null index/unknown aggregation反例约束原框架转换结果，不接受新API。
- `sdks/python/tests/test_computation_host.py`通过公开EchoAgentClient和Session.invoke消费真实Host；structured_output validate使用既有request合同，错误code/message/retryable/handle/details和保护文件前后hash均保留。这个测试声明不代表三语言或最终源码已通过；实际证据仍须对应运行receipt。
- `docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/resource-admission-before/handles.rs`是e155ffd6资源准入复审前原字节归档，包含旧closing验证和Registry测试定义，仅用于复现，不由构建生产模块引用。resource-admission-before的source-manifest把它与mod/integrations源分别记录；后继修复必须由自己的红绿/复审限定，不能把before归档当当前authority。

新增测试、oracle与历史源码不会形成第二调度/游标/资源状态机；生产事实继续由框架、Host唯一Registry/原连接owner及官方客户端ACK提交事实分别拥有。

## Deferred产品分组及支持值边界

旧inventory完整分组只遥测，不扩accepted。managed三逆操作的Ruststabledeletecaller与SDK兼容delete/generationclear边界已实际核对，保DTO与独立framework权威；不存在forward接受就不造第二retirementexecutor或仅为counts新增API。两个源摘录是不可变审查材料，不进入生产构建。

## 新有限native消费者测试依据

ToolValueEffects四文件只创建既有files/shell/web/database/research输入与native反向SearchProvider/Sandbox，47calls/48rows不宣whole绿，3实际缺陷及Clinicaltotal另Finding。RAG/Rules四文件的loopback是真HttpEmbedder，不替Host或第二index；29calls/13POST原scope保0435，新Dcharfix另证。Eval/Improve/Topology四文件注册真实语言AgentFactory/CustomAgent，timeout1秒暴露已有context取消丢失，fixture只oracle/配置不调度/结算。全部源码code_refs明确主测试路径；完整最终sourcecontract/新bin及strongscenarios未绿时不得提升Behavior。

## 已接受source/kind连续性

旧acceptedcanonical承诺只能由exactsource/kind保持或经明确批准迁移/退役；完整库存数量、same-namegetter、publicalias、无consumer、已删除旧owner字段都不能自动改变范围。28profilecorrectfail12missing/noSeedwrite由SpecificFinding承接，proposed兼容选择不humanconfirmed；diagnostic只报全缺失不变唯一policy。

## 最新Tool消费者运行源码归档

- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/python/tests/test_tool_value_effect_host.py`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/python/src/echo_agent_sdk/catalog.py`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/python/src/echo_agent_sdk/client.py`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/python/src/echo_agent_sdk/wire.py`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/typescript/test/tool-value-effect-host.test.js`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/typescript/dist/client.js`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/typescript/dist/catalog.js`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/typescript/dist/wire.js`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/java/src/test/java/com/echoagent/sdk/ToolValueEffectHostTest.java`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/java/src/main/java/com/echoagent/sdk/JsonSupport.java`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/java/src/main/java/com/echoagent/sdk/EchoAgentClient.java`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/java/src/main/java/com/echoagent/sdk/FacadeCatalog.java`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/shared/testing/tool_behavior_fixture.py`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。
- `docs/sdk/governance/evidence/tool-value-effects/b909-natural-eof-after/executed-source/sdks/shared/testing/tool_value_effect_fixture.py`：8d819/b909 finiteafter实际运行输入归档，只支撑原Tools/Search/Sandbox强oracle与publicnative；不是生产第二parser/状态机或新acceptedAPI。

原sourcecontract3cd835部分候选，完整accepted12缺口仍等人选与generator；路径依据不称全局semanticstrict通过。

## 原生验证入口与退出窗口source依据

JavaHostEarlyExit仅原officialchild/late+closing订阅、原factorycleanup与exitcause，未宣livechildpurestdoutEOF；59组合不代新whole。同6Javafixture只八个root转absolute.normalize保持原所有断言/默认Maven运行，node2smoke各Hoststate_root隔离不绕原文件锁。Python正式脚本用同uv依赖/flags/tests改python-m-pytest正确interpreter入口、仍全tests+shared+quickstart，357+18原分段receipt不变成本轮三語whole绿。归档源码是各实际before/after读证据，不生产第二客户端/新API；所有paths原bytes保，task8MiB已诊断不重复或删refs。

## 当前完整语言entry输入依据

POMtestprovider只验证classpath，compile/runtimelibrary不强日志provider；完整语言singleentry原sourcefreeze632全同，原header/README/sourcecut不第二contractauthority。后继scope/producerblockedcontract仍须相应门禁；不能以语言counts恢复全identityparity完成分母。

## 依赖工具事实与人的合并规则

当前audit0warning/denyexit0不自动变成零warning门禁通过；原multipleversionswarn产生50duplicate与AGENTS零warning冲突由具体authorityFinding承接。Statusproposed的绑定50group规则选择没有人批时不riskaccepted/不override，不增加permissiongate或修改原4advisoryexception。

## Factorytwofact与同cleanupowner修复归档

Factory27包.rs是原43ec/before/orderingfix/onlysuccessretry后8e7实际反例源，不入Cargo第二生产模块。只有actualRPC回答+validatedremoteTerminal原gate同时到达才同步释放原lease，held/wronggen仍占用；cleanup只缓存成功，失败原owner保可重试。27原byte与3新增strictlog/root终态非reader重跑，当前fullmatrix/新Hostlanguage尚待，不能用source引用机械raise全族。

## feature测试接线修复依据

六源devdeps/cfg/import最小修补只恢复按实际feature编译测试可用性，before/current归档不入生产双实现；lock/bridge8e7/TaskContexte46保持原byte。noBridge最后1admission已0warning、旧warning日志是历史非当前未修；final14reviewedmatrix运行与后继language另验，不由focusedcounts提全SDK状态。

## 102条新增精确路径依据的有限归属

本节只补当前change-evidence机器要求的exactpath依据。每条文件均已实际读取并核SHA/可用manifest；原before/after/executed源码、实际失败日志、proposed文稿和测试oracle分别有角色，不进入生产构建或第二运行时。

- `contracts/sdk/fixtures/extension/v1/channel-forward-handle-stream-valid.json`：existing approved protocol fixture；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准既有channels六操作fixture；完整Session/manager或streamhandle与generation/Unicode输入，由唯一producer生成，不受12canonicalsource未决身份扩范围；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/candidate-manifest.json`。
- `contracts/sdk/fixtures/extension/v1/channel-forward-handle-valid.json`：existing approved protocol fixture；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准既有channels六操作fixture；完整Session/manager或streamhandle与generation/Unicode输入，由唯一producer生成，不受12canonicalsource未决身份扩范围；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/candidate-manifest.json`。
- `contracts/sdk/fixtures/extension/v1/channel-forward-reply-valid.json`：existing approved protocol fixture；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准既有channels六操作fixture；完整Session/manager或streamhandle与generation/Unicode输入，由唯一producer生成，不受12canonicalsource未决身份扩范围；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/candidate-manifest.json`。
- `contracts/sdk/fixtures/extension/v1/channel-forward-stream-cancel-valid.json`：existing approved protocol fixture；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准既有channels六操作fixture；完整Session/manager或streamhandle与generation/Unicode输入，由唯一producer生成，不受12canonicalsource未决身份扩范围；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/candidate-manifest.json`。
- `contracts/sdk/fixtures/extension/v1/channel-forward-stream-close-valid.json`：existing approved protocol fixture；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准既有channels六操作fixture；完整Session/manager或streamhandle与generation/Unicode输入，由唯一producer生成，不受12canonicalsource未决身份扩范围；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/candidate-manifest.json`。
- `contracts/sdk/fixtures/extension/v1/channel-forward-stream-next-valid.json`：existing approved protocol fixture；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准既有channels六操作fixture；完整Session/manager或streamhandle与generation/Unicode输入，由唯一producer生成，不受12canonicalsource未决身份扩范围；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/candidate-manifest.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/contracts/sdk/accepted-facade-operation-catalog.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/contracts/sdk/facade-operation-catalog.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/contracts/sdk/schema/echo-agent-extension-v1.schema.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/contracts/sdk/source-contract.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/echo-sdk-host/src/core_profile/extension_bridge.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/echo-sdk-protocol/src/facade.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/source/echo-sdk-protocol/src/methods.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/callback-prefix-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/combined-callback-ownership-93de/extension_bridge.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/combined-callback-ownership-93de/receipt.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/counters/before-slow-source.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/counters/green-bridge-with-slow-case.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/counters/old-common-after-source.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/counters/old-common-before-source.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/late-setup-after-source.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/late-setup-before-source.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/contracts/sdk/accepted-facade-operation-catalog.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/contracts/sdk/facade-operation-catalog.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/contracts/sdk/schema/echo-agent-extension-v1.schema.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/contracts/sdk/source-contract.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/echo-sdk-host/src/core_profile/extension_bridge.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/echo-sdk-protocol/src/facade.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/source/echo-sdk-protocol/src/methods.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/final-stream-settlement-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/contracts/sdk/accepted-facade-operation-catalog.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/contracts/sdk/facade-operation-catalog.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/contracts/sdk/schema/echo-agent-extension-v1.schema.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/contracts/sdk/source-contract.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/echo-sdk-host/src/core_profile/extension_bridge.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/echo-sdk-protocol/src/facade.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/source/echo-sdk-protocol/src/methods.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/fused-sandbox-final-epoch/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/missing-terminal-current-after.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/missing-terminal-current-before.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-after-source.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/known-final-counters/prefix-before-source.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-bridge-callback-settlement/raw/extension_bridge.current.rs`：archived before/after/executed source；归`evidence.sdk-extension-teardown-prefix-verification`；原Custom/Channel callback结算、setup ACK、actualresponse/terminal、fusedSandbox/prefix各before/after阶段；按子目录freeze区分，非当前whole源；原摘要定位`docs/sdk/governance/evidence/channel-bridge-callback-settlement/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/channel-handler-forward-candidate/stream-cancel-before/handles.rs`：archived before/after/executed source；归`evidence.sdk-channel-forward-human-scope`；Channelstream取消前原Registry输入，仅旧counter候选；人的ADR0035六操作接受不由before源码扩；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/rag-project-rules-scalar-delta/published-framework-source/echo-tools/src/rag.rs`：archived before/after/executed source；归`evidence.sdk-rag-scalar-verification`；publishedb909原RAG scalar修复框架源；32native/13POST的原输入，不冒其它epoch；原摘要定位`docs/sdk/governance/evidence/rag-project-rules-scalar-delta/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/rag-project-rules/sources/echo-sdk-host/src/core_profile/facade/source_operations.rs`：archived before/after/executed source；归`evidence.sdk-rag-project-rules`；0435旧RAG/Rulesconsumerfreeze；旧char_count错观察不能作新scalar正确；原摘要定位`docs/sdk/governance/evidence/rag-project-rules/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/rag-project-rules/sources/sdks/python/src/echo_agent_sdk/client.py`：archived before/after/executed source；归`evidence.sdk-rag-project-rules`；0435旧RAG/Rulesconsumerfreeze；旧char_count错观察不能作新scalar正确；原摘要定位`docs/sdk/governance/evidence/rag-project-rules/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/README.md`：archived configuration/lock or proposed scope document；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/candidate-manifest.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/extension-contract-tests.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/language-catalog-check.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/language-catalog-update.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/protocol-focused-retry.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/protocol-focused.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/source-pre-generation-manifest.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-channel-forward-protocol/wire-candidate-producer.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-channel-forward-human-scope`；ADR0035已批准Channel六closedoperation stagedproducer111/extension39/languagecatalog原验证；早locked失败与后重试分开，非full28源生成；原摘要定位`docs/sdk/governance/evidence/sdk-channel-forward-protocol/evidence-sha256.json`。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/PREPARE.md`：archived configuration/lock or proposed scope document；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/PROPOSED-ACCEPTED-SOURCE-MIGRATION.md`：archived configuration/lock or proposed scope document；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/.cargo/config.toml`：archived configuration/lock or proposed scope document；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；原摘要定位`docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json`。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/Cargo.lock`：archived configuration/lock or proposed scope document；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；原摘要定位`docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json`。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/contracts/sdk/accepted-external-contract.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；原摘要定位`docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json`。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/contracts/sdk/parity-manifest.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；原摘要定位`docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json`。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/docs/adr/0034-sdk-governance-authority-and-lossless-adaptation.md`：archived configuration/lock or proposed scope document；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；原摘要定位`docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json`。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/echo-sdk-host/Cargo.toml`：archived configuration/lock or proposed scope document；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；原摘要定位`docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json`。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin/scripts/check-sdk-contracts.sh`：archived before/after/executed source；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；原摘要定位`docs/sdk/governance/evidence/sdk-final-framework-contract/before-pin-sha256.json`。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/diagnostic-current-rustdoc-observation.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-locked.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-locked.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-minimal-locked.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-minimal-locked.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-unlocked.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/framework-git-metadata-unlocked.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/full-extraction-first-continuity-failure.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/full-update-complete-missing-report.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/full-update-initial-command.txt`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/full-update-initial.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/lock-exact-delta.patch`：archived configuration/lock or proposed scope document；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/lock-framework-update.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/lock-package-delta.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/lock-quinn-proto-update.log`：immutable actual diagnostic log/command/provenance；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/preparation-source-manifest.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/published-pin-provenance.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/security-current-rustdoc-observation.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-final-framework-contract/snapshot-current-rustdoc-observation.json`：archived protocol/contract/provenance JSON；归`evidence.sdk-accepted-source-continuity-gate`；旧pin/lock/seedbefore档、freshb909 metadata/rustdoc原输入及full28correctfail12/noSeedwrite；proposed迁移未批准，不当新接受规则；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/sdk-language-factory-final/executed-source/echo-sdk-host/src/core_profile/extension_bridge.rs`：archived before/after/executed source；归`evidence.sdk-official-factory-final-language-gate`；bb4f/bridge8e7 Factorycut实运行sourcefreeze，632allSame和singleentry完整0；非旧8d819/sourcegen批准；原摘要定位`docs/sdk/governance/evidence/sdk-language-factory-final/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/sdk-language-factory-final/executed-source/echo-sdk-host/src/core_profile/handles.rs`：archived before/after/executed source；归`evidence.sdk-official-factory-final-language-gate`；bb4f/bridge8e7 Factorycut实运行sourcefreeze，632allSame和singleentry完整0；非旧8d819/sourcegen批准；原摘要定位`docs/sdk/governance/evidence/sdk-language-factory-final/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/sdk-stream-cancellation-routing/before/sdks/python/src/echo_agent_sdk/client.py`：archived before/after/executed source；归`evidence.sdk-stream-cancellation-routing-verification`；早setupACK取消地址旧clientbefore，3native真实redgreencounter输入；原取消route不定义第二Task状态权威；原摘要定位`docs/sdk/governance/evidence/sdk-stream-cancellation-routing/evidence-sha256.json`。
- `docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/diagnostic-client/dist/a2a.d.ts`：archived before/after/executed source；归`evidence.sdk-language-current-first-gate-failure`；8d819原Taskresume取消地址诊断编译client/桥输入，DELETE_ROUTE/CANCEL routefalse的只读timeline；未修复before不当after通过；原摘要定位`docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/diagnostic-client/dist/managed.d.ts`：archived before/after/executed source；归`evidence.sdk-language-current-first-gate-failure`；8d819原Taskresume取消地址诊断编译client/桥输入，DELETE_ROUTE/CANCEL routefalse的只读timeline；未修复before不当after通过；原摘要定位`docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/diagnostic-client/dist/steering_values.d.ts`：archived before/after/executed source；归`evidence.sdk-language-current-first-gate-failure`；8d819原Taskresume取消地址诊断编译client/桥输入，DELETE_ROUTE/CANCEL routefalse的只读timeline；未修复before不当after通过；原摘要定位`docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/source/echo-sdk-host/src/core_profile/extension_bridge.rs`：archived before/after/executed source；归`evidence.sdk-language-current-first-gate-failure`；8d819原Taskresume取消地址诊断编译client/桥输入，DELETE_ROUTE/CANCEL routefalse的只读timeline；未修复before不当after通过；原摘要定位`docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/source/sdks/python/src/echo_agent_sdk/client.py`：archived before/after/executed source；归`evidence.sdk-language-current-first-gate-failure`；8d819原Taskresume取消地址诊断编译client/桥输入，DELETE_ROUTE/CANCEL routefalse的只读timeline；未修复before不当after通过；原摘要定位`docs/sdk/governance/evidence/task-resume-early-ack-cancellation-root-cause/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/tool-value-effects/fused-06bb/executed-source/sdks/python/src/echo_agent_sdk/client.py`：archived before/after/executed source；归`evidence.sdk-ordinary-tools-b909-verification`；fused06bb/preparedsource/publishedee9ff各nativeclientfreeze只支撑该原Tool/Search/Sandboxcounter输入；prepared未执行、旧Sandboxrace另后继；原摘要定位`docs/sdk/governance/evidence/tool-value-effects/fused-06bb/SHA256SUMS.json`。
- `docs/sdk/governance/evidence/tool-value-effects/prepared-source-epoch/sdks/python/src/echo_agent_sdk/client.py`：archived before/after/executed source；归`evidence.sdk-ordinary-tools-b909-verification`；fused06bb/preparedsource/publishedee9ff各nativeclientfreeze只支撑该原Tool/Search/Sandboxcounter输入；prepared未执行、旧Sandboxrace另后继；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/executed-source/sdks/python/src/echo_agent_sdk/client.py`：archived before/after/executed source；归`evidence.sdk-ordinary-tools-b909-verification`；fused06bb/preparedsource/publishedee9ff各nativeclientfreeze只支撑该原Tool/Search/Sandboxcounter输入；prepared未执行、旧Sandboxrace另后继；原摘要定位`docs/sdk/governance/evidence/tool-value-effects/published-framework-b909/SHA256SUMS.json`。
- `sdks/java/src/test/java/com/echoagent/sdk/StreamCancellationRoutesTest.java`：shared oracle/backend obligation test；归`evidence.sdk-stream-cancellation-routing-verification`；原官方ACProute windows negative：terminalbeforeACK/earlycancel/wronggeneration/failedsend/failedclose，原TS19PY39J27范围；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `sdks/python/tests/test_channel_handler_forward_host.py`：native consumer test；归`evidence.sdk-channel-forward-human-scope`；公开ChannelManager/handler/streamcontrol真实Hostconsumer，originalowner/generation/消息shape/cancel/reentry；具体nativeepoch与generator六操作分开；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `sdks/python/tests/test_llm_workflow_stream_host.py`：native consumer test；归`evidence.sdk-llm-workflow-stream`；真实语言LLM/Workflowreversecallbacks及TaskAgent原finally/streamwriter；旧0744raw与typedprojectiondelta分别保；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `sdks/python/tests/test_managed_retirement_host.py`：native consumer test；归`evidence.sdk-managed-retirement`；公开Session.invoke兼容delete/generationclear消费，managedretirementenabled/Unsupported无mutation两场景；不强造scope-retirement三个forwardAPI；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `sdks/python/tests/test_stream_cancellation_route_host.py`：native consumer test；归`evidence.sdk-stream-cancellation-routing-verification`；真实HostdeclaredstreamsetupACK后publicSubagentcancel，heldcleanup原signal/finally；不称5秒强abort物理结算；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `sdks/python/tests/test_stream_cancellation_routes.py`：native consumer test；归`evidence.sdk-stream-cancellation-routing-verification`；officialtransportstreamroute exactaddress/generation/terminal提交负例本地窗口，非mockHost业务事实；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `sdks/shared/testing/compare_channel_forward_results.py`：shared oracle/backend obligation test；归`evidence.sdk-channel-forward-human-scope`；native三语输出比较，原id/root归一化与UTF8/base64/field/sequence事实；只oracle不执行Host/原handler；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `sdks/shared/testing/llm_workflow_stream_fixture.py`：shared oracle/backend obligation test；归`evidence.sdk-llm-workflow-stream`；固定LLMusage/Workflowevent/graph输入与oracle，callback由各语言consumer执行；不造第二graphexecutor；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。
- `sdks/shared/testing/test_managed_retirement_backend.py`：shared oracle/backend obligation test；归`evidence.sdk-managed-retirement`；backend原atomicDocument/CAS/receipt/deadline7retirementnegative测试，与13原共20；不代表三个无acceptedforward消费者已调用；本次未找到包含该单文件摘要的manifest时，保实际读取SHA与原Evidence文件性质/场景定位，不假历史执行或新通过。

以上关联只解释文件性质/所属原证据scope，不复制SDKHostparser/Store/graph状态；日志成功或源码copy不自动verified。既有Channel6fixtures来自人批准ADR0035及原producer，完整accepted12source迁移仍未人批。原版本标记、全Baseline、41family状态和Finding闭合都不在本切片改动范围。

## 当前模型重观察范围

review_scope：本对象在真实当前候选Git `56929154c1851473e9e4837901b610cecf001b2d`（源码摘要`17cf243e8d6341db0ce23c5b9ee39977ce4ab100d6ed1b17ec5a7bb26c19e138`）上重新观察。上列code_refs/各scenario source_refs已逐Gitblob读取并与当前文件SHA精确相同，原锚点均存在。模型继续描述原accepted入口、框架业务/终态权威、Host owner/generation与transport消费职责、生命周期及具名失败义务；源码定位核对不证明所有行为正确、运行证据fresh或合同生成闭合。

原Behavior/status、Map场景mapped/needs_review、Rule/Asset处置、Finding/未决义务与Evidence原epoch支持范围全部保留。mapped仅入口模型，旧3ea不是本snapshot本体；历史receipt不得因本模型观察更新冒current运行。12source迁移、5秒物理结算与依赖50warning规则三选择仍未接受，完整生成/CI/main另验，任何新Rustwrapper不自动扩三语言合同。
