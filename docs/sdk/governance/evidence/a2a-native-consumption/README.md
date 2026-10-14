# Accepted A2A native consumer evidence

Status: focused three-language consumers passed; independent review pending. This packet is a local development milestone, not full SDK/main acceptance.

The real immutable Host `echo-sdk-a634-composed-host` SHA `193e49b8557db9b746a1cc8c6f5cf30bb6d3154a630562f7ef8551b949e878d1` consumes framework `a6346643bda47679a96760f56f45e41fd73b61b0`. The shared peer implements loopback A2A HTTP JSON-RPC (`tasks/send`, `tasks/get`, `tasks/cancel`) and SSE (`tasks/sendSubscribe`); it never manufactures Host replies, handles, state or stream events. It reuses the task/card/event shapes exercised by `echo-sdk-host/tests/facade_feature_adapters_e2e.rs` and the published framework A2A client/types. No production/client/protocol/generated files changed.

Each native client executes the same 24-step scenario through public `Session.invoke`: all nine accepted A2A operations; discovery; send/get/cancel Task DTOs and remote typed failure; two Unicode artifacts followed by complete; local cancel and repeated close; cross-Session resource/stream rejection; future-generation rejection; and use after the owning Session closes. Each run has exactly eight actual HTTP requests, with the expected order and full Unicode request fields. Invalid calls produce no extra HTTP effect. Task timestamp/history/artifacts and the AgentCard are compared in full; remote error retains `-32001` and its Unicode message.

| Consumer | Result | Original command receipt |
| --- | --- | --- |
| TypeScript | 1 pass, 0 fail, 0 skip | `typescript/run.json` |
| Python | 1 pass, 0 fail, 0 skip | `python/run.json` |
| Java | 1 pass, 0 fail, 0 skip | `java/run.json`, original JUnit XML |

`compare.py` and `comparison.json` compare all 24 result rows, all error fields, typed variant/list/map entries (including order), and all eight HTTP request objects across the three languages. Only the four observed Host handle IDs, peer URL, actual JSON-RPC request UUIDs, and absent optional error fields versus null are normalized. Generation, kind, sequence, business fields, error messages, and effects are retained. Raw results and HTTP captures are unchanged. Reproduce with `python3 docs/sdk/governance/evidence/a2a-native-consumption/compare.py`.

`executed-source/` contains the four owned fixtures/tests plus the runtime client entry sources/dist and current shared catalogs. Capture timing is explicit in `receipt.json`: the Python/Java source entities were captured after their successful runs; the same files remained unchanged and all nine current hashes match. Final TypeScript ran after this capture. The packet does not pretend an unrecorded before-run snapshot existed. Python and Java use their public native WireHandle codecs for handle arguments/results.

The first attempts in `fixture-errors/` are retained as fixture errors: TypeScript initially expected `stale_handle` for a future generation (the actual accepted error is `invalid_value`), then decoded the typed variant with the wrong JSON shape; Python initially tried to serialize a native WireHandle directly; Java initially passed raw JsonNode arguments instead of native strings/public WireHandle. Their nonzero exits are not production counterexamples or waived failures. The same scenarios passed after the fixtures were corrected. A first Python cleanup was delayed by the serialization error; its own peer was stopped and the original runner reached terminal exit 1. No foreign process was stopped.

The official `scripts/check-language-sdks.sh` already discovers these Node/Pytest/JUnit tests and shared Python fixtures; no new CI entry or test-only Host API is needed. Canonical Python checks run from `sdks/python`, retaining that project's Ruff configuration; the complete focused checks are in `focused-checks.json`. A preliminary root-CWD Ruff import check used different first-party classification, then the project-CWD check passed; the comparator's initial formatting mismatch was fixed and the same format check passed. No runtime test source changed for those final checks.

This evidence is deliberately limited: generation checks use a future generation, not a restart; stream cancel/close proves the Host adapter's local settlement, not remote process/finally completion. The held SSE peer waits for fixture shutdown. Later publication/idempotency and stream-failure repairs require their own candidate and final whole-language run. No Rust public-identity parity, 12 deferred-identity retirement, full generator or SDK main acceptance is inferred.

Documentation/examples/website: no public API or product behavior changed. This packet documents actual accepted consumer behavior; existing examples and website need no API update. All three tests are part of the existing executable validation discovery.
