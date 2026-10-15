# ADR 0034: SDK Governance Authority and Lossless Adaptation

- Date: 2026-10-12
- Scope: SDK protocol, Host, clients and framework consumer boundary.

## Status

accepted

## Implementation authority

Implementation is authorized by the full-governance goal. The current Host
build pin is published framework `00782b8aeddefe2ea8c566922cbdee6aedc80d70`.
Runtime evidence remains bound to its actual source cut; full accepted-source
continuity generation, remaining behavior closure and SDK main acceptance are
pending. On 2026-10-15 the user explicitly accepted the twelve bounded compatibility
choices in the [source continuity decision](../sdk/governance/accepted-source-continuity-proposal.md).
The exact dispositions below authorize only this SDK slice. The user separately
approved physical-settlement option B and third-repository tool repairs; their
independent upstream writers own those changes. Physical-settlement B is still
pending upstream delivery. Tool A/B subsequently reached published main c0bc306;
SDK action/continuity integration remains the parent task's responsibility. This
slice keeps the original 22e validator and does not implement B, merge main or
clean historical archives.

## Context

At the 2026-10-12 opening snapshot, the independent SDK
`30d7e219cc7d3aef0f75a19217c82898c363f50d` had migration/build evidence but open
behavioral baselines, pinned framework `27c7701e1eb116db1076da7f84bb68898544a44c`,
and initially targeted `95ec66c6083a8feb17a1c28de0d01e1a24ca8a28`.
Those are historical starting facts. Framework repairs have since been delivered
independently and the current exact pin is 00782; build provenance does not
itself close the SDK compatibility or behavioral audit.

Source audits found missing replay ACK coverage, teardown ownership, exact Task
and Delivery claim fencing, managed Store facts and factory-instance routing.
An API inventory or successful catalog lookup cannot prove these behaviors.

## Alternatives

1. Relax client sequence/error checks and fill missing adapter fields with
   defaults. This hides lost facts and cannot satisfy accepted contracts.
2. Add SDK schedulers, cancellation state machines, persistence or receipt
   ledgers. This duplicates framework authorities and is rejected.
3. Keep framework business authority, extend necessary generic framework
   mechanisms there, and repair thin SDK adapters and transport ownership.

## Decision

Choose option 3. Every capability has explicit entrypoints, state authority,
lifecycle, failure scenarios and snapshot-bound verification. SDK-owned formal
documentation and semantic records remain in this repository. Historical
framework Findings retain their IDs, Issue links and immutable provenance.

Compatibility remains the explicitly accepted cross-language contract. Full
Rust public inventory, intrinsic membership, scope counts and implementation
revision are telemetry/build provenance. New source identities do not become
language commitments merely because generation classifies a route as remote.

### Replay and consumed facts

The framework Journal remains the only durable event authority. Host delivery
state distinguishes successful transport enqueue, announced ACK-able coverage
and the consumer's acknowledged cursor. A gap is not proof of processing;
clients explicitly process snapshot/replay before acknowledging its coverage.
Successfully returned replay facts may grant ACK coverage; an empty page and a
caller-supplied future cursor do not. Resume is strictly after the accepted
cursor. Tests must account for the durable tail and unique terminal, including
multiple pages and windows, rather than only compare against the first gap.

### Resources and closing

Keep owner/generation relationships explicit. Close signals admission closure,
settles tracked work, then invalidates handles. A failed or cancelled close
retains a retry owner; dropping a waiter is not proof that cleanup finished.
The Host composes framework ACP/MCP cleanup handles instead of duplicating
their state machines. Rust embedding callers can retain a Host close owner
across cancellation of the serving future. This is a Host/Rust-only API.

The retained Host owner also keeps the single official connection actor
running while framework close cancels active work and awaits its receipt.
Stopping connection polling first would prevent the actor's own Run driver
from settling. Cancelling a caller's serving waiter therefore does not drop
the actor; successful framework cleanup stops and joins it, while failed
cleanup preserves the same actor and close owner for retry. This changes
ownership of the existing transport future, not Run execution authority.

The convenience `run_stdio` entry point cannot return a retry owner when its
future is dropped. Its cancellation guard therefore keeps that same owner and
continues cleanup, including after the first configured close deadline fails.
Attempts use capped asynchronous backoff and the framework's existing close
operation; they do not rerun a Run or any tool side effect. Cleanup does not
wait for transport EOF: a cooperative resource may settle while the client
keeps its connection open. Failure diagnostics report the first failure,
changed failures and sampled unchanged failures. A permanently uncooperative
resource retains pending cleanup ownership until it actually settles or the
embedding runtime terminates; no finite retry count or fabricated terminal
receipt is promised.

