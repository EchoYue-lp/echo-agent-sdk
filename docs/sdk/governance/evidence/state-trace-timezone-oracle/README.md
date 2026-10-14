# State/Trace timestamp oracle repair

The original CI failures compared timestamp strings that represented the same instant. Framework `00782b8aeddefe2ea8c566922cbdee6aedc80d70` owns `DateTime<Utc>` and serializes these fields through `local_rfc3339`/`option_local_rfc3339`: output uses the Host local offset, with nanoseconds preserved. Checkpoint `timestamp` and Trace/RunSummary `started_at`/`finished_at` therefore do not promise the input's `+08:00` spelling. This test-only repair leaves Host serialization, SDK public APIs, persistence authorities and WireTimestamp unchanged.

The full original cancelled CI log is retained in `before-ci/language-cancelled.log` (166,086 bytes, SHA `d4a85d86d1af20f0025d157e66dceed3b4893059c2ae67da2370f254f4dad2ca`). It contains the old State and Trace failures at `not ok 226` and `not ok 227`; cancelled is not a passing terminal. The pairs `2026-10-13T13:00:00.123456789+08:00` / `2026-10-13T05:00:00.123456789+00:00` and corresponding finished time represent exactly `(1791867600,123456789)` and `(1791867601,987654321)`. The offline observation keeps both raw strings. It is not relabeled as a new Host run.

## Exact comparison boundary

The shared test fixture compares copies of whole DTO objects. Only the explicit top-level Checkpoint `timestamp`, or Trace/Summary `started_at` and `finished_at`, use exact integer epoch seconds plus independently captured nanoseconds. Parsing uses ASCII fullmatch, whole-second calendar validation, signed integer offset arithmetic and up to nine fractional digits; it never uses JS Date milliseconds, Python fractional microseconds, floating point, rounding or truncation. UTC Host output must end in `+00:00`; Shanghai output must end in `+08:00`. The input parser accepts valid `Z`, while output checks retain the framework serializer's actual offset form.

All other keys and values remain complete obligations, including unknown fields, nested time-looking strings, optional presence versus null, decimal wire integers and array order. Only `finished_at` may be null. Original payloads, returned DTOs, trace JSONL bytes and errors are preserved. The new full 14-field Summary assertion also catches extra or missing keys. Original state authority, fault preservation, scope clear, index rebuild, resource quota, redaction, repeated-save and real Host restart checks remain. Delivery behavior is untouched.

The original payload's `+08:00` fields and all original non-path fields stayed unchanged. Separately prepared fixtures naturally use different owned root paths, and the candidate adds one `trace_summary_expected` oracle object. The input files are not claimed to be byte-identical. The before source and executed five-file source copies retain the actual cuts.

## Actual validation

| Validation | Actual result |
|---|---|
| Pure oracle tests | 14 passed, 0 failed |
| TypeScript: State and Trace × UTC/Shanghai | 4 passed, 0 failed or selected skips |
| Python: State and Trace × UTC/Shanghai | 4 passed, 0 failed or selected skips |
| Java: State and Trace × UTC/Shanghai | 4 passed; each original XML reports 2 tests, 0 failures/errors/skips |
| Selected timestamp observations | 84 raw fields, each exact instant/nanos preserved and local output offset checked |

Each of the 12 scenes performed a real Host cold restart (generation 2). Trace scenes retained original JSONL bytes and full Summary records; failure scenarios retained the original unchanged-file assertions. There was no global CI timezone setting: each command explicitly used UTC or Asia/Shanghai. TypeScript's production distribution was compiled once before its native tests; only the relevant test scene selectors ran. Java used local **Oracle JDK 21.0.5 and Maven 3.9.0**, not CI Temurin 17. The original command receipts and two fresh Java XML files are retained, with generation times verified inside their actual run windows.

The immutable Host was hashed using 1 MiB streamed reads: 706,935,504 bytes, SHA `0f1ecf23a2875621caf5128327046cbf6592ce61665c6c6872aed33fdf1931df`. `executor-environment-provenance.json` records the original six tool-call environment assignments and ties them to actual receipt hashes and native timezone observations. It is a later archive of original tool-call arguments, not a fabricated runtime environment dump.

Surefire's actual Java temporary roots remained `/var/folders/...` despite the requested Maven tempdir property. The original emitted work paths are recorded in the logs and XML freshness files; selected raw artifacts were copied byte-for-byte into this packet. Their copies are not represented as the original execution roots. No runtime repository, store tree, dependency directory, Host binary or target output is archived here.

The five Time sources stayed unchanged from their reviewed freeze through the runs. During the Java lease, all 37 captured repair-source files, 61 Rust inputs and Cargo.lock also matched before/after. Other owners could subsequently change their own sources; this evidence does not claim a permanent global freeze. The original Java after-summary additionally records a separately authorized three-test FileSha256 unit result; those tests and receipts belong to the Hash owner and are **excluded** from Time12 and Pure14 totals.

The independent reviewer `/root/sdk_capability_acceptance_audit` returned PASS with Critical 0, Important 0 and Minor 0 for the five source files, all 12 actual scenes, 84 timestamp observations, raw-copy provenance and environment bindings. This is an agent review, not a machine signature. Archive checksums independently bind the packet bytes.

## Remaining delivery scope

This is focused local evidence for the named State/Trace oracle defect. It does not claim a new Linux CI result, a complete official language gate after all parallel fixture repairs, SDK main acceptance or registry publication. The original cancelled CI and earlier platform-specific complete language evidence remain separate. Final combined source validation, CI, model Finding closure and delivery are owned by the root task.

No public API or example changes are needed for this test-only correction; the existing language test entries already discover these files, including the shared pure tests. Website product behavior is unchanged. No framework, Host, SDK production client, dependency, contract or workflow source changed in this repair.
