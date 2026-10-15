---
schema_version: 1
kind: evidence
observed_at: 26c781e66260cca2851658f4784193df5980dc22
source_refs:
- echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_non_git_source
- echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered
- echo-sdk-protocol/src/bin/export_schema.rs#acceptance_delta
- echo-sdk-protocol/src/facade.rs#resolve_route
- echo-sdk-protocol/src/inventory.rs#apply_accepted_source_scope
- echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_invalid_deltas_fail_atomically
- echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_new_routable_public_identity_never_implies_acceptance
- echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_reports_all_missing_sources_before_mutation_and_excludes_retired
- echo-sdk-protocol/tests/facade_inventory.rs#canonical_consumer_traits_and_streams_have_executable_routes
- sdks/python/src/echo_agent_sdk/client.py#class EchoAgentClient
- sdks/python/src/echo_agent_sdk/errors.py#def from_exception
- sdks/python/src/echo_agent_sdk/event_identity_values.py#def from_runtime_context
- sdks/python/tests/test_client_close_callbacks.py#test_cancelled_close_waiter_retains_same_cleanup_owner
- sdks/python/tests/test_event_identity_values.py#test_runtime_lineage_parent_survives_mapping_object_and_invocation_projection
- sdks/python/tests/test_protocol_integrity.py#test_cleanup_waits_for_same_pending_construction_and_retries
- sdks/python/tests/test_protocol_integrity.py#test_host_error_retains_handle
- sdks/python/tests/test_protocol_integrity.py#test_standard_acp_keeps_official_error_but_preserves_typed_extension_data
- sdks/typescript/package.json
- sdks/typescript/src/helpers.ts#export class ToolCallParams
- sdks/typescript/src/thinking_profile_values.ts#resolveThinkingProfile
- sdks/typescript/test/intrinsic-tool-values.test.js#ToolCallParams preserves Rust
  parameter typing and validation
- sdks/typescript/test/intrinsic-tool-values.test.js#ToolResult constructors and immutable
  modifiers match Rust semantics
- sdks/typescript/test/thinking-profile-values.test.js#thinking profiles preserve
  provider/model dialect selection
supports:
- discovery.sdk-bootstrap#unresolved:echo-sdk-host/src/core_profile/facade/source_operations.rs
- discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/src/bin/export_schema.rs
- discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/src/facade.rs
- discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/tests/facade_inventory.rs
- discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/client.py
- discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/errors.py
- discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/event_identity_values.py
- discovery.sdk-bootstrap#unresolved:sdks/typescript/src/thinking_profile_values.ts
- discovery.sdk-bootstrap#unresolved:sdks/typescript/test/intrinsic-tool-values.test.js
- map.sdk-integrations#scenario:family-source-operation
- map.sdk-language-clients#scenario:client-lifecycle-result-side-effect
- map.sdk-language-clients#scenario:client-lifecycle-state-authority
- map.sdk-language-clients#scenario:family-intrinsic
- map.sdk-product-boundary#scenario:accepted-contract
- map.sdk-product-boundary#scenario:compatibility-gates
- map.sdk-product-boundary#scenario:wave2-cutover
limitations:
- Knowledge-level entrance identification and semantic owner disposition only; no
  new runtime test or claim that all behavior is equivalent or verified.
- Runtime/source tree unchanged from26c. Existing mapped/needs_review scenarios and58resolved7open/B12/13/Hostcause/Linux/storage
  limits unchanged.
- This is only26c-to-candidate binding; original parent391/1173 diagnostics stay fatal.
  No archive deletion or B policy approval.
id: evidence.sdk-bootstrap-entry-equivalence
evidence_type: behavior_equivalence
before_revision: 26c781e66260cca2851658f4784193df5980dc22
after_revision: source:351b7d0be4b93a9ab5357aa8d08efca0cfdec384d26a0ea22d8b7dc594fd6eab
deleted_paths: []
coverage:
- nine original bootstrap entrance unknowns retained as historical records and replaced
  individually with specific existing protected scenarios
- all nine current source paths and named consumer anchors verified at26c; original
  current code tree and scenario statuses preserved
- current unknown knowledge disposition, not proof of all business behavior correctness
  or a retirement of pending risks
