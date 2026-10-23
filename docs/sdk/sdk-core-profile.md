# echo-agent SDK core profile (`_echo_agent/*`)

The core profile is the first negotiated extension profile of the
echo-agent SDK Host. On one standard ACP v1 connection it exposes the
framework's Agent / Session / Run object model with typed JSON-RPC methods,
generation-fenced handles, full `EventEnvelope` streaming, bounded live
delivery with ACK, durable replay and restart recovery.

> **Governance candidate.** Rust Host and source-built TypeScript, Python
> and Java clients implement this profile. The current lifecycle and recovery
> changes still require final contract, real-Host language and exact-main
> acceptance; historical passing tests do not prove the candidate complete.
> Cross-language compatibility covers the explicitly accepted external
> contract, not every Rust public identity. See the
> [governance evidence matrix](governance/behavior-evidence-matrix.md).

## Enabling the profile

The Host serves the core profile only when all of the following hold:

1. The Host binary was built with the `sdk-core-profile` feature:
   `cargo build -p echo-sdk-host --features sdk-core-profile`.
2. The Host configuration carries an explicit `sdk_profile` section with an
   **absolute** `state_root`. The Host never discovers a state root from the
   working directory, home, `.env` or product configuration.
3. The Client publishes a hello under `clientCapabilities._meta.echo_agent`
   during `initialize`, and every field matches the Host advertisement
   published under `agentCapabilities._meta.echo_agent`.

```jsonc
// host configuration (excerpt)
{
  "schema_version": 1,
  "default_agent": { /* model + agent settings, as the standard profile */ },
  "sdk_profile": {
    "state_root": "/var/lib/echo-agent-sdk-host",   // absolute, created on demand
    "limits": {
      "max_frame_bytes": 1048576,          // one newline-delimited stdin frame
      "max_event_bytes": 1048576,          // one serialized live notification
      "max_outstanding_live_events": 128,  // ACK window before one gap + pause
      "max_replay_events": 512,            // per replay request
      "max_replay_bytes": 8388608,         // per replay request
      "max_open_handles": 512,             // open Agent/Session/Run/Stream handles
      "shutdown_timeout_secs": 5
    }
  }
}
```

Without `sdk_profile` the Host is standard-only: it never advertises the
extension and every `_echo_agent/*` request answers with the official
JSON-RPC `method not found` (-32601). A standard ACP Client is unaffected.

## Negotiation

- **Advertisement.** During `initialize` the Host publishes an
  `EchoAgentCapability` under `agentCapabilities._meta.echo_agent`: the
  extension protocol version, the extension `contract_digest` and
  `source_contract_digest` (the sha256 of the accepted external contract
  artifacts; full Rust inventory is telemetry only), the compiled leaf features,
  the declared capability families
  and the resource limits.
- **Hello.** An SDK Client publishes an `EchoAgentClientHello` under
  `clientCapabilities._meta.echo_agent` with the same version/digest pair
  plus its required features and capabilities.
- **Decision.** Extended mode is entered only when version, both digests,
  every required feature and every required capability match. Any mismatch
  degrades the connection to Standard **without failing `initialize`** and
  without creating handles — the SDK Client fails closed on its side when it
  does not see an acceptable advertisement.
- All extension data lives inside `_meta`; standard ACP fields are never
  modified (ACP extensibility rules).

## Capabilities delivered

| Family | Capability | Methods |
|---|---|---|
| Agent lifecycle | `agent_lifecycle` | `_echo_agent/agent/create`, `/describe`, `/close` |
| Session handles | `session_handles` | `_echo_agent/session/create`, `/load`, `/close` |
| Runs | `runs` | `_echo_agent/run/start`, `/get`, `/wait`, `/cancel`, `/steer`, plus the `_echo_agent/event`, `_echo_agent/event/ack` notifications |
| Event replay | `event_replay` | `_echo_agent/run/replay`, plus the `_echo_agent/gap` notification |

Task graphs, Subagents, structured output and feature operations are added
by `sdk-facade-adapters`; reverse calls are added by `sdk-extension-bridge`.
They are advertised only when their handlers and required features are
compiled. A core-only build does not advertise those additional surfaces.

## Object model and handles

- `agent/create` binds either the Host default definition
  (`{"variant":"host_default"}`) or an explicit versioned config projection
  (model + agent settings, with a mutually exclusive inline/env credential
  source). A handle references the immutable **definition**; every Session
  still constructs its own independent framework Agent.
