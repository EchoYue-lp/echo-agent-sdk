# echo-agent-sdk

Source-first TypeScript, Python, and Java SDKs for the Rust `echo-agent`
framework, connected through a source-built ACP Host and the
`_echo_agent/*` extension contract.

## Migration Status

The SDK-owned import and extraction history includes the original clean pin
`27c7701e1eb116db1076da7f84bb68898544a44c`. The current Host governance candidate
resolves published framework revision
`a6346643bda47679a96760f56f45e41fd73b61b0` from
[framework PR #185](https://github.com/EchoYue-lp/echo-agent/pull/185).
The earlier be5 [published-pin provenance](docs/sdk/governance/evidence/sdk-chart-published-pin/published-pin-provenance.json)
remains historical. The protocol remains framework-free, and full Rust
inventory remains telemetry. The [a634 Rust gates](docs/sdk/governance/evidence/sdk-a634-rust-gates/README.md)
and [complete language gate](docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/README.md)
bind their exact execution inputs; later runtime repairs require their own
verification and do not inherit those earlier passes.

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
