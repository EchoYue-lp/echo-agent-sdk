"""Shared inputs/oracles only. Native SDKs own every actual AgentFactory callback."""

from __future__ import annotations

import argparse
import base64
import json
from pathlib import Path

EVAL = "echo_agent::eval::runner::EvalRunner::"
GRADE = "echo_agent::eval::grader::LlmGrader::"
LOOP = "echo_agent::improve::loop::ImprovementLoop::"
SAVER = "echo_agent::improve::trajectory::TrajectorySaver::"


def ref(name, *path):
    return {"$ref": name, "path": list(path)}


def case(name, task="good", criteria=None):
    return {
        "id": name,
        "name": name,
        "description": "native bounded case 中文🙂",
        "domain": None,
        "task": task,
        "project_fixture": None,
        "success_criteria": criteria
        or {"type": "output_contains", "substring": "OK 中文🙂"},
        "constraints": {},
    }


def prepare(work, group):
    work.mkdir(parents=True, exist_ok=True)
    (work / "runtime-temp").mkdir()
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "fixture",
                "base_url": "http://127.0.0.1:1/v1/chat/completions",
                "api_protocol": "chat_completions",
            },
            "agent": {"name": "fixture", "enable_tools": True, "enable_memory": False},
        },
        "sdk_profile": {
            "state_root": str((work / "state").resolve()),
            "limits": {"max_facade_resources": 64},
        },
    }
    run = {
        "run_id": "trajectory-中文🙂",
        "parent_run_id": "parent🙂",
        "agent_name": "native-agent🙂",
        "model": "fixed-model",
        "provider": None,
        "turn_id": "turn🙂",
        "execution_id": "execute🙂",
        "session_id": "session🙂",
        "status": "completed",
        "input": "读写中文🙂",
        "events": [
            {
                "type": "tool_call",
                "call_id": "call🙂",
                "name": "write_file",
                "args": {"path": "file🙂", "content": "written🙂"},
                "risk": "write",
                "duration_ms": 11,
            },
            {"type": "file_edit", "tool": "write_file", "path": "file🙂"},
            {
                "type": "tool_result",
                "call_id": "call🙂",
                "name": "write_file",
                "success": True,
                "output_preview": "写入🙂",
                "output_truncated": False,
                "duration_ms": 11,
                "original_bytes": 9,
                "returned_bytes": 9,
                "estimated_tokens": 3,
                "output_handling": None,
                "artifact": None,
            },
        ],
        "final_output": "完成🙂",
        "error": None,
        "token_usage": {
            "prompt_tokens": 17,
            "completion_tokens": 5,
            "total_tokens": 22,
            "cached_prompt_tokens": 3,
            "cache_creation_prompt_tokens": 0,
            "usage_reported_calls": 1,
            "usage_missing_calls": 0,
        },
        "timings": {
            "total_duration_ms": 123,
            "llm_duration_ms": 112,
            "tool_duration_ms": 11,
        },
        "started_at": "2026-10-13T00:00:00.123456789Z",
        "finished_at": "2026-10-13T00:00:00.246456789Z",
    }
    assertions = [
        {"id": "a1", "check": "contains OK", "expected": "OK"},
        {"id": "a2", "check": "contains 中文", "expected": "中文"},
    ]
    grade_good = {
        "results": [
            {
                "assertion_id": "a1",
                "passed": True,
                "confidence": 0.875,
                "evidence": "OK",
                "reasoning": "fixed one🙂",
            },
            {
                "assertion_id": "a2",
                "passed": False,
                "confidence": 0.625,
                "evidence": "none",
                "reasoning": "fixed two🙂",
            },
        ],
        "overall_assessment": "fixed grade🙂",
    }
    steps = []

    def add(name, op, args=(), **extra):
        steps.append({"id": name, "operation": op, "arguments": list(args), **extra})

    if group == "eval":
        add("runner", EVAL + "new", [{"$path": "eval-root"}])
        add("default-timeout", EVAL + "timeout_secs", [ref("runner")])
        add("workspace", EVAL + "workspace_root", [ref("runner")])
        add(
            "configured-root",
            EVAL + "workspace_root",
            [ref("runner"), {"$path": "eval-configured"}],
        )
        add(
            "timeout-runner",
            EVAL + "timeout_secs",
            [ref("configured-root"), {"$u64": "1"}],
        )
        add("configured-timeout", EVAL + "timeout_secs", [ref("timeout-runner")])
        add("store", "trace.store.open", ["jsonl", {"$string_path": "traces"}])
        add("save-run", "trace.run.save", [ref("store", "resource"), run])
        add(
            "stored-runner",
            EVAL + "with_run_store",
            [ref("timeout-runner"), ref("store", "resource")],
        )
        add(
            "run-all",
            EVAL + "run_all",
            [
                ref("stored-runner"),
                [case("pass"), case("bad", "bad")],
                {"$factory": True},
            ],
        )
        add(
            "timeout",
            EVAL + "run_all_async",
            [ref("timeout-runner"), [case("timeout", "hang")], {"$factory": True}],
        )
        add(
            "direct",
            EVAL + "run",
            [ref("timeout-runner"), case("direct"), {"$custom": True}],
        )
        add(
            "constraints",
            EVAL + "evaluate_run_constraints",
            [
                ref("runner"),
                {
                    "max_files_changed": 0,
                    "max_tool_calls": 0,
                    "forbidden_paths": ["file🙂"],
                    "required_read_before_edit": True,
                },
                run,
            ],
        )
        add("grader", GRADE + "new")
        add(
            "grader-config",
            EVAL + "with_grader",
            [ref("runner"), ref("grader"), {"$custom": True}],
        )
        steps.append({"id": "mode-grade", "action": "custom_mode", "mode": "grade"})
        add(
            "graded-run",
            EVAL + "run",
            [
                ref("grader-config"),
                case(
                    "graded", criteria={"type": "llm_graded", "assertions": assertions}
                ),
                {"$custom": True},
            ],
        )
        add(
            "grade",
            GRADE + "grade",
            [ref("grader"), "grade-task", "OK", assertions, {"$custom": True}],
        )
        add(
            "grade-null",
            GRADE + "grade_with_trajectory",
            [ref("grader"), "grade-task", "OK", assertions, None, {"$custom": True}],
        )
        add(
            "grade-trajectory",
            GRADE + "grade_with_trajectory",
            [
                ref("grader"),
                "grade-task",
                "OK",
                assertions,
                "工具轨迹🙂",
                {"$custom": True},
            ],
        )
        steps.append(
            {"id": "mode-bad-grade", "action": "custom_mode", "mode": "invalid_grade"}
        )
        add(
            "grade-bad",
            GRADE + "grade",
            [ref("grader"), "grade-task", "OK", assertions, {"$custom": True}],
        )
        add(
            "foreign-runner",
            EVAL + "timeout_secs",
            [ref("runner")],
            session="b",
            error="invalid_value",
        )
    elif group == "improve":
        cases = [case("case-" + str(index)) for index in range(4)]
        add("loop", LOOP + "new")
        add("max", LOOP + "max_iterations", [ref("loop"), {"$u64": "2"}])
        add("holdout", LOOP + "holdout_ratio", [ref("max"), {"$f64": 0.5}])
        add(
            "threshold",
            LOOP + "improvement_threshold",
            [ref("holdout"), {"$f64": 0.75}],
        )
        add("early", LOOP + "run", [ref("threshold"), cases, {"$factory": True}, None])
        add(
            "max-threshold",
            LOOP + "improvement_threshold",
            [ref("holdout"), {"$f64": 1.1}],
        )
        add(
            "maximum",
            LOOP + "run_async",
            [ref("max-threshold"), cases, {"$factory": True}, None],
        )
        add("max-get", LOOP + "max_iterations", [ref("max-threshold")])
        add("holdout-get", LOOP + "holdout_ratio", [ref("max-threshold")])
        add("threshold-get", LOOP + "improvement_threshold", [ref("max-threshold")])
        add("store", "trace.store.open", ["jsonl", {"$string_path": "traces"}])
        add("save-run", "trace.run.save", [ref("store", "resource"), run])
        add("load-run", "trace.run.load", [ref("store", "resource"), run["run_id"]])
        add("sharegpt", SAVER + "convert_run_to_sharegpt", [run])
        add("analyze", "improve.run.analyze", [run])
        add(
            "analyze-failed",
            "improve.run.analyze",
            [{**run, "status": "failed", "error": "真实原因🙂"}],
        )
        add("saver", SAVER + "new", [{"$path": "trajectories"}])
        add("save", SAVER + "save", [ref("saver"), run, "model🙂"])
        add(
            "skip-nonterminal",
            SAVER + "save",
            [
                ref("saver"),
                {**run, "status": "running", "final_output": None},
                "model🙂",
            ],
        )
        add("list", SAVER + "list", [ref("saver"), None])
        add("stats", SAVER + "stats", [ref("saver")])
        steps.append(
            {"id": "block", "action": "block_directory", "directory": "trajectories"}
        )
        add(
            "failed-save",
            SAVER + "save",
            [ref("saver"), run, "model🙂"],
            error="framework_error",
        )
        steps.append(
            {
                "id": "restore",
                "action": "restore_directory",
                "directory": "trajectories",
            }
        )
        add("saver-reopen", SAVER + "new", [{"$path": "trajectories"}])
        add("reopened-list", SAVER + "list", [ref("saver-reopen"), None])
        add("reopened-stats", SAVER + "stats", [ref("saver-reopen")])
    else:
        nodes = [
            {
                "id": "agent🙂",
                "label": "Agent 中文",
                "node_type": "Agent",
                "metadata": {"model": "固定", "optional": ""},
            },
            {
                "id": "subagent🌊",
                "label": "Subagent 🌊",
                "node_type": "Subagent",
                "metadata": {"owner": "agent🙂"},
            },
        ]
        add("tracker", "topology.tracker.open_exact")
        for index, node in enumerate(nodes):
            add("node-" + str(index), "topology.node.add_exact", [ref("tracker"), node])
        add(
            "duration",
            "topology.call.record_with_duration",
            [ref("tracker"), "agent🙂", "subagent🌊", "执行🙂", {"$u64": "123"}],
        )
        add(
            "second-call",
            "topology.call.record",
            [ref("tracker"), "agent🙂", "subagent🌊", "再次🙂"],
        )
        for op in (
            "nodes",
            "edges",
            "stats",
            "snapshot",
            "to_json",
            "to_mermaid",
            "to_dot",
        ):
            add(op, "topology." + op, [ref("tracker")])
        add(
            "foreign",
            "topology.clear",
            [ref("tracker")],
            session="b",
            error="invalid_value",
        )
        add("after-foreign", "topology.stats", [ref("tracker")])
        add("clear", "topology.clear", [ref("tracker")])
        add("empty", "topology.snapshot", [ref("tracker")])
        add("close", "facade.resource.close", [ref("tracker")])
        add("closed", "topology.stats", [ref("tracker")], error="closed_handle")
    spec = {
        "group": group,
        "steps": steps,
        "run": run,
        "grade_good": grade_good,
        "factory_answer": "OK 中文🙂",
    }
    (work / "host.json").write_text(json.dumps(config, ensure_ascii=False))
    (work / "scenario.json").write_text(json.dumps(spec, ensure_ascii=False, indent=2))


