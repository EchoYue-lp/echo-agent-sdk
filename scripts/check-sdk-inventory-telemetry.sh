#!/usr/bin/env bash
set -euo pipefail

# Recompute the complete Rust public inventory. Ordinary telemetry drift is
# reported as a non-blocking signal; generator or framework-provenance failure
# still returns non-zero. This is separate from the blocking external-contract
# gate.
cd "$(dirname "${BASH_SOURCE[0]}")/.."

cargo run -q -p echo-sdk-protocol --bin export_schema --locked -- --telemetry-check
