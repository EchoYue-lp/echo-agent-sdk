# Eval, Improvement and Topology native consumer evidence

Status: the finite native slice passed on the fresh callback-context Host.
TypeScript, Python and Java each passed all three groups with zero failures,
errors or skips. The original Eval failure on Host `0435e0b5…` remains archived;
the new `0a0fe4bd…` binary passes the same strict cancellation/cleanup path.
This evidence does not claim final framework pin or release/main acceptance.

New Host:
`lp-agent/.worktrees/admin/binaries/echo-sdk-callback-context-candidate-host`,
SHA-256 `0a0fe4bd910fd0995b1b4cc30f4ccaa79ddc4c194116397127bec12ba6e89a17`;
accepted source-contract digest remains
`sha256:3cd835dbf40abf1d0aec04e02f3b51411631fcaa50ba7e5f0f2d2d2f500ed9a6`.
The compiled bridge snapshot is `93de426a…`; [receipt.json](receipt.json)
binds its archived source, test/client references, immutable binary and raw files.

The shared fixture prepares only fixed cases, trace data, Host configuration and
assertions. All three native language test files independently implement and
register the actual AgentFactory / CustomAgent callbacks; no Python callback
backend substitutes for TypeScript or Java execution. Runtime directories and
Host TMPDIR stay below `lp-agent/.worktrees/admin/eval-improve-topology-work`.

## Intended finite accepted consumer slice

Eval uses accepted `EvalRunner::new`, configuration field routes for
`timeout_secs` and `workspace_root`, `with_run_store`, `with_grader`, direct
`run`, factory `run_all/run_all_async` and `evaluate_run_constraints`. Fixed
success/failure cases have an actual constructed instance per case. The grader
has two fixed assertions, optional null/present trajectory, valid confidence
values and malformed answer failure. A configured graded runner executes a
real assertion-based case rather than merely constructing a grader resource.

Improvement uses its existing max iterations, holdout and threshold field
routes. Four fixed cases split into two train/two holdout, yielding four actual
factory instances for threshold early stop and eight for two maximum iterations.
Only the accepted LoopResult fields and real iteration/report/factory facts
are asserted. No stop_reason, loop cancellation API or automatic prompt
application is invented.

The same supplied Unicode/tool Run is saved through the actual RunStore,
converted to ShareGPT, analyzed, saved as a real trajectory JSONL file and
reopened. The directory fault preserves original JSONL bytes while a save
fails; restoring the directory recovers the original list/stats. Nonterminal
save returns false. Completed/failed Analyzer inputs retain actual optional
Run error data without inventing a failure-cause field on the critique.

Topology uses the actual owned Tracker, exact metadata nodes, calls/duration,
nodes/edges/stats and JSON/Mermaid/DOT exports. Foreign Session mutation fails
before affecting the original graph, clear resets all projections, and the
closed handle rejects. Automatic timestamps and map iteration order remain raw
facts and are excluded only from stable cross-process comparison.

## Actual old-Host Eval failure

Immutable input Host:
`lp-agent/.worktrees/admin/binaries/echo-sdk-channel-forward-candidate-host`,
SHA-256 `0435e0b57fb26184dc98fe69f9d071826bee1c597e4182a89feead4dfef9f802`.

The runner's setter/getter records timeout `1`. The language's actual factory
callback is still in stream setup, awaiting its cancellation signal. On this
binary the independent Eval cancellation does not reach that callback.
Python's ten-second callback wait fails itself; its EvalResult records Timeout,
“Turn settled after timeout: failed”, roughly 10 seconds and no observed cancel.
Java and TypeScript reach the framework's twelve-second cancellation grace;
the result retains the workspace with “Turn did not settle within 12000 ms
cancellation grace”, roughly 13 seconds. Any later cancel notice after driver
drop does not prove delivery of the original one-second token.

The actual source chain is EvalRunner's invocation cancel/runtime/workspace →
AgentTurnDriver's `execute_stream_with_invocation_context` → default Agent trait
waiting for ordinary stream setup → ExtensionCustomAgentProxy's ordinary
`run_stream`, which uses connection cancellation and Session-only metadata.
The SDK proxy lacks the existing invocation/cancel override. This is an SDK
adapter defect; the native tests do not patch the framework or route around it.
The parent delegated the narrow production repair to the bridge owner.

The original Java class result is retained in [raw/java-before.xml](raw/java-before.xml):
three tests, **one failure, two passes, zero errors/skips**. Its Improvement and
Topology groups passed with actual 12-child closure and original tracker
observations. Python's equivalent full grouped run has one deadline failure
and two passes. Original callbacks, results and immutable input scenarios are
archived as `*-before-*-observed.json` and corresponding scenario files.

The earliest TypeScript run printed three passes because its first oracle only
checked a cancel boolean and Timeout. Inspection found that cancel arrived
only after grace/driver drop with the workspace retained. The stronger oracle
correctly rejects that same unchanged observation; this failed recheck is
[raw/typescript-first-strengthened-oracle.log](raw/typescript-first-strengthened-oracle.log).
The early printed pass is not final evidence. This meaningful test authoring
mistake was corrected without rewriting the original log or observations.

