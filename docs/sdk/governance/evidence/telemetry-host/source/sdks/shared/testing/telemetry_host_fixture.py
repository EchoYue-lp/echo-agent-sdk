"""Prepare real-Host telemetry inputs; no simulated exporter or telemetry state."""

from __future__ import annotations

import argparse
import json
import socket
from pathlib import Path


def prepare(work: Path) -> None:
    work.mkdir(parents=True, exist_ok=True)
    # An unused endpoint is sufficient for subscriber admission reproduction:
    # exporter construction is lazy. This phase claims no collector receipt.
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        port = listener.getsockname()[1]
    scenario = {
        "endpoint": f"http://127.0.0.1:{port}",
        "service_name": "sdk-telemetry-中文🙂",
        "enable_console": False,
        "collector_started": False,
    }
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "unused-telemetry-model",
                "base_url": "http://127.0.0.1:1/v1",
                "api_protocol": "chat_completions",
                "auth_token": "unused-local-fixture",
            },
            "agent": {"name": "telemetry-proof", "enable_tools": True},
        },
        "sdk_profile": {"state_root": str((work / "state").resolve())},
    }
    (work / "host.json").write_text(json.dumps(config, ensure_ascii=False))
    (work / "scenario.json").write_text(json.dumps(scenario, ensure_ascii=False))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    prepare(parser.parse_args().work)
