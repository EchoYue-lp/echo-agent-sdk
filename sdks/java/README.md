# echo-agent Java SDK (source)

This is a source-only Java 17 ACP v1 client built on the official
`com.agentclientprotocol:acp-core:0.17.0` transport/session runtime. The Host executable is supplied
explicitly by the caller; this module does not bundle a JDK, Host binary or
JAR. The transport follows ACP's newline-delimited JSON-RPC framing and the
facade payloads are taken from the checked-in canonical catalog.
Build and run it with JDK 17.

```bash
mvn test
mvn -q package
java -cp target/echo-agent-sdk-source-0.1.0.jar:... \
  com.echoagent.sdk.Example /absolute/path/to/echo-agent-sdk-host \
  /absolute/path/to/host.json
```

`src/main/java/com/echoagent/sdk/Example.java` is the executable Java
quickstart and is compiled and run by the repository language gate.

`EchoAgentClient` exposes `CompletionStage` request methods, `Flow.Publisher`
session updates and run events, `AutoCloseable` Agent/Session/Run handles and typed
`EchoAgentException` failures. Jackson is used only for `_echo_agent/*`
extension values; the official SDK owns ACP framing, session and writer. A
private stdio adapter owns the single original Host process and native shutdown.
No Agent/Task/Subagent execution logic is reimplemented.
Typed `ToolDescriptor`, `LlmClientDescriptor`, `StoreDescriptor`, `CriticDescriptor` and
`ContextCompressorDescriptor`
builders emit the versioned registration snapshots from the shared extension contract.
Their `ToolCall`, `LlmChatCall`, `StoreCall` and `CriticCall` handlers expose Host-issued
identity, deadline and stream handles without hiding the generic `JsonNode` APIs.
`ToolOutcome`, `LlmOutcome`, `StoreOutcome`, `CriticOutcome` and `CompressionOutcome` builders serialize
operation-discriminated results through the existing `_echo_agent/extension/invoke`
callback; they do not create a second execution path. Critic callbacks receive the
Host-issued `critic_critique` input (`task`, `answer`, `context`) and return the structured
`Critique` value (`score`, `passed`, `feedback`, `suggestions`). Verifier policy,
retries and run settlement remain Host-owned; registering a Critic does not enable
verification implicitly.
`registerContextCompressor` supplies a typed `CompressionCall` and accepts a
`CompressionOutput`; invocation deadlines, cancellation and session teardown remain
owned by the same Host extension bridge. `CompressionCall.tokenizer()` plus
`EchoAgentClient.countTokens` use the exact Host-owned tokenizer.
`registerAgentComponent` uses `AgentComponentDescriptor`, `AgentComponentCall`
and `AgentComponentOutcome` for Host-consumed conversation/run/runtime state,
audit, context projection, memory trigger, guard, search,
workflow/checkpoint, revisioned task, sandbox, MCP transport, embedding and
memory promotion. Calls decode to the sealed `AgentComponentRequest` hierarchy,
and results use the sealed `AgentComponentResult` hierarchy; nested component
and operation discriminators remain checked by the Host.
The sealed hierarchies include conversation ensure/search, Run
append/parent-list, IntentClassifier, SkillLoadPolicy, cancel-aware sandbox
execution and Workflow/Sandbox stream variants. Streaming handlers return
`ExtensionOutcome.stream` and publish the separate sealed chunk/terminal types
with `agentComponentChunk` / `agentComponentComplete`.
`EchoAgentClient.call` and `SessionHandle.invoke` select source or family
operations from the shared canonical catalog; process-scoped operations accept
a null handle and Session-bound operations use the Host-issued Session handle.
`WireValues` provides bounded `u64`, `i64`, `bytes`, `utf8Path`, `duration`
and `timestamp` constructors for the extension scalar algebra.
`Utf8Support` provides language-local `splitUtf8Chunks`, `cleanJson` and
`extractJsonFromMarkdown` helpers. `IncrementalUtf8Decoder` incrementally decodes
byte streams without splitting UTF-8 scalars, replaces malformed bytes with
U+FFFD, and retains incomplete suffixes until `finish()`. These helpers are
pure Java utilities and do not add Host, ACP or catalog protocol surface.
`ToolCallParams` provides the Rust parameter getters and required-type checks;
`ToolResult` adds immutable static factories (`success`, `successJson`,
`failure`, `invalidArguments`) and `with*` modifiers over the existing JSON
shape.
Failure categories are closed and preserve Rust recovery actions; ordinary
JSON objects passed to `successJson`/`withData` are recursively encoded as
maps, while the `JsonNode` overload remains the explicit pre-encoded path.
`TaskState` is a Java enum with the same terminal, transition and display
semantics as the Rust A2A value; it does not add a protocol route.
`A2AMessage`, `A2ATaskStatus`, `AgentProvider` and `AgentSkill` provide the
corresponding immutable value constructors and text projections.
`AgentCard.builder(...)` provides the local immutable card and fluent builder;
it does not mirror the Host-owned `from_agent` operation.
`A2AArtifact.newArtifact(...)` and `A2AError.newError(...)` preserve the A2A
artifact/error wire fields as immutable values.
`TaskStatusUpdateEvent`, `TaskArtifactUpdateEvent` and `A2AStreamResponse`
provide the corresponding closed local stream values.
`A2ATaskParams`, `A2ATaskRequest`, `A2ATask`, and `A2ATaskResponse` provide
immutable nested task envelope values; task execution remains Host-owned.
`ThinkingLevel.parse(...)` preserves the Rust reasoning-effort aliases.
`AgentSteerState` and `AgentSteerTurnOutcome` expose the same local lifecycle
values while the active receipt remains Host-owned.
`SubagentCommandPhase` and `SubagentStatus` expose stable phase/status values;
dispatch ownership remains in the Host.
`ContentGuardResult` exposes pass/detect/reject/redact values while guard
execution remains Host-owned.
`GuardDecision` exposes Rust pass/block/warn/transform outcomes as immutable
sealed values.
`DeliveryOutcome` and `DeliveryPhase` expose stable ledger values while the
durable lifecycle remains framework-owned.
`SubagentStopStatus` exposes stable hook terminal values while hook execution
remains framework-owned.
`TaskTerminalStatus` exposes stable task terminal values while task execution
remains framework-owned.
`RuleSource` exposes permission source priority values and alias parsing while
rule evaluation remains framework-owned.
`RuleBehavior` exposes allow/deny/ask payloads and decision conversion while
rule evaluation remains framework-owned.
`PermissionMode` exposes aliases and policy helpers while mode evaluation
remains framework-owned.
`RuleMatcher` exposes pure tool/pattern/permission/all matching semantics while
registry evaluation remains framework-owned.
`CommandCellPhase` exposes stable phases and terminal classification while
command execution remains framework-owned.
Command-cell terminal causes and artifact statuses expose stable values while
execution remains framework-owned.
`TeamStrategy` exposes manager/pipeline/debate/swarm values while Team dispatch
remains framework-owned.
`AcpValues.ConnectionMode`, `AcpValues.ExtensionSettlement`, and
`AcpValues.AcpLedgerLimits` expose ACP runtime values while connection and
event-ledger ownership remain in the Host/Rust runtime.
`AcpAdapterConfig.defaults()` and `validate()` preserve adapter limits and
shutdown-duration validation without constructing the ACP adapter.
`AcpValues.ExtensionLeaseError` preserves typed lease failure text while the
Host retains admission and concurrency authority.
`JwtConfig` and `JwtClaims` preserve local A2A auth configuration and subject
projection while JWT verification remains Host/Rust-owned.
`DependencyKind` and `SkillSource` expose dependency/source values while
probing and skill loading remain framework-owned.
`ContextInheritance` exposes sync/fresh/fork/teammate/team defaults while
Subagent context and dispatch remain framework-owned.
`ObservedIsolation` exposes trim, empty-default, and code-point-safe bounds
while isolation execution remains framework-owned.
`SegmentRange` exposes half-open saturating length and emptiness while cache
state remains framework-owned.
`PromptDiagnostics` exposes section recording and per-id counts while prompt
compilation remains framework-owned.
`SubagentCommandIdentity` preserves durable ID validation and attempt
projection while live-control registry state remains framework-owned.
`LlmUsageStats` preserves cumulative token counters and payload projection while
provider execution remains framework-owned.
`ToolOutputArtifactConfig` preserves retention, threshold, and max-age builders
while artifact writing remains framework-owned.
`SkillValidationReport` preserves violation gating while validation remains
framework-owned.
`SkillContent` preserves structured prompt-block rendering while resource
loading and execution remain framework-owned.
`JsonRpcValues` preserves MCP `2.0` request/notification constructors while
transport remains framework-owned.
`HookAction` preserves tagged configuration and validation while hook
execution remains framework-owned.
`HookEvent` and `HookEventCategory` preserve stable names, `ALL` ordering,
category classification, parsing and matcher predicates while hook dispatch
remains framework-owned.
`EventId`, `StreamId` and `EventIdentity` preserve non-empty validation,
run/chat constructors, correlation fields and immutable `with*` updates.
`InterventionResult` exposes immutable allow/block/cancel/inject/argument
modification factories without owning callback execution.
`TokenBudget`, `TokenBudgetConfig`, `TokenAllocation` and `LlmTimeouts` expose
the same allocation, compression and zero-disabled timeout policy helpers.
`ExecutionUsage.durationMillis()` maps absent duration to zero without owning
run accounting.
`TurnMode` preserves the chat/execute stream flavor without owning the turn
driver.
`RetryPolicy` preserves default/no-retry factories, exponential backoff caps
and optional jitter configuration without running retries.
`ThinkingConfig` preserves disabled/level/budget variants, parsing and
provider effort/budget projections without owning LLM transport.
`TimeValues` preserves Unix timestamps, local-offset formatting, instant
round-trips and optional null values without persisted clock state.
`ThinkingProtocol` preserves provider dialect names and field-emission
semantics without owning provider transport.
`ResourceLimits` preserves default, strict and unrestricted sandbox policy
snapshots without creating sandbox processes.
`ProviderCapabilities` preserves OpenAI-compatible, Anthropic and Ollama
defaults plus provider-name resolution without owning provider transport.
`ThinkingProfile.resolveThinkingProfile` preserves model/provider protocol
selection and manual control levels without contacting a provider.
`ModelProfile`, `ModelProfileOverride`, and `ModelProfileResolver` preserve
provider capability defaults, model limits, tokenizer selection, and provider
then exact override precedence without owning provider transport.
`LlmApiProtocol` preserves endpoint path selection and strict complete-path
detection without opening HTTP connections.
`ModelInputModality` preserves text-only and historical all-supported ordering
without owning model transport.
`ResponseFormat` preserves text, JSON object, and strict JSON Schema tagged
values without validating or executing provider responses.
`PromptCacheLayout` and `SegmentRanges` preserve the Rust system/canonical/
history/runtime segment projection and half-open indexes without owning cache
state or provider placement.
`MemoryScope` preserves lifetime ordering, persistence classification, wire names
and parser aliases without owning memory storage or cleanup.
`MemoryType` and `MemorySource` preserve stability, promotion eligibility,
confidence and recall-weight defaults without owning memory persistence.
`PageInfo` preserves truncation, continuation metadata, and output projection
while collection state remains framework-owned.
`SubagentContext` exposes empty/content semantics while context and dispatch
state remain framework-owned.
`Usage` preserves provider-normalized cache priority and effective token
calculations while LLM execution remains framework-owned.
`HookAction` preserves tagged configuration and validation while hook
execution remains framework-owned.
`LlmUsageStats` preserves cumulative token counters and payload projection while
provider execution remains framework-owned.
`SubagentCommandIdentity` preserves durable ID validation and attempt
projection while live-control registry state remains framework-owned.
`RunHandle.status()`, `RunHandle.outcomeStatus()` and `RunHandle.usage()` query
the Host-owned settled receipt through the canonical Run receiver. They return
`JsonNode` values directly; `WireU64` counters in usage remain textual nodes so
large values are not converted through Java numeric types.
`EchoAgentClient.classifyTurnOutcome(eventWire)` delegates terminal event
classification to the canonical `TurnOutcome::classify` source operation. Use
`WireValues.record` or `WireValues.variant` to construct typed structural event
values; the Java SDK does not duplicate the framework's classification rules.
Session updates and Run events use bounded `Flow.Publisher` instances; event
sequence gaps fail explicitly, consumed events are ACKed, and `closeAsync()`
provides bounded CompletionStage shutdown for sessions, runs and the client.

