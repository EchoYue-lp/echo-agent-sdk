# Complete reviewed be5 language SDK gate

The exact original command `bash ./scripts/check-language-sdks.sh` was executed without filtering, skipping, changing test assertions, increasing timeouts or substituting focused receipts. It reached terminal exit 0 and printed `language SDK source checks passed`. The full raw 91,026-byte output, official run-command receipt and 67 actual Java test XML files are preserved. The original earlier 94,950-byte failed attempt remains untouched in the separate first-attempt packet.

| Original pipeline stage | Actual result |
|---|---|
| Language catalog check and accepted external mappings | Original script proceeds successfully. Scope counts remain nonblocking telemetry. |
| Source-built all-facade Host | Actual build completes; SHA bcb0389ae8d8a946fab9bb123945100722711029bafa8d9d6cadf95a76c9c345 matches the prior immutable Host. |
| TypeScript npm ci/test, SDK build and example build | 290 tests passed, zero failures/cancelled/skipped. |
| TypeScript quickstart | Original `echo-agent` output, command completes. |
| Python Ruff check and format check | All checks passed; 176 files already formatted. |
| Complete Python tests and shared oracle tests | 368 passed and 18 subtests passed; zero failures/skips. |
| Python quickstart | Original `echo-agent` output, command completes. |
| Complete Java tests | 320 passed across 67 XML suites; zero failures/errors/skips. |
| Java dependency classpath and quickstart | Original `echo-agent Java SDK connected: agent-2`, command completes. |

All 759 selected actual runtime source, tests/fixtures (including untracked additions), scripts, CI files, manifests, lock and embedded/generated contract inputs have identical hashes before/after the complete script. `source-scope.json` describes the filter: build outputs/node_modules/venv/caches and separately owned docs/.echo-semantic metadata are excluded. The source-before/after manifest SHA is the same `9263d5e8d6bcf8bd56a905bb0e0e4dc860baeeade99993c26e2a415f2984654e`. Actual compiler inputs are real Git framework be5 with no local source patch; all eight package sources and checkout HEADs were verified before the run.

`rust14-to-language-cut-comparison.json` independently checks every one of the original 655 Rust-matrix source paths against the actual current bytes, not just the 530 overlapping paths in the broader language freeze. Exactly two paths changed after those fourteen Rust gates: Java HostEarlyExitTest atomically publishes its original child PID, and the existing shared tool_value_effect_fixture oracle corrects literal escaped whitespace validation. Both are separately reviewed test-only fixes. There are no missing old paths. All 59 recorded reviewed Host rebuild inputs remain byte-identical, including all Host/Protocol Rust production sources, framework pin/lock/config, embedded contracts, build script and the four persistent shared-61 regression test sources. TypeScript JS was not part of the earlier 655 extension filter but is in that reviewed rebuild-input manifest and in the new broader language freeze. No runtime or API change is inferred from either test correction.

`rust14-build-chain/` copies the original final fourteen-gate summary, before/after source hashes, actual rebuild manifest/inputs/receipt/log and Protocol framework-free tree proof. These are the original bytes, with their historical paths/timestamps retained. The complete script's actual newly built Host hash equals that immutable bcb Host. No immutable binary is overwritten and no runtime state tree is copied into this packet.

The raw log contains 22 actual `agent_client_protocol::jsonrpc::outgoing_actor: Sending error response` runtime WARN notices from deliberate rejected-input/lifecycle cases. `warning-classification.json` retains every complete line and counts routes: extension/unregister 1, data 2, chart 1, git 2, media 1, research 4, workflow 11. Compiler, Ruff and SLF4J provider warning counts are zero. This is not a claim that the raw output contains zero WARN text, and nothing was filtered or suppressed.

Each language's actual full-run computation results are archived separately: 61 calls, 25 typed errors, 11 common oracle groups, five final-pivot boundary checks and eight protected files. The existing shared comparator verifies the three full-run result packets, retaining complete errors and wire arguments while only applying its original work-root/map/unordered-row normalization. The nine real early-exit Java tests and three ToolValue native tests also pass as part of the original complete 320-case run, not only earlier focused runs.

`final-summary.json` binds counts, source hashes, exact Host/pin, warning classification and actual raw results. Cargo/Maven ownership was explicitly released after all script and capture subprocesses terminated. Java runtime temporary paths remain the actual OS/default fixture paths where existing tests choose them; raw paths are not rewritten. The persistent source/runtime package lives inside the requested lp-agent workspace.

This closes this current original complete language command. It does not close the twelve accepted-source identity decisions, complete twenty-eight-profile generation, Task B/grace policy, fresh new-lock audit/deny exact duplicate-group review, strict global semantic acceptance or SDK main/remote delivery. No generated contract, accepted source membership or production code is changed by this evidence archive. Public docs/examples/website need no new feature description; all three existing examples are actually executed by this gate.