def verify(work):
    spec = json.loads((work / "scenario.json").read_text())
    facts = json.loads((work / "observed.json").read_text())
    rows = {row["id"]: row for row in facts["rows"]}
    assert [row["id"] for row in facts["rows"]] == [
        step["id"] for step in spec["steps"]
    ]
    for step in spec["steps"]:
        row = rows[step["id"]]
        if "error" in step:
            assert row["outcome"] == "error" and row["code"] == step["error"], row
        else:
            assert row["outcome"] == "result", row
    value = lambda name: rows[name]["value"]
    if spec["group"] == "eval":
        assert value("configured-timeout") == "1"
        report = value("run-all")
        assert [r["success"] for r in report["results"]] == [True, False]
        timed = value("timeout")["results"][0]
        assert timed["success"] is False and "Timeout" in timed["violations"]
        assert any(
            "Turn settled after timeout: cancelled" == item
            for item in timed["violations"]
        ), timed
        assert not any(
            "Workspace retained" in item or "did not settle" in item
            for item in timed["violations"]
        ), timed
        assert facts["cancel_observed"] == 1 and facts["factory_finally"] == 3
        assert facts["deadline_waiter_pending_during_cleanup"] is True
        assert facts["workspace_exists_during_cleanup"] is True
        assert value("direct")["success"] is True
        assert value("graded-run")["success"] is True
        assert any(metric["score"] == 0.5 for metric in value("graded-run")["metrics"])
        for name in ("grade", "grade-null", "grade-trajectory"):
            assert value(name)["pass_rate"] == 0.5 and [
                r["confidence"] for r in value(name)["results"]
            ] == [0.875, 0.625]
        assert (
            value("grade-bad")["pass_rate"] == 0.0
            and "valid JSON" in value("grade-bad")["overall_assessment"]
        )
        assert len(facts["created"]) == len(facts["closed"]) == 3 and set(
            facts["created"]
        ) == set(facts["closed"])
        assert len(value("constraints")) > 0
    elif spec["group"] == "improve":
        for name, count in [("early", 1), ("maximum", 2)]:
            loop = value(name)
            assert set(loop) == {
                "iterations",
                "best_score",
                "best_iteration",
                "total_duration_ms",
            }
            assert len(loop["iterations"]) == count and loop["best_score"] == 1.0
            for item in loop["iterations"]:
                assert (
                    set(item)
                    == {
                        "iteration",
                        "eval_report",
                        "train_score",
                        "critiques",
                        "suggestions",
                        "duration_ms",
                    }
                    and item["train_score"] == 1.0
                )
        assert (
            len(facts["created"]) == len(facts["closed"]) == 12
            and facts["factory_finally"] == 12
        )
        assert value("stats") == value("reopened-stats") and value("list") == value(
            "reopened-list"
        )
        assert len(value("list")) == 1 and value("save") is True
        assert value("skip-nonterminal") is False
        assert (
            len(value("sharegpt")) == 4
            and "write_file" in value("sharegpt")[1]["value"]
        )
        assert value("analyze")["issues"] and value("analyze")["suggestions"]
        assert value("analyze-failed")["success"] is False
        assert facts["trajectory_bytes_before"] == facts["trajectory_bytes_after"]
        assert len(facts["trajectory_bytes_before"]) == 1
        disk = base64.b64decode(facts["trajectory_bytes_before"][0], validate=True)
        assert disk.endswith(b"\n") and len(disk.splitlines()) == 1
        entry = json.loads(disk)
        assert set(entry) == {
            "id",
            "session_id",
            "conversations",
            "model",
            "completed",
            "timestamp",
            "token_usage",
            "tool_call_count",
            "duration_ms",
        }
        assert (
            entry["id"] == spec["run"]["run_id"]
            and entry["session_id"] == spec["run"]["session_id"]
        )
        assert (
            entry["conversations"] == value("sharegpt")
            and entry["model"] == "model🙂"
            and entry["completed"] is True
        )
        assert (
            entry["token_usage"] == 22
            and entry["tool_call_count"] == 1
            and entry["duration_ms"] == 123
        )
        assert value("stats") == {
            "total": "1",
            "completed": "1",
            "failed": "0",
            "total_tokens": "22",
            "total_tool_calls": "1",
            "avg_duration_ms": "123",
        }
    else:
        stats = value("stats")
        assert (
            stats == {"node_count": "2", "edge_count": "1", "total_calls": "2"}
            and value("after-foreign") == stats
        )
        snapshot = value("snapshot")
        assert sorted(snapshot["nodes"], key=lambda n: n["id"]) == sorted(
            value("nodes"), key=lambda n: n["id"]
        ) and snapshot["edges"] == value("edges")
        assert len(snapshot["nodes"]) == 2 and len(snapshot["edges"]) == 1
        assert snapshot["edges"][0]["total_duration_ms"] == "123"
        assert snapshot["edges"][0]["call_count"] == "2"
        assert "agent🙂" in value("to_mermaid") and "subagent🌊" in value("to_dot")
        assert value("empty") == {
            "nodes": [],
            "edges": [],
            "stats": {"node_count": "0", "edge_count": "0", "total_calls": "0"},
        }
        assert facts["created"] == facts["closed"] == []
    print(
        json.dumps(
            {
                "group": spec["group"],
                "verified": True,
                "factory_created": len(facts["created"]),
                "factory_closed": len(facts["closed"]),
                "rows": len(facts["rows"]),
            },
            ensure_ascii=False,
        )
    )


