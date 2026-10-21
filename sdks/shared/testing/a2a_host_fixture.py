"""A real loopback A2A HTTP/SSE peer; SDK calls still execute the real Host."""

from __future__ import annotations

import argparse
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

TEXT = "请求 中文🙂"
REMOTE_SESSION = "远端-session-🙂"
TASK = "task-中文🙂"


def scenario(base, quota=False):
    client = {"ref": "open", "field": "resource"}
    normal = {"ref": "stream"}
    held = {"ref": "held"}
    closing = {"ref": "closing"}
    if quota:
        return [
            {"id": "open", "op": "a2a.client.open", "args": []},
            {"id": "other", "session": "b", "op": "a2a.client.open", "args": []},
            {
                "id": "held",
                "op": "a2a.task.stream.open",
                "args": [client, base + "/quota-held/held", TEXT, REMOTE_SESSION],
            },
            {
                "id": "quota_rejected",
                "op": "a2a.task.stream.open",
                "args": [client, base + "/quota-held/rejected", TEXT, REMOTE_SESSION],
                "error": "payload_too_large",
            },
            {
                "id": "rejected_source_closed",
                "action": "await_source_closed",
                "source": "rejected",
            },
            {
                "id": "other_get",
                "session": "b",
                "op": "a2a.task.get",
                "args": [{"ref": "other", "field": "resource"}, base + "/a2a", TASK],
            },
            {"id": "close_held", "op": "a2a.stream.close", "args": [held]},
            {
                "id": "held_source_closed",
                "action": "await_source_closed",
                "source": "held",
            },
            {
                "id": "reopened",
                "op": "a2a.task.stream.open",
                "args": [client, base + "/stream", TEXT, REMOTE_SESSION],
            },
            {"id": "item_one", "op": "a2a.stream.next", "args": [{"ref": "reopened"}]},
            {"id": "item_two", "op": "a2a.stream.next", "args": [{"ref": "reopened"}]},
            {"id": "terminal", "op": "a2a.stream.next", "args": [{"ref": "reopened"}]},
            {
                "id": "old_closed",
                "op": "a2a.stream.next",
                "args": [held],
                "error": "closed_handle",
            },
            {
                "id": "reopened_closed",
                "op": "a2a.stream.next",
                "args": [{"ref": "reopened"}],
                "error": "closed_handle",
            },
        ]
    steps = [
        {"id": "open", "op": "a2a.client.open", "args": []},
        {"id": "discover", "op": "a2a.discover", "args": [client, base]},
        {
            "id": "bad_args",
            "op": "a2a.task.send",
            "args": [client, base + "/a2a", TEXT, 7],
            "error": "invalid_value",
        },
        {
            "id": "cross_owner",
            "session": "b",
            "op": "a2a.discover",
            "args": [client, base],
            "error": "invalid_value",
        },
        {
            "id": "future_generation",
            "op": "a2a.discover",
            "args": [{**client, "generation_delta": 1}, base],
            "error": "invalid_value",
        },
        {
            "id": "send",
            "op": "a2a.task.send",
            "args": [client, base + "/a2a", TEXT, REMOTE_SESSION],
        },
        {"id": "get", "op": "a2a.task.get", "args": [client, base + "/a2a", TASK]},
        {
            "id": "cancel_task",
            "op": "a2a.task.cancel",
            "args": [client, base + "/a2a", TASK],
        },
        {
            "id": "remote_error",
            "op": "a2a.task.get",
            "args": [client, base + "/a2a", "missing"],
            "error": "framework_error",
        },
        {
            "id": "stream",
            "op": "a2a.task.stream.open",
            "args": [client, base + "/stream", TEXT, REMOTE_SESSION],
        },
        {
            "id": "stream_cross_owner",
            "session": "b",
            "op": "a2a.stream.next",
            "args": [normal],
            "error": "invalid_value",
        },
        {
            "id": "stream_future_generation",
            "op": "a2a.stream.next",
            "args": [{**normal, "generation_delta": 1}],
            "error": "invalid_value",
        },
        {"id": "item_one", "op": "a2a.stream.next", "args": [normal]},
        {"id": "item_two", "op": "a2a.stream.next", "args": [normal]},
        {"id": "terminal", "op": "a2a.stream.next", "args": [normal]},
        {
            "id": "terminal_closed",
            "op": "a2a.stream.next",
            "args": [normal],
            "error": "closed_handle",
        },
        {
            "id": "held",
            "op": "a2a.task.stream.open",
            "args": [client, base + "/held", TEXT, None],
        },
        {"id": "cancel_stream", "op": "a2a.stream.cancel", "args": [held]},
        {"id": "cancel_terminal", "op": "a2a.stream.next", "args": [held]},
        {
            "id": "closing",
            "op": "a2a.task.stream.open",
            "args": [client, base + "/held", TEXT, None],
        },
        {"id": "close_stream", "op": "a2a.stream.close", "args": [closing]},
        {"id": "close_stream_twice", "op": "a2a.stream.close", "args": [closing]},
        {"id": "close_owner", "action": "close_session"},
        {
            "id": "resource_after_owner_close",
            "session": "b",
            "op": "a2a.discover",
            "args": [client, base],
            "error": "closed_handle",
        },
    ]
    return steps


