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
id: evidence.sdk-bootstrap-entry-continuity
evidence_type: semantic_continuity
merge_base_revision: 26c781e66260cca2851658f4784193df5980dc22
predecessor_revisions:
- 26c781e66260cca2851658f4784193df5980dc22
result_snapshot: source:351b7d0be4b93a9ab5357aa8d08efca0cfdec384d26a0ea22d8b7dc594fd6eab
resolutions:
  discovery.sdk-bootstrap#unresolved:echo-sdk-host/src/core_profile/facade/source_operations.rs:
    disposition: replaced
    replacement_ref: map.sdk-integrations#scenario:family-source-operation
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: 4d48d901027ffd4bd96c672471e033c843ca677b7546ee088f747d2c16c8bff9
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 只已接受具名调用与已审切片；宽cleanup/Sandbox/Task-resume共用B、其余未验组合仍依原Finding，不宣所有source
      operation verified。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
  discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/src/bin/export_schema.rs:
    disposition: replaced
    replacement_ref: map.sdk-product-boundary#scenario:accepted-contract
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: cfc57632280c3cc479c5dc56fc1eeac6caf49b886ab28d35e5cae9e37d870302
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 完整--check原13失败及12身份/另Curator已批准待seed仍独立；入口已知不等producer全绿或本次accept/retire。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
  discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/src/facade.rs:
    disposition: replaced
    replacement_ref: map.sdk-product-boundary#scenario:wave2-cutover
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: 5816328d0bd5950c24ef247b02122ebe759966af3fd191544cec802ea4bb2a87
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 别名/签名映射不授新Rust identity接受权，也不把目录等同所有business
      path执行。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
  discovery.sdk-bootstrap#unresolved:echo-sdk-protocol/tests/facade_inventory.rs:
    disposition: replaced
    replacement_ref: map.sdk-product-boundary#scenario:compatibility-gates
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: e782b8db157be4f72a0275f9ea217bbb3eb9a7eb17027af8ecaa9bc27fede22e
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 复用已审8个scope反例而非数量证明；商品合同13阻断、actual
      native/platform义务保留。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
  discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/client.py:
    disposition: replaced
    replacement_ref: map.sdk-language-clients#scenario:client-lifecycle-state-authority
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: 8122047e21186f9656eec2ea0a27e0e784f427b804128b056d4733c20277f0cb
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 有限8lens及具名错误修复不是所有并发/生命周期组合；新Linux/Host退出原因与7open不因入口闭合而消失。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
  discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/errors.py:
    disposition: replaced
    replacement_ref: map.sdk-language-clients#scenario:client-lifecycle-result-side-effect
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: ee254f7dc2d5fbdf004978dc6ff587a5dc8e4719bdb9239b732b55533c39cdd4
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 本Discovery按26c补真实源码/两consumer定位，复用Language
      supporting角色；不宣旧E已直接专审errors.py，不泛推Host全部error/cause或新业务后验。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
  discovery.sdk-bootstrap#unresolved:sdks/python/src/echo_agent_sdk/event_identity_values.py:
    disposition: replaced
    replacement_ref: map.sdk-language-clients#scenario:family-intrinsic
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: 190acef7236bb378da4d8c4dcbfdd34a114c9dc350ecb485bb93715c365bae05
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 采用当前f594已修源码而非拿ea21/旧0bc读它；PY3/Java3和Java17编译只原purevalue切片，不全Host或所有lineage完整verified。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
  discovery.sdk-bootstrap#unresolved:sdks/typescript/src/thinking_profile_values.ts:
    disposition: replaced
    replacement_ref: map.sdk-language-clients#scenario:family-intrinsic
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: 3aecb3d2cd025ef6009f6273a4a36320ca6044c7b7c8db213ad5c3b7db433b9a
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 本Discovery新补当前源码/consumer定位，不把旧E当专属实现审。只value解析/选择角色，不实际模型能力、远端请求或任意provider兼容承诺。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
  discovery.sdk-bootstrap#unresolved:sdks/typescript/test/intrinsic-tool-values.test.js:
    disposition: replaced
    replacement_ref: map.sdk-language-clients#scenario:family-intrinsic
    predecessor_fingerprints:
      26c781e66260cca2851658f4784193df5980dc22: 234c83cdc036e431b453777f9d3aee5d1214e56c6bfcd55dd7f27dd72f890953
    evidence_refs:
    - evidence.sdk-bootstrap-entry-equivalence
    decision_authorities: []
    compatibility_impact: Only the original entrance/authority/consumer-knowledge
      placeholder is replaced by its existing mapped/needs_review scenario. Runtime
      path, source and business states remain. 只前两段真实断言；末段28条映射计数不证明行为完整，入口mapped不等intrinsic
      family verified。
    rollback_ref: 26c781e66260cca2851658f4784193df5980dc22
---

# evidence.sdk-bootstrap-entry-continuity

## 支持的结论

将原bootstrap九个路径未知命题分别承接到已存在的具体Scenario；不是把“入口知识映射”冒充全部运行行为等价。原路径/源码/具名消费者仍在，原Scenario状态和已知缺陷不变；新当前source观察为26c，不用ea21读取185559后代码。原Discovery完整历史记录及九路径按原字节保在本次候选内。

## 来源与范围

九行事实/23个源码和consumer锚点已逐26c Git读取，8原入口相对0ebe同byte，仅PYidentity185已修并有既有局部三链；CurrentTool九行有界资格PASS只用于职责/authority/consumer不再未知。每个matched是这些实际定位检查，不伪造任何新native/业务执行。Admin单Discovery对照真实产生九个新missing，因此不能用文字保留替代图义务。

## 已知缺口

58resolved/7open、原场景mapped/needs_review、38+2格、B3/12/合同13、Host原因/新Linux、payload必要性与连续性历史诊断均保。旧391/1173不在此豁免；本组三metadata候选即使九replaced合法，也不称globalcontinuity0或授权删除raw。最终归档修复需重新绑定真实父集/候选源摘要，不把此知识迁移当泛化存储迁移许可。