- `session/create` mints a Session over an agent definition. The response
  also carries the ACP Session id, so the same Session is addressable by
  standard `session/prompt` / `session/cancel` — both entries are the same
  object with one run slot.
- `run/start` begins one chat or execute run and returns a **Run handle**
  plus a **Stream handle**. Runs execute asynchronously; `run/wait` waits
  bounded for the single authoritative terminal.
- Handles are `{id, generation, kind}`. Validation order is fixed:
  shape → kind → generation → issued/closed. A never-issued id is
  `invalid_value`, a pre-restart generation is `stale_handle`, a released
  id is `closed_handle`. Handles never rebind.
- `agent/create` with an `idempotency_id` returns the same handle for the
  same canonical config and a typed conflict for a different one.
- `session/create` and `run/start` retain a keyed publication within the
  current connection and handle generation, scoped to the operation and its
  original Agent or Session owner. Identical validated payloads replay the
  full original response, including after a Run completes; different payloads
  return `invalid_request` before new effects. A genuinely closed owner or
  published resource returns `closed_handle`, and that key cannot recreate it.
  Client-side `RunHandle.close()` alone does not retire the Host Run.
  A successful parent `SessionHandle.close()` disposes its client-owned child
  subscriptions, including a delayed start response. Buffered facts are retained
  before local EOF/completion; that boundary never synthesizes a business
  terminal or receipt. Failed parent close keeps the local scope retryable.
  The existing Host tombstones and post-close replay rules are unchanged; see
  [ADR0041](../adr/0041-session-close-local-child-scope.md).
- The first keyed job retains its existing Host settlement owner when an RPC
  observer is dropped. Cancellation before effect admission releases the key;
  after admission the original result/cleanup settles without a second
  execution. Duplicate observation cancellation affects only that waiter.
  Entries are bounded and are not silently evicted. Connection teardown fences
  admission and settles original jobs before discarding these mappings; keyed
  replay across connections or process restarts is not promised. See
  [ADR 0037](../adr/0037-session-run-idempotent-publication.md).

## Standard ↔ extension bridging

In Extended mode the standard responses carry the extension handles too:
`session/new` responses get `_meta.echo_agent.session`, and prompt turns get
`_meta.echo_agent.run` + `stream`. A standard Prompt and an extension
`run/start` share the same run ids, the same cancellation authority and the
same event ledger — `_echo_agent/event` notifications are also emitted for
prompt turns in Extended mode.

## Events, ACK and replay

- Every accepted framework `EventEnvelope` (identity, sequence, content
  hash, parent link preserved verbatim) is committed to the run's durable
  journal first, then delivered as the standard `session/update` projection
  and as a `_echo_agent/event` notification. Exactly one terminal event ends
  a run; terminals come only from the framework.
- The Client acknowledges consumed cursors with `_echo_agent/event/ack`.
  Receiving a notification or enqueueing an outbound frame is not proof of
  application consumption. Live iterators/publishers ACK at their consumption
  boundary; gap recovery requires an explicit ACK after applying the snapshot
  and consuming the returned replay page. Fetching replay never ACKs it.
- While the outstanding (un-ACKed) count/bytes stay inside
  `max_outstanding_live_events` / `max_event_bytes`, delivery is live. At
  the bound the Host sends **one** `_echo_agent/gap` notification and pauses
  live delivery. The Client recovers with `run/replay` from its cursor and
  then ACKs; live delivery resumes after the acknowledged cursor. Host
  memory stays bounded regardless of consumer speed.
- `run/replay` is bounded by `max_replay_events` / `max_replay_bytes` and
  addressed by a current-generation stream handle. Replays below the journal
  retention floor return a typed gap with a snapshot watermark. A replay
  response is validated against the contract before it is sent. Successful
  publication records the returned page as announced, so its consumed cursor
  can be ACKed; failed publication must not advance that bound.
- Snapshot ACK advances the continuation floor monotonically. Live
  continuation starts after the greater of that floor and the sent cursor;
  events covered by the applied snapshot are not resent. Duplicate or older
  ACKs cannot move the floor backward, and ACKs beyond announced live,
  snapshot or replay facts are rejected without resuming delivery. Explicit
  historical replay still honors its requested cursor and journal retention.
  This is the Issue #120 boundary,
  covered by the three language `GapAckHost` scenarios; final candidate
  acceptance remains recorded in the governance evidence matrix.

## Persistence, recovery and shutdown

The state root layout:

