"""Both #120 recovery scenarios use the shared model and a real Host process."""

import asyncio
import json
import os
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
TESTING = Path(__file__).resolve().parents[2] / "shared/testing"


async def wait_for(predicate) -> None:
    loop = asyncio.get_running_loop()
    deadline = loop.time() + 10
    while not predicate():
        assert loop.time() < deadline, "real Host scenario did not reach its barrier"
        await asyncio.sleep(0.01)


def notifications(work: Path) -> list[dict]:
    text = (work / "notifications.jsonl").read_text(encoding="utf-8")
    lines = text.splitlines()
    # A final incomplete line belongs to a concurrent pipe write, not a frame.
    if text and not text.endswith("\n"):
        lines = lines[:-1]
    return [
        value
        for line in lines
        if (value := json.loads(line)).get("method")
        in {"_echo_agent/event", "_echo_agent/gap"}
    ]


def sequence(value: dict) -> int:
    params = value["params"]
    return int(
        params["gap"]["from_sequence"]
        if "gap" in params
        else params["envelope"]["sequence"]
    )


@pytest.mark.skipif(
    not HOST, reason="ECHO_AGENT_SDK_HOST is required for real Host E2E"
)
@pytest.mark.parametrize("mode", ["window", "oversized"])
@pytest.mark.parametrize("consume", [False, True])
@pytest.mark.asyncio
async def test_gap_ack_snapshot_continuation(
    mode: str, consume: bool, tmp_path: Path
) -> None:
    model = await asyncio.create_subprocess_exec(
        "python3",
        str(TESTING / "gap_ack_fixture.py"),
        "--work",
        str(tmp_path),
        "--mode",
        mode,
    )
    try:
        await wait_for(lambda: (tmp_path / "ready").exists())
        async with await EchoAgentClient.spawn(
            "python3",
            str(TESTING / "gap_ack_host_proxy.py"),
            HOST,
            str(tmp_path / "host.json"),
            env={
                **os.environ,
                "NO_PROXY": "127.0.0.1,localhost",
                "no_proxy": "127.0.0.1,localhost",
                "RUST_LOG": "error",
            },
        ) as sdk:
            agent = await sdk.create_agent()
            session = await agent.create_session()
            run = await session.start_run("gap ACK test 中文🌊")
            await wait_for(lambda: (tmp_path / "parked").exists())
            before = notifications(tmp_path)
            gap = next(
                value for value in before if value["method"] == "_echo_agent/gap"
            )
            watermark = int(gap["params"]["gap"]["snapshot_watermark"])
            assert len(before) == (2 if mode == "window" else 1)
            iterator = None
            if consume:
                iterator = run.events
                item = await asyncio.wait_for(anext(iterator), 5)
                if mode == "window":
                    assert item["envelope"]["sequence"] == "1"
                    item = await asyncio.wait_for(anext(iterator), 5)
                assert item["gap"]["snapshot_watermark"] == str(watermark)
            future_ack = {
                "ack": {
                    "stream": run.stream.to_dict(),
                    "last_processed_sequence": str(watermark + 1),
                }
            }
            await sdk.notify("echo_agent/event/ack", future_ack)
            snapshot = await run.get()
            assert len(notifications(tmp_path)) == len(before)
            assert int(snapshot["last_sequence"]) >= watermark
            replay = await run.replay(str(watermark), 64)
            assert replay["events"]
            assert all(int(event["sequence"]) > watermark for event in replay["events"])
            ack = {
                "ack": {
                    "stream": run.stream.to_dict(),
                    "last_processed_sequence": str(watermark),
                }
            }
            await sdk.notify("echo_agent/event/ack", ack)
            if iterator:
                resumed = await asyncio.wait_for(anext(iterator), 5)
                resumed_sequence = (
                    resumed["gap"]["from_sequence"]
                    if "gap" in resumed
                    else resumed["envelope"]["sequence"]
                )
                assert int(resumed_sequence) > watermark
            await wait_for(lambda: len(notifications(tmp_path)) > len(before))
            assert all(
                sequence(value) > watermark
                for value in notifications(tmp_path)[len(before) :]
            )
            announced = len(notifications(tmp_path))
            await sdk.notify("echo_agent/event/ack", ack)
            await run.get()
            assert len(notifications(tmp_path)) == announced
            (tmp_path / "release").touch()
            assert (await run.wait(timeout=5))["settled"]
            assert (await run.get())["status"] == "completed"
            continuation = await run.replay(
                replay["next_cursor"]["last_processed_sequence"], 64
            )
            assert continuation["events"]
            assert all(
                int(event["sequence"]) > watermark for event in continuation["events"]
            )
            assert all(
                sequence(value) > watermark
                for value in notifications(tmp_path)[len(before) :]
            )
            if iterator:
                await iterator.aclose()
            await session.close()
            await agent.close()
    finally:
        model.terminate()
        await asyncio.wait_for(model.wait(), timeout=5)


