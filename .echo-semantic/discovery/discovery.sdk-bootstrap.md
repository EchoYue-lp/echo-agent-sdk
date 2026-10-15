---
schema_version: 1
id: discovery.sdk-bootstrap
kind: discovery
source_snapshot:
  base_revision: 26c781e66260cca2851658f4784193df5980dc22
  content_digest: 351b7d0be4b93a9ab5357aa8d08efca0cfdec384d26a0ea22d8b7dc594fd6eab
scope: independent SDK repository, framework pin and accepted contract boundary
inspected_paths: [Cargo.toml, echo-sdk-protocol, echo-sdk-host, contracts/sdk, sdks, docs/sdk, scripts, MIGRATION-SOURCE.md, docs/adr]
candidate_refs: [map.sdk-product-boundary, asset.sdk-source-product]
unresolved: []
---

# SDK bootstrap discovery

## 扫描范围

扫描 source-import workspace、协议/Host 入口、合同、三语言源码、文档、脚本与 repository ADR。

## 候选事实

仓库已具备单一 SDK 产品边界；protocol purity、external contract 分离、Host pin 和独立语言门禁
已经闭合并由 clean-pin Evidence 证明。

## 归并结果

当前聚合为一个产品 Asset、一个 ownership Behavior、一个 framework authority Rule、一张开放
Capability Map 与 clean-pin Evidence。

## 未决项

列出的动态入口仍需要在后续 capability/lifecycle 治理中进一步建模；它们不否定本次已验证的
repository、dependency、contract 和 language gate 边界。


## 九项入口未知命题的当前有界处置（26c）

本文件原四节正文与以下bootstrap元数据引文均为历史原记录。当前schema的source_snapshot只绑定本次九入口职责/消费者定位的当前源树观察，不声称做了新全仓scan，也不把旧ea21读取成后来的185559源码。旧“仍需后续建模”的具体命题是入口/owner/具名consumer尚未知；本次逐行给出已存在归属与适用边界，所以移出unresolved，既不是bugfree/familyverified，也不接受未知风险或删除产品义务。

原完整bootstrap元数据（逐字保留值和九路径）：

```yaml
schema_version: 1
id: discovery.sdk-bootstrap
kind: discovery
source_snapshot:
  base_revision: ea21dfc58fa576296a0b1d0d3267c9632d84f0ae
  content_digest: d66d41ae73c5e05d8bc1781eb8f457abe1f0e2730a8e3b40f5af32a500a7616f
scope: independent SDK repository, framework pin and accepted contract boundary
inspected_paths: [Cargo.toml, echo-sdk-protocol, echo-sdk-host, contracts/sdk, sdks, docs/sdk, scripts, MIGRATION-SOURCE.md, docs/adr]
candidate_refs: [map.sdk-product-boundary, asset.sdk-source-product]
unresolved: [echo-sdk-host/src/core_profile/facade/source_operations.rs, echo-sdk-protocol/src/bin/export_schema.rs, echo-sdk-protocol/src/facade.rs, echo-sdk-protocol/tests/facade_inventory.rs, sdks/python/src/echo_agent_sdk/client.py, sdks/python/src/echo_agent_sdk/errors.py, sdks/python/src/echo_agent_sdk/event_identity_values.py, sdks/typescript/src/thinking_profile_values.ts, sdks/typescript/test/intrinsic-tool-values.test.js]
```

