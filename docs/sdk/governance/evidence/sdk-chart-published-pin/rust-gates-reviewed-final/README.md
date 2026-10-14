# Final reviewed be5 / shared-61 ordinary SDK Rust matrix

After the parent-dispatched shared four-test-source review passed with zero critical/important/minor findings, the complete original fourteen-command matrix ran on the current exact published framework pin `be5f0a8405b593c4ef964cd6fd2df4beabf4956c`. Every command actually exited zero. All compile/lint/test logs have zero warning/error diagnostics, and all executed tests have zero failed/ignored cases. The official run-command receipts retain execution times, environment, complete output bytes and hashes; failed/intermediate previous source epochs remain in their original evidence directories.

| Gate | Actual final result |
|---|---|
| cargo fmt and fmt check | Both exit 0. |
| Workspace all-targets/all-features Clippy | Warnings denied, exit 0. |
| Production all-features library/bin Clippy | Warnings and unwrap/expect/panic/unreachable denied, exit 0. |
| Workspace all-targets/all-features locked tests | 457 passed: Host lib 171, core E2E 31, bridge E2E 34, facade E2E 12, stdio 4, tool governance 1; Protocol lib 42, exporter 8, ACP 5, core RPC 22, extension contract 39, facade inventory 88. |
| Workspace no-default-features library check | Exit 0. |
| All-features doctests | Exit 0; zero current workspace doctests. |
| Default Protocol tests | 204 passed. |
| Default Host tests | 31 lib and four stdio tests passed. |
| Core-profile Host tests | 85 lib, 13 core E2E and four stdio tests passed. |
| Extension bridge no-run compile | macOS linker flags and warnings-denied flags retained; exit 0. |
| Extension bridge E2E | Warnings denied; 28 passed. |
| Full facade E2E | 31 core and 12 facade tests passed. |
| No-bridge eval/improve/SQLite admission | One passed; this is source admission/adapter reachability, not reverse callback execution without bridge. |

`source-before.json` and `source-after.json` are byte-identical. They cover the selected Rust/language/Cargo/script/CI source inputs; the four exact reviewed test hashes are also independently checked against the frozen CI-regression review manifest before and after the matrix. The current rebuild manifest includes all four test inputs together with every Host/Protocol Rust source and embedded contracts, and verifies those build inputs unchanged. Rust counts repeated across configurations are executions, not additional capabilities. Workspace metadata has no Rust example target; language examples and full native suites are separate parent-owned gates.

Only after all fourteen commands succeeded, fresh locked offline all-features metadata rechecked eight framework packages at the exact published Git revision. Every actual compiler checkout HEAD/tree matches `be5` / `4bff4c2ce436df1606a421f0ac92a905fc5d9b03`, with no local source patch. An actual locked offline no-default Protocol cargo tree exits zero and contains no framework crates, confirming Protocol remains independent.

The same actual `cargo build -p echo-sdk-host --features sdk-facade-all --locked` then exits zero with jobs 2, dev/test debug 0 and incremental 0. Its output SHA-256 is `bcb0389ae8d8a946fab9bb123945100722711029bafa8d9d6cadf95a76c9c345`, exactly matching the already immutable `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/binaries/echo-sdk-be5-chart-dimension-host`. That existing binary is reused without overwriting it; hash equality is an actual observation, not inferred from test-only changes. `source-build-manifest.json` and `final-receipt.json` bind these facts. Cargo/Maven ownership was explicitly released after all producers reached terminal results.

The embedded accepted source-contract digest remains the previous candidate `3cd835dbf40abf1d0aec04e02f3b51411631fcaa50ba7e5f0f2d2d2f500ed9a6`. The twelve accepted identity decisions, full twenty-eight-profile generation, Task B/grace policy and exact SDK main/remote acceptance remain separate. The accepted fifty-group duplicate warning policy requires fresh audit/deny plus exact group comparison and review on the new lock-source epoch; those and the full current shared-61 native pipeline are scheduled by the parent. These fourteen clean Rust gates do not claim all repository policies or foreign physical cleanup are closed.
