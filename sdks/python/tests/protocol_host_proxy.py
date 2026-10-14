"""Fault injection at the ACP boundary while all forwarded requests use a real Host.

Each scenario owns its Host state and records the observed request/response frames.
The proxy never synthesizes successful Agent behavior or changes Host-owned handles.
"""

from __future__ import annotations

import argparse
import json
import signal
import subprocess
import sys
import threading
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", required=True)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--mode", default="normal")
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[3]
    config = json.loads((repo / "echo-sdk-host/config.sdk.example.json").read_text())
    config["sdk_profile"]["state_root"] = str(args.work / "state")
    config["default_agent"]["agent"]["enable_memory"] = True
    config["default_agent"]["agent"]["memory_path"] = str(args.work / "memory.jsonl")
    config_path = args.work / "host.json"
    config_path.write_text(json.dumps(config))
    child = subprocess.Popen(
        [args.host, "--config", str(config_path)],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=sys.stderr,
        text=True,
        encoding="utf-8",
    )

    def stop(_signal, _frame):
        raise SystemExit(0)

    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    requests = {}
    lock = threading.Lock()
    output_lock = threading.Lock()
    injected = False
    cancel_seen = False
    trace = (args.work / "frames.jsonl").open("w", encoding="utf-8")

    def write_frame(frame) -> None:
        with output_lock:
            text = json.dumps(frame, ensure_ascii=False)
            trace.write(text + "\n")
            trace.flush()
            print(text, flush=True)

    def input_loop() -> None:
        nonlocal injected, cancel_seen
        try:
            for line in sys.stdin:
                frame = json.loads(line)
                method = frame.get("method")
                request_id = frame.get("id")
                if method == "_echo_agent/run/cancel":
                    cancel_seen = True
                fail = (
                    args.mode == "agent-close-retry"
                    and method == "_echo_agent/agent/close"
                    or args.mode == "close-retry"
                    and method == "_echo_agent/session/close"
                    or args.mode == "unregister-retry"
                    and method == "_echo_agent/extension/unregister"
                    or args.mode == "run-close-retry"
                    and cancel_seen
                    and method == "_echo_agent/run/wait"
                )
                if fail and not injected:
                    injected = True
                    if args.mode == "run-close-retry":
                        write_frame(
                            {
                                "jsonrpc": "2.0",
                                "id": request_id,
                                "result": {"settled": False},
                            }
                        )
                    else:
                        write_frame(
                            {
                                "jsonrpc": "2.0",
                                "id": request_id,
                                "error": {
                                    "code": -32000,
                                    "message": "injected retryable fault",
                                    "data": {
                                        "code": "extension_timeout",
                                        "message": "injected retryable fault",
                                        "retryable": "after_delay",
                                        "operation": method,
                                    },
                                },
                            }
                        )
                    continue
                if request_id is not None:
                    with lock:
                        requests[str(request_id)] = method
                if child.stdin is not None:
                    child.stdin.write(line)
                    child.stdin.flush()
        finally:
            if child.stdin is not None:
                child.stdin.close()

    thread = threading.Thread(target=input_loop, daemon=True)
    thread.start()
    try:
        if child.stdout is not None:
            for line in child.stdout:
                frame = json.loads(line)
                with lock:
                    method = requests.pop(str(frame.get("id")), None)
                if method == "initialize" and "result" in frame:
                    capability = frame["result"]["agentCapabilities"]["_meta"][
                        "echo_agent"
                    ]
                    if args.mode == "bad-acp-version":
                        frame["result"]["protocolVersion"] = 2
                    elif args.mode == "overflow-acp-version":
                        frame["result"]["protocolVersion"] = 4294967297
                    if args.mode == "overflow-extension-version":
                        capability["extension_protocol_version"] = 4294967297
                    elif args.mode == "nontext-feature":
                        capability["features"].append(17)
                    elif args.mode == "null-capability":
                        capability["capabilities"].append(None)
                    elif args.mode == "empty-capabilities":
                        capability["capabilities"] = []
                    elif args.mode == "missing-limits":
                        capability["limits"] = {}
                    elif args.mode == "duplicate-feature":
                        capability["features"] = ["same", "same"]
                    elif args.mode == "unsorted-features":
                        capability["features"] = ["z", "a"]
                    elif args.mode == "duplicate-capability":
                        capability["capabilities"].append(capability["capabilities"][0])
                    elif args.mode == "zero-limit":
                        capability["limits"]["max_outstanding_live_events"] = "0"
                    elif args.mode == "overflow-limit":
                        capability["limits"]["max_message_bytes"] = (
                            "18446744073709551616"
                        )
                    elif args.mode == "numeric-limit":
                        capability["limits"]["max_message_bytes"] = 100
                    elif args.mode == "invalid-timeout":
                        capability["limits"]["callback_timeout"]["nanos"] = 1000000000
                    elif args.mode == "unicode-features":
                        capability["features"] = ["\ue000", "\U00010000"]
                    if args.mode == "bad-version":
                        capability["extension_protocol_version"] = 2
                    elif args.mode == "bad-digest":
                        capability["contract_digest"] = "sha256:wrong"
                if (
                    method == "_echo_agent/facade/invoke"
                    and args.mode == "no-value"
                    and "result" in frame
                ):
                    frame["result"] = {}
                write_frame(frame)
    finally:
        child.terminate()
        child.wait(timeout=5)
        trace.close()


if __name__ == "__main__":
    main()
