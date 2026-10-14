"""Accepted exact-clear consumer, not direct reverse-callback DTO invocation."""

import asyncio
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest
from echo_agent_sdk import EchoAgentClient, EchoAgentError, WireHandle
from echo_agent_sdk.client import SessionHandle
from test_managed_host import HOST, http_json, register_backend, until_file

pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]
CLEAR = "echo_agent::state::clear_persisted_runtime_incarnation"
DELETE = "echo_agent::state::delete_persisted_conversation"
RETIRE = "runtime_retire_generation_with_context"


def durable(state):
    return {
        key: value
        for key, value in state.items()
        if key not in ("observations", "supported_operations", "calls")
    }


def failure(error):
    return {
        "code": error.code,
        "operation": error.operation,
        "message": str(error),
        "details": error.details,
    }


@pytest.mark.parametrize("enabled", [False, True])
async def test_accepted_exact_clear_retains_scope_projection_and_retirement_proof(
    enabled,
):
    root = Path(
        os.environ.get(
            "ECHO_AGENT_SDK_RETIREMENT_RUNTIME_ROOT",
            str(Path(__file__).resolve().parents[3] / "target/managed-retirement"),
        )
    )
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix=f"python-retirement-{enabled}-", dir=root))
    script = (
        Path(__file__).resolve().parents[2]
        / "shared/testing/managed_persistence_backend.py"
    )
    backend = await asyncio.create_subprocess_exec(
        sys.executable,
        str(script),
        "--work",
        str(work),
        *(["--enable-retirement"] if enabled else []),
        stdout=asyncio.subprocess.DEVNULL,
        stderr=asyncio.subprocess.PIPE,
    )
    proof = {"enabled": enabled, "errors": []}
    try:
        await until_file(work / "ready")
        url = json.loads((work / "backend.json").read_text())["url"]
        async with await EchoAgentClient.spawn(
            HOST, "--config", str(work / "host.json")
        ) as sdk:
            registrations = await register_backend(sdk, url)
            agent = await sdk.create_agent()
            session = await agent.create_session()
            run = await session.start_run("managed retirement 中文🙂")
            assert (await run.wait(timeout=40))["terminal"]["status"] == "completed"
            before = await asyncio.to_thread(http_json, url, "/inspect")
            assert len(before["runtime"]) == 1
            identity, snapshot = next(iter(before["runtime"].items()))
            scope_id = snapshot["scope"]["scope_id"]
            assert snapshot["checkpoint"] is not None
            assert not json.loads(snapshot["checkpoint"]["messages_json"]).get(
                "pending_transcript_projection"
            )
            assert before["receipts"] and before["counters"]["projected_items"] > 0
            proof["before"] = before
            with pytest.raises(EchoAgentError) as rejected:
                await session.invoke(DELETE, [scope_id])
            error = rejected.value
            assert error.code == "framework_error" and error.operation == DELETE
            assert "managed conversation deletion requires" in str(error)
            assert any(
                item["key"] == "framework_failure_code"
                and item["value"] == "runtime_state"
                for item in error.details["fields"]
            )
            proof["errors"].append(failure(error))
            after_delete = await asyncio.to_thread(http_json, url, "/inspect")
            assert durable(after_delete) == durable(before)
            assert after_delete["calls"] == before["calls"]
            if not enabled:
                with pytest.raises(EchoAgentError) as rejected:
                    await session.invoke(CLEAR, [scope_id, identity])
                assert rejected.value.operation == CLEAR
                assert "does not implement " + RETIRE in str(rejected.value)
                proof["errors"].append(failure(rejected.value))
                after = await asyncio.to_thread(http_json, url, "/inspect")
                assert durable(after) == durable(before)
                assert RETIRE not in after["supported_operations"]
            else:
                cleared = await session.invoke(CLEAR, [scope_id, identity])
                assert cleared == {
                    "scope_id": scope_id,
                    "runtime_state_id": identity,
                    "checkpoint_removed": True,
                }
                after = await asyncio.to_thread(http_json, url, "/inspect")
                retired = after["runtime"][identity]
                assert retired["checkpoint"] is None
                assert retired["runtime_state_id"] == identity
                assert retired["scope"] == after["scopes"][scope_id]
                assert retired["scope"] == {
                    **snapshot["scope"],
                    "revision": str(int(snapshot["scope"]["revision"]) + 1),
                }
                assert retired["scope"]["lifecycle"] == "active"
                assert after["conversations"] == before["conversations"]
                assert (
                    after["receipts"] == before["receipts"]
                    and after["batches"] == before["batches"]
                )
                assert after["counters"] == {
                    **before["counters"],
                    "mutations": before["counters"]["mutations"] + 1,
                }
                request = after["cas_proofs"][identity]["request"]
                assert (
                    request["scope_id"] == scope_id
                    and request["runtime_state_id"] == identity
                )
                assert (
                    request["expected_scope_revision"] == snapshot["scope"]["revision"]
                )
                assert request["expected_state_version"] == snapshot["version"]
                assert retired["version"] == {
                    "retired": {
                        "revision": str(
                            int(snapshot["version"]["managed"]["revision"]) + 1
                        ),
                        "operation_id": request["operation_id"],
                    }
                }
                proof["clear_receipt"] = cleared
                replayed = await session.invoke(CLEAR, [scope_id, identity])
                assert replayed == {**cleared, "checkpoint_removed": False}
                replay_state = await asyncio.to_thread(http_json, url, "/inspect")
                assert durable(replay_state) == durable(after)
                proof["replayed_clear_receipt"] = replayed
            retire_calls = [
                call for call in after["calls"] if call["operation"] == RETIRE
            ]
            assert len(retire_calls) == 1
            assert retire_calls[0]["input"]["request"]["runtime_state_id"] == identity
            deadline = retire_calls[0]["input"]["context"]["absolute_deadline_unix_ms"]
            assert isinstance(deadline, str) and int(deadline) > 0
            proof["after"] = after
            await run.close()
            previous_generation = session.wire.generation
            await session.close()
            for registration in registrations:
                await registration.close()
            await agent.close()
            if enabled:
                await sdk.close()
                async with await EchoAgentClient.spawn(
                    HOST, "--config", str(work / "host.json")
                ) as restarted:
                    registrations = await register_backend(restarted, url)
                    agent = await restarted.create_agent()
                    response = await restarted.request(
                        "_echo_agent/session/load",
                        {
                            "agent": agent.wire.to_dict(),
                            "session_id": scope_id,
                            "working_dir": None,
                        },
                    )
                    session = SessionHandle(
                        restarted,
                        agent.wire,
                        WireHandle.from_dict(response["session"]),
                        response["acp_session_id"],
                        WireHandle.from_dict(response["task_run"]),
                    )
                    assert session.wire.generation != previous_generation
                    assert await session.invoke(CLEAR, [scope_id, identity]) == {
                        **proof["clear_receipt"],
                        "checkpoint_removed": False,
                    }
                    recovered = await asyncio.to_thread(http_json, url, "/inspect")
                    assert durable(recovered) == durable(after)
                    assert (
                        len(
                            [
                                call
                                for call in recovered["calls"]
                                if call["operation"] == RETIRE
                            ]
                        )
                        == 1
                    )
                    proof["after_host_restart"] = recovered
                    proof["host_generations"] = [
                        previous_generation,
                        session.wire.generation,
                    ]
                    await session.close()
                    for registration in registrations:
                        await registration.close()
                    await agent.close()
    finally:
        backend.terminate()
        await asyncio.wait_for(backend.wait(), 5)
        (work / "python-observed.json").write_text(
            json.dumps(proof, ensure_ascii=False, indent=2)
        )
        print(f"Managed retirement evidence: {work}")