### Factory instances

The existing Host-minted child `WireHandle` is the sole instance identity.
Before reverse `factory_create_agent`, reserve this identity in the unified
registry and include it in typed input. The client can bind the created object
to that precise child, including concurrent factories returning equal names.
Successful creation binds the descriptor and publishes the same handle.

Child reverse calls additionally carry a typed `factory_origin` handle.
Clients resolve only an already registered, matching factory and child
generation; they never route arbitrary unknown handles to a default factory.
Failed creation, cancelled waiters and publication failures retain or settle
the reserved child's cleanup. Parent unregister cannot discard live children.
Closing one child does not remove its siblings or the parent implementation.

### Managed Store and complete conversion

Bridge only the managed Store operations needed by real framework consumers
or existing accepted APIs. Preserve declared atomic/revisioned/absolute-deadline
capabilities, scope/generation/revision, receipts and outcome-unknown errors.
Missing capabilities reject before side effects. Opaque application payloads
use the existing lossless value encoding; framework identity and transaction
facts retain typed fields. New APIs without consumers remain explicitly
deferred or Rust/Host-only rather than being silently accepted.

Delivery operations accept the caller's complete exact claim, including
attempt identity and `claimed_at`. They never reconstruct a newer claim from
message ID. The required argument shape participates in the operation digest.
Workflow exact-resume checkpoints retain their object shape. Host rejects
positional struct arrays before calling the framework's canonical claim
authority; it does not introduce a second checkpoint validator or state machine.
Explicit wire maps distinguish domain `kind` fields from wire discriminators,
and wire integer tags preserve canonical numeric field types on re-entry.
A formerly public Curator snapshot save, now private upstream, cannot be
reimplemented as blind file writes; its accepted disposition must explicitly
record the replacement by public Curator mutation authority.

### Task backend execution admission

A backend may provide accepted Task CRUD without supporting execution CAS.
The framework Store and service expose a side-effect-free execution capability;
unsupported execution fails before loading, claiming or dispatching tasks.
The existing Task Store component descriptor explicitly declares
`execution_cas_v1`, defaulting to false. Host checks the same Session service
before accepting `task/execute`. It must not probe capability with a mutation,
fallback to another Store, or return success while logging an unobservable
unsupported-runtime failure. Backend code only performs atomic persistence;
framework services continue to own claim, revision, retry and settlement.

## Industry evidence and local tradeoffs

The official LSP diagnostic contract requires optional numeric severity and
integer-or-text codes. The existing accepted Diagnostic projection must retain
those facts; absent severity cannot be reconstructed as Error, and an integer
code cannot become text. Completion's optional kind follows the same rule.
Framework owns decoding and the single URI diagnostic replacement/error cache;
Host and language clients only preserve the resulting values or typed failure.

The following six exact source dispositions were explicitly approved: accept
the two TaskExecution fields and the three supporting DiagnosticCode identities,
and retire CuratorState.save. Their Action/Kind/Source anchors remain valid under
this accepted ADR. They do not approve neighboring public identities or full-inventory language
APIs. The separate twelve-item continuity choices were explicitly approved on
2026-10-15 as recorded below; generation and consumer validation remain required.

### Accept Task result field

- Action: accept
- Kind: struct_field
- Source: echo_orchestration::tasks::runtime::TaskExecution::result
- Reason: preserve the authoritative result committed with Completed, including absent versus explicit null
- Compatibility: expose the field through the existing accepted Task CAS and dependency-result contract
- Rollback: restore the prior framework revision and accepted contract together

### Accept Task interruption field

- Action: accept
- Kind: struct_field
- Source: echo_orchestration::tasks::runtime::TaskExecution::interruption_requested
- Reason: preserve the accepted exact cancellation or pause across durable Task execution
- Compatibility: carry this fact through the existing Task graph contract without another adapter authority
- Rollback: restore the prior framework revision and accepted contract together

### Retire private Curator snapshot save

- Action: retire
- Kind: method
- Source: echo_agent::evolution::curator::CuratorState::save
- Reason: upstream made snapshot save private; SDK cannot duplicate Curator persistence with blind writes
- Compatibility: the old source route returns FeatureUnavailable; existing public Curator authority retains supported mutation and inspection
- Replacement: public Curator mutation and inspection; explicit FeatureUnavailable for the retired direct snapshot-save operation
- Rollback: restore the previous framework revision and its complete accepted contract

