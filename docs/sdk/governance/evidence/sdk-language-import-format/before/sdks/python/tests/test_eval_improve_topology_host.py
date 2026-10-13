"""Shared inputs, actual Python callbacks and real framework evidence."""

import asyncio
import base64
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest
from echo_agent_sdk import EchoAgentClient, EchoAgentError, WireHandle, wire_utf8_path

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required analysis Host missing")
SHARED = (
    Path(__file__).resolve().parents[2]
    / "shared/testing/eval_improve_topology_fixture.py"
)
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]


async def helper(work, *args):
    p = await asyncio.create_subprocess_exec(
        sys.executable,
        str(SHARED),
        "--work",
        str(work),
        *args,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )
    out, err = await p.communicate()
    assert p.returncode == 0, err.decode()
    return out.decode()


def outcome(operation, value=None):
    return {"outcome": "result", "result": {"operation": operation, "value": value}}


@pytest.mark.parametrize("group", ["eval", "improve", "topology"])
async def test_eval_improve_topology_native(group):
    root = Path(
        os.environ.get(
            "ECHO_AGENT_SDK_ANALYSIS_ROOT",
            str(Path(__file__).resolve().parents[3] / "target/analysis-native"),
        )
    )
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-" + group + "-", dir=root))
    await helper(work, "--prepare", group)
    spec = json.loads((work / "scenario.json").read_text())
    facts = {
        "language": "python",
        "group": group,
        "created": [],
        "closed": [],
        "executed": [],
        "callbacks": [],
        "factory_finally": 0,
        "cancel_observed": 0,
        "rows": [],
    }
    instances = {}
    values = {}
    mode = "plain"
    deadline_cancelled = asyncio.Event()
    deadline_release = asyncio.Event()
    factory = None
    sdk = await EchoAgentClient.spawn(
        HOST,
        "--config",
        str(work / "host.json"),
        env={**os.environ, "TMPDIR": str(work / "runtime-temp")},
    )
    try:

        async def factory_handler(call, cancel):
            op = call["invocation"]["operation"]
            facts["callbacks"].append(call)
            if op == "factory_create_agent":
                instance = call["invocation"]["input"]["instance"]
                assert instance["id"] not in instances
                instances[instance["id"]] = instance
                facts["created"].append(instance["id"])
                return outcome(
                    op,
                    {
                        "name": "native-case",
                        "model_name": "native-case",
                        "system_prompt": "native-case",
                        "tool_names": [],
                    },
                )
            assert call["factory_origin"] == factory.wire.to_dict()
            identity = call["extension"]["id"]
            assert identity in instances
            if op == "agent_close":
                facts["closed"].append(identity)
                instances.pop(identity)
                return outcome(op)
            facts["executed"].append(identity)
            try:
                task = call["invocation"]["input"].get(
                    "task", call["invocation"]["input"].get("message", "")
                )
                if task == "hang":
                    await asyncio.wait_for(cancel.wait(), 10)
                    facts["cancel_observed"] += int(cancel.is_set())
                    deadline_cancelled.set()
                    await asyncio.wait_for(deadline_release.wait(), 10)
                    return outcome(op, "cancelled")
                answer = "wrong response" if task == "bad" else spec["factory_answer"]
                if op.endswith("_stream"):
                    await sdk.stream_writer(
                        WireHandle.from_dict(call["stream"])
                    ).complete(
                        {
                            "kind": "agent",
                            "value": {
                                "event": "final_answer",
                                "data": {"text": answer},
                            },
                        }
                    )
                    return {"outcome": "stream", "stream": call["stream"]}
                return outcome(op, answer)
            finally:
                facts["factory_finally"] += 1

        factory = await sdk.register_agent_factory(
            "analysis-factory",
            {"kind": "agent_factory", "descriptor_version": 1},
            factory_handler,
        )

        async def custom_handler(call, _cancel):
            op = call["invocation"]["operation"]
            facts["callbacks"].append(call)
            if op == "agent_close":
                return outcome(op)
            answer = (
                spec["factory_answer"]
                if mode == "plain"
                else json.dumps(spec["grade_good"], ensure_ascii=False)
                if mode == "grade"
                else "not valid JSON🙂"
            )
            if op.endswith("_stream"):
                await sdk.stream_writer(WireHandle.from_dict(call["stream"])).complete(
                    {
                        "kind": "agent",
                        "value": {"event": "final_answer", "data": {"text": answer}},
                    }
                )
                return {"outcome": "stream", "stream": call["stream"]}
            return outcome(op, answer)

        custom = await sdk.register_custom_agent(
            "analysis-custom",
            {
                "kind": "custom_agent",
                "descriptor_version": 1,
                "name": "analysis-custom",
                "model_name": "analysis-custom",
                "system_prompt": "analysis-custom",
                "tool_names": [],
            },
            custom_handler,
        )
        agent = await sdk.create_agent()
        sessions = {
            "a": await agent.create_session(),
            "b": await agent.create_session(),
        }

        def resolve(value):
            if isinstance(value, list):
                return [resolve(item) for item in value]
            if not isinstance(value, dict):
                return value
            if "$ref" in value:
                selected = values[value["$ref"]]
                for field in value.get("path", []):
                    selected = selected[field]
                return (
                    WireHandle.from_dict(selected)
                    if isinstance(selected, dict)
                    and set(selected) == {"id", "kind", "generation"}
                    else selected
                )
            if "$path" in value:
                return wire_utf8_path(str(work / value["$path"]))
            if "$string_path" in value:
                return str(work / value["$string_path"])
            if "$u64" in value:
                return {"kind": "u64", "value": value["$u64"]}
            if "$f64" in value:
                return {"kind": "f64", "value": value["$f64"]}
            if "$factory" in value:
                return factory.wire
            if "$custom" in value:
                return custom.wire
            return {key: resolve(item) for key, item in value.items()}

        for step in spec["steps"]:
            row = {"id": step["id"]}
            if step.get("action") == "custom_mode":
                mode = step["mode"]
                row.update(outcome="result", value=mode)
            elif step.get("action") in ("block_directory", "restore_directory"):
                directory = work / step["directory"]
                backup = directory.with_name(directory.name + ".retained")
                if step["action"] == "block_directory":
                    files = sorted(directory.glob("*.jsonl"))
                    facts["trajectory_bytes_before"] = [
                        base64.b64encode(file.read_bytes()).decode() for file in files
                    ]
                    directory.rename(backup)
                    directory.write_text("blocks write")
                else:
                    directory.unlink()
                    backup.rename(directory)
                    facts["trajectory_bytes_after"] = [
                        base64.b64encode(file.read_bytes()).decode()
                        for file in sorted(directory.glob("*.jsonl"))
                    ]
                row.update(outcome="result", value=True)
            else:
                session = sessions[step.get("session", "a")]
                args = resolve(step["arguments"])
                try:
                    if step["operation"] == "facade.resource.close":
                        value = await sdk.request(
                            "_echo_agent/facade/invoke",
                            {
                                "operation": step["operation"],
                                "signature_digest": sdk.catalog.resolve(
                                    step["operation"]
                                ).signature,
                                "handle": session.wire.to_dict(),
                                "arguments": [
                                    {"kind": "handle", "value": args[0].to_dict()}
                                ],
                            },
                        )
                    else:
                        request = asyncio.create_task(
                            session.invoke(step["operation"], args)
                        )
                        if step["id"] == "timeout":
                            notice = asyncio.create_task(deadline_cancelled.wait())
                            await asyncio.wait(
                                {request, notice},
                                timeout=16,
                                return_when=asyncio.FIRST_COMPLETED,
                            )
                            facts["deadline_waiter_pending_during_cleanup"] = (
                                deadline_cancelled.is_set() and not request.done()
                            )
                            facts["workspace_exists_during_cleanup"] = any(
                                (work / "eval-configured").glob("eval-*")
                            )
                            deadline_release.set()
                            notice.cancel()
                        value = await asyncio.wait_for(request, 25)
                    row.update(outcome="result", value=value)
                    values[step["id"]] = value
                except EchoAgentError as error:
                    row.update(
                        outcome="error",
                        code=error.code,
                        message=str(error),
                        details=error.details,
                        operation=error.operation,
                    )
            facts["rows"].append(row)
            (work / "observed.json").write_text(
                json.dumps(
                    facts,
                    ensure_ascii=False,
                    indent=2,
                    default=lambda value: value.to_dict(),
                )
            )
        assert not instances
        await sessions["a"].close()
        await sessions["b"].close()
        await agent.close()
        await custom.close()
        await factory.close()
    finally:
        deadline_release.set()
        await sdk.close()
        (work / "observed.json").write_text(
            json.dumps(
                facts,
                ensure_ascii=False,
                indent=2,
                default=lambda value: value.to_dict(),
            )
        )
        print("Analysis native evidence: " + str(work))
    print(await helper(work, "--verify"))
