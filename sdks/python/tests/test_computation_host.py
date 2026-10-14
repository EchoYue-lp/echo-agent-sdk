"""Native public SDK computation consumer; shared verifier never calls Host."""

import hashlib
import json
import os
import sys
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError, from_wire, to_wire

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "shared/testing"))
import computation_fixture as fixture
from file_sha256 import file_sha256

HOST = os.environ.get("ECHO_AGENT_SDK_HOST", "").strip() or None
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required computation Host binary is missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]


async def test_computation_host_exact_values_errors_and_effects(tmp_path):
    work = tmp_path
    fixture.prepare(work)
    rows = []
    async with await EchoAgentClient.spawn(
        HOST, "--config", str(work / "host.json")
    ) as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session(str(work))
        identity = {
            "agent": agent.wire.to_dict(),
            "session": session.wire.to_dict(),
            "capability": sdk.capability,
        }
        for step in json.loads((work / "scenario.json").read_text())["rows"]:
            args = step["arguments"]
            row = {**step, "protected_before": fixture.snapshot(work)}
            if step.get("native_dimension_indices"):
                row["arguments_wire"] = [to_wire(value) for value in args]
            try:
                if step["operation"] == "structured_output.validate":
                    value = await sdk.request(
                        "_echo_agent/structured_output/validate",
                        {
                            "operation": step["operation"],
                            "signature_digest": "sha256:" + "0123456789abcdef" * 4,
                            "handle": session.wire.to_dict(),
                            "arguments": [to_wire(item) for item in args],
                        },
                    )
                    result = from_wire(value["value"])
                else:
                    result = await session.invoke(step["operation"], args)
                row["result"] = result
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
            row["protected_after"] = fixture.snapshot(work)
            rows.append(row)
        await session.close()
        await agent.close()
    data = {
        "host": HOST,
        "host_sha256": file_sha256(HOST),
        "work": str(work),
        "rows": rows,
        "raw_identity": identity,
        "actual_exports": {
            "output_text": (work / "output.txt").read_text(),
            "output_json": json.loads((work / "output.json").read_text()),
        },
        "actual_side_effects": {
            "sentinel_contents": (work / "sentinel.txt").read_text(),
            "sentinel_sha256": hashlib.sha256(
                (work / "sentinel.txt").read_bytes()
            ).hexdigest(),
        },
    }
    (work / "results.json").write_text(
        json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    )
    print(
        "Computation verified:",
        json.dumps(fixture.verify(work, data)),
        "evidence:",
        work,
    )
