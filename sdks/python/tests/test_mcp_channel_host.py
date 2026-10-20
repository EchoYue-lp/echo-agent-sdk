import asyncio
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import (
    EchoAgentClient,
    EchoAgentError,
    from_wire,
    to_wire,
)

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required MCP/channel Host missing")
pytestmark = pytest.mark.skipif(not HOST, reason="real Host binary required")
ROOT = Path(__file__).resolve().parents[2] / "shared/testing"


def explicit(value):
    if value is None:
        return {"kind": "null"}
    if isinstance(value, bool):
        return {"kind": "bool", "value": value}
    if isinstance(value, str):
        return {"kind": "string", "value": value}
    if isinstance(value, int):
        return {"kind": "u64" if value >= 0 else "i64", "value": str(value)}
    if isinstance(value, list):
        return {"kind": "list", "value": [explicit(v) for v in value]}
    return {
        "kind": "map",
        "value": [
            {"key": {"kind": "string", "value": k}, "value": explicit(v)}
            for k, v in value.items()
        ],
    }


async def setup(name):
    work = Path(tempfile.mkdtemp(prefix=f"sdk-py-{name}-"))
    print("MCP/channel evidence", work)
    prepare = await asyncio.create_subprocess_exec(
        sys.executable, str(ROOT / "mcp_channel_config_fixture.py"), str(work)
    )
    assert await prepare.wait() == 0
    sdk = await EchoAgentClient.spawn(HOST, "--config", str(work / "host.json"))
    return work, sdk


def live(pid):
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False


async def until(check):
    deadline = asyncio.get_running_loop().time() + 5
    while not check():
        assert asyncio.get_running_loop().time() < deadline
        await asyncio.sleep(0.01)


def save(work, value):
    (work / "observed.json").write_text(json.dumps(value, ensure_ascii=False))


async def fails(work, coroutine, code):
    with pytest.raises(EchoAgentError) as error:
        await coroutine
    assert error.value.code == code
    with (work / "errors.jsonl").open("a") as out:
        out.write(
            json.dumps(
                {
                    "code": error.value.code,
                    "operation": error.value.operation,
                    "message": str(error.value),
                    "retryable": error.value.retryable,
                    "details": error.value.details,
                },
                ensure_ascii=False,
            )
            + "\n"
        )
    return error.value


C = lambda name: "echo_agent::mcp::McpClient::" + name