The current oracle requires original cancellation, a pending Eval request and
still-existing workspace while native cleanup is deliberately held, real callback
finally and factory close, then cancelled settlement without retained workspace
or grace failure. The test releases cleanup after recording these facts so
failures remain bounded. It does not turn a synthetic stream Cancelled into proof
of a finished language callback.

## Fresh native result

| Language | Native groups | Rows per Eval / Improve / Topology | Eval cancellation duration | Factory created / closed / executed |
| --- | ---: | --- | ---: | --- |
| Python | 3 passed | 23 / 27 / 18 | 1004 ms | 15 / 15 / 15 |
| TypeScript | 3 passed | 23 / 27 / 18 | 1007 ms | 15 / 15 / 15 |
| Java | 3 passed | 23 / 27 / 18 | 1012 ms | 15 / 15 / 15 |

Every language retains its own callback objects and owns the real child identity
map. Eval creates three distinct children; Improvement creates twelve. All
fifteen execute and close exactly once, with all fifteen execution `finally`
blocks complete. The shared fixture performs no native callback execution.

In each timeout run, the actual native cancellation signal is observed once
while the original Eval request remains pending and its generated workspace
still exists. Explicit cleanup release then finishes the callback/child close;
EvalResult has `success=false` and exactly the timeout/cancelled settlement facts,
with neither retained workspace nor cancellation-grace failure. This is the
one-second invocation token, not a late notification after thirteen-second
driver abandonment. The native callback contexts now preserve three distinct
`run_id == turn_id == execution_id` triplets with the original Session ID.
The existing wire does not carry full working-directory/history/runtime-state
objects: getter/filesystem observations and retained invocation ownership do not
claim a newly added callback working-directory field.

The two fixed normal Eval cases produce pass/score 1 and fail/score 0, with their
original metrics and violations. Grader replies preserve both fixed assertion
IDs, pass booleans, confidence `0.875/0.625`, evidence/reasoning and overall
assessment. Null and present trajectory use the same real grading consumer;
malformed JSON produces zero pass rate with the original parser cause. The
configured graded runner records score 0.5 and `success=true`, matching the
framework's existing `pass_rate >= 0.5` policy; the test does not substitute its
own threshold. Trace constraints observe the supplied original tool/file Run.

Improvement's threshold `0.75` produces one iteration; threshold `1.1` with max
two produces two. Each LoopResult and iteration has exactly its accepted field
set; `train_score/best_score=1.0`, independent train/holdout reports and real
factory counts explain termination. No fictional stop reason is reported.

Trajectory evidence verifies actual JSONL bytes: one newline-terminated entry,
all nine declared fields, same ShareGPT messages including paired Unicode tool
call/result, model, Run/Session identity, 22 tokens, one tool call and 123 ms.
Directory-blocked save returns the original typed error without changing those
bytes. Reopened list/stats agree; nonterminal input returns false without a new
entry. Analyzer preserves completed/failed success plus actual detected issues
and suggestions, without pretending its critique exposes an independent cause
field.

Topology exports preserve two nodes' full label/type/metadata and the original
edge's latest label, two calls and 123 ms. Parsed JSON exactly agrees with the
typed snapshot, including its genuine automatic timestamp. Foreign owner clear
leaves it unchanged; original clear resets nodes/edges/stats together. Closed
access reports `closed_handle`.

The [cross-language comparison](raw/comparison-final.log) passes for all nine
actual observations. It compares fixed results/metrics, LoopResult, ShareGPT,
Analyzer and Topology fields. Runtime durations, automatic timestamps and map
iteration order are kept in the original evidence and excluded only where they
cannot be equal across independent processes. The exact topology edge duration
and trajectory duration are separately asserted against their fixed inputs.

Java's original [final Surefire XML](raw/TEST-com.echoagent.sdk.EvalImproveTopologyHostTest.xml)
was copied immediately after Maven returned: three tests, zero failures/errors/
skips. Original final logs and every native input/observation are under `raw/`.
Existing SLF4J no-provider diagnostics remain in the Java log. Python Ruff and
format checks and JavaScript syntax check pass.

## Reproduction and remaining delivery gates

Set `ECHO_AGENT_SDK_HOST` to the new immutable binary above,
`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`, and `ECHO_AGENT_SDK_ANALYSIS_ROOT` beneath the
LP-local runtime directory. Run all three groups in each language root:

```sh
# sdks/python
PYTHONPATH=src .venv/bin/python -m pytest -q -s tests/test_eval_improve_topology_host.py

# sdks/typescript, current built public SDK dist
node --test test/eval-improve-topology-host.test.js

# sdks/java, exclusive Maven ownership
mvn -q -Dtest=EvalImproveTopologyHostTest test
```

These focused groups pass on this exact binary. A later binary is not assumed
to equal this candidate or the final published contract. The broader bridge
follow-up for cancellation followed by a late stream setup acknowledgement is
an independently owned interleaving; this strict held-callback case does not
claim every stream acknowledgement ordering covered. Published framework pin,
generated final contract, full SDK gates, independent review, remote CI and
exact-main completion remain separate requirements.

No production clients, Host/framework source, generated contracts, prior
Managed/LLM/ACK evidence, Cargo, stage, commit, push or merge was modified by
this lane. The only files owned here are the new fixture, three new native tests
and this SDK-owned evidence directory.
