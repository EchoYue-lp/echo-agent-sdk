"""Native accepted Tool consumer: actual bytes, cursor fields and process state."""

import asyncio
import copy
import hashlib
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

import pytest
from echo_agent_sdk import EchoAgentClient, EchoAgentError

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "shared/testing"))
import tool_value_effect_fixture as fixture

HOST = os.environ.get("ECHO_AGENT_SDK_HOST", "").strip()
ROOT = Path(
    os.environ.get(
        "ECHO_AGENT_SDK_TOOL_EFFECT_ROOT",
        str(Path(__file__).resolve().parents[3] / "target/tool-value-effects"),
    )
)
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Tool value/effect Host missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]


def processes(work):
    lines = subprocess.check_output(
        ["ps", "-axo", "pid=,ppid=,command="], text=True
    ).splitlines()
    result = []
    for line in lines:
        parts = line.strip().split(None, 2)
        if len(parts) == 3 and parts[2].startswith("tail -f " + str(work / "tail.txt")):
            result.append(
                {"pid": int(parts[0]), "ppid": int(parts[1]), "command": parts[2]}
            )
    return result


def files(work):
    return {
        n: (work / n).read_text(encoding="utf8") if (work / n).is_file() else None
        for n in ["中文🙂/note.txt", "中文🙂/moved.txt"]
    }


async def test_tool_value_effect_native_host():
    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-", dir=ROOT))
    proc = await asyncio.create_subprocess_exec(
        sys.executable, str(Path(fixture.__file__)), "--work", str(work)
    )
    sdk = None
    rows = []
    byid = {}
    try:
        async with asyncio.timeout(10):
            while not (work / "ready").exists():
                assert proc.returncode is None, "fixture exited before ready"
                await asyncio.sleep(0.01)
        sdk = await EchoAgentClient.spawn(
            HOST,
            "--config",
            str(work / "host.json"),
            env={**os.environ, "TMPDIR": str(work / "artifact-temp")},
        )
        agent = await sdk.create_agent()
        session = await agent.create_session(str(work))
        identity = {
            "agent": agent.wire.to_dict(),
            "session": session.wire.to_dict(),
            "capability": sdk.capability,
        }
        for step in json.loads((work / "scenario.json").read_text())["rows"]:
            row = copy.deepcopy(step)
            row["protected_before"] = fixture.snapshot(work)
            row["files_before"] = files(work)
            if step.get("action") == "mutate_artifact":
                artifact = fixture.unrecord(byid[step["ref"]]["result"])
                fixture.decode_path(artifact["path"]).write_text(
                    "mutated 中文🙂\n", encoding="utf8"
                )
            else:
                arguments = fixture.resolve(step["arguments"], byid)
                row["arguments"] = arguments
                try:
                    if step.get("watch_process"):
                        task = asyncio.create_task(
                            session.invoke(step["operation"], arguments)
                        )
                        pids = []
                        for _ in range(50):
                            pids = processes(work)
                            if pids or task.done():
                                break
                            await asyncio.sleep(0.01)
                        row["process_before"] = pids
                        if step.get("terminate_observed"):
                            import signal

                            assert len(pids) == 1, pids
                            assert processes(work) == pids, (
                                "own tail identity changed before signal"
                            )
                            os.kill(pids[0]["pid"], signal.SIGTERM)
                        row["result"] = await task
                        row["process_after"] = processes(work)
                    else:
                        row["result"] = await session.invoke(
                            step["operation"], arguments
                        )
                except EchoAgentError as error:
                    row["error"] = {
                        "code": error.code,
                        "message": str(error),
                        "operation": error.operation,
                        "retryable": error.retryable,
                        "handle": error.handle.to_dict() if error.handle else None,
                        "details": error.details,
                    }
            row["protected_after"] = fixture.snapshot(work)
            row["files_after"] = files(work)
            rows.append(row)
            byid[row["id"]] = row
        await session.close()
        await agent.close()
        data = {
            "work": str(work),
            "host": HOST,
            "host_sha256": hashlib.sha256(Path(HOST).read_bytes()).hexdigest(),
            "raw_identity": identity,
            "rows": rows,
        }
        (work / "results.json").write_text(
            json.dumps(data, ensure_ascii=False, indent=2), encoding="utf8"
        )
        print(
            "Tool value/effects:",
            json.dumps(fixture.verify(work, data), ensure_ascii=False),
            "evidence:",
            work,
        )
    finally:
        if rows and not (work / "results.json").exists():
            (work / "partial-results.json").write_text(
                json.dumps(rows, ensure_ascii=False, indent=2), encoding="utf8"
            )
        if sdk:
            await sdk.close()
        if proc.returncode is None:
            proc.terminate()
        await proc.wait()


