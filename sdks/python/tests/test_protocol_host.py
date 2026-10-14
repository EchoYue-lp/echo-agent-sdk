import os
import sys
import tempfile
from contextlib import asynccontextmanager
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError, WireHandle

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Host E2E binary is missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host binary required"),
]


@asynccontextmanager
async def scenario(mode):
    with tempfile.TemporaryDirectory(prefix="sdk-py-protocol-") as work:
        client = await EchoAgentClient.spawn(
            sys.executable,
            str(Path(__file__).with_name("protocol_host_proxy.py")),
            "--host",
            HOST,
            "--work",
            work,
            "--mode",
            mode,
        )
        async with client:
            yield client


@pytest.mark.parametrize(
    "mode",
    [
        "bad-version",
        "bad-digest",
        "bad-acp-version",
        "overflow-acp-version",
        "overflow-extension-version",
        "nontext-feature",
        "null-capability",
        "empty-capabilities",
        "missing-limits",
        "duplicate-feature",
        "unsorted-features",
        "duplicate-capability",
        "zero-limit",
        "overflow-limit",
        "numeric-limit",
        "invalid-timeout",
    ],
)
async def test_negotiation_rejects(mode):
    with pytest.raises(EchoAgentError) as error:
        async with scenario(mode):
            pytest.fail("must reject")
    assert error.value.code == "extension_capability_mismatch"
    assert error.value.operation == "initialize"
    assert error.value.retryable == "never"


async def test_real_host_typed_error_and_independent_sessions():
    async with scenario("normal") as sdk:
        agent = await sdk.create_agent()
        one, two = await agent.create_session(), await agent.create_session()
        assert one.wire.id != two.wire.id
        stale = WireHandle(one.wire.id, "0", "session")
        with pytest.raises(EchoAgentError) as error:
            await sdk.request("echo_agent/session/close", {"session": stale.to_dict()})
        assert error.value.code == "stale_handle"
        assert error.value.handle == stale
        special = {"__proto__": {"中文": "🙂"}, "constructor": "safe"}
        await two.invoke("memory.store.put", [["protocol"], "特殊键", special])
        stored = await two.invoke("memory.store.get", [["protocol"], "特殊键"])
        assert stored["value"] == special
        await one.close()
        assert await two.invoke("echo_core::agent::Agent::name") == "echo-agent"
        await two.close()
        await agent.close()


async def test_missing_response_envelope():
    async with scenario("no-value") as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session()
        with pytest.raises(EchoAgentError) as error:
            await session.invoke("echo_core::agent::Agent::name")
        assert error.value.code == "serialization_violation"
        await session.close()
        await agent.close()


async def test_session_close_retry():
    async with scenario("close-retry") as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session()
        with pytest.raises(EchoAgentError) as error:
            await session.close()
        assert error.value.code == "extension_timeout"
        await session.close()
        await agent.close()


@pytest.mark.parametrize("mode", ["unregister-retry", "run-close-retry"])
async def test_retry_retains_lifecycle(mode):
    async with scenario(mode) as sdk:
        calls = []

        async def handler(call, cancel):
            calls.append(call)
            return {
                "outcome": "result",
                "result": {
                    "operation": "llm_chat",
                    "value": {
                        "message": {
                            "role": "assistant",
                            "content": {"kind": "string", "value": "中文🙂"},
                        },
                        "finish_reason": "stop",
                        "raw": {"kind": "map", "value": []},
                    },
                },
            }

        registration = await sdk.register_llm_client(
            "protocol-model",
            {
                "kind": "llm_client",
                "descriptor_version": 1,
                "model_name": "protocol-model",
                "supports_streaming": False,
            },
            handler,
        )
        if mode == "unregister-retry":
            with pytest.raises(EchoAgentError) as error:
                await registration.close()
            assert error.value.code == "extension_timeout"
        agent = await sdk.create_agent()
        session = await agent.create_session()
        run = await session.start_run("中文🙂")
        if mode == "run-close-retry":
            with pytest.raises(EchoAgentError) as error:
                await run.close()
            assert error.value.code == "cancellation_timeout"
            await run.close()
        else:
            assert (await run.wait(timeout=5))["settled"]
            assert calls
            await run.close()
        await registration.close()
        await session.close()
        await agent.close()


async def test_agent_close_retry():
    async with scenario("agent-close-retry") as sdk:
        agent = await sdk.create_agent()
        with pytest.raises(EchoAgentError) as error:
            await agent.close()
        assert error.value.code == "extension_timeout"
        await agent.close()


