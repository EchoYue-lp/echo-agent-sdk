"""Actual native ChannelPlugin/MessageHandler forward consumers on a real Host."""

import asyncio
import copy
import hashlib
import json
import os
import sys
import tempfile
from collections import Counter
from pathlib import Path

import pytest

from echo_agent_sdk import (
    EchoAgentClient,
    EchoAgentError,
    WireHandle,
    from_wire,
    to_wire,
)

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Channel forward Host missing")
SHARED = Path(__file__).resolve().parents[2] / "shared/testing"
SPEC = json.loads((SHARED / "channel_forward_scenarios.json").read_text())
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="new-contract real Host required"),
]


def business_wire(value):
    # Attachments own a business `kind`; explicitly encode their map so the
    # public generic WireValue converter cannot treat them as protocol leaves.
    if isinstance(value, dict):
        return {
            "kind": "map",
            "value": [
                {"key": {"kind": "string", "value": key}, "value": business_wire(item)}
                for key, item in value.items()
            ],
        }
    if isinstance(value, list):
        return {"kind": "list", "value": [business_wire(item) for item in value]}
    return to_wire(value)


def result(operation, value=None):
    return {"outcome": "result", "result": {"operation": operation, "value": value}}


def event_parts(value):
    fact = value.get("value", value)
    return fact["variant"], {
        field["name"]: from_wire(field["value"]) for field in fact["fields"]
    }


def normalize(message):
    return {
        **message,
        "reply_to": message.get("reply_to"),
        "attachments": [
            {**item, "filename": item.get("filename")}
            for item in message.get("attachments", [])
        ],
    }


@pytest.fixture(scope="module")
def evidence():
    root = Path(
        os.environ.get(
            "ECHO_AGENT_SDK_CHANNEL_FORWARD_ROOT",
            str(Path(__file__).resolve().parents[3] / "target/channel-forward"),
        )
    )
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-channel-forward-", dir=root))
    receipt = {
        "language": "python",
        "host_sha256": hashlib.sha256(Path(HOST).read_bytes()).hexdigest()
        if HOST
        else None,
        "source_hashes": {
            str(path): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in [Path(__file__), SHARED / "channel_forward_scenarios.json"]
        },
        "scenarios": [],
    }
    yield work, receipt
    (work / "receipt.json").write_text(
        json.dumps(receipt, ensure_ascii=False, indent=2)
    )
    exported = os.environ.get("ECHO_AGENT_SDK_CHANNEL_FORWARD_EVIDENCE")
    if exported:
        Path(exported).mkdir(parents=True, exist_ok=True)
        (Path(exported) / "python.json").write_text(
            json.dumps(receipt, ensure_ascii=False, indent=2)
        )
    print("Channel forward Python receipt: " + str(work / "receipt.json"))


