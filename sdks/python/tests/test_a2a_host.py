import json
import os
import shutil
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
CONFIG = os.environ.get("ECHO_AGENT_SDK_CONFIG")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not (HOST and CONFIG):
    raise RuntimeError("real Host/config required")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not (HOST and CONFIG), reason="real Host required"),
]
RETIRED = json.loads(
    (
        Path(__file__).parents[2] / "shared/testing/a2a_lsp_retirement_contract.json"
    ).read_text()
)


async def test_retired_features_fail_negotiation():
    for feature in ("a2a", "lsp"):
        with pytest.raises(EchoAgentError) as failure:
            await EchoAgentClient.spawn(
                HOST, "--config", CONFIG, required_features=[feature]
            )
        assert failure.value.code == "feature_unavailable"


async def test_retired_methods_operations_and_source_invokes_fail():
    async with await EchoAgentClient.spawn(HOST, "--config", CONFIG) as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session()
        count = 0
        for family in RETIRED["families"]:
            route = next(
                route
                for route in RETIRED["routes"]
                if route["route"] == "family:" + family["family"]
            )
            for operation in family["operations"]:
                signature = next(
                    item["signature_digests"][0]
                    for item in route["operation_signatures"]
                    if item["operation"] == operation
                )
                with pytest.raises(EchoAgentError):
                    await sdk.request(
                        family["methods"][0],
                        {
                            "operation": operation,
                            "signature_digest": signature,
                            "handle": session.wire.to_dict(),
                            "arguments": [],
                        },
                    )
                count += 1
        assert count == 39
        for route in (route for route in RETIRED["routes"] if route["operation"]):
            with pytest.raises(EchoAgentError):
                await sdk.request(
                    "_echo_agent/facade/invoke",
                    {
                        "operation": route["operation"],
                        "signature_digest": route["signature_digests"][0],
                        "handle": agent.wire.to_dict(),
                        "arguments": [
                            {"kind": "handle", "value": session.wire.to_dict()}
                        ],
                    },
                )
        old_values = [
            {
                "kind": "variant",
                "value": {
                    "type_id": "echo_core::plugin::capability::PluginCapability",
                    "variant": "LspServer",
                    "fields": [],
                },
            },
            {
                "kind": "record",
                "value": {
                    "type_id": "echo_agent::plugin::prepared::PluginWiringResult",
                    "fields": [{"name": "lsp_discovered", "value": {"kind": "null"}}],
                },
            },
            {
                "kind": "record",
                "value": {
                    "type_id": "echo_core::plugin::registry::ResolvedComponents",
                    "fields": [{"name": "lsp_config_file", "value": {"kind": "null"}}],
                },
            },
        ]
        for value in old_values:
            with pytest.raises(EchoAgentError) as failure:
                await sdk.request(
                    "_echo_agent/facade/invoke",
                    {
                        "operation": "retirement.probe",
                        "signature_digest": "sha256:" + "0" * 64,
                        "handle": agent.wire.to_dict(),
                        "arguments": [value],
                    },
                )
            assert failure.value.code == "invalid_value"
            assert "invalid facade argument" in failure.value.message
        await session.close()
        other = await agent.create_session()
        await other.close()
        await agent.close()


async def test_old_source_digest_is_rejected():
    shared = Path(__file__).parents[2] / "shared"
    output = Path(tempfile.mkdtemp(prefix="sdk-retired-digest-"))
    shutil.copyfile(
        shared / "facade-operation-catalog.json",
        output / "facade-operation-catalog.json",
    )
    digests = json.loads((shared / "contract-digests.json").read_text())
    digests["source_contract_digest"] = RETIRED["old_source_contract_digest"]
    (output / "contract-digests.json").write_text(json.dumps(digests))
    with pytest.raises(EchoAgentError) as failure:
        await EchoAgentClient.spawn(
            HOST,
            "--config",
            CONFIG,
            catalog_path=output / "facade-operation-catalog.json",
        )
    assert failure.value.code == "extension_capability_mismatch"
