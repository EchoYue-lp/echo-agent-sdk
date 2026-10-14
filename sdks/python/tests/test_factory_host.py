import json
import os
import sys
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, wire_utf8_path

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Factory Host E2E binary missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host binary required"),
]


async def test_factory_children_keep_exact_constructed_objects_through_host_evaluation():
    with tempfile.TemporaryDirectory(prefix="sdk-py-factory-") as work:
        sdk = await EchoAgentClient.spawn(
            sys.executable,
            str(Path(__file__).with_name("protocol_host_proxy.py")),
            "--host",
            HOST,
            "--work",
            work,
            "--mode",
            "normal",
        )
        async with sdk:
            instances = {}
            created = []
            executed = []
            closed = []
            import asyncio

            concurrent_closed = asyncio.Event()
            factory = None

            async def handler(call, _cancel):
                operation = call["invocation"]["operation"]
                if operation == "factory_create_agent":
                    assert call["extension"] == factory.wire.to_dict()
                    instance = call["invocation"]["input"]["instance"]
                    assert instance["generation"] == factory.wire.generation
                    assert instance["id"] not in instances
                    instances[instance["id"]] = f"SDK-object-{len(created)}"
                    created.append(instance)
                    return {
                        "outcome": "result",
                        "result": {
                            "operation": operation,
                            "value": {
                                "name": "same",
                                "model_name": "same",
                                "system_prompt": "same",
                                "tool_names": [],
                            },
                        },
                    }
                assert call["factory_origin"] == factory.wire.to_dict()
                instance = call["extension"]
                assert instance["id"] in instances
                if operation == "agent_close":
                    closed.append(instance["id"])
                    instances.pop(instance["id"])
                    if len(created) > 2 and created[2]["id"] == instance["id"]:
                        concurrent_closed.set()
                    return {
                        "outcome": "result",
                        "result": {"operation": operation, "value": None},
                    }
                assert operation in {
                    "agent_execute",
                    "agent_chat",
                    "agent_execute_stream",
                    "agent_chat_stream",
                }
                executed.append(instance["id"])
                if operation.endswith("_stream"):
                    import asyncio

                    from echo_agent_sdk import WireHandle

                    async def complete():
                        await asyncio.sleep(0)
                        if len(created) > 3 and created[3]["id"] == instance["id"]:
                            await concurrent_closed.wait()
                            assert instance["id"] in instances
                        await sdk.stream_writer(
                            WireHandle.from_dict(call["stream"])
                        ).complete(
                            {
                                "kind": "agent",
                                "value": {
                                    "event": "final_answer",
                                    "data": {"text": instances[instance["id"]]},
                                },
                            }
                        )

                    asyncio.create_task(complete())
                    return {"outcome": "stream", "stream": call["stream"]}
                return {
                    "outcome": "result",
                    "result": {
                        "operation": operation,
                        "value": instances[instance["id"]],
                    },
                }

            factory = await sdk.register_agent_factory(
                "factory-owned",
                {"kind": "agent_factory", "descriptor_version": 1},
                handler,
            )
            agent = await sdk.create_agent()
            session = await agent.create_session()
            runner = await sdk.call(
                "echo_agent::eval::runner::EvalRunner::new",
                agent.wire,
                [session.wire, wire_utf8_path(str(Path(work) / "eval-one"))],
            )
            cases = [
                {
                    "id": name,
                    "name": name,
                    "description": "same descriptor distinct state",
                    "domain": None,
                    "task": "return SDK",
                    "project_fixture": None,
                    "success_criteria": {"type": "output_contains", "substring": "SDK"},
                    "constraints": {},
                }
                for name in ("one", "two")
            ]
            await sdk.call(
                "echo_agent::eval::runner::EvalRunner::run_all",
                agent.wire,
                [session.wire, runner, cases, factory.wire],
            )
            other = await agent.create_session()
            other_runner = await sdk.call(
                "echo_agent::eval::runner::EvalRunner::new",
                agent.wire,
                [other.wire, wire_utf8_path(str(Path(work) / "eval-two"))],
            )
            await asyncio.gather(
                sdk.call(
                    "echo_agent::eval::runner::EvalRunner::run_all",
                    agent.wire,
                    [
                        session.wire,
                        runner,
                        [{**cases[0], "id": "parallel-one"}],
                        factory.wire,
                    ],
                ),
                sdk.call(
                    "echo_agent::eval::runner::EvalRunner::run_all",
                    agent.wire,
                    [
                        other.wire,
                        other_runner,
                        [{**cases[1], "id": "parallel-two"}],
                        factory.wire,
                    ],
                ),
            )
            await sdk.call(
                "echo_agent::eval::runner::EvalRunner::run_all",
                agent.wire,
                [
                    session.wire,
                    runner,
                    [{**cases[0], "id": "parent-still-live"}],
                    factory.wire,
                ],
            )
            assert len(created) == 5
            assert len({item["id"] for item in created}) == 5
            assert set(executed) == {item["id"] for item in created}
            assert set(closed) == {item["id"] for item in created}
            assert not instances
            await factory.close()
            await other.close()
            await session.close()
            await agent.close()


