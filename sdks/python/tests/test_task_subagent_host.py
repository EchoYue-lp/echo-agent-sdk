import asyncio
import copy
import json
import os
import runpy
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


def test_fixture_task_conflict_preserves_classification_without_write(tmp_path):
    state_type = runpy.run_path(FIXTURE)["Fixture"]
    for case, expected, current, execution_drift in (
        ("absent", 1, None, False),
        ("revision", 1, 2, False),
        ("expected_absent", None, 2, False),
        ("execution", 2, 2, True),
    ):
        work = tmp_path / case
        work.mkdir()
        state = state_type(work)
        graph = {
            "snapshot": {
                "revision": current,
                "tasks": [
                    {
                        "spec": {"id": "a"},
                        "execution": {"status": {"status": "pending"}},
                    }
                ],
            }
        }
        # This provider test treats graph content as opaque. The native cases
        # below additionally exercise actual Host-validated graph DTOs.
        if current is not None:
            created = state.store(
                {
                    "operation": "revisioned_task_compare_and_commit",
                    "input": {
                        "scope_id": case,
                        "commit": {"expected_revision": None, "next": graph},
                    },
                }
            )
            assert created["kind"] == "result"
        commit = {"expected_revision": expected, "next": copy.deepcopy(graph)}
        if execution_drift:
            # Both writers observed the same old execution vector. Publish
            # the first writer, then require the second stale one to refuse.
            observed_executions = {
                "a": copy.deepcopy(graph["snapshot"]["tasks"][0]["execution"])
            }
            advanced = copy.deepcopy(graph)
            advanced["snapshot"]["tasks"][0]["execution"]["status"]["status"] = (
                "running"
            )
            first_writer = state.store(
                {
                    "operation": "revisioned_task_compare_and_commit_execution",
                    "input": {
                        "scope_id": case,
                        "commit": {
                            "expected_revision": expected,
                            "expected_executions": observed_executions,
                            "next": advanced,
                        },
                    },
                }
            )
            assert first_writer["kind"] == "result"
            commit["expected_executions"] = observed_executions
        before = state.inspect()
        persisted = work / "task-store.json"
        before_bytes = persisted.read_bytes() if persisted.exists() else None
        rejected = state.store(
            {
                "operation": "revisioned_task_compare_and_commit_execution",
                "input": {"scope_id": case, "commit": commit},
            }
        )
        assert rejected["kind"] == "error"
        error = rejected["error"]
        assert error["code"] == "extension_conflict"
        assert error["details"]["fields"] == [
            {"key": "task_store_error_class", "value": "conflict"},
            {
                "key": "expected_revision",
                "value": "absent" if expected is None else str(expected),
            },
            {
                "key": "current_revision",
                "value": "absent" if current is None else str(current),
            },
        ]
        assert state.inspect() == before
        assert (persisted.read_bytes() if persisted.exists() else None) == before_bytes


async def until(check):
    deadline = asyncio.get_running_loop().time() + 15
    while not await check():
        assert asyncio.get_running_loop().time() < deadline, (
            "framework condition missing"
        )
        await asyncio.sleep(0.01)


