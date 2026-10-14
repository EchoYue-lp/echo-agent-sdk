# Python typed AgentComponent stream projection repair

This delta supersedes the historical Python typed-helper limitation described in
the parent LLM/Workflow evidence. That earlier three-language evidence remains
unchanged and hash-verifiable. The accepted protocol already supplied a stream;
the defect was its loss when Python constructed `AgentComponentCall`.

The implementation adds only `stream: WireHandle | None = None` and restores it
with the existing `_payload_handle(payload, "stream")` converter. Missing or
null remains `None`; explicit handles retain ID, kind and exact generation. No
wire operation, accepted contract, scheduler, state store or cancellation policy
is added. Other callbacks and SDK ACK/session/feed behavior are untouched.

## Actual before / after consumer

Both runs use the same immutable real Host:
`echo-sdk-channel-resource-admission-host`, SHA-256
`07442919df0108592f93585a76f6a9cbc429f279a81876a70b56b959f53c6c2b`.
The source class before and after is retained in
[before-AgentComponentCall.py.txt](before-AgentComponentCall.py.txt) and
[after-AgentComponentCall.py.txt](after-AgentComponentCall.py.txt), with whole-file
and class hashes in the corresponding source JSON files.

The native public test now registers through `register_agent_component`, using
`AgentComponentDescriptor("workflow", ..., supports_streaming=True)`. The callback
receives the public typed call and uses `call.stream` with the public
`stream_writer`; the test no longer extracts stream from a raw payload.

Before the fix, [python-before.log](python-before.log) records one actual selected
test failure. [python-before-observed.json](python-before-observed.json) records
the real first pull as `failed`, sequence `1`, code `framework_error`, cause:

```text
extension bridge failure extension_failed: 'AgentComponentCall' object has no attribute 'stream'
```

There were no successful Workflow stream items. This is an actual typed-consumer
failure from the official SDK / Host callback path, not a DTO-only assertion or
an unsupported backend fixture. The observer and scenario files are preserved
without rewriting random handles or timestamps.

After the two-line field restoration, [python-after.log](python-after.log) records
five passed tests: three actual native Host groups and two focused DTO
regressions. The same typed Workflow callback produces NodeStart, Token,
Completed and natural Complete, with exact sequences `1/2/3/4`. Every stream call
retains stream ID/kind/generation, registered extension identity, invocation ID
and deadline. Nonstream calls retain `stream=None`.

The native negative and lifecycle cases remain intact: NodeError then failed
terminal preserves the concrete source cause; foreign Session pull rejects with
zero callback-count change; parked cancel produces a cancelled terminal; close
settles all five callback bodies and both producers, and subsequent pull returns
`closed_handle`. Existing ephemeral-close booleans remain `false`. The LLM and
pure graph groups also pass through the same real binary after the Python repair.

The two focused regressions check missing/null stream, exact generation
`9007199254740993`, original extension and invocation/deadline fields, and
rejection of a noncanonical or overflowing explicit generation. They introduce
no additional production authority or API.

## Verification scope

Run with `ECHO_AGENT_SDK_HOST` set to the immutable binary,
`ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`, and
`ECHO_AGENT_SDK_LLM_WORKFLOW_ROOT` beneath the LP-local isolated runtime directory:

```sh
PYTHONPATH=src .venv/bin/python -m pytest -q -s tests/test_llm_workflow_stream_host.py
```

The selected before run uses the same test's `-k workflow_extension`; the after
run executes all five tests. [ruff.log](ruff.log) records passed checks and format
checks for the complete Python client and modified consumer test. The separate
[python-suite.log](python-suite.log) is the attempted Python suite without a
configured Host: **3 failed, 265 passed, 54 skipped**. Two Gap-ACK Host cases
attempt to spawn `HOST=None` without a skip guard; the existing lifecycle ACK
consumption test raises `KeyError` for `stream-1`. These failures were reported
to the parent and ACK owner; this narrow repair does not modify their tests or
ACK implementation. The attempt is not a green final gate, and its conditional
skips are not additional native consumer proof. The focused native run above
supplies the actual Host proof. Final combined SDK validation remains required.

The delta receipt binds all original logs, native observations, scenario inputs,
class snapshots and test/source hashes. Current full client-file hashes may
advance when other authorized SDK tasks edit separate parts of that shared file;
the class snapshot identifies this repair's exact production boundary. This
evidence does not prove a later generated Channel contract, dependency revision,
full Rust gate, remote CI, release or main-merge snapshot.

No TypeScript, Java, frozen managed backend, accepted wire schema, examples or
website behavior changed. Documentation is this SDK-owned correction of the
previous helper limitation; existing API examples remain valid. No Cargo,
stage, commit, push or merge was performed by this slice.
