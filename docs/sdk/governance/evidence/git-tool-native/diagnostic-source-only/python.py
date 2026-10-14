"""Finite source-only diagnostic; no branch or final-snapshot acceptance claim."""
import asyncio
import hashlib
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(os.environ.get("ECHO_AGENT_SDK_SOURCE_ROOT", os.getcwd())).resolve()
sys.path.insert(0, str(ROOT / "sdks/python/src"))
from echo_agent_sdk import EchoAgentClient, EchoAgentError

FIXTURE = ROOT / "sdks/shared/testing/git_tool_host_fixture.py"
HOST = os.environ["ECHO_AGENT_SDK_HOST"]
BASE = Path(os.environ["ECHO_AGENT_SDK_GIT_ROOT"])
BASE.mkdir(parents=True, exist_ok=True)
WORK = Path(tempfile.mkdtemp(prefix="python-", dir=BASE))
ROWS, VALUES = [], {}


def oracle(*args):
    return subprocess.check_output([sys.executable, str(FIXTURE), "--work", str(WORK), *args], text=True)


def resolve(value):
    if isinstance(value, list):
        return [resolve(item) for item in value]
    if isinstance(value, dict):
        return VALUES[value["$ref"]] if "$ref" in value else {key: resolve(item) for key, item in value.items()}
    return value


def write():
    (WORK / "results.json").write_text(json.dumps({"scope": "a634-private-heap-partition-source-only-diagnostic", "host_sha256": hashlib.sha256(Path(HOST).read_bytes()).hexdigest(), "work": str(WORK), "rows": ROWS}, ensure_ascii=False, indent=2, default=lambda value: value.to_dict()))


async def main():
    oracle("--prepare")
    oracle("--checkpoint", "prepare-managed")
    scenario = json.loads((WORK / "scenario.json").read_text())
    async with await EchoAgentClient.spawn(HOST, "--config", str(WORK / "host.json"), env=scenario["host_env"]) as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session(scenario["repo"])
        steps = [step for step in scenario["rows"] if step["mode"] == "source"]
        steps.insert(3, {**next(step for step in steps if step["id"] == "source-list"), "id": "alive-after-refusal"})
        for step in steps:
            if step["checkpoint"]:
                oracle("--checkpoint", step["checkpoint"])
            row = {"id": step["id"], "operation": step["operation"], "arguments": resolve(step["arguments"]), "before": json.loads(oracle("--snapshot"))}
            try:
                row["result"] = await session.invoke(step["operation"], row["arguments"])
                VALUES[step["id"]] = row["result"]
            except EchoAgentError as error:
                row["error"] = {"code": error.code, "message": error.message, "operation": error.operation, "retryable": error.retryable, "handle": error.handle.to_dict() if error.handle else None, "details": error.details}
            row["after"] = json.loads(oracle("--snapshot"))
            ROWS.append(row)
            write()
            if step["error"]:
                assert row["error"] == {"code": "framework_error", "message": "Refusing to remove a worktree with uncommitted changes", "operation": step["operation"], "retryable": "never", "handle": None, "details": None}
                assert row["before"] == row["after"]
            else:
                assert "error" not in row, row
            if step["id"] == "alive-after-refusal":
                assert row["result"] == VALUES["source-list"]
                assert row["before"] == row["after"]
            if step["id"] == "source-merge":
                assert row["after"]["repo"]["head"] == row["before"]["source"]["head"]
                assert row["after"]["repo"]["branch"] == "main"
                assert row["after"]["repo"]["files"]["tracked.txt"] == row["before"]["source"]["files"]["tracked.txt"]
                assert row["after"]["repo"]["merge_active"] is False
            if step["id"] == "source-remove":
                assert row["result"] is None
                assert row["after"]["source"]["exists"] is False
                assert "refs/heads/source-proof" in row["after"]["repo"]["refs"]
            if step["id"] == "source-after-list":
                assert len(row["result"]) == 1
                assert row["before"] == row["after"]
        await session.close()
        await agent.close()
    (WORK / "verified.json").write_text(json.dumps({"scope": "finite-source-only-diagnostic", "native_calls": len(ROWS), "operations": sorted({row["operation"] for row in ROWS}), "framework": "a6346643bda47679a96760f56f45e41fd73b61b0", "final_sdk_acceptance": False}, indent=2))
    print(WORK)


try:
    asyncio.run(main())
finally:
    write()
