"""Accepted root Security aliases: deterministic inputs and lossless oracle only.

The text contains a synthetic test token, never a credential. Native SDKs call
real Host source operations; no scanner, regex, model or transport is copied.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

PREFIX = "中文🙂 "
TOKEN = "sk-" + "A" * 20
SUFFIX = " 尾部🌊正常"
OPERATIONS = [
    "echo_agent::security::" + name
    for name in ("contains_secrets", "scan_secrets", "redact_secrets")
]


def prepare(work: Path) -> None:
    work.mkdir(parents=True, exist_ok=True)
    assert len(PREFIX.encode("utf-8")) == 11
    scenario = {
        "operations": OPERATIONS,
        "texts": [
            {"id": "secret", "text": PREFIX + TOKEN + SUFFIX},
            {"id": "empty", "text": ""},
            {"id": "unicode", "text": "纯中文🙂 无敏感内容🌊"},
        ],
        "token": TOKEN,
        "position": "11",
        "redacted": PREFIX + "[REDACTED: OpenAI API Key]" + SUFFIX,
    }
    (work / "scenario.json").write_text(
        json.dumps(scenario, ensure_ascii=False), encoding="utf-8"
    )
    (work / "sentinel.txt").write_text("protected 中文🙂\n", encoding="utf-8")
    config = {
        "schema_version": 1,
        "default_agent": {
            "model": {
                "provider": "fixture",
                "name": "security-unused-model",
                "base_url": "http://127.0.0.1:1/v1/chat/completions",
                "api_protocol": "chat_completions",
                "auth_token": "test-only",
            },
            "agent": {
                "name": "security-native",
                "system_prompt": "Unused model",
                "enable_tools": True,
            },
        },
        "sdk_profile": {"state_root": str(work / "state")},
    }
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")


def snapshot(work: Path) -> dict:
    index = work / "state/host/run-index.json"
    return {
        "sentinel_sha256": hashlib.sha256(
            (work / "sentinel.txt").read_bytes()
        ).hexdigest(),
        "run_index": json.loads(index.read_text()) if index.exists() else None,
    }


def verify(work: Path) -> dict:
    scenario = json.loads((work / "scenario.json").read_text())
    facts = json.loads((work / "observed.json").read_text())
    rows = facts["rows"]
    assert len(rows) == 15
    for row in rows:
        operation = row["operation"]
        assert operation in OPERATIONS
        if row["mode"] == "value":
            assert "error" not in row
            text = next(
                item["text"]
                for item in scenario["texts"]
                if item["id"] == row["text_id"]
            )
            secret = row["text_id"] == "secret"
            if operation.endswith("contains_secrets"):
                assert row["result"] is secret
            elif operation.endswith("scan_secrets"):
                assert row["result"] == (
                    [
                        {
                            "secret_type": "OpenAI API Key",
                            "matched": TOKEN,
                            "position": "11",
                        }
                    ]
                    if secret
                    else []
                )
                if secret:
                    assert isinstance(row["result"][0]["position"], str)
            else:
                assert row["result"] == (scenario["redacted"] if secret else text)
        else:
            assert "result" not in row
            error = row["error"]
            assert error["code"] == "invalid_value" and error["retryable"] == "never"
            assert (
                error["operation"] == "_echo_agent/facade/invoke"
                and error["handle"] is None
            )
            if row["mode"] == "wrong-signature":
                assert error["details"]["facade"] == {
                    "operation": operation,
                    "signature_digest": "sha256:" + "0" * 64,
                }
            else:
                assert error["details"] is None
    raw = facts["raw_scan"]["value"]
    assert raw["kind"] == "list" and len(raw["value"]) == 1
    entries = raw["value"][0]
    assert entries["kind"] == "map"
    fields = {entry["key"]["value"]: entry["value"] for entry in entries["value"]}
    assert len(entries["value"]) == 3 and set(fields) == {
        "secret_type",
        "matched",
        "position",
    }
    assert fields["position"] == {"kind": "u64", "value": "11"}
    assert fields["secret_type"] == {"kind": "string", "value": "OpenAI API Key"}
    assert fields["matched"] == {"kind": "string", "value": TOKEN}
    assert facts["provider_calls"] == 0
    assert facts["before"] == facts["after"]
    assert facts["before"]["run_index"] in (None, [])
    assert snapshot(work) == facts["after"]
    return {
        "source_operations": 3,
        "value_calls": 9,
        "negative_calls": 6,
        "raw_scan_calls": 1,
        "provider_calls": 0,
        "position_encoding": "u64 decimal text at UTF-8 byte offset 11",
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--snapshot", action="store_true")
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    if args.snapshot:
        print(json.dumps(snapshot(args.work)))
    elif args.verify:
        print(json.dumps(verify(args.work)))
    else:
        prepare(args.work)


if __name__ == "__main__":
    main()
