"""Shared deterministic payloads and assertions; native SDKs drive the real Host.

This fixture has no React loop, scheduler, stream owner or simulated Host. It
prepares an unused model transport URL so fallback is an actual failure.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def wire(value):
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
        return {"kind": "list", "value": [wire(item) for item in value]}
    return {
        "kind": "map",
        "value": [
            {"key": wire(key), "value": wire(item)} for key, item in value.items()
        ],
    }


def fields(value):
    if isinstance(value, dict) and value.get("kind") == "variant":
        value = value["value"]
    assert isinstance(value, dict) and "variant" in value, value
    return value["variant"], {item["name"]: item["value"] for item in value["fields"]}


def unbox(value):
    if isinstance(value, dict) and "kind" in value:
        kind = value["kind"]
        if kind in ("string", "u64", "i64", "f64", "bool", "handle", "duration"):
            return value["value"]
        if kind == "null":
            return None
        if kind == "map":
            return {unbox(item["key"]): unbox(item["value"]) for item in value["value"]}
        if kind == "list":
            return [unbox(item) for item in value["value"]]
    return value


def prepare(work):
    work.mkdir(parents=True, exist_ok=True)
    usage = {
        "value": wire(
            {
                "prompt_tokens": 17,
                "completion_tokens": 5,
                "total_tokens": 22,
                "prompt_tokens_details": {"cached_tokens": 3},
            }
        )
    }
    answer = "回答中文🙂"
    response = {
        "message": {
            "role": "assistant",
            "content": wire(answer),
            "name": "assistant-name🙂",
        },
        "finish_reason": "stop",
        "usage": usage,
        "raw": wire(
            {
                "provider": "fixture",
                "ratio": 0.375,
                "optional": None,
                "timestamp": "2026-10-13T00:00:00.123456789Z",
                "identity": "9007199254740993",
            }
        ),
    }
    spec = {
        "answer": answer,
        "system": "system 中文🙂",
        "prompt": "request 用户🌊",
        "history_prompt": "history follow-up🙂",
        "temperature": 0.375,
        "max_tokens": 513,
        "compression": {
            "evict": "old context 待淘汰🙂",
            "retain": "retained context 保留🌊",
            "next": "after compression 下一轮🧪",
            "focus": "preserve retained decisions 保留🙂",
            "token_limit": "100000",
            "compressor_limit": "80000",
        },
        "response": response,
        "llm_chunks": [
            {"kind": "llm", "value": {"role": "assistant", "content": "回答"}},
            {"kind": "llm", "value": {"content": "中文🙂"}},
        ],
        "llm_complete": {
            "kind": "llm",
            "value": {"finish_reason": "stop", "usage": usage},
        },
        "failure": {
            "code": "extension_failed",
            "message": "source-cause 中文🙂",
            "retryable": "never",
            "details": {
                "fields": [{"key": "fixture_cause", "value": "exact-source🙂"}]
            },
        },
        "workflow_output": wire(
            {
                "result": "workflow 中文🙂",
                "steps": [
                    {
                        "agent_name": "node🙂",
                        "input": "input🙂",
                        "output": answer,
                        "elapsed": {"secs": 0, "nanos": 123},
                    }
                ],
                "elapsed": {"secs": 0, "nanos": 456},
            }
        ),
        "workflow_chunks": [
            {
                "kind": "agent_component",
                "value": {
                    "component": "workflow",
                    "event": {
                        "event": "node_start",
                        "node_name": "node🙂",
                        "step_index": "0",
                    },
                },
            },
            {
                "kind": "agent_component",
                "value": {
                    "component": "workflow",
                    "event": {"event": "token", "node_name": "node🙂", "token": answer},
                },
            },
        ],
        "workflow_error_chunk": {
            "kind": "agent_component",
            "value": {
                "component": "workflow",
                "event": {
                    "event": "node_error",
                    "node_name": "node🙂",
                    "error": "source-cause 中文🙂",
                },
            },
        },
        "workflow_complete": {
            "kind": "agent_component",
            "value": {
                "component": "workflow",
                "terminal": {
                    "result": "workflow 中文🙂",
                    "total_steps": "1",
                    "elapsed": {"seconds": "0", "nanos": 456},
                },
            },
        },
        "graph": {
            "name": "pure-graph🙂",
            "nodes": [
                {"name": "start🙂", "type": "router"},
                {"name": "end🙂", "type": "router"},
            ],
            "edges": [{"from": "start🙂", "to": "end🙂"}],
            "entry": "start🙂",
            "finish": ["end🙂"],
        },
        "state": {"input": "graph 中文🙂", "optional": None, "ratio": 0.375},
    }
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "fixture-model",
                "base_url": "http://127.0.0.1:1/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "fixture-secret",
            },
            "agent": {
                "name": "llm-workflow-proof",
                "system_prompt": spec["system"],
                "enable_tools": True,
                "enable_memory": False,
                "max_iterations": 3,
            },
        },
        "sdk_profile": {"state_root": str((work / "state").resolve())},
    }
    (work / "host.json").write_text(json.dumps(config, ensure_ascii=False))
    (work / "scenario.json").write_text(json.dumps(spec, ensure_ascii=False))


def verify_llm(spec, rows):
    assert [row["mode"] for row in rows] == [
        "nonstream",
        "stream",
        "missing_finish",
        "error",
        "cancel",
    ]
    for row in rows:
        assert len(row["calls"]) == (2 if row["mode"] == "nonstream" else 1), row
        call = row["calls"][0]
        expected = "llm_chat_stream" if row["mode"] == "stream" else "llm_chat"
        assert call["operation"] == expected
        request = call["request"]
        assert (
            request["temperature"] == spec["temperature"]
            and request["max_tokens"] == spec["max_tokens"]
        )
        assert set(request) <= {
            "messages",
            "temperature",
            "max_tokens",
            "tools",
            "tool_choice",
            "user_id",
            "response_format",
            "thinking",
            "timeouts",
            "cache_hints",
        }
        assert isinstance(request["tools"], list) and len(request["tools"]) > 0
        for tool in request["tools"]:
            assert set(tool) == {"tool_type", "name", "description", "parameters"}
            assert (
                tool["tool_type"] == "function"
                and isinstance(tool["name"], str)
                and tool["name"]
            )
            assert isinstance(tool["description"], str) and isinstance(
                unbox(tool["parameters"]), dict
            )
        cache = unbox(request["cache_hints"])
        assert set(cache) == {"breakpoints", "stable_prefix_hash", "segments"}
        assert isinstance(cache["stable_prefix_hash"], str)
        messages = request["messages"]
        assert any(
            message["role"] == "system" and spec["system"] in unbox(message["content"])
            for message in messages
        )
        assert (
            messages[-1]["role"] == "user"
            and unbox(messages[-1]["content"]) == spec["prompt"]
        )
        assert row["finally"] == len(row["calls"])
        terminal = row["settled"]["terminal"]
        if row["mode"] in ("nonstream", "stream"):
            assert terminal["status"] == "completed"
            assert str(row["usage"]["tokens_used"]) == "22", row["usage"]
            events = row["replay"]["events"]
            payloads = [
                event.get("payload", event.get("envelope", {}).get("payload"))
                for event in events
            ]
            assert (
                sum(payload["event_type"] == "final_answer" for payload in payloads)
                == 1
            )
            assert spec["answer"] in json.dumps(payloads, ensure_ascii=False)
            if row["mode"] == "stream":
                assert [event["event"] for event in row["sent"]] == [
                    "chunk",
                    "chunk",
                    "complete",
                ]
                assert [event["sequence"] for event in row["sent"]] == ["1", "2", "3"]
            else:
                history = row["calls"][1]["request"]["messages"]
                assert unbox(history[-1]["content"]) == spec["history_prompt"]
                assert any(
                    message["role"] == "assistant"
                    and unbox(message["content"]) == spec["answer"]
                    for message in history
                )
                assert row["history_settled"]["terminal"]["status"] == "completed"
        elif row["mode"] == "cancel":
            assert terminal["status"] == "cancelled" and row["cancel_observed"] is True
        else:
            assert terminal["status"] == "failed"
            message = json.dumps(terminal, ensure_ascii=False)
            assert (
                "bounded finish reason"
                if row["mode"] == "missing_finish"
                else spec["failure"]["message"]
            ) in message


def verify_workflow(spec, facts):
    assert unbox(facts["run"])["result"] == "workflow 中文🙂"
    assert len(facts["calls"]) == 5, facts["calls"]
    for mode, variants in [
        ("success", ["item", "item", "item", "complete"]),
        ("failure", ["item", "failed"]),
    ]:
        events = facts[mode]
        assert [fields(event)[0] for event in events] == variants
        assert [unbox(fields(event)[1]["sequence"]) for event in events] == [
            str(index + 1) for index in range(len(events))
        ]
        owners = [unbox(fields(event)[1]["stream"]) for event in events]
        assert (
            all(owner == owners[0] for owner in owners)
            and owners[0]["kind"] == "stream"
        )
    actual = [
        fields(unbox(fields(event)[1]["value"])) for event in facts["success"][:-1]
    ]
    assert actual[0][0] == "node_start" and {
        key: unbox(value) for key, value in actual[0][1].items()
    } == {"node_name": "node🙂", "step_index": "0"}
    assert actual[1][0] == "token" and {
        key: unbox(value) for key, value in actual[1][1].items()
    } == {"node_name": "node🙂", "token": spec["answer"]}
    assert actual[2][0] == "completed" and {
        key: unbox(value) for key, value in actual[2][1].items()
    } == {
        "result": "workflow 中文🙂",
        "total_steps": "1",
        "elapsed": {"seconds": "0", "nanos": 456},
    }
    node_error = fields(unbox(fields(facts["failure"][0])[1]["value"]))
    assert node_error[0] == "node_error"
    assert unbox(node_error[1]["error"]) == spec["failure"]["message"]
    failed = unbox(fields(facts["failure"][1])[1]["value"])
    assert (
        failed["code"] == "framework_error"
        and spec["failure"]["message"] in failed["message"]
    )
    assert facts["wrong_owner"]["code"] == "invalid_value"
    assert (
        facts["callback_count_before_wrong_owner"]
        == facts["callback_count_after_wrong_owner"]
    )
    assert (
        facts["cancel"] is True and fields(facts["cancel_terminal"])[0] == "cancelled"
    )
    assert unbox(fields(facts["cancel_terminal"])[1]["sequence"]) == "2"
    assert facts["control_terminals"] == {"cancel": 1, "close": 1}
    for event in facts["control_items"].values():
        variant, item = fields(event)
        assert variant == "item" and unbox(item["sequence"]) == "1"
        assert fields(unbox(item["value"])) == actual[0]
    assert set(facts["control_items"]) == {"cancel", "close"}
    assert facts["close"] is False and facts["close_again"] is False
    assert facts["after_close"]["code"] == "closed_handle"
    assert facts["producer_completed"] == 4 and facts["producer_errors"] == []
    assert facts["callbacks_completed"] == 5


def verify_graph(spec, facts):
    assert facts["wrong_owner"]["code"] == "invalid_value"
    assert facts["wrong_cancel"]["code"] == "invalid_value"
    assert facts["cancel"] == {"cancelled": True}
    events = facts["events"]
    assert [fields(event)[0] for event in events] == ["item"] * 5 + ["complete"]
    assert [unbox(fields(event)[1]["sequence"]) for event in events] == [
        str(index + 1) for index in range(6)
    ]
    assert [fields(unbox(fields(event)[1]["value"]))[0] for event in events[:-1]] == [
        "node_start",
        "node_end",
        "node_start",
        "node_end",
        "completed",
    ]
    failure_events = facts["cancelled_events"]
    assert fields(failure_events[-1])[0] == "failed"
    error = unbox(fields(failure_events[-1])[1]["value"])
    assert error["code"] == "cancelled" and "cancel" in error["message"].lower()
    assert facts["callback_count"] == 0


def verify_observation(spec, facts):
    """All native clients must retain the original producer's terminal payload."""
    assert len(facts["calls"]) == 1
    assert facts["held"] == {"physical_completed": 0, "actual_complete_sent": 0}
    assert facts["physical_completed"] == facts["actual_complete_sent"] == 1
    binding = facts["binding"]
    assert binding["stream"]["kind"] == "stream"
    assert binding["extension"]["kind"] == "extension"
    error = facts["observed"]
    assert error["code"] == "framework_error" and error["retryable"] == "never"
    assert error["operation"] == "workflow.stream.next"
    assert error["handle"] == facts["facade_stream"]
    assert error["details"] == {
        "fields": [
            {"key": "observation", "value": "not_settled"},
            {"key": "run_id", "value": binding["run_id"]},
            {"key": "execution_id", "value": binding["execution_id"]},
        ]
    }
    variant, item = fields(facts["item"])
    assert variant == "item", "real Completed payload replaced by generic terminal"
    assert unbox(item["sequence"]) == "1"
    assert unbox(item["stream"]) == facts["facade_stream"]
    completed = unbox(item["value"])
    raw = completed.get("value", completed)
    assert raw["type_id"] == "echo_orchestration::workflow::WorkflowEvent"
    name, payload = fields(completed)
    assert name == "completed"
    assert {key: unbox(value) for key, value in payload.items()} == spec[
        "workflow_complete"
    ]["value"]["terminal"]
    name, terminal = fields(facts["terminal"])
    assert name == "complete" and unbox(terminal["sequence"]) == "2"
    assert unbox(terminal["stream"]) == facts["facade_stream"]
    assert facts["close"] is False