## Governance and failure boundaries

The compatibility promise is the explicitly accepted external contract and
accepted facade catalog. Complete Rust public inventory/intrinsic membership
is non-blocking telemetry; it is not a requirement to expose every Rust
identity in all languages. The current governance candidate awaits final
contract, real-Host three-language and exact-main acceptance. The
[capability/evidence matrix](../../docs/sdk/governance/behavior-evidence-matrix.md)
records behavior evidence and remaining scope; catalog `done` labels alone do
not prove a failure path.

A failed Agent/Session/Run close retains its local cleanup state so the caller
can retry; only successful cleanup ends the stream/releases the owner.
Duplicate successful close is idempotent. Timeout, cancellation, EOF and Host
exit must not be treated as a successful terminal. Facade resources validate
owner plus complete generation-fenced handles; failed publication rolls back
construction rather than returning an unusable handle.

Successful Session close disposes its local Run publishers, including a start
reply arriving after close. Buffered facts are retained before publisher
completion; first Run/update subscriptions after close are already finite.
Completion is local resource disposal, not a Run terminal or receipt. Failed
parent close retains the child scope for retry. Each concurrent close caller
receives an independent observer of the same cleanup owner; cancelling one
observer cannot cancel that owner. A child close may dispose its publisher
after `closed_handle`, `stale_handle` or `host_exited`; `host_shutting_down`
continues to fail and retain retry state. Submitted RPC/ACK owners survive local
scope disposal. See [ADR0041](../../docs/adr/0041-session-close-local-child-scope.md).

