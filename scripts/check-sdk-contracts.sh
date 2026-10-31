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
python3 scripts/check-a2a-lsp-retirement.py

protocol_tree=$(cargo tree -q -p echo-sdk-protocol --no-default-features --locked)
if printf '%s\n' "$protocol_tree" | grep -Eq '(^|[[:space:]])echo_(agent|core|execution|integration|macros|orchestration|state|tools)([[:space:]]|$)'; then
  echo "error: protocol dependency graph contains an echo framework crate" >&2
  exit 1
fi

framework_metadata=$(cargo metadata --format-version 1 --all-features --locked)
framework_source=$(printf '%s\n' "$framework_metadata" | jq -r '
  .packages[] | select(.name == "echo_agent") | .source // empty
')
expected_framework_source="git+https://github.com/EchoYue-lp/echo-agent.git?rev=340993201cc937fc8e4783b66be399ef9f14e3d5#340993201cc937fc8e4783b66be399ef9f14e3d5"
expected_git_packages='[["calamine","0.36.1"],["docx-rs","0.4.22"],["echo_agent","0.2.0"],["echo_core","0.2.0"],["echo_execution","0.2.0"],["echo_integration","0.2.0"],["echo_macros","0.2.0"],["echo_orchestration","0.2.0"],["echo_state","0.2.0"],["echo_tools","0.2.0"],["polars-arrow","0.55.2"],["polars-parquet","0.55.2"],["rust_xlsxwriter","0.82.0"]]'
[[ "$framework_source" == "$expected_framework_source" ]] || {
  echo "error: echo_agent provenance is not pinned to the SDK governance target revision" >&2
  printf 'expected: %s\nactual: %s\n' "$expected_framework_source" "$framework_source" >&2
  exit 1
}
# 八个Framework与五个vendor须同属精HTTPS340；不只检查echo_agent一个包。
printf '%s\n' "$framework_metadata" | jq -e \
  --arg source "$expected_framework_source" --argjson expected "$expected_git_packages" '
  [.packages[] | select((.source // "") | startswith("git+"))] as $git
  | ($git | length) == 13
    and ([$git[] | [.name, .version]] | sort) == ($expected | sort)
    and all($git[]; .source == $source)
' >/dev/null || {
  echo "error: exact 13-package HTTPS framework source set drifted" >&2
  exit 1
}

cargo test -q -p echo-sdk-protocol \
  --test native_message_contract \
  --test facade_inventory \
  --test acp_baseline \
  --test extension_contract \
  --test a2a_lsp_retirement \
  --locked
