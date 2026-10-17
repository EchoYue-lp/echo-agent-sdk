"""Loopback model barriers and captured HTTP input; never writes Host state."""

from __future__ import annotations

import argparse
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

INITIAL = "original request 中文🌊"
STEER = "改为精确回复 corrected 中文🌊"
DRAFT = "superseded draft"
ANSWER = "corrected 中文🌊"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    args = parser.parse_args()
    work = args.work
    work.mkdir(parents=True, exist_ok=True)
    count = 0
    lock = threading.Lock()

    class Handler(BaseHTTPRequestHandler):
        def do_POST(self) -> None:
            nonlocal count
            body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            with lock:
                count += 1
                turn = count
            temporary = work / f"request-{turn}.tmp"
            temporary.write_text(json.dumps(body, ensure_ascii=False), encoding="utf-8")
            temporary.replace(work / f"request-{turn}.json")
            (work / f"parked-{turn}").touch()
            deadline = time.monotonic() + 10
            while not (work / f"release-{turn}").exists():
                if time.monotonic() >= deadline:
                    self.send_error(504, "test did not release the original model call")
                    return
                time.sleep(0.01)
            messages = body.get("messages", [])
            contents = [message.get("content") for message in messages]
            if turn == 1:
                answer = DRAFT
            elif (
                turn == 2
                and INITIAL in contents
                and STEER in contents
                and DRAFT in contents
            ):
                answer = ANSWER
            else:
                self.send_error(
                    422,
                    "original model input did not contain the exact steering history",
                )
                return
            chunk = {
                "id": f"steer-{turn}",
                "choices": [
                    {
                        "index": 0,
                        "delta": {"role": "assistant", "content": answer},
                        "finish_reason": "stop",
                    }
                ],
            }
            # The canonical receipt counts provider-reported model usage.
            chunk["usage"] = {
                "prompt_tokens": 3,
                "completion_tokens": 2,
                "total_tokens": 5,
            }
            payload = f"data: {json.dumps(chunk)}\n\ndata: [DONE]\n\n".encode()
            self.send_response(200)
            self.send_header("Content-Type", "text/event-stream")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)

        def log_message(self, *_args: object) -> None:
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "steer-model",
                "base_url": f"http://127.0.0.1:{server.server_port}/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture-secret",
            },
            "agent": {
                "name": "steer-fixture",
                "system_prompt": "Answer directly.",
                "max_iterations": 4,
                "enable_tools": True,
            },
        },
        "sdk_profile": {"state_root": str(work / "state")},
    }
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")
    (work / "scenario.json").write_text(
        json.dumps(
            {"initial": INITIAL, "steer": STEER, "draft": DRAFT, "answer": ANSWER},
            ensure_ascii=False,
        ),
        encoding="utf-8",
    )
    (work / "ready").touch()
    server.serve_forever()


if __name__ == "__main__":
    main()