@pytest.mark.skipif(
    not HOST, reason="ECHO_AGENT_SDK_HOST is required for real Host E2E"
)
@pytest.mark.parametrize("page_size", [1, 64])
@pytest.mark.asyncio
async def test_replay_consumed_cursor_ack(page_size: int, tmp_path: Path) -> None:
    model = await asyncio.create_subprocess_exec(
        "python3",
        str(TESTING / "gap_ack_fixture.py"),
        "--work",
        str(tmp_path),
        "--mode",
        "window",
    )
    try:
        await wait_for(lambda: (tmp_path / "ready").exists())
        async with await EchoAgentClient.spawn(
            "python3",
            str(TESTING / "gap_ack_host_proxy.py"),
            HOST,
            str(tmp_path / "host.json"),
            env={
                **os.environ,
                "NO_PROXY": "127.0.0.1,localhost",
                "no_proxy": "127.0.0.1,localhost",
                "RUST_LOG": "error",
            },
        ) as sdk:
            agent = await sdk.create_agent()
            session = await agent.create_session()
            run = await session.start_run("replay consumed cursor 中文🌊")
            await wait_for(lambda: (tmp_path / "parked").exists())
            before = notifications(tmp_path)
            gap = next(
                value for value in before if value["method"] == "_echo_agent/gap"
            )
            parked_tail = int((await run.get())["last_sequence"])
            cursor = 0
            consumed: dict[int, str] = {}
            pages = 0
            while cursor < parked_tail:
                page = await run.replay(str(cursor), page_size)
                assert page["events"]
                for event in page["events"]:
                    assert int(event["sequence"]) == cursor + 1
                    cursor = int(event["sequence"])
                    consumed[cursor] = event["event_id"]
                assert page["next_cursor"]["last_processed_sequence"] == str(cursor)
                pages += 1
            assert cursor > int(gap["params"]["gap"]["snapshot_watermark"])
            assert (pages > 1) == (page_size == 1)
            empty = await run.replay(str(parked_tail + 100), page_size)
            assert not empty["events"]
            await sdk.notify(
                "echo_agent/event/ack",
                {
                    "ack": {
                        "stream": run.stream.to_dict(),
                        "last_processed_sequence": empty["next_cursor"][
                            "last_processed_sequence"
                        ],
                    }
                },
            )
            await run.get()
            assert len(notifications(tmp_path)) == len(before)
            await sdk.notify(
                "echo_agent/event/ack",
                {
                    "ack": {
                        "stream": run.stream.to_dict(),
                        "last_processed_sequence": str(cursor),
                    }
                },
            )
            await run.get()
            assert len(notifications(tmp_path)) == len(before)
            (tmp_path / "release").touch()
            await wait_for(lambda: len(notifications(tmp_path)) > len(before))
            assert all(
                sequence(value) > cursor
                for value in notifications(tmp_path)[len(before) :]
            )
            assert (await run.wait(timeout=5))["settled"]
            terminal_tail = int((await run.get())["last_sequence"])
            while cursor < terminal_tail:
                page = await run.replay(str(cursor), page_size)
                assert page["events"]
                for event in page["events"]:
                    assert int(event["sequence"]) == cursor + 1
                    cursor = int(event["sequence"])
                    assert cursor not in consumed
                    consumed[cursor] = event["event_id"]
            await sdk.notify(
                "echo_agent/event/ack",
                {
                    "ack": {
                        "stream": run.stream.to_dict(),
                        "last_processed_sequence": str(cursor),
                    }
                },
            )
            assert len(consumed) == terminal_tail
            all_events = (await run.replay("0", 64))["events"]
            assert (
                sum(
                    event["payload"]["event_type"] == "final_answer"
                    for event in all_events
                )
                == 1
            )
            assert [event["event_id"] for event in all_events] == list(
                consumed.values()
            )
            await session.close()
            await agent.close()
    finally:
        model.terminate()
        await asyncio.wait_for(model.wait(), timeout=5)


