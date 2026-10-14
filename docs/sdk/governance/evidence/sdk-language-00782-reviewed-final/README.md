# Complete language gate on framework 00782

The original `bash ./scripts/check-language-sdks.sh` run exited 0. It built and exercised the real Host, ran all three language suites and their executable quickstarts, and preserved an untruncated 109,080-byte log. The Host bytes match immutable SHA `0f1ecf23a2875621caf5128327046cbf6592ce61665c6c6872aed33fdf1931df`, with exact framework revision `00782b8aeddefe2ea8c566922cbdee6aedc80d70`.

| Actual validation | Result |
|---|---|
| TypeScript | 329 passed, 0 failed, 0 skipped |
| Python | 394 passed and 18 subtests passed |
| Java | 351 tests across 75 freshly generated XML reports; 0 failures, errors or skips |
| TypeScript, Python and Java quickstarts | Each executed successfully by the original sequential gate |

`receipt.json`, `run.json`, `runner.log` and `full-language.log` retain the original bytes. `executed-runner.py` is the exact script used to capture the run, SHA `aa0cb18cc479f93fa3af9464c315f30a51c72ad7ee571f8de0e4f01daad6aa66`. The 75 original Java reports and their summary preserve freshness checks and counts rather than reconstructing totals from an older cut. No runtime repository, fixture state, binary, dependency directory or additional source mutation is included.

## Exact source qualification

The run observed Git HEAD `c1213de0e76589070322ad8137b68b7a84435a24` at its start. Its actual validation input was the **793-file working-tree snapshot**, covering runtime code, tests/fixtures, scripts, manifests, lockfile, CI and generated/embedded contracts. `source-before.json` and `source-after.json` are identical, SHA `c42dbb230948b42708922ac50fffd597b7570413268e8a94f676c8f4d2c8398c`; no input was added, removed or changed during the gate. Current source bytes were checked against the same snapshot when this packet was created. Documentation, semantic metadata and generated runtime/build state were outside that runtime-source comparison.

Five input files were not in the starting commit tree. Four were the new Git fixture/consumer tests, subsequently committed at `fe8451d8f6190ad80405a7717efbd9d505cd74ed`. The fifth was the older untracked `compare_channel_forward_results.py` helper, included in the gate's input and Ruff scope. Its bytes are preserved under `untracked-input/`. `source-git-binding.json` records this distinction. The packet does **not** claim that all 793 files existed in the c121 Git tree, or that merely hashing the helper executed its comparison.

The helper was separately reviewed and then actually used to recompare the original three native Channel forward records. The unchanged helper SHA is `0a687973e1e5d5db74fcf01ea2b96afa75bb5b65a69e75f0a8023ec8cbe7293f`; the supplemental original run receipt and 119-byte log record exit 0, exact core sequences, terminal and callback counts, and twelve invalid-input errors. This is an offline comparison of preserved native evidence; it does not add native scenario counts or reuse a prior helper version's result. The delivery stage list includes the unchanged helper source so that the final candidate can contain every frozen input.

## Preserved runtime diagnostics

The full log contains 22 WARN-level error-response tracing lines from deliberately negative E2E RPC scenarios. They remain unfiltered in the original log and receipt. They are runtime domain error responses, with no compilation-warning suppression added. The packet does not misrepresent them as zero runtime logging.

## Delivery boundary

This result covers the complete language entry on the explicitly recorded working-tree bytes. Earlier a634 and partial Git-only packets remain separate historical evidence and do not replace this gate. Framework main CI, Rust/dependency/contract gates, semantic material closure, independent final review and exact SDK main acceptance retain their own receipts and requirements. This archive neither marks SDK main accepted nor publishes npm, PyPI or Maven packages.
