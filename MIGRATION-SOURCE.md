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

## Clean-Pin State

The framework extraction revision is now pushed and pinned by the SDK Host:
`27c7701e1eb116db1076da7f84bb68898544a44c`. The protocol crate is framework
free, the accepted external contract is separated from the complete Rust
inventory telemetry, and generated artifacts are reproducible from an
independent clone. The exact blocking/non-blocking boundary is recorded in
`contracts/sdk/source-contract.json` and `contracts/sdk/inventory-telemetry.json`.

The source continuity history above remains the ancestry authority; this file
does not claim a published binary or registry release. The cross-repository
design and delivery map live in the `lp-agent` superproject under
`docs/supreme/specs/2026-09-15-echo-agent-sdk-independent-repository/`.
