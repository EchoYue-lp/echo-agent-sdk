"""Deterministic inputs, resolution and assertions; never drives Host.

Every language client calls its public SDK directly. Unknown ids/timestamps are
compared relationally within a run, not rewritten into made-up execution facts.
"""

from __future__ import annotations

import argparse
import copy
import json
import subprocess
import sys
import threading
import time
import uuid
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

CHECKPOINT_FIELDS = [
    "id",
    "graph_name",
    "workflow_run_id",
    "graph_revision",
    "generation",
    "resume_attempt_id",
    "current_node",
    "state_snapshot",
    "path",
    "step_count",
    "created_at",
    "continuation",
    "interrupt_type",
    "parent_checkpoint_id",
    "label",
    "tags",
    "branch",
]


def reference(name: str, *path: str) -> dict:
    return {"$ref": name, "path": list(path)}


def explicit_json_wire(value: object) -> dict:
    # Explicit public WireValue map disambiguates domain objects with a `kind`
    # member from intentionally preserved unknown protocol wrappers.
    if value is None:
        return {"kind": "null"}
    if isinstance(value, bool):
        return {"kind": "bool", "value": value}
    if isinstance(value, str):
        return {"kind": "string", "value": value}
    if isinstance(value, int):
        return {"kind": "u64" if value >= 0 else "i64", "value": str(value)}
    if isinstance(value, float):
        return {"kind": "f64", "value": value}
    if isinstance(value, list):
        return {"kind": "list", "value": [explicit_json_wire(item) for item in value]}
    assert isinstance(value, dict) and all(isinstance(key, str) for key in value)
    return {
        "kind": "map",
        "value": [
            {"key": {"kind": "string", "value": key}, "value": explicit_json_wire(item)}
            for key, item in value.items()
        ],
    }


def checkpoint_json(value: object, results: dict) -> dict:
    checkpoint = resolve(value, results)
    for field in ["generation", "step_count"]:
        raw = checkpoint[field]
        assert isinstance(raw, (int, str)) and not isinstance(raw, bool)
        parsed = int(raw)
        assert str(parsed) == str(raw) and 0 <= parsed < 2**64
        checkpoint[field] = parsed
    return checkpoint


def resolve(value: object, results: dict) -> object:
    if isinstance(value, list):
        return [resolve(item, results) for item in value]
    if not isinstance(value, dict):
        return value
    if "$ref" in value:
        selected = results[value["$ref"]]["value"]
        for field in value.get("path", []):
            selected = selected[field]
        return copy.deepcopy(selected)
    if "$checkpoint" in value:
        return explicit_json_wire(checkpoint_json(value["$checkpoint"], results))
    if "$tuple_checkpoint" in value:
        checkpoint = checkpoint_json(value["$tuple_checkpoint"], results)
        assert set(checkpoint) == set(CHECKPOINT_FIELDS)
        return {
            "kind": "list",
            "value": [
                explicit_json_wire(checkpoint[field]) for field in CHECKPOINT_FIELDS
            ],
        }
    return {key: resolve(item, results) for key, item in value.items()}


