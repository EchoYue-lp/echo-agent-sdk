"""Deterministic loopback model for real-Host gap/ACK tests in every SDK.

The first model turn asks for a missing tool, producing durable events. The
second turn parks until the test has recovered its gap; release then produces
new events and a terminal answer. Files coordinate only this fixture model,
never Host state. Each scenario gets its own config, journal and HTTP server.
"""

from __future__ import annotations

import argparse
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--mode", choices=("window", "oversized"), required=True)
    args = parser.parse_args()
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
