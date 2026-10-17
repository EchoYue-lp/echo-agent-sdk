import asyncio
import json
import os
import shutil
import sys
import tempfile
import urllib.request
from dataclasses import asdict
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Workflow Host missing")
FIXTURE = (
    Path(__file__).resolve().parents[2]
    / "shared/testing/workflow_checkpoint_fixture.py"
)


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
    output, diagnostics = await process.communicate()
    assert process.returncode == 0, diagnostics.decode()
    return output


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host binary required")
async def test_workflow_checkpoint_branch_owner_and_shape_boundary():
    work = Path(tempfile.mkdtemp(prefix="sdk-py-workflow-"))
    sdk = None
    passed = False
    try:
        await helper(work, "--prepare")
        scenario = json.loads((work / "scenario.json").read_text())
        sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
        agent = await sdk.create_agent()
        sessions = {
            "a": await agent.create_session(),
            "b": await agent.create_session(),
        }
        rows = []
        results = {}
        for step in scenario["steps"]:
            session = sessions[step["session"]]
            if step.get("action") == "close":
                await session.close()
                row = {"id": step["id"], "outcome": "closed"}
                rows.append(row)
                results[step["id"]] = row
                (work / "results.json").write_text(json.dumps(rows, ensure_ascii=False))
                continue
            resolution = work / "resolve.json"
            resolution.write_text(
                json.dumps(
                    {"value": step["arguments"], "results": results}, ensure_ascii=False
                )
            )
            arguments = json.loads(await helper(work, "--resolve", str(resolution)))
            (work / f"call-{step['id']}.json").write_text(
                json.dumps(
                    {"operation": step["operation"], "arguments": arguments},
                    ensure_ascii=False,
                )
            )
            try:
                row = {
                    "id": step["id"],
                    "outcome": "result",
                    "value": await sdk.call(step["operation"], session.wire, arguments),
                }
            except EchoAgentError as error:
                row = {
                    "id": step["id"],
                    "outcome": "error",
                    "code": error.code,
                    "message": str(error),
                    "operation": error.operation,
                    "retryable": error.retryable,
                    "details": error.details,
                }
            rows.append(row)
            results[step["id"]] = row
            (work / "results.json").write_text(json.dumps(rows, ensure_ascii=False))
        await sessions["b"].close()
        await agent.close()
        await sdk.close()
        sdk = None
        path = work / "results.json"
        path.write_text(json.dumps(rows, ensure_ascii=False))
        print(
            "Workflow evidence: " + (await helper(work, "--verify", str(path))).decode()
        )
        passed = True
    finally:
        if sdk is not None:
            await sdk.close()
        if passed and not os.environ.get("ECHO_AGENT_SDK_RETAIN_EVIDENCE"):
            shutil.rmtree(work)
        else:
            print(f"Workflow evidence retained: {work}", file=sys.stderr)


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host binary required")
async def test_protected_workflow_claim_keeps_bound_store_and_original_llm_owner():
    from echo_agent_sdk import AgentComponentDescriptor, LlmClientDescriptor

    root = Path(__file__).resolve().parents[3] / "target/workflow-protected-claims"
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-", dir=root))
    backend = await asyncio.create_subprocess_exec(
        sys.executable, str(FIXTURE), "--work", str(work), "--serve-protected"
    )
    sdk = None
    http = None
    rows, results, flights = [], {}, {}
    verified = False
    try:
        deadline = asyncio.get_running_loop().time() + 10
        while not (work / "protected-ready.json").exists():
            assert asyncio.get_running_loop().time() < deadline, (
                "owned Workflow fixture not ready"
            )
            await asyncio.sleep(0.01)
        url = json.loads((work / "protected-ready.json").read_text())["url"]

        async def request(path, value=None):
            def send():
                payload = (
                    None
                    if value is None
                    else json.dumps(value, ensure_ascii=False).encode()
                )
                req = urllib.request.Request(
                    url + path,
                    data=payload,
                    headers={"Content-Type": "application/json"},
                )
                with urllib.request.urlopen(req, timeout=15) as response:
                    assert response.status == 200
                    return json.load(response)

            return await asyncio.to_thread(send)

        http = request
        sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))

        async def llm(call, cancellation):
            assert call.extension is not None and call.invocation_id
            raw = {
                "extension": call.extension.to_dict(),
                "invocation_id": call.invocation_id,
                "deadline": call.deadline,
                "invocation": {
                    "operation": call.operation,
                    "input": dict(call.request),
                },
            }
            response = await http("/llm", raw)
            return (
                {"outcome": "error", "error": response["error"]}
                if "error" in response
                else response["response"]
            )

        await sdk.register_llm_client(
            "protected-llm", LlmClientDescriptor("protected-fixture", False), llm
        )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        scenario = json.loads((work / "protected-scenario.json").read_text())

        async def run(step, arguments):
            try:
                return {
                    "id": step["id"],
                    "outcome": "result",
                    "value": await sdk.call(step["operation"], session.wire, arguments),
                }
            except EchoAgentError as error:
                return {
                    "id": step["id"],
                    "outcome": "error",
                    "code": error.code,
                    "message": str(error),
                    "operation": error.operation,
                    "retryable": error.retryable,
                    "handle": None if error.handle is None else error.handle.to_dict(),
                    "details": error.details,
                }

        for step in scenario["steps"]:
            action = step.get("action")
            if action == "bind_store":
                store = step["store"]

                async def handler(call, cancellation, store=store):
                    assert call.extension is not None and call.invocation_id
                    arguments = asdict(call.request)
                    operation = arguments.pop("operation")
                    raw = {
                        "extension": call.extension.to_dict(),
                        "invocation_id": call.invocation_id,
                        "deadline": call.deadline,
                        "invocation": {
                            "operation": call.operation,
                            "input": {
                                "component": call.component,
                                "call": {"operation": operation, "input": arguments},
                            },
                        },
                    }
                    reply = await http("/store/" + store, raw)
                    if "error" in reply:
                        return {"outcome": "error", "error": reply["error"]}
                    return {
                        "outcome": "result",
                        "result": {
                            "operation": "agent_component_call",
                            "value": {
                                "component": "workflow_checkpoint_store",
                                "result": reply,
                            },
                        },
                    }

                registration = await sdk.register_agent_component(
                    "protected-" + store,
                    AgentComponentDescriptor(
                        "workflow_checkpoint_store",
                        store,
                        claim_heartbeat_interval_ms=1000,
                        protected_claim_v1=step["supported"],
                    ),
                    handler,
                )
                row = {
                    "id": step["id"],
                    "outcome": "registered",
                    "wire": registration.wire.to_dict(),
                }
            elif action in {
                "inspect",
                "wait_protect",
                "release_protect",
                "release_join",
            }:
                path = {
                    "inspect": "/inspect",
                    "wait_protect": "/wait_protect",
                    "release_protect": "/release_protect",
                    "release_join": "/release_join",
                }[action]
                row = {
                    "id": step["id"],
                    "outcome": "observed",
                    "value": await http(path, None if action == "inspect" else {}),
                }
            elif action == "await_call":
                row = {**await flights[step["flight"]], "id": step["id"]}
            else:
                resolution = work / "protected-resolve.json"
                resolution.write_text(
                    json.dumps(
                        {"value": step["arguments"], "results": results},
                        ensure_ascii=False,
                    )
                )
                arguments = json.loads(await helper(work, "--resolve", str(resolution)))
                if action == "start_call":
                    flights[step["id"]] = asyncio.create_task(run(step, arguments))
                    row = {"id": step["id"], "outcome": "started"}
                else:
                    row = await run(step, arguments)
            rows.append(row)
            results[step["id"]] = row
            (work / "protected-results.json").write_text(
                json.dumps(rows, ensure_ascii=False, indent=2)
            )
        print(
            (
                await helper(
                    work, "--verify-protected", str(work / "protected-results.json")
                )
            ).decode()
        )
        verified = True
    finally:
        try:
            if http is not None:
                await http("/release_join", {})
                await asyncio.gather(*flights.values())
        finally:
            try:
                if sdk is not None:
                    if verified:
                        child = sdk.process
                        binding = results["while-physical-held"]["value"]["physical"][
                            "binding"
                        ]
                        address = sdk._callbacks.invocation_cancel.get(
                            binding["invocation_id"]
                        )
                        assert address is not None and not address.is_set()
                    await sdk.close()
                    if verified:
                        assert sdk.process is child and child.returncode is not None
                        after = await http("/inspect")
                        before = results["after-external-join"]["value"]
                        for key in ("stores", "llm_calls", "calls"):
                            assert after[key] == before[key], key
                        (work / "python-close-facts.json").write_text(
                            json.dumps(
                                {
                                    "original_invocation_id": binding["invocation_id"],
                                    "original_cancel_address_retained_before_close": True,
                                    "host_pid": child.pid,
                                    "host_returncode": child.returncode,
                                    "local_transport_cleanup_returned": True,
                                    "framework_receipt_proved": False,
                                    "protected_attempt_unchanged": True,
                                }
                            )
                        )
                        (work / "python-after-close-backend.json").write_text(
                            json.dumps(after)
                        )
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
                        await backend.wait()
                    print("Protected Workflow evidence: " + str(work))
