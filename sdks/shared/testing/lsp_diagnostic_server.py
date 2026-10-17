"""Real diagnostics notification fixture; no ACP Host interception or projection."""

from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path

from lsp_fault_server import SPAN, sdk_projection
from lsp_fault_server import prepare as prepare_request_fixture

MAIN_URI = "file:///中文😀.test"
OTHER_URI = "file:///other.test"
BASE = {"range": SPAN, "message": "中文😀 diagnostic", "source": "fixture"}
VALUES = {
    "valid": [{**BASE, "severity": n, "code": n} for n in [1, 2, 3, 4]]
    + [{**BASE, "code": "7"}, {**BASE, "severity": None, "code": None, "source": None}],
    "integer-bounds": [{**BASE, "code": -(2**31)}, {**BASE, "code": 2**31 - 1}],
    "other": [BASE],
    "clear": [],
    "bad-severity": [{**BASE, "severity": 0}],
    "bad-code-bool": [{**BASE, "code": True}],
    "bad-code-float": [{**BASE, "code": 1.5}],
    "bad-code-overflow": [{**BASE, "code": 2**31}],
    "bad-range-tuple": [{**BASE, "range": [[0, 1], [0, 3]]}],
    "bad-position-tuple": [{**BASE, "range": {"start": [0, 1], "end": SPAN["end"]}}],
    "bad-message": [{**BASE, "message": 42}],
    "mixed": [BASE, {**BASE, "code": False}],
}
SEVERITIES = {1: "Error", 2: "Warning", 3: "Information", 4: "Hint"}


def expected(items):
    values = []
    for item in items:
        code = item.get("code")
        values.append(
            {
                "range": item["range"],
                "severity": SEVERITIES.get(item.get("severity")),
                "message": item["message"],
                "source": item.get("source"),
                "code": None
                if code is None
                else {
                    "kind": "string" if isinstance(code, str) else "integer",
                    "value": code,
                },
            }
        )
    return sdk_projection(values)


CORPUS = [
    {"case": "other", "uri": OTHER_URI, "expected": expected(VALUES["other"])},
    {"case": "valid", "uri": MAIN_URI, "expected": expected(VALUES["valid"])},
    {
        "case": "integer-bounds",
        "uri": MAIN_URI,
        "expected": expected(VALUES["integer-bounds"]),
    },
]
for case in [
    "bad-severity",
    "bad-code-bool",
    "bad-code-float",
    "bad-code-overflow",
    "bad-range-tuple",
    "bad-position-tuple",
    "bad-message",
    "mixed",
    "missing-list",
]:
    CORPUS.extend(
        [
            {"case": "valid", "uri": MAIN_URI, "expected": expected(VALUES["valid"])},
            {"case": case, "uri": MAIN_URI, "error": "framework_error"},
            {"case": "valid", "uri": MAIN_URI, "expected": expected(VALUES["valid"])},
            {"case": "clear", "uri": MAIN_URI, "expected": []},
        ]
    )


def prepare(work: Path) -> None:
    prepare_request_fixture(work)
    config = json.loads((work / "lsp.json").read_text())
    config["languages"]["test"]["args"] = [
        "-u",
        str(Path(__file__).resolve()),
        "--work",
        str(work),
    ]
    (work / "lsp.json").write_text(json.dumps(config))
    (work / "diagnostic-corpus.json").write_text(json.dumps(CORPUS, ensure_ascii=False))


def serve(work: Path) -> None:
    (work / "server.pid").write_text(str(os.getpid()))
    with (work / "diagnostic-frames.jsonl").open("a", encoding="utf-8") as trace:

        def write(frame):
            trace.write(
                json.dumps(
                    {"direction": "response", "frame": frame}, ensure_ascii=False
                )
                + "\n"
            )
            trace.flush()
            body = json.dumps(frame, ensure_ascii=False).encode()
            sys.stdout.buffer.write(b"Content-Length: %d\r\n\r\n" % len(body) + body)
            sys.stdout.buffer.flush()

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
            method = message.get("method")
            if method == "exit":
                break
            if method == "initialize":
                advertised = message["params"]["capabilities"]["textDocument"][
                    "publishDiagnostics"
                ]["relatedInformation"]
                (work / "advertisement.json").write_text(
                    json.dumps({"relatedInformation": advertised})
                )
                write(
                    {
                        "jsonrpc": "2.0",
                        "id": message["id"],
                        "result": {"capabilities": {}},
                    }
                )
            elif method == "shutdown":
                write({"jsonrpc": "2.0", "id": message["id"], "result": None})
            elif method == "textDocument/hover":
                case = message["params"]["textDocument"]["uri"].rsplit("/", 1)[-1]
                uri = OTHER_URI if case == "other" else MAIN_URI
                if case == "missing-uri":
                    params = {"diagnostics": []}
                elif case == "wrong-uri":
                    params = {"uri": 42, "diagnostics": []}
                elif case == "missing-params":
                    params = None
                elif case == "missing-list":
                    params = {"uri": uri}
                else:
                    params = {"uri": uri, "diagnostics": VALUES[case]}
                write(
                    {
                        "jsonrpc": "2.0",
                        "method": "textDocument/publishDiagnostics",
                        "params": params,
                    }
                )
                # Single stdout ordering provides the cache update barrier.
                write(
                    {
                        "jsonrpc": "2.0",
                        "id": message["id"],
                        "result": {"contents": "barrier"},
                    }
                )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--prepare", action="store_true")
    args = parser.parse_args()
    prepare(args.work) if args.prepare else serve(args.work)
