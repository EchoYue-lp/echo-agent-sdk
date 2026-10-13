#!/usr/bin/env bash
set -euo pipefail
sdk_task_root=/Users/ls/MyWork/code/ylp_agent_learn/echo-agent-sdk-worktrees/sdk-semantic-governance
sdk_task_host=/tmp/echo-sdk-governance-parent-driver-host
sdk_task_state=$(mktemp -d /tmp/sdk-full-frozen-language-state.XXXXXX)
export ECHO_AGENT_SDK_HOST="$sdk_task_host"
export ECHO_AGENT_SDK_CONFIG="$sdk_task_state/host.json"
export ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1
jq --arg root "$sdk_task_state" '.sdk_profile.state_root = $root | .default_agent.agent.enable_memory = true | .default_agent.agent.memory_path = ($root + "/memory.jsonl")' "$sdk_task_root/echo-sdk-host/config.sdk.example.json" > "$ECHO_AGENT_SDK_CONFIG"
"$sdk_task_host" --config "$ECHO_AGENT_SDK_CONFIG" --check-config
(cd "$sdk_task_root/sdks/typescript" && npm ci && npm test && node dist-examples/examples/quickstart.js)
(cd "$sdk_task_root/sdks/python" && \
  PYTHONPATH=src uv run --no-project --with ruff ruff check src tests examples ../shared/testing && \
  PYTHONPATH=src uv run --no-project --with ruff ruff format --check src tests examples ../shared/testing && \
  PYTHONPATH=src uv run --no-project --with pytest --with pytest-asyncio --with agent-client-protocol==0.12.1 pytest -q tests ../shared/testing && \
  PYTHONPATH=src uv run python examples/quickstart.py)
(cd "$sdk_task_root/sdks/java" && mvn -q test && mvn -q dependency:build-classpath -Dmdep.outputFile="$sdk_task_state/java.cp" && java -cp "target/classes:$(cat "$sdk_task_state/java.cp")" com.echoagent.sdk.Example "$ECHO_AGENT_SDK_HOST" "$ECHO_AGENT_SDK_CONFIG" "$sdk_task_root/sdks/shared/facade-operation-catalog.json")
printf 'Frozen Host language commands and examples passed\n'
