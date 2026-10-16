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
import subprocess
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


class Fixture:
    def __init__(self, work: Path, observation: bool = False):
        self.work = work
        self.lock = threading.RLock()
        self.graphs = {}
        self.calls = []
        self.commits = []
        self.released = set()
        self.condition = threading.Condition(self.lock)
        self.observation = observation
        self.physical = None
        self.physical_binding = None
        self.model_requests = []

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
                return {
                    "kind": "result",
                    "result": {
                        "operation": operation,
                        "value": {"graph": copy.deepcopy(previous)},
                    },
                }
            if operation not in {
                "revisioned_task_compare_and_commit",
                "revisioned_task_compare_and_commit_execution",
            }:
                raise ValueError(
                    "fixture does not implement any other accepted component operation"
                )
            commit = value["commit"]
            actual = None if previous is None else previous["snapshot"]["revision"]
            expected = commit.get("expected_revision")
            actual_executions = (
                {}
                if previous is None
                else {
                    t["spec"]["id"]: t["execution"]
                    for t in previous["snapshot"]["tasks"]
                }
            )
            if actual != expected or (
                commit.get("expected_executions") is not None
                and actual_executions != commit["expected_executions"]
            ):
                return {
                    "kind": "error",
                    "error": {
                        "code": "extension_conflict",
                        "message": "fixture atomic expected graph/execution comparison failed",
                        "retryable": "never",
                    },
                }
            following = commit["next"]
            self.graphs[scope] = copy.deepcopy(following)
            self.commits.append(
                {
                    "operation": operation,
                    "scope_id": scope,
                    "before": copy.deepcopy(previous),
                    "commit": copy.deepcopy(commit),
                    "after": copy.deepcopy(following),
                }
            )
            (self.work / "task-store.json").write_text(
                json.dumps(self.inspect(), ensure_ascii=False)
            )
            return {
                "kind": "result",
                "result": {
                    "operation": operation,
                    "value": {"graph": copy.deepcopy(following)},
                },
            }

    def admit_physical(self, call):
        """Admission originates only inside the actual Agent callback."""
        with self.lock:
            if self.physical is not None:
                raise ValueError("duplicate physical execution")
            context = call.get("context")
            if context is not None and not isinstance(context, dict):
                raise ValueError(
                    "original invocation context must be an object or absent"
                )
            original_context = context or {}
            for key in ("run_id", "execution_id"):
                value = original_context.get(key)
                if not isinstance(value, str) or not value.strip():
                    raise ValueError("original matching Task identities are required")
            if call.get("invocation", {}).get("operation") != "agent_execute_stream":
                raise ValueError("physical admission must originate in execute_stream")
            for k in ("extension", "stream", "factory_origin"):
                h = call.get(k)
                if k == "factory_origin" and h is None:
                    continue
                if not isinstance(h, dict) or not all(
                    isinstance(h.get(f), str) and h[f]
                    for f in ("id", "generation", "kind")
                ):
                    raise ValueError("original issued handles required")
            if (
                not isinstance(call.get("invocation_id"), str)
                or not call["invocation_id"]
            ):
                raise ValueError("original invocation id required")
            binding = {
                "child": copy.deepcopy(call["extension"]),
                "factory_origin": copy.deepcopy(call.get("factory_origin")),
                "stream": copy.deepcopy(call["stream"]),
                "context": copy.deepcopy(context),
                # Both identities originate in this actual Task claim;
                # the fixture never mints or substitutes another owner.
                "run_id": original_context["run_id"],
                "execution_id": original_context["execution_id"],
                "invocation_id": call["invocation_id"],
            }
            # This process owns the real ready/stopped facts; callback completion,
            # cancellation and Host exit cannot manufacture either proof.
            program = (
                "import sys,time,json,os;from pathlib import Path;"
                "root=Path(sys.argv[1]);binding=json.loads(sys.argv[2]);binding['pid']=os.getpid();"
                "(root/'physical-ready.tmp').write_text(json.dumps(binding));"
                "(root/'physical-ready.tmp').replace(root/'physical-ready.json');"
                "\nwhile not (root/'physical-release').exists(): time.sleep(.01)"
                "\n(root/'physical-stopped.tmp').write_text(json.dumps(binding));"
                "(root/'physical-stopped.tmp').replace(root/'physical-stopped.json')"
            )
            self.physical_binding = binding
            self.physical = subprocess.Popen(
                [sys.executable, "-c", program, str(self.work), json.dumps(binding)]
            )
            deadline = time.monotonic() + 5
            while not (self.work / "physical-ready.json").exists():
                if self.physical.poll() is not None or time.monotonic() >= deadline:
                    raise RuntimeError(
                        "original physical producer did not become ready"
                    )
                time.sleep(0.01)
            expected = {**binding, "pid": self.physical.pid}
            if json.loads((self.work / "physical-ready.json").read_text()) != expected:
                raise RuntimeError("original physical ready binding mismatch")
            return expected

    def release_physical(self):
        with self.lock:
            (self.work / "physical-release").touch()
            return {"released": self.physical is not None}

    def join_physical(self):
        with self.lock:
            process, binding = self.physical, self.physical_binding
        if process is None:
            return {"admitted": False}
        try:
            code = process.wait(timeout=30)
        except subprocess.TimeoutExpired:
            # Failure cleanup acts only on this admitted Popen, never a scan or
            # Host pid. Forced termination is a failing test, not a receipt.
            process.kill()
            process.wait(timeout=5)
            raise
        stopped = json.loads((self.work / "physical-stopped.json").read_text())
        expected = {**binding, "pid": process.pid}
        if code != 0 or stopped != expected:
            raise RuntimeError("original physical join/stopped binding mismatch")
        return {"admitted": True, "exit_code": code, "stopped": stopped}

    def model(self, request):
        with self.lock:
            self.model_requests.append(copy.deepcopy(request))
        raise ValueError("formal Task fixture requires registered Agents")

    def inspect(self):
        with self.lock:
            result = {
                "graphs": copy.deepcopy(self.graphs),
                "commits": copy.deepcopy(self.commits),
                "agent_calls": copy.deepcopy(self.calls),
            }
            if self.observation:
                result["model_requests"] = copy.deepcopy(self.model_requests)
                result["physical"] = (
                    None
                    if self.physical is None
                    else {
                        "binding": {**self.physical_binding, "pid": self.physical.pid},
                        "alive": self.physical.poll() is None,
                        "exit_code": self.physical.poll(),
                        "stopped": (self.work / "physical-stopped.json").exists(),
                    }
                )
            return result