原Discovery blob可从已发布的保存Git `26c781e66260cca2851658f4784193df5980dc22` 读取，SHA256 `a358b6585f90fb403bd3aa223c146e956f8e68fcaa406086bba75001aeb7b33d`；[不可变历史原件](https://github.com/EchoYue-lp/echo-agent-sdk/blob/26c781e66260cca2851658f4784193df5980dc22/.echo-semantic/discovery/discovery.sdk-bootstrap.md)。原source_snapshot的ea21/d66值在引文中按原记录保留，不冒它已被本次重算验证。

当前源码观察为Git `26c781e66260cca2851658f4784193df5980dc22`、官方工作树与同Git source摘要 `351b7d0be4b93a9ab5357aa8d08efca0cfdec384d26a0ea22d8b7dc594fd6eab`；九源相对0ebe有八项byte相同，仅PY EventIdentity为185559已修f594。源码与锚点逐Git核可读，原业务审查和运行收据仍使用各自原epoch。尤其errors与thinking_profile本行是新直接定位，不倒称旧E已专审对应实现。

| 原未决路径/命题 | 当前职责与真实消费者 | 已有语义归属 | 仍保留的界限 |
|---|---|---|---|
| `echo-sdk-host/src/core_profile/facade/source_operations.rs#dispatch_non_git_source`；Host source dispatcher：原framework服务的寻址/无损转换；不拥有第二Task/DAG或Git业务权威。 | `echo-sdk-host/tests/core_profile_e2e.rs#turn_receipt_source_operations_project_live_and_recovered` | `map.sdk-integrations` / `map.sdk-task-subagent` / `map.sdk-extension-bridge` / `evidence.sdk-worktree-source-stack-repair` / `evidence.sdk-tools-bridge-finite-observation` | 只已接受具名调用与已审切片；宽cleanup/Sandbox/Task-resume共用B、其余未验组合仍依原Finding，不宣所有source operation verified。 |
| `echo-sdk-protocol/src/bin/export_schema.rs#acceptance_delta`；唯一artifact生成/检查入口；先应用显式accepted source/kind，再写生成物；不是Rust inventory自动接纳语言合同。 | `echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_new_routable_public_identity_never_implies_acceptance`；`echo-sdk-protocol/src/inventory.rs#apply_accepted_source_scope` | `map.sdk-product-boundary` / `evidence.sdk-protocol-finite-observation` / `audit.sdk-protocol-finite-contract-evidence` / `finding.sdk-accepted-source-continuity` | 完整--check原13失败及12身份/另Curator已批准待seed仍独立；入口已知不等producer全绿或本次accept/retire。 |
| `echo-sdk-protocol/src/facade.rs#resolve_route`；canonical source identity→闭合family route表/operation catalog；是Protocol静态路由合同，Host仍实际执行。 | `echo-sdk-protocol/tests/facade_inventory.rs#canonical_consumer_traits_and_streams_have_executable_routes` | `map.sdk-product-boundary` / `evidence.sdk-wave2-cutover-final-verification` / `rule.framework-runtime-authority` | 别名/签名映射不授新Rust identity接受权，也不把目录等同所有business path执行。 |
| `echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_reports_all_missing_sources_before_mutation_and_excludes_retired`；库存/accepted scope/codec与route的具名测试消费者；supporting测试，不是生产状态registry。 | `echo-sdk-protocol/tests/facade_inventory.rs#acceptance_scope_invalid_deltas_fail_atomically` | `map.sdk-product-boundary` / `evidence.sdk-protocol-finite-observation` / `audit.sdk-protocol-finite-contract-evidence` | 复用已审8个scope反例而非数量证明；商品合同13阻断、actual native/platform义务保留。 |
| `sdks/python/src/echo_agent_sdk/client.py#class EchoAgentClient`；官方ACP客户端/extension callback与本地transport owner；实际close/重试/Factory/cancel地址由既有Language/Bridge模型拥有。 | `sdks/python/tests/test_client_close_callbacks.py#test_cancelled_close_waiter_retains_same_cleanup_owner`；`sdks/python/tests/test_protocol_integrity.py#test_cleanup_waits_for_same_pending_construction_and_retries` | `map.sdk-language-clients` / `map.sdk-extension-bridge` / `evidence.sdk-language-client-lifecycle-current` / `evidence.sdk-python310-factory-timeout-closure` | 有限8lens及具名错误修复不是所有并发/生命周期组合；新Linux/Host退出原因与7open不因入口闭合而消失。 |
| `sdks/python/src/echo_agent_sdk/errors.py#def from_exception`；EchoAgentError.from_exception保code/message/retryable/operation/details/handle的本地typed投影；不从message猜原因。 | `sdks/python/tests/test_protocol_integrity.py#test_host_error_retains_handle`；`sdks/python/tests/test_protocol_integrity.py#test_standard_acp_keeps_official_error_but_preserves_typed_extension_data` | `map.sdk-language-clients` / `behavior.sdk-language-clients` / `rule.framework-runtime-authority` | 本Discovery按26c补真实源码/两consumer定位，复用Language supporting角色；不宣旧E已直接专审errors.py，不泛推Host全部error/cause或新业务后验。 |
| `sdks/python/src/echo_agent_sdk/event_identity_values.py#def from_runtime_context`；EventIdentity pure value构造/immutable投影；原nested lineage丢parent具体缺陷已185559修，状态权威仍原framework。 | `sdks/python/tests/test_event_identity_values.py#test_runtime_lineage_parent_survives_mapping_object_and_invocation_projection` | `map.sdk-language-clients` / `behavior.sdk-family-value` / `evidence.sdk-event-identity-nested-parent-closure` / `audit.sdk-event-identity-nested-parent-closure` / `finding.sdk-event-identity-nested-parent-projection` | 采用当前f594已修源码而非拿ea21/旧0bc读它；PY3/Java3和Java17编译只原purevalue切片，不全Host或所有lineage完整verified。 |
| `sdks/typescript/src/thinking_profile_values.ts#resolveThinkingProfile`；provider/model字符串→只读thinking dialect/levels本地值；resolveThinkingProfile不联系provider。 | `sdks/typescript/test/thinking-profile-values.test.js#thinking profiles preserve provider/model dialect selection` | `map.sdk-language-clients` / `behavior.sdk-family-value` | 本Discovery新补当前源码/consumer定位，不把旧E当专属实现审。只value解析/选择角色，不实际模型能力、远端请求或任意provider兼容承诺。 |
| `sdks/typescript/test/intrinsic-tool-values.test.js#ToolCallParams preserves Rust parameter typing and validation`；ToolCallParams与ToolResult本地值的具名消费者，typing/错误/immutable断言；supporting purevalue而非Mock工具业务E2E。 | `sdks/typescript/test/intrinsic-tool-values.test.js#ToolResult constructors and immutable modifiers match Rust semantics`；`sdks/typescript/src/helpers.ts#export class ToolCallParams`；`sdks/typescript/package.json` | `map.sdk-language-clients` / `behavior.sdk-family-intrinsic` | 只前两段真实断言；末段28条映射计数不证明行为完整，入口mapped不等intrinsic family verified。 |

移出的九个路径没有从Git、原历史记录或当前产品中删除；对应已知功能缺陷继续由各自Finding/Behavior拥有。本候选不改58resolved/7open、inventoryclosed/behavioropen、40格38covered/2notapplicable、accepted seed、B3/12选择、合同13、Hostcause/新Linux验收、SDKmain或payload必要性BLOCK。该Discovery入口命题已知，不等全部动态组合/所有public API已审。

这是普通Discovery已知化的候选，不是删除repair授权或完整连续性结果。原虚拟未知项身份 `discovery.sdk-bootstrap#unresolved:<原路径>` 在历史父树中仍可恢复，九行给出其知识归属；显式continuity的旧391/1173与这些身份迁移仍须合法处置，不因unresolved为空就声称其通过。没有删除raw、抹原failed limits、退役能力或放宽tool/permission；原parent/self诊断不能被本次入口表掩盖。