@pytest.mark.asyncio
async def test_actual_mcp_stdio_metadata_owner_and_process_cleanup():
    work, sdk = await setup("mcp-stdio")
    rows = []
    try:
        agent = await sdk.create_agent()
        one = await agent.create_session()
        two = await agent.create_session()

        async def call(session, op, args):
            value = await sdk.call(op, session.wire, args)
            rows.append({"operation": op, "value": value})
            save(work, rows)
            return value

        ma = (await call(one, "mcp.manager.open", []))["resource"]
        mb = (await call(two, "mcp.manager.open", []))["resource"]
        a = (
            await call(
                one,
                "mcp.server.connect",
                [
                    ma,
                    "same-name",
                    "stdio",
                    sys.executable,
                    [str(ROOT / "mcp_channel_stdio_fixture.py"), str(work), "a"],
                ],
            )
        )["client"]
        b = (
            await call(
                two,
                "mcp.server.connect",
                [
                    mb,
                    "same-name",
                    "stdio",
                    sys.executable,
                    [str(ROOT / "mcp_channel_stdio_fixture.py"), str(work), "b"],
                ],
            )
        )["client"]
        pa = int((work / "a.pid").read_text())
        pb = int((work / "b.pid").read_text())
        assert live(pa) and live(pb)
        assert await call(one, C("server_capabilities"), [a]) == {
            "prompts": {"listChanged": False},
            "resources": {"subscribe": False, "listChanged": False},
            "tools": {},
        }
        assert await call(one, C("tools"), [a]) == [
            {
                "name": "echo",
                "description": "echo Unicode",
                "inputSchema": {
                    "type": "object",
                    "properties": {"text": {"type": "string"}},
                    "required": ["text"],
                },
            }
        ]
        resources = [
            {
                "uri": "fixture://中文/text",
                "name": "中文🙂",
                "description": "text meta",
                "mimeType": "text/plain",
            },
            {
                "uri": "fixture://binary",
                "name": "binary",
                "mimeType": "application/octet-stream",
            },
        ]
        assert await call(one, C("resources"), [a]) == resources
        assert await call(one, C("list_resources"), [a]) == resources
        assert await call(one, C("list_resource_templates"), [a]) == [
            {
                "uriTemplate": "fixture://item/{id}",
                "name": "模板🙂",
                "description": "template",
                "mimeType": "text/plain",
            }
        ]
        assert await call(one, C("read_resource"), [a, "fixture://binary"]) == {
            "contents": [
                {
                    "uri": "fixture://binary",
                    "mimeType": "application/octet-stream",
                    "blob": "AP9BgA==",
                }
            ]
        }
        assert await call(one, C("read_resource"), [a, "fixture://中文/text"]) == {
            "contents": [
                {
                    "uri": "fixture://中文/text",
                    "mimeType": "text/plain",
                    "text": "资源 中文🙂",
                }
            ]
        }
        assert await call(
            one, C("get_prompt"), [a, "compose", {"topic": "无损🧪"}]
        ) == {
            "description": "topic prompt",
            "messages": [
                {"role": "user", "content": {"type": "text", "text": "主题 无损🧪"}}
            ],
        }
        assert await call(one, C("call_tool"), [a, "echo", {"text": "工具🙂"}]) == {
            "content": [{"type": "text", "text": "工具🙂"}],
            "isError": False,
        }
        assert await call(one, C("ping"), [a]) is None
        await fails(work, sdk.call(C("ping"), two.wire, [a]), "invalid_value")
        await call(one, "mcp.server.disconnect", [ma, "same-name"])
        await until(lambda: not live(pa))
        assert live(pb)
        await fails(work, sdk.call(C("ping"), one.wire, [a]), "closed_handle")
        assert await call(two, C("ping"), [b]) is None
        await one.close()
        await fails(
            work, sdk.call("mcp.manager.server_names", two.wire, [ma]), "closed_handle"
        )
        assert live(pb)
        await two.close()
        await until(lambda: not live(pb))
        await agent.close()
        rows.append(
            {
                "process_cleanup": {
                    "a_dead": True,
                    "b_survived_a_close": True,
                    "b_dead_after_own_close": True,
                }
            }
        )
        save(work, rows)
    finally:
        await sdk.close()


@pytest.mark.asyncio
async def test_reverse_transport_failed_close_retains_same_owner_and_quota():
    work, sdk = await setup("mcp-transport-retry")
    calls = []
    closes = 0
    try:

        async def consume(call, cancel):
            nonlocal closes
            inner = call["invocation"]["input"]["call"]
            op = inner["operation"]
            calls.append(inner)
            save(work, calls)
            if op == "mcp_transport_close":
                closes += 1
                if closes == 1:
                    return {
                        "outcome": "error",
                        "error": {
                            "code": "extension_conflict",
                            "message": "controlled transport close failure",
                            "retryable": "after_delay",
                        },
                    }
            if op == "mcp_transport_send":
                request = from_wire(inner["input"]["request"])
                value = {}
                if request["method"] == "initialize":
                    value = {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {},
                        "serverInfo": {"name": "registered-中文🙂", "version": "1"},
                    }
                result = {
                    "operation": op,
                    "value": {
                        "response": to_wire(
                            {
                                "jsonrpc": "2.0",
                                **({"id": request["id"]} if "id" in request else {}),
                                "result": value,
                            }
                        )
                    },
                }
            elif op == "mcp_transport_try_notification":
                result = {"operation": op, "value": {"notification": None}}
            else:
                result = {"operation": op}
            return {
                "outcome": "result",
                "result": {
                    "operation": "agent_component_call",
                    "value": {"component": "mcp_transport", "result": result},
                },
            }

        reg = await sdk.register_extension(
            "agent_component",
            "mcp-transport",
            {
                "kind": "agent_component",
                "descriptor_version": 1,
                "component": "mcp_transport",
                "name": "transport",
                "capabilities": {"supports_notifications": False},
            },
            consume,
        )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        client = await sdk.invoke(
            "echo_integration::mcp::client::McpClient::from_transport",
            agent.wire,
            [session.wire, "registered-中文🙂", reg.wire],
        )
        error = await fails(
            work, sdk.call(C("close"), session.wire, [client]), "framework_error"
        )
        assert error.retryable == "after_delay"
        assert closes == 1
        assert (
            await sdk.call(C("server_name"), session.wire, [client])
            == "registered-中文🙂"
        )
        assert await sdk.call(C("close"), session.wire, [client]) is None
        assert closes == 2
        await fails(work, sdk.call(C("ping"), session.wire, [client]), "closed_handle")
        for _ in range(4):
            await sdk.call("mcp.manager.open", session.wire, [])
        await fails(
            work, sdk.call("mcp.manager.open", session.wire, []), "payload_too_large"
        )
        await session.close()
        await reg.close()
        await agent.close()
        save(work, {"calls": calls, "closeCount": closes, "released_quota_refilled": 4})
    finally:
        await sdk.close()