@pytest.mark.parametrize("window", [1, 2])
@pytest.mark.asyncio
async def test_paused_consumer_external_replay_ack_coalesces_covered_tail(
    window, tmp_path
):
    model = await asyncio.create_subprocess_exec(
        "python3",
        str(TESTING / "gap_ack_fixture.py"),
        "--work",
        str(tmp_path),
        "--mode",
        "window",
    )
    try:
        await wait_for(lambda: (tmp_path / "ready").exists())
        config_path = tmp_path / "host.json"
        config = json.loads(config_path.read_text())
        config["sdk_profile"]["limits"]["max_outstanding_live_events"] = window
        config_path.write_text(json.dumps(config))
        async with await EchoAgentClient.spawn(
            "python3",
            str(TESTING / "gap_ack_host_proxy.py"),
            HOST,
            str(config_path),
            max_buffered_events=window,
        ) as sdk:
            agent = await sdk.create_agent()
            session = await agent.create_session()
            run = await session.start_run("coalesced external recovery")
            await wait_for(lambda: (tmp_path / "parked").exists())
            first = next(
                frame
                for frame in notifications(tmp_path)
                if frame["method"] == "_echo_agent/gap"
            )["params"]["gap"]
            covered = {}
            cursor = "0"
            first_page = await run.replay(cursor, 64)
            for event in first_page["events"]:
                if int(event["sequence"]) <= int(first["snapshot_watermark"]):
                    covered[event["sequence"]] = event["event_id"]
                    cursor = event["sequence"]
            assert cursor == first["snapshot_watermark"]
            await sdk.notify(
                "echo_agent/event/ack",
                {
                    "ack": {
                        "stream": run.stream.to_dict(),
                        "last_processed_sequence": cursor,
                    }
                },
            )
            (tmp_path / "release").touch()
            await wait_for(
                lambda: (
                    len(
                        [
                            frame
                            for frame in notifications(tmp_path)
                            if frame["method"] == "_echo_agent/gap"
                        ]
                    )
                    >= 2
                )
            )
            # The byte tee captures before the official client's observer runs.
            # Wait for that actual receive fact, not merely the proxy's stdout.
            next_watermark = int(
                next(
                    frame
                    for frame in reversed(notifications(tmp_path))
                    if frame["method"] == "_echo_agent/gap"
                )["params"]["gap"]["snapshot_watermark"]
            )
            await wait_for(
                lambda: sdk._events[run.stream.id].last_gap_watermark >= next_watermark
            )
            iterator = run.events
            item = await asyncio.wait_for(anext(iterator), 5)
            assert "gap" in item
            assert item["gap"]["from_sequence"] == str(int(cursor) + 1)
            assert int(item["gap"]["snapshot_watermark"]) > int(cursor)
            await iterator.aclose()
            parked = await run.get()
            page = await run.replay(cursor, 64)
            for event in page["events"]:
                assert event["sequence"] not in covered
                covered[event["sequence"]] = event["event_id"]
            cursor = page["next_cursor"]["last_processed_sequence"]
            await sdk.notify(
                "echo_agent/event/ack",
                {
                    "ack": {
                        "stream": run.stream.to_dict(),
                        "last_processed_sequence": cursor,
                    }
                },
            )
            assert cursor == parked["last_sequence"]
            (tmp_path / "release").touch()
            assert (await run.wait(timeout=10))["settled"]
            continuation = await run.replay(cursor, 64)
            for event in continuation["events"]:
                assert event["sequence"] not in covered
                covered[event["sequence"]] = event["event_id"]
            final = await run.replay("0", 64)
            assert [covered[key] for key in sorted(covered, key=int)] == [
                event["event_id"] for event in final["events"]
            ]
            assert (
                sum(
                    event["payload"]["event_type"] == "final_answer"
                    for event in final["events"]
                )
                == 1
            )
            await session.close()
            await agent.close()
    finally:
        if model.returncode is None:
            model.terminate()
        await model.wait()


