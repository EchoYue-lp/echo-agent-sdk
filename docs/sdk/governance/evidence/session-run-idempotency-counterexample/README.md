# Session / Run idempotency counterexample

Four bounded public-request cases use immutable real Host SHA `193e49b8557db9b746a1cc8c6f5cf30bb6d3154a630562f7ef8551b949e878d1`, the existing memory/permission fixture and registered LlmClient provider. `facts.json` preserves every request, response, typed error, real callback input/context, Run settlement and persisted Session index. The counter exits 0 as a diagnostic collector; its observations demonstrate the gap, not passing idempotency behavior.

The successful collector issues four completed Runs on one Session: identical keyed input twice, changed keyed input, then retry after public RunHandle.close. They create distinct turns and four actual provider callbacks. The concurrent case holds the fifth real callback; duplicate keyed start is rejected by the existing active-Turn guard rather than replaying its response. No timeout/retry/schema change or simulated Host is used.

The first-attempt subdirectory preserves a collector error: it mistakenly sent nonexistent `_echo_agent/run/close`, producing Method not found. That is not classified as the idempotency defect. The corrected script uses existing public RunHandle.close, which only checks Host run/get and retires the client's event view. Closing the real Session is separately observed.

This is a byte-exact minimal selection from the original 48-file Admin archive, whose original manifest is retained. The selection does not claim to contain all 48 files. Actual captured state/journals and direct official-reference HTML remain recoverable at `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/session-run-idempotency-counterexample/`. No Git commit is invented for the binary's composed source. The discovery checkout was `76601b242d7c354352f3b5c7d4bf099873746b14`; it is not claimed to contain all tested runtime source.

See ../../session-run-idempotency-proposal.md for the pending product decision. No production implementation or permission to alter accepted contracts is inferred from this diagnostic packet.
