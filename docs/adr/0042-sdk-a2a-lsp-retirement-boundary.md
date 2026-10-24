# ADR 0042: SDK A2A/LSP retirement boundary

- Date: 2026-10-24
- Status: Accepted; SDK consumer withdrawal candidate. Framework physical retirement remains pending.
- Owners: `echo-agent-sdk` contract and Host adapter maintainers

## Status

accepted

## Context and authority

EKO's approved product direction retains its GUI entry, IM messaging, scheduled
tasks, integrated terminal, MCP and shared Task/Subagent/recovery capabilities.
Its independent TUI/REPL/CLI/JSONL product surfaces are to retire. The same
cross-repository decision explicitly retires A2A/LSP in framework, SDK and EKO.
This is a deliberate product/capability decision; application non-use alone
would not justify deleting a reusable framework API.

This ADR records the SDK portion of that decision. At SDK baseline
`0930eef9f878cf436ed88384030b6bc03d6f6c53`, the implementations, compiled feature
options, language APIs and generated contracts existed in full. The original
documentation-only step changed none of them; Plan04 below now governs the SDK
consumer withdrawal. Existing status ladders and historical acceptance or
failure evidence retain their original source cuts. The repository rules now record this future direction. The SDK index remains
unchanged for the still-full implementation; neither is proof that the later
runtime retirement has occurred.

[ADR 0001](0001-sdk-repository-boundary.md) keeps Protocol/Host/language ownership
in this independent product. [ADR 0032](0032-sdk-contract-scope-classification.md)
separates accepted external identities from Rust inventory telemetry, including
accepted language-native identities that happen to have intrinsic routes.
[ADR 0034](0034-sdk-governance-authority-and-lossless-adaptation.md) preserves
framework runtime authority and lossless adaptation. These boundaries continue
to apply. This implements an approved direction rather than selecting a new
protocol, lifecycle model or state authority.

## Options considered

1. Keep all A2A/LSP adapters indefinitely with disabled features. This leaves
   obsolete contracts/controllers as a permanent second surface and does not
   satisfy semantic retirement.
2. Delete the framework APIs first and regenerate every inventory artifact.
   This can break still-dependent consumers, confuses telemetry with acceptance,
   and wrongly couples the SDK pin to the superproject gitlink.
3. Retire exactly the accepted A2A/LSP surfaces in consumers first, then remove
   framework implementations after consumer acceptance. Preserve shared owners
   and the full-capability recovery references.

Choose option 3. The exact source-derived disposition is maintained in the
[SDK retirement boundary](../sdk/governance/a2a-lsp-retirement-boundary.md).
That list records **266 accepted canonical source/kind identities** (A2A 160,
LSP 106) and **23 accepted catalog routes** (A2A 9, LSP 14). A2A includes 47
accepted intrinsic-route identities; selecting only `route.family == a2a` would
miss them. Closed family operations (9 A2A, 30 LSP) are a separate denominator.

## Decision

### Layering and merge order

The framework retains Agent/Session/Task/Subagent, cancellation, recovery,
execution and storage semantics. EKO owns GUI/IM/cron and local product policy;
SDK owns the accepted wire contract, Host adapters, language bindings and their
resource/stream projection. Protocol remains wire/schema only. None of these
changes creates another executor, state store or recovery authority.

For **withdrawn** APIs, EKO and SDK first stop depending on, advertising and
calling the capabilities while the old framework APIs still exist. Only after
those consumer changes are accepted may framework physical removal merge.
EKO CI tracking framework main must remain buildable at each merge boundary.
For **new** APIs, framework-first delivery and consumer adoption still apply.

The SDK Host pins framework
`f7bac62eee04a963f92a3e4308b6ea2336e91cc0` in its
[manifest](../../echo-sdk-host/Cargo.toml). The approved cross-repository design
records top-level framework baseline
`51726454985fd6f064b21a7e9c539e22d494c987`; that gitlink is not the SDK build pin.
SDK decoupling may first ship on the original pin, then separately assess
whether a later pin upgrade is needed. Do not edit manifests or locks merely
to make the revisions appear uniform.

