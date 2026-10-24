# ADR 0042: SDK A2A/LSP retirement boundary

- Date: 2026-10-24
- Status: Accepted direction; documentation contract only. Runtime retirement is pending.
- Owners: `echo-agent-sdk` contract and Host adapter maintainers

## Context and authority

EKO's approved product direction retains its GUI entry, IM messaging, scheduled
tasks, integrated terminal, MCP and shared Task/Subagent/recovery capabilities.
Its independent TUI/REPL/CLI/JSONL product surfaces are to retire. The same
cross-repository decision explicitly retires A2A/LSP in framework, SDK and EKO.
This is a deliberate product/capability decision; application non-use alone
would not justify deleting a reusable framework API.

This ADR records the SDK portion of that decision. At SDK baseline
`0930eef9f878cf436ed88384030b6bc03d6f6c53`, the implementations, compiled feature
options, language APIs and generated contracts still exist. **This document
changes none of them.** Existing status ladders and historical acceptance or
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
