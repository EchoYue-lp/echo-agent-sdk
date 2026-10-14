# Fourth complete language gate after CI fixture repairs

The original `bash scripts/check-language-sdks.sh` run completed with **exit 0 and finalValidationExit 0**. The untrimmed 105,004-byte log has SHA `7a296605c5dd519840d20afe5e2a32016a43e8e7b10bbe2d505cdf5adcf4257b`. Its real Host is 706,935,504 bytes, SHA `0f1ecf23a2875621caf5128327046cbf6592ce61665c6c6872aed33fdf1931df`. `run.py`, SHA `67e6be01dbe43d66bf63f8c86dedc332359044299be1050079a23d24d2307bcd`, is copied exactly from the actual execution.

| Actual result | Count |
|---|---|
| TypeScript | 336 passed, 0 failed or skipped; quickstart succeeded |
| Python SDK | 415 passed plus 18 subtests; quickstart succeeded |
| Java | 355 tests, 0 failures/errors/skips across 76 fresh XML suites; example succeeded |

All 76 original XML reports match their saved SHA and were written inside the recorded run interval. They are actual fourth-run reports, unlike the stale report-directory totals in earlier failed attempts. `java-xml-freshness.json` preserves original paths and nanosecond mtimes. Full original receipt, log, runner, versions and 800-file before/after maps are byte copies; no runtime fixture tree or build output is included.

## Runtime and source cut

This was a local run using Node **20.20.2**, npm **10.9.4**, SDK Python **3.10.19**, Oracle JDK **21.0.5**, Maven **3.9.0** and `TZ=UTC`. The Python version probe actually ran from the SDK's `sdks/python` directory, as recorded. Local Oracle21 is not Linux CI Temurin17, and this result does not invent a remote CI terminal.

Execution started and finished at Git HEAD `9dd50a08a3f33430ea7d9a1b11df3403e9e6abde`. The 800 runtime-input bytes were unchanged throughout. At that point, 799 matched the committed tree; the independently reviewed production Python client fix was present as an uncommitted input. Its exact SHA is recorded in the original receipt/before map. That file was subsequently committed at `b0e43b92d9be74aff7835b28692acd4f523ccae4`. `exact-git-source-binding.json` verifies **every one of the 800 current file blobs and modes** against b0e43 and the actual run input, without rewriting the run's 9dd start revision. Thus this packet preserves working-tree execution truth and later Git recoverability separately.

The language 800 inputs overlap **60** of the original Rust gate's **61** inputs. `rust-toolchain.toml` is not in the language runner's input map and was separately checked against the original Rust manifest. All 61 current bytes still match that Rust epoch. No Rust matrix was rerun here, and the packet does not claim that the language gate exercised all 61 Rust inputs.

## Preserved failure history

The first failed official attempt stopped in TypeScript on the system Python launcher annotation error; the second stopped on the paused-consumer event-window assertion; the third passed TypeScript but failed Python 3.10 factory-timeout classification. Their original full logs and receipts remain in their already committed individual packages. `history-link-index.json` binds those exact paths and SHA values. Only the three small original derived execution-boundary summaries are copied into `history/`; no prior 100-KiB log is duplicated or relabeled as successful.

The final log contains 22 WARN-level tracing lines for expected negative E2E RPC error responses. `warning-classification.json` retains every original line. These are domain-failure scenarios, not compiler/lint/SLF4J diagnostics; no warning output was suppressed. The original log remains complete and unmodified.

## Delivery boundary

The independent original-evidence reviewer `/root/sdk_capability_acceptance_audit` returned PASS with Critical 0, Important 0 and Minor 0 for this finite complete-language result and exact source qualification. Formal archive material hashes are reviewed separately. This is an agent conclusion, not an invented machine signature.

This packet does not assert remote Linux CI, final SDK main acceptance, closure of every capability, or npm/PyPI/Maven publication. Host/framework, Rust/dependency/contract gates, semantic materials, applicable remote CI and exact merged-main verification keep their separate obligations. No source, CI workflow, semantic object or validation command was changed while creating the archive.
