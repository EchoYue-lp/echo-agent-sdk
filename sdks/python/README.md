# echo-agent Python SDK (source)

The Python SDK is source-only. It uses the official `agent-client-protocol`
asyncio client and launches the Host command supplied by the caller. It does
not install Python, Rust, or a prebuilt Host.
Use Python 3.10 or newer; the SDK does not install an interpreter.

Child diagnostics inherit the application's stderr; stdout remains owned by
the ACP protocol. The SDK creates no private diagnostic pipe, so
sdk.process.stderr is None. If the application redirects its own stderr to
a pipe, the application owns that pipe's reader.

```bash
uv sync
uv run python - <<'PY'
import asyncio
from echo_agent_sdk import EchoAgentClient

async def main():
    async with await EchoAgentClient.spawn(
        "/absolute/path/to/echo-agent-sdk-host",
        "--config", "/absolute/path/to/host.json",
    ) as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session()
        print(await session.invoke("echo_core::agent::Agent::name"))

asyncio.run(main())
PY
```

The same flow is checked in as `examples/quickstart.py`; the repository
language gate compiles and runs it against the source-built Host.

The wrapper preserves ACP errors and `_echo_agent/*` typed values. `RunHandle`
events are exposed as an async iterator; cancellation and close always target
the Host-issued generation-fenced handle.
`EchoAgentClient.call` and `SessionHandle.invoke` resolve canonical operation
identities through the shared catalog, including process-scoped operations
that do not require a Session handle.
`EchoAgentClient.classify_turn_outcome` delegates an explicit `AgentEventWire`
value to the framework's canonical outcome classifier and preserves raw
Variant/Null wire values.
Settled `RunHandle` instances expose `status()`, `outcome_status()`, and
`usage()`; these methods read the canonical framework receipt routes and keep
all `WireU64` usage fields as decimal strings.
The `wire_u64`, `wire_i64`, `wire_bytes`, `wire_utf8_path`, `wire_duration`
and `wire_timestamp` helpers enforce the same lossless scalar bounds before a
request is sent.
Pure local text helpers are available without a Host: `split_utf8_chunks` and
`IncrementalUtf8Decoder` preserve UTF-8 boundaries under a byte cap, while
`clean_json` and `extract_json_from_markdown` mirror the framework's JSON
cleanup and Markdown extraction behavior.
`ToolCallParams` exposes the same typed getters and required-parameter
validation. `ToolResult` is an immutable native value with
`success_result`/`success_json`/`failure_result` factories and `with_*`
modifiers; structured payloads use the shared lossless WireValue conversion.
Failure categories are validated as a closed set with the Rust recovery
mapping, and ordinary JSON objects are encoded as maps even when they contain
`kind`/`value` keys.
`TaskState` is a native string enum with the same terminal, transition and
display semantics as the Rust A2A value.
`A2AMessage`, `A2ATaskStatus`, `AgentProvider` and `AgentSkill` are immutable
native values with the corresponding text/status/provider/skill constructors.
`AgentCard.builder(...)` provides the same local immutable card and fluent
builder semantics; Agent-backed card discovery remains a Rust authority.
`A2AArtifact.new(...)` and `A2AError.new(...)` provide immutable wire DTOs for
artifact chunks and typed task errors.
`TaskStatusUpdateEvent`, `TaskArtifactUpdateEvent` and `A2AStreamResponse`
preserve the local typed stream event union without owning transport.
`A2ATaskParams`, `A2ATaskRequest`, `A2ATask`, and `A2ATaskResponse` provide
the corresponding immutable nested task envelopes.
`ThinkingLevel.parse(...)` preserves the Rust reasoning-effort aliases.
`AgentSteerState` and `AgentSteerTurnOutcome` preserve steering boundaries;
the live receipt and turn remain Host-owned.
`SubagentCommandPhase` and `SubagentStatus` preserve stable strings and parse
errors; dispatch state remains Host-owned.
`ContentGuardResult` preserves guard decisions and PII payloads; execution
remains Host-owned.
Rust `GuardResult` is projected as `GuardDecision` so the existing component
`GuardResult` helper remains source-compatible.
`DeliveryOutcome` and `DeliveryPhase` preserve stable ledger spellings without
owning delivery lifecycle state.
`SubagentStopStatus` preserves stable hook terminal values without owning hook
dispatch.
`TaskTerminalStatus` preserves stable task terminal values without owning task
execution.
`RuleSource` preserves permission source priority values and accepted aliases;
rule evaluation remains Host-owned.
`RuleBehavior` preserves allow/deny/ask payloads and `to_decision` conversion;
rule evaluation remains Host-owned.
`PermissionMode` preserves aliases and write/interaction/classifier helpers;
mode evaluation remains Host-owned.
`RuleMatcher` preserves pure tool/pattern/permission/all matching semantics;
evaluation remains Host-owned.
`CommandCellPhase` preserves stable phases and terminal classification without
owning command execution.
Command-cell terminal causes and artifact statuses preserve stable values
without owning process or artifact execution.
`TeamStrategy` preserves manager/pipeline/debate/swarm values without owning
Team dispatch.
`ConnectionMode`, `ExtensionSettlement`, and `AcpLedgerLimits` preserve ACP
runtime values without owning the connection or event ledger.
`AcpAdapterConfig` and `AcpDuration` preserve adapter limits, validation
errors, and the shutdown duration without constructing an ACP adapter.
`ExtensionLeaseError` preserves typed lease failure text while admission and
concurrency remain Host-owned.
`JwtConfig` and `JwtClaims` preserve local A2A auth configuration and subject
projection; token verification remains Host-owned.
`DependencyKind` and `SkillSource` preserve dependency/source values without
probing or loading skills.
`ContextInheritance` preserves the local inheritance defaults without owning
Subagent context, stores, or dispatch.
`ObservedIsolation` preserves trim, empty-default, and Unicode-safe bounds
without owning isolation execution.
`SegmentRange` preserves half-open saturating length and emptiness without
owning cache state.
`PromptDiagnostics` preserves section recording and per-id counts without
owning prompt compilation.
`SubagentCommandIdentity` and `SubagentAttemptIdentity` preserve durable ID
validation and projection without owning live control.
`LlmUsageStats` preserves cumulative token counters and payload projection
without owning provider execution.
`ToolOutputArtifactConfig` preserves retention, threshold, and max-age builders
without owning artifact writing.
`SkillValidationReport` preserves violation gating without running validation.
`SkillContent` preserves structured prompt-block rendering without loading or
executing resources.
JSON-RPC request and notification values preserve MCP `2.0` constructors
without owning transport.
`HookAction` preserves tagged configuration and validation without executing
hooks.
`HookEvent` and `HookEventCategory` preserve stable names, ordering, category
classification, parsing and matcher predicates without owning hook dispatch.
`EventId`, `StreamId` and `EventIdentity` preserve non-empty validation,
run/chat constructors, correlation fields and immutable `with_*` updates.
`InterventionResult` exposes immutable allow/block/cancel/inject/argument
modification factories without owning callback execution.
`TokenBudget`, `TokenBudgetConfig`, `TokenAllocation` and `LlmTimeouts` expose
the same allocation, compression and zero-disabled timeout policy helpers.
`execution_usage_duration_millis` maps absent duration to zero without owning
run accounting.
`TurnMode` preserves the chat/execute stream flavor without owning the turn
driver.
`RetryPolicy` preserves default/no-retry factories, exponential backoff caps
and optional jitter configuration without running retries.
`ThinkingConfig` preserves disabled/level/budget variants, parsing and
provider effort/budget projections without owning LLM transport.
The time helpers preserve Unix timestamps, local-offset formatting, instant
round-trips and optional null values without persisted clock state.
`ThinkingProtocol` preserves provider dialect names and field-emission
semantics without owning provider transport.
`ResourceLimits` preserves default, strict and unrestricted sandbox policy
snapshots without creating sandbox processes.
`ProviderCapabilities` preserves OpenAI-compatible, Anthropic and Ollama
defaults plus provider-name resolution without owning provider transport.
`ThinkingProfile` and `resolve_thinking_profile` preserve model/provider
protocol selection and manual control levels without contacting a provider.
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
without owning collection state.
`SubagentContext` preserves empty/content semantics without owning context or
dispatch state.
`Usage` preserves provider-normalized cache priority and effective token
calculations without owning LLM execution.
`HookAction` preserves tagged configuration and validation without executing
hooks.
`LlmUsageStats` preserves cumulative token counters and payload projection
without owning provider execution.
`SubagentCommandIdentity` and `SubagentAttemptIdentity` preserve durable ID
validation and projection without owning live control.
Session updates and Run events are bounded async iterators with cursor ACKs,
gap/overflow errors, Host-exit propagation and idempotent close semantics.
Context compressor calls include a Host-owned tokenizer handle;
`EchoAgentClient.count_tokens` uses it for exact calibrated token counts.
Agent component calls expose exported dataclass request variants for
conversation/run/runtime state, audit, context projection, memory trigger,
guard, search, workflow/checkpoint, revisioned task, sandbox, MCP transport,
embedding and memory promotion. Named result factories preserve the same
closed nested discriminator and reject mismatched result shapes. The exported
dataclasses also include conversation ensure/search, Run append/parent-list,
IntentClassifier, SkillLoadPolicy, cancel-aware sandbox execution and
Workflow/Sandbox stream variants; stream writers validate separate typed
chunk and terminal shapes through `agent_component_chunk` and
`agent_component_complete`.

