# SDK Source Import

This repository initially imports the complete SDK product from
`git@github.com:EchoYue-lp/echo-agent.git`.

## Frozen Source

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

## Transitional State

The source framework is still completing semantic governance. This import is
therefore a relocation checkpoint, not an SDK compatibility release:

- the framework revision is not yet guaranteed to be available from the
  remote repository;
- existing contract, generated-artifact, scope-count, and language parity
  drift is intentionally preserved;
- the imported CI workflow still reflects its source-repository layout;
- protocol dependency purification, accepted external contract separation,
  deterministic regeneration, Host integration, and full language validation
  remain deferred until the framework governance revision is frozen.

Do not describe this source-import commit as independently runnable or parity
complete. The cross-repository design and delivery map live in the
`lp-agent` superproject under
`docs/supreme/specs/2026-09-15-echo-agent-sdk-independent-repository/`.
