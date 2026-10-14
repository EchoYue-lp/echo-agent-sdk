# Existing readonly Task context consumption proof

Four tests are added only under `#[cfg(test)]` in the original Host `facade/task_runtime.rs`; its entire production prefix is byte-identical to `before-task_runtime.rs`. `focused-source-manifest.json` binds before/current source and actual final focused log. `task-context-test-only.patch` isolates the new test module. No public helper/context constructor, wire DTO, accepted-source disposition or generator artifact was changed. The twelve source identity decisions remain pending; these tests do not approve retiring their old field identities.

The test fixture reuses the actual canonical InMemoryRevisionedTaskStore, TaskRevisionService, FacadeTaskController, SubagentRegistry and SubagentExecutor. Public RuntimeTaskService.execute claims the original graph and constructs its private TaskSubagentContext. A real registered native Agent's `execute_stream_with_invocation_context` is invoked and captures the actual prompt and complete context. Legacy execute/execute_stream paths explicitly error, so bypassing the intended real context consumer cannot pass. During that original Agent invocation the fixture reads the original canonical Store's still-active claim; it does not reconstruct a claim or fabricate execution identity.

| Test | Actual behavior and falsifiable assertions |
|---|---|
| `task_context_run_id_selects_the_original_graph_and_exact_claim_lineage` | Two graph scopes share the same task/dependency IDs but contain different canonical dependency results. Each actual native prompt must contain its own scope's result and exclude the other. Original runtime/lineage run ID, task ID, revision/attempt and execution ID are checked against the canonical Store claim read while that invocation is active. A wrong graph lookup or substituted owner cannot satisfy these observations. |
| `task_context_waived_skipped_dependency_and_completed_output_reach_actual_prompt` | A Skipped dependency has no result, a separate Completed dependency carries its original output, and the real native invocation must contain an explicit waived-dependency notice plus the completed result. Treating the waived dependency as ordinary completed output would reject before dispatch; waiving everything would lose the real completed result. |
| `task_context_nonwaived_completed_dependency_without_result_rejects_before_agent` | A Completed dependency with no reusable output is not waived. The original runtime/controller persists a Failed target with the specific missing-result error and invokes the native Agent zero times. This rejects default-filling a missing canonical result. |
| `task_context_nondefault_delegation_policy_survives_actual_dispatch_binding` | A nondefault runtime configuration `{can_spawn_subagents:true, delegate_depth:1, max_delegate_depth:3}` reaches the real native invocation's runtime context unchanged. The public Host wire still uses its existing default; the composition test exercises the original runtime configuration without adding a wire policy knob. A getter reset or default reconstruction fails this three-field assertion. |

The final focused command (`focused-active-claim.log`) actually passes four tests, zero failures/ignored. Earlier attempts are retained: `focused-first.log` is a test import compile error (there is no echo_agent::core re-export); `focused-retry.log` is the actual Store rejecting an invalid initial revision 7; `focused-valid-initial-revision.log` passes three tests but the first version incorrectly tried to inspect a claim after completion, when canonical settlement had already cleared it. The final fixture records the existing active claim at real native invocation time. These fixture errors are not reported as production bugs or behavioral red proofs.

This is Host/framework composition evidence using actual local runtime objects, not another ACP/OS-process native Host receipt. Existing three-language Task pause/cancel/result/CAS receipts remain separately scoped. The tests verify concrete current consumption through private getters, not a full TaskSubagentContext DTO projection into a language callback. No setter, alternate graph state or mutable legacy field is reintroduced.

The parent-approved ordinary SDK Rust matrix runs separately under `rust-gates/`, with the official `run-command.mjs` capturing actual commands, environment, raw output byte counts, exit codes and log hashes. The initial focused source is frozen before those gates. Contract full-generation/accepted membership remains deliberately independent and pending explicit migration decisions. Remote Linux/Windows CI and exact main delivery are not local composition proof.

## Independent focused review and ordinary matrix status