@pytest.mark.asyncio
@pytest.mark.parametrize("mode", ["normal", "retry"])
async def test_channel_outbound_attachments_and_exact_stop(mode):
    work, sdk = await setup("channels-" + mode)
    calls = []
    stops = {"a": 0, "b": 0}
    running = {"a": False, "b": False}
    processes = {}
    try:

        async def inbound(call, cancel):
            raise AssertionError(
                "No real inbound path exists: callback must not be faked"
            )

        handler = await sdk.register_extension(
            "channel_message_handler",
            "handler",
            {
                "kind": "channel_message_handler",
                "descriptor_version": 1,
                "handler_id": "handler",
            },
            inbound,
        )
        regs = []
        for channel in ["a", "b"]:

            async def consume(call, cancel, channel=channel):
                op = call["invocation"]["operation"]
                calls.append(
                    {
                        "id": channel,
                        "operation": op,
                        "input": call["invocation"]["input"],
                    }
                )
                save(work, {"calls": calls, "stops": stops, "running": running})
                if op == "channel_start":
                    running[channel] = True
                    processes[channel] = await asyncio.create_subprocess_exec(
                        sys.executable, "-c", "import time; time.sleep(60)"
                    )
                    (work / (channel + ".pid")).write_text(str(processes[channel].pid))
                if op == "channel_stop":
                    stops[channel] += 1
                    if mode == "retry" and channel == "a" and stops["a"] == 1:
                        return {
                            "outcome": "error",
                            "error": {
                                "code": "extension_conflict",
                                "message": "stop once fails",
                                "retryable": "after_delay",
                            },
                        }
                    processes[channel].terminate()
                    await processes[channel].wait()
                    running[channel] = False
                return {"outcome": "result", "result": {"operation": op, "value": None}}

            regs.append(
                await sdk.register_extension(
                    "channel_plugin",
                    "plugin-" + channel,
                    {
                        "kind": "channel_plugin",
                        "descriptor_version": 1,
                        "channel_id": channel,
                        "label": "通道 " + channel,
                        "handler_id": "handler",
                        "capabilities": {
                            "chat_types": ["direct", "group"],
                            "supports_media": True,
                            "supports_threads": True,
                        },
                    },
                    consume,
                )
            )
        agent = await sdk.create_agent()
        session = await agent.create_session()
        manager = (await sdk.call("channels.manager.open", session.wire, []))[
            "resource"
        ]
        for reg in regs:
            assert (
                await sdk.call(
                    "channels.plugin.register",
                    session.wire,
                    [manager, reg.wire, handler.wire],
                )
            )["registered"]
        assert await sdk.call(
            "channels.manager.start", session.wire, [manager, handler.wire]
        ) == {"channels": "2", "failures": "0"}
        assert sorted(
            await sdk.call("channels.manager.ids", session.wire, [manager])
        ) == ["a", "b"]
        import base64

        attachments = [
            {
                "kind": kind,
                "data_base64": base64.b64encode(bytes([0, 255, i, 128])).decode(),
                "filename": f"文件-{kind}-🙂.bin",
            }
            for i, kind in enumerate(["image", "file", "audio", "video"])
        ]
        message = {
            "channel_id": "a",
            "to": "收件人🙂",
            "chat_type": "direct",
            "text": "hello 中文🧪",
            "reply_to": "原消息🙂",
            "attachments": attachments,
        }
        assert (
            await sdk.call(
                "channels.plugin.send", session.wire, [manager, "a", explicit(message)]
            )
        )["sent"]
        assert next(c for c in calls if c["operation"] == "channel_send")["input"] == {
            "message": message
        }
        assert [c["input"] for c in calls if c["operation"] == "channel_start"] == [
            {"handler_id": "handler"},
            {"handler_id": "handler"},
        ]
        if mode == "retry":
            await fails(
                work,
                sdk.call("channels.manager.stop_exact", session.wire, [manager, "a"]),
                "framework_error",
            )
            assert live(processes["a"].pid) and live(processes["b"].pid)
        assert running["b"] and stops["b"] == 0
        assert (
            await sdk.call("channels.manager.stop_exact", session.wire, [manager, "a"])
            is None
        )
        assert not running["a"] and running["b"] and stops["b"] == 0
        assert not live(processes["a"].pid) and live(processes["b"].pid)
        assert stops["a"] == (2 if mode == "retry" else 1)
        assert (
            await sdk.call("channels.manager.health", session.wire, [manager, "b"])
        )["healthy"]
        await session.close()
        assert not running["b"] and stops["b"] == 1 and not live(processes["b"].pid)
        for reg in regs:
            await reg.close()
        await handler.close()
        await agent.close()
        save(
            work,
            {
                "calls": calls,
                "stops": stops,
                "running": running,
                "inbound_handler_calls": 0,
            },
        )
    finally:
        await sdk.close()
        for child in processes.values():
            if child.returncode is None:
                child.terminate()
                await child.wait()