### Exact semantic retirement

The later implementation must remove the listed accepted identities, language
exports/helpers, family methods/operations, feature advertisement, executable
adapters and their active dependencies together. Use the unique generator to
update accepted/source contracts, schemas, fixtures and language projections;
never hand-edit generated JSON. Full Rust inventory remains nonblocking
telemetry and cannot expand the deletion denominator. A shared file, generic
handle, WireValue DTO, capability or error envelope is not retired merely
because an A2A/LSP branch uses it.

Remaining accepted contracts keep their existing behavior and lifecycle. In
particular retain standard ACP and independent `echo-agent-sdk-host` stdio,
MCP, channel/IM consumption, Task/Subagent, streams, replay, cancellation and
recovery, ordinary shell and the extension bridge. SDK Host stdio is not EKO's
retiring CLI product. EKO's file persistence/no-SQLite policy does not authorize
removing reusable framework/SDK storage options. No new permission gate is
introduced for local interactive tools or user-configured MCP extensions.

### Negotiation, failure and recovery

After implementation, advertisements and source-contract digests must honestly
represent the supported contract. An old required feature, source digest,
family method or operation must either fail negotiation or return the applicable
explicit unsupported/invalid diagnostic through existing admission. The
[existing error contract](../../echo-sdk-protocol/src/error.rs),
[hello validation](../../echo-sdk-protocol/src/capability.rs) and
[Host admission ladder](../../echo-sdk-host/src/core_profile/facade/mod.rs)
remain the starting mechanisms: digest/capability mismatch, feature unavailable,
invalid request/value, or standard method-not-found at the appropriate layer.
This ADR does not promise a single error code for every obsolete request.

Do not keep empty-success stubs, silently filter rejected requests, reclassify
accepted identities as deferred to conceal removal, or invent a compatibility
layer that claims old A2A/LSP handles/streams have resumed. Unsupported old
configuration/resource use must be explicit. Existing handle/generation,
terminal cause, replay-gap and physical-settlement semantics remain intact for
retained capabilities; local disposal is never business completion.

## Verification required for later implementation

The later consumer-decoupling result must prove the exact accepted source/kind
and route subtraction, unique generation and unchanged retained behavior.
It needs real-Host positive controls for retained ACP/MCP/Task/Subagent and
resource/stream/cancellation/recovery, plus negative old-feature, old-digest,
old-family and generic-invoke paths. TypeScript, Python and Java must agree on
retained values/results/errors and reject obsolete requests, with no stale
exports/catalog declarations. Run the applicable Host/Protocol/Rust feature
checks, `scripts/check-sdk-contracts.sh`, full `scripts/check-language-sdks.sh`
and its three executable examples. Inventory telemetry is checked separately.
The implementation owner must select the then-applicable cross-platform and
exact-main CI checks; passing this document review proves none of those runs.

This slice only checks Markdown, links, exact inventory derivation and unchanged
implementation/contract inputs. It runs no generator, Host build, language
matrix or Rust full gate. Existing examples remain accurate for the still-full
baseline and stay unchanged. Website projection belongs to the coordinator and
must describe retirement as pending until the relevant implementation ships.

### Build resource and cleanup rule

The user's 2026-10-24 incremental instruction applies to subsequent build
batches without changing the fixed upstream Plan/Design inputs. After each
compilation/test batch actually ends, preserve necessary logs, exit status and
any Host executable/artifact needed later **outside the target directory**.
Confirm no active Cargo/rustc/Host or other consumer is using that exact target,
then promptly run `cargo clean` for the task's own explicitly identified target.
Do not clean primary or another task's cache, remove evidence before saving it,
or claim a deleted executable remains recoverable.

