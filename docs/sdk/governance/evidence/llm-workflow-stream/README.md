# LLM and Workflow stream native consumer evidence

2026-10-13 focused candidate: TypeScript, Python and Java each passed three
native test groups, with zero failures/errors/skips. The three groups cover five
LLM modes plus a real follow-up history turn, registered Workflow execution and
stream lifecycle, and a pure declarative framework graph. Results are from the
same immutable real Host binary, not a simulated Host or private client shim.

Binary:
`lp-agent/.worktrees/admin/binaries/echo-sdk-channel-resource-admission-host`,
SHA-256 `07442919df0108592f93585a76f6a9cbc429f279a81876a70b56b959f53c6c2b`.
Every scenario has an independent directory below
`lp-agent/.worktrees/admin/llm-workflow-stream-work`. [raw/receipt.json](raw/receipt.json)
binds source reference digests, binary digest, nine original observations and
final test logs. [Java Surefire XML](raw/TEST-com.echoagent.sdk.LlmWorkflowStreamHostTest.xml)
was copied immediately after the run: three tests, zero failures/errors/skips.

This binary predates the next Channel forward contract and later dependency
repair. These results prove only the existing LLM/Workflow consumers exercised
here. They do not prove the later full dirty source tree matches this binary,
new Channel operations, all provider combinations, full Rust public inventory,
all-repository gates, SDK release or merged-main completion.

## Authority and public consumer paths

The shared fixture contains deterministic DTO payloads, Host configuration and
field assertions. It drives no Agent, React loop, graph, scheduler or stream
state machine. Its model URL is deliberately unreachable; a missing registered
provider cannot silently succeed by falling back to a fixture HTTP model.

The native language SDKs register LLM implementations through their public
`registerLlmClient` / `register_llm_client` helper. A real Session starts a real
Run. The existing Host `ExtensionLlmClientProxy` calls the implementation using
`llm_chat` or `llm_chat_stream`; the framework owns the final Run outcome, usage
and transcript. A non-streaming result is adapted to a framework chunk only when
it has a bounded finish reason (`extension_bridge.rs`, `chat_response_into_chunk`).
SDK-owned callback stream writers send the two canonical chunks and one complete
notification to the Host-issued stream; no fabricated Run receipt is returned.

Workflow uses the already accepted, closed family operations:

| Operation | Actual path verified |
| --- | --- |
| `workflow.extension.run` | Registered Workflow component receives `workflow_run`; framework restores its `WorkflowOutput`. |
| `workflow.extension.run_stream` | Registered Workflow receives `workflow_run_stream`; canonical NodeStart/Token/Completed or NodeError/failure pass through the Host pull-stream owner. |
| `workflow.graph.run_stream` | The loader builds a real pure router graph; framework emits NodeStart/NodeEnd/Completed events. |
| `workflow.graph.cancel` | Wrong Session rejects; owner cancels the real graph token and the next execution preserves the framework cancellation cause. |
| `workflow.stream.next` | Owner checks, monotonically increasing sequence, final terminal and tombstone. |
| `workflow.stream.cancel` | A parked source is cancelled and its eventual pull terminal is `cancelled`. |
| `workflow.stream.close` | Parked callback finishes before the operation settles; a later pull reports `closed_handle`. |

The graph engine remains framework-owned. `FacadeStreamRuntime` owns the issued
pull-stream lifecycle and sequence. The extension fixture supplies events for its
own registered Workflow implementation; it does not replace the graph engine.

## Exact native observations

| Per-language group | Real callbacks | Relevant outcome |
| --- | ---: | --- |
| LLM | 6 | Nonstream and follow-up history complete; stream completes; missing finish and callback exception fail; cancelled callback observes cancellation. |
| Workflow extension | 5 | One nonstream run, successful stream, failed stream, parked cancel and parked close. All five callback bodies finish; both independent producers finish. |
| Pure graph | 0 | A registered model guard stays uncalled; owner checks and cancellation use the real graph. |

Across the three languages this is nine native tests, 18 actual LLM callbacks,
15 Workflow callbacks, six finished Workflow producers and three untouched
registered graph-model guards. These counts are scenario evidence, not a count
of the accepted API surface or a promise to support every possible provider.

LLM request evidence retains all actually present fields: Unicode system/user
messages, the real second-turn assistant history, temperature `0.375`, max tokens
`513`, the tool definitions and parameter schemas, and typed cache hints.
The fixture validates the closed request field set and every tool's complete
four-field shape. Optional request fields absent from these real framework turns
are not claimed to have been exercised. Request invocation IDs and deadlines are
retained without rewriting them into cross-process constants.

