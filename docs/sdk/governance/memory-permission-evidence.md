# Memory and permission consumer evidence

The SDK tests now run the same Store, recall and permission scenarios through
real Host instances in TypeScript, Python and Java. The shared fixture supplies
only a loopback model and two test-owned Store implementations. Framework Store,
MemoryRecaller, PermissionService, Run receipts and journals remain authoritative.
The fixture does not synthesize Host handles, permission decisions or Run state.

The tests are ordinary native test entrypoints:

- `sdks/typescript/test/memory-permission-host.test.js` runs under `node --test`.
- `sdks/python/tests/test_memory_permission_host.py` runs under pytest.
- `sdks/java/src/test/java/com/echoagent/sdk/MemoryPermissionHostTest.java` runs under JUnit/Maven.

They use `ECHO_AGENT_SDK_HOST`; with `ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`, a missing
Host fails explicitly. The existing language gate discovers these files without
new production APIs or private SDK invocation/reflection. Java consumes public
client methods and WireValues; it compiles no alternate SDK implementation.

## Behavior and authority

| Boundary | Actual behavior asserted | State authority and failure path |
| --- | --- | --- |
| Native Store resource | Accepted InMemoryStore creation/put_raw plus resource put/get/search/list, explicit expiry pruning and content dedup. Raw expiry remains until prune; repeated prune/dedup returns zero. Survivor is one exact original key/content. | Framework Store owns mutations; Host resource owner/generation checks reject another Session while preserving the legitimate owner. Equal-score search and dedup survivor keys have no stable hash-map ordering promise. |
| Reverse Store replacement | Accepted set_store changes the same live Session from A to B. Backend invocation records show B-only subsequent reads/search/writes; A's original data remains. A pre-write callback failure leaves no value, then explicit retry commits. | The Session Agent's current Store is authoritative. Registration permits one Store semantic identity at a time, so A unregister precedes B registration. This is not simultaneous multi-Store registration evidence. |
| Real recall | A fresh default memory-enabled Session receives reviewed Active, unapproved Draft and legacy fixtures through accepted generic Store values. Actual model input contains Active once and excludes Draft/legacy; the real journal reports memory_recalled(count=1), contiguous sequences, unique IDs and one final answer. Content/provenance remain unchanged. | Framework MemoryRecaller owns eligibility and telemetry. Raw KV writing does not create an activation API. Existing Store extensions are unregistered before constructing this Session, preserving the intended native Store authority. |
| Permission rules/cache | Same Agent definition creates independent Sessions. Rule source/reason round-trips; exact-argument Session caching, argument-independent SessionTool, Once-not-cached, narrow revoke, clear and closing A without losing B approvals are checked. | Each Session Agent's PermissionService and its scoped approval cache are the sole decision/state authorities. |
| Permission context | Actual HumanLoop reverse request retains request/session/agent/tool IDs, args, requested timeout, working directory, affected files, impact and special-key metadata. Modified args return unchanged. | Framework builds the request and performs the decision; Host/client adapters preserve the accepted projection. |
| Permission effects | A real Run selects a registered writing Tool. Rejection yields zero Tool callbacks and no file. Approved once yields one callback and exact UTF-8 file content, with real framework settlement. | Framework tool/permission pipeline controls execution. The test never treats a direct permission check as proof that an effect ran or was blocked. |

The shared fixture records incoming model requests and reverse Store calls.
Each test writes its complete native result/error/receipt evidence into its
isolated work directory. Its 59 recorded rows include 52 family-operation results
and two complete typed negative results; these are evidence records, not 59
independent test cases. Each language currently contributes one integrated Host
scenario with the listed assertions.

## Contract boundaries retained

**Permission TTL.** Actual default service stats report a 1,800,000 ms TTL and
cleanup of young entries returns zero. These short tests do not prove expiry.
The accepted API exposes no short-TTL setter or clock advance. True expiry needs
a named real wall-clock test or an explicit remaining evidence scope; no test-only
public API or mutated private Host clock was added.

**Approval receipt.** The current accepted PermissionCheck inventory contains
`decision` and `updated_input`. The framework's newer `approval_receipt` field is
not accepted. All three context checks observe its absence. That observation
requires an explicit Rust/Host-only or additive contract disposition; it must not
automatically expand every Rust public field into a three-language promise.

**Wire types.** TypeScript uses the existing explicit f64 WireValue for the known
float memory-confidence seed; an integral native Number would otherwise infer an
integer. Generic Store callbacks must preserve opaque wire types or use a precise
schema before writing projected canonical integer text back. These scenarios
prove their concrete string/map payloads and native resource timestamp handling;
they do not claim arbitrary generic numeric callback round-trip coverage.

**Raw Store versus layered memory.** Accepted set_store replacement proves raw
CRUD authority. No unaccepted set_memory_store/MemoryLayerManager installer,
permission configuration API or parallel task authority was introduced.

## Evidence and delivery status

The initial temporary three-language execution used immutable Host `6711dd17…`;
all three runners completed, each recording 59 rows, and the independent comparison
script verified the same ordered results/errors, callback metadata, Store authority,
recall journal and real file effects. Raw values remain archived. Comparison
normalizes only documented independent roots/timestamps, equal-score result order
and content-equivalent dedup survivor selection after validating original data.

[The archived evidence package](evidence/sdk-memory-permission-20261013/temporary-consumer/ACTUAL-RESULTS.md)
contains scripts, original input hashes, complete receipts and logs, including
unsuccessful draft attempts and their corrected fixture/input causes. The formal
manifest records exact source hashes and focused test results separately from
that earlier temporary proof. All three focused formal tests pass against the matching workflow-shape Host
`cb9b7fca…`, each one native integrated case with zero failures/errors/skips.
The formal subdirectory retains the exact native receipts, logs, Java XML and
source hashes. These case counts do not inflate the 59 evidence rows into tests.

These focused results do not establish final published framework provenance,
full language CI installation or exact main acceptance. The original complete
language gate must run against the final immutable Host and accepted artifacts.
Examples and website product content need no new API text: this change adds
executable evidence for existing accepted behavior and explicitly retains the
scope limits above. The formal SDK document owns this behavior/evidence boundary.
