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

python3 scripts/check-a2a-lsp-retirement.py

# canonical来源身份与对象取得传输分别验收；本检查不宣称HTTPS发布或冷恢复。
# 完整数据图的8个runtime、3个媒体包和2个Polars必须共同来自获准的精确修订。
framework_revision="0087353fd78eb73ffaa68b255cf4ef807ca96d4a"
framework_repository="git+https://github.com/EchoYue-lp/echo-agent.git"
expected_framework_source="${framework_repository}?rev=${framework_revision}#${framework_revision}"
framework_metadata=$(cargo metadata --all-features --format-version 1 --locked)
framework_packages=$(printf '%s\n' "$framework_metadata" | jq -er \
  --arg source "$expected_framework_source" --arg repository "$framework_repository" '
  ["echo_agent", "echo_core", "echo_execution", "echo_integration", "echo_macros",
   "echo_orchestration", "echo_state", "echo_tools", "calamine", "docx-rs",
   "rust_xlsxwriter", "polars-arrow", "polars-parquet"] as $expected
  | [.packages[] | select(.name as $name | $expected | index($name))] as $packages
  | [.packages[] | select((.source // "") as $origin
      | $origin == $repository or ($origin | startswith($repository + "?"))
        or ($origin | startswith($repository + "#")))] as $framework_packages
  | if ($framework_packages | map(.name) | sort) != ($expected | sort)
       or ($packages | map(.name) | sort) != ($expected | sort)
       or any($packages[]; .source != $source or (.manifest_path | type) != "string")
    then error("framework provenance requires exactly 13 packages from the approved canonical revision")
    else $packages[] | [.name, .manifest_path] | @tsv end
')
framework_manifest=$(printf '%s\n' "$framework_metadata" | jq -er '.packages[] | select(.name == "echo_agent") | .manifest_path')
framework_checkout=$(GIT_OPTIONAL_LOCKS=0 git -C "$(dirname "$framework_manifest")" rev-parse --show-toplevel)
[[ "$(GIT_OPTIONAL_LOCKS=0 git -C "$framework_checkout" rev-parse HEAD)" == "$framework_revision" ]] || {
  echo "error: framework checkout HEAD differs from the approved revision" >&2
  exit 1
}
# Cargo正常生成的未跟踪.cargo-ok不是源码；已跟踪源码必须仍匹配该精确commit。
GIT_OPTIONAL_LOCKS=0 git -C "$framework_checkout" diff --quiet HEAD -- || {
  echo "error: framework checkout tracked source differs from its exact commit" >&2
  exit 1
}
while IFS=$'\t' read -r package manifest; do
  [[ -f "$manifest" && "$(GIT_OPTIONAL_LOCKS=0 git -C "$(dirname "$manifest")" rev-parse --show-toplevel)" == "$framework_checkout" ]] || {
    printf 'error: %s manifest is not in the approved framework checkout\n' "$package" >&2
    exit 1
  }
done <<< "$framework_packages"


# 来源预检通过后才进入会编译并调用rustdoc的唯一合同入口。
cargo run -q -p echo-sdk-protocol --bin export_schema --locked -- --check
scripts/export-language-sdk-catalog.sh --check

protocol_tree=$(cargo tree -q -p echo-sdk-protocol --no-default-features --locked)
if printf '%s\n' "$protocol_tree" | grep -Eq '(^|[[:space:]])echo_(agent|core|execution|integration|macros|orchestration|state|tools)([[:space:]]|$)'; then
  echo "error: protocol dependency graph contains an echo framework crate" >&2
  exit 1
fi


cargo test -q -p echo-sdk-protocol \
  --test facade_inventory \
  --test acp_baseline \
  --test extension_contract \
  --test a2a_lsp_retirement \
  --locked