Use independent targets. At most two focused Rust builds may run concurrently,
each with jobs at most 2 and dev/test debug information disabled; full Rust,
Host and language matrices run one at a time. Record `df`/`du` before and after
heavy builds; below 5 GiB free, stop starting new heavy work. A documentation-only
batch with no task-owned Cargo artifacts reports cleanup as not applicable and
does not clean unrelated directories. Branch/worktree, untracked-source and
historical-evidence disposition remains with the coordinator's separately
authorized maintenance work.

The earlier documentation pass preserved a failed full-scope preflight caused
by JSON-frontmatter identity parsing. After the compatibility repair, the normal
preflight includes `AGENTS.md` and its complete associated semantic set; the
rules and this ADR are synchronized. Original failures and historical Finding
contents remain unchanged. This slice ran no Cargo compilation/test batch and
created no Cargo target artifacts.

## Consequences and recovery

The full-capability branch `chore/Echoyue/full-capabilities-baseline` and tag
`pre-retirement-2026-10-24` locally resolve to SDK
`0930eef9f878cf436ed88384030b6bc03d6f6c53`. Preserve them and all existing source
recovery refs; future restoration starts on a new task branch and assesses the
then-current main, rather than moving the baseline tag or reviving an old dirty
worktree. Other repositories' recovery refs are the coordinator's separate
verification responsibility. Exa `4343f0b7` remains an independent local commit,
without push/merge authorization.

Keep the SDK's exact accepted dependency policy, including its current
49 duplicate-version groups bound to the accepted lock, and its historical
risk/unknown-cause/failure evidence. Their original scopes are neither widened
nor erased by this ADR; no Issue/Finding is automatically closed. This is one
SDK contribution to `product-boundary-contract`, not completion of the entire
Plan, the runtime retirement or a release.

## SDK consumer withdrawal under Plan04

Plan `plan:b6ef1ac6-0745-438d-a3ff-75bb8cdbbcc5` explicitly adds the three
shared LSP plugin identities to the original 266. These 269 defining-source/kind
decisions are the inputs to the existing full `export_schema --update` delta
mechanism, not a second accepted registry. No source is newly accepted.
Framework f7bac62e stays pinned; its old Rust definitions remain telemetry.
Retained plugin aggregates omit the two retired fields rather than synthesizing
empty values, and LspServer input is rejected. Remaining aggregate signatures
and digests may change only as a consequence of this explicit projection.

The decisions below authorize unsupported behavior, not equivalence to the old
A2A/LSP execution. ACP/MCP/Task/Subagent/stdio and R2 lifecycles remain unchanged.

### Retire 001

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2AArtifact
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 002

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AArtifact::append
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 003

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AArtifact::index
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 004

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AArtifact::name
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 005

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AArtifact::parts
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 006

- Action: retire
- Kind: method
- Source: echo_agent::a2a::client::A2AClient::cancel_task
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 007

- Action: retire
- Kind: method
- Source: echo_agent::a2a::client::A2AClient::discover
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 008

- Action: retire
- Kind: method
- Source: echo_agent::a2a::client::A2AClient::get_task
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 009

- Action: retire
- Kind: method
- Source: echo_agent::a2a::client::A2AClient::send_task
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 010

- Action: retire
- Kind: method
- Source: echo_agent::a2a::client::A2AClient::send_task_streaming
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 011

- Action: retire
- Kind: method
- Source: echo_agent::a2a::client::A2AClient::send_task_streaming_with_session
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 012

- Action: retire
- Kind: method
- Source: echo_agent::a2a::client::A2AClient::send_task_with_session
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 013

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2AError
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 014

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AError::code
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 015

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AError::message
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 016

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2AMessage
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 017

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::A2AMessage::agent_text
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 018

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AMessage::parts
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 019

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AMessage::role
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 020

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::A2AMessage::text_content
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 021

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::A2AMessage::user_text
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 022

- Action: retire
- Kind: enum
- Source: echo_agent::a2a::types::A2APart
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 023

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::A2APart::File
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 024

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2APart::File::data
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 025

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2APart::File::mime_type
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 026

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::A2APart::Text
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 027

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2APart::Text::text
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 028

