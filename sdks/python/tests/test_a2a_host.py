"""Accepted A2A calls use a real Host and the shared HTTP/SSE peer."""

import asyncio
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest
from echo_agent_sdk import EchoAgentClient, EchoAgentError, WireHandle

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required A2A Host missing")
FIXTURE = Path(__file__).resolve().parents[2] / "shared/testing/a2a_host_fixture.py"


@pytest.mark.parametrize("quota", [False, True])
@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
async def test_accepted_a2a_http_sse_owner_and_errors(quota):
    root = Path(
        os.environ.get("ECHO_AGENT_SDK_A2A_ROOT", "../../target/a2a-native")
    ).resolve()
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-", dir=root))
    if quota:
        (work / "quota").write_text("1")
    peer = await asyncio.create_subprocess_exec(
        sys.executable, str(FIXTURE), "--work", str(work)
    )
    rows, values = [], {}

    async def fixture_result(*arguments):
        check = await asyncio.create_subprocess_exec(
            sys.executable,
            str(FIXTURE),
            "--work",
            str(work),
            *arguments,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.STDOUT,
        )
        output, _ = await asyncio.wait_for(check.communicate(), 20)
        assert check.returncode == 0, output.decode()
        return json.loads(output)

    try:

        async def ready():
            while not (work / "ready").exists():
                assert peer.returncode is None
                await asyncio.sleep(0.005)

        await asyncio.wait_for(ready(), 10)
        async with await EchoAgentClient.spawn(
            HOST,
            "--config",
            str(work / "host.json"),
            env={"NO_PROXY": "127.0.0.1,localhost", "no_proxy": "127.0.0.1,localhost"},
        ) as sdk:
            agent = await sdk.create_agent()
            sessions = {
                "a": await agent.create_session(str(work)),
                "b": await agent.create_session(str(work)),
            }
            for step in json.loads((work / "scenario.json").read_text()):
                if step.get("action") == "close_session":
                    await sessions["a"].close()
                    rows.append({"id": step["id"], "value": True})
                    continue
                if step.get("action") == "await_source_closed":
                    source = await fixture_result("--await-source", step["source"])
                    rows.append({"id": step["id"], "value": True, "source": source})
                    continue
                args = []
                for arg in step["args"]:
                    if isinstance(arg, dict) and "ref" in arg:
                        value = values[arg["ref"]]
                        if "field" in arg:
                            value = value[arg["field"]]
                        if "generation_delta" in arg:
                            value = {
                                **(
                                    value.to_dict()
                                    if isinstance(value, WireHandle)
                                    else value
                                ),
                                "generation": str(
                                    int(
                                        value.generation
                                        if isinstance(value, WireHandle)
                                        else value["generation"]
                                    )
                                    + arg["generation_delta"]
                                ),
                            }
                        if isinstance(value, dict) and set(value) == {
                            "id",
                            "generation",
                            "kind",
                        }:
                            value = WireHandle.from_dict(value)
                        args.append(value)
                    else:
                        args.append(arg)
                row = {"id": step["id"]}
                try:
                    row["value"] = await sessions[step.get("session", "a")].invoke(
                        step["op"], args
                    )
                    assert "error" not in step, f"{step['id']} unexpectedly succeeded"
                    values[step["id"]] = row["value"]
                except EchoAgentError as error:
                    assert "error" in step
                    row["error"] = {
                        "code": error.code,
                        "message": error.message,
                        "operation": error.operation,
                        "retryable": error.retryable,
                        "details": error.details,
                        "handle": error.handle.to_dict() if error.handle else None,
                    }
                    assert error.code == step["error"], row
                rows.append(row)
                (work / "results.json").write_text(
                    json.dumps(
                        rows,
                        ensure_ascii=False,
                        indent=2,
                        default=lambda value: value.to_dict(),
                    )
                )
            if quota:
                proof = await fixture_result("--verify")
                assert proof["quota_slot_recovered"]
                await sessions["a"].close()
            await sessions["b"].close()
            await agent.close()
        if not quota:
            proof = await fixture_result("--verify")
            assert len(proof["operations"]) == 9
        print(f"A2A evidence {work}")
    finally:
        (work / "results.json").write_text(
            json.dumps(
                rows,
                ensure_ascii=False,
                indent=2,
                default=lambda value: value.to_dict(),
            )
        )
        (work / "stop").write_text("stop")
        await asyncio.wait_for(peer.wait(), 5)
