# SDK A2A/LSP retirement boundary and exact baseline inventory

Status: SDK consumer withdrawal under [ADR 0042](../../adr/0042-sdk-a2a-lsp-retirement-boundary.md).
The original266 inventory below is a historical baseline; current Plan04
withdraws269. 消费者撤出、Framework物理删除及SDK正式采用删后源分别验收；原固定pin不反向证明新源已采用。This document is not a second accepted-contract authority.

## 2026-11-04：接收退役边界回归测试

当前 SDK 在 `83ae59e` 基础上接收候选的四份既有测试增量，共新增154行。
Rust `scalar` 测试检查退役字段／变体被拒绝，同时允许用户自定义类型使用同名值；
Java、Python 和 TypeScript 测试逐项核对 ADR0042 的九项 JWT 退役身份不进入
accepted 合同，也不恢复退役语言义务。生产实现、Host pin、锁和生成合同不变。

`echo-sdk-host/tests/core_profile_e2e.rs` 保留 HEAD 在仓库 `target/` 下创建夹具的
沙箱可见性修复，不采用候选中的系统临时目录版本。此处测试接收不代表候选的
历史验证已自动覆盖当前工作区；实际工程结果与剩余 Source／Final、风险及平台门禁分别结算。

本次未新增 API、调用方式或产品行为，已有 examples 和官网说明无需调整；
四份回归测试进入各仓原有 Cargo／三语言测试入口，不新增平行合同或测试执行器。

## 2026-10-31：正式采用删后来源

当前正式 Host 通过 HTTPS 精确采用 framework `340993201cc937fc8e4783b66be399ef9f14e3d5`
（tree `e6c28f37a881f41bb549452614313557b7c91c68`），SDK 锁 SHA-256 为
`c4488065aa4a94a8a073e427aeddfefe7670d93b709b82555b39f2431954e1c7`。
当前 accepted 集合从5352项退役11项、接受3项 with_id 方法后为5344项身份／430条路由；
native 新增 Rust source 身份为零，没有超出具名批准范围的接受项。后续具名来源处置见
[ADR0044](../../adr/0044-sdk-source-disposition-native-callback.md)。本地验证保留原18项
各自源码与运行资格，正式采用通过完整内容身份桥接，不替代精确 main CI、平台原生或发布验收。

下文2026-10-28准备、Plan04撤出及原基线表均为历史切点。原269身份／23路由／39操作、
5352／431、旧 pin 和锁统计保持原值；当前430条路由不重写为原 LSP 多退役一条。

## 2026-10-28：Framework 删除后的薄投影接缝准备（历史）

SDK Host 的 `resolved_plugin_components_value` 已按治理 SDK1 补丁解除对
`ResolvedComponents::lsp_config_file` 的编译依赖。旧声明的发现与诊断继续由
Framework resolver／prepare 拥有；Host 原样保留 `diagnostics` 的内容、顺序及空列表，
不再追加 SDK 自有的 LSP 退役诊断，也不从路径推测诊断。技能、Agent、Hook、MCP
等保留组件继续使用原投影，没有新增状态权威、wire 字段或语言 API。

本轮只采用正式源码、补充两项投影测试并同步所属文档。测试覆盖 Framework 原诊断
逐项保留，以及空诊断不被补造；尚未启动 Cargo，删后组合的编译与运行资格待
Framework writer 发布新源码并由 Root 开放受控验证窗口。正式 Host pin 仍为
`f7bac62eee04a963f92a3e4308b6ea2336e91cc0`，正式锁 SHA256 仍为
`0dacbb873673a405fed4e820499ff33e40301cefc5905e8c2bc597558156231f`；本轮没有
更新 accepted、生成物或来源校验，也不将旧 234 项 lib／三语言结果重标为删后通过。

SDK1 落实 ADR0042 已批准的退役边界，不引入新的架构决策。`examples` 和官网相关内容
已检查：此次没有新增公开操作、调用方式或产品文案，故不修改；原示例将随删后受控
组合验证重新取证。旧 Evidence 的 after／status／matched 保留原输入资格。

## Plan04 explicit extension and consumer withdrawal

The baseline266 inventory below remains its original source cut. Plan04 adds
only these already-accepted defining-source/kind keys:

- `variant echo_core::plugin::capability::PluginCapability::LspServer`
- `struct_field echo_agent::plugin::prepared::PluginWiringResult::lsp_discovered`
- `struct_field echo_core::plugin::registry::ResolvedComponents::lsp_config_file`

The union is269; the retained set is5352 rather than5355. Routes are454 minus
exactly23 =431, and39 closed operations leave. Additions are zero. ADR0042
provides269 independent machine-readable sections for the existing full
`--retire-source` generator. The shared language catalog/digests still use
`scripts/export-language-sdk-catalog.sh`. The executable exact-delta check is
`scripts/check-a2a-lsp-retirement.py`, bound to published protected baseline
`0930eef9f878cf436ed88384030b6bc03d6f6c53`. Its accepted artifacts are byte-identical
to the task's local S baseline `961da0e0`; the contract CI job explicitly fetches
and verifies tag `pre-retirement-2026-10-24` before checking. It also checks retained
entry identity and prohibits unrelated contract drift. 原消费者撤出阶段无需pin升级（不适用于正式采用物理删除后的新源）.

## Snapshot and scope

The enumeration uses SDK baseline `0930eef9f878cf436ed88384030b6bc03d6f6c53`.
The Host independently pins framework
`f7bac62eee04a963f92a3e4308b6ea2336e91cc0`; top-level framework baseline
`51726454985fd6f064b21a7e9c539e22d494c987` is a distinct input from the approved
cross-repository design. The original documentation-only slice changed no generator, manifest, pin,
lock, code or accepted artifact; Plan04 now changes the SDK consumer inputs
as described above while retaining the independent framework pin.

Formal inventory sources are
[accepted external identities](../../../contracts/sdk/accepted-external-contract.json)
and the [accepted facade catalog](../../../contracts/sdk/accepted-facade-operation-catalog.json).
The [source contract](../../../contracts/sdk/source-contract.json) binds their
runtime compatibility digest. Full Rust inventory, aliases and full Host-only
catalog contents are telemetry/additional implementation inputs, not extra
accepted identities to remove. `sdk_scope == external_contract` is required;
an intrinsic route does not negate that acceptance.

| Denominator                                  |         Current baseline | A2A candidates | LSP candidates | Retained if only these candidates are removed |
| -------------------------------------------- | -----------------------: | -------------: | -------------: | --------------------------------------------: |
| Accepted canonical `(path, kind)` identities |                    5,621 |            160 |            106 |                                         5,355 |
| Accepted facade catalog route rows           |                      454 |              9 |             14 |                                           431 |
| Closed family operation names                | Separate from route rows |              9 |             30 |                  Other family lists unchanged |

A2A's 160 include 113 entries with route family `a2a` and **47 accepted entries
with route family `intrinsic`** (AgentCard/builder, message/task values,
TaskState and JWT helpers). LSP's 106 all have route family `lsp`. Every listed
entry is canonical; aliases must follow their canonical retirement rather than
count again. The retained numbers are subtraction invariants for a later
review, not claims that generation or runtime removal has happened.

### Recompute the exact candidate set

Run from the SDK Git root. This read-only selector checks feature membership,
canonical/source namespace and route family only inside the accepted artifact.
It does not consult the complete Rust inventory.

