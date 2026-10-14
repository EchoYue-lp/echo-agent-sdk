"""Deterministic inputs, resolution and assertions; never drives Host.

Every language client calls its public SDK directly. Unknown ids/timestamps are
compared relationally within a run, not rewritten into made-up execution facts.
"""

from __future__ import annotations

import argparse
import copy
import json
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


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument("--resolve", type=Path)
    parser.add_argument("--verify", type=Path)
    args = parser.parse_args()
    if args.prepare:
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


if __name__ == "__main__":
    main()
