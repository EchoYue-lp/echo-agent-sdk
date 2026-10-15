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

The current Host dependency target is published framework revision
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
remain proposed. Complete accepted contract generation, final SDK gates and SDK
main acceptance are pending. Bounded runtime and language evidence on this pin
does not replace those requirements or claim a binary/registry release.

The source continuity history above remains the ancestry authority; this file
does not claim a published binary or registry release. The cross-repository
design and delivery map live in the `lp-agent` superproject under
`docs/supreme/specs/2026-09-15-echo-agent-sdk-independent-repository/`.
