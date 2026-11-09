# echo-agent SDK (standard ACP Host available)

本轮原生消息候选与来源边界见[原生消息](native-messages.md)和[ADR0043](../adr/0043-native-message-sdk-adapter.md)；三语言合同随包携带，独立打包／加载方法见[源码包自包含](source-packages.md)。[Parquet 真实消费者验证](governance/parquet-consumer-validation.md)仍按具名旧 Host、旧合同快照单独取证，不代表新 pin、安全或整体交付通过。

## 当前来源与消费者退役边界

Plan04 消费者撤出阶段精确退役269项 accepted source/kind 身份、23条路由和39个
family 操作，当时保留5352项身份／431条路由，新增接受为零。原生成器消费266项
及三个具名 LSP plugin 身份；该历史证明见[ADR0042](../adr/0042-sdk-a2a-lsp-retirement-boundary.md)
和[精确处置](governance/a2a-lsp-retirement-boundary.md)。

当前 Host 通过 HTTPS 精确依赖 Framework main `3413d64dae23353c6d02c546ac374deded855827`
（tree `82a242196acd66a92cf0ad3a8708fa2e14e83117`），SDK 锁 SHA-256 为
`95e3caabaf5e72fc71ad1d6b79964eae1883b297f0006667e39c4c3a6eb8c5e5`，
共 654 包；13 个 Git 包来自同一精确来源。
当前范围仍为 5344 项 accepted 身份／430 条路由。[ADR0044](../adr/0044-sdk-source-disposition-native-callback.md)
记录原具名处置；[ADR0046](../adr/0046-cache-layout-source-identity-projection.md) 只再退役两个
`runtime_context` 字段来源、接受两个对应 `dynamic` 字段来源。保留行 `ContextProjection`
仅增加派生 `Default`，不接受 `Default::default` 身份，不新增公开语言字段或 RPC。

2026-11-09，本机完整生成／合同检查、Rust／Host 矩阵以及原完整语言入口均已按实际源码通过。
TypeScript 527 项通过，Python 601 项及 18 个子测试通过；Java 400 项测试零失败、零错误。
各语言保留 1 个原有 Windows 原生跳过项，三个 quickstart 通过，完整语言入口零警告。
本轮固定 c0bc strict Source／Final 尚未执行，精确 SDK main CI、平台原生及发布验收仍分别待验。
精确风险接受和原始回执见[当前验证状态](governance/framework-development-pin-status.md)，
不以历史 main 或上游 Framework CI 替代本 SDK 的资格。

Old source/digest/feature/family requests fail explicitly. Retired plugin
fields are absent, not empty placeholders; LspServer values are rejected.
ACP/MCP/Task/Subagent, Host stdio, streams/cancellation/recovery and R2 parent/
child close retain their original owners, tests and executable examples.
旧 `f7bac62e` pin 及下文历史5,621／454资格保持原快照范围，不作为当前来源、
消费者或发布验收。完整 Rust inventory 继续只是非阻断遥测。

This is the single external entry point for the echo-agent multilingual SDK
program. The SDK preserves the behavior of **explicitly accepted cross-language
contracts** in TypeScript, Python and Java, while the Rust framework owns
execution and state. Its complete Rust public inventory is drift telemetry;
it does not automatically create a three-language API commitment.

Capability governance tracks entrypoints, state authority, lifecycle, failure
paths and behavioral evidence in the [capability map](governance/capability-map.md),
[behavior matrix](governance/behavior-evidence-matrix.md) and
[Finding ledger](governance/finding-ledger.md). Historical build and catalog
gates do not close that audit; unresolved scopes remain explicit there.