class Native:
    def __init__(self, sdk, work):
        self.sdk, self.work = sdk, work
        self.calls = []
        self.plugin_calls = []
        self.finally_count = 0
        self.modes = {}
        self.started = {}
        self.cancelled = {}
        self.release = {}
        self.metadata = []
        self.registrations = []
        self.sessions = []
        self.owners = []
        self.start_fail = False
        self.start_probe = False

    async def row_error(self, awaitable, code="invalid_value"):
        with pytest.raises(EchoAgentError) as caught:
            await awaitable
        error = caught.value
        assert error.code == code, str(error)
        return {
            "code": error.code,
            "message": str(error),
            "operation": error.operation,
            "details": error.details,
        }

    async def register(self, session, manager, channel=None, handler_id=None):
        channel = channel or SPEC["channel_id"]
        handler_id = handler_id or SPEC["handler_id"]
        owner = {
            "session": session,
            "manager": manager,
            "channel": channel,
            "text": SPEC["outbound"]["text"],
        }
        self.owners.append(owner)

        async def handler(call, cancellation):
            operation = call["invocation"]["operation"]
            message = call["invocation"]["input"]["message"]
            mode = self.modes.get(
                message.get("message_id", message.get("text")), "normal"
            )
            self.calls.append(
                {
                    "operation": operation,
                    "message": message,
                    "extension": call["extension"],
                    "invocation_id": call["invocation_id"],
                    "deadline": call["deadline"],
                }
            )
            try:
                ids = await self.sdk.call(
                    "channels.manager.ids", session.wire, [manager]
                )
                listing = await self.sdk.call(
                    "channels.manager.list", session.wire, [manager]
                )
                self.metadata.append({"ids": ids, "list": listing})
                if mode in self.started:
                    self.started[mode].set()
                if mode in {"park", "full", "caller", "stop", "resource", "session"}:
                    if mode == "full":
                        writer = self.sdk.stream_writer(
                            WireHandle.from_dict(call["stream"])
                        )
                        await writer.chunk(
                            {"kind": "channel", "value": SPEC["stream_items"][0]}
                        )
                        await writer.chunk(
                            {"kind": "channel", "value": SPEC["stream_items"][1]}
                        )
                    await asyncio.wait_for(cancellation.wait(), 10)
                    self.cancelled[mode].set()
                    await asyncio.wait_for(self.release[mode].wait(), 10)
                    return (
                        result(operation, SPEC["outbound"])
                        if operation == "channel_handle"
                        else {"outcome": "stream", "stream": call["stream"]}
                    )
                if mode == "error":
                    return {
                        "outcome": "error",
                        "error": SPEC["callback_behaviors"][
                            "fail-handle"
                            if operation == "channel_handle"
                            else "fail-reply"
                        ]["error"],
                    }
                if operation == "channel_reply":
                    return result(operation)
                if operation == "channel_handle":
                    return result(
                        operation,
                        {
                            **SPEC["outbound"],
                            "channel_id": channel,
                            "text": owner["text"],
                        },
                    )
                assert operation == "channel_handle_stream"
                writer = self.sdk.stream_writer(WireHandle.from_dict(call["stream"]))
                await writer.chunk(
                    {
                        "kind": "channel",
                        "value": {**SPEC["stream_items"][0], "channel_id": channel},
                    }
                )
                await writer.complete(
                    {
                        "kind": "channel",
                        "value": {**SPEC["stream_items"][1], "channel_id": channel},
                    }
                )
                return {"outcome": "stream", "stream": call["stream"]}
            finally:
                self.finally_count += 1

        registration = await self.sdk.register_channel_message_handler(
            handler_id,
            {
                "kind": "channel_message_handler",
                "descriptor_version": 1,
                "handler_id": handler_id,
            },
            handler,
        )

        async def plugin(call, _cancel):
            operation = call["invocation"]["operation"]
            self.plugin_calls.append(
                {
                    "operation": operation,
                    "channel": channel,
                    "input": call["invocation"].get("input"),
                }
            )
            if operation == "channel_start":
                if self.start_probe:
                    await self.row_error(
                        self.sdk.call(
                            "channels.handler.handle",
                            session.wire,
                            [
                                manager,
                                channel,
                                business_wire(
                                    {**SPEC["inbound"], "channel_id": channel}
                                ),
                            ],
                        )
                    )
                if self.start_fail:
                    self.start_fail = False
                    return {
                        "outcome": "error",
                        "error": SPEC["callback_behaviors"]["fail-start-once"]["error"],
                    }
            return result(operation)

        plugin_registration = await self.sdk.register_channel_plugin(
            "plugin-" + channel + "-" + str(len(self.owners)),
            {
                "kind": "channel_plugin",
                "descriptor_version": 1,
                "channel_id": channel,
                "label": "native 中文🙂",
                "handler_id": handler_id,
                "capabilities": {
                    "chat_types": ["direct", "group"],
                    "supports_media": True,
                    "supports_threads": True,
                },
            },
            plugin,
        )
        self.registrations.extend([registration, plugin_registration])
        assert (
            await self.sdk.call(
                "channels.plugin.register",
                session.wire,
                [manager, plugin_registration.wire, registration.wire],
            )
        )["registered"]
        owner["handler"] = registration
        owner["plugin"] = plugin_registration
        return owner

    async def start(self, owner):
        return await self.sdk.call(
            "channels.manager.start",
            owner["session"].wire,
            [owner["manager"], owner["handler"].wire],
        )

    async def call(self, owner, operation, message=None):
        args = [owner["manager"], owner["channel"]]
        if message is not None:
            args.append(business_wire(message))
        return await self.sdk.call(operation, owner["session"].wire, args)

    def park(self, mode):
        self.modes[mode] = mode
        self.started[mode] = asyncio.Event()
        self.cancelled[mode] = asyncio.Event()
        self.release[mode] = asyncio.Event()
        return {**SPEC["inbound"], "message_id": mode}

    async def stop(self, owner):
        return await self.sdk.call(
            "channels.manager.stop_exact",
            owner["session"].wire,
            [owner["manager"], owner["channel"]],
        )

    async def close_manager(self, owner):
        operation = "facade.resource.close"
        return await self.sdk.request(
            "_echo_agent/facade/invoke",
            {
                "operation": operation,
                "signature_digest": self.sdk.catalog.resolve(operation).signature,
                "handle": owner["session"].wire.to_dict(),
                "arguments": [{"kind": "handle", "value": owner["manager"].to_dict()}],
            },
        )

    async def close(self):
        for release in self.release.values():
            release.set()
        await self.sdk.close()


