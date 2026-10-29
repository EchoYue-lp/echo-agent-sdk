#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

prepare_consumer_root() {
  local selected=${ECHO_AGENT_SDK_CONSUMER_ROOT:-}
  if [[ -z "$selected" ]]; then
    local parent="$repo_root/.echo-semantic/local/language-consumer-runs"
    mkdir -p "$parent" || return
    selected=$(mktemp -d "$parent/run.XXXXXX") || return
  fi
  # 本地验收仍需保证夹具和恢复材料写在工作根内；不修改调用者提供的有效目录。
  python3 -B - "$repo_root" "$selected" <<'PY'
from pathlib import Path
import sys
workspace = Path(sys.argv[1]).resolve().parent
selected = Path(sys.argv[2])
if not selected.is_absolute():
    raise SystemExit("error: consumer runtime root must be absolute")
selected = selected.resolve()
if selected == workspace or not selected.is_relative_to(workspace):
    raise SystemExit("error: consumer runtime root must be inside the workspace")
selected.mkdir(parents=True, exist_ok=True)
print(selected)
PY
}

command -v node >/dev/null || { echo "error: node is required for the TypeScript SDK" >&2; exit 1; }
command -v npm >/dev/null || { echo "error: npm is required for the TypeScript SDK" >&2; exit 1; }
command -v uv >/dev/null || { echo "error: uv is required for the Python SDK" >&2; exit 1; }
command -v mvn >/dev/null || { echo "error: Maven is required for the Java SDK" >&2; exit 1; }
command -v cargo >/dev/null || { echo "error: Cargo is required for the Host smoke" >&2; exit 1; }

jq -e '.schema_version == 1 and .source_only == true and .typescript.node == ">=20" and .python.python == ">=3.10" and .java.jdk == "17"' \
  "$repo_root/sdks/shared/toolchain.json" >/dev/null || {
  echo "error: shared SDK toolchain contract is invalid" >&2
  exit 1
}
node_major=$(node -p 'Number(process.versions.node.split(".")[0])')
(( node_major >= 20 )) || { echo "error: Node.js 20 or newer is required" >&2; exit 1; }

if ! (cd "$repo_root/sdks/python" && uv run python -c 'import sys; raise SystemExit(0 if sys.version_info >= (3, 10) else 1)'); then
  echo "error: Python 3.10 or newer is required" >&2
  exit 1
fi

java_version=$(java -version 2>&1 | sed -n 's/.*version "\([^"]*\)".*/\1/p' | head -n 1)
java_major=${java_version%%.*}
[[ "$java_major" =~ ^[0-9]+$ ]] && (( java_major >= 17 )) || {
  echo "error: JDK 17 or newer is required" >&2
  exit 1
}

scripts/export-language-sdk-catalog.sh --check

sdk_consumer_root=$(prepare_consumer_root)
export ECHO_AGENT_SDK_CONSUMER_ROOT="$sdk_consumer_root"

