# Python and Java Channel forward native evidence

The fresh candidate Host passed both native language suites on 2026-10-14:
Python **3 passed**, Java **3 tests / 0 failures / 0 errors / 0 skipped**. Python
also has zero skips. The original final logs and immediately copied Java
Surefire XML are retained in each language's `fresh-candidate/` directory.

Binary: `lp-agent/.worktrees/admin/binaries/echo-sdk-channel-forward-candidate-host`,
SHA-256 `0435e0b57fb26184dc98fe69f9d071826bee1c597e4182a89feead4dfef9f802`.
Accepted source-contract digest:
`sha256:3cd835dbf40abf1d0aec04e02f3b51411631fcaa50ba7e5f0f2d2d2f500ed9a6`.
[Native receipt](native-receipt.json) binds test/shared/client source digests,
raw artifacts and the original Host build manifest. This is focused candidate
runtime evidence; final published framework pin, complete SDK gates, remote CI
and exact-main delivery remain separate.

## Actual language consumers and observations

The tests follow ADR 0035 and the approved Channel forward design. They use the
single `sdks/shared/testing/channel_forward_scenarios.json` and comparison tool.
Each language implements its own real ChannelPlugin / MessageHandler callbacks
through the public SDK registration methods. The Python startup fixture writes
only Host configuration, with an unreachable model URL. It is not a callback
backend, simulated Host or private SDK shim.

| Per language | Actual callbacks | Actual `finally` completions | Result |
| --- | ---: | ---: | --- |
| Core, fidelity and invalid inputs | 10 | 10 | Single/stream/reply, four attachment kinds, max u64, optionals, 12 invalid inputs |
| Owner, binding and start state | 6 | 6 | Two original manager entries, foreign/stale handles, start rollback, batch binding and restart |
| Cancellation and close | 6 | 6 | Cancel A preserves independent work, full buffer close, caller drop/late cleanup, resource close and original error |

Each language therefore executes **22 genuine handler callbacks**, with all 22
`finally` blocks complete. Each callback actually awaits the same manager's
`ids` and `list`, yielding 22 metadata pairs. Plugin callbacks are recorded
separately and are not counted as inbound handler consumption.

The core receipt captures exactly one `channel_handle`, one
`channel_handle_stream` and one explicit `channel_reply`. The original stream
callback awaits a first chunk and second terminal item, finishes its own
`finally`, and only then returns its actual reverse RPC result. There is no
independent detached producer or fabricated completion response. Public pull
returns two different outbound items plus one natural complete with exact
sequences `1/2/3`; streaming does not implicitly invoke reply. Explicit reply
returns null. Both languages' original core outputs/counters match the shared
expected receipt.

Inbound Unicode, timestamp `18446744073709551615`, four attachment kinds and
binary base64 round-trip through the real callback. Empty filename and reply
fields remain empty; missing/null optional fields become absent/None, and missing
attachments become an empty list. Fidelity calls occur after the core count
snapshot so they cannot distort core callback comparison. Every shared invalid
input (timestamp number/negative/leading zero/overflow/null/missing, unknown field,
null attachments, bad attachment kind/base64, chat type and channel mismatch)
returns `invalid_value` before any handler callback; all 12 recorded deltas are
zero in each language.

## Owner and lifecycle proof

Extension registration remains connection-scoped and globally unique by semantic
identity. Two Sessions therefore reuse the same actual plugin/handler extension
registrations in two independent framework manager entries with the same channel
ID. The second legitimate forward invocation has a distinct message ID, observed
by the real handler. Wrong Session manager/stream access rejects; older
manager/stream generation rejects with `stale_handle`. A future handler generation
rejects as `invalid_value` before plugin start effects. No synthetic owner context
or duplicated global handler registry was introduced.

Batch start with two original bindings that disagree rejects before either
plugin starts. Unstarted, starting and failed-start rollback reject forward
input. A later successful start accepts the same entry. Stop of manager A does
not prevent a legitimate handler call through manager B, and restart of A again
uses its original binding. Wrong family and wrong digest reject; the original
owner still completes afterwards.

Cancellation A reaches the original retained callback while an independent
single call still succeeds. Pull/close remains pending while that callback's
cleanup gate is held. Only explicit release lets its real `finally` finish and
permits terminal cancellation. The unconsumed stream buffer close likewise
signals cancellation before waiting for the retained callback. Subsequent pull
returns `closed_handle`.

A dropped caller stops waiting without claiming that the callback finished.
Channel stop is then held pending until the original callback observes cancel
and completes cleanup. The entry subsequently restarts successfully. Concrete
native handle/reply errors preserve the shared source messages as
`framework_error`; the fixture does not invent more-specific error codes.
Resource close also remains pending through the original callback cleanup,
then tombstones the manager; later forward input rejects with `closed_handle`.

The raw observations retain actual inbound/reply DTOs, extension and invocation
identities, deadlines, plugin side effects and metadata results. These counters
prove the exercised local callback bodies, not an external OS process or
unobserved framework Run receipt.

## Matrix boundary and original failed attempts

This file records the Python/Java grouped native scenarios. The separate
TypeScript owner supplies its more granular native tests and the single shared
three-language comparison. Internal quota/setup/publication failure and
handler-close fault injection remain framework/Host-unit obligations. No public
fault setter or fabricated handler-close callback is added to obtain them.
A complete twelve-group result requires those separate internal proofs; these
six language tests alone do not claim the entire lifecycle matrix closed.

The first two candidate attempts are retained as `first-run.log`, `second-run.log`
and corresponding Java XML. They exposed test authoring assumptions, not repaired
production bugs: trying to register two different plugin descriptors with one
global channel identity; calling resource close with the source-signature helper
rather than its existing generic facade route; treating a manager returned inside
a map as an already constructed Python handle; and expecting future generation
to classify as old/stale. Tests were corrected against existing source authority,
then both entire suites ran again successfully. No production source changed.

Python Ruff/format and diff whitespace checks pass. Java's existing SLF4J
no-provider diagnostics remain in its original final log. Static preparation
files remain historical and do not substitute for the fresh runtime records.

## Reproduce and deliver

Set the binary above as `ECHO_AGENT_SDK_HOST`,
`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`, and use LP-local paths for
`ECHO_AGENT_SDK_CHANNEL_FORWARD_ROOT` and
`ECHO_AGENT_SDK_CHANNEL_FORWARD_EVIDENCE`. The latter receives `python.json` and
`java.json` in the same shape as the TypeScript receipt.

```sh
# sdks/python
PYTHONPATH=src .venv/bin/python -m pytest -q -s tests/test_channel_handler_forward_host.py

# sdks/java, exclusive Maven ownership
mvn -q -Dtest=ChannelHandlerForwardHostTest test
```

The shared comparison writer checks the three genuine receipts, identical core
outputs, exact terminal/sequence/callback counts and all twelve invalid-input
errors/deltas. It does not manufacture language observations.

Only the two new tests and this evidence directory were changed by this lane.
Existing clients, accepted schemas, LLM/Workflow/Managed/ACK evidence, examples
and website behavior were preserved. Public API documentation/examples are
owned by the protocol/Channel implementation lane; this test-only lane introduces
no additional API. Maven ownership was released immediately after archiving the
successful original XML. No Cargo, stage, commit, push or merge was performed.
