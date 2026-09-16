# echo-agent-sdk

Source-first TypeScript, Python, and Java SDKs for the Rust `echo-agent`
framework, connected through a source-built ACP Host and the
`_echo_agent/*` extension contract.

## Migration Status

This repository contains the SDK-owned history and the clean framework
extraction pin `27c7701e1eb116db1076da7f84bb68898544a44c`. The protocol crate is
framework-free, the Host resolves that exact upstream revision, and accepted
external contract artifacts are separated from full Rust inventory telemetry.
The remaining release status is governed by the language and integration gates;
this source repository does not publish a bundled Host or runtime.
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