- Action: retire
- Kind: enum
- Source: echo_agent::a2a::types::A2AStreamEvent
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 029

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::A2AStreamEvent::ArtifactUpdate
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 030

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AStreamEvent::ArtifactUpdate::0
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 031

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::A2AStreamEvent::StatusUpdate
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 032

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AStreamEvent::StatusUpdate::0
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 033

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2AStreamResponse
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 034

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AStreamResponse::error
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 035

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AStreamResponse::id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 036

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AStreamResponse::jsonrpc
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 037

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2AStreamResponse::result
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 038

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2ATask
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 039

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATask::artifacts
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 040

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATask::history
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 041

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATask::id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 042

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATask::session_id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 043

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATask::status
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 044

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2ATaskParams
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 045

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskParams::id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 046

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskParams::message
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 047

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskParams::session_id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 048

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2ATaskRequest
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 049

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskRequest::id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 050

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskRequest::jsonrpc
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 051

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskRequest::method
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 052

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskRequest::params
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 053

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2ATaskResponse
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 054

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskResponse::error
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 055

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskResponse::id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 056

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskResponse::jsonrpc
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 057

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskResponse::result
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 058

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::A2ATaskStatus
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 059

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskStatus::message
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 060

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::A2ATaskStatus::new
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 061

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskStatus::state
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 062

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::A2ATaskStatus::timestamp
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 063

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::A2ATaskStatus::with_message
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 064

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::AgentAuthentication
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 065

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentAuthentication::schemes
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 066

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::AgentCapabilities
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 067

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCapabilities::push_notifications
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 068

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCapabilities::state_transition_history
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 069

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCapabilities::streaming
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 070

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::AgentCard
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 071

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::authentication
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 072

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCard::builder
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 073

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::capabilities
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 074

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::default_input_modes
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 075

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::default_output_modes
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 076

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::description
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 077

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::name
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 078

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::provider
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 079

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::skills
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 080

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::url
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 081

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentCard::version
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 082

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::AgentCardBuilder
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 083

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::authentication
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 084

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::build
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 085

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::description
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 086

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::input_modes
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 087

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::output_modes
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 088

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::provider
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 089

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::push_notifications
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 090

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::skill
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 091

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::skills
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 092

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::streaming
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 093

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentCardBuilder::version
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 094

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::AgentProvider
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 095

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentProvider::new
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 096

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentProvider::organization
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 097

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentProvider::url
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 098

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentProvider::with_url
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 099

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::AgentSkill
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 100

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentSkill::description
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 101

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentSkill::examples
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 102

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentSkill::id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 103

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentSkill::input_modes
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 104

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentSkill::name
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 105

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentSkill::new
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 106

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentSkill::output_modes
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 107

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AgentSkill::tags
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 108

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentSkill::with_examples
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 109

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::AgentSkill::with_tags
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 110

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::AuthenticationScheme
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 111

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AuthenticationScheme::config
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 112

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::AuthenticationScheme::scheme
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 113

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::ERROR_CODE_INVALID_PARAMS
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 114

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::ERROR_CODE_INVALID_TRANSITION
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 115

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::ERROR_CODE_METHOD_NOT_FOUND
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 116

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::ERROR_CODE_PARSE
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 117

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::ERROR_CODE_TASK_FAILED
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 118

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::ERROR_CODE_TASK_NOT_FOUND
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 119

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::ERROR_CODE_TERMINAL_STATE
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 120

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::JSONRPC_VERSION
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 121

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::auth::JwtClaims::aud
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 122

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::auth::JwtClaims::exp
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 123

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::auth::JwtClaims::extra
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 124

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::auth::JwtClaims::iat
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 125

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::auth::JwtClaims::iss
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 126

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::auth::JwtClaims::jti
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 127

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::auth::JwtClaims::nbf
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 128

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::auth::JwtClaims::sub
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 129

- Action: retire
- Kind: method
- Source: echo_agent::a2a::auth::JwtClaims::subject
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 130

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::auth::JwtConfig
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 131

- Action: retire
- Kind: method
- Source: echo_agent::a2a::auth::JwtConfig::disabled
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 132

