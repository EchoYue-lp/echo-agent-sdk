All files remain temporary and have executed against a real immutable Host:
`/tmp/echo-sdk-governance-parent-driver-host`, SHA256
`6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5`.

Four equivalent scenarios run in TypeScript, Python and Java: exact pause/resume
with canonical sibling results; exact cancellation with independent sibling and
pending dependent continuation; duplicate/idempotent Subagent dispatch with short
await followed by a repeatable full result and rich Unicode constraint; CRUD-only
registered Task Store rejects execution before execution-CAS or user Agent calls.
DAG cases also check relation-CAS stale writers and duplicate execute rejection.

The shared HTTP backend only compares expected revisions/executions atomically
and stores the full next graph supplied by the framework. It chooses no ready
frontier, creates no claims, assigns no statuses and owns no scheduling policy.
Agent response gates control actual user callback completion only. Each A/B/C/D
Agent extension is independent to avoid exclusive-proxy sharing accidentally
serializing a concurrency test. Persisted CAS trace proves canonical claim/revision
relationships and atomic completed result storage; it does not claim direct full
TaskSubagentContext field projection into CustomAgent callbacks, which accept a
task string only. All call/commit raw traces retain opaque generated identities.

`scenario-plan.md` is the accepted-entry/source-evidence design. Temporary Node
imports built SDK dist by an explicit absolute path; Java uses public SDK methods
from existing target/classes, javac outputs only under this temporary directory;
Python uses the existing project's runtime without changing SDK files.

Actual proof: `/tmp/sdk-task-subagent-tmp-three-language-proof.json`; source/log
hashes, isolated state roots and exact normalized results are retained there.
The first Node await draft rejected actual stream invocation (fixture error),
then was corrected to support both existing AgentExecute/AgentExecuteStream as
the original Rust fixture does; the raw first failure log is retained.

These files are candidates for repository-owned executable gates after coordinator
review/copy. They do not prove final published pin/main, full context DTO corpus,
Task Store crash/restart or all accepted failure combinations. No SDK production,
generated scope or framework source changed in this lane.
