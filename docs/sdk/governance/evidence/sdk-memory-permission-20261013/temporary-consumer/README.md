> This file preserves the initial draft preparation scope. For completed real
> three-language execution and its exact limits, read [ACTUAL-RESULTS.md](ACTUAL-RESULTS.md)
> and manifest.json. The initial "no Host launched" statement below applies only
> to that earlier preparation checkpoint, not the subsequent actual consumer run.

# SDK memory / permission consumer candidate

Draft only. All files are staged under `/tmp/sdk-memory-permission-candidate-fixture`.
Repository runtime, test sources, catalog, generated contracts and Cargo inputs are
unchanged. No Host was launched and no Cargo command was run in this lane.

`prepare_scenarios.py` verifies every planned invocation against the current
accepted catalog and writes `scenarios.json`: 4 scenarios / 36 invocation steps.
Catalog SHA-256: ecca20dc7eb2921b76bb9db23c45fbc4dab27eb510f2d80f4fdd6d76ab321cc0.
The individual steps contain exact operations, receiver, positional arguments,
assertions, independent consumer observations and limits. `$...` values denote
real handles/configuration supplied by each language harness, never rewritten IDs.

## Shared helper and language boundaries

`fixture.py --work <isolated-directory>` binds only loopback and writes host.json,
backend.json and ready. It provides:

- `/store`: test-owned A/B native Store operations; side effects and invocation
  inputs are inspectable. Fault injection before mutation is explicit. This is
  an actual user extension implementation, not framework scheduling or recall.
- `/v1/chat/completions`: a real model endpoint returning a terminal model answer
  and capturing actual input messages. Recall assertions must read those messages,
  not the fixture's constant response or an invented memory_recalled event.
- `/inspect`: Store calls and received model requests. Counters are observation
  evidence only; they are never Host handles, Run status or permission decisions.

The backend receives native JSON, after the existing language SDK decodes
StoreCall.value. `store_adapter.py` sketches the Python callback's two conversion
boundaries: from_wire once for store_put input; to_wire once for StoreItem.value
output, with u64 timestamps retaining canonical decimal text. Java should use the
existing StoreCall/StoreItem/StoreOutcome constructors, not untyped Jackson
numeric coercion. TypeScript should use existing StoreInvocation/StoreItem and
WireValue conversion. The family resource search input uses existing Rust SearchMode JSON `"Keyword"`;
the external Store callback carries protocol `{"kind":"keyword"}` instead.
These are distinct adapter boundaries; do not send the former as reverse wire.
HTTP helpers are transport only; neither the backend nor
these adapters evaluates framework permission policy or decides memory recall.

All three languages should run the same scenarios, retaining complete observed
result/error values and actual side effects in separate receipts. Normalize only
map association ordering and opaque per-Host IDs for comparison; never discard
error codes, handles, generations, timestamps, sequence or absent/null fields.

## Scenario / authority / evidence mapping

| Scenario | Existing authority and entry | Assertions that actually prove behavior | Draft status |
| --- | --- | --- | --- |
| Native resource expiry and dedup | accepted InMemoryStore::new / put_raw plus memory.resource.*; framework Store owns operations | seed expiry in accepted StoreItem, raw get retains expires_at, prune1/then0, duplicate content removal1/then0, list/search exact surviving facts; foreign Session handle rejected while owner still works | 11 planned steps; Host not run |
| Reverse Store A to B | accepted ReactAgent::set_store and memory.store.put/get/search/delete; Agent's live Store is authority | real reverse call IDs point to A, after switch every new CRUD uses B; A original value persists, B missing before its own write; failure-before-write has no side effect and retry reaches same B | 8 steps; helper3 unit cases pass, Host not run |
| Real automatic recall | enable_memory default native FileStore, accepted generic Store put/get plus accepted Run | actual model input current user memory context contains approved Active marker once; Draft/Legacy never enter; real journal memory_recalled and Store metadata align; original content/provenance remain unchanged | 4 seeding steps and real Run action; Host not run |
| Permission policy/cache/context/isolation | accepted permission.* live Session PermissionService; same AgentDefinition constructs distinct Session Agents | explicit rule/source/reason round trip, approvals bound to exact scope/tool/input, session_tool semantic, Once never cached, revoke/clear narrow effect, same scope does not cross Sessions, actual HumanLoop reverse request retains IDs/args/approval_context and modified args result | 13 steps plus context/effect actions; Host not run |

The same AgentDefinition yields an independent ReactAgent per Session in
factory.rs:102 and create_session. Two Sessions must be created from the same
Agent handle for the isolation counterexample; using two independent Host
processes would prove a narrower boundary. SessionA close must leave SessionB
permission state and feed available.

## Evidence gaps that must remain explicit

1. **Short permission TTL is not accepted.** Catalog exposes stats/cleanup_expired,
   record/revoke/clear, but no PermissionServiceConfig.cache_ttl,
   SessionApprovalCache::with_ttl, clock advance or session permission replacement.
   Framework default service TTL is30minutes. Validate the actual stats TTL and
   young-entry cleanup0; this does not prove expiration. To prove true SDK TTL,
   use a named opt-in real wall-clock test: observe stats TTL, record an approval,
   poll is_approved across that actual duration, then cleanup_expired count.
   Do not add a test-only public API or mutate Host files/private clocks.
2. **Raw set_store is not set_memory_store.** The latter is not accepted here.
   set_store replacement proves raw CRUD authority. It does not install a memory
   promoter, LayerManager or replace registered recall tools. Real recall should
   use a fresh Session's default enable_memory Store. Generic put of a typed
   reviewed fixture value is supported JSON data, not a claimed activation API.
3. **No semantic-search proof.** Native resource scenario uses Keyword. The
   external Store fixture must declare only modes it implements; an undeclared
   Semantic/Hybrid request should fail before an external callback and is a
   separate capability-negative case, not a zero-results success.
4. **Permission check does not execute effects.** A deny result plus no file from
   a direct check is not enforcement proof. Full effect closure needs a real Run
   selecting an existing registered candidate_write Tool, a controlled real
   HumanLoop answer, and actual callback/file observation. Before publication,
   add model tool-call mode or reuse existing callback Host fixture; no fake
   successful ToolResult may substitute. The current helper only returns final
   model answers, so that effect action is explicitly not implemented yet.
5. **Context and result contract detail.** Register accepted HumanLoopProvider
   before constructing Sessions; Host injects it in extension_bridge.rs:7801.
   Its actual approval_context/request/session/agent fields can be checked through
   the reverse callback. check_value currently projects decision and updated_input
   but omits framework PermissionCheck.approval_receipt. Investigate whether this
   accepted contract requires receipt preservation before claiming no loss;
   do not silently invent a new field in candidate scenarios.
6. Backend-unit prune/dedup tests prove fixture behavior only. Resource tests must
   invoke the actual native framework Store through Host to establish SDK proof.
   Whole three-language gates and exact revision/main acceptance remain pending.

## Local checks actually run

- `python3 -m unittest -v test_fixture.py test_adapter.py`:6 pass; no Host.
- `python3 smoke_http.py`:candidate HTTP Store put/get and model-input recording
  pass; no Host and no substitute Run behavior.
- `python3 prepare_scenarios.py`:36 steps accepted; no generated contract changes.

Parent next action: publish the reviewed fixture/scenario subset in SDK-owned
shared testing and implement the three thin language harnesses; add real model
Tool-call action, then run each scenario against one immutable matching Host.
Finish each report with explicit observed/failed/missing assertions. Final original
full gates must include these scenarios and retain no skipped required Host tests.