@pytest.mark.skipif(
    not HOST, reason="ECHO_AGENT_SDK_HOST is required for real Host E2E"
)
@pytest.mark.asyncio
async def test_replayed_terminal_ack_ends_live_iterator(tmp_path: Path) -> None:
    model = await asyncio.create_subprocess_exec(
        "python3",
        str(TESTING / "gap_ack_fixture.py"),
        "--work",
        str(tmp_path),
        "--mode",
        "oversized",
    )
    try:
        await wait_for(lambda: (tmp_path / "ready").exists())
        async with await EchoAgentClient.spawn(
            "python3",
            str(TESTING / "gap_ack_host_proxy.py"),
            HOST,
            str(tmp_path / "host.json"),
        ) as sdk:
            agent = await sdk.create_agent()
            session = await agent.create_session()
            run = await session.start_run("replayed terminal EOF 中文🌊")
            await wait_for(lambda: (tmp_path / "parked").exists())
            iterator = run.events
            assert "gap" in await asyncio.wait_for(anext(iterator), 5)
            (tmp_path / "release").touch()
            assert (await run.wait(timeout=10))["settled"]
            page = await run.replay("0", 64)
            assert (
                sum(
                    event["payload"]["event_type"] == "final_answer"
                    for event in page["events"]
                )
                == 1
            )
            tail = page["next_cursor"]["last_processed_sequence"]
            assert tail == page["events"][-1]["sequence"]
            announced = len(notifications(tmp_path))
            eof = asyncio.create_task(anext(iterator))
            await run.get()
            assert not eof.done(), "replay fetch alone is not terminal consumption"
            future = {
                "ack": {
                    "stream": run.stream.to_dict(),
                    "last_processed_sequence": str(int(tail) + 1),
                }
            }
            await sdk.notify("echo_agent/event/ack", future)
            await run.get()
            assert not eof.done(), "uncovered future ACK must not end the feed"
            ack = {
                "ack": {"stream": run.stream.to_dict(), "last_processed_sequence": tail}
            }
            await sdk.notify("echo_agent/event/ack", ack)
            await run.get()
            assert len(notifications(tmp_path)) == announced
            with pytest.raises(StopAsyncIteration):
                await asyncio.wait_for(eof, 2)
            with pytest.raises(StopAsyncIteration):
                await anext(iterator)
            await sdk.notify("echo_agent/event/ack", ack)
            await run.get()
            assert len(notifications(tmp_path)) == announced
            await session.close()
            await agent.close()
    finally:
        if model.returncode is None:
            model.terminate()
        await asyncio.wait_for(model.wait(), 5)