async def test_late_factory_construction_remains_owned_until_reserved_child_cleanup():
    import asyncio

    from echo_agent_sdk import EchoAgentError

    with tempfile.TemporaryDirectory(prefix="sdk-py-factory-late-") as directory:
        work = Path(directory)
        sdk = await EchoAgentClient.spawn(
            sys.executable,
            str(Path(__file__).with_name("protocol_host_proxy.py")),
            "--host",
            HOST,
            "--work",
            str(work),
            "--mode",
            "normal",
        )
        release = asyncio.Event()
        constructing = asyncio.Event()
        observations = {"created": 0, "closed": 0}
        cancellation = []
        identity = []

        async def until(predicate):
            async with asyncio.timeout(15):
                while not predicate():
                    await asyncio.sleep(0.01)

        async with sdk:

            async def handler(call, cancel):
                operation = call["invocation"]["operation"]
                if operation == "factory_create_agent":
                    identity.append(
                        (call["extension"], call["invocation"]["input"]["instance"])
                    )
                    cancellation.append(cancel)
                    constructing.set()
                    await release.wait()
                    observations["created"] += 1
                    return {
                        "outcome": "result",
                        "result": {
                            "operation": operation,
                            "value": {
                                "name": "late",
                                "model_name": "late",
                                "system_prompt": "late",
                                "tool_names": [],
                            },
                        },
                    }
                assert operation == "agent_close"
                parent, child = identity[0]
                assert call["factory_origin"] == parent and call["extension"] == child
                assert observations["created"] == 1
                observations["closed"] += 1
                return {
                    "outcome": "result",
                    "result": {"operation": operation, "value": None},
                }

            factory = await sdk.register_extension(
                "agent_factory",
                "late-factory",
                {"kind": "agent_factory", "descriptor_version": 1},
                handler,
                timeout={"seconds": "0", "nanos": 100_000_000},
            )
            agent = await sdk.create_agent()
            session = await agent.create_session()
            runner = await sdk.call(
                "echo_agent::eval::runner::EvalRunner::new",
                agent.wire,
                [session.wire, wire_utf8_path(str(work / "eval"))],
            )
            request = asyncio.create_task(
                sdk.call(
                    "echo_agent::eval::runner::EvalRunner::run_all",
                    agent.wire,
                    [
                        session.wire,
                        runner,
                        [
                            {
                                "id": "late",
                                "name": "late",
                                "description": "delayed constructor",
                                "domain": None,
                                "task": "SDK",
                                "project_fixture": None,
                                "success_criteria": {
                                    "type": "output_contains",
                                    "substring": "SDK",
                                },
                                "constraints": {},
                            }
                        ],
                        factory.wire,
                    ],
                )
            )
            try:
                await constructing.wait()
                await until(lambda: cancellation[0].is_set())

                def cleanup_sent():
                    frames = [
                        json.loads(line)
                        for line in (work / "frames.jsonl").read_text().splitlines()
                    ]
                    return any(
                        frame.get("method") == "_echo_agent/extension/invoke"
                        and frame["params"]["invocation"]["operation"] == "agent_close"
                        and frame["params"]["extension"] == identity[0][1]
                        for frame in frames
                    )

                await until(cleanup_sent)
                assert observations == {"created": 0, "closed": 0}
                with pytest.raises(EchoAgentError) as error:
                    await factory.close()
                assert error.value.code == "extension_conflict"
                release.set()
                await request
                await until(lambda: observations["closed"] == 1)
                assert observations == {"created": 1, "closed": 1}
                await factory.close()
                await session.close()
                await agent.close()
            finally:
                release.set()
                if not request.done():
                    request.cancel()