async def test_async_callback_exception_settles_through_host():
    async with scenario("normal") as sdk:

        async def handler(_call, _cancel):
            raise RuntimeError("callback-中文🙂")

        registration = await sdk.register_extension(
            "llm_client",
            "failing-model",
            {
                "kind": "llm_client",
                "descriptor_version": 1,
                "model_name": "failing-model",
                "supports_streaming": False,
            },
            handler,
        )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        run = await session.start_run("trigger")
        settled = await run.wait(timeout=5)
        assert settled["settled"]
        assert settled["terminal"]["status"] == "failed"
        assert settled["receipt"]["outcome"] == "failed"
        await run.close()
        await session.close()
        await registration.close()
        await agent.close()


async def test_cancelled_tool_waits_for_interrupted_callback_side_effect():
    import asyncio

    async with scenario("normal") as sdk:
        parked = asyncio.Event()
        canonical = []
        interrupted = []
        effects = []
        count = 0

        async def observer(call, _cancel):
            if call.operation == "callback_on_tool_start":
                canonical.append(call.call_id)
            if call.operation == "callback_on_tool_interrupted":
                interrupted.append(call.request)
                await asyncio.sleep(0.25)
                effects.append("committed")
            return call.completed()

        callback = await sdk.register_agent_callback(
            "terminal-observer",
            {"kind": "agent_callback", "descriptor_version": 1},
            observer,
        )

        async def hung(call, cancel):
            if call["invocation"]["operation"] == "tool_validate_parameters":
                return {
                    "outcome": "result",
                    "result": {"operation": "tool_validate_parameters", "value": None},
                }
            parked.set()
            await cancel.wait()
            return {
                "outcome": "error",
                "error": {
                    "code": "cancelled",
                    "message": "cancelled",
                    "retryable": "never",
                },
            }

        tool = await sdk.register_extension(
            "tool",
            "hung-tool",
            {
                "kind": "tool",
                "descriptor_version": 1,
                "name": "hung_fixture",
                "description": "park",
                "parameters": {"kind": "map", "value": []},
                "schema_revision": "1",
                "required_input_modalities": [],
                "required_permissions": [],
                "risk_level": "standard",
                "supports_streaming": False,
                "exempt_from_batch_timeout": False,
                "allows_parallel_batch_execution": True,
                "manages_own_timeout": False,
            },
            hung,
        )

        async def llm(_call, _cancel):
            nonlocal count
            count += 1
            message = (
                {
                    "role": "assistant",
                    "content": {"kind": "string", "value": ""},
                    "tool_calls": [
                        {
                            "id": "model-call",
                            "call_type": "function",
                            "function_name": "hung_fixture",
                            "arguments": '{"key":"中文🙂"}',
                        }
                    ],
                }
                if count == 1
                else {
                    "role": "assistant",
                    "content": {"kind": "string", "value": "done"},
                }
            )
            return {
                "outcome": "result",
                "result": {
                    "operation": "llm_chat",
                    "value": {
                        "message": message,
                        "finish_reason": "tool_calls" if count == 1 else "stop",
                        "raw": {"kind": "map", "value": []},
                    },
                },
            }

        model = await sdk.register_llm_client(
            "tool-model",
            {
                "kind": "llm_client",
                "descriptor_version": 1,
                "model_name": "tool-model",
                "supports_streaming": False,
            },
            llm,
        )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        await session.invoke(
            "echo_agent::agent::react::ReactAgent::set_permission_mode", ["full-auto"]
        )
        run = await session.start_run("cancel tool")
        await asyncio.wait_for(parked.wait(), 5)
        await run.cancel()
        settled = await run.wait(timeout=5)
        assert settled["terminal"]["status"] == "cancelled"
        assert effects == ["committed"]
        assert canonical[0] and interrupted[0].call_id == canonical[0]
        assert interrupted[0].tool == "hung_fixture"
        assert interrupted[0].input == {
            "kind": "map",
            "value": [
                {
                    "key": {"kind": "string", "value": "key"},
                    "value": {"kind": "string", "value": "中文🙂"},
                }
            ],
        }
        await run.close()
        await session.close()
        await tool.close()
        await model.close()
        await callback.close()
        await agent.close()


async def test_negotiation_accepts_rust_unicode_feature_order():
    async with scenario("unicode-features") as sdk:
        assert sdk.capability["features"] == ["\ue000", "\U00010000"]
