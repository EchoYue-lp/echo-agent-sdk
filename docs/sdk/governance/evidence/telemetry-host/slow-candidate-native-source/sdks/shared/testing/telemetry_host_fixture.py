"""Real OTLP collector plus a local model driving the existing read_file path."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

ANSWER = "TELEMETRY_BUSINESS_COMPLETE 中文🙂"
CONTENT = "真实文件内容 telemetry read 中文🙂\n"


def prepare(work: Path, console: bool) -> None:
    work.mkdir(parents=True, exist_ok=True)
    (work / "probe.txt").write_text(CONTENT, encoding="utf8")
    (work / "scenario.json").write_text(
        json.dumps(
            {"service_name": "sdk-telemetry-中文🙂", "enable_console": console},
            ensure_ascii=False,
        )
    )


def serve(work: Path) -> None:
    executable = os.environ.get("ECHO_AGENT_SDK_OTLP_COLLECTOR")
    if not executable:
        raise RuntimeError("real telemetry collector executable is required")
    environment = {**os.environ, "ECHO_TELEMETRY_COLLECTOR_WORK": str(work)}
    with (
        (work / "collector.stdout").open("w") as output,
        (work / "collector.stderr").open("w") as error,
    ):
        collector = subprocess.Popen(
            [executable, "--exact", "native_sdk_collector_fixture", "--nocapture"],
            env=environment,
            stdout=output,
            stderr=error,
        )
        try:
            deadline = time.monotonic() + 10
            while not (work / "collector.json").exists():
                assert collector.poll() is None, "collector exited before ready"
                assert time.monotonic() < deadline, "collector ready timeout"
                time.sleep(0.01)
            scenario = json.loads((work / "scenario.json").read_text())
            scenario.update(json.loads((work / "collector.json").read_text()))
            scenario.update(collector_started=True, fixture_pid=os.getpid())
            (work / "scenario.json").write_text(
                json.dumps(scenario, ensure_ascii=False)
            )
            lock = threading.Lock()

            class Handler(BaseHTTPRequestHandler):
                def log_message(self, *args):
                    pass

                def do_POST(self):
                    request = json.loads(
                        self.rfile.read(int(self.headers["Content-Length"]))
                    )
                    with (
                        lock,
                        (work / "model.jsonl").open("a", encoding="utf8") as records,
                    ):
                        records.write(json.dumps(request, ensure_ascii=False) + "\n")
                    messages = request["messages"]
                    if any(message.get("role") == "tool" for message in messages):
                        assert CONTENT.strip() in json.dumps(
                            messages, ensure_ascii=False
                        ), messages
                        message = {"role": "assistant", "content": ANSWER}
                        reason = "stop"
                    else:
                        message = {
                            "role": "assistant",
                            "content": None,
                            "tool_calls": [
                                {
                                    "id": "telemetry-read",
                                    "type": "function",
                                    "function": {
                                        "name": "read_file",
                                        "arguments": json.dumps(
                                            {"path": str(work / "probe.txt")}
                                        ),
                                    },
                                }
                            ],
                        }
                        reason = "tool_calls"
                    usage = {
                        "prompt_tokens": 17,
                        "completion_tokens": 5,
                        "total_tokens": 22,
                    }
                    if request.get("stream"):
                        delta = dict(message)
                        if "tool_calls" in delta:
                            delta["tool_calls"] = [
                                {"index": 0, **call} for call in delta["tool_calls"]
                            ]
                        chunks = [
                            {
                                "id": "telemetry-model",
                                "model": "telemetry-model",
                                "choices": [
                                    {"index": 0, "delta": delta, "finish_reason": None}
                                ],
                            },
                            {
                                "id": "telemetry-model",
                                "model": "telemetry-model",
                                "choices": [
                                    {"index": 0, "delta": {}, "finish_reason": reason}
                                ],
                                "usage": usage,
                            },
                        ]
                        body = (
                            "".join(
                                "data: "
                                + json.dumps(chunk, ensure_ascii=False)
                                + "\n\n"
                                for chunk in chunks
                            )
                            + "data: [DONE]\n\n"
                        ).encode()
                        content_type = "text/event-stream"
                    else:
                        body = json.dumps(
                            {
                                "id": "telemetry-model",
                                "model": "telemetry-model",
                                "choices": [
                                    {
                                        "index": 0,
                                        "message": message,
                                        "finish_reason": reason,
                                    }
                                ],
                                "usage": usage,
                            },
                            ensure_ascii=False,
                        ).encode()
                        content_type = "application/json"
                    self.send_response(200)
                    self.send_header("Content-Type", content_type)
                    self.send_header("Content-Length", str(len(body)))
                    self.end_headers()
                    self.wfile.write(body)

            server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
            config = {
                "schema_version": 1,
                "default_agent": {
                    "model": {
                        "provider": "fixture",
                        "name": "telemetry-model",
                        "base_url": f"http://127.0.0.1:{server.server_port}/v1",
                        "api_protocol": "chat_completions",
                        "auth_token": "fixture",
                    },
                    "agent": {"name": "telemetry-proof", "enable_tools": True},
                },
                "sdk_profile": {"state_root": str(work / "state")},
            }
            (work / "host.json").write_text(json.dumps(config))
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            (work / "ready").write_text("ready")
            while not (work / "stop").exists():
                assert collector.poll() is None, "collector exited during scenario"
                time.sleep(0.01)
            server.shutdown()
            thread.join()
            server.server_close()
            assert collector.wait(timeout=5) == 0
        finally:
            if collector.poll() is None:
                collector.terminate()
                collector.wait(timeout=5)


def verify(work: Path) -> None:
    facts = json.loads((work / "observed.json").read_text())
    assert facts["before"] == {"initialized": False}, facts
    assert (
        facts["init"]
        == facts["after"]
        == facts["after_shutdown"]
        == {"initialized": True}
    ), facts
    assert facts["shutdown"] == {"shutdown": True}, facts
    assert facts["run_outcome"] == "completed", facts
    assert facts["errors"] == {
        "shutdown_before": "framework_error",
        "invalid_console": "invalid_value",
        "repeat_init": "framework_error",
        "repeat_shutdown": "framework_error",
    }, facts
    requests = [
        json.loads(line) for line in (work / "model.jsonl").read_text().splitlines()
    ]
    assert len(requests) == 2, requests
    assert requests[1]["messages"][-1]["role"] == "tool", requests[1]
    assert CONTENT.strip() in json.dumps(requests[1], ensure_ascii=False)
    records = [json.loads(line) for line in facts["collector_at_shutdown"].splitlines()]
    assert any(r["kind"] == "metrics" for r in records), records
    assert any(r["kind"] == "traces" for r in records), records
    text = json.dumps(records, ensure_ascii=False)
    assert "sdk-telemetry-中文🙂" in text, text
    for name in ("llm.calls", "llm.tokens", "tool.executions", "read_file"):
        assert name in text, (name, text)
    events = facts["replay"]["events"]
    payloads = [event.get("envelope", event)["payload"] for event in events]
    answers = [p["data"] for p in payloads if p["event_type"] == "final_answer"]
    assert answers == [{"kind": "string", "value": ANSWER}], answers
    proof = {
        "real_model_requests": len(requests),
        "collector_requests": len(records),
        "shutdown_flush": True,
        "business_file_content_consumed": True,
        "console": facts["scenario"]["enable_console"],
    }
    (work / "normalized.json").write_text(
        json.dumps(proof, ensure_ascii=False, indent=2)
    )
    print(json.dumps(proof))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--console", action="store_true")
    parser.add_argument("--serve", action="store_true")
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    if args.serve:
        serve(args.work)
    elif args.verify:
        verify(args.work)
    else:
        prepare(args.work, args.console)
