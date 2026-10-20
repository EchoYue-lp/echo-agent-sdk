import asyncio
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

import pytest

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required Host LSP E2E binary missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]
FIXTURE = Path(__file__).parents[2] / "shared/testing/lsp_fault_server.py"


async def test_lsp_common_values_failures_and_manager_lifecycle():
    from echo_agent_sdk import EchoAgentClient, EchoAgentError

    work = Path(tempfile.mkdtemp(prefix="sdk-py-lsp-"))
    print(f"LSP evidence {work}")
    await asyncio.to_thread(
        subprocess.run,
        [sys.executable, str(FIXTURE), "--prepare", "--work", str(work)],
        check=True,
    )
    async with await EchoAgentClient.spawn(
        HOST, "--config", str(work / "host.json")
    ) as sdk:
        agent = await sdk.create_agent()
        session = await agent.create_session()
        manager = await session.invoke("lsp.manager.open_exact")
        await session.invoke(
            "lsp.manager.load_config",
            [manager, json.loads((work / "lsp.json").read_text())],
        )
        await session.invoke("lsp.manager.start_server", [manager, "test"])
        option = await session.invoke("lsp.manager.get_client", [manager, "test"])
        assert option["kind"] == "variant"
        assert option["value"]["variant"] == "some"
        client = next(
            field["value"]["value"]
            for field in option["value"]["fields"]
            if field["name"] == "client"
        )
        observed = []
        for item in json.loads((work / "corpus.json").read_text()):
            operation = f"lsp.client.{item['operation']}"
            args = [client, f"file:///{item['case']}", {"line": 0, "character": 3}]
            if "error" in item:
                with pytest.raises(EchoAgentError) as error:
                    await asyncio.wait_for(session.invoke(operation, args), 3)
                assert error.value.code == item["error"]
                assert "Server returned error" in str(error.value)
                assert error.value.retryable == "never"
                observed.append(
                    {
                        "operation": item["operation"],
                        "case": item["case"],
                        "error": error.value.code,
                        "message": error.value.message,
                        "retryable": error.value.retryable,
                        "error_operation": error.value.operation,
                        "details": error.value.details,
                    }
                )
                assert await session.invoke(
                    "lsp.client.hover",
                    [client, "file:///hover-string", {"line": 0, "character": 3}],
                ) == {"contents": "中文😀", "range": None}
            else:
                value = await session.invoke(operation, args)
                assert value == item["expected"]
                observed.append(
                    {
                        "operation": item["operation"],
                        "case": item["case"],
                        "value": value,
                    }
                )
        pid = int((work / "server.pid").read_text())
        os.kill(pid, 0)
        await session.invoke("lsp.manager.shutdown_all", [manager])
        with pytest.raises(ProcessLookupError):
            os.kill(pid, 0)
        with pytest.raises(EchoAgentError) as error:
            await session.invoke(
                "lsp.client.initialize", [client, "file:///cannot-revive"]
            )
        assert error.value.code == "framework_error"
        assert "not initialized" in str(error.value).lower()
        assert int((work / "server.pid").read_text()) == pid
        observed.append(
            {
                "case": "manager-close",
                "process_alive_before": True,
                "process_alive_after": False,
                "stale_error": "framework_error",
                "resurrected": False,
            }
        )
        (work / "observed.json").write_text(json.dumps(observed, ensure_ascii=False))
        await session.close()
        await agent.close()