def verify_compressor(spec, facts):
    expected = spec["compression"]
    assert facts["decoy_calls"] == 0
    assert len(facts["compressions"]) == 3
    assert len(facts["model_requests"]) == 3
    assert len(facts["settlements"]) == 3
    assert all(row["terminal"]["status"] == "completed" for row in facts["settlements"])
    assert facts["binding"] == facts["original_extension"]
    for index, call in enumerate(facts["compressions"]):
        assert call["extension"] == facts["original_extension"]
        assert call["invocation_id"] and call["deadline"]
        assert int(call["input"]["token_limit"]) + int(call["protected_tokens"]) == int(
            expected["compressor_limit"]
        ), {
            "index": index,
            "limit": call["input"]["token_limit"],
            "protected_tokens": call["protected_tokens"],
        }
        assert call["input"]["current_query"] == (
            None if index == 0 else expected["retain"]
        )
        assert call["input"]["focus_instructions"] == (
            expected["focus"] if index == 1 else None
        )
        assert call["token_count"] == "1"
        assert call["wrong_owner"]["code"] == "invalid_value"
        assert facts["released_tokenizers"][index]["code"] == "closed_handle"
        assert call["output"]["checkpoint"] is None
        before = call["input"]["messages"]
        retained, evicted = call["output"]["messages"], call["output"]["evicted"]
        assert len(before) == len(retained) + len(evicted)
        assert index == 0 or any(
            unbox(message["content"]) == expected["retain"] for message in retained
        )
        assert not any(
            unbox(message["content"]) == expected["evict"] for message in retained
        )
        assert len(evicted) == (2 if index == 1 else 0)
        stats = facts["compression_results"][index]["stats"]
        source_before = facts["before_messages"][index]
        source_after = facts["after_messages"][index]
        excluded = [
            message
            for message in source_before
            if not any(
                candidate["role"] == message["role"]
                and unbox(candidate["content"]) == message["content"]
                for candidate in before
            )
        ]
        assert call["protected_messages"] == excluded
        assert int(stats["before_count"]) == len(source_before)
        assert int(stats["after_count"]) == len(source_after)
        assert int(stats["evicted"]) == len(evicted)
        if index:
            assert int(stats["before_count"]) - int(stats["after_count"]) == len(
                evicted
            )
    first = facts["compressions"][1]
    assert any(
        unbox(message["content"]) == expected["evict"]
        for message in first["output"]["evicted"]
    )
    # The independent model callback observes framework consumption, rather
    # than trusting the compressor's own output or a locally reconstructed list.
    next_messages = facts["model_requests"][2]["messages"]
    next_text = [unbox(message["content"]) for message in next_messages]
    assert expected["evict"] not in next_text
    assert "answer: " + expected["evict"] not in next_text
    assert expected["retain"] in next_text
    assert "answer: " + expected["retain"] in next_text
    assert next_text[-1] == expected["next"]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument(
        "--verify", choices=("llm", "workflow", "graph", "observation", "compressor")
    )
    arguments = parser.parse_args()
    if arguments.prepare:
        prepare(arguments.work)
    if arguments.verify:
        spec = json.loads((arguments.work / "scenario.json").read_text())
        facts = json.loads((arguments.work / "observed.json").read_text())
        globals()["verify_" + arguments.verify](spec, facts)
        print(
            json.dumps(
                {"group": arguments.verify, "verified": True}, ensure_ascii=False
            )
        )


if __name__ == "__main__":
    main()
