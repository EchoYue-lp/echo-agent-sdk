"""Temporary real-Host input/fault fixture; owns no SDK/framework state logic."""

import argparse
import calendar
import datetime
import json
import re
import sys
from pathlib import Path


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n")


def prepare(work):
    if (work / "fixture.json").exists():
        raise ValueError("refuse replacing prior fixture inputs")
    work.mkdir(parents=True, exist_ok=True)
    checkpoint = {
        "conversation_id": "runtime-A-中文🙂",
        "messages_json": '[{"role":"user","content":"保留所有字段🧭"}]',
        "current_plan": "legacy artifact-only text",
        "active_skills": ["技能A", "skill-🙂"],
        "blocked_reason": "等待输入中文",
        "working_dir": str(work.resolve()),
        "timestamp": "2026-10-13T13:00:00.123456789+08:00",
    }
    second = dict(
        checkpoint,
        conversation_id="runtime-B-🙂",
        current_plan=None,
        blocked_reason=None,
        active_skills=[],
        messages_json="[]",
    )
    trace = {
        "run_id": "trace-中文🙂",
        "parent_run_id": "parent-🧭",
        "agent_name": "测量agent",
        "model": "fixture-model",
        "provider": "fixture-provider",
        "turn_id": "turn-中文",
        "execution_id": "subagent-execution-🙂",
        "session_id": "trace-session-中文",
        "status": "completed",
        "input": "输入🙂 sk-abcdefghijklmnopqrstuvwxyz123456",
        "events": [
            {
                "type": "context_compression",
                "source": "压缩🙂",
                "before_messages": 9,
                "after_messages": 4,
                "before_tokens": 123,
                "after_tokens": 57,
                "protected_context_tokens": 21,
                "protected_message_count": 2,
            },
            {
                "type": "tool_call",
                "call_id": "call-🧭",
                "name": "read_file",
                "args": {"path": "/临时🙂", "query": "中文"},
                "risk": "read_only",
                "duration_ms": 13,
            },
            {"type": "error", "message": "诊断🙂 sk-abcdefghijklmnopqrstuvwxyz123456"},
        ],
        "final_output": "结果中文🧭",
        "error": None,
        "token_usage": {
            "prompt_tokens": 123,
            "completion_tokens": 57,
            "total_tokens": 180,
            "cached_prompt_tokens": 31,
            "cache_creation_prompt_tokens": 11,
            "usage_reported_calls": 2,
            "usage_missing_calls": 1,
        },
        "timings": {
            "total_duration_ms": 9007199254740993,
            "llm_duration_ms": 127,
            "tool_duration_ms": 13,
        },
        "started_at": "2026-10-13T13:00:00.123456789+08:00",
        "finished_at": "2026-10-13T13:00:01.987654321+08:00",
    }
    expected_trace = json.loads(json.dumps(trace))
    expected_trace["input"] = "输入🙂 [REDACTED]"
    expected_trace["events"][2]["message"] = "诊断🙂 [REDACTED]"
    summary_expected = {
        key: expected_trace[key]
        for key in (
            "run_id",
            "parent_run_id",
            "session_id",
            "agent_name",
            "model",
            "provider",
            "turn_id",
            "execution_id",
            "status",
            "started_at",
            "finished_at",
            "token_usage",
        )
    }
    summary_expected["input_preview"] = expected_trace["input"]
    summary_expected["total_duration_ms"] = expected_trace["timings"][
        "total_duration_ms"
    ]
    write(
        work / "fixture.json",
        {
            "scope_a": "scope-A-中文🙂",
            "scope_b": "scope-B-🙂",
            "checkpoint_a": checkpoint,
            "checkpoint_b": second,
            "trace": trace,
            "trace_expected": expected_trace,
            "trace_summary_expected": summary_expected,
            "delivery": {
                "message_id": "message-中文🙂",
                "route": "channel/local",
                "payload": {"text": "待投递🧭", "nullable": None},
            },
            "trace_dir": str((work / "traces").resolve()),
        },
    )
    write(
        work / "host.json",
        {
            "schema_version": 1,
            "default_agent": {
                "model": {
                    "provider": "fixture",
                    "name": "fixture",
                    "base_url": "http://127.0.0.1:1/v1",
                    "api_protocol": "chat_completions",
                    "auth_token": "fixture",
                },
                "agent": {"name": "fixture", "enable_tools": True},
            },
            "sdk_profile": {
                "state_root": str((work / "state").resolve()),
                "limits": {"max_facade_resources": 1},
            },
        },
    )
    print(work / "host.json")


def block(work, target):
    path = work / target
    backup = path.with_name(path.name + ".fixture-backup")
    if backup.exists() or not path.is_dir():
        raise ValueError(
            "fault must target one existing directory without a previous backup"
        )
    path.rename(backup)
    path.write_text("fixture regular file blocks directory I/O\n")
    write(
        work / "fault.json",
        {
            "target": target,
            "backup": str(backup),
            "fault": "regular-file-directory-block",
        },
    )


