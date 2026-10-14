"""Byte-preserving test tee around the real Host, without protocol decisions.

SDK feeds may legitimately deduplicate identical events. The raw transcript
lets the regression prove that the Host itself did not resend covered facts.
Only stdout is tee'd; Host stderr remains stderr and no credentials are logged.
"""

from __future__ import annotations

import subprocess
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

    def forward_input() -> None:
        try:
            for line in sys.stdin.buffer:
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
