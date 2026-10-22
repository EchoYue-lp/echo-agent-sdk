import base64
import json
import os
import sys
from pathlib import Path

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


REQUIRE_WINDOWS = os.environ.get("ECHO_AGENT_SDK_REQUIRE_WINDOWS_NATIVE") == "1"
if REQUIRE_WINDOWS and (
    sys.platform != "win32" or not HOST or not os.environ.get("ECHO_AGENT_SDK_CONFIG")
):
    raise RuntimeError("required Windows native Host/config/platform missing")


@pytest.mark.asyncio
@pytest.mark.skipif(
    sys.platform != "win32" or not HOST, reason="Windows native Host required"
)
async def test_windows_native_plugin_registry_path_round_trip(tmp_path):
    units = [
        0x0043,
        0x003A,
        0x005C,
        0x4E2D,
        0x6587,
        0xD83D,
        0xDE42,
        0x002D,
        0xD800,
        0x0041,
        0xDC00,
    ]

    def encode(values):
        return (
            base64.b64encode(b"".join(unit.to_bytes(2, "little") for unit in values))
            .decode("ascii")
            .rstrip("=")
        )

    path = {
        "kind": "path",
        "value": {
            "encoding": "windows",
            "utf16_base64": encode(units),
            "display": "Z:/display-诱饵",
        },
    }
    child_a = units + [ord(value) for value in "\\child-A"]
    child_b = child_a + [ord(value) for value in "\\child-B"]
    config = json.loads(Path(os.environ["ECHO_AGENT_SDK_CONFIG"]).read_text())
    config["sdk_profile"]["state_root"] = str(tmp_path / "state")
    if "memory_path" in config["default_agent"]["agent"]:
        config["default_agent"]["agent"]["memory_path"] = str(tmp_path / "memory.jsonl")
    config_path = tmp_path / "host.json"
    config_path.write_text(json.dumps(config))
    sdk = await EchoAgentClient.spawn(HOST, "--config", str(config_path))
    try:
        agent = await sdk.create_agent()
        session = await agent.create_session(str(tmp_path))
        operation = "echo_core::plugin::registry::PluginRegistry::with_paths"
        data_dir = "echo_core::plugin::registry::PluginRegistry::data_dir_for"
        registry = await session.invoke(operation, [path, path, None])
        first = await session.invoke(data_dir, [registry, "child-A"])
        assert first["kind"] == "path" and first["value"]["encoding"] == "windows"
        assert first["value"]["utf16_base64"] == encode(child_a)
        assert first["value"].get("display") != path["value"]["display"]
        second_registry = await session.invoke(operation, [path, first, None])
        second = await session.invoke(data_dir, [second_registry, "child-B"])
        assert second["kind"] == "path" and second["value"]["encoding"] == "windows"
        assert second["value"]["utf16_base64"] == encode(child_b)
        (tmp_path / "windows-path-proof.json").write_text(
            json.dumps(
                {
                    "platform": sys.platform,
                    "units": units,
                    "first": first,
                    "second": second,
                    "native_calls": 4,
                }
            )
        )
        print(
            "Windows native path consumer: 1 passed, 0 failed, 0 skipped; 4 native calls"
        )
        await session.close()
        await agent.close()
    finally:
        await sdk.close()
