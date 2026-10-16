# SDK extension bridge (bidirectional)

The extension bridge lets a host language implement public `echo_agent`
framework traits and have the Rust Agent call them back over the same ACP
connection — with Rust semantics preserved under timeout, cancellation,
disconnect and generation races. It is negotiated as the
`extension_bridge` capability of the `_echo_agent` profile and compiled
only when the Host is built with the `sdk-extension-bridge` feature.
That feature also enables `sdk-facade-adapters`, because compressor callbacks
invoke their temporary Host tokenizer through the canonical facade route;
feature-surface advertisement therefore reflects both compiled handlers.

Status: implemented in the Rust Host and three source-built language SDKs.
Current governance changes to lifecycle, persistence and callback routing are
candidates pending final real-Host language validation and exact-main
acceptance. Compatibility is limited to the explicitly accepted external
contract; complete intrinsic membership is telemetry. Evidence and unresolved
items are maintained in the [governance matrix](governance/behavior-evidence-matrix.md),
not inferred from catalog `done` labels.

Build the Host from source with `cargo build -p echo-sdk-host
--features sdk-extension-bridge --locked`; this feature includes the core
and facade-adapter profiles. The Host does not bundle a language runtime or a
prebuilt artifact.

## Model

```text
framework trait call (Tool / LlmClient / Store / ...)
  -> thin proxy (echo-sdk-host)
  -> lease from the connection's ExtensionInvocationAuthority
  -> one typed _echo_agent/extension/invoke request (Host -> SDK)
  -> SDK dispatcher runs the host-language implementation
  -> result | stream | typed error
  -> proxy restores the Rust trait value; framework state machines continue
```

One connection owns one invocation authority
(`echo_agent::acp::ExtensionInvocationAuthority`): bounded concurrency
permits, per-invocation cancellation, exclusive-mutation leases and
exactly-once settlement. Proxies never touch a second run/session/terminal
authority — `ToolManager` policy, run terminals and receipts stay with the
framework.

## Extension kinds and operations

Each registration names one `ExtensionKind`, a typed per-kind
`ExtensionDescriptor` (versioned; unknown versions fail closed) and a
client-side `implementation_id`. The closed `ExtensionOperation` set binds
every reverse call to its kind; dispatching an operation to the wrong kind
is rejected before any callback leaves the process.

| Kind | Operations | Injection point |
|---|---|---|
| `tool` | `tool_execute`, `tool_execute_stream`, `tool_validate_parameters` | `ReactAgent::add_tool` |
| `llm_client` | `llm_chat`, `llm_chat_stream` | `ReactAgent::set_llm_client` |
| `store` | `store_put/get/search/search_with/delete/list_namespaces/list` (+prune/dedup) | `set_memory_store` for approved memory recall/search; explicit `set_store` and raw KV operations retain separate caller semantics |
| `human_loop_provider` | `human_loop_request` | `set_approval_provider` + the appeal tool |
| `hook` | `hook_run` | `HookRegistry::set_programmatic_hook` |
| `agent_callback` | `callback_on_*` (observational) | `add_callback` |
| `intervention_callback` | `intervention_on_tool_call/think_start/final_answer` | `add_intervention_callback` |
| `agent_factory` | `factory_create_agent` | `register_subagent_factory` (lazy construction) |
| `custom_agent` | `agent_execute(_stream)/chat(_stream)/close` | `register_agent` (subagent dispatch by name) |
| `channel_plugin` | `channel_start/stop/send/health` | `channels` facade manager (when `framework-channels` + `sdk-extension-bridge` are compiled) |
| `channel_message_handler` | `channel_handle`, `channel_handle_stream`, `channel_reply` | ChannelPlugin/MessageHandler reverse adapter |
| `context_compressor` | `compressor_compress` | `ReactAgent::set_compressor` for Sessions constructed after registration |
| `agent_component` | closed component operation enum plus component stream | ConversationStore, RunStore, RuntimeStateStore, AuditLogger, PreModelContextProjector, MemoryTriggerSink, Guard, SearchProvider, Workflow CheckpointStore, RevisionedTaskStore, SandboxExecutor, McpTransport, Embedder, MemoryPromoter, Workflow, IntentClassifier and SkillLoadPolicy; injected into new Sessions or consumed by an explicit resource operation |

