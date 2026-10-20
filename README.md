# echo-agent-sdk

Source-first TypeScript, Python, and Java SDKs for the Rust `echo-agent`
framework, connected through a source-built ACP Host and the
`_echo_agent/*` extension contract.

## Migration Status

The SDK-owned import and extraction history includes the original clean pin
`27c7701e1eb116db1076da7f84bb68898544a44c`. The current Host pins published
framework `3560812caede84998a1f6f26d7bf3fa81ea0486c` from
[framework PR #193](https://github.com/EchoYue-lp/echo-agent/pull/193), with its
exact squash tree and all seven exact-main CI jobs verified. The canonical
producer completed all 28 profiles and wrote 117 artifacts; the complete
contract check passed without blocking or telemetry drift. The 5,621 accepted
identity/kind keys and 454 accepted routes are unchanged. Only the accepted
`set_model` Rust signature follows its fallible result; new named framework
constructors and context-only fields are not automatically language APIs.
Current source, Host and gate qualifications remain tied to their actual cuts;
see [current acceptance](docs/sdk/governance/capability-review-remaining.md).
Exact SDK-main/platform CI and native Hook/Stop acceptance remain separate.

The previous `6563d26570cc4a8d93c86b7e37443b0559476807` checkpoint came from
[framework PR #191](https://github.com/EchoYue-lp/echo-agent/pull/191). Its
`RunEntry::wait_result` inventory entry remains Host/Rust-only telemetry; those
historical runtime results do not qualify the current successor source.

An earlier governance candidate resolved
`00782b8aeddefe2ea8c566922cbdee6aedc80d70` from
[framework PR #186](https://github.com/EchoYue-lp/echo-agent/pull/186).
It included the framework Git branch-switch repair; this SDK lock also selects
rustls 0.23.46. Signed source/tree, framework post-main CI and the SDK
[published-pin dependency/build checks](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-published-00782-pin/README.md)
have actual reviewed receipts. [Rust14](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-00782-final-rust-gates/README.md)
and the [fourth complete language run](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-ci-repairs-final/README.md)
passed locally with all 800 execution inputs recoverable in Git. Its Oracle
JDK21 result does not replace Linux CI Temurin17; new fixture/action revisions
still require actual remote CI and current-source semantic/main acceptance.
The earlier be5 [published-pin provenance](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-chart-published-pin/published-pin-provenance.json)
remains historical. The protocol remains framework-free, and full Rust
inventory remains telemetry. The [historical a634 Rust gates](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-a634-rust-gates/README.md)
and [complete language gate](https://github.com/EchoYue-lp/echo-agent-sdk/blob/0ebe15f66d0aa57c7b7a92ae68cd1be106114ae9/docs/sdk/governance/evidence/sdk-language-a634-reviewed-final/README.md)
bind their exact execution inputs; later runtime repairs require their own
verification and do not inherit those earlier passes.

The [twelve accepted-source continuity choices](docs/sdk/governance/accepted-source-continuity-proposal.md)
have been accepted and implemented through the canonical producer; their
authority is recorded in [ADR0034](docs/adr/0034-sdk-governance-authority-and-lossless-adaptation.md).
SDK main acceptance remains pending. Published dependency and passing bounded runtime checks do not by
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
