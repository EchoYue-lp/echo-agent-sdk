"""Deterministic files and oracle only; native consumers make all SDK calls."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
from pathlib import Path

ORIGINAL = "one🙂\n".encode()
APPENDED = ORIGINAL + b"x"
REPLACEMENT = "two🙂\n".encode() + b"!"
PREFIX = "echo_core::utils::fs::"


def prepare(work):
    work.mkdir(parents=True, exist_ok=True)
    (work / "guarded.txt").write_bytes(ORIGINAL)
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "fixture",
                "base_url": "http://127.0.0.1:1/v1/chat/completions",
                "api_protocol": "chat_completions",
            },
            "agent": {
                "name": "identity-proof",
                "enable_tools": True,
                "enable_memory": False,
            },
        },
        "sdk_profile": {"state_root": str(work / "state")},
    }
    (work / "host.json").write_text(json.dumps(config))
    (work / "scenario.json").write_text(
        json.dumps(
            {
                "initial_base64": base64.b64encode(ORIGINAL).decode(),
                "appended_base64": base64.b64encode(APPENDED).decode(),
                "replacement_base64": base64.b64encode(REPLACEMENT).decode(),
                "initial_length": len(ORIGINAL),
                "replaced_length": len(REPLACEMENT),
            },
            indent=2,
        )
    )
    assert len(APPENDED) == len(REPLACEMENT)


def snapshot(work):
    result = {}
    for name in ["guarded.txt", "original-backup.txt"]:
        path = work / name
        if path.exists():
            data = path.read_bytes()
            stat = path.stat()
            result[name] = {
                "sha256": hashlib.sha256(data).hexdigest(),
                "base64": base64.b64encode(data).decode(),
                "byte_length": len(data),
                "device": str(stat.st_dev),
                "inode": str(stat.st_ino),
            }
        else:
            result[name] = None
    return result


def verify(work, data):
    rows = data["rows"]
    assert [row["id"] for row in rows] == [
        "open-original",
        "length-original",
        "append-original",
        "length-appended",
        "replace-same-length",
        "stale-length",
        "stale-append",
        "old-descriptor-length",
        "open-replacement",
        "length-replacement",
        "append-replacement",
    ]
    byid = {row["id"]: row for row in rows}
    assert int(byid["length-original"]["value"]) == len(ORIGINAL)
    assert byid["append-original"]["value"] is None
    assert int(byid["length-appended"]["value"]) == len(APPENDED)
    swap = byid["replace-same-length"]["after"]
    assert (
        swap["guarded.txt"]["byte_length"]
        == swap["original-backup.txt"]["byte_length"]
        == len(APPENDED)
    )
    assert (swap["guarded.txt"]["device"], swap["guarded.txt"]["inode"]) != (
        swap["original-backup.txt"]["device"],
        swap["original-backup.txt"]["inode"],
    )
    for name in ["stale-length", "stale-append"]:
        row = byid[name]
        assert row["before"] == row["after"] == swap
        error = row["error"]
        assert (
            error["code"] == "framework_error"
            and error["operation"] == row["operation"]
            and error["retryable"] == "never"
        )
        assert error["handle"] is None and error["details"] is None
        assert (
            "existing regular file identity changed; verified reopen required:"
            in error["message"]
        )
        assert "result" not in row and "value" not in row
    assert int(byid["old-descriptor-length"]["value"]) == len(APPENDED)
    assert int(byid["length-replacement"]["value"]) == len(REPLACEMENT)
    assert byid["append-replacement"]["value"] is None
    final = snapshot(work)
    assert final["original-backup.txt"]["base64"] == base64.b64encode(APPENDED).decode()
    assert (
        final["guarded.txt"]["base64"] == base64.b64encode(REPLACEMENT + b"z").decode()
    )
    assert data["closed"] is True
    return {
        "native_calls": 10,
        "same_length_replacement_rejections": 2,
        "both_files_unchanged_on_each_rejection": True,
        "new_guard_can_append": True,
        "authority": "original framework file descriptor identity, not content snapshot",
        "platform_scope": "actual native filesystem identity observed on current host",
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--verify", type=Path)
    args = parser.parse_args()
    if args.verify:
        print(json.dumps(verify(args.work, json.loads(args.verify.read_text()))))
    else:
        prepare(args.work)


if __name__ == "__main__":
    main()