async def setup(evidence, name):
    root, receipt = evidence
    work = root / name
    work.mkdir()
    process = await asyncio.create_subprocess_exec(
        sys.executable, str(SHARED / "mcp_channel_config_fixture.py"), str(work)
    )
    assert await process.wait() == 0
    config_path = work / "host.json"
    config = json.loads(config_path.read_text())
    config["sdk_profile"]["limits"]["shutdown_timeout_secs"] = 5
    config_path.write_text(json.dumps(config))
    sdk = await EchoAgentClient.spawn(HOST, "--config", str(config_path))
    native = Native(sdk, work)
    native.agent = await sdk.create_agent()
    for _ in range(2):
        session = await native.agent.create_session()
        native.sessions.append(session)
    manager = (await sdk.call("channels.manager.open", native.sessions[0].wire, []))[
        "resource"
    ]
    manager = WireHandle.from_dict(manager) if isinstance(manager, dict) else manager
    owner = await native.register(native.sessions[0], manager)
    return native, owner, receipt


def archive(native, receipt, rows):
    receipt["scenarios"].extend(rows)
    (native.work / "observed.json").write_text(
        json.dumps(
            {
                "calls": native.calls,
                "plugin_calls": native.plugin_calls,
                "finally_count": native.finally_count,
                "metadata": native.metadata,
                "rows": rows,
            },
            ensure_ascii=False,
            indent=2,
        )
    )


