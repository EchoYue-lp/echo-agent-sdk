"""Temporary real-Host input/fault fixture; owns no SDK/framework state logic."""
import argparse
import json
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
    second = dict(checkpoint, conversation_id="runtime-B-🙂", current_plan=None,
                  blocked_reason=None, active_skills=[], messages_json="[]")
    trace = {
        "run_id": "trace-中文🙂", "parent_run_id": "parent-🧭",
        "agent_name": "测量agent", "model": "fixture-model", "provider": "fixture-provider",
        "turn_id": "turn-中文", "execution_id": "subagent-execution-🙂",
        "session_id": "trace-session-中文", "status": "completed",
        "input": "输入🙂 sk-abcdefghijklmnopqrstuvwxyz123456",
        "events": [
            {"type": "context_compression", "source": "压缩🙂", "before_messages": 9,
             "after_messages": 4, "before_tokens": 123, "after_tokens": 57,
             "protected_context_tokens": 21, "protected_message_count": 2},
            {"type": "tool_call", "call_id": "call-🧭", "name": "read_file",
             "args": {"path": "/临时🙂", "query": "中文"}, "risk": "read_only", "duration_ms": 13},
            {"type": "error", "message": "诊断🙂 sk-abcdefghijklmnopqrstuvwxyz123456"},
        ],
        "final_output": "结果中文🧭", "error": None,
        "token_usage": {"prompt_tokens": 123, "completion_tokens": 57, "total_tokens": 180,
                        "cached_prompt_tokens": 31, "cache_creation_prompt_tokens": 11,
                        "usage_reported_calls": 2, "usage_missing_calls": 1},
        "timings": {"total_duration_ms": 9007199254740993,
                    "llm_duration_ms": 127, "tool_duration_ms": 13},
        "started_at": "2026-10-13T13:00:00.123456789+08:00",
        "finished_at": "2026-10-13T13:00:01.987654321+08:00",
    }
    expected_trace = json.loads(json.dumps(trace))
    expected_trace["input"] = "输入🙂 [REDACTED]"
    expected_trace["events"][2]["message"] = "诊断🙂 [REDACTED]"
    write(work / "fixture.json", {"scope_a": "scope-A-中文🙂", "scope_b": "scope-B-🙂",
                                  "checkpoint_a": checkpoint, "checkpoint_b": second,
                                  "trace": trace, "trace_expected": expected_trace,
                                  "delivery": {"message_id": "message-中文🙂", "route": "channel/local",
                                               "payload": {"text": "待投递🧭", "nullable": None}},
                                  "trace_dir": str((work / "traces").resolve())})
    write(work / "host.json", {"schema_version": 1,
          "default_agent": {"model": {"provider": "fixture", "name": "fixture",
              "base_url": "http://127.0.0.1:1/v1", "api_protocol": "chat_completions", "auth_token": "fixture"},
              "agent": {"name": "fixture", "enable_tools": True}},
          "sdk_profile": {"state_root": str((work / "state").resolve()),
                          "limits": {"max_facade_resources": 1}}})
    print(work / "host.json")


def block(work, target):
    path = work / target
    backup = path.with_name(path.name + ".fixture-backup")
    if backup.exists() or not path.is_dir():
        raise ValueError("fault must target one existing directory without a previous backup")
    path.rename(backup)
    path.write_text("fixture regular file blocks directory I/O\n")
    write(work / "fault.json", {"target": target, "backup": str(backup), "fault": "regular-file-directory-block"})


def unblock(work):
    record = json.loads((work / "fault.json").read_text())
    target = work / record["target"]
    if not target.is_file():
        raise ValueError("blocked regular file was replaced; refuse cleanup")
    target.unlink()
    Path(record["backup"]).rename(target)
    (work / "fault.json").unlink()


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--work", type=Path, required=True)
    p.add_argument("action", choices=["prepare", "block-owners", "block-trace", "unblock"])
    a = p.parse_args()
    if a.action == "prepare": prepare(a.work)
    elif a.action == "block-owners": block(a.work, "state/runtime_state/_runtime_owners")
    elif a.action == "block-trace": block(a.work, "traces")
    else: unblock(a.work)

if __name__ == "__main__": main()