Registrations are **connection-owned**: they never survive a Host restart
or a reconnect, and they take effect for Session Agents constructed after
the registration. Re-registering the same identity with the same
registration fingerprint (descriptor plus default timeout) returns the same
handle; a different snapshot is a typed `extension_conflict`.

Logical runtime identities are unique within a connection: Tool and directly
registered CustomAgent names may have only one owner, and
Store/HumanLoopProvider are singletons. LlmClient registrations are the
deliberate exception; the most recent registration is selected by monotonic
registration order. Opaque handle UUID order never chooses an implementation.

## Reverse invocation contract

`_echo_agent/extension/invoke` (Host → SDK) carries the extension handle,
a fresh invocation identity (never a JSON-RPC request id), a named
`invocation` union containing the operation and its typed input, an optional
session/run correlation context, the deadline and — for streaming operations
— the **Host-minted stream handle** the SDK must echo.

The protocol crate exposes the operation-discriminated `ExtensionInvocation`
and `ExtensionResult` unions, plus public DTOs for each extension kind. The
generated schema defines the shapes consumed by TypeScript, Python and Java
dispatchers; `WireValue` remains only at explicitly open leaf fields such as
user parameters, metadata and provider-specific raw values.

An LlmClient descriptor may set `supports_streaming: false`. The Host then
invokes `llm_chat` and adapts the single typed response into the framework's
one-terminal chunk stream; it never calls an operation the implementation
declared unsupported. The response must contain a non-empty, bounded
`finish_reason`; the Host rejects an ambiguous response instead of inventing
`stop` or `tool_calls`.

A ContextCompressor call carries the framework message set, lossless token
limit, current query, focus instructions and a temporary Host-owned tokenizer
resource. The three SDKs expose `countTokens` / `count_tokens` against that
resource, so a compressor uses the exact calibrated tokenizer selected by the
ContextManager instead of a language heuristic. The resource is owner-checked
and released when the callback settles. The SDK returns retained and evicted
messages plus an optional compression checkpoint; cancellation, timeout and
disconnect settlement remain Host-owned.
Its descriptor carries the stable strategy name used by Rust compression
metrics. `ReactAgent::set_compressor` accepts an extension handle for an
explicit live-Session replacement; different Sessions can bind different
registrations.

Agent infrastructure components share one typed bridge family but not one
untyped callback. The descriptor has a closed component discriminator;
`AgentComponentCallInputWire` and `AgentComponentCallResultWire` are
operation-discriminated unions that freeze every argument/result shape and
lossless integer field. Rust proxies implement the actual framework traits and
validate the returned component and operation before restoring the value.
TypeScript exposes the same discriminated unions, Python validates the closed
operation set and named argument map, and Java decodes calls into the sealed
`AgentComponentRequest` hierarchy.

Defaultable Rust trait methods are still explicit bridge operations when an
implementation may override them: ConversationStore ensure/search, RunStore
append/parent-list, Sandbox cancel-aware execution and Workflow/Sandbox streams
all cross the callback boundary. `supports_streaming` is valid only for
SandboxExecutor and Workflow. Their stream items use the Host-issued extension
stream handle and separate typed Sandbox/Workflow chunk and terminal unions.
Sandbox `complete/failed` and Workflow `completed` can only be sent as the
outer stream terminal; output/node/token events can only be chunks. The Host
then republishes Workflow extension events through the same canonical
`WorkflowEvent` projection as graph streams. No buffered result is presented
as a live stream. IntentClassifier receives the exact user input and message
context and can be installed with an explicit IntentRouter config and
available-skill fence. SkillLoadPolicy receives every public descriptor field
through `skill_load_allows`; discovery, prepared-plugin registration and
reconciliation await that same callback on the live Session Agent.

WorkflowCheckpointStore does not inherit a successful no-op for claim
settlement. The bridge transports generation-CAS save plus attempt-fenced
renew, acknowledge and requeue calls. Its descriptor declares a canonical
`claim_heartbeat_interval_ms` from 1 through 300000, and the Host schedules
renewal from that negotiated capability. Missing operations, owner mismatch or
unsupported settlement fail before a continuation can be reported as settled.

