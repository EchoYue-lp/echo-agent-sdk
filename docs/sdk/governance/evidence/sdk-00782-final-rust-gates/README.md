# Published 00782 SDK Rust14 gate evidence

This packet preserves one complete execution of the original 14-command SDK Rust gate sequence against framework `00782b8aeddefe2ea8c566922cbdee6aedc80d70`. All 14 commands exited 0; raw logs are untruncated. `exact-original-commands.json` records the argv and per-step environment overrides; `execute-14.py` is the actual execution driver, archived as experimental provenance rather than a new production entry point.

The execution started from SDK Git `c1213de0e76589070322ad8137b68b7a84435a24`. All 61 Rust/Cargo/configuration input hashes matched before and after every gate, and after the complete sequence. Archive verification also matched those 61 files against the execution Git blobs and the current working tree. Cargo.lock SHA256 is `88d99f3880f75315f60c8eca48035eb6151a1a055aeda7f621e689b354d411b3`; the private thin source adapter is `2df1bbe8ff3221c690d016379030dee6f76afa8f1893f8ccadb1791d14769649`. Later evidence commits do not change this recorded execution cut.

| Gate | Actual tests passed / failed / ignored |
| --- | --- |
| 05: workspace, all targets and features | 474 / 0 / 0 |
| 07: workspace doctests | 0 / 0 / 0 |
| 08: default protocol | 204 / 0 / 0 |
| 09: default Host | 35 / 0 / 0 |
| 10: core profile | 111 / 0 / 0 |
| 12: extension bridge E2E | 28 / 0 / 0 |
| 13: facade/core E2E | 32 + 12 / 0 / 0 |
| 14: no-bridge source admission | 1 / 0 / 0 |

These feature profiles overlap. The primary workspace count is 474, and the table must not be summed into a unique-test total. Gates 01–04, 06 and 11 perform formatting, linting, checking or compilation. Formatter execution and check both exited 0 without changing the 61 inputs.

The shared target, two build jobs, zero dev/test debug information and disabled incremental compilation are recorded in `matrix.json`. Only steps 11 and 12 explicitly set `RUSTFLAGS=-A linker_messages -D warnings`; other steps use the original repository configuration. Compiler warning/error diagnostics are 0. Step 05 retains ten expected runtime RPC WARN records from negative error-response cases; they are preserved in the original log and are not compiler warnings.

Before Cargo started, an initial generic source scan omitted `rust-toolchain.toml` and found 60 inputs; its assertion stopped preparation. The driver was then prepared using the exact original 61-path basis. A subsequent attempt to invoke the not-yet-created driver also stopped before Cargo. These preparation errors are recorded session facts, not Rust gate failures, and were not converted into fabricated command receipts. `start-source.json` retains the first correction. A later archive preflight rejected an outdated HEAD basis; no formal files were written until the existing goal record was refreshed and the independent path check exited 0.

The 35 original members are byte copies from the Admin execution directory. `combined-receipt.json` binds their original paths, hashes and lengths; `SHA256SUMS` covers every formal member except itself. Duplicate wrapper stdout files remain in Admin and are not required to reconstruct the recorded command outcomes. Raw absolute paths and timestamps are retained. No source, configuration, contract seed or generated JSON was changed while archiving.

This is Rust-only evidence for the recorded published-pin source cut. Dependency-policy verification, native Git behavior, the ongoing complete language run, full accepted-contract generation, the twelve source-continuity decisions and final main delivery have their own evidence and completion criteria. This packet does not claim those are complete. The existing adapter documentation, language examples and website require no change for this archival-only slice. Independent archive review follows this seal; earlier focused pin and adapter reviews are separate.
