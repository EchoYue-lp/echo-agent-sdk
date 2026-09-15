# echo-agent-sdk

Source-first TypeScript, Python, and Java SDKs for the Rust `echo-agent`
framework, connected through a source-built ACP Host and the
`_echo_agent/*` extension contract.

## Migration Status

This repository contains the SDK source-continuity checkpoint extracted from
`echo-agent`, including the SDK-owned history and the frozen framework source
at `c5f7688`. Independent build and compatibility closure remain deferred to
the next migration outcome. See [MIGRATION-SOURCE.md](MIGRATION-SOURCE.md) and
[ADR 0001](docs/adr/0001-sdk-repository-boundary.md) for the source boundary,
history, known gaps, and ownership decision.

The repository must not be described as independently runnable or parity
complete until its framework pin, protocol boundary, contract artifacts, Host
integration, and all three language gates have been completed together.

## Contents

- `echo-sdk-protocol/`: ACP extension wire values, schema, catalog, and contract tooling.
- `echo-sdk-host/`: source-built Rust Host adapting `echo_agent` to ACP and SDK methods.
- `contracts/sdk/`: canonical SDK schemas, fixtures, catalogs, and imported inventory artifacts.
- `sdks/`: TypeScript, Python, Java, and shared source SDK assets.
- `docs/sdk/`: SDK protocol, Host, extension bridge, and facade documentation.
- `scripts/`: contract and source-language verification entry points.