async def test_search_provider_real_agent_result_and_failure():
    from echo_agent_sdk import (
        AgentComponentDescriptor,
        AgentComponentOutcome,
        SearchProviderResult,
        to_wire,
    )

    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-search-", dir=ROOT))
    proc = await asyncio.create_subprocess_exec(
        sys.executable, str(Path(fixture.__file__)), "--work", str(work)
    )
    sdk = None
    rows = []
    try:
        async with asyncio.timeout(10):
            while not (work / "ready").exists():
                assert proc.returncode is None
                await asyncio.sleep(0.01)
        sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
        calls = []

        async def provider(call, cancellation):
            row = {
                "operation": call.operation,
                "request": {
                    "operation": call.request.operation,
                    "query": call.request.query,
                    "max_results": call.request.max_results,
                },
                "invocation_id": call.invocation_id,
                "deadline": dict(call.deadline),
                "cancelled": cancellation.is_set(),
            }
            calls.append(row)
            if call.request.query.startswith("failure"):
                raise RuntimeError("provider failed 中文🙂")
            values = [
                {
                    "title": "标题中文🙂",
                    "url": "https://example.invalid/paper",
                    "snippet": "摘要 β🙂",
                },
                {
                    "title": "Second",
                    "url": "https://example.invalid/other",
                    "snippet": "",
                },
            ]
            row["sent"] = values
            return AgentComponentOutcome(
                SearchProviderResult.searched(to_wire(value) for value in values)
            )

        registration = await sdk.register_typed_extension(
            "tool-search-proof",
            AgentComponentDescriptor("search_provider", "controlled-search"),
            provider,
        )
        agent = await sdk.create_agent()
        session = await agent.create_session(str(work))
        await session.invoke(
            "echo_agent::agent::react::ReactAgent::set_permission_mode", ["full-auto"]
        )
        for prompt in ["search-success", "search-error"]:
            before = fixture.snapshot(work)
            run = await session.start_run(prompt)
            settled = await run.wait(timeout=15)
            replay = await run.replay("0", 512)
            rows.append(
                {
                    "prompt": prompt,
                    "settled": settled,
                    "replay": replay,
                    "protected_before": before,
                    "protected_after": fixture.snapshot(work),
                }
            )
            await run.close()
        await session.close()
        await agent.close()
        await registration.close()
        data = {
            "work": str(work),
            "calls": calls,
            "rows": rows,
            "host": HOST,
            "host_sha256": hashlib.sha256(Path(HOST).read_bytes()).hexdigest(),
        }
        (work / "search-results.json").write_text(
            json.dumps(data, ensure_ascii=False, indent=2)
        )
        assert [call["request"]["query"] for call in calls] == [
            "success 中文🙂",
            "failure 中文🙂",
            "failure 中文🙂",
            "failure 中文🙂",
        ], calls
        assert all(
            call["request"]["operation"] == "search_provider_search"
            and call["request"]["max_results"] == 10
            for call in calls
        ), calls
        assert len({call["invocation_id"] for call in calls}) == 4
        requests = [
            json.loads(line) for line in (work / "model.jsonl").read_text().splitlines()
        ]
        tool_messages = [
            m for r in requests for m in r["messages"] if m["role"] == "tool"
        ]
        assert any(
            "标题中文🙂" in str(m["content"])
            and "https://example.invalid/paper" in str(m["content"])
            and "摘要 β🙂" in str(m["content"])
            for m in tool_messages
        ), tool_messages
        assert any(
            "provider failed 中文🙂" in str(m["content"]) for m in tool_messages
        ), tool_messages
        assert all(r["protected_before"] == r["protected_after"] for r in rows)
        print(
            "SearchProvider true Agent exact title/url/snippet/failure: evidence:", work
        )
    finally:
        if sdk:
            await sdk.close()
        if proc.returncode is None:
            proc.terminate()
        await proc.wait()


