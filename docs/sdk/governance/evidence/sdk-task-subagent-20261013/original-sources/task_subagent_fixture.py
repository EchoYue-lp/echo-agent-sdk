"""Common test fixture with response gates and a registered Task Store CAS backend.

It neither schedules tasks nor invents their state transitions. The real
framework sends a complete candidate graph and expected executions; this
backend owns only atomic compare/store, matching RevisionedTaskStore's trait.
The gate controls only completion of real language-owned custom Agent calls.
"""
from __future__ import annotations
import argparse
import copy
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


class Fixture:
    def __init__(self, work: Path):
        self.work = work
        self.lock = threading.RLock()
        self.graphs = {}
        self.calls = []
        self.commits = []
        self.released = set()
        self.condition = threading.Condition(self.lock)

    def record_agent(self, call):
        with self.lock:
            self.calls.append(copy.deepcopy(call))
            return {"ordinal": len(self.calls)}

    def gate(self, lane, attempt):
        with self.condition:
            deadline = time.monotonic() + 30
            while (lane, attempt) not in self.released:
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    raise TimeoutError(f"response gate not released: {lane}/{attempt}")
                self.condition.wait(remaining)
        return {"released": True}

    def release(self, lane, attempt):
        with self.condition:
            self.released.add((lane, attempt))
            self.condition.notify_all()
        return {"released": True}

    def store(self, call):
        operation, value = call["operation"], call["input"]
        scope = value["scope_id"]
        with self.lock:
            previous = self.graphs.get(scope)
            if operation == "revisioned_task_load":
                return {"kind": "result", "result": {"operation": operation, "value": {"graph": copy.deepcopy(previous)}}}
            if operation not in {"revisioned_task_compare_and_commit", "revisioned_task_compare_and_commit_execution"}:
                raise ValueError("fixture does not implement any other accepted component operation")
            commit = value["commit"]
            actual = None if previous is None else previous["snapshot"]["revision"]
            expected = commit.get("expected_revision")
            actual_executions = {} if previous is None else {t["spec"]["id"]: t["execution"] for t in previous["snapshot"]["tasks"]}
            if actual != expected or (commit.get("expected_executions") is not None and actual_executions != commit["expected_executions"]):
                return {"kind": "error", "error": {"code": "extension_conflict", "message": "fixture atomic expected graph/execution comparison failed", "retryable": "never"}}
            following = commit["next"]
            self.graphs[scope] = copy.deepcopy(following)
            self.commits.append({"operation": operation, "scope_id": scope, "before": copy.deepcopy(previous), "commit": copy.deepcopy(commit), "after": copy.deepcopy(following)})
            (self.work / "task-store.json").write_text(json.dumps(self.inspect(), ensure_ascii=False))
            return {"kind": "result", "result": {"operation": operation, "value": {"graph": copy.deepcopy(following)}}}

    def inspect(self):
        with self.lock:
            return {"graphs": copy.deepcopy(self.graphs), "commits": copy.deepcopy(self.commits), "agent_calls": copy.deepcopy(self.calls)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    args = parser.parse_args()
    args.work.mkdir(parents=True, exist_ok=True)
    state = Fixture(args.work)
    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            self.reply(state.inspect())
        def do_POST(self):
            value = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            if self.path == "/agent":
                result = state.record_agent(value)
            elif self.path == "/release":
                result = state.release(value["lane"], value["attempt"])
            elif self.path == "/gate":
                result = state.gate(value["lane"], value["attempt"])
            elif self.path == "/store":
                result = state.store(value)
            else:
                raise ValueError("unknown test fixture endpoint")
            self.reply(result)
        def reply(self, result):
            data = json.dumps(result, ensure_ascii=False).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)
        def log_message(self, *_args):
            pass
    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    config = {"schema_version": 1,"default_agent": {"model": {"provider":"fixture","name":"task-fixture","base_url":"http://127.0.0.1:1/v1/chat/completions","api_protocol":"chat_completions"},"agent":{"name":"task-fixture","max_iterations":2,"enable_tools":True}},"sdk_profile":{"state_root":str(args.work/"state"),"limits":{"max_callback_concurrency":16,"max_extension_invocations":32}}}
    (args.work/"host.json").write_text(json.dumps(config))
    (args.work/"fixture.json").write_text(json.dumps({"url":f"http://127.0.0.1:{server.server_port}"}))
    (args.work/"ready").touch()
    server.serve_forever()

if __name__ == "__main__":
    main()
