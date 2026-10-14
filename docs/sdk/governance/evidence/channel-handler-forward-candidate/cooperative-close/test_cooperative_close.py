"""Real public Channel stream close must retain an unfinished callback owner."""
import asyncio
import importlib.util
import json
import os
from pathlib import Path

import pytest

SDK = Path(__file__).resolve().parents[6]
loader = importlib.util.spec_from_file_location("forward_fixture", SDK / "sdks/python/tests/test_channel_handler_forward_host.py")
fixture = importlib.util.module_from_spec(loader)
loader.loader.exec_module(fixture)


@pytest.mark.asyncio
async def test_channel_close_deadline_retains_actual_callback_and_address():
    work = Path(os.environ["ECHO_AGENT_SDK_COOPERATIVE_CLOSE_ROOT"])
    work.mkdir(parents=True, exist_ok=True)
    process = await asyncio.create_subprocess_exec("python3", str(fixture.SHARED / "mcp_channel_config_fixture.py"), str(work))
    assert await process.wait() == 0
    config = json.loads((work / "host.json").read_text())
    config["sdk_profile"]["limits"]["shutdown_timeout_secs"] = 1
    (work / "host.json").write_text(json.dumps(config))
    sdk = await fixture.EchoAgentClient.spawn(fixture.HOST, "--config", str(work / "host.json"))
    native = fixture.Native(sdk, work)
    native.agent = await sdk.create_agent()
    session = await native.agent.create_session()
    manager = (await sdk.call("channels.manager.open", session.wire, []))["resource"]
    owner = await native.register(session, manager)
    message = native.park("park")
    stream = None
    try:
        await native.start(owner)
        stream = await native.call(owner, "channels.handler.handle_stream", message)
        stream = fixture.WireHandle.from_dict(stream) if isinstance(stream, dict) else stream
        await asyncio.wait_for(native.started["park"].wait(), 2)
        before = native.finally_count
        close_result, error_code = None, None
        try:
            close_result = await asyncio.wait_for(sdk.call("channels.stream.close", session.wire, [stream]), 2)
        except fixture.EchoAgentError as error:
            error_code = error.code
            assert error.retryable == "after_delay"
        await asyncio.wait_for(native.cancelled["park"].wait(), 2)
        observed = {"close_result": close_result, "error_code": error_code, "callback_finally_before": before, "callback_finally_after_close": native.finally_count, "original_callback_still_held": not native.release["park"].is_set()}
        (work / "observed.json").write_text(json.dumps(observed, indent=2))
        assert error_code == "framework_error", "close succeeded while the original Channel callback was still held"
        assert native.finally_count == before
        native.release["park"].set()
        deadline = asyncio.get_running_loop().time() + 2
        while native.finally_count == before:
            assert asyncio.get_running_loop().time() < deadline
            await asyncio.sleep(0.01)
        assert await sdk.call("channels.stream.close", session.wire, [stream]) is True
        await native.row_error(sdk.call("channels.stream.next", session.wire, [stream]), "closed_handle")
    finally:
        native.release["park"].set()
        await asyncio.sleep(0.05)
        if stream:
            try:
                await sdk.call("channels.stream.close", session.wire, [stream])
            except fixture.EchoAgentError:
                pass
        await sdk.close()


@pytest.mark.asyncio
async def test_channel_bad_chunk_retains_original_failure_and_cleanup_owner():
    work = Path(os.environ["ECHO_AGENT_SDK_COOPERATIVE_CLOSE_ROOT"]) / "bad-chunk"
    work.mkdir(parents=True, exist_ok=True)
    process = await asyncio.create_subprocess_exec("python3", str(fixture.SHARED / "mcp_channel_config_fixture.py"), str(work))
    assert await process.wait() == 0
    config = json.loads((work / "host.json").read_text())
    config["sdk_profile"]["limits"]["shutdown_timeout_secs"] = 1
    (work / "host.json").write_text(json.dumps(config))
    sdk = await fixture.EchoAgentClient.spawn(fixture.HOST, "--config", str(work / "host.json"))
    release, cancelled = asyncio.Event(), asyncio.Event()
    finally_count = 0
    stream = None
    try:
        async def handler(call, cancel):
            nonlocal finally_count
            writer = sdk.stream_writer(fixture.WireHandle.from_dict(call["stream"]))
            message = {**fixture.SPEC["outbound"], "attachments": [{"kind":"image", "data_base64":"!"}]}
            try:
                await writer.chunk({"kind":"channel", "value":message})
                await cancel.wait()
                cancelled.set()
                await release.wait()
                return {"outcome":"stream", "stream":call["stream"]}
            finally:
                finally_count += 1
        reg = await sdk.register_channel_message_handler("bad-handler", {"kind":"channel_message_handler", "descriptor_version":1, "handler_id":"bad-handler"}, handler)
        async def plugin(call, cancel):
            return fixture.result(call["invocation"]["operation"])
        plugin_reg = await sdk.register_channel_plugin("bad-plugin", {"kind":"channel_plugin", "descriptor_version":1, "channel_id":fixture.SPEC["channel_id"], "label":"badchunk", "handler_id":"bad-handler", "capabilities":{"chat_types":["direct"],"supports_media":True,"supports_threads":False}}, plugin)
        agent = await sdk.create_agent()
        session = await agent.create_session()
        manager = (await sdk.call("channels.manager.open", session.wire, []))["resource"]
        await sdk.call("channels.plugin.register", session.wire, [manager,plugin_reg.wire,reg.wire])
        await sdk.call("channels.manager.start", session.wire, [manager,reg.wire])
        stream = await sdk.call("channels.handler.handle_stream", session.wire, [manager,fixture.SPEC["channel_id"],fixture.business_wire(fixture.SPEC["inbound"])])
        stream = fixture.WireHandle.from_dict(stream) if isinstance(stream,dict) else stream
        response, error = None, None
        try:
            response = await asyncio.wait_for(sdk.call("channels.stream.next",session.wire,[stream]),2)
        except fixture.EchoAgentError as caught:
            error = caught
        await asyncio.wait_for(cancelled.wait(),2)
        (work/"observed.json").write_text(json.dumps({"early_response":response,"error_code":error.code if error else None,"finally_count":finally_count,"callback_held":not release.is_set()},indent=2))
        assert error and error.code == "framework_error" and error.retryable == "after_delay", "stream returned failed/closed before the original bad-chunk callback finished"
        assert finally_count == 0
        release.set()
        await asyncio.sleep(0.05)
        terminal = await sdk.call("channels.stream.next",session.wire,[stream])
        variant, fields = fixture.event_parts(terminal)
        assert variant == "failed"
        assert "base64" in str(fields["value"])
        assert finally_count == 1
    finally:
        release.set()
        await asyncio.sleep(0.05)
        if stream:
            try:
                await sdk.call("channels.stream.close",session.wire,[stream])
            except fixture.EchoAgentError:
                pass
        await sdk.close()