Client `closeAsync()` first waits for its existing callback/resource cleanup.
The native transport then drains accepted frames through the official writer,
closes the original child process's stdin, and awaits that same process's actual
exit and local I/O cleanup. Its five-second native deadline includes writing,
EOF and exit; expiry force-stops the original child and reports failure rather
than successful flush. Deadline and post-exit completion select one outcome
before publishing it; an already selected failure keeps its place while native
cleanup finishes, and completed success prevents later deadline effects.
Failed cleanup retains unfinished ownership for retry;
cancelling a close waiter does not cancel that owner. This does not establish a
Run terminal, telemetry export success or cleanup of an external producer;
writer flush proves local frame write completion only.

Actual Host process exit promptly ends pending Java requests and event/update
subscriptions with `transport_error`. Cancelling initialize closes the same
spawned Host through the existing session close path; failed close retains its
cleanup owner for retry. Stdout EOF while the child remains alive is outside
this process-exit guarantee, and transport failure proves neither a Run terminal
nor cleanup of an external producer.

Factory callbacks echo the Host-reserved instance handle. Child invocations
carry `factory_origin`, binding that instance to its parent registration.
Concurrent same-name factory instances remain independent; no name-based
fallback routes an unknown child. A construction timeout/cancellation retains
cleanup ownership until construction really finishes, including cleanup of a
late-created child. Constructor shutdown also retains the original user
`CompletionStage` and signals its existing cancellation token; cancelling a
future projection is not construction completion. Client close waits for the
original constructor and then cleans the reserved child exactly once, including
partial construction failure. If construction cannot settle within the close
budget, the parent registration, child and original stage remain retryable.
Concurrent cleanup callbacks for the same complete child
handle share one actual cleanup job. Each invocation keeps its own deadline
and cancellation wait; these do not cancel the cleanup job. Only its successful
`agent_close` unit result releases the child and records idempotent closure.
A failed cleanup attempt retains the live child and allows a new attempt;
connection close signals a separate cleanup-owner cancellation token and
waits for the original `CompletionStage` to actually complete. Cancelling a
`toCompletableFuture()` projection is not native completion proof. If an
original stage remains pending for five seconds, close reports
`extension_timeout` and retains the same child, stage and routing for retry;
`closed` stays false. Each `closeAsync()` call returns its own waiting stage;
cancelling one waiter leaves later waiters attached to the same private cleanup
owner. A subsequent close joins that retained stage and can complete after the
original work settles successfully. Actual stage completion alone does not
prove resource cleanup: exceptional completion, a typed error or a malformed
outcome makes client close fail and retains the child and handler. A later
close starts one new attempt for each failed child; children that already
closed successfully never repeat cleanup. Malformed, null or streaming close
results fail explicitly and release only the failed attempt, retaining retry
rights while the connection is live. These maps describe local
reverse-call ownership and do not decide framework Run or Task terminal state.