def serve(work):
    work.mkdir(parents=True, exist_ok=True)
    lock = threading.Lock()
    source_started = {name: threading.Event() for name in ("held", "rejected")}
    source_closed = {name: threading.Event() for name in ("held", "rejected")}
    source_ids = {name: [] for name in ("held", "rejected")}
    closures = {}

    def capture(value):
        with lock, (work / "http.jsonl").open("a", encoding="utf8") as out:
            out.write(json.dumps(value, ensure_ascii=False) + "\n")

    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, *args):
            pass

        def reply(self, value, content_type="application/json"):
            body = (
                value
                if isinstance(value, bytes)
                else json.dumps(value, ensure_ascii=False).encode()
            )
            self.send_response(200)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(body)
            self.wfile.flush()
            self.close_connection = True

        def do_GET(self):
            if self.path.startswith("/quota-source-closed/"):
                name = self.path.removeprefix("/quota-source-closed/")
                assert name in source_started
                started = source_started[name].wait(timeout=5)
                closed = started and source_closed[name].wait(timeout=5)
                with lock:
                    value = {
                        "source": name,
                        "started": started,
                        "closed": closed,
                        "request_ids": list(source_ids[name]),
                        "closure": closures.get(name),
                        "held_started": source_started["held"].is_set(),
                        "held_closed": source_closed["held"].is_set(),
                    }
                self.reply(value)
                return
            capture({"path": self.path, "method": "GET"})
            assert self.path == "/.well-known/agent.json", self.path
            self.reply(
                {
                    "name": "远端 Agent🙂",
                    "description": "A2A 本地协议证据",
                    "url": base + "/a2a",
                    "version": "1.0.0",
                    "skills": [],
                    "defaultInputModes": ["text/plain"],
                    "defaultOutputModes": ["text/plain"],
                    "capabilities": {"streaming": True},
                }
            )

        def do_POST(self):
            request = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            capture({"path": self.path, "method": "POST", "request": request})
            if self.path.startswith("/quota-held/"):
                name = self.path.removeprefix("/quota-held/")
                assert (
                    name in source_started
                    and request["method"] == "tasks/sendSubscribe"
                )
                with lock:
                    source_ids[name].append(request["id"])
                self.send_response(200)
                self.send_header("Content-Type", "text/event-stream")
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(b": independent quota source ready\n\n")
                self.wfile.flush()
                source_started[name].set()
                # These two sources never produce finite events or stop themselves.
                # Only the original HTTP client's socket drop satisfies each event.
                try:
                    assert self.rfile.read(1) == b"", (
                        "held source received unexpected client data"
                    )
                    closure = "eof"
                except ConnectionResetError:
                    closure = "connection_reset"
                capture(
                    {
                        "path": self.path,
                        "source": name,
                        "request_id": request["id"],
                        "source_socket_closed": closure,
                    }
                )
                with lock:
                    closures[name] = closure
                source_closed[name].set()
                self.close_connection = True
                return
            if self.path == "/held":
                assert request["method"] == "tasks/sendSubscribe"
                self.send_response(200)
                self.send_header("Content-Type", "text/event-stream")
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(b": held stream\n\n")
                self.wfile.flush()
                while not (work / "stop").exists():
                    time.sleep(0.01)
                self.close_connection = True
                return
            if self.path == "/stream":
                assert request["method"] == "tasks/sendSubscribe"
                events = [
                    {
                        "type": "artifact",
                        "taskId": TASK,
                        "artifact": {"parts": [{"type": "text", "text": "one 中文🙂"}]},
                        "final": False,
                    },
                    {
                        "type": "artifact",
                        "taskId": TASK,
                        "artifact": {
                            "parts": [{"type": "text", "text": "two 中文🙂"}],
                            "append": True,
                        },
                        "final": True,
                    },
                ]
                self.reply(
                    "".join(
                        "data: " + json.dumps(event, ensure_ascii=False) + "\n\n"
                        for event in events
                    ).encode(),
                    "text/event-stream",
                )
                return
            assert self.path == "/a2a"
            if request["params"].get("id") == "missing":
                self.reply(
                    {
                        "jsonrpc": "2.0",
                        "id": request["id"],
                        "error": {"code": -32001, "message": "task missing 中文🙂"},
                    }
                )
                return
            state = "canceled" if request["method"] == "tasks/cancel" else "completed"
            self.reply(
                {
                    "jsonrpc": "2.0",
                    "id": request["id"],
                    "result": {
                        "id": TASK,
                        "sessionId": REMOTE_SESSION,
                        "status": {"state": state, "timestamp": "2026-10-14T00:00:00Z"},
                        "history": [
                            {
                                "role": "agent",
                                "parts": [{"type": "text", "text": "结果 中文🙂"}],
                            }
                        ],
                        "artifacts": [
                            {
                                "name": "artifact🙂",
                                "parts": [{"type": "text", "text": "payload 中文🙂"}],
                            }
                        ],
                    },
                }
            )

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    base = f"http://127.0.0.1:{server.server_port}"
    quota = (work / "quota").exists()
    (work / "scenario.json").write_text(
        json.dumps(scenario(base, quota=quota), ensure_ascii=False, indent=2)
    )
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "unused",
                "base_url": "http://127.0.0.1:1/v1",
                "api_protocol": "chat_completions",
                "auth_token": "unused",
            },
            "agent": {"name": "a2a-proof", "enable_tools": True},
        },
        "sdk_profile": {
            "state_root": str(work / "state"),
            "limits": {"max_facade_streams": 1} if quota else {},
        },
    }
    (work / "host.json").write_text(json.dumps(config))
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    (work / "ready").write_text("ready")
    while not (work / "stop").exists():
        time.sleep(0.01)
    server.shutdown()
    thread.join()
    server.server_close()