Accepted Workflow A adds one explicit `protected_claim_v1` capability and
`workflow_checkpoint_protect_claim({checkpoint_id, attempt_id})` reverse call
with a unit result. The Store must atomically protect that exact attempt
before reporting success; a flag alone is no protection. This branch has
focused Protocol/client evidence and the Host pins published framework
`9886502c83d6b1f6fa9212eb221db0e5874deaee`. Full integration and final
cross-language governance acceptance remain pending.

The accepted contract keeps legacy/false-capability Store registration and
unrelated operations usable. Graph capability belongs to its originally bound
Store. Any claim-backed Approved/Modified node continuation, including branch
execution, must reject missing support before claim, fork-save or effects with
`feature_unavailable`/`never`; fresh execution, Deferred and no-node Rejected
retain their specified paths. An unknown protected attempt cannot expire or be
overwritten/deleted/cleared into a retry. Only its real owner can settle it by
the original acknowledge/requeue calls. Protection inherits the provider's
durability, so an in-memory instance is not persistent across process restart.
Owner loss may block indefinitely; no force-unlock or receipt-query API is added.
Old binaries must not access protected data, including through old delete/clear.

Managed ConversationStore/RuntimeStateStore callbacks expose the eleven
explicit managed persistence operations, including capability declarations,
atomic transcript application, revision/generation CAS, projection receipts,
acknowledgement and scope retirement. `PersistenceCallContext` retains the
exact deadline and cause; a backend must check the deadline at its actual
commit boundary. Unsupported operations fail explicitly, and an unknown
commit outcome remains `OutcomeUnknown` until resolved from the backend's
receipt/authority; the bridge cannot fill in success or a default revision.

RevisionedTaskStore registration declares `execution_cas_v1` explicitly
(default false). A CRUD-only store is valid for task creation/update/list, but
execution without execution-CAS support fails before dispatch or mutation.
Execution commits preserve the exact claim, expected executions, result
(including absent versus explicit null) and Cancelled/Paused disposition in
one authoritative CAS. TaskRun control and a Session Run's cancellation scope
remain distinct; the bridge does not add a scheduler or terminal authority.

Cancel-aware Sandbox callbacks preserve the framework error domain: extension
cancel and timeout settle as `SandboxError::Cancelled` / `SandboxError::Timeout`,
and Run cancellation waits for the component cleanup call before publishing
the cancelled terminal.

AgentComponent admission is operation-scoped. Workflow run/run-stream calls
are exclusive because the Rust trait takes `&mut self`; Send+Sync `&self`
components such as Store, AuditLogger, Guard, SearchProvider and Embedder may
serve multiple Sessions concurrently. Same-Session callback mutation remains
an independent `extension_conflict` rule.

MCP transport notifications start polling only when `notification_rx()` is
requested, use a bounded 64-item oldest-drop queue, and stop on transport
close. MCP initialization failure closes the supplied transport. A successful
`McpClient::from_transport` initialization whose facade handle cannot be
published also closes the client before returning the quota error. Manager
connect plus client/tool handle publication is transactional: any tool-handle
quota failure removes the client handle and disconnects the manager.

No public `extension` item is relabeled as a same-topic family/core operation.
Traits with a live Host consumption point use a typed proxy. Rust generic,
borrowed-view, `FnOnce`, marker/builder, event-bus registration and
runtime-owned construction traits without a Host call site carry an exact
process-local evidence route. Only an explicitly accepted portable contract
creates a language interface/helper obligation.
The complete intrinsic membership and its digest are inventory telemetry.
Only explicitly accepted external contracts participate in compatibility
negotiation; a new Rust intrinsic item does not create a language obligation.

AgentFactory results are invocation-scoped instances rather than direct
CustomAgent registrations. The Host reserves a unique child handle for each
construction; the result must echo that exact instance. Subsequent callbacks
carry typed `factory_origin` binding the child to its parent registration, so
SDK dispatch never guesses by logical name or falls back to an unrelated
implementation. Same-name concurrent constructions are independent; each
child's mutable execution remains exclusive.