The parent assigned an independent read-only review of the test-only source and its real original Store/Executor/context observations. The reviewer returned pass with zero critical/important/minor findings. This closes only the four current getter-consumption composition cases; it grants no accepted-source retirement or broad SDK completion.

The initial ordinary matrix has four successful steps: formatting, formatting check, workspace all-targets/all-features Clippy with warnings denied, and production lib/bin all-features Clippy with warnings plus unwrap/expect/panic/unreachable denied. Its workspace all-targets/all-features locked test step exits 101: Host lib 167 passed, core-profile E2E 31 passed, then extension-bridge E2E 32 passed and two failed (`eval_and_improve_source_adapters_invoke_agent_factory_lazily`, `same_factory_constructs_independent_instances_across_concurrent_sessions`). Both failure diagnostics and the complete 25,123 output bytes are preserved in `rust-gates/workspace-tests.log` and its official runner receipt. The remaining nine matrix steps were not executed. The parent has assigned the Factory bridge owner to repair and review those failures; this lane is not authorized to weaken their assertions or skip the full matrix.

All `.rs` files stored under this evidence directory, including `before-task_runtime.rs`, are immutable diagnostic source snapshots outside Cargo targets. They are not new public types, a second implementation or an executable authority. The only executable change is the named cfg-test module in the original production module; its production prefix remains unchanged. A resumed matrix must bind the new actual approved bridge source and retain this failed attempt verbatim.

## Feature wiring repair review cut

After the separate Factory repair, `rust-gates-factory-final/` passed its first eight steps and stopped at the default Host test compile: runtime ownership tests used optional `futures` and `tokio-util` while the core-profile feature was disabled. The minimal repair adds those same version requirements as development dependencies; production dependency features and `Cargo.lock` bytes remain unchanged. The exact previously failing default command then passed 31 Host lib tests and four stdio tests (`default-host-devdeps-repaired.log`).

`rust-gates-final-devdeps/` passed its first nine steps, then the core-profile-only build exposed a `WirePath` import hidden behind the facade feature plus unused feature-specific helpers. `feature-wiring-only.patch` isolates the six-file incremental repair from the saved immediately preceding source bytes; it excludes other task-owned changes already in this working tree. The five Rust files change only imports and `cfg` availability. Factory reservation helpers retain their unit-test availability, MCP publication/cleanup helpers use their actual facade call-site conditions, and tool-result projections remain available to bridge/facade consumers and existing roundtrip tests. The analysis dispatcher uses its eval/improve call-site union. Channel cooperative producer access is gated to channels or its unit tests; the existing Sandbox ephemeral cooperative path, cancellation body and cleanup ownership are unchanged. Managed projections remain available to facade bridge consumers and facade unit roundtrips through `all(sdk-facade-adapters, any(sdk-extension-bridge, test))`. No test assertion or runtime function body was changed.

Actual focused configuration receipts are retained separately:

| Configuration | Receipt and actual result |
|---|---|
| Core profile | `feature-combinations-preflight/ci-core-profile.log`: 85 lib, 13 core E2E and four stdio tests passed; zero warnings. |
| Extension bridge compile with warnings denied | `feature-combinations-repaired/ci-bridge-no-run.log`: exit 0. Earlier denied-unused diagnostics remain in `feature-combinations-preflight/ci-bridge-no-run.log`. |
| Extension bridge E2E with warnings denied | `feature-combinations-repaired/ci-bridge-e2e.log`: 28 passed, zero failures/ignored/warnings. |
| Full facade E2E | `feature-combinations-repaired/ci-facade-e2e.log`: 31 core and 12 facade tests passed, zero failures/ignored/warnings. |
| Eval/improve/SQLite facade without bridge | `no-bridge-final-zero-warning.log`: one admission test passed, zero failures/ignored/warnings. The preceding successful test with an unused managed-projection warning remains in `feature-combinations-repaired/ci-no-bridge-source-admission.log`; it is not a zero-warning gate. |

