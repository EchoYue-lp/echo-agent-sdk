# Actual three-language memory / permission consumer results

Final temporary runners all complete against immutable Host
`/tmp/echo-sdk-governance-parent-driver-host`, SHA-256
`6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5`.
No Cargo, repository source/test writes, generation, SDK dist replacement or
Java target compilation was performed. Java candidate code compiles only into
this directory's `java-classes`. The prior draft README is historical preparation;
this file and final comparison/manifest describe actual completed work.

Each language records59 evidence rows:52 accepted family-operation results,
2 complete typed negative results, native automatic-recall model/journal proof,
actual permission context callback/result, actual permission Tool side effects,
TTL observation and all11 reverse Store callback requests. Rows are evidence
records, not independent test-count claims. Three actual runners exit0; six
helper unit tests and the helper HTTP smoke also pass.

`cross-language-comparison.json` and the executable comparison assert the same
ordered method/results, exact negative codes/retry/method/handle/details, all
original permission context fields/modified args, Store callback routing, actual
file side effects and framework recall journal. No catalog `done` label serves
as consumer evidence.

The native memory resource is a real InMemoryStore in Host. Expiration is seeded
through accepted put_raw with expires_at2; raw get preserves it before explicit
prune, then prune1/0; dedup1/0 preserves one original duplicate. Owner mismatch
is `invalid_value`. Search ties and native hash-map dedup survivor selection have
no deterministic original-key promise: full original data is retained, each
key/content is verified and results compared by the documented equivalence.

Same live Session switches Store A to B through accepted set_store. Host permits
one Store semantic registration at a time; actual conflicting attempts are kept
in historical failure logs. The valid sequence unregisters A, registers B, then
switches the same Session. All later callbacks address B; exact A data remains in
the user fixture backend. The injected pre-write callback fault is
`framework_error`, with no write followed by a successful retry.

Real recall uses a new default native Store Session after B unregister. Active
fixture metadata has exact user evidence and an explicit fixture-owner receipt;
Draft and legacy records remain inspectable but never enter actual model input.
All three journals have one `memory_recalled(count=1)`, unique event IDs,
contiguous sequence and one final_answer. Actual current user input contains the
Active marker once and no Draft/Legacy markers. Original memory content and
provenance remain exact, while actual telemetry changes are retained.

Same AgentDefinition creates separate Sessions A/B for permissions. Rule/reason,
cache exact args versus session_tool, once-never-cached, narrow revoke, and A-close
leaving B approved are actual checks. HumanLoop reverse request retains original
request/session/agent/tool identity, args,1-second requested timeout, workspace,
affected files, impact and special-key metadata. Modified args return unchanged.
A separate real Agent Run selects the registered write Tool. Rejection produces
zero actual callbacks and no file; approved_once produces one callback and exact
UTF-8 file content. Both Runs settle from real framework receipts. This is actual
execution effect evidence, not a facade decision being mistaken for execution.

Clock values, independent temporary roots and opaque per-Host identities are
retained in raw evidence. Comparison normalizes only documented independent
Store timestamps/recall telemetry clocks after canonicalu64 validation, temporary
roots, equal-score result order and content-equivalent dedup survivor keys.
TypeScript seeds the known f64 confidence through existing explicit WireValue,
so integral Number inference cannot turn it into a different generic JSON type.

`scope-disposition.json` explicitly retains the remaining30-minute permission
TTL expiry gap and the new framework approval_receipt field outside current
accepted field inventory. No new wire/API is proposed by these scripts.

Parent next delivery work: copy/review these scenarios and thin language runners
into the SDK-owned executable test chain, choose explicit scope disposition,
then rerun original complete gates on the exact final Host/revision/main.
Temporary consumer success does not prove CI installation or final delivery.