Nonstream response payloads include assistant name, Unicode content, finish
reason, usage (`17 + 5 = 22`, with cached-token detail), and typed raw provider
data including a fractional value, null, nanosecond timestamp text and an integer
identity preserved as text. These are the exact submitted DTO fields retained
in the shared scenario file. The actual framework consumer evidence is the
single final answer, Run terminal and exact 22-token usage; no public raw-response
getter is invented to claim further echo-back coverage. The stream writer records
notifications `chunk/1`, `chunk/2`, `complete/3` with finish and usage. The missing
finish test fails with the framework's original bounded-finish reason. Throwing
the callback's concrete `source-cause 中文🙂` produces a failed real Run retaining
that message. Cancellation reaches the real registered callback and all six
LLM handler `finally` blocks finish.

Workflow success yields pull sequences `1/2/3/4` with exact NodeStart, Token,
Completed and Complete fields, preserving node name, result, `total_steps=1`,
and elapsed `{seconds:"0",nanos:456}`. Its direct output retains the detailed
step fields and native `{secs,nanos}` duration representation. Failure yields
`NodeError/1` followed by `failed/2`, with the same concrete source message.
The existing adapter projects that failure as `framework_error`; this evidence
does not claim that all original extension details survive the adapter.

Two Sessions verify stream and graph ownership. The foreign stream pull rejects
with `invalid_value` and leaves callback count exactly `2 → 2`. Foreign graph
run/cancel reject before a later owner run still completes naturally. The owner
graph cancel returns `{cancelled:true}`; a subsequent graph stream fails with
typed `cancelled`, original “cancelled after 0 steps” cause and framework terminal
metadata. It is not rewritten into ordinary successful EOF.

The extension cancel and close scenarios deliberately keep the source callback
parked until the client initiates lifecycle control, then explicitly release it.
All callback bodies and producers are joined/finished; source cancellation
signal observation is specifically proved by the LLM scenario. The parked
Workflow scenarios prove Host ownership and final settlement, without asserting
that a detached SDK producer received a cancellation token after callback return.

## Existing helper and return-shape limits

Python's public typed `AgentComponentCall` currently retains its request,
extension, invocation ID and deadline, but omits the Host-issued `stream` field.
Consequently this Workflow stream test uses the existing public
`register_extension("agent_component", ...)`, validates every request with
`AgentComponentCall.from_payload`, and passes the original public payload's
`stream` to the public `stream_writer`. TypeScript's typed helper retains stream;
Java's typed `AgentComponentCall.raw()` retains it. No SDK implementation or
public API changed. This is a precise remaining typed-helper projection gap,
not an absence of an accepted public Workflow streaming consumer.

For an ephemeral Workflow extension stream, both the first `stream.close` and
the retry currently return `false`. `finish_record` first closes the ephemeral
anchor; registry resource close cascades to its stream, so the subsequent direct
stream removal returns false. Every native test proves callback completion and
subsequent `next → closed_handle`. The boolean is preserved as observed; no
successful `true` is fabricated and no production semantics were patched.

## Reproduction and delivery scope

Set `ECHO_AGENT_SDK_HOST` to the binary above,
`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`, and `ECHO_AGENT_SDK_LLM_WORKFLOW_ROOT` to the
LP-local isolated runtime directory. Run the following in each language root:

```sh
# sdks/typescript (current built public SDK dist)
node --test test/llm-workflow-stream-host.test.js

# sdks/python
PYTHONPATH=src .venv/bin/python -m pytest -q -s tests/test_llm_workflow_stream_host.py

# sdks/java, with exclusive Maven ownership
mvn -q -Dtest=LlmWorkflowStreamHostTest test
```

Python Ruff checks and format checks, JavaScript syntax and diff whitespace
checks passed. Existing SLF4J no-provider diagnostics remain in Java's original
log. Early authoring mismatches (raw Python outcome envelope, expected graph
cancel error code and ephemeral close boolean) were resolved by reading the
existing source; no production bug repair was made. Candidate logs are preserved
separately from final results.

Only the four new test/fixture files and this new SDK-owned evidence directory
belong to this slice. Managed retirement evidence remains frozen. Examples and
website need no update because this slice changes neither the public API nor
product behavior. No contract/schema edits, Cargo invocation, stage, commit,
push or merge was performed.

Evidence footer: TypeScript **3/3**, Python **3/3**, Java **3/3**, all zero failure,
error and skip; nine actual native groups using one verified immutable binary.
Remaining final-snapshot, independent review and repository delivery gates are
separate from this focused evidence.
