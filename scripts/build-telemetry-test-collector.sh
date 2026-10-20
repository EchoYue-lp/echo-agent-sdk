#!/usr/bin/env bash
# Test-only OTLP receiver from the precisely pinned framework source. No service
# API is added to the SDK Host and no Python gRPC package is needed.
set -euo pipefail
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
framework_manifest=$(cargo metadata --manifest-path "$repo_root/Cargo.toml" --format-version 1 --locked | python3 -c 'import json,sys; packages=json.load(sys.stdin)["packages"]; print(next(p["manifest_path"] for p in packages if p["name"] == "echo_agent"))')
# Keep dependency test artifacts in the SDK's build root, including when
# the framework source itself lives in Cargo's read-only Git checkout cache.
collector_target=${CARGO_TARGET_DIR:-$repo_root/target}
if [[ "$collector_target" != /* ]]; then
  collector_target="$repo_root/$collector_target"
fi
export CARGO_TARGET_DIR="$collector_target"
# Cargo JSON stdout identifies the real executable without guessing a hash.
cargo test --manifest-path "$framework_manifest" -p echo_agent --no-default-features --features telemetry --test telemetry_composition --no-run --locked --message-format=json | python3 -c 'import json,sys
paths=[]
for line in sys.stdin:
    value=json.loads(line)
    if value.get("reason")=="compiler-artifact" and value.get("target",{}).get("name")=="telemetry_composition" and value.get("executable"):
        paths.append(value["executable"])
if len(paths)!=1:
    raise SystemExit("expected exactly one framework telemetry collector executable")
print(paths[0])'