@pytest.mark.skipif(not HOST, reason="ECHO_AGENT_SDK_HOST is required")
@pytest.mark.parametrize("success", [False, True])
@pytest.mark.asyncio
async def test_real_host_gap_before_official_ack_completion(success, tmp_path):
    """The genuine Host resumes while the genuine ACP write is still pending."""
    model = await asyncio.create_subprocess_exec(
        "python3",
        str(TESTING / "gap_ack_fixture.py"),
        "--work",
        str(tmp_path),
        "--mode",
        "oversized",
    )
    try:
        await wait_for(lambda: (tmp_path / "ready").exists())
        async with await EchoAgentClient.spawn(
            "python3",
            str(TESTING / "gap_ack_host_proxy.py"),
            HOST,
            str(tmp_path / "host.json"),
        ) as sdk:
            agent = await sdk.create_agent()
            session = await agent.create_session()
            run = await session.start_run("pending ACK real continuation 中文🌊")
            await wait_for(lambda: (tmp_path / "parked").exists())
            await wait_for(lambda: run.stream.id in sdk._events)
            page = await run.replay("0", 64)
            cursor = int(page["next_cursor"]["last_processed_sequence"])
            feed = sdk._events[run.stream.id]
            old_sequence = feed.last_sequence
            assert cursor > old_sequence
            before = len(notifications(tmp_path))
            native = sdk.connection.ext_notification
            entered = asyncio.Event()
            completion = asyncio.get_running_loop().create_future()

            async def gated_notification(method, params):
                await native(method, params)
                if method == "echo_agent/event/ack":
                    entered.set()
                    await completion

            sdk.connection.ext_notification = gated_notification
            task = asyncio.create_task(
                sdk.notify(
                    "echo_agent/event/ack",
                    {
                        "ack": {
                            "stream": run.stream.to_dict(),
                            "last_processed_sequence": str(cursor),
                        }
                    },
                )
            )
            try:
                await asyncio.wait_for(entered.wait(), 5)
                assert run.stream not in sdk._event_cursors
                (tmp_path / "release").touch()
                await wait_for(lambda: len(notifications(tmp_path)) > before)
                next_gap = notifications(tmp_path)[before]["params"]["gap"]
                assert int(next_gap["from_sequence"]) == cursor + 1
                await wait_for(
                    lambda: feed.last_sequence == int(next_gap["snapshot_watermark"])
                )
                assert run.stream not in sdk._event_cursors
                assert not feed.queue._closed
                if success:
                    completion.set_result(None)
                    await task
                    assert sdk._event_cursors[run.stream] == cursor
                else:
                    completion.set_exception(
                        ConnectionError("write outcome unknown after real peer gap")
                    )
                    with pytest.raises(EchoAgentError):
                        await task
                    assert run.stream not in sdk._event_cursors
                sdk.connection.ext_notification = native
                item = await asyncio.wait_for(anext(run.events), 5)
                assert "gap" in item
                assert int(item["gap"]["from_sequence"]) == (
                    cursor + 1 if success else 1
                )
                assert int(item["gap"]["snapshot_watermark"]) > cursor
                assert not feed.queue._closed
                assert not feed.pending_ack_sequences
                assert (await run.wait(timeout=5))["settled"]
            finally:
                sdk.connection.ext_notification = native
                if not completion.done():
                    completion.set_result(None)
                await asyncio.gather(task, return_exceptions=True)
            evidence = os.environ.get("ECHO_AGENT_SDK_RECOVERY_EVIDENCE")
            if evidence:
                (Path(evidence) / f"python-pending-gap-{success}.json").write_text(
                    json.dumps(
                        {
                            "success": success,
                            "replay": page,
                            "ack_cursor": cursor,
                            "prior_sequence": old_sequence,
                            "continued_gap": next_gap,
                            "delivered": item,
                            "notifications": notifications(tmp_path),
                        },
                        indent=2,
                    ),
                    encoding="utf-8",
                )
            await session.close()
            await agent.close()
    finally:
        if model.returncode is None:
            model.terminate()
        await asyncio.wait_for(model.wait(), 5)


