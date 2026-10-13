# Full generator acceptance authority

`contracts/sdk/accepted-external-contract.json` is the persisted source acceptance
seed. Full Rust inventory remains telemetry. A public item becoming routable, or
having language implementation status `done`, does not expand the accepted SDK
contract.

The full `export_schema` producer restores membership by the exact defining
source identity and item kind. This distinguishes identically named methods and
fields. Signature digests are not membership keys: a changed accepted shape
stays accepted and participates in the blocking source digest. Re-export aliases
retain the same source membership. Intrinsic Rust implementation specializations
remain visible in inventory without creating additional accepted commitments.

The complete Host operation catalog is generated from all inventory obligations.
The accepted catalog is generated from canonical accepted obligations and the existing
explicit protocol family table. A new deferred source operation changes Host
inventory, but does not enter the language catalog or accepted source digest.

## Explicit changes

Full `--update` accepts these repeatable, exact changes:

```text
--accept-source <item-kind> <defining-source-identity> <docs/adr/file.md#section>
--retire-source <item-kind> <defining-source-identity> <docs/adr/file.md#section>
```

Each reference must name an existing ADR with an approved `Status` section.
The referenced section must contain exactly one `Action`, `Kind` and `Source`
line matching the invocation, plus nonempty `Reason`, `Compatibility` and
`Rollback` lines. Retirement also requires `Replacement`, including an explicit
unsupported disposition when applicable. Mentioning an identity in prose,
referring to a draft, or reusing an acceptance decision to retire it fails.

For example, an approved ADR's section can state:

```text
- Action: accept
- Kind: struct_field
- Source: echo_orchestration::tasks::runtime::TaskExecution::result
- Reason: preserve the authoritative Task output
- Compatibility: expose the result through the existing Task CAS contract
- Rollback: restore the previous framework revision and accepted contract
```

The retained command log records each decision reference; the generated
accepted contract diff is the reviewable resulting scope. There is no separate
accepted registry. Read-only checks, telemetry mode and partial wire candidates
reject these changes.

An accepted source disappearing from current rustdoc fails before writes unless
its exact identity/kind is explicitly retired. An item-kind change also requires
a reviewed disposition. Retirement must document its replacement or explicit
unsupported behavior; absence alone is not proof that the commitment was retired.
New acceptance requires a current deferred identity. Rust/Host mechanisms cannot
be promoted by mistaking transport reachability for a language API.

The framework recovery reservation
`echo_agent::acp::AcpSession::reserve_recovered_turn_ids` remains Rust/Host-only.
Accepted Task execution fields, managed persistence DTOs and Tool effects must be
reviewed against their existing accepted behavioral contracts; they do not grant
acceptance to every neighboring runtime function or public field.

## Validation scope

The regression suite checks unknown routable identities, exact additions,
retirement failures before mutation, alias/signature fidelity, current accepted
seed reproduction, and the recovery reservation boundary. Full generation and
final `--check` still require the exact published framework target revision.
`--wire-candidate-update` cannot establish that final inventory or build proof.
The generator's accepted artifacts, examples, real Host scenarios and independent
review must agree before main acceptance.

This change does not add an Agent/runtime capability, so existing executable SDK
examples remain the consumer verification entry points. It changes SDK contract
production rather than framework/EKO runtime behavior; the website sync is only
needed if its public SDK contract-governance explanation exposes this workflow.

## Rustdoc build output directory

The full producer resolves `CARGO_TARGET_DIR` once against the SDK repository
root. An absolute value is used directly; a relative value is anchored to that
root; when the variable is absent, the directory is `<SDK root>/target`. The
resolved directory is passed explicitly as `--target-dir` for both the local
SDK package and external framework manifests, so changing Cargo's working
workspace cannot change a relative override's meaning. JSON is read from that
same directory's `doc/<target-name>.json`, rather than a stale default artifact.

This output-location fix preserves the pinned rustdoc toolchain, feature/profile
and per-run cfg identities and all acceptance decisions. A shared stable Cargo
cache does not itself prove nightly rustdoc artifacts already exist: full
extraction must execute and retain its actual command/result before generation
is called verified. No SDK runtime, new dependency or accepted API is added.