- Action: retire
- Kind: method
- Source: echo_agent::a2a::auth::JwtConfig::hs256
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 133

- Action: retire
- Kind: trait_impl
- Source: echo_agent::a2a::auth::JwtConfig::impl<Debug>
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 134

- Action: retire
- Kind: method
- Source: echo_agent::a2a::auth::JwtConfig::is_enabled
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 135

- Action: retire
- Kind: method
- Source: echo_agent::a2a::auth::JwtConfig::rs256
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 136

- Action: retire
- Kind: method
- Source: echo_agent::a2a::auth::JwtConfig::with_audience
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 137

- Action: retire
- Kind: method
- Source: echo_agent::a2a::auth::JwtConfig::with_issuer
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 138

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::auth::JwtConfigError
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 139

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::METHOD_CANCEL
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 140

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::METHOD_GET
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 141

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::METHOD_SEND
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 142

- Action: retire
- Kind: constant
- Source: echo_agent::a2a::types::METHOD_SEND_SUBSCRIBE
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 143

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::TaskArtifactUpdateEvent
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 144

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::TaskArtifactUpdateEvent::artifact
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 145

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::TaskArtifactUpdateEvent::is_final
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 146

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::TaskArtifactUpdateEvent::task_id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 147

- Action: retire
- Kind: enum
- Source: echo_agent::a2a::types::TaskState
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 148

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::TaskState::Canceled
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 149

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::TaskState::Completed
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 150

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::TaskState::Failed
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 151

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::TaskState::InputRequired
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 152

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::TaskState::Submitted
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 153

- Action: retire
- Kind: variant
- Source: echo_agent::a2a::types::TaskState::Working
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 154

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::TaskState::can_transition_to
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 155

- Action: retire
- Kind: trait_impl
- Source: echo_agent::a2a::types::TaskState::impl<Display>
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 156

- Action: retire
- Kind: method
- Source: echo_agent::a2a::types::TaskState::is_terminal
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 157

- Action: retire
- Kind: struct
- Source: echo_agent::a2a::types::TaskStatusUpdateEvent
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 158

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::TaskStatusUpdateEvent::is_final
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 159

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::TaskStatusUpdateEvent::status
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 160

- Action: retire
- Kind: struct_field
- Source: echo_agent::a2a::types::TaskStatusUpdateEvent::task_id
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 161

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::CompletionItem
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 162

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::CompletionItem::detail
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 163

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::CompletionItem::documentation
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 164

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::CompletionItem::insert_text
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 165

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::CompletionItem::kind
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 166

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::CompletionItem::label
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 167

- Action: retire
- Kind: enum
- Source: echo_core::lsp::types::CompletionItemKind
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 168

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Class
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 169

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Color
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 170

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Constant
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 171

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Constructor
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 172

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Enum
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 173

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::EnumMember
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 174

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Event
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 175

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Field
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 176

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::File
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 177

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Folder
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 178

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Function
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 179

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Interface
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 180

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Keyword
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 181

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Method
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 182

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Module
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 183

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Operator
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 184

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Property
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 185

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Reference
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 186

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Snippet
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 187

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Struct
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 188

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Text
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 189

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::TypeParameter
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 190

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Unit
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 191

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Value
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 192

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::CompletionItemKind::Variable
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 193

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::Diagnostic
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 194

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Diagnostic::code
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 195

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Diagnostic::message
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 196

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Diagnostic::range
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 197

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Diagnostic::severity
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 198

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Diagnostic::source
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 199

- Action: retire
- Kind: enum
- Source: echo_core::lsp::types::DiagnosticCode
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 200

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::DiagnosticCode::Integer
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 201

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::DiagnosticCode::String
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 202

- Action: retire
- Kind: enum
- Source: echo_core::lsp::types::DiagnosticSeverity
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 203

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::DiagnosticSeverity::Error
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 204

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::DiagnosticSeverity::Hint
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 205

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::DiagnosticSeverity::Information
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 206

