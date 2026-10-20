# SDK Source Continuity

This repository initially imports the complete SDK product from
`git@github.com:EchoYue-lp/echo-agent.git`.

## Initial Import

- Source revision: `989e3296cdf34e31c4150f99ac1615cf7aacfb7a`
- Source branch: `fix/Echoyue/turn-terminal-delivery-settlement`
- Imported archive SHA-256:
  `0f2f9aa7d18058014e27322f4b6113b9ad799ff119df9065e17ff89566ea9384`
- Imported paths: `echo-sdk-protocol/`, `echo-sdk-host/`, `contracts/sdk/`,
  `sdks/`, `docs/sdk/`, the three SDK scripts, and the source Rust CI workflow.
- Supporting files: `Cargo.lock`, `.cargo/`, `.gitignore`, `LICENSE`,
  `deny.toml`, and `rust-toolchain.toml`.

The archive includes the SDK-owned tracked working-tree changes present when
the snapshot was taken. The source `echo-agent` checkout was not modified,
staged, committed, or reset by this import.

## Frozen Framework Source

- Frozen framework revision:
  `c5f7688212d45d5bdcdbf60342605e8bfb176cae`
- Filtered SDK history tip before the repository merge:
  `28d6709c93a7809c1eb3ba744d3e4cc89b0332f7`
- Filtered history scope: the SDK-owned paths above, SDK ADR 0028/0031/0032,
  the source-first SDK design history, and the source Rust CI workflow.

The range from the initial committed revision `989e3296` to the frozen
framework revision changes 17 SDK-owned files (`+588/-118`). Four of those
files were already present at their frozen contents because the initial import
also captured the source working tree. Synchronizing the complete frozen tree
therefore changes 13 files in this repository. Directory-level comparison,
not the local diff count, is the continuity authority.

The final SDK tree matches the frozen framework SDK-owned paths byte for byte,
except for three repository adaptations: `echo-sdk-protocol/Cargo.toml` and
`echo-sdk-host/Cargo.toml` retain independent Git dependency declarations, and
`docs/sdk/acp-agent-adapter.md` links to the frozen framework example through
GitHub instead of an unavailable sibling path.

The source history is merged as a second parent rather than copied only as
prose. This preserves both the new repository's initial commit and the filtered
framework SDK history without rewriting either history.

## Historical Extraction Pin

The initial independent-repository extraction pinned the pushed framework
revision `27c7701e1eb116db1076da7f84bb68898544a44c`. That revision belongs to the
historical migration above; it is not the current Host dependency. The protocol
crate was made framework-free, and accepted external contracts were separated
from the complete Rust inventory telemetry. The exact blocking/non-blocking
boundary remains recorded in `contracts/sdk/source-contract.json` and
`contracts/sdk/inventory-telemetry.json`.

## Current Governance Candidate Pin

