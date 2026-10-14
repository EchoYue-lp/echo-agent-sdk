import asyncio
import json
import os
import sys
import tempfile
import urllib.request
from pathlib import Path

import pytest

from echo_agent_sdk import (
    EchoAgentClient,
    EchoAgentError,
    WireHandle,
    from_wire,
    to_wire,
)

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required real Task/Subagent Host binary missing")
pytestmark = pytest.mark.skipif(not HOST, reason="real Host binary required")
FIXTURE = str(
    Path(__file__).resolve().parents[2] / "shared/testing/task_subagent_fixture.py"
)


async def until(check):
    deadline = asyncio.get_running_loop().time() + 15
    while not await check():
        assert asyncio.get_running_loop().time() < deadline, (
            "framework condition missing"
        )
        await asyncio.sleep(0.01)


async def launch(work):
    backend = await asyncio.create_subprocess_exec(
        sys.executable, FIXTURE, "--work", str(work)
    )

    async def ready():
        return (work / "ready").exists()

    await until(ready)
    url = json.loads((work / "fixture.json").read_text())["url"]

    async def http(path, payload=None):
        def send():
            data = (
                None
                if payload is None
                else json.dumps(payload, ensure_ascii=False).encode()
            )
            request = urllib.request.Request(
                url + path, data=data, headers={"Content-Type": "application/json"}
            )
            with urllib.request.urlopen(request, timeout=35) as response:
                return json.load(response)

        return await asyncio.to_thread(send)

    return backend, http


def handle(value):
    return value.to_dict() if isinstance(value, WireHandle) else value


async def register_store(sdk, http, execution):
    async def consume(call, cancel):
        response = await http("/store", call["invocation"]["input"]["call"])
        return (
            {"outcome": "error", "error": response["error"]}
            if response["kind"] == "error"
            else {
                "outcome": "result",
                "result": {
                    "operation": "agent_component_call",
                    "value": {
                        "component": "revisioned_task_store",
                        "result": response["result"],
                    },
                },
            }
        )

    return await sdk.register_extension(
        "agent_component",
        "task-store",
        {
            "kind": "agent_component",
            "descriptor_version": 1,
            "component": "revisioned_task_store",
            "name": "task-store",
            "capabilities": {"execution_cas_v1": execution},
        },
        consume,
    )


