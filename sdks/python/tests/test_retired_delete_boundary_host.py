"""真实退休后catalog上的本地拒绝/Host负控；不修改seed或生产适配器。"""

import json
import os
import sys
import tempfile
from contextlib import asynccontextmanager
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required retirement Host is missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]
DELETE = "echo_agent::state::delete_persisted_conversation"
METHOD = "_echo_agent/facade/invoke"
OLD_SIGNATURE = (
    "sha256:97b7ff491c492fa6e3b26e6130c5f51285ff1f1608ae64d447de744152c501e2"
)
RUNTIME_ROOT = Path(
    os.environ.get(
        "ECHO_AGENT_SDK_RETIRED_DELETE_RUNTIME_ROOT",
        str(Path(__file__).resolve().parents[3] / "target/retired-delete-boundary"),
    )
)


def requests(work):
    frames = [
        json.loads(line)
        for line in (work / "client-frames.jsonl").read_text().splitlines()
        if line
    ]
    return [
        {
            "id": frame["id"],
            "method": frame["method"],
            "operation": frame.get("params", {}).get("operation"),
        }
        for frame in frames
        if "id" in frame and isinstance(frame.get("method"), str)
    ]


def unavailable(error):
    assert error.code == "feature_unavailable"
    assert error.retryable == "never"
    assert error.operation == DELETE
    assert "unmanaged conversation deletion is retired" in str(error)


@asynccontextmanager
async def scenario(label):
    RUNTIME_ROOT.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix=f"python-{label}-", dir=RUNTIME_ROOT))
    proof = {
        "label": label,
        "epoch": os.environ.get("ECHO_AGENT_SDK_TEST_EPOCH", "unspecified"),
        "work": str(work),
    }
    client = None
    try:
        client = await EchoAgentClient.spawn(
            sys.executable,
            str(Path(__file__).with_name("protocol_host_proxy.py")),
            "--host",
            HOST,
            "--work",
            str(work),
            "--mode",
            "normal",
        )
        assert all(item.operation != DELETE for item in client.catalog.operations()), (
            "requires actual retired catalog"
        )
        agent = await client.create_agent()
        session = await agent.create_session()
        await agent.describe()
        yield client, agent, session, work, proof
        proof["passed"] = True
    except BaseException as error:
        proof["failure"] = {"type": type(error).__name__, "message": str(error)}
        raise
    finally:
        try:
            if client is not None:
                await client.close()
        finally:
            (work / "boundary-proof.json").write_text(
                json.dumps(proof, ensure_ascii=False, indent=2) + "\n"
            )


async def test_retired_delete_all_public_local_entry_points_submit_no_request_ids():
    async with scenario("local") as (client, agent, session, work, proof):
        before = requests(work)
        for call in [
            lambda: client.invoke(DELETE, agent.wire, [session.wire, "scope"]),
            lambda: client.call(DELETE, agent.wire, [session.wire, "scope"]),
            lambda: session.invoke(DELETE, ["scope"]),
        ]:
            with pytest.raises(EchoAgentError) as failure:
                await call()
            unavailable(failure.value)
        await agent.describe()
        delta = requests(work)[len(before) :]
        assert len(delta) == 1 and delta[0]["method"] == "_echo_agent/agent/describe"
        proof["submitted"] = delta


async def test_retired_delete_exact_matching_and_closed_session_priority():
    async with scenario("negative") as (client, agent, session, work, proof):
        before = requests(work)
        for unknown in [DELETE + "_unknown", "prefix::" + DELETE, DELETE.upper()]:
            with pytest.raises(KeyError):
                client.catalog.signature(unknown)
            for call in [
                lambda unknown=unknown: client.invoke(unknown, agent.wire),
                lambda unknown=unknown: client.call(unknown, agent.wire),
                lambda unknown=unknown: session.invoke(unknown),
            ]:
                with pytest.raises(KeyError):
                    await call()
        await agent.describe()
        delta = requests(work)[len(before) :]
        assert len(delta) == 1 and delta[0]["method"] == "_echo_agent/agent/describe"
        proof["unknown_submissions"] = delta
        await session.close()
        before_closed = requests(work)
        for operation in [DELETE, DELETE + "_unknown"]:
            with pytest.raises(EchoAgentError) as failure:
                await session.invoke(operation)
            assert failure.value.code == "closed_handle"
        await agent.describe()
        delta_closed = requests(work)[len(before_closed) :]
        assert (
            len(delta_closed) == 1
            and delta_closed[0]["method"] == "_echo_agent/agent/describe"
        )
        proof["closed_session_submissions"] = delta_closed


async def test_direct_host_retired_delete_unknown_and_malformed_controls():
    async with scenario("host") as (client, _agent, _session, _work, proof):
        params = {
            "operation": DELETE,
            "signature_digest": OLD_SIGNATURE,
            "handle": None,
            "arguments": [],
        }
        with pytest.raises(EchoAgentError) as failure:
            await client.request(METHOD, params)
        unavailable(failure.value)
        for changed, expected_code in [
            ({**params, "operation": DELETE + "_unknown"}, "invalid_value"),
            ({**params, "arguments": "not-an-array"}, "invalid_request"),
        ]:
            with pytest.raises(EchoAgentError) as failure:
                await client.request(METHOD, changed)
            assert failure.value.code == expected_code
        proof["direct_host_controls"] = ["retired", "unknown", "malformed"]
