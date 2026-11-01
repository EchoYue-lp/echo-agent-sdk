"""真实公开 SDK 的 Parquet 消费；共享夹具只核输入与结果，不调用 Host。"""

import json
import os
import sys
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "shared/testing"))
import parquet_roundtrip_fixture as fixture
from file_sha256 import file_sha256

HOST = os.environ.get("ECHO_AGENT_SDK_HOST", "").strip() or None
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Parquet Host binary is missing")


@pytest.mark.asyncio
@pytest.mark.skipif(not HOST, reason="real Host required")
async def test_parquet_roundtrip_host(tmp_path):
    fixture.prepare(tmp_path)
    rows = []
    async with await EchoAgentClient.spawn(
        HOST,
        "--config",
        str(tmp_path / "host.json"),
        catalog_path=os.environ.get("ECHO_AGENT_SDK_CATALOG_PATH"),
    ) as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session(str(tmp_path))
        for step in json.loads((tmp_path / "scenario.json").read_text()):
            row = dict(step)
            try:
                row["result"] = await session.invoke(
                    step["operation"], step["arguments"]
                )
                assert step["expected_error"] is None, step["id"]
            except EchoAgentError as error:
                row["error"] = {
                    "code": error.code,
                    "message": error.message,
                    "operation": error.operation,
                    "retryable": error.retryable,
                    "handle": error.handle.to_dict() if error.handle else None,
                    "details": error.details,
                }
                assert error.code == step["expected_error"], row
            rows.append(row)
        await session.close()
        await agent.close()
    evidence = {
        "host": HOST,
        "host_sha256": file_sha256(HOST),
        "catalog_path": os.environ.get("ECHO_AGENT_SDK_CATALOG_PATH"),
        "rows": rows,
    }
    (tmp_path / "results.json").write_text(
        json.dumps(evidence, ensure_ascii=False), encoding="utf-8"
    )
    print("Parquet evidence:", tmp_path, fixture.verify(tmp_path, evidence))