async def test_channel_single_stream_reply_typed_fidelity_and_validation(evidence):
    native, owner, receipt = await setup(evidence, "core")
    rows = []
    try:
        assert await native.start(owner) == {"channels": "1", "failures": "0"}
        single = await native.call(owner, "channels.handler.handle", SPEC["inbound"])
        stream = await native.call(
            owner, "channels.handler.handle_stream", SPEC["inbound"]
        )
        events = [
            await native.sdk.call(
                "channels.stream.next", owner["session"].wire, [stream]
            )
            for _ in range(3)
        ]
        variants, parts = zip(*(event_parts(event) for event in events))
        assert variants == ("item", "item", "complete")
        assert all(item["stream"] == stream for item in parts)
        reply = await native.call(owner, "channels.handler.reply", SPEC["outbound"])
        core = {
            "single": normalize(single),
            "items": [normalize(item["value"]) for item in parts[:2]],
            "terminal": variants[-1],
            "sequences": [item["sequence"] for item in parts],
            "reply": reply,
            "callback_counts": dict(
                Counter(call["operation"] for call in native.calls)
            ),
        }
        assert core == SPEC["normalized_core_expected"]
        assert native.calls[0]["message"] == SPEC["inbound"]
        rows.append({"scenario": "single-stream-reply", "observed": core})
        before = len(native.calls)
        for invalid in SPEC["invalid_inbound"]:
            message = copy.deepcopy(SPEC["inbound"])
            if "remove" in invalid:
                message.pop(invalid["remove"])
            else:
                message[invalid["field"]] = invalid["value"]
            error = await native.row_error(
                native.call(owner, "channels.handler.handle", message)
            )
            rows.append(
                {
                    "scenario": invalid["id"],
                    "error": error,
                    "error_code": error["code"],
                    "callback_delta_expected": 0,
                    "callback_delta": len(native.calls) - before,
                }
            )
        for filename in (None, "", "missing"):
            message = copy.deepcopy(SPEC["inbound"])
            for attachment in message["attachments"]:
                if filename == "missing":
                    attachment.pop("filename")
                else:
                    attachment["filename"] = filename
            await native.call(owner, "channels.handler.handle", message)
            assert [
                item.get("filename")
                for item in native.calls[-1]["message"]["attachments"]
            ] == [None if filename == "missing" else filename] * 4
        bare = copy.deepcopy(SPEC["inbound"])
        bare.pop("attachments")
        await native.call(owner, "channels.handler.handle", bare)
        assert native.calls[-1]["message"].get("attachments", []) == []
        for reply_to in (None, "", "missing"):
            message = copy.deepcopy(SPEC["outbound"])
            if reply_to == "missing":
                message.pop("reply_to")
            else:
                message["reply_to"] = reply_to
            await native.call(owner, "channels.handler.reply", message)
            assert native.calls[-1]["message"].get("reply_to") == (
                None if reply_to == "missing" else reply_to
            )
        await native.stop(owner)
    finally:
        await native.close()
        archive(native, receipt, rows)