The current source pin is accepted published framework
`3560812caede84998a1f6f26d7bf3fa81ea0486c`, tree
`bbacaad650b935c516194a9373580fe7747c8256`, from
[PR #193](https://github.com/EchoYue-lp/echo-agent/pull/193).
Its exact-main [CI run 39355472505](https://github.com/EchoYue-lp/echo-agent/actions/runs/39355472505)
completed all seven jobs successfully. The squash tree matches reviewed framework
`8138d81efa69a3852d81ddef70c7f417c8a3b962`; upstream source digest is
`4ad31ae30cca2dfe0d0b04d90dd656b2f3ddb3280337f5e4407e9fbe6fed8276`.
The SDK lock SHA is
`58599a9f95c7b524fe63c97a560d2bac235539570b2ed9ed6165586c793444a3`:
all 656 package identities, versions, checksums and dependency edges are unchanged;
only eight framework and three upstream vendored Git sources moved to this pin.
The sole full 28-profile/117-artifact update and original full contract entry both
exited zero. Accepted identities/kinds remain 5621 and routes remain 454; only the
accepted set_model signature digest changed from 20f0cd… to d89f95… for Result<()>.
No new constructor or cursor named field was automatically accepted.

The pre-media-repair SDK208a Host build is `0bbc72ba1cb08e7264704647566e490afc4918de57cc22fcee8a414fe9e518a8`.
It preserves the saved development profile, has zero build warnings and zero
tracked-source drift, and passes named finite model, compressor, base/current,
busy-read, core and live consumer after cases. These are distinct source/runtime
cuts; two test-fixture failures remain preserved rather than relabelled as product
repairs. On public SDK208a the original fourteen Rust commands, one full language
entry (TS357, Python441 plus18 subtests, Java378 with76 fresh XML reports), all three
quickstarts and fresh actual-lock dependency scans have now exited zero. Source
inputs and Host bytes/mtime remained unchanged; Java was Oracle21.0.5. The sole
warning exception is the originally accepted49 exact duplicate groups, with no
new group/version/category; original HTTPS failures and process-only official
SSH transport recovery remain preserved. Four source Findings are locally
resolved only through their concrete three-link chains. Named semantic
settlement, historical Linux unknown, nativeHook/Stop and exact SDK main
acceptance remain separate. Prior results below retain their historical
provenance; upstream acceptance and focused consumers do not imply SDK release.

The media successor repairs only the two accepted operation-to-Tool spellings.
Its newly compiled Host `a17db78e5d3e73a5edf9ef529d387a218aa248d7042b08e12b88b843cd53baaf`
has focused actual after evidence for both original counterexamples and the
three native language media stories, retaining the original text oracles.
The same-root-cause Finding now has concrete repair/verification/rereview chains
and is locally resolved after current SDK9ef frozen-source Rust14 and the one
original whole language entry plus all three quickstarts completed successfully.
Their source/Host proofs remain distinct from the old SDK208a evidence. Exact
SDK-main/platform/nativeHook and the unrelated historical Linux cause remain pending.

### Previous 6563 source and runtime evidence

The current Host dependency is published framework
`6563d26570cc4a8d93c86b7e37443b0559476807`, tree
`2875ec0d92aed44b35e0fe9b6b00357380c1557d`, merged through
[framework PR #191](https://github.com/EchoYue-lp/echo-agent/pull/191).
Its sole parent is `9886502c83d6b1f6fa9212eb221db0e5874deaee`.
The signed commit and all seven successful jobs of exact-main CI run
`38958501642` were checked against that exact revision.
All eight framework packages and the three upstream vendored packages
(`calamine`, `docx-rs`, `rust_xlsxwriter`) in this SDK's actual lock use that
one Git revision; the lock SHA-256 is
`90d5b82ef54a56d22fc079f9065e86e817a81b2431e265bf1d47ebe3897f40d5`.
Only those eleven source origins changed. Package versions, checksums and
dependency edges are unchanged. No path override or borrowed framework lock
is used.

The earlier `9886502c` pin's sole full 28-profile generator and complete
contract check both exited0. Those receipts remain historical. The new pin's
canonical producer completed all 28 actual profiles and wrote 117 artifacts.
The complete contract check exited0 with zero blocking or telemetry drift;
its Protocol suites passed 5/42/91 tests. Accepted identity/kind membership
remains exactly 5,621 and its 454 routes are unchanged in both directions;
the five blocking/shared contract files are byte-identical to the prior pin.
The only new public identity, `RunEntry::wait_result`, remains Host/Rust-only
telemetry. Current SDK source `8481ba5823aa190cf76859d6af0570898c432bbd`
passed all fourteen original local Rust gates with zero compiler warnings and
zero drift across 1,212 inputs. Its immutable Host SHA-256 is
`0982121cb1e99284b4a7d7e07af35037f9f22bd653528d9716d425f5609407a0`.
The original complete language entry passed TypeScript354, Python440 plus18
subtests, Java377 across76 fresh XML reports, and all three quickstarts on
that same Host. Exact SDK-main Linux/Windows CI, canonical/native host final
verification and the historical managed passive-exit investigation remain
separate. Accepted intrinsic helper, source-operation consumer/lifecycle, and ContextInheritance promise groups still require bounded evidence reconciliation before this draft PR can merge. These are original accepted obligations, not a new Rust-inventory parity requirement.
The necessary full-SHA annotated evidence tag preserves this actual
runtime source across a later squash; it is not release or main acceptance. The two approved fallible Rust signatures come from the canonical
producer, and the protected-claim addition remains limited to the separately
accepted capability and reverse Unit operation. Earlier pin receipts do not
qualify this source. See [the current task ledger](docs/MASTER-PLAN.md) and
[ADR0039](docs/adr/0039-sdk-cancellation-observation.md).

## Historical Governance Pin 00782

The earlier Host dependency target was published framework revision
`00782b8aeddefe2ea8c566922cbdee6aedc80d70`, merged through
[framework PR #186](https://github.com/EchoYue-lp/echo-agent/pull/186).
The signed published source tree matches the independently reviewed candidate;
it repairs Git branch switching through the original framework authority.
The SDK lock additionally selects rustls 0.23.46. Framework post-main CI and
[SDK pin/lock build and dependency checks](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-published-00782-pin/README.md)
have actual reviewed receipts. [Rust14](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-00782-final-rust-gates/README.md)
and the [fourth complete language run](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-ci-repairs-final/README.md)
passed locally; all 800 execution inputs are recoverable at b0e43/b315.
This local Oracle21 result does not prove Linux CI Temurin17 or new action
warning fixes, which require their actual remote execution.
The earlier `be5f0a8405b593c4ef964cd6fd2df4beabf4956c`
[published-pin provenance](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-chart-published-pin/published-pin-provenance.json)
remains historical evidence for that older dependency/build, not validation of
this new Host target. The [historical a634 Rust gates](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-a634-rust-gates/README.md)
and [complete language gate](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/README.md)
retain their exact source input hashes. Runtime repairs made after those runs,
including stream failure classification and current publication/idempotency
verification, retain separately bound evidence. Full contract generation, new
applicable CI, current-source semantics and exact SDK main acceptance remain
separate.
This changes the current build input, not the initial import archive, filtered
history or extraction ancestry hashes recorded above.

The [twelve accepted-source continuity choices](docs/sdk/governance/accepted-source-continuity-proposal.md)
were explicitly accepted on 2026-10-15, as recorded in ADR0034. Their generator
checkpoint and bounded consumer evidence do not replace final SDK gates or
exact-main acceptance and do not claim a binary/registry release.

The source continuity history above remains the ancestry authority; this file
does not claim a published binary or registry release. The cross-repository
design and delivery map live in the `lp-agent` superproject under
`docs/supreme/specs/2026-09-15-echo-agent-sdk-independent-repository/`.