def verify_formal_observation(facts):
    """Read actual consumer records; never generate a graph or settlement."""
    scope, revision = facts["scope"], facts["revision"]
    original = facts["original"]
    call, binding = original["call"], original["binding"]
    held, after = facts["held"], facts["after"]
    before_tasks = {
        task["spec"]["id"]: task["execution"]
        for task in held["graphs"][scope]["snapshot"]["tasks"]
    }
    final_tasks = {
        task["spec"]["id"]: task["execution"]
        for task in after["graphs"][scope]["snapshot"]["tasks"]
    }
    claim = before_tasks["a"]["claim"]
    assert before_tasks["a"]["status"]["status"] == "running"
    assert before_tasks["a"].get("result") is None
    assert before_tasks["a"]["retry_count"] == 0 and claim
    assert claim["revision"] == revision and claim["spec_hash"]
    execution_id = (
        f"{scope}:a:{claim['revision']}:{claim['attempt']}:{claim['claim_id']}"
    )
    assert binding["run_id"] == call["context"]["run_id"] == scope
    assert binding["execution_id"] == call["context"]["execution_id"] == execution_id
    assert binding["context"] == call["context"]
    assert binding["invocation_id"] == call["invocation_id"]
    assert binding["child"] == call["extension"]
    assert binding["stream"] == call["stream"]
    assert binding["factory_origin"] is None and call.get("factory_origin") is None
    assert original["error"]["details"]["fields"] == [
        {"key": "observation", "value": "not_settled"},
        {"key": "run_id", "value": scope},
        {"key": "execution_id", "value": execution_id},
    ]
    assert held["physical"]["binding"] == binding
    assert held["physical"]["alive"] and not held["physical"]["stopped"]
    assert facts["held_counts"] == {"a": 1, "b": 1, "c": 0, "d": 0}
    assert before_tasks["b"]["status"]["status"] == "completed"
    # C/D belong to the next wave of this graph. ADR0060 keeps that wave
    # behind every receipt of the current A/B wave, including unknown A.
    assert before_tasks["c"]["status"]["status"] == "pending"
    assert before_tasks["d"]["status"]["status"] == "pending"
    assert after["physical"]["binding"] == binding
    assert after["physical"]["exit_code"] == 0 and after["physical"]["stopped"]
    assert facts["joined"] == {"admitted": True, "exit_code": 0, "stopped": binding}
    assert facts["counts"] == {"a": 1, "b": 1, "c": 1, "d": 1}
    assert held["model_requests"] == after["model_requests"] == []
    assert after["graphs"][scope]["snapshot"]["revision"] == revision
    assert all(t["status"]["status"] == "completed" for t in final_tasks.values())
    assert all(t["status"]["status"] == "completed" for t in facts["listed"]["tasks"])
    assert facts["listed_again"] == facts["listed"]
    assert final_tasks["a"].get("claim") is None
    assert final_tasks["a"]["retry_count"] == 0
    assert final_tasks["b"] == before_tasks["b"]

    def output(execution):
        assert execution["result"]["kind"] == "map"
        fields = [
            field["value"]
            for field in execution["result"]["value"]
            if field["key"] == {"kind": "string", "value": "output"}
        ]
        assert len(fields) == 1 and fields[0]["kind"] == "string"
        return fields[0]["value"]

    assert output(final_tasks["a"]) == "A-after-physical-close 中文🧪"
    for lane in "bcd":
        assert output(final_tasks[lane]) == f"{lane.upper()}-canonical-result 中文🧪"
    for lane in "cd":
        assert "Completed dependency b" in facts["prompts"][f"{lane}/1"]
        assert "B-canonical-result" in facts["prompts"][f"{lane}/1"]
    assert "Completed dependency a" in facts["prompts"]["d/1"]
    assert "A-after-physical-close" in facts["prompts"]["d/1"]
    transitions = []
    for commit in after["commits"]:
        following = {
            task["spec"]["id"]: task["execution"]
            for task in commit["after"]["snapshot"]["tasks"]
        }
        if following["b"]["status"]["status"] == "completed":
            assert following["b"] == before_tasks["b"]
        if commit["operation"] != "revisioned_task_compare_and_commit_execution":
            continue
        previous = {
            task["spec"]["id"]: task["execution"]
            for task in commit["before"]["snapshot"]["tasks"]
        }
        if (
            previous["a"]["status"]["status"] == "running"
            and following["a"]["status"]["status"] == "completed"
        ):
            transitions.append(commit)
    assert len(transitions) == 1
    settlement = transitions[0]["commit"]
    assert settlement["expected_revision"] == revision
    assert settlement["expected_executions"]["a"]["claim"] == claim
    return {"same_attempt_terminal_cas": 1, "executions": facts["counts"]}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--observation", action="store_true")
    parser.add_argument("--verify-formal", type=Path)
    args = parser.parse_args()
    if args.verify_formal:
        print(
            json.dumps(
                verify_formal_observation(json.loads(args.verify_formal.read_text()))
            )
        )
        return
    args.work.mkdir(parents=True, exist_ok=True)
    state = Fixture(args.work, args.observation)

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            self.reply(state.inspect())

        def do_POST(self):
            value = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
            if args.observation and self.path == "/v1/chat/completions":
                state.model(value)
                return
            if args.observation and self.path.startswith("/physical/"):
                if self.path == "/physical/admit":
                    result = state.admit_physical(value)
                elif self.path == "/physical/release":
                    result = state.release_physical()
                elif self.path == "/physical/join":
                    result = state.join_physical()
                else:
                    raise ValueError("unknown physical fixture operation")
            elif args.observation and self.path == "/shutdown":
                state.release_physical()
                result = state.join_physical()
                self.reply(result)
                threading.Thread(target=server.shutdown).start()
                return
            elif self.path == "/agent":
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
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(data)

        def log_message(self, *_args):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "task-fixture",
                "base_url": "http://127.0.0.1:1/v1/chat/completions",
                "api_protocol": "chat_completions",
            },
            "agent": {
                "name": "task-fixture",
                "max_iterations": 2,
                "enable_tools": True,
            },
        },
        "sdk_profile": {
            "state_root": str(args.work / "state"),
            "limits": {"max_callback_concurrency": 16, "max_extension_invocations": 32},
        },
    }
    if args.observation:
        config["default_agent"]["model"]["base_url"] = (
            f"http://127.0.0.1:{server.server_port}/v1/chat/completions"
        )
    (args.work / "host.json").write_text(json.dumps(config))
    (args.work / "fixture.json").write_text(
        json.dumps({"url": f"http://127.0.0.1:{server.server_port}"})
    )
    (args.work / "ready").touch()
    try:
        server.serve_forever()
    finally:
        if args.observation:
            state.release_physical()
            state.join_physical()
        server.server_close()


if __name__ == "__main__":
    main()
