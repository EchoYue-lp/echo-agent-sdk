# State / Trace / Delivery candidate acceptance draft

This is prepared input and assertion scope, not an executed test receipt. Only
existing accepted family operations and public SDK value projection are used.
Framework state/RunStore/DeliveryLedger remain authoritative.

State:

1. Save checkpoint A under scope A and B under scope B, with runtime IDs distinct
   from scope IDs. `state.checkpoint.get` receives the checkpoint conversation/
   runtime ID, not scope ID; assert full DTO (including legacy plan, skills,
   blocked reason, cwd, precise timestamp and messages_json).
2. Compare `state.runtime.list(scope)` to exact singleton runtime ID. Locate
   `_runtime_owners` file by reading its runtime_state_id, never duplicating
   framework ID/path encoding. Assert file checkpoint equals returned DTO.
3. After Host closes, replace `_runtime_owners` directory by regular file in a
   fresh fixture before next-save RPC. Use fault *after Host bootstrap* (bootstrap
   creates this directory). Assert typed FrameworkError with operation and no
   missing checkpoint falsely reported as success. Restore original directory;
   same-host retry must persist and return full unchanged B checkpoint.
4. Corrupt A owner JSON after successful save, preserve bytes for restore. Get A
   must fail explicitly (not null); global owner scans may also fail because
   corrupted authority is not merely a missing record. Restore, no synthetic
   guessed receipt. Do not promise all-or-none scope clearing absent contract.
5. `state.scope.clear(scope A)` removes exact A runtime/checkpoint while B remains
   byte/DTO equal. `_scope_index` is a projection: remove A index JSON only in a
   dedicated fixture, enumerate and inspect authoritative reconstruction. A
   corrupt projection is not permission to delete its owner records.
6. Close actual Host/process, restart new Host on same state root, create fresh
   Agent/Session, get B full DTO. Compare old handle rejection at new generation.
   This proves filesystem restart, not shared in-process Weak/cache persistence.

Trace:

1. `trace.store.open("jsonl", traces absolute directory)`; save complete fixture
   Run with Unicode correlation identities, typed events, all nonzero cache/token
   fields, mixed nullable fields, timings beyond JS safe integer and nanosecond
   RFC3339. Identity metadata must remain, input/error content is sanitized.
2. Load whole Run and exact list_session/list_recent summary field subset.
   Read actual `{run_id}.jsonl` (locate by decoding records rather than duplicate
   path sanitizer). Compare durable record and Host projection including redacted
   synthetic secret. No artifact/content secret leaks through stdout/trace.
3. After opening the store, replace its directory by regular file. Save must
   return typed I/O-derived error, original old Run cache must remain exact.
   Restore directory, retry same store save, assert filesystem bytes changed only
   after real success. A failed *open* must not consume resource quota; close then
   valid open succeeds with max_facade_resources=1.
4. Close Host and actual process, restart and reopen new store on same directory.
   Compare complete sanitized Run again, including all optional fields and exact
   nonzero counters/timing. Counter JSON can project as canonical decimal strings
   in native SDKs; normalize deliberately, never cast JS unsafe numeric values.
5. Malformed Run field types explicitly reject before mutation; retain known
   Unicode fixture state. Retention list pagination is separate future scope,
   not implied by this one-run sample.

Delivery (explicitly memory, same instance only):

1. Open one ledger, enqueue one Unicode payload. Concurrent pre-effect public calls to
   `delivery.claim_next` can return two increasing attempts: the newest claim is
   the sole current authority, and the old exact claim cannot begin an effect.
   After effect_started, repeated/concurrent strict claim_next must return null. Compare
   message_id/route/payload/attempt_id/attempt=1/claimed_at typed RFC3339.
2. Defer exact claim with past deadline and reason; reclaim yields same payload,
   different attempt_id and attempt=3 after the two concurrent pre-effect claims. Snapshot sequence increases for actual
   enqueue/claim/defer/reclaim only.
3. Attempt transition/settle/defer with stale claim1. Each returns explicit error;
   before/after record snapshots and same-instance `delivery.recover` sequence receipts identical, so no
   journal append or state mutation was accepted on stale identity.
4. Transition actual claim2 effect_started, settle completed, recover same ledger.
   Match exact settled outcome/correlation/attempt/sequence; second recover is
   idempotent. `DeliveryLedger` is in-memory, therefore this is not crash-durable
   recovery, external delivery exactly-once, or a cross-process receipt proof.

Required execution after adoption: same scene/fixture and assertions through
TypeScript public Session.call/invoke, Python public Session.invoke, Java public
Session.invoke; immutable real Host bound to current framework successor, no skips.
A framework-only test or catalog.done cannot substitute these consumer results.
