# echo-agent-sdk

Source-first TypeScript, Python, and Java SDKs for the Rust `echo-agent`
framework, connected through a source-built ACP Host and the
`_echo_agent/*` extension contract.

## Migration Status

The SDK-owned import and extraction history includes the original clean pin
`27c7701e1eb116db1076da7f84bb68898544a44c`. The current Host governance candidate
resolves published framework revision
`be5f0a8405b593c4ef964cd6fd2df4beabf4956c`, as recorded in the
[published-pin provenance](docs/sdk/governance/evidence/sdk-chart-published-pin/published-pin-provenance.json).
The protocol remains framework-free, and full Rust inventory remains telemetry.

The [twelve accepted-source continuity choices](docs/sdk/governance/accepted-source-continuity-proposal.md)
are still proposed; complete contract generation and SDK main acceptance remain
pending. Published dependency and passing bounded runtime checks do not by
themselves establish final compatibility or release. This source repository
does not publish a bundled Host or runtime.
See [MIGRATION-SOURCE.md](MIGRATION-SOURCE.md) and
[ADR 0001](docs/adr/0001-sdk-repository-boundary.md) for the source boundary,
history, provenance, and ownership decision.

## Contents

- `echo-sdk-protocol/`: ACP extension wire values, schema, catalog, and contract tooling.
- `echo-sdk-host/`: source-built Rust Host adapting `echo_agent` to ACP and SDK methods.
- `contracts/sdk/`: accepted external contracts, schemas, fixtures, Host catalog, and Rust inventory telemetry.
- `sdks/`: TypeScript, Python, Java, and shared source SDK assets.
- `docs/sdk/`: SDK protocol, Host, extension bridge, and facade documentation.
- `scripts/`: contract and source-language verification entry points.