A construction deadline or cancellation does not prove construction stopped.
The SDK retains the pending construction/child cleanup owner until actual
completion and closes a late-created child exactly once. Parent unregister
cannot discard this pending ownership. Child cleanup failure remains an error
with a retry owner; successful cleanup releases the handle. A failed child
close can be retried through that same owner. Concurrent close callers share
successful cleanup, and later closes do not repeat it. A failed acknowledgement
does not become a permanent cached close result or release the child address.

For streaming children, the original RPC answer and a validated remote stream
terminal must both arrive before the invocation is released. That release
precedes delivery of its terminal to a consumer that may immediately close the
same child. A pending answer, a missing or invalid terminal, and a caller drop
do not release an active invocation. Factory children are excluded from the
Session-visible registration view, so another Session cannot capture a
temporary instance.

Store injection does not promote raw data to approved memory. `set_memory_store`
binds the framework's approved-memory recall/search interface; automatic
memory writing/promotion requires the framework `MemoryLayerManager` path.
Explicit `set_store` and accepted `memory.put/get` KV calls remain raw Store
operations and must not implicitly turn a value into Active memory.

The SDK answers with exactly one of:

- `result` — one typed payload (the trait's return value);
- `stream` — the echoed stream handle, payload delivered through
  `_echo_agent/extension/stream` notifications;
- `error` — a typed `EchoSdkError`.

Failure semantics (no built-in fallbacks, design §12.1):

- **deadline / cancellation of physical producers** — for Factory/CustomAgent,
  Tool/LLM execution, Sandbox/Workflow execution or cleanup and their driven
  streams, signal the original execution owner and retain its context/resources
  until matching closure facts. A local timeout or cancellation token is not
  proof that physical work stopped; ordinary first-cause errors keep their
  final classification after actual cleanup. This does not change ordinary
  Store/managed-call deadlines, unknown-commit rules, or best-effort Hook and
  notification policy. Explicit observation below is still handled before any
  resultless callback fallback; it is not converted to Unit/default/failure;
- **explicit observation** — a complete, validated `framework_error` with
  `retryable: never`, unique `observation: not_settled` and original non-empty
  `run_id` (plus non-empty `execution_id` when present) is nonterminal. It is
  not permission to retry the mutation. Three SDKs retain the original callback
  cancellation address; public `streamWriter` / `stream_writer` `failed(error)`
  keeps the same writer open for the original producer's real terminal. A
  terminal already submitted by that writer cannot be reopened by a late
  observation continuation. Ordinary or malformed failure envelopes keep their
  ordinary terminal policy; the Host separately retains uncertain owners for
  malformed observation hints without trusting their identities;
- **disconnect** — the official transport can reject pending requests or end
  local subscriptions. This proves neither a framework Run terminal nor cleanup
  of an external producer. A callback response lost to transport is not a
  physical receipt; unknown owners retain their original lease/context;
- **late response** — a completed receipt cannot be overwritten. While the
  original callback is still unknown, its real matching response and valid
  remote terminal can release that callback's original resources, once. This
  does not fabricate a foreign execution receipt when setup returned no
  continuation source. If the original framework owner still drives that same
  stream, its real result and original EOF must reach the existing driver/CAS
  and unique canonical receipt; the earlier observation must not discard that
  later result. Neither case starts a replacement execution or driver;
- **re-entry** — a second exclusive invocation on the same registration, or an
  extension callback attempting an exclusive mutation of its active Session,
  fails fast with `extension_conflict` instead of waiting on the Agent lock
  (design §12.3).

## Streams

Streaming callbacks deliver `chunk` events with per-stream contiguous
sequences starting at 1 and exactly one terminal (`complete`, `failed` or
`cancelled`). The Host enforces exactly-one-terminal and sequence
continuity at a bounded sink. A wrong payload kind, gap, duplicate before
settlement or mailbox overflow settles a typed failed terminal immediately;
events for unknown or already-released streams are discarded with bounded
diagnostics. A terminal ends payload delivery. For Channel and CustomAgent
callbacks, it also remains distinct from the actual original response/producer settlement:
the retained owner must finish before its stream address is released. See
[Channel consumption](channel-handler-consumption.md) for the Channel callback
return and CustomAgent early-setup-ACK contracts. The callback mailbox capacity
uses the negotiated
`max_stream_buffer_events` / Host `max_outstanding_live_events` bound, so the
advertised limit and runtime allocation cannot drift.

A valid asynchronous producer may return its Stream setup ACK immediately.
The SDK continues forwarding cancellation to that original producer's signal
while the stream is active, including cancellation received before the ACK.
After its cleanup finishes, the producer must successfully send the terminal
for the original matching stream handle and generation. A wrong-generation
terminal, failed terminal send or failed close does not release that route;
retry using the same stream. An early terminal and a later ACK are handled as
separate facts. The SDK does not infer physical cleanup from either message.

The contract uses different named unions for chunks and completion. Tool
chunks can only be progress/output while Tool completion carries a result;
LLM chunks cannot carry a finish reason while completion requires one; and
CustomAgent chunks contain only non-terminal Agent events while completion
contains only final answer, error or cancellation.

## Negotiation and admission

- The Host advertises `extension_bridge` plus its bounds
  (`max_registered_extensions`, descriptor/payload/stream byte limits,
  in-flight invocation and callback concurrency, default callback
  timeout) only when the bridge is compiled; a plain standard Client
  receives method-not-found for every `_echo_agent/extension/*` call.
- Every handler walks the fixed ladder: extended-mode gate → capability
  gate → handle shape/kind/generation/closed → descriptor and payload
  bounds → framework work.
- Connection teardown order (design §12.3): close new admission → cancel
  in-flight invocations → bounded settlement drain → profile flush →
  Session Agent/MCP close → release extension registrations and handles.
  Already issued streams may still deliver their bounded, ordered prefix and
  terminal through the original owner/generation checks. Closing admission
  does not discard the prefix needed to validate that producer's terminal.
  Cooperative cleanup timeout retains the same retry owner; it is not a
  successful callback close.

## Secrets and output discipline

Descriptors, events, errors and diagnostics never carry credentials; the
Host's credential stays out of stderr (asserted in the E2E). stdout carries
only the official ACP wire.

## Relation to the facade feature adapters

The facade feature adapters
([facade-feature-adapters.md](facade-feature-adapters.md)) route the
feature-family surfaces to the framework's own services; the bridge remains
the only reverse-call path for consumer-implemented traits. A facade family
never simulates a result when a bridge callback fails. Tool stream `Failed`
and remote `Cancelled` terminals retain their typed timeout/cancellation
classification in `ToolResult.failure`: the adapter uses the framework's
`ToolFailure::from_error`, with possible side effects derived from the registered
Tool risk (read-only versus standard/dangerous). The original failure message
is preserved. This is necessary because ToolManager consumes a `Complete`
result directly. LLM remote cancellation likewise remains the framework's
Agent cancellation domain. Opaque SDK error details have no corresponding Rust
failure field and are not invented as another DTO or inferred from messages.

## Verification

- `echo-sdk-protocol` contract tests: typed descriptors, operation
  taxonomy, official RPC derives, fixtures and fail-closed samples
  (`tests/extension_contract.rs`, `tests/core_rpc_contract.rs`).
- Shared runtime regression: concurrency permits, exclusive conflicts,
  deadline/cancel settlement, late-response discard, teardown order
  (`tests/acp_extension_runtime.rs`).
- Real-process E2E with the official Client as the SDK dispatcher
  (`echo-sdk-host/tests/extension_bridge_e2e.rs`): tool round trip with
  callbacks, intervention and hooks; Store/HITL; AgentFactory/CustomAgent;
  streaming and non-streaming LlmClient; ContextCompressor injection,
  late binding and compression; Agent component consumption; ChannelPlugin
  lifecycle/send and original MessageHandler forward consumption;
  malformed kind, oversized chunk and
  out-of-order immediate failure; duplicate terminal; real Host mailbox
  backpressure; registration deadline with late response; framework
  cancellation/consumer-drop notice; SDK disconnect; plain-client
  fail-closed; semantic identity conflicts, unregister idempotency and the
  stale-generation ladder.
- Host bridge unit tests cover complete typed AgentEvent round trips,
  duplicate sequence settlement and bounded-mailbox failure retention.
