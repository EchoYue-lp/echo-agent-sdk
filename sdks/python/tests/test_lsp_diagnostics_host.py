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
    raise RuntimeError("required LSP diagnostics Host missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]
FIXTURE = Path(__file__).parents[2] / "shared/testing/lsp_diagnostic_server.py"


async def test_diagnostic_notification_replacement_errors_and_runtime_settlement():
    from echo_agent_sdk import EchoAgentClient, EchoAgentError

    work = Path(tempfile.mkdtemp(prefix="sdk-py-lsp-diagnostics-"))
    print(f"LSP diagnostics evidence {work}")
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

        async def open_manager():
            manager = await session.invoke("lsp.manager.open_exact")
            await session.invoke(
                "lsp.manager.load_config",
                [manager, json.loads((work / "lsp.json").read_text())],
            )
            await session.invoke("lsp.manager.start_server", [manager, "test"])
            option = await session.invoke("lsp.manager.get_client", [manager, "test"])
            assert option["kind"] == "variant" and option["value"]["variant"] == "some"
            client = next(
                field["value"]["value"]
                for field in option["value"]["fields"]
                if field["name"] == "client"
            )
            return manager, client

        manager, client = await open_manager()
        assert json.loads((work / "advertisement.json").read_text()) == {
            "relatedInformation": False
        }
        corpus = json.loads((work / "diagnostic-corpus.json").read_text())
        other = corpus[0]["expected"]
        observed = []
        for item in corpus:
            await session.invoke(
                "lsp.client.hover",
                [client, f"file:///{item['case']}", {"line": 0, "character": 3}],
            )
            if "error" in item:
                with pytest.raises(EchoAgentError) as error:
                    await session.invoke(
                        "lsp.client.diagnostics", [client, item["uri"]]
                    )
                assert error.value.code == item["error"]
                assert "publishDiagnostics" in error.value.message
                observed.append(
                    {
                        "case": item["case"],
                        "error": error.value.code,
                        "message": error.value.message,
                        "retryable": error.value.retryable,
                        "error_operation": error.value.operation,
                        "details": error.value.details,
                    }
                )
                assert (
                    await session.invoke(
                        "lsp.client.diagnostics", [client, "file:///other.test"]
                    )
                    == other
                )
            else:
                value = await session.invoke(
                    "lsp.client.diagnostics", [client, item["uri"]]
                )
                assert value == item["expected"]
                observed.append({"case": item["case"], "value": value})
        pid = int((work / "server.pid").read_text())
        await session.invoke("lsp.manager.shutdown_all", [manager])
        with pytest.raises(ProcessLookupError):
            os.kill(pid, 0)
        for fatal in ["missing-uri", "wrong-uri", "missing-params"]:
            manager, client = await open_manager()
            pid = int((work / "server.pid").read_text())
            with pytest.raises(EchoAgentError) as error:
                await asyncio.wait_for(
                    session.invoke(
                        "lsp.client.hover",
                        [client, f"file:///{fatal}", {"line": 0, "character": 3}],
                    ),
                    3,
                )
            assert error.value.code == "framework_error"
            deadline = asyncio.get_running_loop().time() + 3
            while True:
                status = await session.invoke("lsp.client.status", [client])
                if (
                    not status["running"]
                    or asyncio.get_running_loop().time() >= deadline
                ):
                    break
                await asyncio.sleep(0.01)
            assert status["running"] is False and status["initialized"] is False
            assert (
                status["pid"] is None and "publishDiagnostics" in status["last_error"]
            )
            with pytest.raises(ProcessLookupError):
                os.kill(pid, 0)
            with pytest.raises(EchoAgentError) as error:
                await session.invoke(
                    "lsp.client.diagnostics", [client, "file:///中文😀.test"]
                )
            assert (
                error.value.code == "framework_error"
                and "publishDiagnostics" in error.value.message
            )
            await session.invoke("lsp.manager.shutdown_all", [manager])
            observed.append(
                {
                    "case": fatal,
                    "pending_error": "framework_error",
                    "runtime_running": False,
                    "process_alive": False,
                    "diagnostics_error": "framework_error",
                }
            )
        (work / "observed-diagnostics.json").write_text(
            json.dumps(observed, ensure_ascii=False)
        )
        await session.close()
        await agent.close()
