"""Native Python calls accepted RAG/InstructionResolver operations on real Host."""

import asyncio
import hashlib
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError, WireHandle

FIXTURE = (
    Path(__file__).resolve().parents[2] / "shared/testing/rag_project_rules_fixture.py"
)
ROOT = Path(
    os.environ.get(
        "ECHO_AGENT_SDK_RAG_RULES_ROOT",
        str(Path(__file__).resolve().parents[3] / "target/rag-project-rules"),
    )
)
HOST = os.environ.get("ECHO_AGENT_SDK_HOST", "").strip()
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required RAG/project-rules Host missing")


async def helper(work, *arguments):
    process = await asyncio.create_subprocess_exec(
        sys.executable,
        str(FIXTURE),
        "--work",
        str(work),
        *arguments,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )
    output, error = await process.communicate()
    assert process.returncode == 0, error.decode()
    return output.decode()


def portable(value):
    if isinstance(value, WireHandle):
        return {"kind": "handle", "value": value.to_dict()}
    if isinstance(value, dict):
        return {key: portable(item) for key, item in value.items()}
    if isinstance(value, list):
        return [portable(item) for item in value]
    return value


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
async def test_rag_project_rules_shared_authority_errors_and_scope():
    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-", dir=ROOT))
    await helper(work)
    diagnostics = (work / "server.stderr").open("w")
    server = await asyncio.create_subprocess_exec(
        sys.executable,
        str(FIXTURE),
        "--work",
        str(work),
        "--serve",
        stdout=asyncio.subprocess.DEVNULL,
        stderr=diagnostics,
    )
    rows, values, sdk = [], {}, None
    try:
        for _ in range(200):
            if (work / "endpoint.json").exists():
                break
            await asyncio.sleep(0.01)
        endpoint = json.loads((work / "endpoint.json").read_text())["url"]
        env = {
            "EMBEDDING_BASEURL": endpoint,
            "EMBEDDING_MODEL": "fixture-中文-embedding",
            "EMBEDDING_APIKEY": "fixture-key-not-secret",
        }
        sdk = await EchoAgentClient.spawn(
            HOST, "--config", str(work / "host.json"), env=env
        )
        agent = await sdk.create_agent()
        sessions = {
            name: await agent.create_session(str(work / cwd))
            for name, cwd in {
                "a": "repo/nested/leaf",
                "b": "other",
                "empty": "empty",
                "new": "repo/nested/leaf",
            }.items()
        }

        def resolve(value, session):
            if isinstance(value, dict):
                return {key: resolve(item, session) for key, item in value.items()}
            if isinstance(value, str):
                if value == "$session":
                    return session.wire
                if value.startswith("$") and not value.startswith("$WORK"):
                    return values[value[1:]]
                return value.replace("$WORK", str(work))
            return value

        for step in json.loads((work / "scenario.json").read_text())["steps"]:
            if step.get("action"):
                if step["action"] == "close":
                    await sessions[step["session"]].close()
                else:
                    write = work / "action.json"
                    write.write_text(json.dumps(step, ensure_ascii=False))
                    await helper(work, "--action", str(write))
                continue
            session = sessions[step["session"]]
            arguments = [resolve(value, session) for value in step["arguments"]]
            row = {
                "id": step["id"],
                "operation": step["operation"],
                "arguments": portable(arguments),
            }
            try:
                receiver = (
                    agent.wire
                    if step["operation"].startswith("echo_core::")
                    else session.wire
                )
                result = await sdk.call(step["operation"], receiver, arguments)
                if step.get("save"):
                    values[step["save"]] = result
                row["result"] = portable(result)
            except EchoAgentError as error:
                row["error"] = {
                    "code": error.code,
                    "message": error.message,
                    "operation": error.operation,
                    "retryable": error.retryable,
                    "details": error.details,
                    "handle": portable(error.handle),
                }
            rows.append(row)
            (work / "rows.json").write_text(
                json.dumps(rows, ensure_ascii=False, indent=2)
            )
        for name in ["b", "empty", "new"]:
            await sessions[name].close()
        await agent.close()
        await sdk.close()
        sdk = None
        async with await EchoAgentClient.spawn(
            HOST,
            "--config",
            str(work / "host.json"),
            env={**env, "EMBEDDING_MODEL": ""},
        ) as bad:
            agent = await bad.create_agent()
            session = await agent.create_session(str(work / "empty"))
            with pytest.raises(EchoAgentError) as caught:
                await bad.call("rag_search", session.wire, ["query", 1])
            error = caught.value
            rows.append(
                {
                    "id": "invalid-config",
                    "operation": "rag_search",
                    "error": {
                        "code": error.code,
                        "message": error.message,
                        "operation": error.operation,
                        "retryable": error.retryable,
                        "details": error.details,
                        "handle": portable(error.handle),
                    },
                }
            )
            await session.close()
            await agent.close()
        evidence = {
            "language": "python",
            "host": HOST,
            "host_sha256": hashlib.sha256(Path(HOST).read_bytes()).hexdigest(),
            "work": str(work),
            "rows": rows,
            "server_pid": server.pid,
        }
        (work / "results.json").write_text(
            json.dumps(evidence, ensure_ascii=False, indent=2)
        )
        print(
            "RAG/project-rules verified:",
            await helper(work, "--verify", str(work / "results.json")),
            "evidence:",
            work,
        )
    finally:
        if sdk is not None:
            await sdk.close()
        server.terminate()
        await server.wait()
        diagnostics.close()
        print("RAG/project-rules retained:", work)