@pytest.mark.asyncio
@pytest.mark.parametrize("action", ["pause", "cancel"])
async def test_exact_task_control(action):
    work = Path(tempfile.mkdtemp(prefix=f"sdk-py-task-{action}-"))
    print("Task evidence", work)
    backend, http = await launch(work)
    sdk = None
    jobs = []
    try:
        sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
        registrations = [await register_store(sdk, http, True)]
        counts = dict.fromkeys("abcd", 0)
        prompts = {}
        callback_errors = []
        canonical_task_run_scope = None
        for lane in "abcd":

            async def consume(call, cancel, lane=lane):
                operation = call["invocation"]["operation"]
                if operation == "agent_close":
                    return {
                        "outcome": "result",
                        "result": {"operation": operation, "value": None},
                    }
                assert operation == "agent_execute_stream", operation
                assert call["context"]["run_id"] == canonical_task_run_scope
                counts[lane] += 1
                attempt = counts[lane]
                prompts[f"{lane}/{attempt}"] = call["invocation"]["input"]["task"]
                await http(
                    "/agent",
                    {
                        "lane": lane,
                        "attempt": attempt,
                        "task": prompts[f"{lane}/{attempt}"],
                        "extension": call["extension"],
                    },
                )

                async def produce():
                    try:
                        if lane in "ab" and attempt == 1:
                            response = asyncio.create_task(
                                http("/gate", {"lane": lane, "attempt": attempt})
                            )
                            interrupted = asyncio.create_task(cancel.wait())
                            _done, pending = await asyncio.wait(
                                [response, interrupted],
                                return_when=asyncio.FIRST_COMPLETED,
                            )
                            for task in pending:
                                task.cancel()
                        value = (
                            {"event": "cancelled"}
                            if cancel.is_set()
                            else {
                                "event": "final_answer",
                                "data": {
                                    "text": f"{lane.upper()}-canonical-result 中文🧪"
                                },
                            }
                        )
                        await sdk.stream_writer(
                            WireHandle.from_dict(call["stream"])
                        ).complete({"kind": "agent", "value": value})
                    except Exception as error:
                        callback_errors.append(repr(error))
                        raise

                jobs.append(asyncio.create_task(produce()))
                return {"outcome": "stream", "stream": call["stream"]}

            registrations.append(
                await sdk.register_extension(
                    "custom_agent",
                    f"task-{lane}",
                    {
                        "kind": "custom_agent",
                        "descriptor_version": 1,
                        "name": f"task-{lane}",
                        "model_name": "fixture",
                        "system_prompt": "fixture",
                        "tool_names": [],
                    },
                    consume,
                )
            )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        task_run = session.task_run.to_dict()
        canonical_task_run_scope = session.acp_session_id
        tasks = [
            {
                "id": "a",
                "title": "A",
                "description": "controlled A",
                "extension": {"subagent": "task-a"},
            },
            {
                "id": "b",
                "title": "B",
                "description": "independent B",
                "extension": {"subagent": "task-b"},
            },
            {
                "id": "c",
                "title": "C",
                "description": "after B",
                "depends_on": ["b"],
                "extension": {"subagent": "task-c"},
            },
        ]
        if action == "pause":
            tasks.append(
                {
                    "id": "d",
                    "title": "D",
                    "description": "after resume A and B",
                    "depends_on": ["a", "b"],
                    "extension": {"subagent": "task-d"},
                }
            )
        created = await sdk.request(
            "_echo_agent/task/create",
            {
                "task_run": task_run,
                "spec": to_wire({"execution_mode": "parallel", "tasks": tasks}),
            },
        )
        handles = {task["id"]: created["tasks"][i] for i, task in enumerate(tasks)}
        renamed = await sdk.request(
            "_echo_agent/task/update",
            {
                "task_run": task_run,
                "patch": to_wire(
                    {
                        "base_revision": int(created["revision"]),
                        "reason": "fixture rename",
                        "operations": [
                            {
                                "op": "update",
                                "task_id": "a",
                                "patch": {"title": "A canonical v2"},
                            }
                        ],
                    }
                ),
            },
        )
        before = await http("/inspect")
        with pytest.raises(EchoAgentError) as stale:
            await sdk.request(
                "_echo_agent/task/update",
                {
                    "task_run": task_run,
                    "patch": to_wire(
                        {
                            "base_revision": int(created["revision"]),
                            "reason": "stale writer",
                            "operations": [
                                {
                                    "op": "update",
                                    "task_id": "a",
                                    "patch": {"title": "must not commit"},
                                }
                            ],
                        }
                    ),
                },
            )
        assert stale.value.code == "framework_error"
        assert await http("/inspect") == before
        await sdk.request("_echo_agent/task/execute", {"task_run": task_run})

        async def entered():
            return counts["a"] == counts["b"] == 1

        await until(entered)
        with pytest.raises(EchoAgentError) as duplicate:
            await sdk.request("_echo_agent/task/execute", {"task_run": task_run})
        assert (
            duplicate.value.code == "invalid_value"
            and "already executing" in duplicate.value.message
        )
        assert (
            await sdk.request(
                "_echo_agent/task/control",
                {"task_run": task_run, "task": handles["a"], "action": action},
            )
        )["accepted"]
        listed = {}

        async def snapshot():
            return await sdk.request("_echo_agent/task/list", {"task_run": task_run})

        async def controlled():
            nonlocal listed
            listed = await snapshot()
            return any(
                t["task"]["id"] == handles["a"]["id"]
                and t["status"]["status"]
                == ("paused" if action == "pause" else "cancelled")
                for t in listed["tasks"]
            )

        await until(controlled)
        states = {t["task"]["id"]: t["status"]["status"] for t in listed["tasks"]}
        assert (
            states[handles["b"]["id"]] == "running"
            and states[handles["c"]["id"]] == "pending"
        )
        await http("/release", {"lane": "b", "attempt": 1})

        async def siblings():
            nonlocal listed
            listed = await snapshot()
            return all(
                any(
                    t["task"]["id"] == handles[lane]["id"]
                    and t["status"]["status"] == "completed"
                    for t in listed["tasks"]
                )
                for lane in "bc"
            )

        await until(siblings)
        assert (
            "Completed dependency b" in prompts["c/1"]
            and "B-canonical-result" in prompts["c/1"]
        )
        if action == "pause":
            assert (
                await sdk.request(
                    "_echo_agent/task/control",
                    {"task_run": task_run, "task": handles["a"], "action": "resume"},
                )
            )["accepted"]

            async def execute_again():
                try:
                    await sdk.request(
                        "_echo_agent/task/execute", {"task_run": task_run}
                    )
                    return True
                except EchoAgentError as error:
                    if (
                        error.code != "invalid_value"
                        or "already executing" not in error.message
                    ):
                        raise
                    return False

            await until(execute_again)

            async def all_completed():
                nonlocal listed
                listed = await snapshot()
                return all(
                    t["status"]["status"] == "completed" for t in listed["tasks"]
                )

            await until(all_completed)
            assert counts == {"a": 2, "b": 1, "c": 1, "d": 1}
            assert (
                "Completed dependency b" in prompts["d/1"]
                and "B-canonical-result" in prompts["d/1"]
            )
        else:
            assert counts == {"a": 1, "b": 1, "c": 1, "d": 0}
        persisted = await http("/inspect")
        graph = next(iter(persisted["graphs"].values()))
        assert graph["snapshot"]["revision"] == renamed["revision"]
        execution_commits = [
            c
            for c in persisted["commits"]
            if c["operation"] == "revisioned_task_compare_and_commit_execution"
        ]
        running = [
            t["execution"]["claim"]
            for c in execution_commits
            for t in c["after"]["snapshot"]["tasks"]
            if t["execution"]["status"]["status"] == "running"
        ]
        assert all(
            c
            and c["revision"] == renamed["revision"]
            and c["claim_id"]
            and c["spec_hash"]
            for c in running
        )
        completed_b = [
            next(
                t["execution"]
                for t in c["after"]["snapshot"]["tasks"]
                if t["spec"]["id"] == "b"
            )
            for c in execution_commits
            if any(
                t["spec"]["id"] == "b"
                and t["execution"]["status"]["status"] == "completed"
                for t in c["after"]["snapshot"]["tasks"]
            )
        ]
        assert completed_b and all(b == completed_b[0] for b in completed_b)
        assert (
            from_wire(completed_b[0]["result"])["output"] == "B-canonical-result 中文🧪"
        )
        (work / "observed.json").write_text(
            json.dumps(
                {
                    "action": action,
                    "counts": counts,
                    "statuses": [t["status"] for t in listed["tasks"]],
                    "graph": graph,
                    "persisted": persisted,
                },
                ensure_ascii=False,
            )
        )
        await http("/release", {"lane": "a", "attempt": 1})
        await session.close()
        for registration in registrations:
            await registration.close()
        await agent.close()
        await asyncio.gather(*jobs)
        assert not callback_errors, callback_errors
    finally:
        if sdk:
            await sdk.close()
        backend.terminate()
        await backend.wait()


