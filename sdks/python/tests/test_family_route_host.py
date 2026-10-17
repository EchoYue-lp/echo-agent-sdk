import os

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST", "").strip()
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required family-route Host missing")


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host binary required")
async def test_native_family_calls_use_catalog_wire_method():
    sdk = await EchoAgentClient.spawn(
        HOST, "--config", os.environ["ECHO_AGENT_SDK_CONFIG"]
    )
    try:
        agent = await sdk.create_agent()
        session = await agent.create_session()
        text = "中😀 alice@example.com 尾"
        expected = "中😀 [REDACTED:EMAIL] 尾"
        assert (
            await sdk.call("content-guard.redact_exact", session.wire, [text])
            == expected
        )
        assert (
            await sdk.family(
                "content_guard", "content-guard.redact_exact", session.wire, [text]
            )
            == expected
        )
        with pytest.raises(EchoAgentError, match="does not belong"):
            await sdk.family(
                "memory", "content-guard.redact_exact", session.wire, [text]
            )
        await session.close()
        await agent.close()
    finally:
        await sdk.close()
