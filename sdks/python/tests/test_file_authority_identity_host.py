"""Native public SDK consumes the original retained FileAuthority identity."""

import base64
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError, wire_utf8_path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "shared/testing"))
import file_authority_identity_fixture as fixture
from file_sha256 import file_sha256

HOST = os.environ.get("ECHO_AGENT_SDK_HOST", "").strip()
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required FileAuthority Host missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]


async def test_same_length_replacement_rejects_stale_guard_without_writing():
    root = Path(__file__).resolve().parents[3] / "target/file-authority-identity"
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-", dir=root))
    fixture.prepare(work)
    rows = []
    async with await EchoAgentClient.spawn(
        HOST, "--config", str(work / "host.json")
    ) as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session(str(work))
        path = wire_utf8_path(str(work / "guarded.txt"))

        async def call(identity, operation, args):
            row = {
                "id": identity,
                "operation": fixture.PREFIX + operation,
                "before": fixture.snapshot(work),
            }
            row["arguments"] = [
                argument.to_dict() if hasattr(argument, "to_dict") else argument
                for argument in args
            ]
            try:
                value = await session.invoke(fixture.PREFIX + operation, args)
                row["value"] = value.to_dict() if hasattr(value, "to_dict") else value
            except EchoAgentError as error:
                row["error"] = {
                    "code": error.code,
                    "message": str(error),
                    "operation": error.operation,
                    "retryable": error.retryable,
                    "handle": error.handle.to_dict() if error.handle else None,
                    "details": error.details,
                }
                value = None
            row["after"] = fixture.snapshot(work)
            rows.append(row)
            return value

        guard = await call("open-original", "open_existing_regular_guard", [path])
        await call("length-original", "matching_existing_regular_len", [path, guard])
        encoded = lambda value: {
            "kind": "bytes",
            "value": {"base64": base64.b64encode(value).decode().rstrip("=")},
        }
        await call(
            "append-original",
            "append_existing_matching",
            [path, guard, len(fixture.ORIGINAL), encoded(b"x"), "sync_data"],
        )
        await call("length-appended", "matching_existing_regular_len", [path, guard])
        before = fixture.snapshot(work)
        (work / "guarded.txt").rename(work / "original-backup.txt")
        (work / "replacement.tmp").write_bytes(fixture.REPLACEMENT)
        (work / "replacement.tmp").replace(work / "guarded.txt")
        rows.append(
            {
                "id": "replace-same-length",
                "before": before,
                "after": fixture.snapshot(work),
            }
        )
        await call("stale-length", "matching_existing_regular_len", [path, guard])
        await call(
            "stale-append",
            "append_existing_matching",
            [path, guard, len(fixture.APPENDED), encoded(b"bad"), "sync_data"],
        )
        await call("old-descriptor-length", "ExistingRegularFileGuard::len", [guard])
        fresh = await call("open-replacement", "open_existing_regular_guard", [path])
        await call("length-replacement", "matching_existing_regular_len", [path, fresh])
        await call(
            "append-replacement",
            "append_existing_matching",
            [path, fresh, len(fixture.REPLACEMENT), encoded(b"z"), "sync_data"],
        )
        await session.close()
        await agent.close()
    data = {
        "host": HOST,
        "host_sha256": file_sha256(HOST),
        "work": str(work),
        "rows": rows,
        "closed": True,
    }
    (work / "results.json").write_text(json.dumps(data, ensure_ascii=False, indent=2))
    print("FileAuthority identity:", fixture.verify(work, data), "evidence:", work)