def prepare(work: Path) -> None:
    work.mkdir(parents=True, exist_ok=True)
    graph = {
        "name": "shared-workflow-中文🙂",
        "nodes": [
            {"name": name, "type": "router"}
            for name in ["start", "gate", "yes", "no", "end"]
        ],
        "edges": [
            {"from": "start", "to": "gate"},
            {
                "from": "gate",
                "condition": {
                    "key": "approved",
                    "equals": True,
                    "then": "yes",
                    "else": "no",
                },
            },
            {"from": "yes", "to": "end"},
            {"from": "no", "to": "end"},
        ],
        "entry": "start",
        "finish": ["end"],
        "interrupt_before": ["gate", "end"],
    }
    steps = []
    handle = reference("build", "resource")
    values = {
        "approved": True,
        "memo": "checkpoint 中文🙂",
        "context": {"optional": None, "list": ["a", "b"]},
    }

    def add(name: str, operation: str, arguments: list, **extra: object):
        steps.append(
            {
                "id": name,
                "operation": operation,
                "arguments": arguments,
                "session": "a",
                **extra,
            }
        )

    cp = lambda name: {"$checkpoint": reference(name, "checkpoint")}
    cp_id = lambda name: reference(name, "checkpoint", "id")
    add("build", "workflow.graph.build", [json.dumps(graph, ensure_ascii=False)])
    add("yes-direct", "workflow.graph.run", [handle, values])
    add("no-direct", "workflow.graph.run", [handle, {**values, "approved": False}])
    add("first", "workflow.graph.run_until_interrupt", [handle, values])
    add("load-first", "workflow.graph.load_checkpoint", [handle, cp_id("first")])
    add("restore-first", "workflow.graph.restore", [handle, cp_id("first")])
    add(
        "tag-first",
        "workflow.graph.tag_checkpoint",
        [handle, cp_id("first"), "审查🙂", ["approved", "中文"]],
    )
    add("load-tagged", "workflow.graph.load_checkpoint", [handle, cp_id("first")])
    add("defer", "workflow.graph.resume", [handle, cp_id("first"), "defer"])
    add("load-after-defer", "workflow.graph.load_checkpoint", [handle, cp_id("first")])
    add(
        "resume-exact",
        "workflow.graph.resume_exact",
        [handle, {"$checkpoint": reference("load-tagged")}, "approve"],
    )
    add("load-consumed", "workflow.graph.load_checkpoint", [handle, cp_id("first")])
    add(
        "duplicate-exact",
        "workflow.graph.resume_exact",
        [handle, {"$checkpoint": reference("load-tagged")}, "approve"],
        error_code="framework_error",
    )
    add(
        "resume-finish",
        "workflow.graph.resume",
        [handle, cp_id("resume-exact"), "approve"],
    )
    add(
        "duplicate-id",
        "workflow.graph.resume",
        [handle, cp_id("resume-exact"), "approve"],
        error_code="invalid_value",
    )
    # Branch the same paused parent in a separate workflow execution. Parent
    # remains stored; a fresh child interrupt carries real framework lineage.
    add(
        "branch-parent",
        "workflow.graph.run_until_interrupt",
        [handle, {**values, "approved": False}],
    )
    add(
        "branch",
        "workflow.graph.branch",
        [
            handle,
            cp_id("branch-parent"),
            {"approved": True, "branch_note": "支线🙂"},
            "side-中文",
        ],
    )
    add(
        "parent-after-branch",
        "workflow.graph.load_checkpoint",
        [handle, cp_id("branch-parent")],
    )
    add("branch-load", "workflow.graph.load_checkpoint", [handle, cp_id("branch")])
    add(
        "branch-finish",
        "workflow.graph.resume_exact",
        [handle, cp("branch"), "approve"],
    )
    add(
        "parent-updates",
        "workflow.graph.resume_with_state",
        [handle, cp("branch-parent"), {"approved": False, "updated": "无损🙂"}],
    )
    add(
        "parent-finish",
        "workflow.graph.resume",
        [handle, cp_id("parent-updates"), "approve"],
    )
    add("reject-parent", "workflow.graph.run_until_interrupt", [handle, values])
    add(
        "reject",
        "workflow.graph.resume",
        [handle, cp_id("reject-parent"), "reject", "不执行🙂"],
    )
    add(
        "load-rejected",
        "workflow.graph.load_checkpoint",
        [handle, cp_id("reject-parent")],
    )
    add(
        "reject-duplicate",
        "workflow.graph.resume",
        [handle, cp_id("reject-parent"), "approve"],
        error_code="invalid_value",
    )
    add("list-remaining", "workflow.graph.list_checkpoints", [handle])
    add("list-by-graph", "workflow.graph.list_checkpoints_by_graph", [handle])
    add(
        "cross-owner",
        "workflow.graph.run",
        [handle, values],
        session="b",
        error_code="invalid_value",
    )
    add("state-new", "workflow.state.new", [{"unicode": "状态🙂", "optional": None}])
    state = reference("state-new", "resource")
    add("state-get", "workflow.state.get", [state, "unicode"])
    add(
        "state-set",
        "workflow.state.set",
        [state, "nested", {"labels": ["a", "中文"], "flag": False}],
    )
    add("state-snapshot", "workflow.state.snapshot", [state])
    add("state-keys", "workflow.state.keys", [state])
    add(
        "state-cross-owner",
        "workflow.state.get",
        [state, "unicode"],
        session="b",
        error_code="invalid_value",
    )
    add(
        "quota",
        "workflow.graph.build",
        [json.dumps(graph, ensure_ascii=False)],
        error_code="payload_too_large",
    )
    # Critical shape probe: serde derives accept structs from sequences too.
    # Probe on its own checkpoint so a true error does not poison other cases.
    add("shape-parent", "workflow.graph.run_until_interrupt", [handle, values])
    add(
        "tuple-resume",
        "workflow.graph.resume_exact",
        [
            handle,
            {"$tuple_checkpoint": reference("shape-parent", "checkpoint")},
            "approve",
        ],
        error_code="invalid_value",
        counterexample=True,
    )
    add(
        "shape-parent-after",
        "workflow.graph.load_checkpoint",
        [handle, cp_id("shape-parent")],
    )
    add("shape-updates-parent", "workflow.graph.run_until_interrupt", [handle, values])
    add(
        "tuple-with-state",
        "workflow.graph.resume_with_state",
        [
            handle,
            {"$tuple_checkpoint": reference("shape-updates-parent", "checkpoint")},
            {"approved": False, "unapproved_mutation": "must-not-execute"},
        ],
        error_code="invalid_value",
        counterexample=True,
    )
    add(
        "shape-updates-after",
        "workflow.graph.load_checkpoint",
        [handle, cp_id("shape-updates-parent")],
    )
    steps.append({"id": "close-a", "action": "close", "session": "a"})
    add(
        "closed-graph",
        "workflow.graph.run",
        [handle, values],
        error_code="closed_handle",
    )
    add(
        "closed-state",
        "workflow.state.get",
        [state, "unicode"],
        error_code="closed_handle",
    )
    add(
        "replacement-graph",
        "workflow.graph.build",
        [json.dumps(graph, ensure_ascii=False)],
        session="b",
    )
    add(
        "replacement-state",
        "workflow.state.new",
        [{"unicode": "remaining🙂"}],
        session="b",
    )
    add(
        "replacement-quota",
        "workflow.state.new",
        [{}],
        session="b",
        error_code="payload_too_large",
    )
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "fixture",
                "base_url": "http://127.0.0.1:1/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture-secret",
            },
            "agent": {
                "name": "workflow-proof",
                "system_prompt": "fixture",
                "enable_tools": True,
                "enable_memory": False,
            },
        },
        "sdk_profile": {
            "state_root": str(work / "state"),
            "limits": {"max_facade_resources": 2},
        },
    }
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")
    (work / "scenario.json").write_text(
        json.dumps(
            {"steps": steps, "graph": graph, "state": values}, ensure_ascii=False
        ),
        encoding="utf-8",
    )