## Typed ExtensionBridge

`ToolDescriptor`, `LlmClientDescriptor`, `StoreDescriptor`, and
`CriticDescriptor` provide typed registration snapshots while the generic
`register_extension` APIs remain available. Typed callbacks receive
`ToolCall`, `LlmChatCall`, `StoreCall`, `CriticCall`, or `CompressionCall` and return
`ExtensionResultOutcome`, `CritiqueOutcome`, `CompressionOutcome`, `ExtensionStreamOutcome`, or
`ExtensionErrorOutcome` (or an existing wire mapping):

```python
from echo_agent_sdk import (
    EchoAgentClient,
    ExtensionResultOutcome,
    ToolCall,
    ToolDescriptor,
)


class SearchTool:
    async def execute(self, call: ToolCall, cancellation):
        return ExtensionResultOutcome(call.operation, {
            "kind": "text",
            "success": True,
            "output": f"search: {call.parameters}",
            "truncated": False,
        })


async def register(sdk: EchoAgentClient):
    return await sdk.register_tool(
        "search-tool",
        ToolDescriptor(name="search", description="Search documents"),
        SearchTool(),
    )
```

These classes only decode/encode the `_echo_agent/extension/*` contract; the
Rust Host remains the execution, lifecycle, cancellation, and settlement
authority. Critic callbacks receive the Host-issued `critic_critique` input
(`task`, `answer`, and `context`) and return the structured result
(`score`, `passed`, `feedback`, and `suggestions`). Verifier policy, retries,
and run settlement remain Host-owned; registering a Critic does not implicitly
enable verification.
`register_context_compressor` exposes the typed message set, integer token
limit and optional focus fields without moving compression state into Python;
the Rust Host still owns timeout, cancellation and Session teardown.
`register_agent_component` exposes the closed component/operation contract for
Host-consumed conversation, run/runtime-state, audit, context-projector and
memory-trigger implementations without moving Agent lifecycle state into Python.

