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

The current Host resolves the published framework revision
`b9099558620f5f9693c736c85c214db14f62a8cb`. Its exact dependency and build evidence
are recorded in [published-pin provenance](docs/sdk/governance/evidence/sdk-final-framework-contract/published-pin-provenance.json).
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
