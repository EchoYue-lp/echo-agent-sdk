---
schema_version: 1
id: evidence.sdk-parent-driver-candidate-verification
kind: evidence
observed_at: 9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d
source_refs:
- echo-sdk-host/tests/core_profile_e2e.rs#valid_hello_completes_full_core_lifecycle
- echo-sdk-host/tests/facade_feature_adapters_e2e.rs#agent_cascade_preserves_same_named_other_manager_and_releases_resources
- echo-sdk-host/src/lib.rs#manual_close_keeps_active_run_actor_until_cleanup_and_receipt_settle
- echo-sdk-host/tests/extension_bridge_e2e.rs#sandbox_cancel_bridge_waits_for_cleanup_and_preserves_cancelled_terminal
- echo-sdk-host/src/core_profile/wire.rs#managed_checkpoint_and_receipts_preserve_large_revisions_and_subseconds
- echo-sdk-host/src/core_profile/facade/memory.rs#malformed_namespace_never_writes_and_replacement_store_is_live_authority
- echo-sdk-host/src/core_profile/facade/state_delivery.rs#exact_claim_roundtrip_rejects_delayed_attempt_and_id_only
- echo-sdk-host/src/core_profile/facade/observability.rs#failed_trace_open_does_not_consume_the_only_resource_slot
- echo-sdk-host/src/core_profile/facade/source_operations.rs#factory_failure_retains_canonical_cause_in_real_eval_result
- echo-sdk-host/src/core_profile/facade/telemetry.rs#invalid_console_flag_is_rejected_before_process_initialization
- echo-sdk-host/src/core_profile/facade/tools.rs#tool_family_projection_preserves_partial_effects_and_artifact
- echo-sdk-host/src/core_profile/facade/mod.rs#family_handle_argument_retains_opaque_resource_address
- echo-sdk-host/src/core_profile/events.rs#real_journal_retention_page_authorizes_only_its_returned_watermark
supports:
- behavior.sdk-family-agent-lifecycle
- behavior.sdk-family-session
- behavior.sdk-family-run
- behavior.sdk-family-event-replay
- behavior.sdk-family-task
- behavior.sdk-family-subagent
- behavior.sdk-family-bridge
- behavior.sdk-family-standard
- behavior.sdk-family-value
- behavior.sdk-family-intrinsic
- behavior.sdk-family-memory
- behavior.sdk-family-delivery
- behavior.sdk-family-state
- behavior.sdk-family-trace
- behavior.sdk-family-eval
- behavior.sdk-family-improve
- behavior.sdk-family-permission
- behavior.sdk-family-mcp
- behavior.sdk-family-a2a
- behavior.sdk-family-lsp
- behavior.sdk-family-channels
- behavior.sdk-family-telemetry
- behavior.sdk-family-topology
- behavior.sdk-family-structured-output
- behavior.sdk-family-source-operation
- behavior.sdk-family-invoke
limitations:
- Original6711 Rust418/TS240/Python300+18/Java264 executable scope; repository source changed afterward.
- Current Workflow and Tool validator differ from frozen55runtimefiles; corresponding current supports excluded until successor receipts are bound.
- Supports are named partial scenario assertions only; every Behavior remains needs_review.
- Temporary framework path patches, not published framework pin/main or final accepted inventory.
- Testing value-only and tool clusters have separate bounded Evidence; neither counts as new Mock runtime.
- Channel registration/start/send/stop evidence does not support channel_message_handler inbound consumer.
- Tool family validation changed after frozen6711; corresponding tool families excluded from current supports, old receipt retained.
asset_refs:
- asset.sdk-historical-proof-location
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

## 本次 current_source_business_rereview

原已审SDKruntime代码来源Git569/source17cf和治理文稿可恢复Git`9db7bd7f0b732ff7aa3bafa07cce02cc292e2a9d`（source0a13）明确分开。独立reader`current_tool_boundary_review`本次实际重读原业务fault/完整字段/副作用/owner/调用方变化与14Rust/newbb4f632原after证据，消息回收UTC`2026-10-14T01:56:14.724478+00:00`，不是原执行时间。结果：**具名normal/recovery/ACK/taskResult/cleanup有限PASS；26宽supports物理5s I1 BLOCK/needs_review**。

原55runtime/58namedbody6711历史whole不是currentrun。新14/bb4f632只支持具名对应行为；sourcechangedcallbacklease/Taskcontroller/Toolvalidator已分current审，但不能把26支持当全当前能力闭合或造3ea/9ewholeGit。5.10s强abort清claimphysicalalive原独立Finding仍OPEN，不删26义务避BLOCK。

原观测marker`source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404`、旧run-time/host/sourcehash/raw和正文保，所有source_refs从真实9dbGitblob读取且锚点存在；现在SDKbe5pin与新测试写入会改变当前文件，不能把历史b909当前源码审升级成未来be5gate。真实9db框架pin为b909，工作树be5after另Evidence处理，不将两个仓tree混写。新增门禁sourceRef只具名Scope，没有以整count判族verified；supports/limits原保、不声明旧3ea整仓可恢复、不改unknown成无缺陷。

原义务和全文历史主张保留以审查，若旧unknown因素（TTL/三getter消费）已有新原证据，只在本段给来源区别，不抹原epoch。ChartI1/5sB历史BLOCK明确不因observed版本合法消失，12source/CI/main/globalacceptedproducer仍pending，41family/lens状态不提升。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