Managed ConversationStore/RuntimeStateStore callbacks retain declared
capabilities, atomic receipts, revisions/generations and exact commit
deadlines. Unsupported calls fail explicitly; `OutcomeUnknown` is unresolved
until backend evidence establishes the outcome. RevisionedTaskStore is
CRUD-only unless its descriptor opts into `execution_cas_v1` (default false).
Execution without this capability fails before work starts; implementations
that opt in must atomically preserve expected executions, exact claims,
results and Cancelled/Paused disposition. SDK callbacks transport these facts;
the framework alone schedules tasks and owns terminal state.

`set_memory_store` connects approved-memory recall/search. Automatic memory
writing/promotion requires the framework MemoryLayerManager. Explicit
`set_store` and accepted raw KV `memory.put/get` calls do not promote their
values into Active memory.

Run event publishers ACK only after the subscriber's `onNext` consumption
boundary, not when transport buffering accepts a notification. Processing
that continues asynchronously after `onNext` must be coordinated by the
subscriber. `run.replay(afterSequence, maxEvents)` validates/records the page
without ACKing it. After applying a required snapshot and consuming the page,
use `sdk.notify("_echo_agent/event/ack", params)` with `params.ack.stream =
run.stream().toJson()` and `params.ack.last_processed_sequence =
page.path("next_cursor").path("last_processed_sequence").asText()`.
The ACK establishes the monotonic continuation floor; subsequent live facts
must be above it. Await the returned `CompletionStage` rather than treating
notification enqueue as application processing.
If replay includes the journal's terminal event, a successfully submitted
explicit ACK covering that event also ends the local event feed naturally.
Fetching the page or sending an uncovered future cursor does not end it;
this delivery projection does not change the authoritative Run state.

