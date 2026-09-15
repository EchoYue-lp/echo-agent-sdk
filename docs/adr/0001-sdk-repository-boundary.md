# ADR 0001: Independent SDK Repository Boundary

- Status: Accepted
- Date: 2026-09-16
- Owners: `echo-agent-sdk` maintainers

## Status

Accepted.

## Context

The SDK product originally lived inside the `echo-agent` framework repository.
It includes the Rust protocol and Host crates, the accepted contract artifacts,
TypeScript, Python, and Java clients, SDK documentation, generators, examples,
and SDK-specific CI. Keeping those assets in the framework workspace couples
unrelated framework changes to every SDK toolchain and leaves product ownership
ambiguous.

The initial independent-repository import was taken from framework revision
`989e3296cdf34e31c4150f99ac1615cf7aacfb7a`. The framework later froze semantic
governance at `c5f7688212d45d5bdcdbf60342605e8bfb176cae`, which contains additional
committed SDK changes. Removing the framework copy before importing those
changes would lose the latest contract and language evidence.

## Options

1. Keep the complete SDK product in the framework workspace.
2. Keep only a snapshot in this repository and record the old repository SHA in
   prose.
3. Merge filtered SDK-owned framework history into this repository, synchronize
   the frozen SDK tree, and continue with independent versions and CI.

## Decision

Choose option 3.

The independent repository owns `echo-sdk-protocol`, `echo-sdk-host`,
`contracts/sdk`, `sdks`, `docs/sdk`, SDK generators, examples, ADRs, and CI.
The framework owns the reusable `echo_agent` facade, ACP adapter, and all
Agent/Run/Task/Subagent, event, cancellation, recovery, persistence, and Tool
semantics. Dependency direction is SDK to framework only.

Import filtered SDK-owned Git history through an explicit merge so both the new
repository's initial commit and the source framework history remain ancestors.
The resulting worktree uses the SDK-owned files from framework revision
`c5f7688212d45d5bdcdbf60342605e8bfb176cae`. The two repository-local Cargo
dependency declarations may differ so the SDK can resolve the framework
without a sibling checkout; the ACP adapter document may replace its sibling
example path with an immutable framework URL. Do not force-push or rewrite
either source history.

This source-continuity step does not purify the protocol dependency, change the
accepted external contract, regenerate artifacts, or claim that the SDK is
independently runnable. Those changes require their own reviewed delivery.

## Consequences

- The framework can remove its editable SDK copy without losing committed SDK
  behavior, contracts, documentation, or history.
- SDK ADRs and current product documentation have one long-term owner.
- A framework dependency update becomes an explicit SDK repository change.
- Historical merge ancestry is preserved even though future framework and SDK
  development use independent branches and release cadence.

## Verification

- Prove the final SDK commit has both the initial import lineage and the filtered
  framework SDK history as ancestors.
- Compare every SDK-owned path with framework revision `c5f7688`, allowing only
  the two documented Cargo manifest dependency adaptations and the immutable
  ACP example link adaptation.
- Validate repository-local SDK documentation links and Cargo metadata.
- Push the task commit without force and verify the remote SHA.
