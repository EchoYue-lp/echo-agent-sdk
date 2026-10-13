---
{
  "schema_version": 1,
  "id": "evidence.sdk-parent-driver-candidate-verification",
  "kind": "evidence",
  "observed_at": "source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404",
  "source_refs": [
    "docs/sdk/governance/evidence/sdk-parent-driver-20261013/manifest.json",
    "docs/sdk/governance/evidence/sdk-parent-driver-20261013/sdk-parent-driver-local-proof.json",
    "docs/sdk/governance/evidence/sdk-parent-driver-20261013/sdk-full-parent-driver-language-proof.json",
    "docs/sdk/governance/evidence/sdk-parent-driver-20261013/sdk-full-parent-driver-language-gates.log",
    "docs/sdk/governance/evidence/sdk-parent-driver-20261013/current-source-comparison.json",
    "echo-sdk-host/tests/core_profile_e2e.rs#valid_hello_completes_full_core_lifecycle",
    "echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources",
    "echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle",
    "echo-sdk-host/tests/core_profile_e2e.rs#extended_standard_prompt_bridges_shared_core_handles",
    "echo-sdk-host/tests/core_profile_e2e.rs#explicit_session_create_reserves_persisted_turn_identity_before_new_run",
    "echo-sdk-host/tests/core_profile_e2e.rs#session_load_reserves_persisted_turn_identity_before_new_run",
    "echo-sdk-host/tests/core_profile_e2e.rs#malformed_recovered_identity_rejects_actual_method_without_handle_publication",
    "echo-sdk-host/tests/core_profile_e2e.rs#killed_active_run_is_interrupted_never_completed",
    "echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#sandbox_cancel_bridge_waits_for_cleanup_and_preserves_cancelled_terminal",
    "echo-sdk-host/tests/core_profile_e2e.rs#ack_window_snapshot_ack_resumes_strictly_after_watermark",
    "echo-sdk-host/tests/core_profile_e2e.rs#consumed_single_replay_page_ack_resumes_live_after_page_cursor",
    "echo-sdk-host/tests/core_profile_e2e.rs#consumed_multiple_replay_page_ack_resumes_live_after_page_cursor",
    "echo-sdk-host/tests/core_profile_e2e.rs#task_rpc_shares_the_session_task_authority",
    "echo-sdk-host/tests/core_profile_e2e.rs#task_pause_is_exact_and_resume_reuses_canonical_dependency_result",
    "echo-sdk-host/tests/core_profile_e2e.rs#task_cancel_is_exact_and_pending_sibling_continues",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#crud_only_task_store_rejects_execute_before_runtime_side_effects",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#revisioned_task_extension_is_shared_by_crud_and_execution",
    "echo-sdk-host/tests/core_profile_e2e.rs#subagent_short_await_preserves_result_and_dispatch_receipt",
    "echo-sdk-host/tests/core_profile_e2e.rs#subagent_family_dispatches_and_controls_live_attempts",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#same_factory_constructs_independent_instances_across_concurrent_sessions",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#tool_bridge_round_trip_with_callbacks_intervention_and_hook",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#managed_persistence_deadline_and_lost_ack_preserve_commit_authority",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#managed_commit_disconnect_restart_replays_same_receipt",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#cancelled_initial_stream_response_releases_session_callback_owner",
    "echo-sdk-host/src/core_profile/wire.rs#managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds",
    "echo-sdk-host/src/core_profile/wire.rs#unix_non_utf8_paths_round_trip_without_loss",
    "echo-sdk-host/tests/core_profile_e2e.rs#negotiated_facade_admission_fails_closed",
    "echo-sdk-host/src/core_profile/facade/memory.rs#malformed_namespace_never_writes_and_replacement_store_is_live_authority",
    "echo-sdk-host/src/core_profile/facade/memory.rs#unsupported_store_operation_keeps_memory_cause_without_mutation",
    "echo-sdk-host/tests/core_profile_e2e.rs#workflow_family_runs_declarative_graphs_over_the_framework_engine",
    "echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_cancel_preserves_canonical_failure_in_rpc_and_stream_terminal",
    "echo-sdk-host/tests/facade_feature_adapters_e2e.rs#workflow_stream_uses_unified_handle_backpressure_and_close",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#workflow_agent_component_stream_flows_through_facade_pull_stream",
    "echo-sdk-host/tests/core_profile_e2e.rs#state_delivery_and_trace_families_use_framework_services",
    "echo-sdk-host/src/core_profile/facade/state_delivery.rs#exact_claim_roundtrip_rejects_delayed_attempt_and_id_only",
    "echo-sdk-host/src/core_profile/facade/observability.rs#failed_trace_open_does_not_consume_the_only_resource_slot",
    "echo-sdk-host/tests/core_profile_e2e.rs#eval_and_improve_families_use_the_framework_analyzers",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#eval_and_improve_source_adapters_invoke_agent_factory_lazily",
    "echo-sdk-host/src/core_profile/facade/source_operations.rs#factory_failure_retains_canonical_cause_in_real_eval_result",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#store_and_human_loop_extensions_round_trip",
    "echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_quota_failure_does_not_publish_unreturned_prefix",
    "echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_file_batch_quota_failure_does_not_publish_unreturned_prefix",
    "echo-sdk-host/tests/facade_feature_adapters_e2e.rs#mcp_config_batch_preserves_best_effort_and_existing_alias",
    "echo-sdk-host/tests/core_profile_e2e.rs#integration_families_use_framework_managers_and_clients",
    "echo-sdk-host/tests/facade_feature_adapters_e2e.rs#a2a_sse_stream_delivers_items_then_terminal",
    "echo-sdk-host/tests/extension_bridge_e2e.rs#channel_plugin_facade_round_trip",
    "echo-sdk-host/src/core_profile/facade/telemetry.rs#invalid_console_flag_is_rejected_before_process_initialization",
    "echo-sdk-host/tests/core_profile_e2e.rs#tool_families_execute_framework_tools_with_the_session_cwd",
    "echo-sdk-host/tests/core_profile_e2e.rs#source_files_and_skill_registry_preserve_rust_authority",
    "echo-sdk-host/src/core_profile/facade/tools.rs#tool_family_projection_preserves_partial_effects_and_artifact",
    "echo-sdk-host/tests/core_profile_e2e.rs#remaining_tool_families_execute_framework_tools_locally",
    "echo-sdk-host/src/core_profile/facade/tools.rs#rag_index_and_search_share_one_session_store",
    "echo-sdk-host/tests/core_profile_e2e.rs#stateful_source_resources_preserve_owner_state_and_close",
    "echo-sdk-host/tests/facade_feature_adapters_e2e.rs#facade_resource_limits_match_the_advertised_bound",
    "echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address",
    "echo-sdk-host/src/core_profile/events.rs#real_journal_retention_page_authorizes_only_its_returned_watermark",
    "echo-sdk-host/src/core_profile/events.rs#replay_publication_failure_and_empty_future_page_grant_no_ack_coverage"
  ],
  "supports": [
    "behavior.sdk-family-agent-lifecycle",
    "behavior.sdk-family-session",
    "behavior.sdk-family-run",
    "behavior.sdk-family-event-replay",
    "behavior.sdk-family-task",
    "behavior.sdk-family-subagent",
    "behavior.sdk-family-bridge",
    "behavior.sdk-family-standard",
    "behavior.sdk-family-value",
    "behavior.sdk-family-intrinsic",
    "behavior.sdk-family-memory",
    "behavior.sdk-family-delivery",
    "behavior.sdk-family-state",
    "behavior.sdk-family-trace",
    "behavior.sdk-family-eval",
    "behavior.sdk-family-improve",
    "behavior.sdk-family-permission",
    "behavior.sdk-family-mcp",
    "behavior.sdk-family-a2a",
    "behavior.sdk-family-lsp",
    "behavior.sdk-family-channels",
    "behavior.sdk-family-telemetry",
    "behavior.sdk-family-topology",
    "behavior.sdk-family-structured-output",
    "behavior.sdk-family-source-operation",
    "behavior.sdk-family-invoke"
  ],
  "limitations": [
    "Original6711 Rust418/TS240/Python300+18/Java264 executable scope; repository source changed afterward.",
    "Current Workflow and Tool validator differ from frozen55runtimefiles; corresponding current supports excluded until successor receipts are bound.",
    "Supports are named partial scenario assertions only; every Behavior remains needs_review.",
    "Temporary framework path patches, not published framework pin/main or final accepted inventory.",
    "Testing value-only and tool clusters have separate bounded Evidence; neither counts as new Mock runtime.",
    "Channel registration/start/send/stop evidence does not support channel_message_handler inbound consumer.",
    "Tool family validation changed after frozen6711; corresponding tool families excluded from current supports, old receipt retained."
  ]
}
---

