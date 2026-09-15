# echo-agent-sdk

Source-first TypeScript, Python, and Java SDKs for the Rust `echo-agent`
framework, connected through a source-built ACP Host and the
`_echo_agent/*` extension contract.

## Migration Status

This repository currently contains the source-import checkpoint extracted
from `echo-agent`. The upstream framework is still completing semantic
governance, so independent build and compatibility closure are intentionally
deferred. See [MIGRATION-SOURCE.md](MIGRATION-SOURCE.md) for the exact source
revision, imported paths, known gaps, and the next convergence boundary.

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