async def launch(work, *, observation=False):
    backend = await asyncio.create_subprocess_exec(
        sys.executable,
        FIXTURE,
        "--work",
        str(work),
        *(["--observation"] if observation else []),
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
@pytest.mark.parametrize("action", ["pause", "cancel", "observation"])
async def test_exact_task_control(action):
    work = Path(tempfile.mkdtemp(prefix=f"sdk-py-task-{action}-"))
    print("Task evidence", work)
    backend, http = await launch(work, observation=action == "observation")
    sdk = None
    jobs = []
    facts = {"action": action}
    observed = asyncio.Event()
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
                if action == "observation" and lane == "a":
                    binding = await http("/physical/admit", call)
                    assert binding["context"] == call["context"]
                    assert binding["factory_origin"] is None
                    error = {
                        "code": "framework_error",
                        "retryable": "never",
                        "message": "same Task attempt remains physically held 中文🙂",
                        "operation": "_echo_agent/extension/invoke",
                        "details": {
                            "fields": [
                                {"key": "observation", "value": "not_settled"},
                                {"key": "run_id", "value": binding["run_id"]},
                                {
                                    "key": "execution_id",
                                    "value": binding["execution_id"],
                                },
                            ]
                        },
                    }
                    writer = sdk.stream_writer(WireHandle.from_dict(call["stream"]))

                    async def observed_producer():
                        await writer.failed(error)
                        facts["original"] = {
                            "call": call,
                            "binding": binding,
                            "error": error,
                        }
                        observed.set()
                        facts["joined"] = await http("/physical/join", {})
                        await writer.complete(
                            {
                                "kind": "agent",
                                "value": {
                                    "event": "final_answer",
                                    "data": {"text": "A-after-physical-close 中文🧪"},
                                },
                            }
                        )

                    jobs.append(asyncio.create_task(observed_producer()))
                    return {"outcome": "stream", "stream": call["stream"]}

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
        if action in ("pause", "observation"):
            tasks.append(
                {
                    "id": "d",
                    "title": "D",
                    "description": "after A and B"
                    if action == "observation"
                    else "after resume A and B",
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
        if action == "observation":
            await asyncio.wait_for(observed.wait(), 15)
            await http("/release", {"lane": "b", "attempt": 1})
            listed = {}

            async def b_completed():
                nonlocal listed
                listed = await sdk.request(
                    "_echo_agent/task/list", {"task_run": task_run}
                )
                return any(
                    t["task"]["id"] == handles["b"]["id"]
                    and t["status"]["status"] == "completed"
                    for t in listed["tasks"]
                )

            await until(b_completed)
            facts.update(
                held=await http("/inspect"), held_list=listed, held_counts=counts.copy()
            )
            await http("/physical/release", {})
            await asyncio.wait_for(asyncio.gather(*jobs), 10)

            async def all_formal_completed():
                nonlocal listed
                listed = await sdk.request(
                    "_echo_agent/task/list", {"task_run": task_run}
                )
                return all(
                    t["status"]["status"] == "completed" for t in listed["tasks"]
                )

            await until(all_formal_completed)
            facts.update(
                after=await http("/inspect"),
                counts=counts.copy(),
                prompts=prompts,
                scope=canonical_task_run_scope,
                revision=renamed["revision"],
                listed=listed,
                listed_again=await sdk.request(
                    "_echo_agent/task/list", {"task_run": task_run}
                ),
            )
            (work / "observed.json").write_text(json.dumps(facts, ensure_ascii=False))
            verifier = await asyncio.create_subprocess_exec(
                sys.executable,
                FIXTURE,
                "--work",
                str(work),
                "--verify-formal",
                str(work / "observed.json"),
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
            )
            output, diagnostics = await verifier.communicate()
            assert verifier.returncode == 0, diagnostics.decode()
            print(output.decode())
            await session.close()
            for registration in registrations:
                await registration.close()
            await agent.close()
            return
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
    except Exception as error:
        if action == "observation":
            facts["failure"] = {"name": type(error).__name__, "message": str(error)}
        raise
    finally:
        try:
            if action == "observation":
                await http("/release", {"lane": "b", "attempt": 1})
                await http("/physical/release", {})
                facts["cleanup_join"] = await http("/physical/join", {})
                if jobs:
                    await asyncio.wait_for(asyncio.gather(*jobs), 10)
            if sdk:
                await sdk.close()
        finally:
            try:
                if action == "observation":
                    await http("/shutdown", {})
                else:
                    backend.terminate()
                await asyncio.wait_for(backend.wait(), 5)
                if action == "observation":
                    facts["backend_exit"] = backend.returncode
                    assert backend.returncode == 0
            finally:
                if action == "observation":
                    if backend.returncode is None:
                        backend.kill()
                        await asyncio.wait_for(backend.wait(), 5)
                        facts["forced_backend_cleanup"] = True
                    (work / "observed.json").write_text(
                        json.dumps(facts, ensure_ascii=False)
                    )


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


@pytest.mark.asyncio
@pytest.mark.parametrize("carrier", ["setup", "stream"])
async def test_observation_preserves_real_run_and_same_physical_producer(carrier):
    """The callback admits the owned process; neither transport nor EOF stops it."""
    root = Path(__file__).resolve().parents[3] / "target/cancellation-observation"
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix=f"python-{carrier}-", dir=root))
    print("B observation evidence", work)
    backend = await asyncio.create_subprocess_exec(
        sys.executable, FIXTURE, "--work", str(work), "--observation"
    )
    sdk = None
    jobs = []
    http = None
    facts = {
        "carrier": carrier,
        "source_kind": "issued_stream_with_late_terminal"
        if carrier == "stream"
        else "issued_stream_without_successor_terminal",
        "actual_complete_sent": False,
        "callbacks": [],
        "closes": [],
    }
    try:

        async def ready():
            return (work / "ready").exists()

        await until(ready)
        url = json.loads((work / "fixture.json").read_text())["url"]

        async def request(path, payload=None):
            def send():
                body = None if payload is None else json.dumps(payload).encode()
                req = urllib.request.Request(
                    url + path, data=body, headers={"Content-Type": "application/json"}
                )
                with urllib.request.urlopen(req, timeout=35) as response:
                    return json.load(response)

            return await asyncio.to_thread(send)

        http = request
        sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
        observed = asyncio.Event()
        factory = None

        async def consume(call, _cancel):
            operation = call["invocation"]["operation"]
            facts["callbacks"].append(call)
            await http("/agent", call)
            if operation == "factory_create_agent":
                return {
                    "outcome": "result",
                    "result": {
                        "operation": operation,
                        "value": {
                            "name": "original-physical",
                            "model_name": "fixture",
                            "system_prompt": "",
                            "tool_names": [],
                        },
                    },
                }
            assert call["factory_origin"] == factory.wire.to_dict()
            if operation == "agent_close":
                state = await http("/inspect")
                assert call["extension"] == state["physical"]["binding"]["child"]
                assert state["physical"]["stopped"] is True
                assert state["physical"]["exit_code"] == 0
                facts["closes"].append(call)
                return {
                    "outcome": "result",
                    "result": {"operation": operation, "value": None},
                }
            assert operation == "agent_execute_stream"
            binding = await http("/physical/admit", call)
            facts["binding"] = binding
            assert binding["context"] == call["context"]
            assert call["context"].get("run_id") is None
            assert binding["execution_id"] == call["context"]["execution_id"]
            assert binding["invocation_id"] == call["invocation_id"]
            error = {
                "code": "framework_error",
                "retryable": "never",
                "message": "original physical cleanup remains held 中文🙂",
                "operation": "_echo_agent/extension/invoke",
                "details": {
                    "fields": [
                        {"key": "observation", "value": "not_settled"},
                        {"key": "run_id", "value": binding["run_id"]},
                        {"key": "execution_id", "value": binding["execution_id"]},
                    ]
                },
            }
            facts["error"] = error
            writer = sdk.stream_writer(WireHandle.from_dict(call["stream"]))

            async def produce():
                if carrier == "stream":
                    await writer.failed(error)
                    observed.set()
                joined = await http("/physical/join", {})
                assert joined == {"admitted": True, "exit_code": 0, "stopped": binding}
                facts["joined"] = joined
                if carrier == "stream":
                    await writer.complete(
                        {
                            "kind": "agent",
                            "value": {
                                "event": "final_answer",
                                "data": {
                                    "text": "same physical producer actually exited"
                                },
                            },
                        }
                    )
                    facts["actual_complete_sent"] = True

            job = asyncio.create_task(produce())
            jobs.append(job)
            if carrier == "setup":
                observed.set()
                return {"outcome": "error", "error": error}
            return {"outcome": "stream", "stream": call["stream"]}

        factory = await sdk.register_agent_factory(
            "observation-factory",
            {"kind": "agent_factory", "descriptor_version": 1},
            consume,
        )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        run = await session.start_run("delegate original physical work", execute=True)
        await asyncio.wait_for(observed.wait(), 15)
        waited = await run.wait(timeout=0.25)
        state, replay, held = (
            await run.get(),
            await run.replay(),
            await http("/inspect"),
        )
        facts.update(
            run=run.wire.to_dict(), waited=waited, state=state, replay=replay, held=held
        )
        assert waited == {"settled": False}
        assert (
            state["status"] == "running"
            and "terminal" not in state
            and "receipt" not in state
        )
        assert all(
            row["payload"]["event_type"] not in {"final_answer", "cancelled", "error"}
            for row in replay["events"]
        )
        assert len(held["model_requests"]) == 1 and not facts["closes"]
        assert (
            held["physical"]["alive"] is True and held["physical"]["stopped"] is False
        )
        assert held["physical"]["binding"] == facts["binding"]
        created = [
            call
            for call in facts["callbacks"]
            if call["invocation"]["operation"] == "factory_create_agent"
        ]
        executed = [
            call
            for call in facts["callbacks"]
            if call["invocation"]["operation"] == "agent_execute_stream"
        ]
        assert len(created) == len(executed) == 1
        assert (
            facts["binding"]["child"] == created[0]["invocation"]["input"]["instance"]
        )
        assert facts["binding"]["stream"] == executed[0]["stream"]
        with pytest.raises(EchoAgentError) as collision:
            await session.start_run("must not admit another execution", execute=True)
        assert (
            collision.value.code == "invalid_request"
            and collision.value.retryable == "after_delay"
        )
        assert collision.value.operation == "_echo_agent/run/start"
        facts["held_admission_error"] = {
            "code": collision.value.code,
            "message": collision.value.message,
            "retryable": collision.value.retryable,
            "operation": collision.value.operation,
            "details": collision.value.details,
            "handle": collision.value.handle,
        }
        await http("/physical/release", {})
        await asyncio.wait_for(asyncio.gather(*jobs), 10)
        facts["after"] = await http("/inspect")
        assert facts["after"]["physical"]["exit_code"] == 0
        if carrier == "stream":

            async def child_closed():
                return len(facts["closes"]) == 1

            await until(child_closed)
        # Foreign work cannot settle a Host Run with a different identity.
        assert facts["binding"]["run_id"] != run.wire.id
        one, two = await asyncio.gather(run.wait(timeout=0.25), run.wait(timeout=0.25))
        assert one == two == {"settled": False}
        facts["after_wait"] = one
        facts["after_run"] = await run.get()
        assert facts["after_run"]["status"] == "running"
        assert (
            "terminal" not in facts["after_run"] and "receipt" not in facts["after_run"]
        )
        facts["after_replay"] = await run.replay()
        assert all(
            row["payload"]["event_type"] not in {"final_answer", "cancelled", "error"}
            for row in facts["after_replay"]["events"]
        )
        facts["after"] = await http("/inspect")
        assert facts["actual_complete_sent"] is (carrier == "stream")
        assert len(facts["closes"]) == (1 if carrier == "stream" else 0)
        assert len(facts["after"]["model_requests"]) == 1
        assert (
            sum(
                call["invocation"]["operation"] == "agent_execute_stream"
                for call in facts["callbacks"]
            )
            == 1
        )
    finally:
        try:
            if http is not None:
                await http("/physical/release", {})
                facts["cleanup_join"] = await http("/physical/join", {})
                if jobs:
                    await asyncio.wait_for(asyncio.gather(*jobs), 10)
        finally:
            try:
                if sdk is not None:
                    await sdk.close()
            finally:
                try:
                    if http is not None:
                        await http("/shutdown", {})
                    else:
                        backend.terminate()
                    await asyncio.wait_for(backend.wait(), 5)
                    assert backend.returncode == 0
                finally:
                    if backend.returncode is None:
                        backend.kill()
                        await asyncio.wait_for(backend.wait(), 5)
                        facts["forced_backend_cleanup"] = True
                    (work / "observed.json").write_text(
                        json.dumps(facts, ensure_ascii=False)
                    )
