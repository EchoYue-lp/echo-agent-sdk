"""Native public SDK drives actual LLM callbacks and framework Workflow streams."""

import asyncio
import copy
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest
from echo_agent_sdk import (
    AgentComponentCall,
    AgentComponentDescriptor,
    EchoAgentClient,
    EchoAgentError,
    LlmClientDescriptor,
    WireHandle,
)

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required LLM/Workflow Host missing")
FIXTURE = (
    Path(__file__).resolve().parents[2]
    / "shared/testing/llm_workflow_stream_fixture.py"
)
ROOT = Path(
    os.environ.get(
        "ECHO_AGENT_SDK_LLM_WORKFLOW_ROOT",
        str(Path(__file__).resolve().parents[3] / "target/llm-workflow-stream"),
    )
)
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]


async def helper(work, *args):
    process = await asyncio.create_subprocess_exec(
        sys.executable,
        str(FIXTURE),
        "--work",
        str(work),
        *args,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )
    output, error = await process.communicate()
    assert process.returncode == 0, error.decode()
    return output.decode()


async def setup(group):
    ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix=f"python-{group}-", dir=ROOT))
    await helper(work, "--prepare")
    spec = json.loads((work / "scenario.json").read_text())
    sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
    return work, spec, sdk


def error_fact(error):
    return {
        "code": error.code,
        "message": str(error),
        "operation": error.operation,
        "details": error.details,
    }


async def observed_error(awaitable):
    with pytest.raises(EchoAgentError) as failure:
        await awaitable
    return error_fact(failure.value)


async def collect(session, stream, bound=12):
    events = []
    for _ in range(bound):
        value = await asyncio.wait_for(
            session.invoke("workflow.stream.next", [stream]), 10
        )
        events.append(value)
        variant = value.get("value", value).get("variant")
        if variant != "item":
            return events
    raise AssertionError("workflow did not reach bounded terminal")


async def test_llm_real_registered_provider_full_requests_usage_failure_and_cancel():
    work, spec, sdk = await setup("llm")
    rows = []
    try:
        for mode in ("nonstream", "stream", "missing_finish", "error", "cancel"):
            row = {
                "mode": mode,
                "calls": [],
                "finally": 0,
                "cancel_observed": False,
                "sent": [],
            }
            rows.append(row)
            started = asyncio.Event()
            producers = []

            async def handler(
                call,
                cancellation,
                *,
                row=row,
                mode=mode,
                started=started,
                producers=producers,
            ):
                row["calls"].append(
                    {
                        "operation": call.operation,
                        "request": dict(call.request),
                        "invocation_id": call.invocation_id,
                        "deadline": call.deadline,
                    }
                )
                started.set()
                try:
                    if mode == "error":
                        raise RuntimeError(spec["failure"]["message"])
                    if mode == "cancel":
                        await asyncio.wait_for(cancellation.wait(), 10)
                        row["cancel_observed"] = cancellation.is_set()
                    if mode == "stream":
                        assert call.stream is not None
                        writer = sdk.stream_writer(call.stream)

                        async def produce():
                            await asyncio.sleep(0)
                            for sequence, chunk in enumerate(spec["llm_chunks"], 1):
                                await writer.chunk(chunk)
                                row["sent"].append(
                                    {
                                        "event": "chunk",
                                        "sequence": str(sequence),
                                        "value": chunk,
                                    }
                                )
                            await writer.complete(spec["llm_complete"])
                            row["sent"].append(
                                {
                                    "event": "complete",
                                    "sequence": "3",
                                    "value": spec["llm_complete"],
                                }
                            )

                        producers.append(asyncio.create_task(produce()))
                        return {"outcome": "stream", "stream": call.stream.to_dict()}
                    response = copy.deepcopy(spec["response"])
                    if mode == "missing_finish":
                        response.pop("finish_reason")
                    return response
                finally:
                    row["finally"] += 1

            registration = await sdk.register_llm_client(
                "proof-" + mode,
                LlmClientDescriptor("fixture-model", mode == "stream"),
                handler,
            )
            agent = await sdk.create_agent()
            session = await agent.create_session()
            await session.invoke(
                "echo_agent::agent::react::ReactAgent::set_temperature",
                [spec["temperature"]],
            )
            await session.invoke(
                "echo_agent::agent::react::ReactAgent::set_max_tokens",
                [spec["max_tokens"]],
            )
            run = await session.start_run(spec["prompt"])
            if mode == "cancel":
                await asyncio.wait_for(started.wait(), 5)
                await run.cancel()
            row["settled"] = await run.wait(timeout=15)
            row["usage"] = await run.usage()
            row["replay"] = await run.replay("0", 512)
            await asyncio.gather(*producers)
            if mode == "nonstream":
                history = await session.start_run(spec["history_prompt"])
                row["history_settled"] = await history.wait(timeout=15)
                await history.close()
            await run.close()
            await session.close()
            await agent.close()
            await registration.close()
    finally:
        await sdk.close()
        (work / "observed.json").write_text(
            json.dumps(rows, ensure_ascii=False, indent=2)
        )
        print("LLM/Workflow evidence: " + str(work))
    print(await helper(work, "--verify", "llm"))


