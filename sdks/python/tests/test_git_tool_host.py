"""Real Host Git calls; shared oracle observes owned files, refs and index entries."""

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "shared/testing"))
from file_sha256 import file_sha256

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Git Host missing")
FIXTURE = (
    Path(__file__).resolve().parents[2] / "shared/testing/git_tool_host_fixture.py"
)


def resolve(value, results):
    if isinstance(value, list):
        return [resolve(v, results) for v in value]
    if isinstance(value, dict):
        if "$ref" in value:
            return results[value["$ref"]]
        return {key: resolve(v, results) for key, v in value.items()}
    return value


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
async def test_git_tools_and_worktree_source_exact_local_effects():
    root = Path(
        os.environ.get("ECHO_AGENT_SDK_GIT_ROOT", "../../target/git-native")
    ).resolve()
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-", dir=root))

    def oracle(*args):
        return subprocess.check_output(
            [sys.executable, str(FIXTURE), "--work", str(work), *args], text=True
        )

    oracle("--prepare")
    scenario = json.loads((work / "scenario.json").read_text())
    rows, results = [], {}
    try:
        async with await EchoAgentClient.spawn(
            HOST, "--config", str(work / "host.json"), env=scenario["host_env"]
        ) as sdk:
            agent = await sdk.create_agent()
            session = await agent.create_session(scenario["repo"])
            for step in scenario["rows"]:
                if step["checkpoint"]:
                    oracle("--checkpoint", step["checkpoint"])
                row = {
                    "id": step["id"],
                    "operation": step["operation"],
                    "arguments": resolve(step["arguments"], results),
                    "before": json.loads(oracle("--snapshot")),
                }
                try:
                    row["result"] = await session.invoke(
                        step["operation"], row["arguments"]
                    )
                    results[step["id"]] = row["result"]
                except EchoAgentError as error:
                    row["error"] = {
                        "code": error.code,
                        "message": error.message,
                        "operation": error.operation,
                        "retryable": error.retryable,
                        "handle": error.handle.to_dict() if error.handle else None,
                        "details": error.details,
                    }
                row["after"] = json.loads(oracle("--snapshot"))
                rows.append(row)
                (work / "results.json").write_text(
                    json.dumps(
                        {
                            "work": str(work),
                            "host": HOST,
                            "host_sha256": file_sha256(HOST),
                            "issued_session": session.wire.to_dict(),
                            "rows": rows,
                        },
                        ensure_ascii=False,
                        indent=2,
                        default=lambda value: value.to_dict(),
                    )
                )
            await session.close()
            await agent.close()
        proof = json.loads(oracle("--verify"))
        assert len(proof["tool_operations"]) == 9
        assert len(proof["source_operations"]) == 4
        assert len(proof["groups"]) == 3
        assert len(proof["checkpoint_operations"]) == 3
        assert proof["checkpoint_calls"] == 7
    finally:
        print(f"Git evidence retained {work}")
