import asyncio
import json
import os
import sys
import tempfile
import urllib.request
from pathlib import Path

import pytest

from echo_agent_sdk import AgentComponentDescriptor, EchoAgentClient, managed
from echo_agent_sdk.client import AgentComponentOutcome

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required managed Host E2E binary missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="managed real Host requires binary"),
]


def http_json(url, path, payload=None):
    request = urllib.request.Request(
        url + path,
        data=None
        if payload is None
        else json.dumps(payload, ensure_ascii=False).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(request, timeout=40) as response:
        return json.loads(response.read())


async def until_file(path, timeout=5):
    async with asyncio.timeout(timeout):
        while not path.exists():
            await asyncio.sleep(0.01)


async def register_backend(sdk, url):
    async def handler(call, _cancel):
        response = await asyncio.to_thread(
            http_json,
            url,
            "/call",
            {"operation": call.component_operation, "input": call.request.to_wire()},
        )
        if response["kind"] == "error":
            return {"outcome": "error", "error": response["error"]}
        result = response["result"]
        _, field, kind = managed.MANAGED_OPERATIONS[result["operation"]]
        typed = managed.decode(kind, result["value"][field])
        name = (
            "".join(item.title() for item in result["operation"].split("_")) + "Result"
        )
        return AgentComponentOutcome(getattr(managed, name)(typed))

    registrations = []
    for component in ("conversation_store", "runtime_state_store"):
        descriptor = AgentComponentDescriptor(
            component,
            "managed-" + component,
            conversation_projection="atomic_v1"
            if component == "conversation_store"
            else "unsupported",
            runtime_state="revisioned_v1"
            if component == "runtime_state_store"
            else "unsupported",
            persistence_call="absolute_deadline_v1",
        )
        registrations.append(
            await asyncio.wait_for(
                sdk.register_agent_component(
                    "managed-" + component, descriptor, handler
                ),
                timeout=10,
            )
        )
    return registrations


@pytest.mark.parametrize(
    "fault", ["lose_first_apply_response", "expire_before_work_once"]
)
async def test_managed_commit_authority_is_preserved_through_real_host(fault):
    with tempfile.TemporaryDirectory(prefix="sdk-py-managed-") as directory:
        work = Path(directory)
        script = (
            Path(__file__).resolve().parents[2]
            / "shared/testing/managed_persistence_backend.py"
        )
        backend = await asyncio.create_subprocess_exec(
            sys.executable,
            str(script),
            "--work",
            str(work),
            "--fault",
            fault,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )
        try:
            await until_file(work / "ready")
            url = json.loads((work / "backend.json").read_text())["url"]
            sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
            async with sdk:
                registrations = await register_backend(sdk, url)
                agent = await asyncio.wait_for(sdk.create_agent(), 10)
                session = await asyncio.wait_for(agent.create_session(), 10)
                run = await asyncio.wait_for(
                    session.start_run("first managed turn"), 10
                )
                settled = await run.wait(timeout=40)
                first = await asyncio.to_thread(http_json, url, "/inspect")
                if fault == "expire_before_work_once":
                    assert first["counters"]["mutations"] == 0
                    assert not first["receipts"]
                    assert settled["terminal"]["status"] == "failed"
                else:
                    assert first["receipts"]
                    assert first["counters"]["projected_items"] > 0
                    receipts = dict(first["receipts"])
                    replay = await run.replay("0", 512)
                    settlements = [
                        event["payload"]
                        for event in replay["events"]
                        if event["payload"]["event_type"]
                        == "transcript_projection_settlement"
                    ]
                    assert any(
                        managed.decode_settlement_event(event["data"]).error_class
                        == managed.TranscriptProjectionErrorClass.OUTCOME_UNKNOWN
                        for event in settlements
                    )
                    assert any(
                        "pending_transcript_projection"
                        in snapshot["checkpoint"]["messages_json"]
                        for snapshot in first["runtime"].values()
                    )
                    second = await session.start_run("second managed turn")
                    second_settled = await second.wait(timeout=40)
                    assert second_settled["settled"]
                    final = await asyncio.to_thread(http_json, url, "/inspect")
                    for operation, receipt in receipts.items():
                        assert final["receipts"][operation] == receipt
                    assert final["counters"]["duplicate_applies"] > 0
                    await second.close()
                await run.close()
                await session.close()
                for registration in registrations:
                    await registration.close()
                await agent.close()
        finally:
            backend.terminate()
            await asyncio.wait_for(backend.wait(), timeout=5)


async def test_managed_committed_held_response_survives_host_restart_same_receipt():
    from echo_agent_sdk import WireHandle
    from echo_agent_sdk.client import SessionHandle

    with tempfile.TemporaryDirectory(prefix="sdk-py-managed-restart-") as directory:
        work = Path(directory)
        script = (
            Path(__file__).resolve().parents[2]
            / "shared/testing/managed_persistence_backend.py"
        )
        backend = await asyncio.create_subprocess_exec(
            sys.executable,
            str(script),
            "--work",
            str(work),
            "--fault",
            "hold_first_apply_response",
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )
        sdk = None

        async def session_for(client, agent, load):
            result = await client.request(
                "echo_agent/session/load" if load else "echo_agent/session/create",
                {
                    "agent": agent.wire.to_dict(),
                    "session_id": "py-managed-restart",
                    "working_dir": None,
                },
            )
            return SessionHandle(
                client,
                agent.wire,
                WireHandle.from_dict(result["session"]),
                result["acp_session_id"],
                WireHandle.from_dict(result["task_run"]),
            )

        def pending(state):
            return any(
                json.loads(snapshot["checkpoint"]["messages_json"]).get(
                    "pending_transcript_projection"
                )
                for snapshot in state["runtime"].values()
            )

        try:
            await until_file(work / "ready")
            url = json.loads((work / "backend.json").read_text())["url"]
            sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
            await register_backend(sdk, url)
            agent = await sdk.create_agent()
            session = await session_for(sdk, agent, False)
            previous = session.wire
            await session.start_run("commit then disconnect")
            async with asyncio.timeout(35):
                while (await asyncio.to_thread(http_json, url, "/inspect"))[
                    "observations"
                ]["held_responses"] != 1:
                    await asyncio.sleep(0.01)
            committed = await asyncio.to_thread(http_json, url, "/inspect")
            assert committed["receipts"] and pending(committed)
            await sdk.close()
            sdk = None
            await asyncio.to_thread(http_json, url, "/release", {})
            sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
            registrations = await register_backend(sdk, url)
            agent = await sdk.create_agent()
            session = await session_for(sdk, agent, True)
            assert session.wire.generation != previous.generation
            run = await session.start_run("continue after restart")
            assert (await run.wait(timeout=40))["terminal"]["status"] == "completed"
            final = await asyncio.to_thread(http_json, url, "/inspect")
            assert final["counters"]["duplicate_applies"] > 0
            assert not pending(final)
            for operation, receipt in committed["receipts"].items():
                assert final["receipts"][operation] == receipt
            items = 0
            for conversation in final["conversations"].values():
                ordinals = set()
                for item in conversation["messages"]:
                    assert item["ordinal"] not in ordinals
                    ordinals.add(item["ordinal"])
                    items += 1
            assert items == final["counters"]["projected_items"]
            await run.close()
            await session.close()
            for registration in registrations:
                await registration.close()
            await agent.close()
        finally:
            if sdk is not None:
                await sdk.close()
            if backend.returncode is None:
                backend.terminate()
            await backend.wait()