DiagnosticCode's closed Integer(i32)/String value is a supporting representation
of the already accepted Diagnostic.code field. Current format-61 rustdoc from
published framework `b9099558620f5f9693c736c85c214db14f62a8cb` confirms its
canonical defining enum in `echo_core::lsp::types`, and the two named variants
at the same defining source. The exact observation is retained in
`docs/sdk/governance/evidence/sdk-final-framework-contract/diagnostic-current-rustdoc-observation.json`.
Parser helpers and recovery reservations remain Rust/Host-only. Existing
accepted Completion.kind, Diagnostic.severity and Diagnostic.code identities
retain membership while corrected optional/type signatures enter the blocking
contract digest. The following three supporting identities were approved. This ADR does not
itself write the accepted seed; the unique generator's complete output still
requires generation and validation against the final framework pin.

### Accept Diagnostic code enum

- Action: accept
- Kind: enum
- Source: echo_core::lsp::types::DiagnosticCode
- Reason: retain the protocol integer-versus-string distinction of the already accepted Diagnostic.code field
- Compatibility: expose the existing closed Integer(i32) and String(String) value through the accepted lossless diagnostic projection
- Rollback: restore the previous framework revision and its complete accepted diagnostic contract together

### Accept Diagnostic integer code variant

- Action: accept
- Kind: variant
- Source: echo_core::lsp::types::DiagnosticCode::Integer
- Reason: preserve signed LSP integer codes without coercing them to text or losing negative values
- Compatibility: support only the integer branch of the already accepted Diagnostic.code field
- Rollback: restore the previous framework revision and its complete accepted diagnostic contract together

### Accept Diagnostic string code variant

- Action: accept
- Kind: variant
- Source: echo_core::lsp::types::DiagnosticCode::String
- Reason: preserve textual diagnostic codes, including strings whose characters spell a number
- Compatibility: support only the string branch of the already accepted Diagnostic.code field
- Rollback: restore the previous framework revision and its complete accepted diagnostic contract together

