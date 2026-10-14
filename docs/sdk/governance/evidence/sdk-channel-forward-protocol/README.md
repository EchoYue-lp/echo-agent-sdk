# Channel forward protocol source candidate

Scope: six approved channels-family operations from ADR 0035 and the Channel handler forward design. Framework owns the original entry/handler lifecycle. Host owns resource and stream adapter admission. Protocol owns only the closed operation names and typed wire contract. The existing FeatureOperationRequest, ChannelInboundMessageWire, ChannelOutboundMessageWire, WireHandle and WireValue are reused; no fourth handler, alternate wire method, DTO registry or execution authority is introduced.

`facade.rs` adds handler.handle/handle_stream/reply and stream.next/cancel/close. Typed shapes use the existing DTO-derived JSON schema, restricting attachment kinds and STANDARD canonical padded base64 only within the forward admission contract. Reverse DTO acceptance is unchanged. Manager arguments require facade_resource handles, control arguments and returned stream handles require stream kind. The existing FacadeStreamEvent remains a WireValue variant with stream and positive sequence; item/failed have value, complete/cancelled do not. Outbound item schema is included in the signature contract metadata.

`schema.rs` supplies six valid request fixtures using the same production all_fixtures/export chain, original Session owner handles, complete manager/stream handles with generation 7, Unicode messages, maximum u64 timestamp and binary attachments. Generated JSON must only be emitted by the existing export_schema producer.

`extension_contract.rs` adds three source tests: six exact operations and typed input/digest behavior; six canonical owner-fenced fixtures; existing stream result shapes. Inputs test missing/null/unknown required fields, wrong handle kind, argument count, u64 overflow/noncanonical text, all attachment kinds, optional missing/null/empty filenames, invalid STANDARD/trailing bits and attachments null. Results test stream kind, null reply, boolean controls and exact stream type/terminal shape.

The phase in `source-pre-generation-manifest.json` is deliberately pending compile/generation. `rustfmt --edition 2024` on the three files and `git diff --check` passed. The first authorized focused Cargo command exited 101 before compilation because `--locked` refused a required Cargo.lock update; `protocol-focused.log` preserves the actual diagnostic. Per the parent instruction, no lock mutation or fallback command was attempted. Generated artifacts have not been changed by this slice.

Required next commands, in the parent-owned Cargo window:

1. `cargo test -p echo-sdk-protocol --test extension_contract channels_forward --locked`.
2. The unique staged producer: `cargo run -q -p echo-sdk-protocol --bin export_schema --locked -- --wire-candidate-update`.
3. `scripts/export-language-sdk-catalog.sh`, then `scripts/export-language-sdk-catalog.sh --check`.
4. Applicable protocol/fixture/schema contract tests and generator consistency checks, followed by true-Host language consumers once the new framework API and Host adapter are ready.

The staged producer intentionally skips rustdoc discovery and retains committed inventory/source scope. It cannot replace the final full `--update`/`--check` after the published framework successor pin. Formal API docs and examples are parent/Host-consumer lane work; website applicability is recorded by that delivery lane. No completion or main delivery is claimed here.

## Validated staged candidate

The parent repaired the scoped framework YAML dependency lock and reissued the exclusive window. `protocol-focused-retry.log` records 3/3 new channels tests passed with `--locked`. `wire-candidate-producer.log` records the **existing single producer** completing `--wire-candidate-update`, 111 artifacts emitted from the committed inventory and accepted source seed. `language-catalog-update.log` and `language-catalog-check.log` record the existing language projection update and drift check passing. `extension-contract-tests.log` records all 39 extension tests passing, zero ignored. The current six typed operation signatures and 113 generated/shared artifact hashes are in `candidate-manifest.json`, together with three source hashes and both original producer inputs.

Only the staged schema/fixtures/catalog/source-digest candidate is complete here. The parent owns the successor framework pin, full inventory `--update`/`--check`, new Host compilation and three native language consumers. The previous frozen Host will correctly fail the new digest handshake and is not used as new consumer proof. Cargo execution was released to the parent immediately after these commands reached actual terminal success.
