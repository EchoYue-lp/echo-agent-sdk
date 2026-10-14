"""Real LSP stdio server, reusing framework response-fidelity regression cases.

This is a language server fixture, never an ACP Host proxy. SDK requests reach
an unchanged real Host and its framework StdioLspClient. --prepare only writes
isolated configuration and the common expected-value corpus.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path

SPAN = {"start": {"line": 0, "character": 1}, "end": {"line": 0, "character": 3}}
LOCATION = {"uri": "file:///中文😀.test", "range": SPAN}
CORPUS = [
    {
        "operation": "hover",
        "case": "hover-markup",
        "expected": {"contents": "中文😀", "range": SPAN},
    },
    {
        "operation": "hover",
        "case": "hover-string",
        "expected": {"contents": "中文😀", "range": None},
    },
    {"operation": "hover", "case": "null", "expected": None},
    {"operation": "goto_definition", "case": "null", "expected": []},
    {"operation": "find_references", "case": "null", "expected": []},
    {"operation": "completion", "case": "null", "expected": []},
    {"operation": "goto_definition", "case": "location", "expected": [LOCATION]},
    {"operation": "find_references", "case": "locations", "expected": [LOCATION]},
    {
        "operation": "completion",
        "case": "completion",
        "expected": [
            {
                "label": "中文😀",
                "kind": "Function",
                "detail": "fn 中文()",
                "insert_text": "中文😀()",
                "documentation": "说明😀",
            }
        ],
    },
    {
        "operation": "completion",
        "case": "completion-no-kind",
        "expected": [
            {
                "label": "中文😀",
                "kind": None,
                "detail": None,
                "insert_text": None,
                "documentation": None,
            }
        ],
    },
    {
        "operation": "completion",
        "case": "completion-null-kind",
        "expected": [
            {
                "label": "中文😀",
                "kind": None,
                "detail": None,
                "insert_text": None,
                "documentation": None,
            }
        ],
    },
]
CORPUS.extend(
    [
        {
            "operation": "hover",
            "case": "hover-array",
            "expected": {
                "contents": '中文😀\n\n```rust\nlet 中文 = "😀";\n```',
                "range": SPAN,
            },
        },
        {
            "operation": "completion",
            "case": "completion-plain-doc",
            "expected": [
                {
                    "label": "中文😀",
                    "kind": "Function",
                    "detail": None,
                    "insert_text": "中文😀()",
                    "documentation": "说明😀",
                }
            ],
        },
        {
            "operation": "completion",
            "case": "completion-list",
            "expected": [
                {
                    "label": "中文😀",
                    "kind": "Function",
                    "detail": None,
                    "insert_text": "中文😀()",
                    "documentation": None,
                }
            ],
        },
    ]
)


def sdk_projection(value):
    """Expected declared SDK scalar projection preserves integer text, not JS numbers."""
    if isinstance(value, int) and not isinstance(value, bool):
        return str(value)
    if isinstance(value, list):
        return [sdk_projection(item) for item in value]
    if isinstance(value, dict):
        return {key: sdk_projection(item) for key, item in value.items()}
    return value


for item in CORPUS:
    item["expected"] = sdk_projection(item["expected"])

for operation, cases in {
    "hover": [
        "hover-missing",
        "hover-bad-range",
        "hover-tuple-range",
        "hover-tuple-position",
        "null-error",
        "tuple-error",
        "malformed-error-code",
        "malformed-error-message",
        "invalid-version",
        "result-and-error",
        "missing-result",
    ],
    "goto_definition": ["tuple-locations", "mixed-tuple-locations"],
    "find_references": ["tuple-locations"],
    "completion": ["bad-completion"],
}.items():
    CORPUS.extend(
        {"operation": operation, "case": case, "error": "framework_error"}
        for case in cases
    )


def prepare(work: Path) -> None:
    work.mkdir(parents=True, exist_ok=True)
    repo = Path(__file__).resolve().parents[3]
    config = json.loads((repo / "echo-sdk-host/config.sdk.example.json").read_text())
    config["sdk_profile"]["state_root"] = str(work / "state")
    (work / "host.json").write_text(json.dumps(config))
    lsp = {
        "languages": {
            "test": {
                "language": "test",
                "command": sys.executable,
                "args": ["-u", str(Path(__file__).resolve()), "--work", str(work)],
                "extensions": [".test"],
                "env": {},
                "initialization_options": None,
                "max_restarts": 0,
            }
        }
    }
    (work / "lsp.json").write_text(json.dumps(lsp))
    (work / "corpus.json").write_text(json.dumps(CORPUS, ensure_ascii=False))


def response(message: dict) -> dict | None:
    method = message.get("method")
    if method == "initialize":
        return {
            "result": {
                "capabilities": {
                    "hoverProvider": True,
                    "definitionProvider": True,
                    "referencesProvider": True,
                    "completionProvider": {},
                }
            }
        }
    if method == "shutdown":
        return {"result": None}
    if not str(method).startswith("textDocument/"):
        return None
    case = message["params"]["textDocument"]["uri"].rsplit("/", 1)[-1]
    if message["params"].get("position") != {"line": 0, "character": 3}:
        return {"error": {"code": -32602, "message": "UTF-16 position changed"}}
    results = {
        "null": None,
        "hover-markup": {
            "contents": {"kind": "markdown", "value": "中文😀"},
            "range": SPAN,
        },
        "hover-string": {"contents": "中文😀"},
        "hover-array": {
            "contents": ["中文😀", {"language": "rust", "value": 'let 中文 = "😀";'}],
            "range": SPAN,
        },
        "hover-missing": {"range": SPAN},
        "hover-bad-range": {
            "contents": "中文😀",
            "range": {"start": {"line": 0, "character": -1}, "end": SPAN["end"]},
        },
        "hover-tuple-range": {"contents": "中文😀", "range": [[0, 1], [0, 3]]},
        "hover-tuple-position": {
            "contents": "中文😀",
            "range": {"start": [0, 1], "end": SPAN["end"]},
        },
        "location": LOCATION,
        "locations": [LOCATION],
        "tuple-locations": [["file:///中文😀.test", SPAN]],
        "mixed-tuple-locations": [LOCATION, ["file:///中文😀.test", SPAN]],
        "completion": [
            {
                "label": "中文😀",
                "kind": 3,
                "insertText": "中文😀()",
                "detail": "fn 中文()",
                "documentation": {"kind": "markdown", "value": "说明😀"},
            }
        ],
        "completion-plain-doc": [
            {
                "label": "中文😀",
                "kind": 3,
                "insertText": "中文😀()",
                "documentation": "说明😀",
            }
        ],
        "completion-list": {
            "isIncomplete": False,
            "items": [{"label": "中文😀", "kind": 3, "insertText": "中文😀()"}],
        },
        "completion-no-kind": [{"label": "中文😀"}],
        "completion-null-kind": [{"label": "中文😀", "kind": None}],
        "bad-completion": [{"label": "中文😀", "kind": 0}],
    }
    special = {
        "null-error": {"result": {"contents": "ambiguous"}, "error": None},
        "tuple-error": {"error": [-32603, "tuple error", None]},
        "malformed-error-code": {"error": {"code": "bad", "message": "invalid"}},
        "malformed-error-message": {"error": {"code": -32603}},
        "result-and-error": {
            "result": {"contents": "ambiguous"},
            "error": {"code": -32603, "message": "ambiguous"},
        },
        "missing-result": {},
        "invalid-version": {"jsonrpc": "1.0", "result": {"contents": "wrong version"}},
    }
    return special[case] if case in special else {"result": results[case]}


def serve(work: Path) -> None:
    (work / "server.pid").write_text(str(os.getpid()))
    with (work / "lsp-frames.jsonl").open("a", encoding="utf-8") as trace:
        try:
            while True:
                header = sys.stdin.buffer.readline()
                if not header:
                    break
                if not header.lower().startswith(b"content-length:"):
                    continue
                size = int(header.split(b":", 1)[1].strip())
                sys.stdin.buffer.readline()
                message = json.loads(sys.stdin.buffer.read(size))
                trace.write(
                    json.dumps(
                        {"direction": "request", "frame": message}, ensure_ascii=False
                    )
                    + "\n"
                )
                trace.flush()
                if message.get("method") == "exit":
                    break
                result = response(message)
                if result is None:
                    continue
                frame = {"jsonrpc": "2.0", "id": message["id"], **result}
                trace.write(
                    json.dumps(
                        {"direction": "response", "frame": frame}, ensure_ascii=False
                    )
                    + "\n"
                )
                trace.flush()
                body = json.dumps(frame, ensure_ascii=False).encode()
                sys.stdout.buffer.write(
                    b"Content-Length: %d\r\n\r\n" % len(body) + body
                )
                sys.stdout.buffer.flush()
        finally:
            (work / "server.exited").write_text(str(os.getpid()))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--prepare", action="store_true")
    args = parser.parse_args()
    prepare(args.work) if args.prepare else serve(args.work)
