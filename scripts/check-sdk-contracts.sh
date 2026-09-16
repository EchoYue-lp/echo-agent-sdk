#!/usr/bin/env bash

# Read-only SDK contract drift check (design §20.2).
#
# Regenerates every contract artifact in memory. Accepted external artifacts,
# extension schema and fixtures are blocking; the complete Rust inventory and
# Host-only catalog are retained as non-blocking telemetry.
#
# Prerequisites (NOT auto-installed by this script; design §16/§20.5):
#   rustup toolchain install <toolchain from contracts/sdk/toolchain.json>
#
# Regenerate after intentional changes with:
#   cargo run -p echo-sdk-protocol --bin export_schema --locked -- --update

set -euo pipefail
cd "$(dirname "$0")/.."

toolchain=$(python3 -c "import json;print(json.load(open('contracts/sdk/toolchain.json'))['rustdoc']['toolchain'])")
if ! rustup which --toolchain "$toolchain" rustdoc >/dev/null 2>&1; then
  echo "error: rustdoc toolchain $toolchain is not installed." >&2
  echo "       install it with: rustup toolchain install $toolchain" >&2
  exit 1
fi

cargo run -q -p echo-sdk-protocol --bin export_schema --locked -- --check
scripts/export-language-sdk-catalog.sh --check

protocol_tree=$(cargo tree -q -p echo-sdk-protocol --no-default-features --locked)
if printf '%s\n' "$protocol_tree" | grep -Eq '(^|[[:space:]])echo_(agent|core|execution|integration|macros|orchestration|state|tools)([[:space:]]|$)'; then
  echo "error: protocol dependency graph contains an echo framework crate" >&2
  exit 1
fi

framework_source=$(cargo metadata --format-version 1 --locked | jq -r '
  .packages[] | select(.name == "echo_agent") | .source // empty
')
expected_framework_source="git+https://github.com/EchoYue-lp/echo-agent.git?rev=27c7701e1eb116db1076da7f84bb68898544a44c#27c7701e1eb116db1076da7f84bb68898544a44c"
[[ "$framework_source" == "$expected_framework_source" ]] || {
  echo "error: echo_agent provenance is not pinned to the SDK extraction revision" >&2
  printf 'expected: %s\nactual: %s\n' "$expected_framework_source" "$framework_source" >&2
  exit 1
}

cargo test -q -p echo-sdk-protocol \
  --test facade_inventory \
  --test acp_baseline \
  --test extension_contract \
  --locked