- Action: retire
- Kind: variant
- Source: echo_core::lsp::types::DiagnosticSeverity::Warning
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 207

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::HoverInfo
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 208

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::HoverInfo::contents
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 209

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::HoverInfo::range
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 210

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::Location
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 211

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Location::range
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 212

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Location::uri
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 213

- Action: retire
- Kind: struct_field
- Source: echo_integration::lsp::config::LspConfig::servers
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 214

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::LspConfigFile
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 215

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspConfigFile::languages
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 216

- Action: retire
- Kind: enum
- Source: echo_core::lsp::client::LspError
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 217

- Action: retire
- Kind: variant
- Source: echo_core::lsp::client::LspError::CommunicationError
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 218

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::client::LspError::CommunicationError::0
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 219

- Action: retire
- Kind: variant
- Source: echo_core::lsp::client::LspError::InvalidUri
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 220

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::client::LspError::InvalidUri::0
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 221

- Action: retire
- Kind: variant
- Source: echo_core::lsp::client::LspError::NotInitialized
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 222

- Action: retire
- Kind: variant
- Source: echo_core::lsp::client::LspError::NotRunning
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 223

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::client::LspError::NotRunning::0
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 224

- Action: retire
- Kind: variant
- Source: echo_core::lsp::client::LspError::ServerError
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 225

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::client::LspError::ServerError::0
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 226

- Action: retire
- Kind: variant
- Source: echo_core::lsp::client::LspError::SpawnError
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 227

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::client::LspError::SpawnError::0
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 228

- Action: retire
- Kind: variant
- Source: echo_core::lsp::client::LspError::Timeout
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 229

- Action: retire
- Kind: struct
- Source: echo_integration::lsp::manager::LspManager
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 230

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::configured_languages
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 231

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::get_client
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 232

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::get_client_for_file
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 233

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::load_config
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 234

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::new
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 235

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::restart_server
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 236

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::running_servers
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 237

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::set_project_root
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 238

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::shutdown_all
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 239

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::start_server
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 240

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::status_all
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 241

- Action: retire
- Kind: method
- Source: echo_integration::lsp::manager::LspManager::stop_server
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 242

- Action: retire
- Kind: type_alias
- Source: echo_core::lsp::client::LspResult
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 243

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::LspServerConfig
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 244

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerConfig::args
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 245

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerConfig::command
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 246

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerConfig::env
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 247

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerConfig::extensions
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 248

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerConfig::initialization_options
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 249

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerConfig::language
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 250

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerConfig::max_restarts
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 251

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::LspServerStatus
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 252

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerStatus::initialized
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 253

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerStatus::language
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 254

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerStatus::last_error
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 255

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerStatus::pid
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 256

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerStatus::restart_count
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 257

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::LspServerStatus::running
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 258

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::Position
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 259

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Position::character
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 260

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Position::line
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 261

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::Range
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 262

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Range::end
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 263

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::Range::start
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 264

- Action: retire
- Kind: struct
- Source: echo_core::lsp::types::TextChange
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 265

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::TextChange::range
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 266

- Action: retire
- Kind: struct_field
- Source: echo_core::lsp::types::TextChange::text
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 267

- Action: retire
- Kind: variant
- Source: echo_core::plugin::capability::PluginCapability::LspServer
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 268

- Action: retire
- Kind: struct_field
- Source: echo_agent::plugin::prepared::PluginWiringResult::lsp_discovered
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.

### Retire 269

- Action: retire
- Kind: struct_field
- Source: echo_core::plugin::registry::ResolvedComponents::lsp_config_file
- Reason: Plan04 explicitly withdraws the A2A/LSP SDK consumer surface, including the three named shared plugin identities.
- Compatibility: This identity leaves the accepted SDK contract; other accepted source/kind identities remain. Old requests fail explicitly.
- Replacement: unsupported; no A2A/LSP execution or fake empty compatibility result.
- Rollback: restore SDK 961da0e0d17b815653d80cb4bf4973248f543770 and its complete contracts/clients together on a new task branch.