scenario_results:
  entry-1:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:echo-sdk-host/src/core_profile/facade/source_operations.rs
    replacement_ref: map.sdk-integrations#scenario:family-source-operation
    source_refs:
    - echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_non_git_source
    - echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
  entry-2:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/src/bin/export_schema.rs
    replacement_ref: map.sdk-product-boundary#scenario:accepted-contract
    source_refs:
    - echo-sdk-protocol/src/bin/export_schema.rs#acceptance_delta
    - echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_new_routable_public_identity_never_implies_acceptance
    - echo-sdk-protocol/src/inventory.rs#apply_accepted_source_scope
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
  entry-3:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/src/facade.rs
    replacement_ref: map.sdk-product-boundary#scenario:wave2-cutover
    source_refs:
    - echo-sdk-protocol/src/facade.rs#resolve_route
    - echo-sdk-protocol/tests/facade_inventory.rs#canonical_consumer_traits_and_streams_have_executable_routes
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
  entry-4:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/tests/facade_inventory.rs
    replacement_ref: map.sdk-product-boundary#scenario:compatibility-gates
    source_refs:
    - echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_reports_all_missing_sources_before_mutation_and_excludes_retired
    - echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_invalid_deltas_fail_atomically
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
  entry-5:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/client.py
    replacement_ref: map.sdk-language-clients#scenario:client-lifecycle-state-authority
    source_refs:
    - sdks/python/src/echo_agent_sdk/client.py#class EchoAgentClient
    - sdks/python/tests/test_client_close_callbacks.py#test_cancelled_close_waiter_retains_same_cleanup_owner
    - sdks/python/tests/test_protocol_integrity.py#test_cleanup_waits_for_same_pending_construction_and_retries
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
  entry-6:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/errors.py
    replacement_ref: map.sdk-language-clients#scenario:client-lifecycle-result-side-effect
    source_refs:
    - sdks/python/src/echo_agent_sdk/errors.py#def from_exception
    - sdks/python/tests/test_protocol_integrity.py#test_host_error_retains_handle
    - sdks/python/tests/test_protocol_integrity.py#test_standard_acp_keeps_official_error_but_preserves_typed_extension_data
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
  entry-7:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/event_identity_values.py
    replacement_ref: map.sdk-language-clients#scenario:family-intrinsic
    source_refs:
    - sdks/python/src/echo_agent_sdk/event_identity_values.py#def from_runtime_context
    - sdks/python/tests/test_event_identity_values.py#test_runtime_lineage_parent_survives_mapping_object_and_invocation_projection
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
  entry-8:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:sdks/typescript/src/thinking_profile_values.ts
    replacement_ref: map.sdk-language-clients#scenario:family-intrinsic
    source_refs:
    - sdks/typescript/src/thinking_profile_values.ts#resolveThinkingProfile
    - sdks/typescript/test/thinking-profile-values.test.js#thinking profiles preserve
      provider/model dialect selection
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
  entry-9:
    status: matched
    original_obligation: discovery.sdk-bootstrap#unresolved:sdks/typescript/test/intrinsic-tool-values.test.js
    replacement_ref: map.sdk-language-clients#scenario:family-intrinsic
    source_refs:
    - sdks/typescript/test/intrinsic-tool-values.test.js#ToolCallParams preserves
      Rust parameter typing and validation
    - sdks/typescript/test/intrinsic-tool-values.test.js#ToolResult constructors and
      immutable modifiers match Rust semantics
    - sdks/typescript/src/helpers.ts#export class ToolCallParams
    - sdks/typescript/package.json
    scope: Only originally unknown entrance/owner/named-consumer knowledge now represented;
      not all runtime behavior equivalence.
command_results:
- command: Admin official public22e extraction of9 old virtualIDs/fingerprints and
    validation of23 real26c source/consumer references/existing canonical scenario
    targets; independent CurrentTool bounded fact review PASS; no runtime execution
  exit_code: 0
---

# evidence.sdk-bootstrap-entry-equivalence

## 支持的结论

将原bootstrap九个路径未知命题分别承接到已存在的具体Scenario；不是把“入口知识映射”冒充全部运行行为等价。原路径/源码/具名消费者仍在，原Scenario状态和已知缺陷不变；新当前source观察为26c，不用ea21读取185559后代码。原Discovery完整历史记录及九路径按原字节保在本次候选内。

## 来源与范围

九行事实/23个源码和consumer锚点已逐26c Git读取，8原入口相对0ebe同byte，仅PYidentity185已修并有既有局部三链；CurrentTool九行有界资格PASS只用于职责/authority/consumer不再未知。每个matched是这些实际定位检查，不伪造任何新native/业务执行。Admin单Discovery对照真实产生九个新missing，因此不能用文字保留替代图义务。

## 已知缺口

58resolved/7open、原场景mapped/needs_review、38+2格、B3/12/合同13、Host原因/新Linux、payload必要性与连续性历史诊断均保。旧391/1173不在此豁免；本组三metadata候选即使九replaced合法，也不称globalcontinuity0或授权删除raw。最终归档修复需重新绑定真实父集/候选源摘要，不把此知识迁移当泛化存储迁移许可。