async def test_channel_owner_complete_binding_start_states_and_digest(evidence):
    native, owner, receipt = await setup(evidence, "owner-binding")
    rows = []
    try:
        before = len(native.calls)
        inactive = await native.row_error(
            native.call(owner, "channels.handler.handle", SPEC["inbound"])
        )
        rows.append(
            {
                "scenario": "unstarted",
                "error": inactive,
                "callback_delta_expected": 0,
                "callback_delta": len(native.calls) - before,
            }
        )
        wrong = await native.sdk.register_channel_message_handler(
            "other-handler",
            {
                "kind": "channel_message_handler",
                "descriptor_version": 1,
                "handler_id": "other-handler",
            },
            lambda *_: result("channel_reply"),
        )
        before_start = len(native.plugin_calls)
        await native.row_error(
            native.sdk.call(
                "channels.manager.start",
                owner["session"].wire,
                [owner["manager"], wrong.wire],
            )
        )
        assert len(native.plugin_calls) == before_start
        changed_handler = WireHandle(
            owner["handler"].wire.id, "18446744073709551615", "extension"
        )
        await native.row_error(
            native.sdk.call(
                "channels.manager.start",
                owner["session"].wire,
                [owner["manager"], changed_handler],
            )
        )
        assert len(native.plugin_calls) == before_start
        batch = (
            await native.sdk.call("channels.manager.open", owner["session"].wire, [])
        )["resource"]
        batch = WireHandle.from_dict(batch) if isinstance(batch, dict) else batch
        batch_starts = []

        async def batch_plugin(call, _cancel):
            if call["invocation"]["operation"] == "channel_start":
                batch_starts.append(call["invocation"])
            return result(call["invocation"]["operation"])

        for name, bound in (("batch-a", owner["handler"]), ("batch-b", wrong)):
            registration = await native.sdk.register_channel_plugin(
                name,
                {
                    "kind": "channel_plugin",
                    "descriptor_version": 1,
                    "channel_id": name,
                    "label": name,
                    "handler_id": SPEC["handler_id"]
                    if bound is owner["handler"]
                    else "other-handler",
                    "capabilities": {
                        "chat_types": ["direct"],
                        "supports_media": False,
                        "supports_threads": False,
                    },
                },
                batch_plugin,
            )
            await native.sdk.call(
                "channels.plugin.register",
                owner["session"].wire,
                [batch, registration.wire, bound.wire],
            )
        await native.row_error(
            native.sdk.call(
                "channels.manager.start",
                owner["session"].wire,
                [batch, owner["handler"].wire],
            )
        )
        assert batch_starts == []
        rows.append(
            {
                "scenario": "batch-original-binding-before-all-effects",
                "plugin_start_effects": 0,
            }
        )
        native.start_probe = True
        native.start_fail = True
        assert (await native.start(owner))["failures"] == "1"
        await native.row_error(
            native.call(owner, "channels.handler.handle", SPEC["inbound"])
        )
        assert (await native.start(owner))["failures"] == "0"
        native.start_probe = False
        await native.row_error(
            native.sdk.call(
                "channels.handler.handle",
                native.sessions[1].wire,
                [owner["manager"], owner["channel"], business_wire(SPEC["inbound"])],
            )
        )
        manager_b = (
            await native.sdk.call("channels.manager.open", native.sessions[1].wire, [])
        )["resource"]
        manager_b = (
            WireHandle.from_dict(manager_b)
            if isinstance(manager_b, dict)
            else manager_b
        )
        second = {**owner, "session": native.sessions[1], "manager": manager_b}
        assert (
            await native.sdk.call(
                "channels.plugin.register",
                second["session"].wire,
                [manager_b, owner["plugin"].wire, owner["handler"].wire],
            )
        )["registered"]
        assert (await native.start(second))["failures"] == "0"
        assert (
            normalize(
                await native.call(
                    second,
                    "channels.handler.handle",
                    {**SPEC["inbound"], "message_id": "second-owner-message"},
                )
            )
            == SPEC["outbound"]
        )
        assert native.calls[-1]["message"]["message_id"] == "second-owner-message"
        bad_manager = WireHandle(owner["manager"].id, "0", owner["manager"].kind)
        stale = await native.row_error(
            native.sdk.call(
                "channels.handler.handle",
                owner["session"].wire,
                [bad_manager, owner["channel"], business_wire(SPEC["inbound"])],
            ),
            "stale_handle",
        )
        stream = await native.call(
            owner, "channels.handler.handle_stream", SPEC["inbound"]
        )
        await native.row_error(
            native.sdk.call("channels.stream.next", native.sessions[1].wire, [stream])
        )
        stale_stream = WireHandle(stream.id, "0", "stream")
        await native.row_error(
            native.sdk.call(
                "channels.stream.next", owner["session"].wire, [stale_stream]
            ),
            "stale_handle",
        )
        await native.row_error(
            native.sdk.call("workflow.stream.next", owner["session"].wire, [stream])
        )
        await native.sdk.call("channels.stream.close", owner["session"].wire, [stream])
        digest_error = await native.row_error(
            native.sdk.request(
                "_echo_agent/channels/op",
                {
                    "operation": "channels.handler.handle",
                    "signature_digest": "sha256:" + "0" * 64,
                    "handle": owner["session"].wire.to_dict(),
                    "arguments": [
                        to_wire(owner["manager"]),
                        to_wire(owner["channel"]),
                        business_wire(SPEC["inbound"]),
                    ],
                },
            ),
            "invalid_value",
        )
        rows.append(
            {
                "scenario": "owner-generation-binding-start-digest",
                "stale": stale,
                "digest": digest_error,
                "original_entry_survived": normalize(
                    await native.call(owner, "channels.handler.handle", SPEC["inbound"])
                )
                == SPEC["outbound"],
            }
        )
        await native.stop(owner)
        assert (
            normalize(
                await native.call(second, "channels.handler.handle", SPEC["inbound"])
            )
            == SPEC["outbound"]
        )
        await native.start(owner)
        native.modes["handle-error"] = "error"
        handle_failure = await native.row_error(
            native.call(
                owner,
                "channels.handler.handle",
                {**SPEC["inbound"], "message_id": "handle-error"},
            ),
            "framework_error",
        )
        assert (
            SPEC["callback_behaviors"]["fail-handle"]["error"]["message"]
            in handle_failure["message"]
        )
        assert (
            normalize(
                await native.call(owner, "channels.handler.handle", SPEC["inbound"])
            )
            == SPEC["outbound"]
        )
        rows.append(
            {
                "scenario": "fail-handle",
                "error": handle_failure,
                "error_code": handle_failure["code"],
                "callback_delta": 1,
            }
        )
        await native.stop(second)
        await wrong.close()
    finally:
        await native.close()
        archive(native, receipt, rows)