def verify(work: Path, rows: list) -> dict:
    scenario = json.loads((work / "scenario.json").read_text())
    assert [row["id"] for row in rows] == [step["id"] for step in scenario["steps"]]
    results = {row["id"]: row for row in rows}
    for step, row in zip(scenario["steps"], rows):
        if step.get("action") == "close":
            assert row["outcome"] == "closed"
        elif "error_code" in step:
            if not step.get("counterexample"):
                assert (
                    row["outcome"] == "error" and row["code"] == step["error_code"]
                ), (step, row)
        else:
            assert row["outcome"] == "result", row

    def value(name):
        return results[name]["value"]

    def number(raw):
        assert not isinstance(raw, bool) and str(int(raw)) == str(raw)
        return int(raw)

    def checkpoint(name):
        return value(name)["checkpoint"]

    def check_cp(cp, node, path):
        assert set(cp) == set(CHECKPOINT_FIELDS)
        assert cp["graph_name"] == scenario["graph"]["name"]
        assert all(
            isinstance(cp[key], str) and cp[key]
            for key in ["id", "workflow_run_id", "graph_revision", "created_at"]
        )
        assert cp["resume_attempt_id"] is None
        assert cp["current_node"] == node and cp["path"] == path
        assert number(cp["step_count"]) == len(path)
        assert (
            cp["continuation"] == {"kind": "node"}
            and cp["interrupt_type"] == "beforenode"
        )
        assert set(cp["state_snapshot"]) == {"values", "messages", "current_node"}, cp[
            "state_snapshot"
        ]

    for name, path in [
        ("yes-direct", ["start", "gate", "yes", "end"]),
        ("no-direct", ["start", "gate", "no", "end"]),
        ("resume-finish", ["start", "gate", "yes", "end"]),
        ("branch-finish", ["start", "gate", "yes", "end"]),
        ("parent-finish", ["start", "gate", "no", "end"]),
    ]:
        result = value(name)
        assert result["outcome"] == "completed" and result["path"] == path
        assert number(result["steps"]) == len(path)
    first = checkpoint("first")
    check_cp(first, "gate", ["start"])
    assert first["state_snapshot"]["values"] == scenario["state"]
    assert (
        first["parent_checkpoint_id"] is None
        and first["label"] is None
        and first["tags"] == []
        and first["branch"] is None
    )
    assert value("load-first") == first
    assert value("restore-first") == {
        "state": first["state_snapshot"],
        "checkpoint": first,
    }
    tagged = value("load-tagged")
    assert tagged == {
        **first,
        "generation": value("load-after-defer")["generation"],
        "label": "审查🙂",
        "tags": ["approved", "中文"],
    }
    assert number(tagged["generation"]) == number(first["generation"]) + 1
    assert checkpoint("defer") == tagged and value("defer")["outcome"] == "deferred"
    assert value("load-after-defer") == tagged
    end = checkpoint("resume-exact")
    check_cp(end, "end", ["start", "gate", "yes"])
    assert (
        end["workflow_run_id"] == first["workflow_run_id"]
        and end["graph_revision"] == first["graph_revision"]
    )
    assert (
        end["parent_checkpoint_id"] == first["id"]
        and number(end["generation"]) == number(tagged["generation"]) + 1
    )
    assert value("load-consumed") is None
    parent = checkpoint("branch-parent")
    child = checkpoint("branch")
    check_cp(child, "end", ["start", "gate", "yes"])
    assert value("parent-after-branch") == parent
    assert (
        child["workflow_run_id"] != parent["workflow_run_id"]
        and child["graph_revision"] == parent["graph_revision"]
    )
    assert child["parent_checkpoint_id"] not in {
        None,
        parent["id"],
    }  # child derives from the consumed fork checkpoint
    assert child["state_snapshot"]["values"] == {
        **parent["state_snapshot"]["values"],
        "approved": True,
        "branch_note": "支线🙂",
    }
    assert value("branch-load") == child
    no = checkpoint("parent-updates")
    check_cp(no, "end", ["start", "gate", "no"])
    assert (
        no["workflow_run_id"] == parent["workflow_run_id"]
        and no["parent_checkpoint_id"] == parent["id"]
    )
    assert no["state_snapshot"]["values"] == {
        **parent["state_snapshot"]["values"],
        "approved": False,
        "updated": "无损🙂",
    }
    assert (
        value("reject")["outcome"] == "rejected"
        and value("reject")["reason"] == "不执行🙂"
    )
    assert (
        value("reject")["path"] == ["start"] and number(value("reject")["steps"]) == 1
    )
    assert value("load-rejected") is None
    assert (
        value("list-remaining") == {"checkpoints": [], "truncated": False}
        and value("list-by-graph") == []
    )
    assert value("state-get") == {"value": "状态🙂"}
    assert value("state-snapshot")["values"] == {
        "unicode": "状态🙂",
        "optional": None,
        "nested": {"labels": ["a", "中文"], "flag": False},
    }
    assert set(value("state-keys")["keys"]) == {"unicode", "optional", "nested"}
    print(
        "Workflow shape observations: "
        + json.dumps(
            {
                name: {
                    "outcome": results[name]["outcome"],
                    "code": results[name].get("code"),
                    "after": value(after),
                }
                for name, after in [
                    ("tuple-resume", "shape-parent-after"),
                    ("tuple-with-state", "shape-updates-after"),
                ]
            },
            ensure_ascii=False,
        )
    )
    tuple_result = results["tuple-resume"]
    assert (
        tuple_result["outcome"] == "error" and tuple_result["code"] == "invalid_value"
    ), (
        "tuple shape accepted; positive routing/checkpoint/state assertions above all passed",
        tuple_result,
    )
    assert value("shape-parent-after") == checkpoint("shape-parent"), (
        "malformed tuple consumed canonical checkpoint"
    )
    tuple_updates = results["tuple-with-state"]
    assert (
        tuple_updates["outcome"] == "error" and tuple_updates["code"] == "invalid_value"
    ), ("tuple state updates accepted", tuple_updates)
    assert value("shape-updates-after") == checkpoint("shape-updates-parent"), (
        "malformed tuple updates consumed canonical checkpoint"
    )
    return {
        "rows": len(rows),
        "scope": "routing/checkpoint/decision/branch/state/owner/close/quota",
        "not_proved": [
            "cross-process MemoryStore persistence",
            "all 24 accepted workflow operations",
            "extension workflow callbacks and stream cancellation",
        ],
    }