def unblock(work):
    record = json.loads((work / "fault.json").read_text())
    target = work / record["target"]
    if not target.is_file():
        raise ValueError("blocked regular file was replaced; refuse cleanup")
    target.unlink()
    Path(record["backup"]).rename(target)
    (work / "fault.json").unlink()


# Test oracle only: the original framework serializes DateTime<Utc> in the
# Host local offset. Keep nanoseconds independent of datetime microseconds.
_RFC3339 = re.compile(
    r"([0-9]{4})-([0-9]{2})-([0-9]{2})T"
    r"([0-9]{2}):([0-9]{2}):([0-9]{2})"
    r"(?:\.([0-9]{1,9}))?(Z|[+-][0-9]{2}:[0-9]{2})"
)
_TIMESTAMP_FIELD_SETS = {("timestamp",), ("started_at", "finished_at")}
_LOCAL_OFFSETS = {"UTC": "+00:00", "Asia/Shanghai": "+08:00"}


def exact_timestamp(value):
    """Return an exact instant without floating point or fractional truncation."""
    if not isinstance(value, str):
        raise TypeError("timestamp must be an RFC3339 string")
    match = _RFC3339.fullmatch(value)
    if match is None:
        raise AssertionError(f"invalid RFC3339 timestamp: {value!r}")
    year, month, day, hour, minute, second = map(int, match.groups()[:6])
    # Validate the whole-second calendar independently of the captured fraction.
    datetime.datetime(
        year, month, day, hour, minute, second, tzinfo=datetime.timezone.utc
    )
    offset = match.group(8)
    offset_seconds = 0
    if offset != "Z":
        offset_hour, offset_minute = int(offset[1:3]), int(offset[4:6])
        if offset_hour >= 24 or offset_minute >= 60:
            raise AssertionError(f"invalid RFC3339 offset: {offset!r}")
        offset_seconds = (offset_hour * 3600 + offset_minute * 60) * (
            1 if offset[0] == "+" else -1
        )
    seconds = calendar.timegm((year, month, day, hour, minute, second))
    return {
        "epoch_seconds": str(seconds - offset_seconds),
        "nanos": int((match.group(7) or "").ljust(9, "0")),
    }


def compare_timestamp_documents(actual, expected, fields, host_timezone=None):
    """Compare full DTO copies; only explicit root timestamp fields use instants."""
    fields = tuple(fields)
    if fields not in _TIMESTAMP_FIELD_SETS:
        raise AssertionError("timestamp comparison requires one declared DTO field set")
    if not isinstance(actual, dict) or not isinstance(expected, dict):
        raise TypeError("timestamp comparison requires complete DTO objects")
    actual_copy, expected_copy = dict(actual), dict(expected)
    observations = []
    for field in fields:
        # Presence remains a full-document obligation: absent is never null.
        for value in (actual, expected):
            if field not in value:
                raise AssertionError(f"required timestamp field is missing: {field}")
        a, e = actual[field], expected[field]
        if a is None or e is None:
            if field != "finished_at" or a is not None or e is not None:
                raise AssertionError(f"timestamp null mismatch: {field}")
            observations.append({"field": field, "actual_raw": a, "expected_raw": e})
            continue
        a_instant, e_instant = exact_timestamp(a), exact_timestamp(e)
        if host_timezone in _LOCAL_OFFSETS:
            required = _LOCAL_OFFSETS[host_timezone]
            if not a.endswith(required):
                raise AssertionError(
                    f"Host {host_timezone} timestamp must end in {required}: {a!r}"
                )
        actual_copy[field], expected_copy[field] = a_instant, e_instant
        observations.append(
            {
                "field": field,
                "actual_raw": a,
                "expected_raw": e,
                "actual_instant": a_instant,
                "expected_instant": e_instant,
                "host_timezone": host_timezone,
            }
        )
    # Serialized equality keeps unknown keys, null, nested values, array order
    # and bool/number distinctions. Callers already canonicalize wire integers.
    if json.dumps(actual_copy, sort_keys=True, ensure_ascii=False) != json.dumps(
        expected_copy, sort_keys=True, ensure_ascii=False
    ):
        raise AssertionError(
            "complete timestamp DTO mismatch: "
            + json.dumps({"actual": actual, "expected": expected}, ensure_ascii=False)
        )
    return observations


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--work", type=Path, required=True)
    p.add_argument(
        "action",
        choices=[
            "prepare",
            "block-owners",
            "block-trace",
            "unblock",
            "compare-timestamps",
        ],
    )
    a = p.parse_args()
    if a.action == "compare-timestamps":
        request = json.load(sys.stdin)
        print(json.dumps(compare_timestamp_documents(**request), ensure_ascii=False))
    elif a.action == "prepare":
        prepare(a.work)
    elif a.action == "block-owners":
        block(a.work, "state/runtime_state/_runtime_owners")
    elif a.action == "block-trace":
        block(a.work, "traces")
    else:
        unblock(a.work)


if __name__ == "__main__":
    main()
