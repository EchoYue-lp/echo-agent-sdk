"""Deterministic loopback model for real-Host gap/ACK tests in every SDK.

The first model turn asks for a missing tool, producing durable events. The
second turn parks until the test has recovered its gap; release then produces
new events and a terminal answer. Files coordinate only this fixture model,
never decide Host state. Each scenario gets its own config, journal and HTTP
server. Optional journal actions replace only a test-owned directory and retain
its original inode/data for restoration; no RPC or transport error is simulated.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


def canonical_replay(value):
    """Wire maps are associations; sort complete entry multisets without losing duplicates."""
    if isinstance(value, list):
        return [canonical_replay(item) for item in value]
    if isinstance(value, dict):
        result = {key: canonical_replay(item) for key, item in value.items()}
        if result.get("kind") == "map" and isinstance(result.get("value"), list):
            result["value"] = sorted(
                result["value"],
                key=lambda item: json.dumps(item, sort_keys=True, ensure_ascii=False),
            )
        return result
    return value


def journal_action(work: Path, stream_id: str, action: str) -> dict:
    """Replace only a test-owned, index-addressed journal; retain its original inode."""
    state = work / "state"
    records = json.loads((state / "host/run-index.json").read_text(encoding="utf-8"))
    if isinstance(records, dict):
        raise TypeError("run index must use the actual Host record array")
    (record,) = [item for item in records if item["stream_id"] == stream_id]
    run_id = record["run_id"]
    safe = (
        "".join(
            char if char.isascii() and (char.isalnum() or char in "-_") else "_"
            for char in run_id
        )[:128]
        or "unnamed"
    )
    directory = state / "journals" / safe
    original = directory.with_name(directory.name + ".retained-test-original")
    if action == "replace":
        assert directory.is_dir() and not original.exists()
        stat = directory.stat()
        files = {
            str(path.relative_to(directory)): hashlib.sha256(
                path.read_bytes()
            ).hexdigest()
            for path in directory.rglob("*")
            if path.is_file()
        }
        assert files, "the real journal must contain committed data before replacement"
        directory.rename(original)
        directory.mkdir()
        fact = {
            "run_id": run_id,
            "stream_id": stream_id,
            "directory": str(directory),
            "retained": str(original),
            "original_inode": stat.st_ino,
            "replacement_inode": directory.stat().st_ino,
            "files": files,
        }
        assert fact["original_inode"] != fact["replacement_inode"]
        (work / "journal-replacement.json").write_text(
            json.dumps(fact, indent=2), encoding="utf-8"
        )
        return fact
    fact = json.loads((work / "journal-replacement.json").read_text(encoding="utf-8"))
    assert original.is_dir() and directory.is_dir() and not list(directory.iterdir())
    directory.rmdir()
    original.rename(directory)
    assert directory.stat().st_ino == fact["original_inode"]
    assert {
        str(path.relative_to(directory)): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in directory.rglob("*")
        if path.is_file()
    } == fact["files"]
    return {"restored": True, "files_unchanged": True, "stream_id": stream_id}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--mode", choices=("window", "oversized"), required=True)
    parser.add_argument("--journal-action", choices=("replace", "restore"))
    parser.add_argument("--stream-id")
    parser.add_argument("--compare-pages", nargs=2, type=Path)
    args = parser.parse_args()
    if args.compare_pages:
        first, second = [
            json.loads(path.read_text(encoding="utf-8")) for path in args.compare_pages
        ]
        assert canonical_replay(first) == canonical_replay(second), (
            "committed replay pages differ beyond association order"
        )
        print(json.dumps({"same_complete_page": True, "events": len(first["events"])}))
        return
    if args.journal_action:
        assert args.stream_id, "journal action requires the original stream id"
        print(
            json.dumps(journal_action(args.work, args.stream_id, args.journal_action))
        )
        return
    work = args.work
    count = 0
    lock = threading.Lock()

    class Handler(BaseHTTPRequestHandler):
        def do_POST(self) -> None:
            nonlocal count
            self.rfile.read(int(self.headers.get("Content-Length", "0")))
            with lock:
                count += 1
                turn = count
            if turn == 1:
                delta = {
                    "role": "assistant",
                    "tool_calls": [
                        {
                            "index": 0,
                            "id": "call-gap",
                            "type": "function",
                            "function": {
                                "name": "missing_fixture_tool",
                                "arguments": "{}",
                            },
                        }
                    ],
                }
                reason = "tool_calls"
            else:
                (work / "parked").touch()
                deadline = time.monotonic() + 30
                while not (work / "release").exists():
                    if time.monotonic() >= deadline:
                        self.send_error(504, "test did not release the model")
                        return
                    time.sleep(0.01)
                delta = {"role": "assistant", "content": "continued 中文🌊"}
                reason = "stop"
            chunk = {
                "id": "fixture",
                "choices": [{"index": 0, "delta": delta, "finish_reason": reason}],
            }
            body = f"data: {json.dumps(chunk)}\n\ndata: [DONE]\n\n".encode()
            self.send_response(200)
            self.send_header("Content-Type", "text/event-stream")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def log_message(self, *_args: object) -> None:
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "fixture-model",
                "base_url": f"http://127.0.0.1:{server.server_port}/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture-secret",
            },
            "agent": {
                "name": "gap-fixture",
                "system_prompt": "Answer directly.",
                "max_iterations": 4,
                "enable_tools": True,
            },
        },
        "sdk_profile": {
            "state_root": str(work / "state"),
            "limits": {
                "max_outstanding_live_events": 1,
                "max_event_bytes": 1 if args.mode == "oversized" else 1024 * 1024,
            },
        },
    }
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")
    (work / "ready").touch()
    server.serve_forever()


if __name__ == "__main__":
    main()