async def test_workflow_extension_run_stream_error_cancel_close_and_owner():
    work, spec, sdk = await setup("workflow")
    facts = {
        "calls": [],
        "producer_completed": 0,
        "producer_errors": [],
        "callbacks_completed": 0,
    }
    releases = {}
    started = {}
    producers = []
    try:

        async def handler(call, cancellation):
            mode = call.request.input
            facts["calls"].append(
                {
                    "operation": call.operation,
                    "component_operation": call.component_operation,
                    "input": mode,
                    "extension": call.extension.to_dict() if call.extension else None,
                    "invocation_id": call.invocation_id,
                    "deadline": call.deadline,
                }
            )
            if call.component_operation == "workflow_run":
                assert call.stream is None
                return {
                    "outcome": "result",
                    "result": {
                        "operation": "agent_component_call",
                        "value": {
                            "component": "workflow",
                            "result": {
                                "operation": "workflow_run",
                                "value": {"output": spec["workflow_output"]},
                            },
                        },
                    },
                }
            started[mode].set()
            stream = call.stream
            assert stream is not None, "typed component dropped Host-issued stream"
            facts["calls"][-1]["stream"] = stream.to_dict()
            assert (
                stream.kind == "stream"
                and stream.generation == call.extension.generation
            )
            writer = sdk.stream_writer(stream)

            async def produce():
                try:
                    if mode in releases:
                        await releases[mode].wait()
                        return
                    chunks = (
                        [spec["workflow_error_chunk"]]
                        if mode == "failure"
                        else spec["workflow_chunks"]
                    )
                    for chunk in chunks:
                        await writer.chunk(chunk)
                    if mode == "failure":
                        await writer.failed(spec["failure"])
                    else:
                        await writer.complete(spec["workflow_complete"])
                except Exception as error:
                    facts["producer_errors"].append(str(error))
                    raise
                finally:
                    facts["producer_completed"] += 1

            if mode in releases:
                # No independent producer is created for parked sources. The
                # callback itself remains owned until explicitly released.
                await releases[mode].wait()
            else:
                producers.append(asyncio.create_task(produce()))
            return {"outcome": "stream", "stream": stream.to_dict()}

        descriptor = AgentComponentDescriptor(
            "workflow", "proof-workflow", supports_streaming=True
        )

        async def owned_handler(call, cancellation):
            try:
                return await handler(call, cancellation)
            finally:
                facts["callbacks_completed"] += 1

        registration = await sdk.register_agent_component(
            "proof-workflow", descriptor, owned_handler
        )
        agent = await sdk.create_agent()
        a = await agent.create_session()
        b = await agent.create_session()
        facts["run"] = await a.invoke(
            "workflow.extension.run", [registration.wire, "input🙂"]
        )
        for mode in ("success", "failure"):
            started[mode] = asyncio.Event()
            stream = await a.invoke(
                "workflow.extension.run_stream", [registration.wire, mode]
            )
            if mode == "success":
                await asyncio.wait_for(started[mode].wait(), 5)
                facts["callback_count_before_wrong_owner"] = len(facts["calls"])
                facts["wrong_owner"] = await observed_error(
                    b.invoke("workflow.stream.next", [stream])
                )
                facts["callback_count_after_wrong_owner"] = len(facts["calls"])
            facts[mode] = await collect(a, stream)
        for mode in ("cancel", "close"):
            releases[mode] = asyncio.Event()
            started[mode] = asyncio.Event()
            stream = await a.invoke(
                "workflow.extension.run_stream", [registration.wire, mode]
            )
            await asyncio.wait_for(started[mode].wait(), 5)
            if mode == "cancel":
                facts["cancel"] = await a.invoke("workflow.stream.cancel", [stream])
                releases[mode].set()
                facts["cancel_terminal"] = await a.invoke(
                    "workflow.stream.next", [stream]
                )
            else:
                closing = asyncio.create_task(
                    a.invoke("workflow.stream.close", [stream])
                )
                await asyncio.sleep(0)
                releases[mode].set()
                facts["close"] = await closing
                facts["close_again"] = await a.invoke("workflow.stream.close", [stream])
                facts["after_close"] = await observed_error(
                    a.invoke("workflow.stream.next", [stream])
                )
        await asyncio.gather(*producers)
        await a.close()
        await b.close()
        await agent.close()
        await registration.close()
    finally:
        for release in releases.values():
            release.set()
        await sdk.close()
        (work / "observed.json").write_text(
            json.dumps(facts, ensure_ascii=False, indent=2)
        )
        print("LLM/Workflow evidence: " + str(work))
    print(await helper(work, "--verify", "workflow"))