def compare(paths):
    receipts = [json.loads(Path(path).read_text()) for path in paths]
    grouped = {}
    for receipt in receipts:
        grouped.setdefault(receipt["group"], []).append(receipt)

    def normalize(value):
        if isinstance(value, list):
            items = [normalize(item) for item in value]
            if items and isinstance(items[0], dict) and "id" in items[0]:
                items.sort(key=lambda item: item["id"])
            return items
        if isinstance(value, dict):
            return {
                key: normalize(item)
                for key, item in value.items()
                if key
                not in {
                    "duration_ms",
                    "total_duration_ms",
                    "timestamp",
                    "last_called",
                    "first_call",
                    "last_call",
                }
            }
        return value

    selected = {
        "eval": [
            "configured-timeout",
            "run-all",
            "direct",
            "graded-run",
            "constraints",
            "grade",
            "grade-null",
            "grade-trajectory",
            "grade-bad",
        ],
        "improve": [
            "early",
            "maximum",
            "max-get",
            "holdout-get",
            "threshold-get",
            "sharegpt",
            "analyze",
            "analyze-failed",
            "stats",
            "reopened-stats",
        ],
        "topology": ["nodes", "edges", "stats", "after-foreign", "empty"],
    }
    for group, rows in grouped.items():
        assert {row["language"] for row in rows} == {"typescript", "python", "java"}, (
            group
        )
        values = []
        for receipt in rows:
            actual = {row["id"]: row for row in receipt["rows"]}
            values.append(
                {name: normalize(actual[name]["value"]) for name in selected[group]}
            )
        assert all(value == values[0] for value in values[1:]), (group, values)
    assert set(grouped) == {"eval", "improve", "topology"}
    print(
        "three native language fixed Eval, Improvement, Trajectory and Topology results agree; durations and automatic timestamps remain raw evidence"
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--prepare", choices=["eval", "improve", "topology"])
    parser.add_argument("--verify", action="store_true")
    parser.add_argument("--compare", nargs="+")
    args = parser.parse_args()
    if args.prepare:
        prepare(args.work, args.prepare)
    if args.verify:
        verify(args.work)
    if args.compare:
        compare(args.compare)


if __name__ == "__main__":
    main()