def decode(value):
    if not isinstance(value, dict) or "kind" not in value:
        return value
    kind = value["kind"]
    if kind in ("u64", "i64", "string", "bool", "f64", "handle"):
        return value["value"]
    if kind == "null":
        return None
    if kind == "variant":
        return {
            "type": value["value"]["type_id"],
            "variant": value["value"]["variant"],
            "fields": {
                field["name"]: decode(field["value"])
                for field in value["value"]["fields"]
            },
        }
    if kind == "list":
        return [decode(item) for item in value["value"]]
    if kind == "map":
        return {decode(item["key"]): decode(item["value"]) for item in value["value"]}
    return value


def await_source_closed(work, name):
    from urllib.request import urlopen

    steps = json.loads((work / "scenario.json").read_text())
    base = next(step for step in steps if step["id"] == "held")["args"][1].removesuffix(
        "/quota-held/held"
    )
    with urlopen(base + "/quota-source-closed/" + name, timeout=15) as response:
        value = json.load(response)
    assert value["source"] == name and value["started"] and value["closed"], value
    assert len(value["request_ids"]) == 1, value
    assert value["closure"] in ("eof", "connection_reset"), value
    if name == "rejected":
        assert value["held_started"] and not value["held_closed"], value
    else:
        assert value["held_closed"], value
    (work / ("source-closed-" + name + ".json")).write_text(
        json.dumps(value, ensure_ascii=False, indent=2)
    )
    print(json.dumps(value, ensure_ascii=False))