class ProtectedClaimFixture:
    """A language-owned CheckpointStore test backend, never a Graph executor.

    Its Store lock/CAS implements the promise declared by the test provider.
    Node scheduling, capability admission and settlement remain real Host/F.
    """

    def __init__(self, work):
        self.work = work
        self.lock = threading.RLock()
        self.stores = {
            name: {"pending": {}, "claims": {}} for name in ("legacy", "protected")
        }
        self.calls = []
        self.llm_calls = []
        self.llm_admissions = []
        self.physical = None
        self.binding = None
        self.joined = None
        self.condition = threading.Condition(self.lock)
        self.protect_waiting = None
        self.protect_released = False

    def component(self, store_name, call):
        request = call["invocation"]["input"]["call"]
        operation, args = request["operation"], request["input"]
        with self.lock:
            store = self.stores[store_name]
            self.calls.append({"store": store_name, "call": copy.deepcopy(call)})
            pending, claims = store["pending"], store["claims"]
            checkpoint_id = args.get("checkpoint_id")
            if operation in {
                "workflow_checkpoint_save",
                "workflow_checkpoint_save_if_generation",
            }:
                checkpoint = protected_unbox(args["checkpoint"])
                checkpoint_id = checkpoint["id"]
                if checkpoint_id in claims:
                    if operation.endswith("_if_generation"):
                        return {"operation": operation, "value": {"committed": False}}
                    return self.failure("active claim cannot be overwritten")
                if operation.endswith("_if_generation"):
                    previous = pending.get(checkpoint_id)
                    matched = previous is not None and previous["generation"] == int(
                        args["expected_generation"]
                    )
                    if matched:
                        pending[checkpoint_id] = checkpoint
                    return {"operation": operation, "value": {"committed": matched}}
                pending[checkpoint_id] = checkpoint
            elif operation == "workflow_checkpoint_load":
                checkpoint = pending.get(checkpoint_id)
                if checkpoint is None and checkpoint_id in claims:
                    checkpoint = claims[checkpoint_id]["checkpoint"]
                return {
                    "operation": operation,
                    "value": {
                        "checkpoint": None
                        if checkpoint is None
                        else explicit_json_wire(checkpoint)
                    },
                }
            elif operation == "workflow_checkpoint_claim":
                checkpoint = pending.pop(checkpoint_id, None)
                if checkpoint is not None:
                    checkpoint = copy.deepcopy(checkpoint)
                    checkpoint["resume_attempt_id"] = str(uuid.uuid4())
                    claims[checkpoint_id] = {
                        "checkpoint": checkpoint,
                        "protected": False,
                    }
                return {
                    "operation": operation,
                    "value": {
                        "checkpoint": None
                        if checkpoint is None
                        else explicit_json_wire(checkpoint)
                    },
                }
            elif operation in {
                "workflow_checkpoint_protect_claim",
                "workflow_checkpoint_ack_claim",
                "workflow_checkpoint_requeue_claim",
                "workflow_checkpoint_renew_claim",
            }:
                claim = claims.get(checkpoint_id)
                if (
                    claim is None
                    or claim["checkpoint"]["resume_attempt_id"] != args["attempt_id"]
                ):
                    return self.failure("exact checkpoint attempt does not match")
                if operation == "workflow_checkpoint_protect_claim":
                    if store_name != "protected":
                        return self.failure(
                            "legacy provider has no protection capability"
                        )
                    claim["protected"] = True
                    if (
                        claim["checkpoint"]["state_snapshot"]["values"]["input"]
                        == "protect-error-after-commit 中文🙂"
                    ):
                        return self.failure(
                            "protection committed but no positive response returned"
                        )
                    if (
                        claim["checkpoint"]["state_snapshot"]["values"]["input"]
                        == "protected-success 中文🙂"
                    ):
                        self.protect_waiting = {
                            "checkpoint_id": checkpoint_id,
                            "attempt_id": args["attempt_id"],
                        }
                        self.condition.notify_all()
                        if not self.condition.wait_for(
                            lambda: self.protect_released, timeout=10
                        ):
                            raise TimeoutError(
                                "original protect response gate was not released"
                            )
                elif operation == "workflow_checkpoint_ack_claim":
                    del claims[checkpoint_id]
                elif operation == "workflow_checkpoint_requeue_claim":
                    checkpoint = copy.deepcopy(claims.pop(checkpoint_id)["checkpoint"])
                    checkpoint["resume_attempt_id"] = None
                    pending[checkpoint_id] = checkpoint
            else:
                return self.failure("fixture operation not implemented: " + operation)
            return {"operation": operation}

    @staticmethod
    def failure(message):
        return {
            "error": {
                "code": "extension_failed",
                "retryable": "never",
                "message": message,
            }
        }

    def llm(self, call):
        with self.lock:
            self.llm_calls.append(copy.deepcopy(call))
            request_text = json.dumps(call["invocation"]["input"], ensure_ascii=False)
            active = [
                item
                for store in self.stores.values()
                for item in store["claims"].values()
                if item["checkpoint"]["state_snapshot"]["values"]["input"]
                in request_text
            ]
            if not active and "fresh-without-claim 中文🙂" in request_text:
                self.llm_admissions.append(
                    {"input": "fresh-without-claim 中文🙂", "protected": None}
                )
                return {
                    "response": {
                        "message": {
                            "role": "assistant",
                            "content": {
                                "kind": "string",
                                "value": "Workflow真实结果 中文🙂",
                            },
                        },
                        "finish_reason": "stop",
                        "raw": explicit_json_wire(
                            {
                                "provider": "protected-fixture",
                                "text": "Workflow真实结果 中文🙂",
                            }
                        ),
                    }
                }
            if len(active) != 1:
                raise ValueError(
                    "actual LLM callback must match one original claimed input"
                )
            claim = active[0]
            self.llm_admissions.append(
                {
                    "input": claim["checkpoint"]["state_snapshot"]["values"]["input"],
                    "protected": claim["protected"],
                    "provider_response_gate_released": self.protect_released,
                }
            )
            if (
                claim["checkpoint"]["state_snapshot"]["values"]["input"]
                == "protected-unknown 中文🙂"
            ):
                binding = self.admit_physical(call)
                return {
                    "error": {
                        "code": "framework_error",
                        "retryable": "never",
                        "message": "original Workflow LLM physical work remains held 中文🙂",
                        "operation": "_echo_agent/extension/invoke",
                        "details": {
                            "fields": [
                                {"key": "observation", "value": "not_settled"},
                                {"key": "run_id", "value": binding["run_id"]},
                                {
                                    "key": "execution_id",
                                    "value": binding["execution_id"],
                                },
                            ]
                        },
                    }
                }
            return {
                "response": {
                    "message": {
                        "role": "assistant",
                        "content": {
                            "kind": "string",
                            "value": "Workflow真实结果 中文🙂",
                        },
                    },
                    "finish_reason": "stop",
                    "raw": explicit_json_wire(
                        {
                            "provider": "protected-fixture",
                            "text": "Workflow真实结果 中文🙂",
                        }
                    ),
                }
            }

    def admit_physical(self, call):
        if self.physical is not None:
            raise ValueError("only the original LLM physical producer may be admitted")
        if call.get("invocation", {}).get("operation") != "llm_chat":
            raise ValueError(
                "physical admission requires the actual nonstream LLM callback"
            )
        if not isinstance(call.get("invocation_id"), str) or not call["invocation_id"]:
            raise ValueError("original invocation identity missing")
        extension = call.get("extension")
        if not isinstance(extension, dict) or extension.get("kind") != "extension":
            raise ValueError("original LLM extension handle missing")
        # These are the real foreign work identities minted at its admission,
        # not guessed Host Run/Task ids or checkpoint workflow_run_id.
        self.binding = {
            "extension": copy.deepcopy(extension),
            "invocation_id": call["invocation_id"],
            "run_id": str(uuid.uuid4()),
            "execution_id": str(uuid.uuid4()),
        }
        program = (
            "import sys,time,json,os;from pathlib import Path;root=Path(sys.argv[1]);"
            "binding=json.loads(sys.argv[2]);binding['pid']=os.getpid();"
            "(root/'physical-ready.tmp').write_text(json.dumps(binding));"
            "(root/'physical-ready.tmp').replace(root/'physical-ready.json');"
            "\nwhile not (root/'physical-release').exists():time.sleep(.01)"
            "\n(root/'physical-stopped.tmp').write_text(json.dumps(binding));"
            "(root/'physical-stopped.tmp').replace(root/'physical-stopped.json')"
        )
        self.physical = subprocess.Popen(
            [sys.executable, "-c", program, str(self.work), json.dumps(self.binding)],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        deadline = time.monotonic() + 5
        while not (self.work / "physical-ready.json").exists():
            if self.physical.poll() is not None or time.monotonic() >= deadline:
                raise RuntimeError("owned physical producer did not become ready")
            time.sleep(0.01)
        binding = {**self.binding, "pid": self.physical.pid}
        if json.loads((self.work / "physical-ready.json").read_text()) != binding:
            raise ValueError("actual producer ready binding mismatch")
        return binding

    def wait_protect(self):
        with self.condition:
            if not self.condition.wait_for(
                lambda: self.protect_waiting is not None, timeout=10
            ):
                raise TimeoutError(
                    "real protect callback did not reach the held response"
                )
            return self.inspect()

    def release_protect(self):
        with self.condition:
            self.protect_released = True
            self.condition.notify_all()
        return {"released": True}

    def release_join(self):
        self.release_protect()
        with self.lock:
            if self.joined is not None:
                return copy.deepcopy(self.joined)
            process = self.physical
            (self.work / "physical-release").touch()
        if process is None:
            return {"admitted": False}
        try:
            _, stderr = process.communicate(timeout=10)
        except subprocess.TimeoutExpired:
            process.kill()
            process.communicate(timeout=5)
            raise
        binding = {**self.binding, "pid": process.pid}
        stopped = json.loads((self.work / "physical-stopped.json").read_text())
        if process.returncode != 0 or stopped != binding:
            raise ValueError("original physical join/stopped proof mismatch")
        result = {
            "admitted": True,
            "exit_code": process.returncode,
            "stopped": stopped,
            "stderr": stderr.decode(),
        }
        with self.lock:
            self.joined = result
        return copy.deepcopy(result)

    def inspect(self):
        with self.lock:
            return {
                "stores": copy.deepcopy(self.stores),
                "calls": copy.deepcopy(self.calls),
                "llm_calls": copy.deepcopy(self.llm_calls),
                "llm_admissions": copy.deepcopy(self.llm_admissions),
                "protect_waiting": copy.deepcopy(self.protect_waiting),
                "physical": None
                if self.physical is None
                else {
                    "binding": {**self.binding, "pid": self.physical.pid},
                    "alive": self.physical.poll() is None,
                    "exit_code": self.physical.poll(),
                },
            }


def protected_unbox(value):
    kind = value["kind"]
    if kind == "null":
        return None
    if kind in {"u64", "i64"}:
        return int(value["value"])
    if kind in {"bool", "string", "f64"}:
        return value["value"]
    if kind == "map":
        return {
            protected_unbox(item["key"]): protected_unbox(item["value"])
            for item in value["value"]
        }
    if kind == "list":
        return [protected_unbox(item) for item in value["value"]]
    raise ValueError("unsupported fixture checkpoint wire kind: " + kind)


def prepare_protected(work):
    # Original router-only prepare/corpus is not changed by this optional mode.
    prepare(work)
    config = json.loads((work / "host.json").read_text())
    config["sdk_profile"]["limits"]["max_facade_resources"] = 8
    (work / "host.json").write_text(json.dumps(config))
    graph = {
        "name": "protected-workflow-中文🙂",
        "nodes": [
            {"name": "start", "type": "router"},
            {
                "name": "agent",
                "type": "agent",
                "model": "protected-fixture",
                "input_key": "input",
                "output_key": "output",
            },
            {"name": "finish", "type": "router"},
        ],
        "edges": [{"from": "start", "to": "agent"}, {"from": "agent", "to": "finish"}],
        "entry": "start",
        "finish": ["finish"],
        "interrupt_before": ["agent"],
    }
    steps = []

    def add(name, operation, arguments):
        steps.append({"id": name, "operation": operation, "arguments": arguments})

    steps.append(
        {
            "id": "bind-legacy",
            "action": "bind_store",
            "store": "legacy",
            "supported": False,
        }
    )
    add("legacy-graph", "workflow.graph.build", [json.dumps(graph, ensure_ascii=False)])
    old = reference("legacy-graph", "resource")
    add(
        "legacy-fresh",
        "workflow.graph.run",
        [old, {"input": "fresh-without-claim 中文🙂"}],
    )
    add(
        "legacy-pause",
        "workflow.graph.run_until_interrupt",
        [old, {"input": "legacy-bound 中文🙂"}],
    )
    steps.append(
        {
            "id": "bind-protected",
            "action": "bind_store",
            "store": "protected",
            "supported": True,
        }
    )
    add(
        "protected-graph",
        "workflow.graph.build",
        [json.dumps(graph, ensure_ascii=False)],
    )
    live = reference("protected-graph", "resource")
    checkpoint = {"$checkpoint": reference("legacy-pause", "checkpoint")}
    cp_id = reference("legacy-pause", "checkpoint", "id")
    add("legacy-defer", "workflow.graph.resume", [old, cp_id, "defer"])
    for name, op, args in [
        ("legacy-approve", "workflow.graph.resume", [old, cp_id, "approve"]),
        ("legacy-exact", "workflow.graph.resume_exact", [old, checkpoint, "approve"]),
        (
            "legacy-updates",
            "workflow.graph.resume_with_state",
            [old, checkpoint, {"input": "must not execute"}],
        ),
        (
            "legacy-branch",
            "workflow.graph.branch",
            [old, cp_id, {}, "must not save fork"],
        ),
    ]:
        add(name, op, args)
    steps.append({"id": "after-legacy-refusals", "action": "inspect"})
    add(
        "legacy-reject",
        "workflow.graph.resume",
        [old, cp_id, "reject", "no Node effect"],
    )
    add(
        "success-pause",
        "workflow.graph.run_until_interrupt",
        [live, {"input": "protected-success 中文🙂"}],
    )
    steps.append(
        {
            "id": "success-resume-start",
            "action": "start_call",
            "operation": "workflow.graph.resume",
            "arguments": [
                live,
                reference("success-pause", "checkpoint", "id"),
                "approve",
            ],
        }
    )
    steps.append({"id": "protect-response-held", "action": "wait_protect"})
    steps.append({"id": "release-protect-response", "action": "release_protect"})
    steps.append(
        {
            "id": "success-resume",
            "action": "await_call",
            "flight": "success-resume-start",
        }
    )
    steps.append({"id": "after-success", "action": "inspect"})
    add(
        "protect-error-pause",
        "workflow.graph.run_until_interrupt",
        [live, {"input": "protect-error-after-commit 中文🙂"}],
    )
    add(
        "protect-error-resume",
        "workflow.graph.resume",
        [live, reference("protect-error-pause", "checkpoint", "id"), "approve"],
    )
    steps.append({"id": "after-protect-error", "action": "inspect"})
    add(
        "unknown-pause",
        "workflow.graph.run_until_interrupt",
        [live, {"input": "protected-unknown 中文🙂"}],
    )
    unknown_id = reference("unknown-pause", "checkpoint", "id")
    unknown_cp = {"$checkpoint": reference("unknown-pause", "checkpoint")}
    add("unknown-resume", "workflow.graph.resume", [live, unknown_id, "approve"])
    steps.append({"id": "while-physical-held", "action": "inspect"})
    add("unknown-repeat", "workflow.graph.resume_exact", [live, unknown_cp, "approve"])
    add(
        "unknown-metadata",
        "workflow.graph.tag_checkpoint",
        [live, unknown_id, "must not overwrite", []],
    )
    steps.append({"id": "physical-release-join", "action": "release_join"})
    add(
        "unknown-after-external-join",
        "workflow.graph.resume_exact",
        [live, unknown_cp, "approve"],
    )
    steps.append({"id": "after-external-join", "action": "inspect"})
    (work / "protected-scenario.json").write_text(
        json.dumps({"steps": steps, "graph": graph}, ensure_ascii=False)
    )


def verify_protected(work, rows):
    scenario = json.loads((work / "protected-scenario.json").read_text())
    assert [row["id"] for row in rows] == [step["id"] for step in scenario["steps"]]
    result = {row["id"]: row for row in rows}
    expected_errors = {
        "legacy-approve",
        "legacy-exact",
        "legacy-updates",
        "legacy-branch",
        "protect-error-resume",
        "unknown-resume",
        "unknown-repeat",
        "unknown-metadata",
        "unknown-after-external-join",
    }
    for step in scenario["steps"]:
        row = result[step["id"]]
        if step["id"] in expected_errors:
            assert row["outcome"] == "error", row
            assert row["code"] == (
                "feature_unavailable"
                if step["id"].startswith("legacy-")
                else "framework_error"
            ), row
            assert (
                row["retryable"] == "never"
                and row["operation"] == "_echo_agent/workflow/op"
            ), row
            assert isinstance(row["message"], str) and row["message"], row
        elif "operation" in step and step.get("action") != "start_call":
            assert row["outcome"] == "result", row
    old_resource = result["legacy-graph"]["value"]["resource"]
    for name in ("legacy-approve", "legacy-exact", "legacy-updates", "legacy-branch"):
        assert (
            result[name]["handle"] == old_resource and result[name]["details"] is None
        ), result[name]
    legacy = result["after-legacy-refusals"]["value"]
    assert len(legacy["llm_calls"]) == 1, legacy
    assert result["legacy-fresh"]["value"]["outcome"] == "completed"
    assert (
        result["legacy-fresh"]["value"]["state"]["values"]["output"]
        == "Workflow真实结果 中文🙂"
    )
    old_calls = [
        item["call"]["invocation"]["input"]["call"]["operation"]
        for item in legacy["calls"]
        if item["store"] == "legacy"
    ]
    assert not any(
        op in old_calls
        for op in ("workflow_checkpoint_claim", "workflow_checkpoint_protect_claim")
    ), old_calls
    # The only initial save belongs to run_until_interrupt, never branch gate.
    assert old_calls.count("workflow_checkpoint_save") == 1, old_calls
    assert result["legacy-defer"]["value"]["outcome"] == "deferred"
    assert result["legacy-reject"]["value"]["outcome"] == "rejected"
    response_held = result["protect-response-held"]["value"]
    assert (
        len(response_held["llm_calls"]) == 1
        and response_held["protect_waiting"] is not None
    ), response_held
    completed = result["success-resume"]["value"]
    assert completed["outcome"] == "completed" and completed["path"] == [
        "start",
        "agent",
        "finish",
    ], completed
    assert completed["state"]["values"]["output"] == "Workflow真实结果 中文🙂", (
        completed
    )
    success = result["after-success"]["value"]
    assert len(success["llm_calls"]) == 2 and success["physical"] is None, success
    assert success["llm_admissions"] == [
        {"input": "fresh-without-claim 中文🙂", "protected": None},
        {
            "input": "protected-success 中文🙂",
            "protected": True,
            "provider_response_gate_released": True,
        },
    ], success
    success_id = result["success-pause"]["value"]["checkpoint"]["id"]
    calls = [
        item["call"]["invocation"]["input"]["call"]
        for item in success["calls"]
        if item["store"] == "protected"
    ]
    related = [
        call for call in calls if call["input"].get("checkpoint_id") == success_id
    ]
    control = [
        call
        for call in related
        if call["operation"]
        not in {"workflow_checkpoint_load", "workflow_checkpoint_renew_claim"}
    ]
    assert [call["operation"] for call in control] == [
        "workflow_checkpoint_claim",
        "workflow_checkpoint_protect_claim",
        "workflow_checkpoint_ack_claim",
    ], related
    assert control[1]["input"]["attempt_id"] == control[2]["input"]["attempt_id"], (
        related
    )
    assert success_id not in success["stores"]["protected"]["claims"]
    failed_protect = result["after-protect-error"]["value"]
    assert (
        len(failed_protect["llm_calls"]) == 2 and failed_protect["physical"] is None
    ), failed_protect
    failed_id = result["protect-error-pause"]["value"]["checkpoint"]["id"]
    assert (
        failed_protect["stores"]["protected"]["claims"][failed_id]["protected"] is True
    )
    failed_calls = [
        item["call"]["invocation"]["input"]["call"]
        for item in failed_protect["calls"]
        if item["store"] == "protected"
    ]
    failed_calls = [
        call for call in failed_calls if call["input"].get("checkpoint_id") == failed_id
    ]
    assert not any(
        call["operation"]
        in {"workflow_checkpoint_ack_claim", "workflow_checkpoint_requeue_claim"}
        for call in failed_calls
    ), failed_calls
    held = result["while-physical-held"]["value"]
    assert len(held["llm_calls"]) == 3 and held["physical"]["alive"] is True, held
    assert held["llm_admissions"][-1] == {
        "input": "protected-unknown 中文🙂",
        "protected": True,
        "provider_response_gate_released": True,
    }, held
    binding = held["physical"]["binding"]
    assert binding["invocation_id"] == held["llm_calls"][-1]["invocation_id"]
    assert binding["extension"] == held["llm_calls"][-1]["extension"]
    unknown_id = result["unknown-pause"]["value"]["checkpoint"]["id"]
    original = held["stores"]["protected"]["claims"][unknown_id]
    assert original["protected"] is True and set(original["checkpoint"]) == set(
        CHECKPOINT_FIELDS
    ), original
    fields = result["unknown-resume"]["details"]["fields"]
    assert fields == [
        {"key": "observation", "value": "not_settled"},
        {"key": "run_id", "value": binding["run_id"]},
        {"key": "execution_id", "value": binding["execution_id"]},
    ], fields
    joined = result["physical-release-join"]["value"]
    assert joined == {
        "admitted": True,
        "exit_code": 0,
        "stopped": binding,
        "stderr": "",
    }, joined
    final = result["after-external-join"]["value"]
    assert len(final["llm_calls"]) == 3 and final["physical"]["alive"] is False, final
    assert final["stores"]["protected"]["claims"][unknown_id] == original
    unknown_calls = [
        item["call"]["invocation"]["input"]["call"]
        for item in final["calls"]
        if item["store"] == "protected"
    ]
    unknown_calls = [
        call
        for call in unknown_calls
        if call["input"].get("checkpoint_id") == unknown_id
    ]
    assert (
        sum(
            call["operation"] == "workflow_checkpoint_protect_claim"
            for call in unknown_calls
        )
        == 1
    )
    assert not any(
        call["operation"]
        in {"workflow_checkpoint_ack_claim", "workflow_checkpoint_requeue_claim"}
        for call in unknown_calls
    ), unknown_calls
    assert len(final["stores"]["legacy"]["claims"]) == 0
    assert len(final["stores"]["protected"]["claims"]) == 2
    return {
        "rows": len(rows),
        "Node_callbacks": len(final["llm_calls"]),
        "same_attempt_protect_then_ack": True,
        "bound_legacy_four_entries_rejected": True,
        "original_physical_joined": True,
        "not_proved": [
            "provider process restart durability",
            "native File TTL/versioned format",
            "Workflow Run receipt from an externally finished unary observation",
            "actual transport packet loss of a protection positive response",
        ],
    }


def serve_protected(work):
    prepare_protected(work)
    state = ProtectedClaimFixture(work)

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *args):
            pass

        def reply(self, value, status=200):
            body = json.dumps(value, ensure_ascii=False).encode()
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_GET(self):
            self.reply(state.inspect())

        def do_POST(self):
            try:
                value = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
                if self.path.startswith("/store/"):
                    result = state.component(self.path.removeprefix("/store/"), value)
                elif self.path == "/llm":
                    result = state.llm(value)
                elif self.path == "/wait_protect":
                    result = state.wait_protect()
                elif self.path == "/release_protect":
                    result = state.release_protect()
                elif self.path in {"/release_join", "/shutdown"}:
                    result = state.release_join()
                else:
                    raise ValueError("unknown fixture endpoint")
                self.reply(result)
                if self.path == "/shutdown":
                    threading.Thread(target=server.shutdown).start()
            except (
                ValueError,
                KeyError,
                TypeError,
                OSError,
                RuntimeError,
                subprocess.SubprocessError,
            ) as error:
                self.reply({"fixture_error": str(error)}, 500)

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    (work / "protected-ready.tmp").write_text(
        json.dumps({"url": f"http://127.0.0.1:{server.server_port}"})
    )
    (work / "protected-ready.tmp").replace(work / "protected-ready.json")
    try:
        server.serve_forever()
    finally:
        try:
            state.release_join()
        finally:
            server.server_close()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument("--resolve", type=Path)
    parser.add_argument("--verify", type=Path)
    parser.add_argument("--serve-protected", action="store_true")
    parser.add_argument("--verify-protected", type=Path)
    args = parser.parse_args()
    if args.serve_protected:
        serve_protected(args.work)
    elif args.prepare:
        prepare(args.work)
    elif args.resolve:
        request = json.loads(args.resolve.read_text())
        print(
            json.dumps(
                resolve(request["value"], request["results"]), ensure_ascii=False
            )
        )
    elif args.verify:
        print(json.dumps(verify(args.work, json.loads(args.verify.read_text()))))
    elif args.verify_protected:
        print(
            json.dumps(
                verify_protected(
                    args.work, json.loads(args.verify_protected.read_text())
                )
            )
        )


if __name__ == "__main__":
    main()