# SDK parent-driver candidate execution audit

## 支持的结论

Original immutable Host6711 uses the corrected canonical framework parent-driver
combination. Repository-owned archived logs prove SDK Rust418 (13 targets) and
TS240/Python300+18/Java264 plus examples; zero final-command failures and skips.
The supports list refers only to unchanged, named partial assertions inspected
against archived function/file fingerprints. It is not a 39-family closure vote.

## 来源与范围

The archive manifest retains exact commands, failed initial test, repair, final
logs and SHA256. Host is 6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5;
public manifest remains framework95ec plus temporary canonical path patch.
The final55 runtime/test input hash list is compared against current disk in
current-source-comparison.json. Workflow source subsequently changed; the earlier
whole418/three-language run cannot certify that successor. New Workflow candidate
receipts must describe their own source and narrower execution scope.

The earlier fc63 receipt remains a historical object, with original27files and
selected-function comparison archived separately. Java cleanup source evolved to
2bad; its current matching264-suite record belongs the original6711 language run,
not the old fc63 Java233 record. New Task/Subagent temporary four-scenario proofs
and LSP diagnostic42-observation corpus are separate bounded candidates; no test
count automatically proves every operation or lifecycle.

## 已知缺口

Workflow successor and all other changed production require fresh matching
verification. Channel MessageHandler inbound consumer is open; recorded plugin
start/send/stop cannot substitute it. Final explicit accepted-field inventory,
DiagnosticCode supporting union, no-patch published-pin build, platform/failure
matrices, final independent review and exact main remain pending. Baseline and
all family/group behaviors retain open/needs_review states.

## Tool validator后继边界

SDK tools.rs 在原6711运行后增加复用framework唯一schema/custom validator与arity拒绝；当前Tool families不能继承旧整体通过。预修改真实copy的源码f1以txt原字节保存，不用逆向重建冒原backup。旧工具簇正向/失败输入仍是该旧来源的有限历史证据，新增validator和chart坏type路径需自己的真实反例/验证。
