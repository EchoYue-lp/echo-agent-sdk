import json
import sys
from pathlib import Path

work = Path(sys.argv[1])
work.mkdir(parents=True, exist_ok=True)
(work / "host.json").write_text(
    json.dumps(
        {
            "schema_version": 1,
            "default_agent": {
                "model": {
                    "provider": "fixture",
                    "name": "fixture",
                    "base_url": "http://127.0.0.1:1/v1/chat/completions",
                    "api_protocol": "chat_completions",
                },
                "agent": {
                    "name": "fixture",
                    "enable_tools": True,
                    "enable_memory": False,
                },
            },
            "sdk_profile": {
                "state_root": str(work / "state"),
                "limits": {
                    "max_facade_resources": 4,
                    "shutdown_timeout_secs": 2,
                    "max_callback_concurrency": 8,
                },
            },
        }
    )
)
