"""Shared inputs/oracle for native public Session/Run idempotency consumers.

Reuses the existing registered-LLM counterexample and byte-preserving gap Host
tee. No model loop, Host, publication registry or transport is implemented here.
Native callbacks count real provider effects; the fixture prepares only config,
exact keys/Unicode payloads and checks complete observations.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def prepare(work: Path) -> None:
    work.mkdir(parents=True, exist_ok=True)
    (work / "changed-cwd").mkdir(exist_ok=True)
    spec = {
        "keys": {
            "session": "session-key 中文🌊",
            "run": "run-key 中文🌊",
            "busy": "pre-effect-retry 中文🌊",
        },
        "prompt": "same keyed request 中文🌊",
        "changed_prompt": "changed keyed request 中文🌊",
        "busy_prompt": "unkeyed parked original 中文🌊",
        "retry_prompt": "keyed retry after busy 中文🌊",
        "response": {
            "message": {
                "role": "assistant",
                "content": {"kind": "string", "value": "one canonical answer 中文🌊"},
            },
            "finish_reason": "stop",
            "raw": {"kind": "map", "value": []},
        },
        "descriptor": {
            "kind": "llm_client",
            "descriptor_version": 1,
            "model_name": "idempotency-native",
            "supports_streaming": False,
        },
    }
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "idempotency-native",
                "base_url": "http://127.0.0.1:1/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "test-only",
            },
            "agent": {
                "name": "idempotency-native",
                "system_prompt": "Answer directly.",
                "enable_tools": True,
            },
        },
        "sdk_profile": {"state_root": str(work / "state")},
    }
    (work / "scenario.json").write_text(
        json.dumps(spec, ensure_ascii=False), encoding="utf-8"
    )
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")


def verify(work: Path) -> dict:
    facts = json.loads((work / "observed.json").read_text(encoding="utf-8"))
    groups = facts["groups"]
    assert set(groups) == {"session", "run", "owner", "busy", "closed"}
    session = groups["session"]
    assert session["first"] == session["duplicate"] == session["concurrent"]
    assert session["creations"] == 1
    assert (
        session["changed"]["code"] == "invalid_request"
        and session["changed"]["retryable"] == "never"
    )
    run = groups["run"]
    assert (
        run["first"]
        == run["concurrent"]
        == run["completed"]
        == run["after_local_close"]
    )
    assert (
        run["provider_calls"] == 1
        and run["notifications_before_retry"] == run["notifications_after_retry"]
    )
    assert (
        run["changed"]["code"] == "invalid_request"
        and run["changed"]["retryable"] == "never"
    )
    assert (
        run["settlement"]["settled"]
        and run["settlement"]["terminal"]["status"] == "completed"
    )
    assert (
        groups["owner"]["distinct_session"]
        and groups["owner"]["distinct_run"]
        and groups["owner"]["provider_calls"] == 1
    )
    assert (
        groups["busy"]["rejection"]["code"] == "invalid_request"
        and groups["busy"]["rejection"]["retryable"] == "after_delay"
    )
    assert (
        groups["busy"]["provider_calls_while_rejected"] == 1
        and groups["busy"]["provider_calls_after_retry"] == 2
    )
    assert groups["busy"]["retry"]["run"] != groups["busy"]["original"]["run"]
    assert (
        groups["closed"]["session"]["code"]
        == groups["closed"]["run"]["code"]
        == "closed_handle"
    )
    assert (
        groups["closed"]["session"]["retryable"]
        == groups["closed"]["run"]["retryable"]
        == "never"
    )
    # Preserve the entire actual error signature, including the original owner.
    agent = facts["agent"]
    checks = (
        (
            session["changed"],
            "invalid_request",
            "never",
            "_echo_agent/session/create",
            agent,
        ),
        (
            run["changed"],
            "invalid_request",
            "never",
            "_echo_agent/run/start",
            session["first"]["session"],
        ),
        (
            groups["busy"]["rejection"],
            "invalid_request",
            "after_delay",
            "_echo_agent/run/start",
            None,
        ),
        (
            groups["closed"]["session"],
            "closed_handle",
            "never",
            "_echo_agent/session/create",
            agent,
        ),
        (
            groups["closed"]["run"],
            "closed_handle",
            "never",
            "_echo_agent/session",
            session["first"]["session"],
        ),
    )
    for error, code, retryable, operation, handle in checks:
        assert error == {
            "code": code,
            "retryable": retryable,
            "operation": operation,
            "details": None,
            "handle": handle,
        }
    index = facts["session_index"]
    assert len(index) == 2 and len({row["session_id"] for row in index}) == 2
    for response, owner in (
        (session["first"], agent),
        (groups["owner"]["session"], facts["other_agent"]),
    ):
        (record,) = [
            row for row in index if row["session_id"] == response["acp_session_id"]
        ]
        assert record["agent_handle_id"] == owner["id"]
        assert json.loads(record["agent_config_fingerprint"]) == {
            "variant": "host_default"
        }
        assert (
            record["cwd"]["encoding"] == "utf8"
            and Path(record["cwd"]["path"]).is_absolute()
        )
        assert (
            response["session"]["kind"] == "session"
            and response["task_run"]["kind"] == "task_run"
        )
        assert (
            response["session"]["generation"]
            == response["task_run"]["generation"]
            == owner["generation"]
        )
    stream = run["first"]["stream"]
    notices = [
        row["params"]
        for row in facts["notifications"]
        if row.get("method") == "_echo_agent/event"
        and row["params"]["stream"] == stream
    ]
    assert notices and len({row["envelope"]["event_id"] for row in notices}) == len(
        notices
    )
    assert [row["envelope"]["sequence"] for row in notices] == [
        str(index + 1) for index in range(len(notices))
    ]
    assert (
        sum(
            row["envelope"]["payload"]["event_type"]
            in {"final_answer", "cancelled", "error"}
            for row in notices
        )
        == 1
    )
    assert all(row["envelope"]["stream_id"] == stream["id"] for row in notices)
    assert facts["provider_calls"] == facts["provider_finally"] == 4
    assert facts["session_index_count"] == 2
    assert len({call["invocation_id"] for call in facts["calls"]}) == 4
    assert all(call["operation"] == "llm_chat" for call in facts["calls"])
    return {
        "groups": 5,
        "provider_calls": 4,
        "session_creations": 2,
        "all_complete_publication_responses_equal": True,
        "duplicate_notices": 0,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    if args.verify:
        print(json.dumps(verify(args.work)))
    else:
        prepare(args.work)


if __name__ == "__main__":
    main()