async def test_workflow_pure_graph_stream_owner_cancel_without_callback_execution():
    work, spec, sdk = await setup("graph")
    facts = {"callback_count": 0}
    try:

        async def forbidden_model(_call, _cancellation):
            facts["callback_count"] += 1
            raise RuntimeError("pure router graph must not invoke model")

        guard = await sdk.register_llm_client(
            "graph-model-guard",
            LlmClientDescriptor("fixture-model", False),
            forbidden_model,
        )
        agent = await sdk.create_agent()
        a = await agent.create_session()
        b = await agent.create_session()
        graph = (
            await a.invoke(
                "workflow.graph.build", [json.dumps(spec["graph"], ensure_ascii=False)]
            )
        )["resource"]
        facts["wrong_owner"] = await observed_error(
            b.invoke("workflow.graph.run_stream", [graph, spec["state"]])
        )
        facts["wrong_cancel"] = await observed_error(
            b.invoke("workflow.graph.cancel", [graph])
        )
        stream = await a.invoke("workflow.graph.run_stream", [graph, spec["state"]])
        facts["events"] = await collect(a, stream)
        facts["cancel"] = await a.invoke("workflow.graph.cancel", [graph])
        stream = await a.invoke("workflow.graph.run_stream", [graph, spec["state"]])
        facts["cancelled_events"] = await collect(a, stream)
        await a.close()
        await b.close()
        await agent.close()
        await guard.close()
    finally:
        await sdk.close()
        (work / "observed.json").write_text(
            json.dumps(facts, ensure_ascii=False, indent=2)
        )
        print("LLM/Workflow evidence: " + str(work))
    print(await helper(work, "--verify", "graph"))


async def test_component_stream_projection_preserves_optional_handle_and_exact_generation():
    payload = {
        "extension": {
            "id": "extension🙂",
            "kind": "extension",
            "generation": "9007199254740993",
        },
        "invocation_id": "invoke🙂",
        "deadline": {"seconds": "1", "nanos": 123},
        "invocation": {
            "operation": "agent_component_call_stream",
            "input": {
                "component": "workflow",
                "call": {
                    "operation": "workflow_run_stream",
                    "input": {"input": "中文🙂"},
                },
            },
        },
    }
    for optional in ({}, {"stream": None}):
        call = AgentComponentCall.from_payload({**payload, **optional})
        assert call.stream is None
        assert call.request.input == "中文🙂"
    stream = {"id": "stream🙂", "kind": "stream", "generation": "9007199254740993"}
    call = AgentComponentCall.from_payload({**payload, "stream": stream})
    assert call.stream == WireHandle.from_dict(stream)
    assert call.stream.to_dict() == stream
    assert call.extension.to_dict() == payload["extension"]
    assert (
        call.invocation_id == payload["invocation_id"]
        and call.deadline == payload["deadline"]
    )


async def test_component_stream_projection_rejects_malformed_handle_without_lossy_default():
    payload = {
        "invocation": {
            "operation": "agent_component_call_stream",
            "input": {
                "component": "workflow",
                "call": {
                    "operation": "workflow_run_stream",
                    "input": {"input": "中文🙂"},
                },
            },
        }
    }
    for stream in (
        {"id": "stream", "kind": "stream", "generation": "01"},
        {"id": "stream", "kind": "stream", "generation": "18446744073709551616"},
    ):
        with pytest.raises((ValueError, OverflowError)):
            AgentComponentCall.from_payload({**payload, "stream": stream})