Each successful transport submission advances the local consumed cursor
monotonically. Pending, failed or cancelled submissions do not retire queue
facts or establish replay-terminal EOF; transport failure reports
`ack_submission: outcome_unknown` in error details, because a notification
has no peer acceptance receipt. Another concurrent ACK cannot borrow a
pending submission's cursor. The framework journal remains authoritative.
Cancelling the returned caller future does not cancel the submission's
completion observer. A later actual success still commits its cursor; client
close settles pending submission ownership without claiming consumption.

If a valid pending ACK's continuation reaches a full live queue, ordered
notification delivery waits for that ACK's local projection or actual queue
capacity/lifecycle progress. It never reserves extra data capacity or retires a
prefix before native write success. Subscriber demand, cancellation and close
can release this wait without waiting for ACK IO; a still-full queue after a
failed ACK retains the existing overflow/replay error policy. Later notifications
can temporarily wait behind this ordered admission fence; independent RPC
responses and reverse request execution use the official session's separate
paths. The ACK send itself has no new timeout or peer acceptance receipt.

Typed ACP `error.data` becomes `EchoAgentException`, preserving `code()`,
`retryable()`, `operation()`, `handle()` and `details()`. Standard ACP errors
retain official wire semantics; human messages are not parsed into domain
codes. Prefer `closeAsync()` when integrating asynchronous lifecycle cleanup;
`close()` is the blocking `AutoCloseable` convenience. Full-range integer
values stay textual/BigInteger rather than floating point. Paths and time
retain their tagged encoding. Unknown tagged values round-trip; malformed
known values and missing nested discriminators fail. Object keys such as
`kind` remain domain data unless explicitly inside a wire envelope.


### Channel handler consumption examples

The real Host examples for the six canonical Channel operations and Sandbox
cooperative close are described in
[Channel consumption](../../docs/sdk/channel-handler-consumption.md). The
existing native Channel forward tests are executable examples and are included
in `scripts/check-language-sdks.sh`; they register real callbacks, consume the
original handler, and verify typed results and cleanup. MessageHandler callbacks
retain their awaited work/cleanup lifetime. An asynchronous producer using the
existing early Stream setup ACK may return immediately; cancellation continues
to reach its original signal until the matching terminal is successfully sent.
The producer must finish cleanup before declaring that terminal. Cancellation
before ACK and terminal before ACK preserve their original stream identity;
wrong-generation or failed terminal sends and failed close retain the route
for retry on the same stream. Neither ACK nor notification delivery lets the
SDK infer physical cleanup. Full inventory and intrinsic membership remain
non-blocking telemetry.

The source verification suite and standalone quickstart use a test-scoped `slf4j-nop` provider matching `slf4j-api` 2.0.18. This explicitly preserves SLF4J’s default no-operation behavior without missing-provider warnings. The SDK library’s compile/runtime dependencies do not include this provider; applications choose their own logging provider.