`feature-wiring-review-manifest.json` binds all six before/current source pairs, frozen current copies, the isolated patch, raw diagnostic/result receipts and unchanged bridge/Task-test/lock anchors. These focused receipts come from successive documented source epochs. The final managed-module attribute was applied after the bridge/facade receipts, and its exact no-bridge command passed afterward. A complete fourteen-step matrix for the final reviewed cut has not started; it follows independent review. Earlier partial matrices are not reported as fourteen successful gates.

This repairs private implementation and test feature availability, so public SDK documentation, examples and website behavior do not change. The matrix compiles/tests existing Rust targets; language examples remain separately validated by the native language gates. No schema, generated catalog, public API, accepted-source membership, framework pin, or pending twelve-identity decision changes in this repair.


## Final reviewed ordinary Rust matrix and immutable Host

The independent six-file feature-wiring review passed with zero critical/important/minor findings. The complete original fourteen-step matrix then ran again on that frozen cut under `rust-gates-reviewed-final/`. Every command exited zero, with zero warning/error diagnostics and zero failed/ignored executed tests. `source-before.json` and `source-after.json` are byte-identical; all six reviewed source hashes still match. Earlier failed and intermediate-warning epochs remain intact.

| Final gate | Actual result |
|---|---|
| Formatting and formatting check | Both exit 0. |
| Workspace all-targets/all-features Clippy | Warnings denied; exit 0. |
| Production lib/bin all-features Clippy | Warnings and unwrap/expect/panic/unreachable denied; exit 0. |
| Workspace all-targets/all-features tests | 457 passed: Host lib 171, core E2E 31, bridge E2E 34, facade E2E 12, stdio 4, tool governance 1; Protocol lib 42, exporter 8, ACP baseline 5, core RPC 22, extension contracts 39, facade inventory 88. |
| Workspace no-default-features library check | Exit 0. |
| All-features doctests | Exit 0; zero doctests exist in the current two workspace crates. |
| Default Protocol tests | 204 passed. |
| Default Host tests | 31 lib and four stdio tests passed. Feature-inapplicable integration binaries contain zero active tests, as recorded in raw output. |
| Core-profile Host tests | 85 lib, 13 core E2E and four stdio tests passed. |
| Extension bridge test compile | macOS linker flags preserved and warnings denied; exit 0. |
| Extension bridge E2E | Warnings denied; 28 passed. |
| Full facade E2E | 31 core and 12 facade tests passed. |
| No-bridge eval/improve/SQLite source admission | One passed; this proves source admission/adapter reachability, not reverse callbacks without the bridge. |

Counts across feature gates repeat some tests and are not additional capability counts. Cargo metadata confirms two libraries, two binaries and nine integration-test targets, with no Rust example targets. The separate native language pipelines own the language examples and full native suites; this matrix does not replace them.

Only after all fourteen commands succeeded, the same `cargo build -p echo-sdk-host --features sdk-facade-all --locked` completed successfully. Fresh offline locked metadata verifies all eight framework packages have their original Git source at `b9099558620f5f9693c736c85c214db14f62a8cb`; each actual compiler checkout reports that same HEAD. The Cargo config has no local source patch. All recorded build inputs are identical before and after compilation.

The copied read-only binary is `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/binaries/echo-sdk-b909-factory-settlement-host`, SHA-256 `bb4f3c2b7f94795394be96179217f3db4b8036af8c9c8cd81c7636d7112f5ecd`. `source-build-manifest.json` binds its actual build receipt, all Host/Protocol Rust source hashes, manifests/config/lock and embedded contract inputs. `final-receipt.json` binds each of the fourteen official raw receipts, execution times, byte counts, test containers and final source cut. Cargo/Maven ownership was released after the build terminal result.

The embedded source contract remains the existing candidate digest `sha256:3cd835dbf40abf1d0aec04e02f3b51411631fcaa50ba7e5f0f2d2d2f500ed9a6`. The twelve accepted-source identity decisions, full twenty-eight-profile regeneration, audit/deny advisory policy, exact SDK main/remote CI delivery, and three-language verification of this new binary remain separate. This receipt does not assert physical cleanup of a foreign producer or stronger Task grace semantics.
