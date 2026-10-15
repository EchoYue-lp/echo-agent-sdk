"""Byte-preserving test tee around the real Host, without protocol decisions.

SDK feeds may legitimately deduplicate identical events. The raw transcript
lets the regression prove that the Host itself did not resend covered facts.
Only stdout is tee'd; Host stderr remains stderr and no credentials are logged.
"""

from __future__ import annotations

import subprocess
import json
import time
import sys
import threading
from pathlib import Path


def main() -> None:
    host, config = sys.argv[1:3]
    transcript = Path(config).parent / "notifications.jsonl"
    child = subprocess.Popen(
        [host, "--config", config], stdin=subprocess.PIPE, stdout=subprocess.PIPE
    )
    if child.stdin is None or child.stdout is None:
        raise RuntimeError("Host stdio was not piped")

    ack_counts = {}
    first_watermark = None
    release = threading.Event()
    log_lock = threading.Lock()

    def record(kind, frame):
        with log_lock, (transcript.parent / "gate-timeline.jsonl").open("a") as output:
            output.write(json.dumps({"time_ns": time.monotonic_ns(), "kind": kind, "frame": frame}) + "\n")

    def forward_input() -> None:
        try:
            for line in sys.stdin.buffer:
                frame = json.loads(line)
                record("client_to_host", frame)
                ack = frame.get("params", {}).get("ack")
                if ack:
                    sequence = ack["last_processed_sequence"]
                    ack_counts[sequence] = ack_counts.get(sequence, 0) + 1
                    if first_watermark == sequence and ack_counts[sequence] >= 2:
                        record("duplicate_ack_observed_release_original_gap", frame)
                        release.set()
                child.stdin.write(line)
                child.stdin.flush()
        except BrokenPipeError:
            pass
        finally:
            child.stdin.close()

    threading.Thread(target=forward_input, daemon=True).start()
    try:
        with transcript.open("wb") as capture:
            for line in child.stdout:
                frame = json.loads(line)
                record("host_to_proxy_received", frame)
                if frame.get("method") == "_echo_agent/gap":
                    gap = frame["params"]["gap"]
                    if first_watermark is None:
                        first_watermark = gap["snapshot_watermark"]
                    elif int(gap["snapshot_watermark"]) > int(first_watermark):
                        record("hold_original_successor_gap_before_duplicate_ack", frame)
                        if not release.wait(10):
                            raise TimeoutError("diagnostic duplicate ACK gate not reached")
                        record("release_original_successor_gap", frame)
                capture.write(line)
                capture.flush()
                sys.stdout.buffer.write(line)
                sys.stdout.buffer.flush()
        sys.exit(child.wait())
    finally:
        if child.poll() is None:
            child.terminate()
            child.wait(timeout=5)


if __name__ == "__main__":
    main()