[Cap'n Proto RPC](https://capnproto.org/rpc.html) treats object/interface
references as first-class RPC results with connection-bound lifetimes. The
SDK uses its existing generation-fenced handles and precise origin binding,
instead of introducing another instance ID or guessing by names.

[Tokio graceful shutdown](https://tokio.rs/tokio/topics/shutdown) separates
signalling shutdown from waiting for tracked tasks to finish. The Host follows
that ownership ordering and keeps failed cleanup addressable. It does not add
online-service permission gates to this local SDK.

Framework Task repair separately records the reusable persistence and scoped
cancellation choices, including Temporal cancellation scopes and LangGraph
checkpointed task results. SDK consumes those primitives; EKO reviewer,
worktree and UI policies remain application concerns.

## Consequences and verification

Factory creation input, reverse-call origin, managed bridge DTOs and Delivery
claim shape change accepted protocol facts. Regenerate schema, fixtures,
accepted artifacts and language catalogs through the unique generator, update
clients/examples and verify mismatched contracts fail explicitly.

Required evidence includes red/green counterexamples; real Host scenarios in
TypeScript, Python and Java; owner retry and publication rollback; expired
deadlines with no commit; committed response loss and receipt reconciliation;
Task sibling isolation; Journal identity/recovery; independent final review;
applicable Rust, contract, language and CI gates; and exact main acceptance.
The governance documents retain open status until those obligations are met.
Package publication remains a separate task.

### Channel resource admission and close

The framework owned Channel operation retains the original plugin and cleanup
entry; it does not grant permission to start after a Host resource address has
closed. Releasing the manager registry guard alone leaves a captured future
able to poll after resource cleanup and restart an unaddressable plugin.

Reuse the existing `FacadeResourceRecord.lifecycle` authority as a read/write
admission guard. Channel register/start/stop/send/health operations retain a
shared permit through their original framework operation. Resolution checks
owner/generation/closing before waiting, then checks the same HandleRegistry
again after obtaining the permit. Resource close takes the exclusive permit,
waits for admitted operations, marks closing and settles the original manager
before tombstoning the address. Queued operations that acquire a permit after
close must return `closed_handle` without executing a plugin. A failed close
keeps closing true and the same retry owner; it does not reopen admission.

`ids` and `list` read registration metadata without the operation permit, so
plugin callbacks may await those reads. Once close has marked the resource or
Session closing, those callbacks get the existing `closed_handle` response.
A callback that awaits another mutation or close of its own resource is not a
supported recursive operation: an exclusive close queued ahead of the nested
operation can otherwise wait for the outer permit. The accepted metadata
reentry guarantee does not promise recursive lifecycle execution.

A second Channel registry/state machine, an SDK copy of framework rollback,
and a mutex that serializes all healthy Channel operations were rejected. The
latter would prevent independent B operations while A is parked. This change
belongs to Host handle/resource admission; generic plugin/handler lifecycle
and captured instance semantics stay with the framework. No inbound or
handler-close reverse wire is added.

The deterministic regression uses an actual registered native ChannelPlugin,
the real framework owned start future and `SessionFacadeRuntime::close_resource`.
It records plugin side-effect counters and resource tombstones; it is a Host
unit composition test, not a sixth three-language public callback scenario or
proof of operating-system process cleanup. Published dependency and full
review/gate evidence remain prerequisites for delivery.

## Custom Agent invocation cancellation and callback ownership

The CustomAgent adapter overrides the existing framework cancellation and invocation-context entry points. It passes the canonical `AgentInvocationContext` invocation token directly to the original reverse call; another active Run of the registered Session cannot replace that token. The existing invocation envelope carries the registered Session identity and the original run, turn, message and execution correlation fields. The complete Rust context remains retained with the returned stream, including opaque resource guards and workspace ownership. Working directory, history, runtime-state identity and policy objects are not new language wire fields: the accepted CustomAgent request remains text input plus the existing diagnostic envelope, and this change does not claim to project those additional fields or add multimodal support.

The CustomAgent stream contract already permits a setup Stream response before later stream notifications. Its retained callback owner therefore waits for both the original RPC response and a validated remote terminal, in either order. Cancellation sends the existing cooperative notice to that exact invocation. A local timeout or backpressure failure does not become evidence that the remote producer finished. A connection-owned collector retains the original callback lease and sink after caller drop until those facts or actual incoming connection closure, while the framework turn driver remains the only authority for terminal receipts, cancellation grace and workspace retention.

This differs from the newly approved Channel forward callback, whose callback return without a terminal is explicitly an invalid stream flow. The two private adapter policies share transport ownership and the existing sink sequence ledger. They do not change Tool, LLM, factory scheduling, task scheduling or other common bridge callers, and do not add a wire operation or a second lifecycle authority. A language callback's declared terminal does not cover work the user detaches outside the callback/stream contract or cleanup deliberately performed after that declared terminal.


## Accepted source continuity decision (2026-10-15)

Human approval: the user explicitly accepted item 1, the exact seven Security
source migrations and five field identity retirements. No new language API,
framework state owner or getter acceptance is implied. Existing Curator,
TaskExecution and DiagnosticCode dispositions above remain in force.

### Continuity retire 1

- Action: retire
- Kind: struct_field
- Source: echo_agent::agent::snapshot::AgentRunSnapshot::skill_curator
- Reason: old public field identity no longer represents the canonical framework authority
- Compatibility: retire only the obsolete snapshot field identity; preserve the existing Skill or Task/Plan capability without a second authority
- Replacement: SkillMutationAuthority and its ChangeLog; obsolete Curator-only owner unsupported
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity retire 2

- Action: retire
- Kind: struct_field
- Source: echo_agent::agent::snapshot::ToolRuntime::plan_state
- Reason: old public field identity no longer represents the canonical framework authority
- Compatibility: retire only the obsolete snapshot field identity; preserve the existing Skill or Task/Plan capability without a second authority
- Replacement: revisioned TaskRun graph; Plan artifact and Todo projection; obsolete plan_state owner unsupported
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity retire 3

- Action: retire
- Kind: struct
- Source: echo_agent::security::SecretMatch
- Reason: canonical defining source moved to core retention; facade remains compatible
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Replacement: echo_core::utils::retention::SecretMatch
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity accept 3

- Action: accept
- Kind: struct
- Source: echo_core::utils::retention::SecretMatch
- Reason: one-to-one defining-source replacement of echo_agent::security::SecretMatch
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together

### Continuity retire 4

- Action: retire
- Kind: struct_field
- Source: echo_agent::security::SecretMatch::matched
- Reason: canonical defining source moved to core retention; facade remains compatible
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Replacement: echo_core::utils::retention::SecretMatch::matched
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity accept 4

- Action: accept
- Kind: struct_field
- Source: echo_core::utils::retention::SecretMatch::matched
- Reason: one-to-one defining-source replacement of echo_agent::security::SecretMatch::matched
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together

### Continuity retire 5

- Action: retire
- Kind: struct_field
- Source: echo_agent::security::SecretMatch::position
- Reason: canonical defining source moved to core retention; facade remains compatible
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Replacement: echo_core::utils::retention::SecretMatch::position
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity accept 5

- Action: accept
- Kind: struct_field
- Source: echo_core::utils::retention::SecretMatch::position
- Reason: one-to-one defining-source replacement of echo_agent::security::SecretMatch::position
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together

### Continuity retire 6

- Action: retire
- Kind: struct_field
- Source: echo_agent::security::SecretMatch::secret_type
- Reason: canonical defining source moved to core retention; facade remains compatible
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Replacement: echo_core::utils::retention::SecretMatch::secret_type
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity accept 6

- Action: accept
- Kind: struct_field
- Source: echo_core::utils::retention::SecretMatch::secret_type
- Reason: one-to-one defining-source replacement of echo_agent::security::SecretMatch::secret_type
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together

### Continuity retire 7

- Action: retire
- Kind: function
- Source: echo_agent::security::contains_secrets
- Reason: canonical defining source moved to core retention; facade remains compatible
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Replacement: echo_core::utils::retention::contains_secrets
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity accept 7

- Action: accept
- Kind: function
- Source: echo_core::utils::retention::contains_secrets
- Reason: one-to-one defining-source replacement of echo_agent::security::contains_secrets
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together

### Continuity retire 8

- Action: retire
- Kind: function
- Source: echo_agent::security::redact_secrets
- Reason: canonical defining source moved to core retention; facade remains compatible
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Replacement: echo_core::utils::retention::redact_secrets
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity accept 8

- Action: accept
- Kind: function
- Source: echo_core::utils::retention::redact_secrets
- Reason: one-to-one defining-source replacement of echo_agent::security::redact_secrets
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together

### Continuity retire 9

- Action: retire
- Kind: function
- Source: echo_agent::security::scan_secrets
- Reason: canonical defining source moved to core retention; facade remains compatible
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Replacement: echo_core::utils::retention::scan_secrets
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity accept 9

- Action: accept
- Kind: function
- Source: echo_core::utils::retention::scan_secrets
- Reason: one-to-one defining-source replacement of echo_agent::security::scan_secrets
- Compatibility: retain echo_agent::security reexports, existing operation names, value shape and errors
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together

### Continuity retire 10

- Action: retire
- Kind: struct_field
- Source: echo_orchestration::tasks::runtime::TaskSubagentContext::delegation_policy
- Reason: old public field identity no longer represents the canonical framework authority
- Compatibility: retain native claim, waiver and delegation through the existing Host adapter; no direct field or getter language API
- Replacement: existing TaskSubagentContext read-only getter and Host mapping (Rust/Host-only)
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity retire 11

- Action: retire
- Kind: struct_field
- Source: echo_orchestration::tasks::runtime::TaskSubagentContext::run_id
- Reason: old public field identity no longer represents the canonical framework authority
- Compatibility: retain native claim, waiver and delegation through the existing Host adapter; no direct field or getter language API
- Replacement: existing TaskSubagentContext read-only getter and Host mapping (Rust/Host-only)
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation

### Continuity retire 12

- Action: retire
- Kind: struct_field
- Source: echo_orchestration::tasks::runtime::TaskSubagentContext::waived_dependency_ids
- Reason: old public field identity no longer represents the canonical framework authority
- Compatibility: retain native claim, waiver and delegation through the existing Host adapter; no direct field or getter language API
- Replacement: existing TaskSubagentContext read-only getter and Host mapping (Rust/Host-only)
- Rollback: restore the pre-slice accepted seed, generated contracts and matching framework pin together; never revert a single identity in isolation


### Continuity rollback boundary

The recoverable task undo point is SDK `a48854a792d146c823e89fcfa2237f2559290c15`.
Restoring this slice's complete source/accepted seed/generated files returns to
that intentionally blocked pre-migration candidate; it does not make its old
source-continuity gate pass. A runnable compatibility rollback must also restore
the matching framework revision and complete catalog/digest consumers as one
reviewed snapshot. Never restore only an individual retired field/seed key,
reintroduce obsolete runtime owners, or claim a pre-migration blocked candidate
is a verified release. No rollback, pin downgrade or merge is executed here.