## Governance and failure boundaries

The compatibility promise is the explicitly accepted external contract and
accepted facade catalog. Complete Rust public inventory/intrinsic membership
is non-blocking telemetry; it is not a requirement to expose every Rust
identity in all languages. Accepted governance at [eb223986](https://github.com/EchoYue-lp/echo-agent-sdk/commit/eb22398676310fcbd67e9f4babe4afcefd56ed06)
passed [nine original exact-main CI jobs](https://github.com/EchoYue-lp/echo-agent-sdk/actions/runs/39720432985) and a GitHub-only
cold source check with 36 necessary refs: 50 behaviors verified, 75 Findings
resolved and one historical risk explicitly accepted by the human. The
[current acceptance summary](../../docs/sdk/governance/capability-review-remaining.md)
retains the UNKNOWN managed-exit cause and its unchanged ten-second assertion;
risk acceptance is not a repair. The
[capability/evidence matrix](../../docs/sdk/governance/behavior-evidence-matrix.md)
records behavior evidence and remaining scope; catalog `done` labels alone do
not prove a failure path.

A failed Agent/Session/Run close retains its local cleanup state so the caller
can retry; only successful cleanup ends the stream/releases the owner.
Client close retains one asynchronous cleanup owner. Cancelling a waiting caller
leaves that operation available to a second `close()` waiter; `_closed` becomes
true only after connection cleanup and actual child-process exit. The official
async context is exited once: its generator may be exhausted after a failure.
Retries retain the same raw connection and process owner, close stdin, wait
for graceful exit, then terminate/kill as needed, with each wait bounded to
two seconds. A remaining shutdown error leaves the client available for retry;
no empty context exit or signal submission proves that the process exited.
During connection
shutdown, cancelled reverse callbacks propagate cancellation instead of queuing
responses to the stopped official sender. Normal invocation cancellation still
returns the typed cancellation outcome on the live connection.

Duplicate successful close is idempotent. Timeout, cancellation, EOF and Host
exit must not be treated as a successful terminal. Facade resources validate
owner plus complete generation-fenced handles; failed publication rolls back
construction rather than returning an unusable handle.

Factory callbacks echo the Host-reserved instance handle. Child invocations
carry `factory_origin`, binding that instance to its parent registration.
Concurrent same-name factory instances remain independent; no name-based
fallback routes an unknown child. A construction timeout/cancellation retains
cleanup ownership until construction really finishes, including cleanup of a
late-created child.

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

Live `RunHandle.events` is bounded. Its async iterator ACKs at its consumption
boundary when iteration resumes after yielding an event, including iterator
close. This does not prove arbitrary business work launched asynchronously
has finished; callers must coordinate that work with iteration. Merely
buffering a notification does not ACK it. A live terminal event drains its
stream mailbox and ends iteration naturally; other Session feeds continue.
`await run.replay(after_sequence, max_events)` validates and records the page
without ACKing. After applying any required snapshot and consuming the page,
ACK explicitly through `await sdk.notify("echo_agent/event/ack", {"ack": {
"stream": run.stream.to_dict(), "last_processed_sequence":
page["next_cursor"]["last_processed_sequence"]}})`. A gap/snapshot ACK
establishes the monotonic continuation floor; future live facts must be above
it. Python passes extension names without the leading underscore to the
official transport, which supplies the ACP extension wire name.
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

When a valid pending ACK can free a full queue, continuation waits for its
local projection or real queue capacity/closure progress. A failed ACK does
not retire its covered prefix; the existing overflow/replay policy applies.
Events and gaps remain ordered per stream, while other streams and RPCs use
their independent paths. The data-queue bound does not bound memory held by
notification handlers that the official transport has already dispatched.
Cancelling an awaiting Python caller leaves its already-started submission
owned by the client until the official transport actually completes. A late
success commits its own cursor; a late failure is reported to the event loop
exception handler with unknown outcome details. Client close cancels and
settles pending submission jobs instead of leaving detached send tasks.

Typed Host ACP `error.data` becomes `EchoAgentError` with `code`, `retryable`,
`operation`, `handle` and `details` preserved. Ordinary ACP messages are not
parsed into domain failure codes. Python cancellation remains asynchronous;
use `await handle.close()` and observe failures before discarding the handle.
Integers retain the full wire range/canonical decimal text, paths retain their
declared encoding, and Unicode is never byte-truncated. Unknown tagged values
round-trip; malformed known values or missing nested discriminators fail.
Map keys such as `kind` remain domain data rather than an implicit wire tag.


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
