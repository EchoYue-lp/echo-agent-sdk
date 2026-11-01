"""真实 Host 的 Parquet 消费输入与独立结果检查；本模块不发送 RPC。"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from computation_fixture import wire
from file_sha256 import file_sha256

SDK = Path(__file__).resolve().parents[3]
INPUT = "名称,数值,备注\n你好🧪,42.5,\ncafé,-7.25,保留\n末尾,,中文🙂\n"
EXPECTED = [
    {"名称": "你好🧪", "数值": 42.5, "备注": None},
    {"名称": "café", "数值": -7.25, "备注": "保留"},
    {"名称": "末尾", "数值": None, "备注": "中文🙂"},
]
SENTINEL = b"preserve-existing-output\n"


def prepare(work: Path) -> None:
    work.mkdir(parents=True, exist_ok=True)
    (work / "input.csv").write_bytes(INPUT.encode("utf-8"))
    (work / "invalid.parquet").write_bytes(b"PAR1invalid dataPAR1")
    (work / "sentinel.parquet").write_bytes(SENTINEL)
    config = json.loads((SDK / "echo-sdk-host/config.sdk.example.json").read_text())
    config["sdk_profile"]["state_root"] = str(work / "state")
    config["default_agent"]["agent"]["memory_path"] = str(work / "memory.jsonl")
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")
    cases = [
        ("write", "export_data", ["input.csv", "output.parquet", "parquet"], None),
        ("read", "read_data", ["output.parquet"], None),
        (
            "roundtrip",
            "export_data",
            ["output.parquet", "roundtrip.json", "json"],
            None,
        ),
        ("invalid", "read_data", ["invalid.parquet"], "framework_error"),
        (
            "preserve",
            "export_data",
            ["input.csv", "sentinel.parquet", "unsupported"],
            "framework_error",
        ),
    ]
    files = {
        "input.csv",
        "output.parquet",
        "roundtrip.json",
        "invalid.parquet",
        "sentinel.parquet",
    }
    rows = [
        {
            "id": identity,
            "operation": operation,
            "arguments": [
                str(work / item) if item in files else item for item in arguments
            ],
            "expected_error": error,
        }
        for identity, operation, arguments, error in cases
    ]
    (work / "scenario.json").write_text(
        json.dumps(rows, ensure_ascii=False), encoding="utf-8"
    )


def verify(work: Path, evidence: dict) -> dict:
    assert evidence["host_sha256"] == file_sha256(evidence["host"])
    scenario = json.loads((work / "scenario.json").read_text())
    rows = evidence["rows"]
    assert len(rows) == len(scenario) == 5
    for expected, observed in zip(scenario, rows):
        assert all(observed[key] == value for key, value in expected.items()), observed
        if expected["expected_error"] is None:
            assert "error" not in observed, observed
            result = observed["result"]
            assert result["success"] is True, observed
            assert wire(result["data"]) == json.loads(result["output"]), observed
        else:
            error = observed["error"]
            assert error["code"] == expected["expected_error"], observed
            assert error["operation"] == expected["operation"], observed
            assert error["retryable"] == "never" and error["message"], observed
            assert error["handle"] is None, observed
            assert {
                entry["key"]: entry["value"] for entry in error["details"]["fields"]
            } == {
                "framework_failure_category": "tool",
                "framework_terminal_kind": "failed",
                "framework_failure_code": "tool",
                "framework_retryable": "false",
            }, observed
    read = next(row for row in rows if row["id"] == "read")
    data = json.loads(read["result"]["output"])
    assert data["format"] == "parquet" and data["rows"] == 3 and data["columns"] == 3
    assert data["preview"] == EXPECTED, data
    assert json.loads((work / "roundtrip.json").read_text()) == EXPECTED
    assert (work / "input.csv").read_bytes() == INPUT.encode("utf-8")
    assert (work / "sentinel.parquet").read_bytes() == SENTINEL
    assert (work / "invalid.parquet").read_bytes() == b"PAR1invalid dataPAR1"
    parquet = (work / "output.parquet").read_bytes()
    assert parquet.startswith(b"PAR1") and parquet.endswith(b"PAR1")
    # 物理帧证据加真实 Host 回读；codec metadata 与安全修复资格仍由框架专项验证。
    assert b"\x28\xb5\x2f\xfd" in parquet, "Parquet output has no Zstandard frame"
    return {
        "host_sha256": evidence["host_sha256"],
        "real_host_calls": 5,
        "typed_errors": 2,
        "exact_rows": 3,
        "zstd_frame_present": True,
        "parquet_sha256": hashlib.sha256(parquet).hexdigest(),
        "input_sha256": file_sha256(work / "input.csv"),
        "framework_source_qualified": False,
        "scope": "真实SDK数据往返；不替代新pin、codec metadata、安全或原生验收",
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--verify", type=Path)
    args = parser.parse_args()
    if args.verify:
        print(
            json.dumps(
                verify(args.work, json.loads(args.verify.read_text())),
                ensure_ascii=False,
            )
        )
    else:
        prepare(args.work)
