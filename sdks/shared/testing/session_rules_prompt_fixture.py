"""Observe real provider requests; never inject project rules into SDK messages."""

from __future__ import annotations

import argparse
import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

CUSTOM = "CUSTOM_SYSTEM_KEEP 中文😀"
CASES = [
    {
        "id": "a1",
        "cwd": "a/nested",
        "user": "QUESTION_a1 用户🙂",
        "required": [CUSTOM, "A_ROOT_RULE", "A_NESTED_V1"],
        "forbidden": ["B_ROOT_RULE", "B_NESTED_RULE", "A_NESTED_V2"],
    },
    {
        "id": "b1",
        "cwd": "b/nested",
        "user": "QUESTION_b1 用户🙂",
        "required": [CUSTOM, "B_ROOT_RULE", "B_NESTED_RULE"],
        "forbidden": ["A_ROOT_RULE", "A_NESTED_V1", "A_NESTED_V2"],
    },
    {
        "id": "a2",
        "cwd": "a/nested",
        "user": "QUESTION_a2 用户🙂",
        "required": [CUSTOM, "A_ROOT_RULE", "A_NESTED_V2"],
        "forbidden": ["B_ROOT_RULE", "B_NESTED_RULE", "A_NESTED_V1"],
    },
]


def prepare(work: Path) -> None:
    work.mkdir(parents=True, exist_ok=True)
    for project in ("a", "b"):
        (work / project / ".git").mkdir(parents=True)
        (work / project / "nested").mkdir()
    for name, value in {
        "a/AGENTS.md": "A_ROOT_RULE 根🙂",
        "a/nested/CLAUDE.md": "A_NESTED_V1 子规则😀",
        "b/AGENTS.md": "B_ROOT_RULE 根🙂",
        "b/nested/CLAUDE.md": "B_NESTED_RULE 子规则😀",
    }.items():
        (work / name).write_text(value + "\n", encoding="utf-8")
    (work / "scenario.json").write_text(json.dumps(CASES, ensure_ascii=False, indent=2))


def verify(work: Path) -> None:
    requests = [
        json.loads(line) for line in (work / "requests.jsonl").read_text().splitlines()
    ]
    rows = json.loads((work / "native-results.json").read_text())
    assert len(requests) == len(rows) == len(CASES), (requests, rows)
    normalized = []
    for case, request, row in zip(CASES, requests, rows):
        assert request["case"] == row["id"] == case["id"]
        assert request["valid"], request
        assert row["outcome"] == "completed", row
        messages = request["request"]["messages"]
        system = "\n".join(
            str(m.get("content", "")) for m in messages if m.get("role") == "system"
        )
        user = "\n".join(
            str(m.get("content", "")) for m in messages if m.get("role") == "user"
        )
        assert all(value in system for value in case["required"]), system
        assert not any(value in system for value in case["forbidden"]), system
        assert case["user"] in user, user
        assert not any(value in user for value in case["required"]), user
        # TS exposes StreamEvent wrappers; Python/Java expose envelopes.
        # Retain native-results unchanged and inspect these public shapes.
        events = row["replay"]["events"]
        envelopes = [event.get("envelope", event) for event in events]
        finals = [
            event["payload"]
            for event in envelopes
            if event["payload"]["event_type"] == "final_answer"
        ]
        assert len(finals) == 1, events
        answer = "RULES_OBSERVED_" + case["id"] + " 中文🙂"
        assert finals[0]["data"] == {"kind": "string", "value": answer}, finals
        normalized.append(
            {
                "id": case["id"],
                "outcome": row["outcome"],
                "answer": answer,
                "system_contains": case["required"],
                "system_excludes": case["forbidden"],
                "user_preserved": True,
            }
        )
    (work / "normalized.json").write_text(
        json.dumps(normalized, ensure_ascii=False, indent=2)
    )
    print(
        json.dumps(
            {
                "cases": len(normalized),
                "provider_requests": len(requests),
                "normalized": normalized,
            },
            ensure_ascii=False,
        )
    )


def serve(work: Path) -> None:
    lock = threading.Lock()

    class Handler(BaseHTTPRequestHandler):
        def do_POST(self) -> None:
            body = json.loads(
                self.rfile.read(int(self.headers.get("Content-Length", "0")))
            )
            messages = body["messages"]
            system = "\n".join(
                str(m.get("content", "")) for m in messages if m.get("role") == "system"
            )
            user = "\n".join(
                str(m.get("content", "")) for m in messages if m.get("role") == "user"
            )
            matches = [case for case in CASES if case["user"] in user]
            case = (
                matches[0]
                if len(matches) == 1
                else {"id": "unknown", "required": [], "forbidden": []}
            )
            valid = (
                len(matches) == 1
                and all(v in system for v in case["required"])
                and not any(v in system for v in case["forbidden"])
            )
            with lock, (work / "requests.jsonl").open("a", encoding="utf-8") as output:
                output.write(
                    json.dumps(
                        {"case": case["id"], "valid": valid, "request": body},
                        ensure_ascii=False,
                    )
                    + "\n"
                )
            answer = (
                ("RULES_OBSERVED_" if valid else "RULES_MISSING_")
                + case["id"]
                + " 中文🙂"
            )
            self.send_response(200)
            if body.get("stream"):
                self.send_header("Content-Type", "text/event-stream")
                self.end_headers()
                chunk = {
                    "id": "session-rules",
                    "choices": [
                        {
                            "index": 0,
                            "delta": {"role": "assistant", "content": answer},
                            "finish_reason": "stop",
                        }
                    ],
                }
                self.wfile.write(
                    (
                        "data: "
                        + json.dumps(chunk, ensure_ascii=False)
                        + "\n\ndata: [DONE]\n\n"
                    ).encode()
                )
            else:
                self.send_header("Content-Type", "application/json")
                self.end_headers()
                self.wfile.write(
                    json.dumps(
                        {
                            "id": "session-rules",
                            "choices": [
                                {
                                    "index": 0,
                                    "message": {"role": "assistant", "content": answer},
                                    "finish_reason": "stop",
                                }
                            ],
                            "usage": {
                                "prompt_tokens": 1,
                                "completion_tokens": 1,
                                "total_tokens": 2,
                            },
                        },
                        ensure_ascii=False,
                    ).encode()
                )
            self.wfile.flush()

        def log_message(self, *_args: object) -> None:
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "session-rules",
                "base_url": f"http://127.0.0.1:{server.server_port}/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture-key",
            },
            "agent": {
                "name": "session-rules",
                "system_prompt": CUSTOM,
                "max_iterations": 2,
                "enable_tools": True,
                "enable_memory": False,
            },
        },
        "sdk_profile": {"state_root": str(work / "state")},
    }
    (work / "host.json").write_text(json.dumps(config, ensure_ascii=False))
    (work / "ready").write_text(str(server.server_port))
    server.serve_forever()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--serve", action="store_true")
    parser.add_argument("--mutate", action="store_true")
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    if args.serve:
        serve(args.work)
    elif args.mutate:
        (args.work / "a/nested/CLAUDE.md").write_text("A_NESTED_V2 新规则🙂\n")
    elif args.verify:
        verify(args.work)
    else:
        prepare(args.work)


if __name__ == "__main__":
    main()
