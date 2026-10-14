# Workflow candidate execution slice (draft, not runtime proof)

All files remain under `/tmp/sdk-workflow-candidate-fixture`; SDK production,
contract/catalog and tests have not been changed. No Cargo or SDK source writes occurred. Native language runs now have actual
red proof in red-evidence.json against immutable Host6711dd17.

`workflow_fixture.py` prepares a five-router declarative graph with the
existing conditional edge, interrupts before `gate` and `end`, two Session
owners and resource quota two. It resolves previous real response identities
into the next call and validates observations; it sends no ACP request and
implements no workflow state authority. Each native SDK runner calls public
`sdk.call` directly against the real configured Host.

Current drafted scenarios cover direct yes/no routes; object resume_exact;
canonical load and restore; label/tags and generation increase; defer without
consumption or execution; duplicate exact and id resume rejection; branch
with updates and parent preservation; child workflow_run_id differs while
graph_revision matches, real consumed fork ancestry remains opaque; normal
resume_with_state; rejection with Unicode reason and consumed checkpoint;
list/load; SharedState get/set/keys/snapshot; cross Session graph/state denial;
Session close fences graph/state access and releases resource quota to the
other Session.

`Checkpoint` is returned as JSON encoded through WireValue. Native u64/i64
projection intentionally returns canonical decimal text. Before object or
sequence checkpoint input, the draft explicitly re-encodes only declared
`generation` and `step_count` numeric fields as u64 after canonical/range checks;
all id, revision, state and timestamp fields remain untouched. The full
checkpoint is encoded with public WireValue.Map recursively, so domain
continuation={kind:node} is ordinary JSON data rather than an intentionally
preserved unknown protocol wrapper. Java passes explicit tags as JsonNode
(the public wire API representation), not as a native Map that would double
encode them. This avoids
mistaking an SDK caller's string-to-integer type error for the actual shape
counterexample.

The independent shape probe obtains a real object checkpoint, converts the
same seventeen fields into the exact framework serde declaration sequence,
and calls resume_exact approve. Expected contract behavior is invalid_value
and unchanged stored checkpoint. The current Host calls serde_json::from_value
without explicit object check, and serde derive can accept a struct from a
sequence. If the real Host accepts it and consumes or advances the stored
checkpoint, preserve that precise red result and report a Finding before any
production patch. The probe has an independent checkpoint so earlier routing
proofs remain distinguishable.

Defaults use framework MemoryStore; no restart persistence guarantee. This
slice does not cover all twenty-four operations: graph streams, extension
Workflow callbacks, cancellation, async multi-owner cleanup and parallel DAG
remain separate proof obligations. No new API, validator, Store, manager,
retry/executor or approval authority is proposed. Branch name belongs to the
fork checkpoint; the current continue_run source only copies execution id,
generation and parent id, so the draft does not invent child branch metadata
propagation. Any unexpected result must be compared with the actual framework
path before expanding the contract.

Intended final destinations once coordinator approves copy and real runs:

- shared/testing/workflow_fixture.py
- typescript/test/workflow-host.test.js
- python/tests/test_workflow_host.py
- java/src/test/java/com/echoagent/sdk/WorkflowHostTest.java

Runner helper subprocesses only resolve already captured input/output JSON;
those results do not stand in for native client calls. Each language uses an
isolated temporary work/state directory and retains failures for diagnosis.
The integer re-encoding is explicit caller adaptation, not a hidden alternate
checkpoint model.


Actual old-Host results: 48 scenario records = 47 Workflow calls + 1 Session
close per language. Both resume_exact and resume_with_state accept the
seventeen-field sequence, consume the real canonical checkpoint, and advance
to the end interrupt. The state-update probe actually inserts
unapproved_mutation=must-not-execute and follows the no branch. All three SDK
raw outputs show original parent checkpoint load=null afterward. This is a
failure, not a passing contract scenario. Expected array input rejection remains
unchanged in the fixture. The verifier checks legal route/context/claim/branch,
state/owner/quota assertions before reporting the first array-shape failure.
The independent resume_with_state side effect is additionally checked against
its saved raw output. Green needs a different immutable repaired Host.
