# Shared Task/Subagent true Host scenarios

Existing accepted DTO/API paths:
- sdk.request `_echo_agent/task/create` {task_run, spec: toWireValue(spec)}
- update {task_run, patch:toWireValue({base_revision,reason,operations})}
- list/execute {task_run}; control {task_run,task,action}
- `_echo_agent/subagent/dispatch` {session,request:toWireValue(spec),idempotency_id}
- await {subagent,timeout:{seconds:"0",nanos:5000000}} / normal30s
- control {subagent,action:"guidance",payload:"late"}; no pause/resume on Subagent.
No new operation or host-language scheduler is required. All three clients use
existing public request/registration/stream writers and normal typed handles.

Common registered RevisionedTaskStore fixture handles existing three operations
load/relationCAS/executionCAS only. It compares expected revision/execution vector
atomically and stores exactly framework-supplied next. No DAG validation,
frontier choice, status rewriting, claim generation or retry policy exists here.
The store exposes test-only snapshots/commit traces for evidence of same scope,
claim/revision/result. It owns neither execution nor terminal decisions.

Custom Agent A/B/C/D/await each registers an independent actual extension. Their
stream callback returns the registered call.stream immediately and a retained
language task awaits external HTTP response gate; canceled stream handler records
signal then emits interrupted terminal. No shared-instance exclusive callback
is used for independent A/B branches. Gate only delays user Agent completion.
Custom callback payload has task String only, so it cannot prove TaskSubagentContext
field-level round-trip; canonical claim/context evidence comes from accepted
TaskStore expected_executions and committed next, not fabricated callback fields.

Scenario1 cancel exact A:
Create Session S, parallel A/B, C depends B; all have extension.subagent names.
Start execute, wait fixture A/B stream-enter markers, list A/B Running C Pending.
Duplicate execute must explicit conflict with no second A/B calls.
Control A cancel, await A Cancelled while B Running C Pending. Release B, framework
completes B then C; C actual prompt contains canonical "Completed dependency b"
and B result. Assert A never retried, B/C calls once. Inspect canonical backend
Completed B/C contain result in same execution commit; relation revision stable.

Scenario2 pause/resume exact A:
Same plus D depends A+B. Pause A and prove B/C continue to Completed. Resume A;
wait through typed live-execute conflict until first execution retires (condition
on explicit response, not fixed sleep); start exact same graph. New A completes,
D actual prompt includes persisted B and new A. Assert B/C not redispatched and
backend B result/claim unchanged while A new attempt identity changes. Stale
revision update uses the previous committed relation rev after an actual harmless
rename patch and must reject before mutation; trace/count after rejected operation
unchanged. Parent must match actual framework error codes, not guess.

Scenario3 subagent short-await + idempotent receipt:
Use independent session/registered await-target. Dispatch same payload/idempotency
concurrently twice; same opaque handle and one callback; rich constraints,
prompt_context criteria, Unicode are present in actual compiled prompt. Await5ms
must settledfalse/resultabsent while fixture parked; release, await full result,
then repeat await equals full result exactly. Compare normalized output/status/mode/
truncation/usage and identity associations, preserving variable native duration
in raw transcript but not requiring independently run clock values equal. Late
guidance acceptedfalse, different payload same receipt invalid_value; caller-set
execution_id/foreign Session run_id rejected with no dispatch side effect.

Scenario4 CRUD-only store admission:
Fresh Host/client only register RevisionedTaskStore execution_cas_v1:false. Create
one task and verify Pending in same backend. execute returns feature_unavailable
with operation and task_run handle; no executionCAS operation and zero user Agent
callbacks before failure. Session close settles resources. A register/load/CRUD
call is not called a hidden execution side effect; only actual execution/backend
mutations and Agent invocation counters count.

Compare exact normalized scenarios across TS/Python/Java: statuses/controlaccepts,
revision deltas, call counts/order dependencies, persisted outputs, explicit error
code/op/retryable/handle-kind, same-claim context relationships. Opaque generated
ids remain raw evidence; compare equality/ownership relations, not arbitraryUUID.
Side-effect trace records original full callback/commit DTOs and external gate.

Remaining beyond these four: full context DTO corpus/delegation/isolation fields,
external failures/restart/durable Task Store crash matrices and final published
framework pin. These scenarios cannot silently mark entire family verified.
All candidate files stay in /tmp until coordinator freezes/copies matching inputs.
