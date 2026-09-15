# Shared SDK contract inputs

`facade-operation-catalog.json` and `contract-digests.json` are generated from
the accepted external Rust contracts. Do not edit them by hand. Regenerate
with `../scripts/export-language-sdk-catalog.sh` after an intentional contract
change; the contract and language gates fail on accepted-artifact drift.

Caller-provided runtime requirements are recorded in
[`toolchain.json`](toolchain.json): Node.js 20+, Python 3.10+, and JDK 17.
These are compatibility requirements only; the repository does not install or
bundle any runtime.