@pytest.mark.asyncio
async def test_channel_owned_callbacks_reenter_metadata_and_preserve_close_owner():
    work, sdk = await setup("channels-owned-reentry")
    events, owners, phases, stops = [], {}, {}, {"a": 0, "b": 0, "other": 0}
    parked, release = asyncio.Event(), asyncio.Event()
    park_health = False
    fail_close_once = True

    async def close_manager(session, manager):
        operation = "facade.resource.close"
        return await sdk.request(
            "_echo_agent/facade/invoke",
            {
                "operation": operation,
                "signature_digest": sdk.catalog.resolve(operation).signature,
                "handle": session.wire.to_dict(),
                "arguments": [{"kind": "handle", "value": manager}],
            },
        )

    try:

        async def inbound(call, cancel):
            raise AssertionError("no accepted inbound consumer; do not fabricate one")

        handler = await sdk.register_extension(
            "channel_message_handler",
            "owned-handler",
            {
                "kind": "channel_message_handler",
                "descriptor_version": 1,
                "handler_id": "owned-handler",
            },
            inbound,
        )
        agent = await sdk.create_agent()
        one, two = await agent.create_session(), await agent.create_session()
        ma = (await sdk.call("channels.manager.open", one.wire, []))["resource"]
        mb = (await sdk.call("channels.manager.open", two.wire, []))["resource"]
        regs = []
        for key, channel, session, manager, ids in [
            ("a", "a", one, ma, ["a", "b"]),
            ("b", "b", one, ma, ["a", "b"]),
            ("other", "c", two, mb, ["c"]),
        ]:
            owners[key], phases[key] = (session, manager, ids), "active"

            async def consume(call, cancel, key=key):
                nonlocal fail_close_once
                operation = call["invocation"]["operation"]
                session, manager, ids = owners[key]
                phase = phases[key]
                for metadata in ["ids", "list"]:
                    try:
                        value = await asyncio.wait_for(
                            sdk.call(
                                "channels.manager." + metadata, session.wire, [manager]
                            ),
                            2,
                        )
                        assert phase == "active"
                        assert (
                            sorted(value if metadata == "ids" else value["channels"])
                            == ids
                        )
                        events.append(
                            {
                                "id": key,
                                "operation": operation,
                                "phase": phase,
                                "metadata": metadata,
                                "result": "read-before-return",
                            }
                        )
                    except EchoAgentError as error:
                        assert phase != "active" and error.code == "closed_handle"
                        events.append(
                            {
                                "id": key,
                                "operation": operation,
                                "phase": phase,
                                "metadata": metadata,
                                "result": error.code,
                            }
                        )
                save(work, events)
                if key == "a" and operation == "channel_health" and park_health:
                    parked.set()
                    await release.wait()
                if operation == "channel_stop":
                    stops[key] += 1
                    if key == "a" and phase == "resource-close" and fail_close_once:
                        fail_close_once = False
                        return {
                            "outcome": "error",
                            "error": {
                                "code": "extension_conflict",
                                "message": "owned stop once fails",
                                "retryable": "after_delay",
                            },
                        }
                return {
                    "outcome": "result",
                    "result": {"operation": operation, "value": None},
                }

            reg = await sdk.register_extension(
                "channel_plugin",
                "owned-" + key,
                {
                    "kind": "channel_plugin",
                    "descriptor_version": 1,
                    "channel_id": channel,
                    "label": "owned 中文🙂 " + key,
                    "handler_id": "owned-handler",
                    "capabilities": {
                        "chat_types": ["direct"],
                        "supports_media": False,
                        "supports_threads": False,
                    },
                },
                consume,
            )
            regs.append(reg)
            await sdk.call(
                "channels.plugin.register",
                session.wire,
                [manager, reg.wire, handler.wire],
            )
        assert await sdk.call(
            "channels.manager.start", one.wire, [ma, handler.wire]
        ) == {"channels": "2", "failures": "0"}
        assert await sdk.call(
            "channels.manager.start", two.wire, [mb, handler.wire]
        ) == {"channels": "1", "failures": "0"}
        message = {
            "channel_id": "a",
            "to": "用户🙂",
            "chat_type": "direct",
            "text": "重入中文",
        }
        assert (
            await sdk.call(
                "channels.plugin.send", one.wire, [ma, "a", explicit(message)]
            )
        )["sent"]
        park_health = True
        health = asyncio.create_task(
            sdk.call("channels.manager.health", one.wire, [ma, "a"])
        )
        try:
            await asyncio.wait_for(parked.wait(), 3)
            assert not health.done()
            assert sorted(
                await asyncio.wait_for(
                    sdk.call("channels.manager.ids", one.wire, [ma]), 2
                )
            ) == ["a", "b"]
            assert (
                await asyncio.wait_for(
                    sdk.call("channels.manager.health", one.wire, [ma, "b"]), 2
                )
            )["healthy"]
            assert (
                await asyncio.wait_for(
                    sdk.call("channels.manager.health", two.wire, [mb, "c"]), 2
                )
            )["healthy"]
            assert not health.done()
        finally:
            release.set()
        assert (await health)["healthy"]
        assert (await sdk.call("channels.manager.stop", one.wire, [ma, "b"]))["stopped"]
        assert (
            await sdk.call("channels.manager.stop_exact", one.wire, [ma, "a"]) is None
        )
        assert stops == {"a": 1, "b": 1, "other": 0}
        await sdk.call("channels.manager.start", one.wire, [ma, handler.wire])
        assert await sdk.call("channels.manager.stop_all", one.wire, [ma]) is None
        assert stops == {"a": 2, "b": 2, "other": 0}
        await sdk.call("channels.manager.start", one.wire, [ma, handler.wire])
        phases["a"] = phases["b"] = "resource-close"
        await fails(work, close_manager(one, ma), "framework_error")
        assert stops == {"a": 3, "b": 3, "other": 0}
        assert (await sdk.call("channels.manager.health", two.wire, [mb, "c"]))[
            "healthy"
        ]
        await close_manager(one, ma)
        assert stops == {"a": 4, "b": 3, "other": 0}
        await fails(
            work, sdk.call("channels.manager.ids", one.wire, [ma]), "closed_handle"
        )
        # The independent manager remains live; three released quota slots are reusable.
        for _ in range(3):
            await sdk.call("channels.manager.open", one.wire, [])
        await fails(
            work, sdk.call("channels.manager.open", one.wire, []), "payload_too_large"
        )
        await one.close()
        phases["other"] = "session-close"
        await asyncio.wait_for(two.close(), 5)
        assert stops == {"a": 4, "b": 3, "other": 1}
        for reg in regs:
            await reg.close()
        await handler.close()
        await agent.close()
        save(
            work,
            {
                "events": events,
                "stops": stops,
                "parked_other_access": True,
                "quota_refilled": 3,
                "inbound_handler_calls": 0,
            },
        )
    finally:
        release.set()
        await sdk.close()