def verify(work):
    rows = json.loads((work / "results.json").read_text())
    steps = json.loads((work / "scenario.json").read_text())
    assert [r["id"] for r in rows] == [s["id"] for s in steps]
    results = {r["id"]: r for r in rows}
    for step in steps:
        row = results[step["id"]]
        if "error" in step:
            assert row["error"]["code"] == step["error"], row
            assert row["error"]["message"] and row["error"]["operation"], row
        else:
            assert "error" not in row, row
    if (work / "quota").exists():
        rejected_source = results["rejected_source_closed"]["source"]
        held_source = results["held_source_closed"]["source"]
        assert rejected_source["source"] == "rejected" and rejected_source["closed"]
        assert rejected_source["held_started"] and not rejected_source["held_closed"]
        assert held_source["source"] == "held" and held_source["closed"]
        assert (
            len(rejected_source["request_ids"]) == len(held_source["request_ids"]) == 1
        )
        assert rejected_source["request_ids"] != held_source["request_ids"]
        step_ids = [step["id"] for step in steps]
        assert (
            step_ids.index("quota_rejected")
            < step_ids.index("rejected_source_closed")
            < step_ids.index("close_held")
        )
        assert (
            step_ids.index("close_held")
            < step_ids.index("held_source_closed")
            < step_ids.index("reopened")
        )
        assert all(step.get("action") != "close_session" for step in steps)
        rejected = results["quota_rejected"]
        assert "value" not in rejected, "quota refusal must not publish a stream handle"
        assert rejected["error"]["code"] == "payload_too_large"
        assert rejected["error"]["retryable"] == "after_delay"
        assert results["close_held"]["value"] is True
        assert results["other_get"]["value"]["id"] == TASK
        assert results["held"]["value"] != results["reopened"]["value"]
        sequences = []
        for name, variant, sequence in [
            ("item_one", "item", "1"),
            ("item_two", "item", "2"),
            ("terminal", "complete", "3"),
        ]:
            value = decode(results[name]["value"])
            assert (
                value["type"] == "echo_sdk::FacadeStreamEvent"
                and value["variant"] == variant
            )
            assert str(value["fields"]["sequence"]) == sequence
            sequences.append(sequence)
            if variant == "item":
                expected = "one 中文🙂" if name == "item_one" else "two 中文🙂"
                assert (
                    value["fields"]["value"]["artifact"]["parts"][0]["text"] == expected
                )
        http = [
            json.loads(line) for line in (work / "http.jsonl").read_text().splitlines()
        ]
        assert any(row.get("request", {}).get("method") == "tasks/get" for row in http)
        for name, fact in [("held", held_source), ("rejected", rejected_source)]:
            requests = [
                row
                for row in http
                if row.get("path") == "/quota-held/" + name and row.get("request")
            ]
            assert (
                len(requests) == 1
                and requests[0]["request"]["id"] == fact["request_ids"][0]
            )
            assert any(
                row.get("source") == name
                and row.get("request_id") == fact["request_ids"][0]
                and row.get("source_socket_closed")
                for row in http
            )
        normalized = {
            "quota_slot_recovered": True,
            "sequences": sequences,
            "operations": sorted({step["op"] for step in steps if "op" in step}),
            "errors": {
                row["id"]: row["error"]["code"] for row in rows if "error" in row
            },
            "scope": "Host HTTP source drop and slot recovery only; subscribe may precede quota refusal, no remote physical task-stop guarantee",
        }
        (work / "normalized.json").write_text(
            json.dumps(normalized, ensure_ascii=False, indent=2)
        )
        print(json.dumps(normalized, ensure_ascii=False))
        return
    assert results["discover"]["value"]["name"] == "远端 Agent🙂"
    for name, state in [
        ("send", "completed"),
        ("get", "completed"),
        ("cancel_task", "canceled"),
    ]:
        task = results[name]["value"]
        assert task["id"] == TASK and task["sessionId"] == REMOTE_SESSION
        assert task["status"] == {"state": state, "timestamp": "2026-10-14T00:00:00Z"}
        assert task["history"][0]["parts"][0]["text"] == "结果 中文🙂"
        assert task["artifacts"][0]["parts"][0]["text"] == "payload 中文🙂"
    for name, variant, sequence in [
        ("item_one", "item", "1"),
        ("item_two", "item", "2"),
        ("terminal", "complete", "3"),
        ("cancel_terminal", "cancelled", "1"),
    ]:
        event = decode(results[name]["value"])
        assert (
            event["type"] == "echo_sdk::FacadeStreamEvent"
            and event["variant"] == variant
        ), event
        assert str(event["fields"]["sequence"]) == sequence, event
        if variant == "item":
            value = event["fields"]["value"]
            assert value["type"] == "artifact" and value["taskId"] == TASK
            assert value["artifact"]["parts"][0]["text"] == (
                "one 中文🙂" if name == "item_one" else "two 中文🙂"
            )
    assert results["cancel_stream"]["value"] is True
    assert results["close_stream"]["value"] is True
    assert results["close_stream_twice"]["value"] is False
    http = [json.loads(line) for line in (work / "http.jsonl").read_text().splitlines()]
    expected_http_methods = [
        "GET",
        "tasks/send",
        "tasks/get",
        "tasks/cancel",
        "tasks/get",
        "tasks/sendSubscribe",
        "tasks/sendSubscribe",
        "tasks/sendSubscribe",
    ]
    assert len(http) == len(expected_http_methods), http
    assert [
        row.get("request", {}).get("method", "GET") for row in http
    ] == expected_http_methods
    for row in (http[1], http[5]):
        assert row["request"]["params"]["sessionId"] == REMOTE_SESSION
        assert row["request"]["params"]["message"] == {
            "role": "user",
            "parts": [{"type": "text", "text": TEXT}],
        }
    assert all(row["request"]["params"]["id"] == TASK for row in http[2:4])
    normalized = {
        "operations": sorted({step["op"] for step in steps if "op" in step}),
        "http_methods": [row.get("request", {}).get("method", "GET") for row in http],
        "errors": {row["id"]: row["error"]["code"] for row in rows if "error" in row},
        "task_states": [
            results[name]["value"]["status"]["state"]
            for name in ["send", "get", "cancel_task"]
        ],
        "stream_variants": [
            decode(results[name]["value"])["variant"]
            for name in ["item_one", "item_two", "terminal", "cancel_terminal"]
        ],
        "sequences": [
            str(decode(results[name]["value"])["fields"]["sequence"])
            for name in ["item_one", "item_two", "terminal", "cancel_terminal"]
        ],
        "unicode_preserved": True,
        "invalid_calls_no_http": True,
        "local_control_scope": "stream cancellation/close joins Host adapter only; no assertion of remote physical task termination",
    }
    assert len(normalized["operations"]) == 9
    (work / "normalized.json").write_text(
        json.dumps(normalized, ensure_ascii=False, indent=2)
    )
    print(json.dumps(normalized, ensure_ascii=False))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--verify", action="store_true")
    parser.add_argument("--await-source", choices=("held", "rejected"))
    args = parser.parse_args()
    if args.await_source:
        await_source_closed(args.work, args.await_source)
    elif args.verify:
        verify(args.work)
    else:
        serve(args.work)