async def test_channel_cancel_keeps_peer_full_buffer_close_and_late_finally(evidence):
    native, owner, receipt = await setup(evidence, "lifecycle")
    rows = []
    try:
        await native.start(owner)
        stream_a = await native.call(
            owner, "channels.handler.handle_stream", native.park("park")
        )
        await asyncio.wait_for(native.started["park"].wait(), 5)
        assert (
            await native.sdk.call(
                "channels.stream.cancel", owner["session"].wire, [stream_a]
            )
            is True
        )
        terminal = asyncio.create_task(
            native.sdk.call("channels.stream.next", owner["session"].wire, [stream_a])
        )
        await asyncio.wait_for(native.cancelled["park"].wait(), 5)
        assert not terminal.done()
        assert (
            normalize(
                await native.call(owner, "channels.handler.handle", SPEC["inbound"])
            )
            == SPEC["outbound"]
        )
        native.release["park"].set()
        assert event_parts(await terminal)[0] == "cancelled"
        stream = await native.call(
            owner, "channels.handler.handle_stream", native.park("full")
        )
        await asyncio.wait_for(native.started["full"].wait(), 5)
        closing = asyncio.create_task(
            native.sdk.call("channels.stream.close", owner["session"].wire, [stream])
        )
        await asyncio.wait_for(native.cancelled["full"].wait(), 5)
        assert not closing.done()
        native.release["full"].set()
        await closing
        await native.row_error(
            native.sdk.call("channels.stream.next", owner["session"].wire, [stream]),
            "closed_handle",
        )
        message = native.park("caller")
        pending = asyncio.create_task(
            native.call(owner, "channels.handler.handle", message)
        )
        await asyncio.wait_for(native.started["caller"].wait(), 5)
        pending.cancel()
        with pytest.raises(asyncio.CancelledError):
            await pending
        stop = asyncio.create_task(native.stop(owner))
        await asyncio.wait_for(native.cancelled["caller"].wait(), 5)
        assert not stop.done()
        native.release["caller"].set()
        await stop
        assert (await native.start(owner))["failures"] == "0"
        native.modes["error"] = "error"
        failure = await native.row_error(
            native.call(
                owner, "channels.handler.reply", {**SPEC["outbound"], "text": "error"}
            ),
            "framework_error",
        )
        assert (
            SPEC["callback_behaviors"]["fail-reply"]["error"]["message"]
            in failure["message"]
        )
        stream = await native.call(
            owner, "channels.handler.handle_stream", native.park("resource")
        )
        await asyncio.wait_for(native.started["resource"].wait(), 5)
        closing = asyncio.create_task(native.close_manager(owner))
        await asyncio.wait_for(native.cancelled["resource"].wait(), 5)
        assert not closing.done()
        native.release["resource"].set()
        await closing
        await native.row_error(
            native.call(owner, "channels.handler.handle", SPEC["inbound"]),
            "closed_handle",
        )
        rows.append(
            {
                "scenario": "cancel-peer-full-close-caller-drop-resource-close",
                "cancel_peer_survived": True,
                "cleanup_waited": True,
                "reply_error": failure,
                "callback_finally": native.finally_count,
                "callbacks": len(native.calls),
            }
        )
        assert native.finally_count == len(native.calls)
    finally:
        await native.close()
        archive(native, receipt, rows)