external_incomplete=$(jq -r '
  [.entries[] | .languages | to_entries[] | select(.value.status != "done")] | length
' "$repo_root/contracts/sdk/accepted-external-contract.json")
[[ "$external_incomplete" == "0" ]] || {
  echo "error: external SDK contracts contain incomplete language mappings" >&2
  exit 1
}
scope_counts=$(jq -r '
  [.entries[] | select(.canonical)]
  | [
      ([.[] | select(.sdk_scope == "external_contract")] | length),
      ([.[] | select(.sdk_scope == "host_or_rust_only")] | length),
      ([.[] | select(.sdk_scope == "language_intrinsic")] | length),
      ([.[] | select(.sdk_scope == "internal_helper")] | length),
      ([.[] | select(.sdk_scope == "deferred")] | length)
    ]
  | @tsv
' "$repo_root/contracts/sdk/parity-manifest.json")
printf 'SDK scopes (telemetry only, canonical): external=%s host_or_rust_only=%s language_intrinsic=%s internal_helper=%s deferred=%s\n' \
  ${scope_counts//$'\t'/ }

# A caller may retain the current batch's freshly built Host outside target
# before cargo clean. The mandatory digest binds this invocation to that exact
# executable; all language suites/Host E2E and examples still run below.
prebuilt_host=${ECHO_AGENT_SDK_PREBUILT_HOST:-}
if [[ -n "$prebuilt_host" ]]; then
  [[ "$prebuilt_host" == /* && -x "$prebuilt_host" ]] || { echo "error: prebuilt Host must be an absolute executable" >&2; exit 1; }
  expected_host_digest=${ECHO_AGENT_SDK_PREBUILT_HOST_SHA256:-}
  [[ "$expected_host_digest" =~ ^[0-9a-f]{64}$ ]] || { echo "error: exact prebuilt Host SHA256 required" >&2; exit 1; }
  actual_host_digest=$(shasum -a 256 "$prebuilt_host" | awk '{print $1}')
  [[ "$actual_host_digest" == "$expected_host_digest" ]] || { echo "error: prebuilt Host digest mismatch" >&2; exit 1; }
else
  cargo build -q -p echo-sdk-host --features sdk-facade-all --locked
fi
host_state=$(mktemp -d "${TMPDIR:-/tmp}/echo-sdk-language-state.XXXXXX")
host_config=$(mktemp "${TMPDIR:-/tmp}/echo-sdk-language-config.XXXXXX")
jq --arg root "$host_state" '.sdk_profile.state_root = $root | .default_agent.agent.enable_memory = true | .default_agent.agent.memory_path = ($root + "/memory.jsonl")' \
  "$repo_root/echo-sdk-host/config.sdk.example.json" > "$host_config"
host_target_dir=${CARGO_TARGET_DIR:-$repo_root/target}
if [[ "$host_target_dir" != /* ]]; then
  host_target_dir="$repo_root/$host_target_dir"
fi
export ECHO_AGENT_SDK_HOST="${prebuilt_host:-$host_target_dir/debug/echo-agent-sdk-host}"
export ECHO_AGENT_SDK_CONFIG="$host_config"
export ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1
export ECHO_AGENT_SDK_OTLP_COLLECTOR
if [[ -n "$prebuilt_host" ]]; then
  [[ "${ECHO_AGENT_SDK_OTLP_COLLECTOR:-}" == /* && -x "$ECHO_AGENT_SDK_OTLP_COLLECTOR" ]] || { echo "error: preserved collector must be an absolute executable" >&2; exit 1; }
  expected_collector_digest=${ECHO_AGENT_SDK_PREBUILT_COLLECTOR_SHA256:-}
  [[ "$expected_collector_digest" =~ ^[0-9a-f]{64}$ ]] || { echo "error: exact collector SHA256 required" >&2; exit 1; }
  actual_collector_digest=$(shasum -a 256 "$ECHO_AGENT_SDK_OTLP_COLLECTOR" | awk '{print $1}')
  [[ "$actual_collector_digest" == "$expected_collector_digest" ]] || { echo "error: preserved collector digest mismatch" >&2; exit 1; }
else
  ECHO_AGENT_SDK_OTLP_COLLECTOR=$(bash "$repo_root/scripts/build-telemetry-test-collector.sh")
fi

(cd "$repo_root/sdks/typescript" && npm ci && npm test && node dist-examples/examples/quickstart.js)
(cd "$repo_root/sdks/python" && \
  PYTHONPATH=src uv run --no-project --with ruff ruff check src tests examples ../shared/testing && \
  PYTHONPATH=src uv run --no-project --with ruff ruff format --check src tests examples ../shared/testing && \
  PYTHONPATH=src uv run --no-project --with pytest --with pytest-asyncio --with agent-client-protocol==0.12.1 python -m pytest -q tests ../shared/testing && \
  PYTHONPATH=src uv run python examples/quickstart.py)
# 三语言测试同时验证 tgz/wheel/sdist/JAR 的包内合同；Java 示例也走包内默认资源。
(cd "$repo_root/sdks/java" && mvn -q test && mvn -q dependency:build-classpath -Dmdep.outputFile="$host_state/java.cp" && java -cp "target/classes:$(cat "$host_state/java.cp")" com.echoagent.sdk.Example "$ECHO_AGENT_SDK_HOST" "$ECHO_AGENT_SDK_CONFIG")

printf 'language SDK source checks passed\n'
