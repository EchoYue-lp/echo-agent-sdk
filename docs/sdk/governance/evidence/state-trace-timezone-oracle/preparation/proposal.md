# State/Trace timestamp oracle — bounded implementation proposal

Status: implementation proposal only. Source SDK and Host have not been modified or executed by this lane.

The current framework owns `DateTime<Utc>` and the `local_rfc3339`/`option_local_rfc3339` serializer. `AgentCheckpoint.timestamp`, Trace Run `started_at`/`finished_at`, and RunSummary fields preserve the instant and nine-digit fractional precision; serialization uses the Host local offset. `None` serializes as null. Host state_delivery parses the original DTO and serializes the original authority. No API promises retention of the input's `+08:00` spelling. WireTimestamp seconds/nanos is a different accepted type and is not changed here.

The existing cancelled CI raw file is 166,086 bytes, SHA `d4a85d86d1af20f0025d157e66dceed3b4893059c2ae67da2370f254f4dad2ca`. Its two observed instant pairs are precisely `(1791867600,123456789)` and `(1791867601,987654321)`. The input +08 and output +00 strings differ while these pairs agree. The offline observation JSON preserves these raw values and hashes; it does not claim a new Host test or a successful CI terminal.

## Smallest shared comparison

Extend the existing shared state_trace_delivery_fixture.py with test-only pure helpers and a `compare-timestamps` CLI action. Native clients pass their already integer-canonicalized actual and expected documents through stdin JSON, specifying **only top-level timestamp field names**. The helper permits exactly the checkpoint `timestamp` or Trace/Summary `started_at`, `finished_at` set; it rejects another field set. This avoids broad recursive replacement of fields inside tool args/events, unknown nested DTOs, or Delivery claimed_at.

Parse these strings using an anchored RFC3339 pattern and integer arithmetic: validate whole-second date/time using Python datetime with no fractional argument, convert those integer seconds via calendar.timegm, apply the signed minute offset, and separately right-pad the captured 0–9 fractional digits to exactly nanoseconds. Never use datetime's fractional precision, datetime.timestamp floating point, JS Date milliseconds, float parsing, rounding or substring truncation. Return an exact decimal epoch-second string and integer nanos. Malformed dates, offset hour/minute or fractions over nine digits fail rather than defaulting. Null remains null; missing versus null and unknown fields remain distinguishable.

Only the explicitly named top-level values are replaced in **copies** for comparison. All other values, keys (including unknown keys), optional presence/null, integer decimal text, object field sets and array order are compared completely. Canonical JSON equality distinguishes bools from numbers. Original inputs, Host DTOs, disk JSONL bytes and error records remain unmodified in proof artifacts. Add raw actual/expected time values and exact pairs to observations so equivalent-time comparison does not conceal output formatting.

Three consumers reuse this one helper. TypeScript and Java invoke the existing fixture process with JSON stdin; Python imports the same pure helper. Native helpers keep existing integer canonicalization. Checkpoint comparisons explicitly name timestamp. Trace load, JSONL disk, saved/load-after-fault and cold restart explicitly name started_at/finished_at. Summary comparison upgrades the current selected-field loop to one full expected Summary document assembled from the fixture's already redacted short input, typed IDs, token usage, duration and exact timestamp fields; extra/missing summary keys then fail. Every unchanged failure/cleanup/ownership and byte-preservation assertion remains.

## Validation after path/lease approval

Add one meaningful pure helper test file to cover equivalent +08/+00/Z, one-nanosecond mismatch (especially last three digits), second mismatch, invalid offset/fraction/date, null versus missing, unknown key/nested timestamp changes and array reorder. These test the accepted comparison boundary rather than mirroring implementation steps.

Run the existing State and Trace scenes in TypeScript, Python and Java under genuine `TZ=UTC` and `TZ=Asia/Shanghai`: 2 scenes × 3 languages × 2 zones = 12 actual tests. Each already includes save/get, actual disk observation, injected failure preservation, cold Host restart and Trace summary. Assert raw serialized offset is exactly +00:00 in UTC and +08:00 in Shanghai; the locked Chrono serializer does not emit Z, while the pure input parser still accepts valid Z; this is separate from exact instant/nanos comparison. Preserve the original +08 payload in both zones. Require nine fractional digits in these deliberately nine-digit fixtures and exact pairs above. Do not set a global CI TZ, alter production serialization or weaken error checks.

Cross-zone cold restore is optional: the same saved documents can be reopened by a fresh Host with the opposite TZ via explicit per-spawn env, proving local output changes while exact instants remain. It adds no durability promise, but can be omitted if the existing 12 complete scenes already provide sufficient evidence. Root chooses this small additional consumer scope before implementation.

## Proposed source paths (await Root scope confirmation)

- sdks/shared/testing/state_trace_delivery_fixture.py
- sdks/shared/testing/test_state_trace_delivery_timestamps.py (new pure helper regressions)
- sdks/typescript/test/state-trace-delivery-host.test.js
- sdks/python/tests/test_state_trace_delivery_host.py
- sdks/java/src/test/java/com/echoagent/sdk/StateTraceDeliveryHostTest.java
- docs/sdk/governance/evidence/state-trace-timezone-oracle/ (bounded original/new raw evidence and rationale)

No Host, framework, SDK public client, dependency, contracts, CI workflow, telemetry, binary-hash lane, semantic model or foreign workspace path is modified. Every actual producer waits for Root's preflight/path confirmation and unique Maven/native lease. The original CI log and old source hashes stay immutable.
