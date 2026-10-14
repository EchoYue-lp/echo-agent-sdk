"""Independent loopback model barriers for true Host parallel Session tests.

Files control only model responses. They never create Host handles, rewrite
wire frames, or decide Run status. Each prompt names an independent lane.
"""

from __future__ import annotations

import argparse
import json
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    args = parser.parse_args()
    work = args.work

    class Handler(BaseHTTPRequestHandler):
        def do_POST(self) -> None:
            request = json.loads(
                self.rfile.read(int(self.headers.get("Content-Length", "0")))
            )
            user = next(
                message.get("content", "")
                for message in reversed(request["messages"])
                if message.get("role") == "user"
            )
            lane = next(
                label for label in ("a", "b", "normal") if f"lane-{label}" in user
            )
            (work / f"requested-{lane}").touch()
            self.send_response(200)
            self.send_header("Content-Type", "text/event-stream")
            self.end_headers()

            def emit(delta: dict, finish: str | None = None) -> None:
                chunk = {
                    "id": f"fixture-{lane}",
                    "choices": [{"index": 0, "delta": delta, "finish_reason": finish}],
                }
                self.wfile.write(f"data: {json.dumps(chunk)}\n\n".encode())
                self.wfile.flush()

            try:
                emit(
                    {
                        "role": "assistant",
                        "reasoning_content": f"observed-{lane} 中文🌊",
                    }
                )
                (work / f"parked-{lane}").touch()
                deadline = time.monotonic() + 30
                while not (work / f"release-{lane}").exists():
                    if time.monotonic() >= deadline:
                        return
                    time.sleep(0.01)
                emit({"content": f"completed-{lane} 中文🌊"}, "stop")
                self.wfile.write(b"data: [DONE]\n\n")
                self.wfile.flush()
                (work / f"completed-{lane}").touch()
            except (BrokenPipeError, ConnectionResetError):
                (work / f"disconnected-{lane}").touch()

        def log_message(self, *_args: object) -> None:
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "parallel-model",
                "base_url": f"http://127.0.0.1:{server.server_port}/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture-secret",
            },
            "agent": {
                "name": "parallel-fixture",
                "max_iterations": 2,
                "enable_tools": True,
            },
        },
        "sdk_profile": {
            "state_root": str(work / "state"),
            "limits": {
                "max_outstanding_live_events": 256,
                "max_event_bytes": 1024 * 1024,
            },
        },
    }
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")
    (work / "ready").touch()
    server.serve_forever()


if __name__ == "__main__":
    main()
