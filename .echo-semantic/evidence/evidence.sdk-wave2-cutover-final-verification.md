---
schema_version: 1
id: evidence.sdk-wave2-cutover-final-verification
kind: evidence
observed_at: 146f69a923e7df02417528c3dce6533312be85e1
source_refs:
  - AGENTS.md
  - Cargo.lock
  - MIGRATION-SOURCE.md
  - README.md
  - README.zh.md
  - contracts/sdk/inventory-telemetry.json
  - contracts/sdk/source-contract.json
  - docs/adr/0001-sdk-repository-boundary.md
  - echo-sdk-host/Cargo.toml
  - echo-sdk-protocol/src/facade.rs
  - echo-sdk-protocol/tests/facade_inventory.rs
  - scripts/check-sdk-contracts.sh
  - scripts/check-sdk-inventory-telemetry.sh
  - scripts/check-language-sdks.sh
supports: [finding.sdk-wave2-cutover-integrity, behavior.sdk-repository-ownership, rule.framework-runtime-authority, asset.sdk-source-product]
limitations:
  - source delivery is verified; binary, registry and installer publication remain outside the accepted repository extraction outcome
  - future framework revisions require a new explicit SDK dependency update and inventory review
---

# SDK Wave 2 cutover final verification

## 支持的结论

SDK PR #2 merged with a real two-parent merge commit
`146f69a923e7df02417528c3dce6533312be85e1`. Its second parent is the reviewed cutover head
`c8941c6033c8cd5f698502483fb28a80953be48d`, so source-continuity and clean-pin history remain
reachable from SDK `main`.

The Host manifest, lockfile, provenance gate and inventory telemetry pin framework main
`27c7701e1eb116db1076da7f84bb68898544a44c`. The unique generator records 9,724 canonical
identities with scope counts `5,622 / 1,774 / 790 / 90 / 1,448` and inventory digest
`sha256:150957be45b705d68a750917735710d777c642e05066d31374602fd37cdbdbbb`.

## 来源与范围

Evidence covers Git ancestry, framework provenance, generated contracts, Host adapter compilation,
protocol purity and TypeScript/Python/Java source delivery.

## 执行证据

- local formatter, two Clippy profiles, all-target/all-feature tests and no-default check passed;
- Host core, extension, facade and stdio E2E passed;
- contract and inventory telemetry checks reported zero drift;
- TypeScript 157/157, Python 177/177, Java Maven and real Host quickstarts passed;
- cargo audit and cargo deny passed;
- SDK PR #2 remote CI completed 8/8 successfully;
- `b80cf06`, `89d18f0`, `6f743d1`, `863cd5b` and `c8941c6` are ancestors of SDK main.

## 已知缺口

No precompiled artifacts or registry packages were published by this outcome. The compatibility
claim is limited to the committed source, framework revision and generated contract evidence.
