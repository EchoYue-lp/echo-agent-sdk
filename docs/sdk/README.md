# echo-agent SDK (standard ACP Host available)

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

> **Implemented source candidate: standard ACP, core extension profile,
> extension bridge and facade feature adapters. Final governance acceptance is open.** The source-built `echo-agent-sdk-host` passes
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
> routing, including real Workflow/A2A pull streams. Source-built TypeScript,
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

| Artifact | Purpose |
|---|---|
| `contracts/sdk/acp-baseline.json` | Pinned official ACP wire version (1), crate and schema artifact versions; tests assert the lockfile matches. |
| `contracts/sdk/toolchain.json` | The exact nightly toolchain used for rustdoc-JSON inventory generation. Contributors only; normal builds never need it. |
| `contracts/sdk/public-api.txt` | Complete deterministic root-facade snapshot used for Rust inventory telemetry; it is not a runtime compatibility gate. |
| `contracts/sdk/parity-manifest.schema.json` | Machine schema for the complete inventory telemetry manifest. |
| `contracts/sdk/parity-manifest.json` | Complete facade inventory classified by consumer-facing `sdk_scope`, semantic rule, ACP relationship, feature condition, adapter operation and per-language mapping/test status. |
| `contracts/sdk/accepted-external-contract.json` | Generated blocking contract containing only canonical identities explicitly classified as `external_contract`; aliases remain in telemetry and inherit their canonical scope. |
| `contracts/sdk/accepted-external-contract.schema.json` | Machine schema for the blocking external contract artifact. |
| `contracts/sdk/accepted-facade-operation-catalog.json` | Language-facing catalog filtered to accepted non-intrinsic routes. |
| `contracts/sdk/inventory-telemetry.json` | Compact non-blocking report with framework provenance, full inventory digest and scope counts. |
| `contracts/sdk/schema/echo-agent-extension-v1.schema.json` | Generated JSON Schema of the `_echo_agent/*` extension DTOs and method catalog. |
| `contracts/sdk/fixtures/extension/v1/` | Golden fixtures: valid samples must round-trip losslessly, invalid samples must be rejected deterministically. |
| `contracts/sdk/source-contract.json` | Small generated digest over the accepted external contract and accepted facade catalog, embedded by the Host and matched by the Client hello. |

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

| Status | Meaning | Reached |
|---|---|---|
| **Design** | The design document is agreed | ✅ |
| **Contract** | Full generation and accepted source/catalog continuity match the final pin | Pending: twelve source choices and full generation |
| **ACP conformant** | A standard ACP v1 client passes the supported profile against a real source-built Host | ✅ |
| **Core extension profile** | The negotiated `_echo_agent/*` core families run against a real Host with typed lifecycle, events, replay and recovery | ✅ (Rust Host only) |
| **Host facade routing** | Accepted source operations/consumer traits/streams have a concrete Host route or an explicit local boundary | Historical Plan 08 route baseline; current capability/failure review remains separate |
| **Runnable** | A real Host plus each language's declared external SDK path executes end-to-end | Snapshot-bound language gate passed; later repairs require current-source verification |
| **External contract complete** | Accepted capability behavior, lifecycle and failures pass the final generated contract and matched-source evidence | Pending final continuity/behavior/main acceptance; 5,622 is the frozen seed count |
| **Deferred capability grouping** | Existing inventory is grouped without adding external acceptance | [14 bounded Rust/Host/deferred groups](governance/deferred-capabilities.md); final inventory regeneration pending |
| **Published** | Registry/binary publication — **explicitly out of scope**; this design ships source only | never (by design) |

### Identity inventory scope

The following counts describe the committed extraction inventory used by the
wire candidate. They are historical telemetry, not a freshly generated a634
public inventory or proof that the twelve continuity choices were accepted.
The current pin is `a6346643bda47679a96760f56f45e41fd73b61b0`; see
[source continuity](../../MIGRATION-SOURCE.md), the
[a634 Rust gate](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-a634-rust-gates/README.md) and
[complete language gate](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/README.md)
for exact input cuts. Subsequent repairs and final publication/idempotency
validation remain separately bound.

The committed parity manifest contains 9,724 canonical identities. TypeScript,
Python, and Java all mark 5,622 of them `done`; the
remaining 4,102 identities are all classified under the `intrinsic` route
surface. Standard ACP, core extension, family, bridge, invoke, and value route
surfaces have no remaining not-done canonical identities.

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

| `sdk_scope` | Canonical items | Current meaning |
|---|---:|---|
| `external_contract` | 5,622 | Frozen accepted canonical membership and language mapping; capability behavior requires separate evidence. |
| `host_or_rust_only` | 1,774 | Process-local or Host-owned runtime authority. |
| `language_intrinsic` | 790 | Rust syntax, trait implementations and callback types represented idiomatically. |
| `internal_helper` | 90 | Rust testing support, not an SDK product surface. |
| `deferred` | 1,448 | Potential capabilities awaiting capability-level contract review. |

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
The closed A2A `TaskState` transition/display slice is also implemented in all
three languages, without changing the wire surface.
The related A2A Message, TaskStatus, Provider and Skill value constructors are
covered by the same native-only intrinsic boundary.
The Agent Card and fluent local builder are also available in all three
languages; `from_agent` remains open because it requires the Rust Agent
authority.
The immutable A2A Artifact and Error DTOs preserve their wire fields in all
three languages as well.
The A2A status/artifact stream event and JSON-RPC response DTOs are also
available as local immutable values; stream transport remains Host-owned.
Task request/params/response and task history envelopes are available as
immutable nested DTOs; execution remains a Host operation.
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
Permission modes expose their canonical IDs, aliases, and policy predicates;
mode evaluation remains framework-owned.
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
A2A `JwtConfig` and `JwtClaims` are available as local immutable projections;
JWT verification, key handling, and A2A server ownership remain Host/Rust-owned.
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
Prompt cache layout values preserve the Rust system/canonical/history/runtime
segment projection and half-open indexes without owning cache state or provider
placement.
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