```text
<state_root>/runtime_state/     framework Agent checkpoints (FileRuntimeStateStore)
<state_root>/journals/<run>/    one segmented journal per run (full events)
<state_root>/host/generation    monotonic Host generation counter
<state_root>/host/session-index.json  Session identity, Agent binding and lossless cwd
<state_root>/host/run-index.json  run identity/status/terminal records
```

- Each start advances the Host generation; the process-exclusive state-store
  lease prevents two live Hosts on one root.
- `session/load` re-binds a persisted session id at the **current**
  generation: fresh Session/Run/Stream handles, historical runs reported
  from the index, journals replayable. The framework resumes the Agent from
  its committed checkpoint; completed tool facts are not re-executed.
- A run that was active when the previous process died is recovered as
  `status: "interrupted"`: `run/get` shows no terminal and no receipt,
  `run/wait` answers the typed `host_exited` error. The Host never revives
  the interrupted driver and never fabricates a completed terminal. New runs
  continue from the framework's committed checkpoint only.
- Shutdown order (stdin EOF and clean close share it): stop run admission →
  cancel active runs → bounded wait for receipts → flush journals/index →
  close Session Agents → release handles. Timeout or cleanup failure is an
  error, not a fabricated terminal or a successful release. Failed close
  retains the cleanup owner for retry; successful repeated close is idempotent.
- Rust embedders can retain a `HostCloseOwner` for the owned Host entry points.
  Its close path keeps the official connection actor polling while framework
  drivers and callbacks settle. This is a Rust/Host ownership facility, not
  a new language wire operation. The active-Run manual-close fix is part of
  the current candidate and needs final validation before a completion claim.

## Errors

Framework-backed feature failures retain their canonical category, terminal
kind, cause code, retryability and optional HTTP status in bounded error
details. Cancellation is `cancelled`; a framework timeout is `framework_error`
with a `timed_out` cause. The operation remains non-retryable: a retryable cause
does not authorize repeating an operation with potentially committed effects.

`_echo_agent/*` failures use one fixed JSON-RPC server-error code
(`-32050`); `error.data` carries the bounded `EchoSdkError` envelope
(`code`, `message`, `retryable`, optional `operation`/`handle`/`details`).
Malformed JSON, bad params and unknown methods keep returning the standard
JSON-RPC errors from the official runtime. The closed error-code list and
bounds live in `echo-sdk-protocol` (`error.rs`) and the generated schema
(`contracts/sdk/schema/echo-agent-extension-v1.schema.json`).

## Transport discipline

stdin frames are byte-counted before the official JSON-RPC parser sees them;
a frame larger than `max_frame_bytes` fails the connection with a bounded
stderr diagnostic and no business side effect. stdout remains the single
official ACP writer; diagnostics go to stderr and never contain credentials.


## Relation to the extension bridge

The extension bridge ([sdk-extension-bridge.md](sdk-extension-bridge.md))
is a separately compiled, separately advertised capability over this same
core profile: it registers host-language trait implementations and calls
them back through the same connection-scoped invocation authority. The
bridge changes nothing about core profile semantics — standard
initialize/session/prompt, handle ladders, ACK-gated live delivery and
replay behave identically with or without it.

## Relation to the facade feature adapters

The facade feature adapters
([facade-feature-adapters.md](facade-feature-adapters.md)) are another
separately compiled layer (`sdk-facade-adapters`) over this same core
profile: they add the task/subagent/structured-output families and the
feature-family operation surfaces on top of the core handles and
events. Core profile semantics — handle ladders, ACK-gated delivery,
replay and recovery — stay identical with or without them.


### Accepted Security source continuity

The accepted Security defining sources are `echo_core::utils::retention::{SecretMatch,
contains_secrets, redact_secrets, scan_secrets}`. The `echo_agent::security` Rust
reexports and the existing three `echo_agent::security::*` wire operation names
remain supported with the same result shape and error contract. Source signature
digests are generated for the current framework; consumers use the matching
catalog and Host build. See [ADR 0034](../adr/0034-sdk-governance-authority-and-lossless-adaptation.md#accepted-source-continuity-decision-2026-10-15).

The former public `TaskSubagentContext` fields `run_id`, `delegation_policy` and
`waived_dependency_ids` are retired accepted identities. Their native behavior
continues through existing read-only Rust/Host getters and the Task adapter;
this does not expose those getters as language APIs. The obsolete snapshot owner
fields `AgentRunSnapshot.skill_curator` and `ToolRuntime.plan_state` are retired;
Skill mutation authority and the revisioned TaskRun graph remain the respective
single owners. Existing Skill, Task and Plan capabilities continue through their
current routes.