> **Accepted SDK governance snapshot: [eb223986](https://github.com/EchoYue-lp/echo-agent-sdk/commit/eb22398676310fcbd67e9f4babe4afcefd56ed06).**
> Standard ACP, core extension profile, extension bridge and accepted facade
> adapters passed [nine original exact-main CI jobs](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39720432985) and a
> GitHub-only cold source check with 36 necessary refs. Current scope and
> retained risks are in [current acceptance](governance/capability-review-remaining.md). The source-built `echo-agent-sdk-host` passes
> initialize/new/prompt/update/cancel and shutdown scenarios through the
> official v1 Client, and the negotiated `_echo_agent/*` **core profile**
> (Agent/Session/Run handles, full events, ACK/replay, restart recovery) passes
> real-process E2E ([sdk-core-profile.md](sdk-core-profile.md)), the negotiated
> bidirectional **extension bridge** passes real-process E2E for Tool,
> LlmClient, Store, hooks and callbacks
> ([sdk-extension-bridge.md](sdk-extension-bridge.md)), and the **facade
> feature adapters** serve the task/subagent/structured-output, stateful,
> integration and tool families over the framework's own authorities
> ([facade-feature-adapters.md](facade-feature-adapters.md)). Plan 08 closes
> the Rust Host's canonical source-operation, consumer-trait and public-stream
> routing, including real Workflow pull streams. Source-built TypeScript,
> Python and Java clients resolve the accepted source/family operations
> through the shared catalog and preserve the declared WireValue shapes. The
> executable route baseline passed against its recorded source-built Host;
> current repair verification is separately bound. Manifest
> schema v2 separates the current external contract from Host/Rust-only,
> language-intrinsic, helper and deferred identities. See [Status
> ladder](#status-ladder) for the exact claims.

## What the SDK program is

- **Source-first delivery.** The repository ships source only. Developers
  compile everything (Host, SDK, tests) from the same Git revision. No
  precompiled binaries, npm packages, wheels or JARs are published, and the
  build never downloads prebuilt project artifacts.
- **ACP-first wire.** The stable [Agent Client Protocol
  v1](https://agentclientprotocol.com/protocol/overview) is the only base
  client↔agent protocol. This project never forks or re-declares the
  official JSON-RPC envelope, `initialize`, Session, Prompt, ContentBlock,
  updates or stop reasons — they come from the official
  [`agent-client-protocol`](https://crates.io/crates/agent-client-protocol)
  ecosystem artifacts pinned in
  [`contracts/sdk/acp-baseline.json`](../../contracts/sdk/acp-baseline.json).
- **Two profiles, one authority.** A standard ACP v1 client can use the
  Host's standard profile without knowing anything about echo-agent. The
  full echo-agent SDK profile negotiates `_echo_agent/*` extension methods
  (via the official `_meta` capability mechanism) for its explicitly accepted
  external capability surface. Both profiles project the same Rust execution
  and state authority — the wire never becomes a second framework.

Details of the two profiles, the extension namespace and the error/lossless
scalar rules live in [protocol.md](protocol.md). The delivered core profile —
negotiation, handles, events/ACK/replay and recovery semantics — is specified
in [sdk-core-profile.md](sdk-core-profile.md). The delivered facade family
routes, feature model, resource/stream lifecycle and error boundaries are
specified in [facade-feature-adapters.md](facade-feature-adapters.md).

The implemented Rust adapter and its current method/content boundary are
documented in [acp-agent-adapter.md](acp-agent-adapter.md).
Build, configuration and lifecycle instructions for the executable are in
[acp-standard-host.md](acp-standard-host.md).

Source SDK clients live under [`sdks/`](../../sdks/):

- [`sdks/typescript`](../../sdks/typescript) uses the official ACP TypeScript client;
- [`sdks/python`](../../sdks/python) uses the official ACP asyncio client;
- [`sdks/java`](../../sdks/java) provides the Java 17 CompletionStage/Flow client.

Each directory is source-only and requires the caller to provide an absolute
Host executable and configuration path.
Runtime compatibility is recorded in [`sdks/shared/toolchain.json`](../../sdks/shared/toolchain.json):
Node.js 20+, Python 3.10+, and JDK 17. The SDKs never install or bundle these
runtimes.

The cross-language source gate is `./scripts/check-language-sdks.sh`. It builds
the Host from the current checkout, validates every executable language mapping
in the parity manifest, checks all catalog operation identities and signatures,
compiles and runs the TypeScript/Python quickstarts plus Java example, runs the
TypeScript/Python/Java suites, and exercises Agent/Session, canonical family
operations and facade invoke against that real Host. Intrinsic mappings remain
explicit until their language-native behavior is delivered.

## Contract artifacts

| Artifact                                                   | Purpose                                                                                                                                                                          |
| ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `contracts/sdk/acp-baseline.json`                          | Pinned official ACP wire version (1), crate and schema artifact versions; tests assert the lockfile matches.                                                                     |
| `contracts/sdk/toolchain.json`                             | The exact nightly toolchain used for rustdoc-JSON inventory generation. Contributors only; normal builds never need it.                                                          |
| `contracts/sdk/public-api.txt`                             | Complete deterministic root-facade snapshot used for Rust inventory telemetry; it is not a runtime compatibility gate.                                                           |
| `contracts/sdk/parity-manifest.schema.json`                | Machine schema for the complete inventory telemetry manifest.                                                                                                                    |
| `contracts/sdk/parity-manifest.json`                       | Complete facade inventory classified by consumer-facing `sdk_scope`, semantic rule, ACP relationship, feature condition, adapter operation and per-language mapping/test status. |
| `contracts/sdk/accepted-external-contract.json`            | Generated blocking contract containing only canonical identities explicitly classified as `external_contract`; aliases remain in telemetry and inherit their canonical scope.    |
| `contracts/sdk/accepted-external-contract.schema.json`     | Machine schema for the blocking external contract artifact.                                                                                                                      |
| `contracts/sdk/accepted-facade-operation-catalog.json`     | Language-facing catalog filtered to accepted non-intrinsic routes.                                                                                                               |
| `contracts/sdk/inventory-telemetry.json`                   | Compact non-blocking report with framework provenance, full inventory digest and scope counts.                                                                                   |
| `contracts/sdk/schema/echo-agent-extension-v1.schema.json` | Generated JSON Schema of the `_echo_agent/*` extension DTOs and method catalog.                                                                                                  |
| `contracts/sdk/fixtures/extension/v1/`                     | Golden fixtures: valid samples must round-trip losslessly, invalid samples must be rejected deterministically.                                                                   |
| `contracts/sdk/source-contract.json`                       | Small generated digest over the accepted external contract and accepted facade catalog, embedded by the Host and matched by the Client hello.                                    |

The generating code lives in the workspace member crate
[`echo-sdk-protocol`](../../echo-sdk-protocol/) (`publish = false`). All
artifacts are machine-generated; regeneration is deterministic and
byte-stable:

```bash
# regenerate after an intentional facade or contract change
cargo run -p echo-sdk-protocol --bin export_schema --locked -- --update

# read-only drift check (also run by SDK contract CI)
./scripts/check-sdk-contracts.sh
```

During a framework adapter upgrade, the same generator can prepare a wire
candidate without rerunning the upstream rustdoc inventory:

```bash
cargo run -p echo-sdk-protocol --bin export_schema --locked -- --wire-candidate-update
./scripts/export-language-sdk-catalog.sh
```

This candidate combines the committed inventory and accepted identity scope
with current wire definitions. It refreshes schema, fixtures, catalogs and
their compatibility digest, and records its input hashes. It does not refresh
framework provenance or accept new Rust identities. A release or completed
governance outcome still requires full generation against the final published
framework revision and the read-only contract gate above.

## Status ladder

The SDK program separates implementation milestones, snapshot-bound evidence
and final accepted-contract delivery. The entries below distinguish historical
source/seed results from current governance acceptance; catalog mapping counts
alone are not behavioral evidence.

| Status                           | Meaning                                                                                                                     | Reached                                                                                                                                                                                                        |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Design**                       | The design document is agreed                                                                                               | ✅                                                                                                                                                                                                             |
| **Contract**                     | Accepted source/catalog continuity and blocking artifacts match the accepted contract                                       | Full 28-profile read-only check passed: 113 blocking artifacts current; four non-blocking Rust inventory drift artifacts retained without a 117-artifact update                                                |
| **ACP conformant**               | A standard ACP v1 client passes the supported profile against a real source-built Host                                      | ✅                                                                                                                                                                                                             |
| **Core extension profile**       | The negotiated `_echo_agent/*` core families run against a real Host with typed lifecycle, events, replay and recovery      | ✅ (Rust Host only)                                                                                                                                                                                            |
| **Host facade routing**          | Accepted source operations/consumer traits/streams have a concrete Host route or an explicit local boundary                 | Accepted capability/failure obligations settled at eb223986; historical Plan 08 evidence keeps its original execution cut                                                                                      |
| **Runnable**                     | A real Host plus each language's declared external SDK path executes end-to-end                                             | Original full three-language entry and three quickstarts passed at eb223986; Linux Temurin 17 and three named native Windows consumers have separate actual evidence                                           |
| **External contract complete**   | Accepted capability behavior, lifecycle and failures have contract, matched-source evidence and explicit residual decisions | Historical pre-withdrawal acceptance: 5,621 identities / 454 routes, 50 behaviors verified, 75 Findings resolved and one historical risk accepted by the human; 97 Audits remain bounded examined observations |
| **Deferred capability grouping** | Existing inventory is grouped without adding external acceptance                                                            | [Bounded Rust/Host/deferred groups](governance/deferred-capabilities.md); 当前生成清单有2,890项 deferred 身份，不自动接受。                                              |
| **Published**                    | Registry/binary publication — **explicitly out of scope**; this design ships source only                                    | never (by design)                                                                                                                                                                                              |

### Identity inventory scope

下段记录旧 pin 的上游接受切点，所有 SHA、CI 链接及锁统计均保留历史范围。

当时 source-built Host 采用已接受 framework `f7bac62eee04a963f92a3e4308b6ea2336e91cc0`
(tree `0acc8d94603b647e4d795c0b2eea20ecb0c3da7d`), from
[PR #197](https://github.com/EchoYue-lp/echo-agent/pull/197). Its original
[main CI run 39605663648](https://github.com/EchoYue-lp/echo-agent/actions/runs/39605663648)
completed all seven jobs, followed by an independent remote-only cold source
check. This upstream acceptance is not a passing SDK Host or consumer result.
The historical pin-switch SDK lock SHA-256 was `1c865a57b7e17587fba2f668d956832cd7b0f6c6167e198304c965227f3debaa`: only the eight framework and three
upstream vendored Git source identities change from the existing webpki-patched
SDK lock; all 656 package versions, checksums and dependency edges are unchanged.
A03 official Cargo update and locked offline metadata each exited 0 using frozen
existing user-mirror cache inputs, and produced this exact lock. The metadata used
`--no-deps` (`resolve=null`); this is source-only proof, not fresh registry, cold
cache, yanked-status or full graph validation. Subsequent SDK acceptance at
[eb223986](https://github.com/EchoYue-lp/echo-agent-sdk/commit/eb22398676310fcbd67e9f4babe4afcefd56ed06) passed [all nine original exact-main CI jobs](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39720432985)
and a GitHub-only cold source check using 36 necessary historical refs.
It records 50 verified behaviors, 75 resolved Findings and one human-accepted
historical risk; the four non-blocking Rust inventory drift artifacts remain
telemetry and were not regenerated.
At that historical qualification cut, membership remained 5,621 identity/kind keys and 454 routes. Pin/lock
provenance does not authorize a 117-artifact update, new public language APIs,
or automatic closure of the historical managed-exit cause.

下面先保留早期生成切点的来源记录；当前统计另列于表格。

The previously generated Host pin was published framework
`3560812caede84998a1f6f26d7bf3fa81ea0486c` from framework PR193.
The original full 28-profile/117-artifact generation and contract gate completed
against that pin. That historical manifest recorded 10,973 canonical identities,
5,621 accepted keys and 454 routes. 当前生成清单记录11,113项 canonical 身份、
5,344项 accepted keys／430条路由；下表按实际 `canonical: true` 和 `sdk_scope`
统计。See [source continuity](../../MIGRATION-SOURCE.md)
and [current acceptance](governance/capability-review-remaining.md) for exact cuts.
SDK208a/Host0bbc and the successor SDK9ef/Hosta17 retain distinct evidence cuts.
The successor original Rust14 and the one whole language entry plus all three
quickstarts now passed locally, with the media Finding concretely resolved.
Exact SDK-main/platform/nativeHook and the historical Linux cause remain separate.

An older extraction/wire-candidate note recorded 9,724 canonical identities,
5,622 language-done identities and4,102 remaining intrinsic-route identities.
Those are historical telemetry from that note, not the current committed
manifest or current accepted membership. The old a634
[Rust gate](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-a634-rust-gates/README.md) and
[language gate](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/README.md)
links preserve their historical input cuts and do not qualify the current Host.

These counts are SDK inventory telemetry, not repository-wide semantic progress
and not an instruction to open one pull request per identity. The complete
inventory is non-blocking drift telemetry; only the explicitly accepted
external contract and catalog are compatibility gates. Further language work is grouped by an
externally useful SDK capability or a confirmed Finding, while whole-workspace
governance is measured through Capability, Behavior, Rule, state authority,
lifecycle, Finding, and Evidence coverage. See
[ADR 0031](../adr/0031-sdk-identity-governance-scope.md).

Manifest schema v2 adds an identity-level `sdk_scope` that is independent from
the adapter route and per-language status:

| `sdk_scope`          | Canonical items | Current meaning                                                                                            |
| -------------------- | --------------: | ---------------------------------------------------------------------------------------------------------- |
| `external_contract`  |           5,344 | Frozen accepted canonical membership and language mapping; capability behavior requires separate evidence. |
| `host_or_rust_only`  |           1,951 | Process-local or Host-owned runtime authority.                                                             |
| `language_intrinsic` |             833 | Rust syntax, trait implementations and callback types represented idiomatically.                           |
| `internal_helper`    |              95 | Rust testing support, not an SDK product surface.                                                          |
| `deferred`           |           2,890 | Potential capabilities awaiting capability-level contract review.                                          |

The scope is generated, aliases inherit their canonical identity, and no row
is manually promoted. An intrinsic route becomes a blocking external contract
only through an explicit accepted disposition; route membership alone does not
promote it. A deferred item is not rejected, but it does not become thousands
of separate
tasks. See [ADR 0032](../adr/0032-sdk-contract-scope-classification.md).

Acceptance and completed evidence support claims only about the declared
external SDK surface. It does not justify claiming that every public Rust identity is
available in every language. Executable routes use the canonical resolver and
serializable values use the lossless WireValue algebra; Host/Rust-only,
language-intrinsic, helper and deferred identities remain explicit inventory
dispositions. The first intrinsic value slice (`ToolCallParams` and
`ToolResult`) is implemented and tested in all three languages; the remaining
deferred set is reviewed only in externally useful capability groups.
The local `ThinkingLevel` enum and parser preserve Rust reasoning-effort names
and aliases in all three language SDKs.
Steering lifecycle state and terminal outcome values are also available as
local immutable values; receipt and turn ownership remain Host-side.
Subagent command phases and terminal statuses are available as local values;
the Subagent dispatch and message receipt remain Host-owned.
Content-guard pass/detect/reject/redact decisions are available as local
immutable values; guard execution remains Host/Rust-owned.
Rust `GuardResult` is exposed as the language-native `GuardDecision` value;
Python's existing component `GuardResult` remains unchanged.
Delivery outcomes and phases are available as local values; the durable ledger
and delivery lifecycle remain framework-owned.
Subagent hook stop statuses are also local values; hook execution and dispatch
remain framework-owned.
Task terminal statuses are local values; PlanTask execution and task lifecycle
remain framework-owned.
Permission rule source values are local projections; rule evaluation and
priority authority remain in Rust/Host.
Permission rule behaviors are also local projections with decision conversion;
evaluation and RuleRegistry authority remain in Rust/Host.
PermissionMode 只接受精确的 `default`、`plan`、`auto-edit`、`full-auto`、`auto`、`bubble`、
`dont-ask`、`strict`；旧别名、大小写和首尾空白不再归一。八模式的 write/interaction/classifier
helper、默认值及 canonical wire 输出保持，权限求值仍归 Framework。见 [ADR0045](../adr/0045-sdk-permission-mode-canonical-inputs.md)。
Permission rule matchers expose pure parse/display/matching helpers; registry
and evaluation authority remain in Rust/Host.
Command-cell phases are local values; command process/sandbox execution remains
framework-owned.
Command-cell terminal causes and artifact statuses are local values; process
and artifact writer ownership remains framework-owned.
Team strategy values are local projections; Team dispatch and coordination remain
framework-owned.
ACP connection mode, extension settlement, and bounded ledger-limit values are
also available as native local projections; ACP connection/session and ledger
ownership remain Host/Rust-owned.
`AcpAdapterConfig` is available as a validated local configuration snapshot;
constructing or running the ACP adapter remains a Host/Rust responsibility.
Typed ACP lease errors preserve the Rust display text while lease admission and
concurrency decisions remain Host/Rust-owned.
Skill dependency kinds and skill sources are also local values; probing,
loading, and source policy remain framework/Host-owned.
Subagent context inheritance defaults are available as immutable local values;
message history, tools, memory stores, and dispatch remain framework-owned.
Observed isolation names preserve trim/default and Unicode-safe 512-scalar
bounds; isolation provider execution remains framework-owned.
Segment ranges preserve Rust's half-open, saturating length semantics without
owning the message cache.
Prompt diagnostics preserve section recording and per-id counts without owning
prompt compilation.
Subagent command/attempt identities preserve validation and attempt projection
without owning live-control registry state.
Cumulative Subagent LLM usage values preserve sticky reporting, token
accumulation, and payload projection without owning provider execution.
Tool output artifact configuration preserves retention, threshold, and max-age
defaults without owning artifact writing.
Skill validation reports preserve violation gating without running Skill
validation or loading authority.
Skill content values preserve structured prompt-block rendering without loading
or executing resources.
MCP JSON-RPC request and notification values preserve the `2.0` constructors
without owning MCP transport.
Hook action values preserve tagged configuration and validation without
executing commands, HTTP, MCP, or Subagent actions.
Hook event names and categories preserve the Rust matcher classification,
stable `ALL` ordering, PascalCase parsing, and tool/matcher predicates without
owning hook dispatch.
Event and stream identity values preserve non-empty validation, UUID-backed
run/chat constructors, optional correlation fields, and immutable updates;
the Host still owns stream sequencing and lifecycle.
Intervention result factories preserve allow/block/cancel/inject/argument
modification decisions as local immutable values; callback execution stays
with the Host.
Token budget and LLM timeout policies preserve allocation percentages,
compression thresholds, report projections, and zero-disables-timeout behavior
as local configuration values.
Execution usage duration helpers preserve Rust's absent-to-zero projection
without owning run accounting.
Turn mode values preserve the chat/execute stream flavor without owning the
turn driver.
Retry policy values preserve no-retry/default construction, exponential
backoff caps and optional jitter configuration without executing retries.
Thinking configuration values preserve disabled/level/budget variants,
flexible parsing and provider effort/budget projections without owning LLM
transport.
Time helpers preserve Unix timestamps, local-offset RFC3339 serialization,
UTC round-trips and null option values without owning persisted clock state.
Thinking protocol values preserve provider dialect names and field-emission
semantics without owning provider transport.
Sandbox resource limit values preserve default, strict and unrestricted policy
snapshots without creating sandbox processes or owning execution lifecycle.
Provider capability values preserve OpenAI-compatible, Anthropic and Ollama
defaults plus provider-name resolution without owning provider transport.
Thinking profile values preserve model/provider protocol selection and manual
control levels without contacting an LLM provider.
Model profile values preserve provider capability defaults, model limits,
tokenizer selection, and provider/exact override precedence without owning
provider transport.
LLM API protocol values preserve endpoint path selection, strict complete-path
detection, and Anthropic fallback semantics without opening HTTP connections.
Model input modality values preserve text-only and historical all-supported
ordering without owning model transport.
Response format values preserve text, JSON object, and strict JSON Schema tags
without validating or executing provider responses.
缓存布局值保留 system、canonical、history 和动态尾段的只读分段及半开索引。
TypeScript／Java 的 `runtimeContext`、Python 的 `runtime_context`、既有构造参数与 Host
`cacheHints.segments.runtime_context` JSON key 保持原名，映射 Rust `dynamic`；canonical 独立。
动态尾段只接受严格 v2 Tail 信封及末尾连续块，任意 system 消息都不进入动态段。
裸前缀、v1、Boundary、正文伪造元数据均不构成合法 Tail；不新增 `dynamic` 语言别名。
这些值不拥有缓存存储或 provider 断点，详见[ADR0046](../adr/0046-cache-layout-source-identity-projection.md)。
Memory scope values preserve Rust lifetime ordering, persistence classification,
wire names and parser aliases without owning memory storage or cleanup.
Memory type/source values preserve stability, promotion eligibility, confidence
and recall-weight defaults without owning memory persistence or evolution.
Page metadata values preserve truncation, continuation metadata, and output
projection without owning collection state.
Subagent context snapshots preserve empty/content semantics without owning
tools, messages, stores, or dispatch.
Provider-normalized Usage values preserve cache priority and effective token
calculations without owning LLM execution.
Hook action values preserve tagged configuration and validation without
executing commands, HTTP, MCP, or Subagent actions.
Cumulative Subagent LLM usage values preserve sticky reporting, token
accumulation, and payload projection without owning provider execution.
Subagent command/attempt identities preserve validation and attempt projection
without owning live-control registry state.

## For contributors

- Compatibility tests read the accepted external contract. Full Rust inventory
  scope counts and intrinsic membership digests are telemetry; their changes
  do not freeze the language SDKs or extend the accepted contract.
- The inventory toolchain is pinned in
  [`toolchain.json`](../../contracts/sdk/toolchain.json); install it with
  `rustup toolchain install <toolchain>` if you intend to regenerate
  contracts. Nothing in a normal build installs it for you.
- Any new public facade item or signature change appears in the complete
  inventory. Non-external changes are reported by telemetry without changing
  the runtime digest; an accepted external change **blocks CI** until its
  semantic mapping and generated artifacts are reviewed. Cross-crate glob
  re-exports are expanded from matching framework rustdoc documents instead of
  stored as `::*` placeholders. Public registry re-exports use the exact
  locked dependency's rustdoc JSON; procedural and declarative macros carry
  behavior-source digests so helper/body changes remain observable.
- Extension versioning, digest and compatibility rules: see
  [protocol.md](protocol.md#versioning-and-compatibility).

## Related reading

- Design document: `docs/supreme/specs/2026-09-04-source-first-multilanguage-sdk-runtime/design.md`
  (repository-internal, the authoritative design source)
- ADR 0028: `docs/adr/0028-source-first-multilanguage-sdk-runtime.md`
- Official ACP documentation: <https://agentclientprotocol.com/>
- Official Rust SDK: <https://github.com/agentclientprotocol/rust-sdk>

Channel MessageHandler forward calls, cooperative stream close and executable
three-language callback examples are documented in
[Channel consumption](channel-handler-consumption.md). Full Rust inventory and
intrinsic membership remain non-blocking telemetry; these examples exercise the
six explicitly accepted Channel family operations.

### Running languages after cleaning the Rust batch

The ordinary `scripts/check-language-sdks.sh` still builds the current Host and
test collector. When a Rust qualification batch has just built and verified
them, preserve both executable files and their source/lock/command evidence
outside target before cleaning. The same language entry can then use the
absolute `ECHO_AGENT_SDK_PREBUILT_HOST`, mandatory
`ECHO_AGENT_SDK_PREBUILT_HOST_SHA256`, absolute `ECHO_AGENT_SDK_OTLP_COLLECTOR`
and mandatory `ECHO_AGENT_SDK_PREBUILT_COLLECTOR_SHA256`. It verifies both
executable digests and runs the complete original three-language/real-Host/
example chain without another Cargo build. This mode requires evidence that
the artifacts came from the current source batch; a digest alone is not proof
of current-source behavior or a replacement for those fresh consumer tests.

## 当前 Sandbox 命令与退役删除边界

Sandbox命令继续传递六字段值：`kind`、`minimum_isolation`、`working_dir`、`env`、`timeout`、`stdin`，保持原Shell/Program/Code、可选缺省/Null与Duration表示。Host拒绝未知顶层字段和本机`resource_guards`/`cancel`注入；这两个字段只由原Rust物理调用owner持有，不能通过JSON构造。裸SandboxCommand没有新增可调用handle。

旧`echo_agent::state::delete_persisted_conversation`已退出可执行catalog；精确调用返回`feature_unavailable`、`never`、原operation与退役说明。本地拒绝不提交RPC或生成managed身份；managed删除必须保留原请求身份。相近名字、大小写、未知操作与关闭句柄继续原语言合同。

命令guard由既有CallbackInvocationOwner保留到真实物理结算；取消信号、stream setup ACK和观察者drop都不是停止凭证。token选择为显式参数、command.cancel、原run/connection；连接关闭继续取消原lease。