async def test_sandbox_stream_real_process_completion_cancel_and_close():
    import shlex
    import signal

    from echo_agent_sdk import AgentComponentDescriptor, WireHandle, from_wire, to_wire

    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-sandbox-", dir=ROOT))
    fixture_process = await asyncio.create_subprocess_exec(
        sys.executable, str(Path(fixture.__file__)), "--work", str(work)
    )
    sdk = None
    facts = []
    tasks = []
    try:
        async with asyncio.timeout(10):
            while not (work / "ready").exists():
                assert fixture_process.returncode is None
                await asyncio.sleep(0.01)
        sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
        for mode in ["success", "cancel", "close"]:
            fact = {
                "mode": mode,
                "started": False,
                "cancel_observed": False,
                "finally": 0,
                "pid": None,
                "exit_code": None,
                "sent": [],
            }
            facts.append(fact)
            started = asyncio.Event()
            settled = asyncio.Event()

            async def handler(
                payload,
                cancellation,
                *,
                fact=fact,
                started=started,
                settled=settled,
                mode=mode,
            ):
                request = payload["invocation"]["input"]["call"]
                if request["operation"] == "sandbox_cleanup":
                    assert fact["finally"] == 1 and fact.get("pid_alive") is False, (
                        "cleanup called before original producer exit observation"
                    )
                    fact["cleanup_calls"] = fact.get("cleanup_calls", 0) + 1
                    return {
                        "outcome": "result",
                        "result": {
                            "operation": "agent_component_call",
                            "value": {
                                "component": "sandbox_executor",
                                "result": {"operation": "sandbox_cleanup"},
                            },
                        },
                    }
                if request["operation"] == "sandbox_is_available":
                    return {
                        "outcome": "result",
                        "result": {
                            "operation": "agent_component_call",
                            "value": {
                                "component": "sandbox_executor",
                                "result": {
                                    "operation": "sandbox_is_available",
                                    "value": {"available": True},
                                },
                            },
                        },
                    }
                assert request["operation"] == "sandbox_execute_stream"
                command = from_wire(request["input"]["command"])
                fact["command"] = command
                fact["invocation_id"] = payload["invocation_id"]
                stream = WireHandle.from_dict(payload["stream"])
                writer = sdk.stream_writer(stream)
                child = None
                try:
                    child = await asyncio.create_subprocess_exec(
                        *shlex.split(command["kind"]["Shell"]),
                        stdout=asyncio.subprocess.PIPE,
                        stderr=asyncio.subprocess.PIPE,
                        start_new_session=True,
                    )
                    fact["pid"] = child.pid
                    fact["started"] = True
                    started.set()
                    if mode == "success":
                        stdout, stderr = await child.communicate()
                        fact["exit_code"] = child.returncode
                        for channel, data in [("stdout", stdout), ("stderr", stderr)]:
                            if data:
                                text = data.decode("utf8")
                                fact["sent"].append({"channel": channel, "chunk": text})
                                await writer.agent_component_chunk(
                                    {
                                        "component": "sandbox",
                                        "event": {
                                            "event": "output",
                                            "channel": channel,
                                            "chunk": text,
                                        },
                                    }
                                )
                        result = {
                            "exit_code": child.returncode,
                            "stdout": stdout.decode(),
                            "stderr": stderr.decode(),
                            "duration": {"secs": 0, "nanos": 1},
                            "sandbox_type": "owned-native-fixture",
                            "timed_out": False,
                            "cancelled": False,
                            "output_truncated": False,
                            "stdout_bytes": len(stdout),
                            "stderr_bytes": len(stderr),
                        }
                        await writer.agent_component_complete(
                            {
                                "component": "sandbox",
                                "terminal": {
                                    "terminal": "complete",
                                    "result": to_wire(result),
                                },
                            }
                        )
                        return {"outcome": "stream", "stream": stream.to_dict()}
                    await asyncio.wait_for(cancellation.wait(), 5)
                    fact["cancel_observed"] = cancellation.is_set()
                    os.killpg(child.pid, signal.SIGTERM)
                    await child.wait()
                    fact["exit_code"] = child.returncode
                    return {"outcome": "cancelled"}
                finally:
                    if child and child.returncode is None:
                        os.killpg(child.pid, signal.SIGTERM)
                        await child.wait()
                        fact["exit_code"] = child.returncode
                    fact["finally"] += 1
                    settled.set()

            registration = await sdk.register_extension(
                "agent_component",
                "sandbox-" + mode,
                AgentComponentDescriptor(
                    "sandbox_executor",
                    "native-sandbox",
                    isolation_level="process",
                    supports_streaming=True,
                ).to_wire(),
                handler,
            )
            agent = await sdk.create_agent()
            session = await agent.create_session(str(work))
            shell = (
                "printf 'sandbox 中文🙂\\n'"
                if mode == "success"
                else "tail -f " + str(work / "tail.txt")
            )
            command = {
                "kind": {"Shell": shell},
                "minimum_isolation": None,
                "working_dir": None,
                "env": {},
                "timeout": {"secs": 30, "nanos": 0},
                "stdin": None,
            }
            stream = await session.invoke(
                "sandbox.extension.run_stream", [registration.wire, command]
            )
            fact["stream"] = stream.to_dict()
            await asyncio.wait_for(started.wait(), 5)
            if mode == "success":
                fact["events"] = []
                for _ in range(5):
                    event = await asyncio.wait_for(
                        session.invoke("sandbox.stream.next", [stream]), 5
                    )
                    fact["events"].append(event)
                    assert event["value"]["variant"] != "failed", json.dumps(
                        event, ensure_ascii=False
                    )
                    if event["value"]["variant"] == "complete":
                        fact["terminal_returned_after_cleanup"] = settled.is_set()
                        break
            else:
                op = (
                    "sandbox.stream.cancel"
                    if mode == "cancel"
                    else "sandbox.stream.close"
                )
                before = asyncio.get_running_loop().time()
                fact["close_result"] = await asyncio.wait_for(
                    session.invoke(op, [stream]), 8
                )
                fact["control_elapsed_ms"] = (
                    asyncio.get_running_loop().time() - before
                ) * 1000
                fact["control_returned_after_cleanup"] = settled.is_set()
            await asyncio.wait_for(settled.wait(), 6)
            fact["pid_alive"] = False
            try:
                os.kill(fact["pid"], 0)
                fact["pid_alive"] = True
            except ProcessLookupError:
                pass
            fact["repeated_close"] = await session.invoke(
                "sandbox.stream.close", [stream]
            )
            fact["protected"] = fixture.snapshot(work)
            await session.close()
            await agent.close()
            await registration.close()
        data = {
            "work": str(work),
            "host": HOST,
            "host_sha256": hashlib.sha256(Path(HOST).read_bytes()).hexdigest(),
            "facts": facts,
        }
        (work / "sandbox-results.json").write_text(
            json.dumps(data, ensure_ascii=False, indent=2)
        )
        assert all(
            f["started"] and f["finally"] == 1 and not f["pid_alive"] for f in facts
        ), facts
        assert facts[0]["sent"] == [{"channel": "stdout", "chunk": "sandbox 中文🙂\n"}]
        assert all(f["cancel_observed"] and f["exit_code"] < 0 for f in facts[1:]), (
            facts
        )
        assert facts[0]["events"][-1]["value"]["variant"] == "complete"
        print("Sandbox true process complete/cancel/close: evidence:", work)
    finally:
        (work / "sandbox-partial.json").write_text(
            json.dumps(facts, ensure_ascii=False, indent=2)
        )
        if tasks:
            await asyncio.gather(*tasks, return_exceptions=True)
        if sdk:
            await sdk.close()
        if fixture_process.returncode is None:
            fixture_process.terminate()
        await fixture_process.wait()