@pytest.mark.asyncio
@pytest.mark.parametrize("kind", ["subagent-await", "crud-only"])
async def test_subagent_receipt_and_crud_admission(kind):
    work = Path(tempfile.mkdtemp(prefix=f"sdk-py-task-{kind}-"))
    print("Task evidence", work)
    backend, http = await launch(work)
    sdk = None
    jobs = []
    try:
        sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
        registrations = []
        invocations = 0
        raw_call = None
        if kind == "crud-only":
            registrations.append(await register_store(sdk, http, False))

        async def consume(call, cancel):
            nonlocal invocations, raw_call
            operation = call["invocation"]["operation"]
            if operation == "agent_close":
                return {
                    "outcome": "result",
                    "result": {"operation": operation, "value": None},
                }
            assert operation in ["agent_execute", "agent_execute_stream"]
            invocations += 1
            raw_call = call
            await http(
                "/agent",
                {
                    "lane": "await",
                    "attempt": 1,
                    "task": call["invocation"]["input"]["task"],
                    "context": call.get("context"),
                    "extension": call["extension"],
                },
            )
            if operation.endswith("_stream"):

                async def produce():
                    await http("/gate", {"lane": "await", "attempt": 1})
                    await sdk.stream_writer(
                        WireHandle.from_dict(call["stream"])
                    ).complete(
                        {
                            "kind": "agent",
                            "value": {
                                "event": "final_answer",
                                "data": {"text": "full retained result 中文🧪"},
                            },
                        }
                    )

                jobs.append(asyncio.create_task(produce()))
                return {"outcome": "stream", "stream": call["stream"]}
            await http("/gate", {"lane": "await", "attempt": 1})
            return {
                "outcome": "result",
                "result": {
                    "operation": operation,
                    "value": "full retained result 中文🧪",
                },
            }

        registrations.append(
            await sdk.register_extension(
                "custom_agent",
                "await-target",
                {
                    "kind": "custom_agent",
                    "descriptor_version": 1,
                    "name": "await-target",
                    "model_name": "fixture",
                    "system_prompt": "fixture",
                    "tool_names": [],
                },
                consume,
            )
        )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        task_run = session.task_run.to_dict()
        if kind == "crud-only":
            await sdk.request(
                "_echo_agent/task/create",
                {
                    "task_run": task_run,
                    "spec": to_wire(
                        {
                            "tasks": [
                                {
                                    "id": "pending",
                                    "title": "pending",
                                    "description": "no user call",
                                    "extension": {"subagent": "await-target"},
                                }
                            ]
                        }
                    ),
                },
            )
            before = await http("/inspect")
            with pytest.raises(EchoAgentError) as error:
                await sdk.request("_echo_agent/task/execute", {"task_run": task_run})
            assert (
                error.value.code == "feature_unavailable"
                and error.value.handle == session.task_run
            )
            assert await http("/inspect") == before and invocations == 0
            assert all(
                t["status"]["status"] == "pending"
                for t in (
                    await sdk.request("_echo_agent/task/list", {"task_run": task_run})
                )["tasks"]
            )
            (work / "observed.json").write_text(
                json.dumps(
                    {
                        "kind": kind,
                        "invocations": invocations,
                        "execution_commits": 0,
                        "error": "feature_unavailable",
                    }
                )
            )
        else:
            payload = {
                "session": session.wire.to_dict(),
                "request": to_wire(
                    {
                        "agent_name": "await-target",
                        "task": "await retained 中文🧪",
                        "mode_override": "sync",
                        "constraints": ["constraint retained 🧪"],
                        "prompt_context": {"acceptance_criteria": ["field preserved"]},
                    }
                ),
                "idempotency_id": "one-receipt",
            }
            one, two = await asyncio.gather(
                sdk.request("_echo_agent/subagent/dispatch", payload),
                sdk.request("_echo_agent/subagent/dispatch", payload),
            )
            assert one["subagent"] == two["subagent"]

            async def entered():
                return invocations == 1

            await until(entered)
            short = await sdk.request(
                "_echo_agent/subagent/await",
                {
                    "subagent": one["subagent"],
                    "timeout": {"seconds": "0", "nanos": 5000000},
                },
            )
            assert short["settled"] is False and "result" not in short
            with pytest.raises(EchoAgentError) as error:
                await sdk.request(
                    "_echo_agent/subagent/dispatch",
                    {
                        **payload,
                        "request": to_wire(
                            {
                                "agent_name": "await-target",
                                "task": "conflicting payload",
                            }
                        ),
                    },
                )
            assert error.value.code == "invalid_value"
            await http("/release", {"lane": "await", "attempt": 1})
            full = await sdk.request(
                "_echo_agent/subagent/await", {"subagent": one["subagent"]}
            )
            repeat = await sdk.request(
                "_echo_agent/subagent/await", {"subagent": one["subagent"]}
            )
            assert full == repeat and full["settled"]
            assert from_wire(full["result"])["output"] == "full retained result 中文🧪"
            assert "constraint retained 🧪" in raw_call["invocation"]["input"]["task"]
            assert (
                await sdk.request(
                    "_echo_agent/subagent/control",
                    {
                        "subagent": one["subagent"],
                        "action": "guidance",
                        "payload": "late",
                    },
                )
            )["accepted"] is False
            assert invocations == 1
            (work / "observed.json").write_text(
                json.dumps(
                    {
                        "kind": kind,
                        "invocations": invocations,
                        "short": short,
                        "full": full,
                        "rawCall": raw_call,
                    },
                    ensure_ascii=False,
                )
            )
        await session.close()
        for registration in registrations:
            await registration.close()
        await agent.close()
        await asyncio.gather(*jobs)
    finally:
        if sdk:
            await sdk.close()
        backend.terminate()
        await backend.wait()