```python
import json
import subprocess
from collections import Counter

def baseline(path):
    return json.loads(subprocess.check_output(["git", "show", f"0930eef9f878cf436ed88384030b6bc03d6f6c53:{path}"]))

entries = baseline("contracts/sdk/accepted-external-contract.json")["entries"]
catalog = baseline("contracts/sdk/accepted-facade-operation-catalog.json")

def belongs(entry, family):
    paths = [entry["path"], *entry["source_paths"]]
    return (
        family in entry["features"]
        or any(f"::{family}::" in path for path in paths)
        or entry["route"]["family"] == family
    )

for family in ("a2a", "lsp"):
    chosen = [entry for entry in entries if belongs(entry, family)]
    routes = [route for route in catalog["routes"] if route["family"] == family]
    print(family, len(chosen), Counter(entry["kind"] for entry in chosen), len(routes))
    for entry in chosen:
        print(entry["path"], entry["kind"], entry["source_paths"], entry["route"]["route"])
```

The detailed tables below retain the exact canonical path, kind, defining
`source_paths` and route. Signature digests, feature semantics, classification
and per-language mapping/test identifiers remain in the linked accepted JSON;
the route table and closed operation list are copied from the formal catalog.
All these candidate entries have the single feature `a2a` or `lsp`, respectively.

## Implementation and consumer disposition

| Owner / formal location                                                                                                                                                                                                                                                        | 原Plan04处置范围                                                                                      | Preserve / check                                                                                                         |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| [Host manifest](../../../echo-sdk-host/Cargo.toml)                                                                                                                                                                                                                             | `framework-a2a`, `framework-lsp`, their membership in `sdk-facade-all`                                       | Original framework pin can remain for consumer decoupling; default runtime ACP/MCP and other feature passthroughs remain |
| [Compiled feature authority](../../../echo-sdk-host/src/features.rs)                                                                                                                                                                                                           | Two leaf-feature bindings and two family advertisements                                                      | Advertisement still follows compiled handlers; retain other families                                                     |
| [Integration adapter](../../../echo-sdk-host/src/core_profile/facade/integrations.rs)                                                                                                                                                                                          | `A2aClientRecord`, `LspManagerRecord`, a2a/lsp/lsp_clients maps, family handlers and owned teardown branches | MCP/topology in the same file and generic lifecycle owners remain                                                        |
| [Facade admission](../../../echo-sdk-host/src/core_profile/facade/mod.rs), [registry](../../../echo-sdk-host/src/core_profile/facade/registry.rs), [streams](../../../echo-sdk-host/src/core_profile/facade/stream.rs)                                                         | Family-specific dispatch/resource references after their users go                                            | Generic invoke, ownership/generation, stream cancellation and error semantics remain                                     |
| [Protocol family definitions](../../../echo-sdk-protocol/src/facade.rs), [method catalog](../../../echo-sdk-protocol/src/catalog.rs), [inventory classification](../../../echo-sdk-protocol/src/inventory.rs), [exporter](../../../echo-sdk-protocol/src/bin/export_schema.rs) | Two closed families, methods, exact accepted intrinsic groups and a2a/lsp profile inputs                     | Inspect shared WireValue/DTO/capability/handle users before removal; use the unique generator                            |
| `TS A2A values`（已撤出历史路径：`../../../sdks/typescript/src/a2a.ts`）, [exports](../../../sdks/typescript/src/index.ts), `JWT values`（已撤出历史路径：`../../../sdks/typescript/src/jwt_values.ts`）, [catalog](../../../sdks/typescript/src/catalog.ts)                                                           | Exact A2A exports, helpers and catalog membership                                                            | Shared JWT/security consumers and generic Session invoke remain                                                          |
| `Python A2A values`（已撤出历史路径：`../../../sdks/python/src/echo_agent_sdk/a2a.py`）, [exports](../../../sdks/python/src/echo_agent_sdk/__init__.py), `JWT values`（已撤出历史路径：`../../../sdks/python/src/echo_agent_sdk/jwt_values.py`）, [catalog](../../../sdks/python/src/echo_agent_sdk/catalog.py)        | Same exact accepted A2A subset and catalog membership                                                        | Shared value modules and generic Session invoke remain                                                                   |
| [Java package](../../../sdks/java/src/main/java/com/echoagent/sdk/)                                                                                                                                                                                                            | A2A, AgentCard/builder, TaskState, relevant JWT value members and facade catalog subset                      | These are names within this accepted subset, not authority to remove ordinary task state or all JWT code                 |
| [Shared language catalog](../../../sdks/shared/facade-operation-catalog.json)                                                                                                                                                                                                  | Exact 23 route projection and two family declarations                                                        | Generator-owned; generic invoke and all retained families stay                                                           |

There is no dedicated LSP language module/class required by the inspected
clients: TS/Python/Java use the catalog, Session `invoke` and typed WireValue/
resource/stream handling. Do not invent or delete a guessed `LspClient` wrapper.
In particular, A2A `TaskState` is not the shared TaskRun/PlanTask state authority.

### Active validation, examples, docs and CI consumers

- Protocol checks: [facade inventory](../../../echo-sdk-protocol/tests/facade_inventory.rs)
  and [core RPC contract](../../../echo-sdk-protocol/tests/core_rpc_contract.rs).
  Host checks: [feature adapters](../../../echo-sdk-host/tests/facade_feature_adapters_e2e.rs)
  and [core profile](../../../echo-sdk-host/tests/core_profile_e2e.rs).
- TS tests: `sdks/typescript/test/{a2a-host,a2a-task-state,lsp-host,lsp-diagnostics-host,jwt-values,catalog}.test.js`.
  Python tests: `sdks/python/tests/test_{a2a_host,a2a_task_state,lsp_host,lsp_diagnostics_host,jwt_values,catalog}.py`.
  Java tests in `sdks/java/src/test/java/com/echoagent/sdk/` include
  `A2AHostTest.java`, `A2AValueTest.java`, `LspHostTest.java`, `LspDiagnosticsHostTest.java`.
  Inspect shared catalog/JWT tests rather than deleting entire test files by a word match.
- Local peer fixtures: `A2A server`（已撤出历史路径：`../../../sdks/shared/testing/a2a_host_fixture.py`）,
  `LSP faults`（已撤出历史路径：`../../../sdks/shared/testing/lsp_fault_server.py`）,
  `LSP diagnostics`（已撤出历史路径：`../../../sdks/shared/testing/lsp_diagnostic_server.py`）.
  Later replace active obsolete positive coverage with explicit negative
  retirement coverage and retained-capability controls; preserve historical evidence.
- Executable examples: [TS quickstart](../../../sdks/typescript/examples/quickstart.ts),
  [Python quickstart](../../../sdks/python/examples/quickstart.py),
  [Java Example](../../../sdks/java/src/main/java/com/echoagent/sdk/Example.java).
  They remain unchanged and applicable at this still-full baseline; their
  continued execution is required after retirement, not inferred from this doc.
- Formal current explanations remain in [feature adapters](../facade-feature-adapters.md),
  [protocol](../protocol.md), [SDK index](../README.md) and each language README.
  Later align active support tables after implementation; do not rewrite historic
  acceptance evidence, Findings or old source-cut results as though deletion repaired them.
- [Contract gate](../../../scripts/check-sdk-contracts.sh),
  [language gate](../../../scripts/check-language-sdks.sh),
  [inventory telemetry gate](../../../scripts/check-sdk-inventory-telemetry.sh)
  and [CI](../../../.github/workflows/rust-ci.yml) consume feature profiles and
  shared catalogs transitively. No direct `a2a`/`lsp` literal was found in scripts/CI
  at this snapshot; that is not proof that their full-profile inputs are unaffected.

SDK source/contract/Host and all three language checks must be repeated after
the future real implementation. This document's count/link checks do not prove
consumer compatibility, physical settlement, native platforms, main CI or release.
Website projection and cross-repository rule synchronization belong to the
coordinator; this SDK slice changes no website content.