@pytest.mark.skipif(not HOST, reason="real Host is required")
@pytest.mark.asyncio
async def test_live_journal_directory_replacement_is_typed_and_session_isolated(
    tmp_path: Path,
) -> None:
    async def action(stream, kind):
        child = await asyncio.create_subprocess_exec(
            "python3",
            str(TESTING / "gap_ack_fixture.py"),
            "--work",
            str(tmp_path),
            "--mode",
            "window",
            "--journal-action",
            kind,
            "--stream-id",
            stream.id,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )
        out, err = await asyncio.wait_for(child.communicate(), 5)
        assert child.returncode == 0, err.decode()
        return json.loads(out)

    async def same_page(first, second, label):
        paths = [tmp_path / f"{label}-{side}.json" for side in ("before", "after")]
        for path, value in zip(paths, (first, second)):
            path.write_text(json.dumps(value), encoding="utf-8")
        child = await asyncio.create_subprocess_exec(
            "python3",
            str(TESTING / "gap_ack_fixture.py"),
            "--work",
            str(tmp_path),
            "--mode",
            "window",
            "--compare-pages",
            *map(str, paths),
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )
        out, err = await asyncio.wait_for(child.communicate(), 5)
        assert child.returncode == 0, err.decode()
        assert json.loads(out)["same_complete_page"]

    model = await asyncio.create_subprocess_exec(
        "python3",
        str(TESTING / "gap_ack_fixture.py"),
        "--work",
        str(tmp_path),
        "--mode",
        "window",
    )
    replaced = False
    run = None
    try:
        await wait_for(lambda: (tmp_path / "ready").exists())
        async with await EchoAgentClient.spawn(
            "python3",
            str(TESTING / "gap_ack_host_proxy.py"),
            HOST,
            str(tmp_path / "host.json"),
            env={
                **os.environ,
                "NO_PROXY": "127.0.0.1,localhost",
                "no_proxy": "127.0.0.1,localhost",
            },
        ) as sdk:
            agent = await sdk.create_agent()
            session = await agent.create_session()
            run = await session.start_run("original journal 中文🌊")
            await wait_for(lambda: (tmp_path / "parked").exists())
            (tmp_path / "release").touch()
            settled = await run.wait()
            assert settled["settled"] and settled["terminal"]["status"] == "completed"
            before = await run.replay("0", 64)
            snapshot = await run.get()
            assert before["events"] and snapshot["receipt"]
            other_agent = await sdk.create_agent()
            other_session = await other_agent.create_session()
            other = await other_session.start_run("independent journal 中文🌊")
            assert (await other.wait())["settled"]
            other_before = await other.replay("0", 64)
            assert other.stream != run.stream and other_before["events"]
            fact = await action(run.stream, "replace")
            replaced = True
            errors = []
            try:
                for _ in range(2):
                    with pytest.raises(EchoAgentError) as caught:
                        await run.replay("0", 64)
                    error = caught.value
                    assert error.code == "replay_unavailable"
                    assert error.retryable == "after_delay"
                    assert error.operation == "_echo_agent/run/replay"
                    assert error.details is None and error.handle is None
                    assert "directory identity changed" in error.message
                    errors.append(
                        {
                            "code": error.code,
                            "retryable": error.retryable,
                            "operation": error.operation,
                            "details": error.details,
                            "handle": error.handle,
                            "message": error.message,
                        }
                    )
                assert await run.get() == snapshot, (
                    "failed replay cannot fabricate a new terminal or receipt"
                )
                await same_page(other_before, await other.replay("0", 64), "other")
            finally:
                restored = await action(run.stream, "restore")
                replaced = False
            assert restored["restored"] and restored["files_unchanged"]
            await same_page(before, await run.replay("0", 64), "restored")
            proof = {
                "stream": run.stream.to_dict(),
                "other_stream": other.stream.to_dict(),
                "errors": errors,
                "directory": fact,
                "restored": restored,
                "receipt_unchanged": True,
                "other_journal_unchanged": True,
                "replay_restored": True,
                "page": before,
            }
            if root := os.environ.get("ECHO_AGENT_SDK_JOURNAL_EVIDENCE"):
                (Path(root) / "python.json").write_text(
                    json.dumps(proof, indent=2), encoding="utf-8"
                )
            await other_session.close()
            await other_agent.close()
            await session.close()
            await agent.close()
    finally:
        if replaced and run is not None:
            await action(run.stream, "restore")
        if model.returncode is None:
            model.terminate()
        await asyncio.wait_for(model.wait(), 5)
