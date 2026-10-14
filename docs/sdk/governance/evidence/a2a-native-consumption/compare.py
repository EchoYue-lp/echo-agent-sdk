"""Compare complete consumer rows and HTTP effects, retaining typed wire lists."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def normalized(language):
    directory = ROOT / language
    rows = json.loads((directory / "results.json").read_text())
    by_id = {row["id"]: row for row in rows}
    card = by_id["discover"]["value"]
    assert card == {
        "name": "远端 Agent🙂",
        "description": "A2A 本地协议证据",
        "url": card["url"],
        "version": "1.0.0",
        "skills": [],
        "defaultInputModes": ["text/plain"],
        "defaultOutputModes": ["text/plain"],
        "capabilities": {
            "streaming": True,
            "pushNotifications": False,
            "stateTransitionHistory": False,
        },
    }
    for name, state in (
        ("send", "completed"),
        ("get", "completed"),
        ("cancel_task", "canceled"),
    ):
        assert by_id[name]["value"] == {
            "id": "task-中文🙂",
            "sessionId": "远端-session-🙂",
            "status": {"state": state, "timestamp": "2026-10-14T00:00:00Z"},
            "history": [
                {"role": "agent", "parts": [{"type": "text", "text": "结果 中文🙂"}]}
            ],
            "artifacts": [
                {
                    "name": "artifact🙂",
                    "parts": [{"type": "text", "text": "payload 中文🙂"}],
                }
            ],
        }
    assert (
        by_id["remote_error"]["error"]["message"]
        == "A2A error -32001: task missing 中文🙂"
    )
    substitutions = {}
    for name in ("open", "stream", "held", "closing"):
        handle = by_id[name]["value"]
        if name == "open":
            handle = handle["resource"]
        substitutions[handle["id"]] = f"<handle:{name}>"
    substitutions[by_id["discover"]["value"]["url"].removesuffix("/a2a")] = "<peer>"
    http = [
        json.loads(line) for line in (directory / "http.jsonl").read_text().splitlines()
    ]
    for index, effect in enumerate(http):
        if "request" in effect:
            substitutions[effect["request"]["id"]] = f"<request:{index}>"

    def visit(value):
        if isinstance(value, str):
            for original, symbolic in substitutions.items():
                value = value.replace(original, symbolic)
            return value
        if isinstance(value, list):
            return [visit(item) for item in value]
        if isinstance(value, dict):
            return {key: visit(item) for key, item in value.items()}
        return value

    for row in rows:
        if "error" in row:
            # Optional error fields: JS undefined serializes as absence;
            # Python/Java materialize the same absence as null.
            row["error"].setdefault("details", None)
            row["error"].setdefault("handle", None)
    return visit({"rows": rows, "http": http})


if __name__ == "__main__":
    values = {
        language: normalized(language) for language in ("typescript", "python", "java")
    }
    assert values["typescript"] == values["python"] == values["java"], (
        "complete consumer/effect mismatch"
    )
    (ROOT / "comparison.json").write_text(
        json.dumps(
            {
                "languages": list(values),
                "equal": True,
                "normalization": [
                    "four observed handle IDs",
                    "observed peer URL",
                    "observed JSON-RPC request UUIDs",
                    "absent optional error details/handle = null",
                ],
                "retained": "all other rows/error fields/typed variant fields and ordered map entries/HTTP request fields",
                "normalized_complete_observation": values["typescript"],
            },
            ensure_ascii=False,
            indent=2,
        )
        + "\n"
    )
    print("three complete observations equal: 24 steps, 8 HTTP effects each")
