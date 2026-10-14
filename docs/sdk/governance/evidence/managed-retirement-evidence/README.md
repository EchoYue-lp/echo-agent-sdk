# Managed exact retirement: accepted Host consumer evidence

2026-10-13 candidate evidence. Three native language suites passed two scenarios
each, with no skipped tests; backend checks passed 20 tests (the unchanged 13
existing tests plus seven retirement obligations). This evidence adds the eighth
real managed callback consumer without claiming closure of all 11 DTO operations.

Execution used the immutable real Host
`echo-sdk-owned-channel-host`, SHA-256
`7bbd1569367b0b1004fbe1cbc537b6869a2bc685bb1ca14e63228b1746aee3b2`.
All scenario directories are isolated beneath `lp-agent/.worktrees/admin/runtime/managed-retirement`.
Raw outputs and hashes are in [raw/receipt.json](raw/receipt.json), with the
observed dirty-source references and binary hash in
[raw/source-freeze.json](raw/source-freeze.json). These are focused candidate
checks on that binary, not the later final-snapshot Rust or merge gates.

## Actual forward consumer and authority

The public `SessionHandle.invoke` in TypeScript, Python and Java resolves these
already accepted canonical operations from the catalog:

- `echo_agent::state::clear_persisted_runtime_incarnation` reaches
  `echo-sdk-host/src/core_profile/facade/source_operations.rs:3914`, obtains the
  Session's live ConversationStore / RuntimeStateStore pair, and calls the
  framework clear helper. The framework settles any pending projection, loads
  the current generation, prepares its identity-bound retirement request and
  invokes `retire_runtime_generation_with_context` in `src/state/mod.rs:2588`.
  The existing Host adapter in `extension_bridge.rs:4295` forwards the typed
  request and absolute deadline to the registered language component.
- `echo_agent::state::delete_persisted_conversation` reaches the same live pair,
  then the framework compatibility entry point rejects a managed pair with
  `ManagedStateRequiresCas` before calling a backend mutation.

Framework remains the settlement, retry, Run and lifecycle authority. The
test-only backend owns only its existing atomic scope/checkpoint/projection
document and exact CAS proof. It neither schedules pending intents nor fabricates
framework results. It retains its default seven advertised operations; the
positive scenario explicitly opts into the existing retirement contract using
`--enable-retirement`. No public API, protocol schema, production Rust or language
client implementation changed in this slice.

## Six isolated native scenarios

Each scenario starts a real managed Agent turn so the framework creates the
scope, checkpoint, transcript projections and acknowledgements. The test never
invokes a reverse callback DTO as a substitute for the forward consumer.

| Language | Native scenarios | Unsupported clear mutation delta | Enabled clear mutation delta | Delete callbacks | Enabled restart generations |
| --- | ---: | ---: | ---: | ---: | --- |
| TypeScript | 2 | 0 | 1 | 0 | `1 → 2` |
| Python | 2 | 0 | 1 | 0 | `1 → 2` |
| Java | 2 | 0 | 1 | 0 | `1 → 2` |

The disabled scenario returns the backend's deliberate `Unsupported` through
the real forward clear path. All durable maps and counters remain byte-field
equivalent to the pre-clear observation. This is a negative capability fixture,
not a production bug or a failing final test.

The enabled scenario receives `checkpoint_removed=true`. It verifies the exact
runtime owner, scope, expected scope/state revisions, retirement operation ID,
deadline wire string, tombstone revision and retained receipt/projection fields:

- Generation version becomes `retired`; its checkpoint becomes `null`.
- Scope revision and generation revision each advance once. Scope lifecycle
  remains `active`; conversation epoch and transcript authority stay unchanged.
- Conversation messages, projection batches and committed projection receipts
  remain unchanged. Projected-item and duplicate-apply counters do not increase.
- A repeated canonical clear returns `checkpoint_removed=false` and makes no
  mutation. A new Host process loads the same scope with a new handle generation,
  returns the same no-op clear result and retains all durable facts. Only one
  retirement callback occurs, including across that real Host restart.

Managed compatibility delete is exercised in every scenario. It produces
`framework_error`, canonical operation identity and the existing framework
`runtime_state` category/code with `failed`, `framework_retryable=false`.
The message retains the original “Managed runtime state requires compare-and-save”
cause and stable managed-delete requirement. No more-specific wire error code is
invented: the framework currently maps these causes to `runtime_state`.
The tests compare both durable state and the entire callback log before/after
delete, proving zero backend calls as well as zero mutation.

## Backend checks and exact scope

The seven added backend checks cover atomic retirement and exact replay after
backend restart; the default Unsupported capability; stale CAS / wrong owner;
pending-projection fencing; tampered identity / expired deadline; concurrent
retries with one mutation; and exact `9007199254740993` revisions plus failed
publication without a partial tombstone. The replay test also proves stale saves
cannot resurrect a retired generation. These are backend obligations, separate
from the six native Host scenarios.

These three typed reverse operations remain explicitly Unsupported here:

| Reverse operation | Current accepted forward consumer evidence |
| --- | --- |
| `conversation_delete_managed_with_context` | Adapter is defined; the accepted compatibility delete intentionally rejects before this callback. No accepted stable-request forward delete was added. |
| `runtime_begin_scope_retirement_with_context` | Adapter is defined; exact generation clear does not call scope retirement. |
| `runtime_continue_scope_retirement_with_context` | Adapter is defined; exact generation clear does not advance a scope-retirement manifest. |

Definition/registration and DTO conversion for these operations do not constitute
real forward-consumer evidence. This slice does not close the broad managed-store
Finding and does not expand accepted forward APIs to manufacture coverage.

## Reproduction and recorded limits

Set `ECHO_AGENT_SDK_HOST` to the immutable binary above,
`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`, and
`ECHO_AGENT_SDK_RETIREMENT_RUNTIME_ROOT` to the isolated LP-local runtime root.
Then run:

```sh
# SDK repository root
python3 -m unittest discover -s sdks/shared/testing -p 'test_managed*backend.py' -q

# sdks/python; use interpreter invocation because relocation left old executable shebangs
PYTHONPATH=src .venv/bin/python -m pytest -q -s tests/test_managed_retirement_host.py

# sdks/typescript; current built public SDK dist
node --test test/managed-retirement-host.test.js

# sdks/java; exclusive Maven execution
mvn -q -Dtest=ManagedRetirementHostTest test
```

Python Ruff checks/format checks and JavaScript syntax check passed. The Java
Surefire XML was copied immediately after its final run; it records exactly two
tests, zero failures/errors/skips. Maven's existing SLF4J no-provider diagnostic
is preserved in the raw log. This focused run did not execute Cargo or claim full
repository gate completion.

Initial authoring errors were corrected: an assumed nonexistent specific wire
code was replaced with the framework's current generic code, TypeScript uses
the public `validateManaged` DTO validator, and Java compares reparsed JSON
counters to avoid `IntNode`/`LongNode` test equality differences. The initial
Java assertion-failure XML remains archived separately and is not counted as
final evidence. These were test authoring errors, not repaired production bugs.

Documentation is this SDK-owned evidence record. Examples and website require
no synchronized change because this slice changes test-only backend obligations
and consumer evidence without changing any public product behavior or API.

Evidence footer: TypeScript `2 passed / 0 failed / 0 skipped`; Python
`2 passed`; Java `2 tests / 0 failures / 0 errors / 0 skipped`; backend
`20 passed`. Native source operations observed: three languages ×
`(2 disabled + 4 enabled)` = 18 accepted invocations. The immutable binary and
each source/raw artifact digest are retained in the linked JSON receipts.