## A2A — exact accepted identity and kind set

Count: 160. Kind distribution: `{"constant": 12, "enum": 3, "method": 38, "struct": 20, "struct_field": 75, "trait_impl": 2, "variant": 10}`.

Feature: `a2a`; Host passthrough: `framework-a2a`; family method: `_echo_agent/a2a/op`.

| Canonical path                                                 | Kind           | Formal source_paths                                                    | Route                                                                                                            |
| -------------------------------------------------------------- | -------------- | ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `echo_agent::a2a::A2AArtifact`                                 | `struct`       | `echo_agent::a2a::types::A2AArtifact`                                  | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AArtifact::append`                         | `struct_field` | `echo_agent::a2a::types::A2AArtifact::append`                          | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AArtifact::index`                          | `struct_field` | `echo_agent::a2a::types::A2AArtifact::index`                           | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AArtifact::name`                           | `struct_field` | `echo_agent::a2a::types::A2AArtifact::name`                            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AArtifact::parts`                          | `struct_field` | `echo_agent::a2a::types::A2AArtifact::parts`                           | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AClient::cancel_task`                      | `method`       | `echo_agent::a2a::client::A2AClient::cancel_task`                      | `family-operation:a2a:a2a.task.cancel:echo_agent::a2a::client::A2AClient::cancel_task`                           |
| `echo_agent::a2a::A2AClient::discover`                         | `method`       | `echo_agent::a2a::client::A2AClient::discover`                         | `family-operation:a2a:a2a.discover:echo_agent::a2a::client::A2AClient::discover`                                 |
| `echo_agent::a2a::A2AClient::get_task`                         | `method`       | `echo_agent::a2a::client::A2AClient::get_task`                         | `family-operation:a2a:a2a.task.get:echo_agent::a2a::client::A2AClient::get_task`                                 |
| `echo_agent::a2a::A2AClient::send_task`                        | `method`       | `echo_agent::a2a::client::A2AClient::send_task`                        | `family-operation:a2a:a2a.task.send:echo_agent::a2a::client::A2AClient::send_task`                               |
| `echo_agent::a2a::A2AClient::send_task_streaming`              | `method`       | `echo_agent::a2a::client::A2AClient::send_task_streaming`              | `family-operation:a2a:a2a.task.stream.open:echo_agent::a2a::client::A2AClient::send_task_streaming`              |
| `echo_agent::a2a::A2AClient::send_task_streaming_with_session` | `method`       | `echo_agent::a2a::client::A2AClient::send_task_streaming_with_session` | `family-operation:a2a:a2a.task.stream.open:echo_agent::a2a::client::A2AClient::send_task_streaming_with_session` |
| `echo_agent::a2a::A2AClient::send_task_with_session`           | `method`       | `echo_agent::a2a::client::A2AClient::send_task_with_session`           | `family-operation:a2a:a2a.task.send:echo_agent::a2a::client::A2AClient::send_task_with_session`                  |
| `echo_agent::a2a::A2AError`                                    | `struct`       | `echo_agent::a2a::types::A2AError`                                     | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AError::code`                              | `struct_field` | `echo_agent::a2a::types::A2AError::code`                               | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AError::message`                           | `struct_field` | `echo_agent::a2a::types::A2AError::message`                            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AMessage`                                  | `struct`       | `echo_agent::a2a::types::A2AMessage`                                   | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::A2AMessage::agent_text`                      | `method`       | `echo_agent::a2a::types::A2AMessage::agent_text`                       | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::A2AMessage::parts`                           | `struct_field` | `echo_agent::a2a::types::A2AMessage::parts`                            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AMessage::role`                            | `struct_field` | `echo_agent::a2a::types::A2AMessage::role`                             | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AMessage::text_content`                    | `method`       | `echo_agent::a2a::types::A2AMessage::text_content`                     | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::A2AMessage::user_text`                       | `method`       | `echo_agent::a2a::types::A2AMessage::user_text`                        | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::A2APart`                                     | `enum`         | `echo_agent::a2a::types::A2APart`                                      | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2APart::File`                               | `variant`      | `echo_agent::a2a::types::A2APart::File`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2APart::File::data`                         | `struct_field` | `echo_agent::a2a::types::A2APart::File::data`                          | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2APart::File::mime_type`                    | `struct_field` | `echo_agent::a2a::types::A2APart::File::mime_type`                     | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2APart::Text`                               | `variant`      | `echo_agent::a2a::types::A2APart::Text`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2APart::Text::text`                         | `struct_field` | `echo_agent::a2a::types::A2APart::Text::text`                          | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamEvent`                              | `enum`         | `echo_agent::a2a::types::A2AStreamEvent`                               | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamEvent::ArtifactUpdate`              | `variant`      | `echo_agent::a2a::types::A2AStreamEvent::ArtifactUpdate`               | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamEvent::ArtifactUpdate::0`           | `struct_field` | `echo_agent::a2a::types::A2AStreamEvent::ArtifactUpdate::0`            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamEvent::StatusUpdate`                | `variant`      | `echo_agent::a2a::types::A2AStreamEvent::StatusUpdate`                 | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamEvent::StatusUpdate::0`             | `struct_field` | `echo_agent::a2a::types::A2AStreamEvent::StatusUpdate::0`              | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamResponse`                           | `struct`       | `echo_agent::a2a::types::A2AStreamResponse`                            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamResponse::error`                    | `struct_field` | `echo_agent::a2a::types::A2AStreamResponse::error`                     | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamResponse::id`                       | `struct_field` | `echo_agent::a2a::types::A2AStreamResponse::id`                        | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamResponse::jsonrpc`                  | `struct_field` | `echo_agent::a2a::types::A2AStreamResponse::jsonrpc`                   | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2AStreamResponse::result`                   | `struct_field` | `echo_agent::a2a::types::A2AStreamResponse::result`                    | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATask`                                     | `struct`       | `echo_agent::a2a::types::A2ATask`                                      | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATask::artifacts`                          | `struct_field` | `echo_agent::a2a::types::A2ATask::artifacts`                           | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATask::history`                            | `struct_field` | `echo_agent::a2a::types::A2ATask::history`                             | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATask::id`                                 | `struct_field` | `echo_agent::a2a::types::A2ATask::id`                                  | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATask::session_id`                         | `struct_field` | `echo_agent::a2a::types::A2ATask::session_id`                          | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATask::status`                             | `struct_field` | `echo_agent::a2a::types::A2ATask::status`                              | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskParams`                               | `struct`       | `echo_agent::a2a::types::A2ATaskParams`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskParams::id`                           | `struct_field` | `echo_agent::a2a::types::A2ATaskParams::id`                            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskParams::message`                      | `struct_field` | `echo_agent::a2a::types::A2ATaskParams::message`                       | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskParams::session_id`                   | `struct_field` | `echo_agent::a2a::types::A2ATaskParams::session_id`                    | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskRequest`                              | `struct`       | `echo_agent::a2a::types::A2ATaskRequest`                               | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskRequest::id`                          | `struct_field` | `echo_agent::a2a::types::A2ATaskRequest::id`                           | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskRequest::jsonrpc`                     | `struct_field` | `echo_agent::a2a::types::A2ATaskRequest::jsonrpc`                      | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskRequest::method`                      | `struct_field` | `echo_agent::a2a::types::A2ATaskRequest::method`                       | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskRequest::params`                      | `struct_field` | `echo_agent::a2a::types::A2ATaskRequest::params`                       | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskResponse`                             | `struct`       | `echo_agent::a2a::types::A2ATaskResponse`                              | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskResponse::error`                      | `struct_field` | `echo_agent::a2a::types::A2ATaskResponse::error`                       | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskResponse::id`                         | `struct_field` | `echo_agent::a2a::types::A2ATaskResponse::id`                          | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskResponse::jsonrpc`                    | `struct_field` | `echo_agent::a2a::types::A2ATaskResponse::jsonrpc`                     | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskResponse::result`                     | `struct_field` | `echo_agent::a2a::types::A2ATaskResponse::result`                      | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskStatus`                               | `struct`       | `echo_agent::a2a::types::A2ATaskStatus`                                | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::A2ATaskStatus::message`                      | `struct_field` | `echo_agent::a2a::types::A2ATaskStatus::message`                       | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskStatus::new`                          | `method`       | `echo_agent::a2a::types::A2ATaskStatus::new`                           | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::A2ATaskStatus::state`                        | `struct_field` | `echo_agent::a2a::types::A2ATaskStatus::state`                         | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskStatus::timestamp`                    | `struct_field` | `echo_agent::a2a::types::A2ATaskStatus::timestamp`                     | `value:a2a`                                                                                                      |
| `echo_agent::a2a::A2ATaskStatus::with_message`                 | `method`       | `echo_agent::a2a::types::A2ATaskStatus::with_message`                  | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentAuthentication`                         | `struct`       | `echo_agent::a2a::types::AgentAuthentication`                          | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentAuthentication::schemes`                | `struct_field` | `echo_agent::a2a::types::AgentAuthentication::schemes`                 | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCapabilities`                           | `struct`       | `echo_agent::a2a::types::AgentCapabilities`                            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCapabilities::push_notifications`       | `struct_field` | `echo_agent::a2a::types::AgentCapabilities::push_notifications`        | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCapabilities::state_transition_history` | `struct_field` | `echo_agent::a2a::types::AgentCapabilities::state_transition_history`  | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCapabilities::streaming`                | `struct_field` | `echo_agent::a2a::types::AgentCapabilities::streaming`                 | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard`                                   | `struct`       | `echo_agent::a2a::types::AgentCard`                                    | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentCard::authentication`                   | `struct_field` | `echo_agent::a2a::types::AgentCard::authentication`                    | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::builder`                          | `method`       | `echo_agent::a2a::types::AgentCard::builder`                           | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentCard::capabilities`                     | `struct_field` | `echo_agent::a2a::types::AgentCard::capabilities`                      | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::default_input_modes`              | `struct_field` | `echo_agent::a2a::types::AgentCard::default_input_modes`               | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::default_output_modes`             | `struct_field` | `echo_agent::a2a::types::AgentCard::default_output_modes`              | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::description`                      | `struct_field` | `echo_agent::a2a::types::AgentCard::description`                       | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::name`                             | `struct_field` | `echo_agent::a2a::types::AgentCard::name`                              | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::provider`                         | `struct_field` | `echo_agent::a2a::types::AgentCard::provider`                          | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::skills`                           | `struct_field` | `echo_agent::a2a::types::AgentCard::skills`                            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::url`                              | `struct_field` | `echo_agent::a2a::types::AgentCard::url`                               | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCard::version`                          | `struct_field` | `echo_agent::a2a::types::AgentCard::version`                           | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentCardBuilder`                            | `struct`       | `echo_agent::a2a::types::AgentCardBuilder`                             | `intrinsic:builder-or-factory`                                                                                   |
| `echo_agent::a2a::AgentCardBuilder::authentication`            | `method`       | `echo_agent::a2a::types::AgentCardBuilder::authentication`             | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::build`                     | `method`       | `echo_agent::a2a::types::AgentCardBuilder::build`                      | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::description`               | `method`       | `echo_agent::a2a::types::AgentCardBuilder::description`                | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::input_modes`               | `method`       | `echo_agent::a2a::types::AgentCardBuilder::input_modes`                | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::output_modes`              | `method`       | `echo_agent::a2a::types::AgentCardBuilder::output_modes`               | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::provider`                  | `method`       | `echo_agent::a2a::types::AgentCardBuilder::provider`                   | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::push_notifications`        | `method`       | `echo_agent::a2a::types::AgentCardBuilder::push_notifications`         | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::skill`                     | `method`       | `echo_agent::a2a::types::AgentCardBuilder::skill`                      | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::skills`                    | `method`       | `echo_agent::a2a::types::AgentCardBuilder::skills`                     | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::streaming`                 | `method`       | `echo_agent::a2a::types::AgentCardBuilder::streaming`                  | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentCardBuilder::version`                   | `method`       | `echo_agent::a2a::types::AgentCardBuilder::version`                    | `intrinsic:language-local-authority-construction`                                                                |
| `echo_agent::a2a::AgentProvider`                               | `struct`       | `echo_agent::a2a::types::AgentProvider`                                | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentProvider::new`                          | `method`       | `echo_agent::a2a::types::AgentProvider::new`                           | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentProvider::organization`                 | `struct_field` | `echo_agent::a2a::types::AgentProvider::organization`                  | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentProvider::url`                          | `struct_field` | `echo_agent::a2a::types::AgentProvider::url`                           | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentProvider::with_url`                     | `method`       | `echo_agent::a2a::types::AgentProvider::with_url`                      | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentSkill`                                  | `struct`       | `echo_agent::a2a::types::AgentSkill`                                   | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentSkill::description`                     | `struct_field` | `echo_agent::a2a::types::AgentSkill::description`                      | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentSkill::examples`                        | `struct_field` | `echo_agent::a2a::types::AgentSkill::examples`                         | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentSkill::id`                              | `struct_field` | `echo_agent::a2a::types::AgentSkill::id`                               | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentSkill::input_modes`                     | `struct_field` | `echo_agent::a2a::types::AgentSkill::input_modes`                      | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentSkill::name`                            | `struct_field` | `echo_agent::a2a::types::AgentSkill::name`                             | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentSkill::new`                             | `method`       | `echo_agent::a2a::types::AgentSkill::new`                              | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentSkill::output_modes`                    | `struct_field` | `echo_agent::a2a::types::AgentSkill::output_modes`                     | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentSkill::tags`                            | `struct_field` | `echo_agent::a2a::types::AgentSkill::tags`                             | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AgentSkill::with_examples`                   | `method`       | `echo_agent::a2a::types::AgentSkill::with_examples`                    | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AgentSkill::with_tags`                       | `method`       | `echo_agent::a2a::types::AgentSkill::with_tags`                        | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::AuthenticationScheme`                        | `struct`       | `echo_agent::a2a::types::AuthenticationScheme`                         | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AuthenticationScheme::config`                | `struct_field` | `echo_agent::a2a::types::AuthenticationScheme::config`                 | `value:a2a`                                                                                                      |
| `echo_agent::a2a::AuthenticationScheme::scheme`                | `struct_field` | `echo_agent::a2a::types::AuthenticationScheme::scheme`                 | `value:a2a`                                                                                                      |
| `echo_agent::a2a::ERROR_CODE_INVALID_PARAMS`                   | `constant`     | `echo_agent::a2a::types::ERROR_CODE_INVALID_PARAMS`                    | `value:a2a`                                                                                                      |
| `echo_agent::a2a::ERROR_CODE_INVALID_TRANSITION`               | `constant`     | `echo_agent::a2a::types::ERROR_CODE_INVALID_TRANSITION`                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::ERROR_CODE_METHOD_NOT_FOUND`                 | `constant`     | `echo_agent::a2a::types::ERROR_CODE_METHOD_NOT_FOUND`                  | `value:a2a`                                                                                                      |
| `echo_agent::a2a::ERROR_CODE_PARSE`                            | `constant`     | `echo_agent::a2a::types::ERROR_CODE_PARSE`                             | `value:a2a`                                                                                                      |
| `echo_agent::a2a::ERROR_CODE_TASK_FAILED`                      | `constant`     | `echo_agent::a2a::types::ERROR_CODE_TASK_FAILED`                       | `value:a2a`                                                                                                      |
| `echo_agent::a2a::ERROR_CODE_TASK_NOT_FOUND`                   | `constant`     | `echo_agent::a2a::types::ERROR_CODE_TASK_NOT_FOUND`                    | `value:a2a`                                                                                                      |
| `echo_agent::a2a::ERROR_CODE_TERMINAL_STATE`                   | `constant`     | `echo_agent::a2a::types::ERROR_CODE_TERMINAL_STATE`                    | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JSONRPC_VERSION`                             | `constant`     | `echo_agent::a2a::types::JSONRPC_VERSION`                              | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::aud`                              | `struct_field` | `echo_agent::a2a::auth::JwtClaims::aud`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::exp`                              | `struct_field` | `echo_agent::a2a::auth::JwtClaims::exp`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::extra`                            | `struct_field` | `echo_agent::a2a::auth::JwtClaims::extra`                              | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::iat`                              | `struct_field` | `echo_agent::a2a::auth::JwtClaims::iat`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::iss`                              | `struct_field` | `echo_agent::a2a::auth::JwtClaims::iss`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::jti`                              | `struct_field` | `echo_agent::a2a::auth::JwtClaims::jti`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::nbf`                              | `struct_field` | `echo_agent::a2a::auth::JwtClaims::nbf`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::sub`                              | `struct_field` | `echo_agent::a2a::auth::JwtClaims::sub`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::JwtClaims::subject`                          | `method`       | `echo_agent::a2a::auth::JwtClaims::subject`                            | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::JwtConfig`                                   | `struct`       | `echo_agent::a2a::auth::JwtConfig`                                     | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::JwtConfig::disabled`                         | `method`       | `echo_agent::a2a::auth::JwtConfig::disabled`                           | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::JwtConfig::hs256`                            | `method`       | `echo_agent::a2a::auth::JwtConfig::hs256`                              | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::JwtConfig::impl<Debug>`                      | `trait_impl`   | `echo_agent::a2a::auth::JwtConfig::impl<Debug>`                        | `intrinsic:rust-trait-implementation`                                                                            |
| `echo_agent::a2a::JwtConfig::is_enabled`                       | `method`       | `echo_agent::a2a::auth::JwtConfig::is_enabled`                         | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::JwtConfig::rs256`                            | `method`       | `echo_agent::a2a::auth::JwtConfig::rs256`                              | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::JwtConfig::with_audience`                    | `method`       | `echo_agent::a2a::auth::JwtConfig::with_audience`                      | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::JwtConfig::with_issuer`                      | `method`       | `echo_agent::a2a::auth::JwtConfig::with_issuer`                        | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::JwtConfigError`                              | `struct`       | `echo_agent::a2a::auth::JwtConfigError`                                | `family:a2a`                                                                                                     |
| `echo_agent::a2a::METHOD_CANCEL`                               | `constant`     | `echo_agent::a2a::types::METHOD_CANCEL`                                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::METHOD_GET`                                  | `constant`     | `echo_agent::a2a::types::METHOD_GET`                                   | `value:a2a`                                                                                                      |
| `echo_agent::a2a::METHOD_SEND`                                 | `constant`     | `echo_agent::a2a::types::METHOD_SEND`                                  | `value:a2a`                                                                                                      |
| `echo_agent::a2a::METHOD_SEND_SUBSCRIBE`                       | `constant`     | `echo_agent::a2a::types::METHOD_SEND_SUBSCRIBE`                        | `value:a2a`                                                                                                      |
| `echo_agent::a2a::TaskArtifactUpdateEvent`                     | `struct`       | `echo_agent::a2a::types::TaskArtifactUpdateEvent`                      | `value:a2a`                                                                                                      |
| `echo_agent::a2a::TaskArtifactUpdateEvent::artifact`           | `struct_field` | `echo_agent::a2a::types::TaskArtifactUpdateEvent::artifact`            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::TaskArtifactUpdateEvent::is_final`           | `struct_field` | `echo_agent::a2a::types::TaskArtifactUpdateEvent::is_final`            | `value:a2a`                                                                                                      |
| `echo_agent::a2a::TaskArtifactUpdateEvent::task_id`            | `struct_field` | `echo_agent::a2a::types::TaskArtifactUpdateEvent::task_id`             | `value:a2a`                                                                                                      |
| `echo_agent::a2a::TaskState`                                   | `enum`         | `echo_agent::a2a::types::TaskState`                                    | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskState::Canceled`                         | `variant`      | `echo_agent::a2a::types::TaskState::Canceled`                          | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskState::Completed`                        | `variant`      | `echo_agent::a2a::types::TaskState::Completed`                         | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskState::Failed`                           | `variant`      | `echo_agent::a2a::types::TaskState::Failed`                            | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskState::InputRequired`                    | `variant`      | `echo_agent::a2a::types::TaskState::InputRequired`                     | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskState::Submitted`                        | `variant`      | `echo_agent::a2a::types::TaskState::Submitted`                         | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskState::Working`                          | `variant`      | `echo_agent::a2a::types::TaskState::Working`                           | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskState::can_transition_to`                | `method`       | `echo_agent::a2a::types::TaskState::can_transition_to`                 | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskState::impl<Display>`                    | `trait_impl`   | `echo_agent::a2a::types::TaskState::impl<Display>`                     | `intrinsic:rust-trait-implementation`                                                                            |
| `echo_agent::a2a::TaskState::is_terminal`                      | `method`       | `echo_agent::a2a::types::TaskState::is_terminal`                       | `intrinsic:language-local-value-method`                                                                          |
| `echo_agent::a2a::TaskStatusUpdateEvent`                       | `struct`       | `echo_agent::a2a::types::TaskStatusUpdateEvent`                        | `value:a2a`                                                                                                      |
| `echo_agent::a2a::TaskStatusUpdateEvent::is_final`             | `struct_field` | `echo_agent::a2a::types::TaskStatusUpdateEvent::is_final`              | `value:a2a`                                                                                                      |
| `echo_agent::a2a::TaskStatusUpdateEvent::status`               | `struct_field` | `echo_agent::a2a::types::TaskStatusUpdateEvent::status`                | `value:a2a`                                                                                                      |
| `echo_agent::a2a::TaskStatusUpdateEvent::task_id`              | `struct_field` | `echo_agent::a2a::types::TaskStatusUpdateEvent::task_id`               | `value:a2a`                                                                                                      |

### 9 accepted catalog routes

| Route                                                                                                            | Handler operation      | Required feature | Items |
| ---------------------------------------------------------------------------------------------------------------- | ---------------------- | ---------------- | ----: |
| `family-operation:a2a:a2a.discover:echo_agent::a2a::client::A2AClient::discover`                                 | `a2a.discover`         | `a2a`            |     1 |
| `family-operation:a2a:a2a.task.cancel:echo_agent::a2a::client::A2AClient::cancel_task`                           | `a2a.task.cancel`      | `a2a`            |     1 |
| `family-operation:a2a:a2a.task.get:echo_agent::a2a::client::A2AClient::get_task`                                 | `a2a.task.get`         | `a2a`            |     1 |
| `family-operation:a2a:a2a.task.send:echo_agent::a2a::client::A2AClient::send_task`                               | `a2a.task.send`        | `a2a`            |     1 |
| `family-operation:a2a:a2a.task.send:echo_agent::a2a::client::A2AClient::send_task_with_session`                  | `a2a.task.send`        | `a2a`            |     1 |
| `family-operation:a2a:a2a.task.stream.open:echo_agent::a2a::client::A2AClient::send_task_streaming`              | `a2a.task.stream.open` | `a2a`            |     1 |
| `family-operation:a2a:a2a.task.stream.open:echo_agent::a2a::client::A2AClient::send_task_streaming_with_session` | `a2a.task.stream.open` | `a2a`            |     1 |
| `family:a2a`                                                                                                     | `None`                 | `a2a`            |     1 |
| `value:a2a`                                                                                                      | `None`                 | `a2a`            |   105 |

### 9 closed family operations

- `a2a.client.open`
- `a2a.discover`
- `a2a.task.send`
- `a2a.task.get`
- `a2a.task.cancel`
- `a2a.task.stream.open`
- `a2a.stream.next`
- `a2a.stream.cancel`
- `a2a.stream.close`

### Language mappings

- typescript: `{"generated_or_lossless_value": 117, "idiomatic_method": 36, "native_async_stream": 2, "native_language_construct": 3, "opaque_lifecycle_handle": 2}`; contract groups: `sdk-parity/typescript/a2a_agent_card`, `sdk-parity/typescript/a2a_stream_values`, `sdk-parity/typescript/a2a_task_envelopes`, `sdk-parity/typescript/a2a_task_state`, `sdk-parity/typescript/a2a_values`, `sdk-parity/typescript/a2a_wire_values`, `sdk-parity/typescript/facade_handle`, `sdk-parity/typescript/facade_operation`, `sdk-parity/typescript/facade_stream`, `sdk-parity/typescript/jwt_values`, `sdk-parity/typescript/wire_value`
- python: `{"generated_or_lossless_value": 117, "idiomatic_method": 36, "native_async_stream": 2, "native_language_construct": 3, "opaque_lifecycle_handle": 2}`; contract groups: `sdk-parity/python/a2a_agent_card`, `sdk-parity/python/a2a_stream_values`, `sdk-parity/python/a2a_task_envelopes`, `sdk-parity/python/a2a_task_state`, `sdk-parity/python/a2a_values`, `sdk-parity/python/a2a_wire_values`, `sdk-parity/python/facade_handle`, `sdk-parity/python/facade_operation`, `sdk-parity/python/facade_stream`, `sdk-parity/python/jwt_values`, `sdk-parity/python/wire_value`
- java: `{"generated_or_lossless_value": 117, "idiomatic_method": 36, "native_async_stream": 2, "native_language_construct": 3, "opaque_lifecycle_handle": 2}`; contract groups: `sdk-parity/java/a2a_agent_card`, `sdk-parity/java/a2a_stream_values`, `sdk-parity/java/a2a_task_envelopes`, `sdk-parity/java/a2a_task_state`, `sdk-parity/java/a2a_values`, `sdk-parity/java/a2a_wire_values`, `sdk-parity/java/facade_handle`, `sdk-parity/java/facade_operation`, `sdk-parity/java/facade_stream`, `sdk-parity/java/jwt_values`, `sdk-parity/java/wire_value`

## LSP — exact accepted identity and kind set

Count: 106. Kind distribution: `{"enum": 4, "method": 12, "struct": 11, "struct_field": 40, "type_alias": 1, "variant": 38}`.

Feature: `lsp`; Host passthrough: `framework-lsp`; family method: `_echo_agent/lsp/op`.

| Canonical path                                             | Kind           | Formal source_paths                                                | Route                                                                                                                  |
| ---------------------------------------------------------- | -------------- | ------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------- |
| `echo_agent::lsp::CompletionItem`                          | `struct`       | `echo_core::lsp::types::CompletionItem`                            | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItem::detail`                  | `struct_field` | `echo_core::lsp::types::CompletionItem::detail`                    | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItem::documentation`           | `struct_field` | `echo_core::lsp::types::CompletionItem::documentation`             | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItem::insert_text`             | `struct_field` | `echo_core::lsp::types::CompletionItem::insert_text`               | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItem::kind`                    | `struct_field` | `echo_core::lsp::types::CompletionItem::kind`                      | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItem::label`                   | `struct_field` | `echo_core::lsp::types::CompletionItem::label`                     | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind`                      | `enum`         | `echo_core::lsp::types::CompletionItemKind`                        | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Class`               | `variant`      | `echo_core::lsp::types::CompletionItemKind::Class`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Color`               | `variant`      | `echo_core::lsp::types::CompletionItemKind::Color`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Constant`            | `variant`      | `echo_core::lsp::types::CompletionItemKind::Constant`              | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Constructor`         | `variant`      | `echo_core::lsp::types::CompletionItemKind::Constructor`           | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Enum`                | `variant`      | `echo_core::lsp::types::CompletionItemKind::Enum`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::EnumMember`          | `variant`      | `echo_core::lsp::types::CompletionItemKind::EnumMember`            | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Event`               | `variant`      | `echo_core::lsp::types::CompletionItemKind::Event`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Field`               | `variant`      | `echo_core::lsp::types::CompletionItemKind::Field`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::File`                | `variant`      | `echo_core::lsp::types::CompletionItemKind::File`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Folder`              | `variant`      | `echo_core::lsp::types::CompletionItemKind::Folder`                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Function`            | `variant`      | `echo_core::lsp::types::CompletionItemKind::Function`              | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Interface`           | `variant`      | `echo_core::lsp::types::CompletionItemKind::Interface`             | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Keyword`             | `variant`      | `echo_core::lsp::types::CompletionItemKind::Keyword`               | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Method`              | `variant`      | `echo_core::lsp::types::CompletionItemKind::Method`                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Module`              | `variant`      | `echo_core::lsp::types::CompletionItemKind::Module`                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Operator`            | `variant`      | `echo_core::lsp::types::CompletionItemKind::Operator`              | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Property`            | `variant`      | `echo_core::lsp::types::CompletionItemKind::Property`              | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Reference`           | `variant`      | `echo_core::lsp::types::CompletionItemKind::Reference`             | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Snippet`             | `variant`      | `echo_core::lsp::types::CompletionItemKind::Snippet`               | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Struct`              | `variant`      | `echo_core::lsp::types::CompletionItemKind::Struct`                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Text`                | `variant`      | `echo_core::lsp::types::CompletionItemKind::Text`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::TypeParameter`       | `variant`      | `echo_core::lsp::types::CompletionItemKind::TypeParameter`         | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Unit`                | `variant`      | `echo_core::lsp::types::CompletionItemKind::Unit`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Value`               | `variant`      | `echo_core::lsp::types::CompletionItemKind::Value`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::CompletionItemKind::Variable`            | `variant`      | `echo_core::lsp::types::CompletionItemKind::Variable`              | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Diagnostic`                              | `struct`       | `echo_core::lsp::types::Diagnostic`                                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Diagnostic::code`                        | `struct_field` | `echo_core::lsp::types::Diagnostic::code`                          | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Diagnostic::message`                     | `struct_field` | `echo_core::lsp::types::Diagnostic::message`                       | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Diagnostic::range`                       | `struct_field` | `echo_core::lsp::types::Diagnostic::range`                         | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Diagnostic::severity`                    | `struct_field` | `echo_core::lsp::types::Diagnostic::severity`                      | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Diagnostic::source`                      | `struct_field` | `echo_core::lsp::types::Diagnostic::source`                        | `value:lsp`                                                                                                            |
| `echo_agent::lsp::DiagnosticCode`                          | `enum`         | `echo_core::lsp::types::DiagnosticCode`                            | `value:lsp`                                                                                                            |
| `echo_agent::lsp::DiagnosticCode::Integer`                 | `variant`      | `echo_core::lsp::types::DiagnosticCode::Integer`                   | `value:lsp`                                                                                                            |
| `echo_agent::lsp::DiagnosticCode::String`                  | `variant`      | `echo_core::lsp::types::DiagnosticCode::String`                    | `value:lsp`                                                                                                            |
| `echo_agent::lsp::DiagnosticSeverity`                      | `enum`         | `echo_core::lsp::types::DiagnosticSeverity`                        | `value:lsp`                                                                                                            |
| `echo_agent::lsp::DiagnosticSeverity::Error`               | `variant`      | `echo_core::lsp::types::DiagnosticSeverity::Error`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::DiagnosticSeverity::Hint`                | `variant`      | `echo_core::lsp::types::DiagnosticSeverity::Hint`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::DiagnosticSeverity::Information`         | `variant`      | `echo_core::lsp::types::DiagnosticSeverity::Information`           | `value:lsp`                                                                                                            |
| `echo_agent::lsp::DiagnosticSeverity::Warning`             | `variant`      | `echo_core::lsp::types::DiagnosticSeverity::Warning`               | `value:lsp`                                                                                                            |
| `echo_agent::lsp::HoverInfo`                               | `struct`       | `echo_core::lsp::types::HoverInfo`                                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::HoverInfo::contents`                     | `struct_field` | `echo_core::lsp::types::HoverInfo::contents`                       | `value:lsp`                                                                                                            |
| `echo_agent::lsp::HoverInfo::range`                        | `struct_field` | `echo_core::lsp::types::HoverInfo::range`                          | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Location`                                | `struct`       | `echo_core::lsp::types::Location`                                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Location::range`                         | `struct_field` | `echo_core::lsp::types::Location::range`                           | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Location::uri`                           | `struct_field` | `echo_core::lsp::types::Location::uri`                             | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspConfig::servers`                      | `struct_field` | `echo_integration::lsp::config::LspConfig::servers`                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspConfigFile`                           | `struct`       | `echo_core::lsp::types::LspConfigFile`                             | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspConfigFile::languages`                | `struct_field` | `echo_core::lsp::types::LspConfigFile::languages`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError`                                | `enum`         | `echo_core::lsp::client::LspError`                                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::CommunicationError`            | `variant`      | `echo_core::lsp::client::LspError::CommunicationError`             | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::CommunicationError::0`         | `struct_field` | `echo_core::lsp::client::LspError::CommunicationError::0`          | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::InvalidUri`                    | `variant`      | `echo_core::lsp::client::LspError::InvalidUri`                     | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::InvalidUri::0`                 | `struct_field` | `echo_core::lsp::client::LspError::InvalidUri::0`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::NotInitialized`                | `variant`      | `echo_core::lsp::client::LspError::NotInitialized`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::NotRunning`                    | `variant`      | `echo_core::lsp::client::LspError::NotRunning`                     | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::NotRunning::0`                 | `struct_field` | `echo_core::lsp::client::LspError::NotRunning::0`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::ServerError`                   | `variant`      | `echo_core::lsp::client::LspError::ServerError`                    | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::ServerError::0`                | `struct_field` | `echo_core::lsp::client::LspError::ServerError::0`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::SpawnError`                    | `variant`      | `echo_core::lsp::client::LspError::SpawnError`                     | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::SpawnError::0`                 | `struct_field` | `echo_core::lsp::client::LspError::SpawnError::0`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspError::Timeout`                       | `variant`      | `echo_core::lsp::client::LspError::Timeout`                        | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspManager`                              | `struct`       | `echo_integration::lsp::manager::LspManager`                       | `family:lsp`                                                                                                           |
| `echo_agent::lsp::LspManager::configured_languages`        | `method`       | `echo_integration::lsp::manager::LspManager::configured_languages` | `family-operation:lsp:lsp.language.configured:echo_integration::lsp::manager::LspManager::configured_languages`        |
| `echo_agent::lsp::LspManager::get_client`                  | `method`       | `echo_integration::lsp::manager::LspManager::get_client`           | `family-operation:lsp:lsp.manager.get_client:echo_integration::lsp::manager::LspManager::get_client`                   |
| `echo_agent::lsp::LspManager::get_client_for_file`         | `method`       | `echo_integration::lsp::manager::LspManager::get_client_for_file`  | `family-operation:lsp:lsp.manager.get_client_for_file:echo_integration::lsp::manager::LspManager::get_client_for_file` |
| `echo_agent::lsp::LspManager::load_config`                 | `method`       | `echo_integration::lsp::manager::LspManager::load_config`          | `family-operation:lsp:lsp.manager.load_config:echo_integration::lsp::manager::LspManager::load_config`                 |
| `echo_agent::lsp::LspManager::new`                         | `method`       | `echo_integration::lsp::manager::LspManager::new`                  | `family-operation:lsp:lsp.manager.open_exact:echo_integration::lsp::manager::LspManager::new`                          |
| `echo_agent::lsp::LspManager::restart_server`              | `method`       | `echo_integration::lsp::manager::LspManager::restart_server`       | `family-operation:lsp:lsp.manager.restart_server:echo_integration::lsp::manager::LspManager::restart_server`           |
| `echo_agent::lsp::LspManager::running_servers`             | `method`       | `echo_integration::lsp::manager::LspManager::running_servers`      | `family-operation:lsp:lsp.server.running:echo_integration::lsp::manager::LspManager::running_servers`                  |
| `echo_agent::lsp::LspManager::set_project_root`            | `method`       | `echo_integration::lsp::manager::LspManager::set_project_root`     | `family-operation:lsp:lsp.manager.set_project_root:echo_integration::lsp::manager::LspManager::set_project_root`       |
| `echo_agent::lsp::LspManager::shutdown_all`                | `method`       | `echo_integration::lsp::manager::LspManager::shutdown_all`         | `family-operation:lsp:lsp.manager.shutdown_all:echo_integration::lsp::manager::LspManager::shutdown_all`               |
| `echo_agent::lsp::LspManager::start_server`                | `method`       | `echo_integration::lsp::manager::LspManager::start_server`         | `family-operation:lsp:lsp.manager.start_server:echo_integration::lsp::manager::LspManager::start_server`               |
| `echo_agent::lsp::LspManager::status_all`                  | `method`       | `echo_integration::lsp::manager::LspManager::status_all`           | `family-operation:lsp:lsp.server.status_all:echo_integration::lsp::manager::LspManager::status_all`                    |
| `echo_agent::lsp::LspManager::stop_server`                 | `method`       | `echo_integration::lsp::manager::LspManager::stop_server`          | `family-operation:lsp:lsp.manager.stop_server:echo_integration::lsp::manager::LspManager::stop_server`                 |
| `echo_agent::lsp::LspResult`                               | `type_alias`   | `echo_core::lsp::client::LspResult`                                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerConfig`                         | `struct`       | `echo_core::lsp::types::LspServerConfig`                           | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerConfig::args`                   | `struct_field` | `echo_core::lsp::types::LspServerConfig::args`                     | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerConfig::command`                | `struct_field` | `echo_core::lsp::types::LspServerConfig::command`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerConfig::env`                    | `struct_field` | `echo_core::lsp::types::LspServerConfig::env`                      | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerConfig::extensions`             | `struct_field` | `echo_core::lsp::types::LspServerConfig::extensions`               | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerConfig::initialization_options` | `struct_field` | `echo_core::lsp::types::LspServerConfig::initialization_options`   | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerConfig::language`               | `struct_field` | `echo_core::lsp::types::LspServerConfig::language`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerConfig::max_restarts`           | `struct_field` | `echo_core::lsp::types::LspServerConfig::max_restarts`             | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerStatus`                         | `struct`       | `echo_core::lsp::types::LspServerStatus`                           | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerStatus::initialized`            | `struct_field` | `echo_core::lsp::types::LspServerStatus::initialized`              | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerStatus::language`               | `struct_field` | `echo_core::lsp::types::LspServerStatus::language`                 | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerStatus::last_error`             | `struct_field` | `echo_core::lsp::types::LspServerStatus::last_error`               | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerStatus::pid`                    | `struct_field` | `echo_core::lsp::types::LspServerStatus::pid`                      | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerStatus::restart_count`          | `struct_field` | `echo_core::lsp::types::LspServerStatus::restart_count`            | `value:lsp`                                                                                                            |
| `echo_agent::lsp::LspServerStatus::running`                | `struct_field` | `echo_core::lsp::types::LspServerStatus::running`                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Position`                                | `struct`       | `echo_core::lsp::types::Position`                                  | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Position::character`                     | `struct_field` | `echo_core::lsp::types::Position::character`                       | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Position::line`                          | `struct_field` | `echo_core::lsp::types::Position::line`                            | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Range`                                   | `struct`       | `echo_core::lsp::types::Range`                                     | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Range::end`                              | `struct_field` | `echo_core::lsp::types::Range::end`                                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::Range::start`                            | `struct_field` | `echo_core::lsp::types::Range::start`                              | `value:lsp`                                                                                                            |
| `echo_agent::lsp::TextChange`                              | `struct`       | `echo_core::lsp::types::TextChange`                                | `value:lsp`                                                                                                            |
| `echo_agent::lsp::TextChange::range`                       | `struct_field` | `echo_core::lsp::types::TextChange::range`                         | `value:lsp`                                                                                                            |
| `echo_agent::lsp::TextChange::text`                        | `struct_field` | `echo_core::lsp::types::TextChange::text`                          | `value:lsp`                                                                                                            |

### 14 accepted catalog routes

| Route                                                                                                                  | Handler operation                 | Required feature | Items |
| ---------------------------------------------------------------------------------------------------------------------- | --------------------------------- | ---------------- | ----: |
| `family-operation:lsp:lsp.language.configured:echo_integration::lsp::manager::LspManager::configured_languages`        | `lsp.language.configured`         | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.get_client:echo_integration::lsp::manager::LspManager::get_client`                   | `lsp.manager.get_client`          | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.get_client_for_file:echo_integration::lsp::manager::LspManager::get_client_for_file` | `lsp.manager.get_client_for_file` | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.load_config:echo_integration::lsp::manager::LspManager::load_config`                 | `lsp.manager.load_config`         | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.open_exact:echo_integration::lsp::manager::LspManager::new`                          | `lsp.manager.open_exact`          | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.restart_server:echo_integration::lsp::manager::LspManager::restart_server`           | `lsp.manager.restart_server`      | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.set_project_root:echo_integration::lsp::manager::LspManager::set_project_root`       | `lsp.manager.set_project_root`    | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.shutdown_all:echo_integration::lsp::manager::LspManager::shutdown_all`               | `lsp.manager.shutdown_all`        | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.start_server:echo_integration::lsp::manager::LspManager::start_server`               | `lsp.manager.start_server`        | `lsp`            |     1 |
| `family-operation:lsp:lsp.manager.stop_server:echo_integration::lsp::manager::LspManager::stop_server`                 | `lsp.manager.stop_server`         | `lsp`            |     1 |
| `family-operation:lsp:lsp.server.running:echo_integration::lsp::manager::LspManager::running_servers`                  | `lsp.server.running`              | `lsp`            |     1 |
| `family-operation:lsp:lsp.server.status_all:echo_integration::lsp::manager::LspManager::status_all`                    | `lsp.server.status_all`           | `lsp`            |     1 |
| `family:lsp`                                                                                                           | `None`                            | `lsp`            |     1 |
| `value:lsp`                                                                                                            | `None`                            | `lsp`            |    93 |

### 30 closed family operations

- `lsp.manager.open`
- `lsp.manager.open_exact`
- `lsp.manager.load_config`
- `lsp.manager.set_project_root`
- `lsp.manager.start_server`
- `lsp.manager.stop_server`
- `lsp.manager.restart_server`
- `lsp.manager.shutdown_all`
- `lsp.manager.get_client`
- `lsp.manager.get_client_for_file`
- `lsp.server.status`
- `lsp.language.list`
- `lsp.language.configured`
- `lsp.server.running`
- `lsp.server.status_all`
- `lsp.client.language`
- `lsp.client.is_running`
- `lsp.client.is_initialized`
- `lsp.client.initialize`
- `lsp.client.shutdown`
- `lsp.client.diagnostics`
- `lsp.client.goto_definition`
- `lsp.client.find_references`
- `lsp.client.hover`
- `lsp.client.completion`
- `lsp.client.did_open`
- `lsp.client.did_change`
- `lsp.client.did_save`
- `lsp.client.did_close`
- `lsp.client.status`

### Language mappings

- typescript: `{"generated_or_lossless_value": 93, "idiomatic_method": 12, "opaque_lifecycle_handle": 1}`; contract groups: `sdk-parity/typescript/facade_handle`, `sdk-parity/typescript/facade_operation`, `sdk-parity/typescript/wire_value`
- python: `{"generated_or_lossless_value": 93, "idiomatic_method": 12, "opaque_lifecycle_handle": 1}`; contract groups: `sdk-parity/python/facade_handle`, `sdk-parity/python/facade_operation`, `sdk-parity/python/wire_value`
- java: `{"generated_or_lossless_value": 93, "idiomatic_method": 12, "opaque_lifecycle_handle": 1}`; contract groups: `sdk-parity/java/facade_handle`, `sdk-parity/java/facade_operation`, `sdk-parity/java/wire_value`

## 正式采用删后源

原“No pin upgrade is needed”仅指SDK消费者撤出阶段。实际接入物理删除后的Framework必须
绑定真实删除提交、正式manifest/lock、同源Host及原合同门禁；受控path只提供其明确范围。
当前5352/431保留合同与269/23/39原撤销证明各自保持，不从完整inventory遥测推接受范围，
不以共享投影文案更新冒编译或正式Git资格。旧固定基线表及历史失败回执不重标新结果。


## 2026-10-31：Framework 340 组合的精确后继

原 Plan04 的固定 rollback 基线、266 项家族身份＋3 项具名补充、23 条路由、39 个操作和 ADR0042 的269项对账继续逐项检查。当前组合在该基线上仅增加已批准的11项具名退休与3个带原生调用ID的回调接入，最终 accepted 为5344、可执行路由为430。

13条保留记录按完整前后记录摘要精确承接，其余 retained 记录继续逐项相等；摘要不跳过签名、route、语言映射或其它字段。旧rollback到当前的7条保留route变化也逐条固定前后摘要；其中包含原Plan04已产生的value聚合变化，不能与formal3f到当前的6条变化混用。除原23条A2A/LSP路由外，只新增退出精确旧DELETE source route，家族和操作集合不扩张。

正式caller的完整生成检查、语言导出检查、Protocol无Framework依赖图、五组Protocol原测试保持。来源metadata在all-features下完整解析，八个Framework与五个vendor的名字、版本、source须组成精确13包HTTPS340集合。来源pin接续不等于依赖风险接受；SDK deny基线仍按其独立真实scanner与精组复核另行裁决，不自动续期。
