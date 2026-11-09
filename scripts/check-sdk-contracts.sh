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

# 先验证实际来源再编译/rustdoc：source 字符串不能证明缓存 checkout 的真实字节。
# 固定已发布main3413与13个精确name/version，移植main的HEAD/clean/manifest同根检查。
framework_revision="3413d64dae23353c6d02c546ac374deded855827"
framework_metadata=$(cargo metadata --format-version 1 --all-features --locked)
expected_framework_source="git+https://github.com/EchoYue-lp/echo-agent.git?rev=${framework_revision}#${framework_revision}"
expected_git_packages='[["calamine","0.38.0"],["docx-rs","0.4.22"],["echo_agent","0.2.0"],["echo_core","0.2.0"],["echo_execution","0.2.0"],["echo_integration","0.2.0"],["echo_macros","0.2.0"],["echo_orchestration","0.2.0"],["echo_state","0.2.0"],["echo_tools","0.2.0"],["polars-arrow","0.55.2"],["polars-parquet","0.55.2"],["rust_xlsxwriter","0.82.0"]]'
# 八个Framework与五个vendor须同属精HTTPS已发布main3413；不只检查echo_agent一个包。
framework_packages=$(printf '%s\n' "$framework_metadata" | jq -er \
  --arg source "$expected_framework_source" --argjson expected "$expected_git_packages" '
  [.packages[] | select((.source // "") | startswith("git+"))] as $git
  | [.packages[] | select(.name as $name | $expected | map(.[0]) | index($name))] as $packages
  | if ($git | length) != 13
       or ([$git[] | [.name, .version]] | sort) != ($expected | sort)
       or ([$packages[] | [.name, .version]] | sort) != ($expected | sort)
       or any($packages[]; .source != $source or (.manifest_path | type) != "string")
    then error("exact 13-package HTTPS framework source set drifted")
    else $packages[] | [.name, .manifest_path] | @tsv end
') || {
  echo "error: exact 13-package HTTPS framework source set drifted" >&2
  exit 1
}
framework_manifest=$(printf '%s\n' "$framework_metadata" | jq -er '.packages[] | select(.name == "echo_agent") | .manifest_path')
framework_checkout=$(GIT_OPTIONAL_LOCKS=0 git -C "$(dirname "$framework_manifest")" rev-parse --show-toplevel)
[[ "$(GIT_OPTIONAL_LOCKS=0 git -C "$framework_checkout" rev-parse HEAD)" == "$framework_revision" ]] || {
  echo "error: framework checkout HEAD differs from the approved revision" >&2
  exit 1
}
# Cargo生成的未跟踪.cargo-ok不是源码；检查包含已暂存和未暂存的跟踪文件变化。
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

# 来源预检通过后才进入唯一生成链；不能靠生成后的结果替代源码准入。
cargo run -q -p echo-sdk-protocol --bin export_schema --locked -- --check
scripts/export-language-sdk-catalog.sh --check

protocol_tree=$(cargo tree -q -p echo-sdk-protocol --no-default-features --locked)
if printf '%s\n' "$protocol_tree" | grep -Eq '(^|[[:space:]])echo_(agent|core|execution|integration|macros|orchestration|state|tools)([[:space:]]|$)'; then
  echo "error: protocol dependency graph contains an echo framework crate" >&2
  exit 1
fi

cargo test -q -p echo-sdk-protocol \
  --test native_message_contract \
  --test facade_inventory \
  --test acp_baseline \
  --test extension_contract \
  --test a2a_lsp_retirement \
  --locked
