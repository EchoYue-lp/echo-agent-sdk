"""Native registered LLM effects validate the approved bounded publication keys."""

import asyncio
import copy
import json
import os
import sys
import tempfile
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError, RunHandle, WireHandle

HOST = os.environ.get("ECHO_AGENT_SDK_HOST")
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required idempotency Host missing")
TESTING = Path(__file__).resolve().parents[2] / "shared/testing"
FIXTURE = TESTING / "session_run_idempotency_fixture.py"
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]


def error_fact(error):
    return {
        "code": error.code,
        "retryable": error.retryable,
        "operation": error.operation,
        "details": error.details,
        "handle": error.handle.to_dict() if error.handle else None,
    }


async def test_session_run_keys_replay_original_effect_and_retry_pre_effect_busy():
    root = Path(
        os.environ.get("ECHO_AGENT_SDK_IDEMPOTENCY_ROOT", tempfile.gettempdir())
    )
    root.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="python-", dir=root))

    async def helper(*args):
        process = await asyncio.create_subprocess_exec(
            sys.executable,
            str(FIXTURE),
            "--work",
            str(work),
            *args,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )
        try:
            out, err = await asyncio.wait_for(process.communicate(), 5)
            assert process.returncode == 0, err.decode()
            return out.decode()
        finally:
            if process.returncode is None:
                process.kill()
                await process.wait()

    await helper()
    spec = json.loads((work / "scenario.json").read_text())
    facts = {"groups": {}, "calls": [], "requests": [], "provider_finally": 0}
    held = None
    sessions = []
    async with await EchoAgentClient.spawn(
        sys.executable,
        str(TESTING / "gap_ack_host_proxy.py"),
        HOST,
        str(work / "host.json"),
        env={
            **os.environ,
            "NO_PROXY": "127.0.0.1,localhost",
            "no_proxy": "127.0.0.1,localhost",
        },
    ) as sdk:

        async def handler(call, _cancellation):
            # Generic public registration keeps the complete callback envelope.
            facts["calls"].append(
                {
                    "operation": call["invocation"]["operation"],
                    "invocation_id": call["invocation_id"],
                    "context": call.get("context"),
                    "input": call["invocation"]["input"],
                }
            )
            wait = held
            try:
                if wait is not None:
                    await asyncio.wait_for(wait.wait(), 10)
                return {
                    "outcome": "result",
                    "result": {
                        "operation": "llm_chat",
                        "value": copy.deepcopy(spec["response"]),
                    },
                }
            finally:
                facts["provider_finally"] += 1

        registration = await sdk.register_extension(
            "llm_client", "idempotency-native", spec["descriptor"], handler
        )

        async def rpc(method, params):
            row = {"method": method, "params": copy.deepcopy(params)}
            facts["requests"].append(row)
            try:
                row["response"] = await sdk.request(method.removeprefix("_"), params)
                return row["response"]
            except EchoAgentError as error:
                row["error"] = error_fact(error)
                raise

        async def failure(method, params):
            with pytest.raises(EchoAgentError) as caught:
                await rpc(method, params)
            return error_fact(caught.value)

        async def settled(response):
            return await rpc(
                "_echo_agent/run/wait",
                {"run": response["run"], "timeout": {"seconds": "5", "nanos": 0}},
            )

        async def until(predicate):
            deadline = asyncio.get_running_loop().time() + 5
            while not predicate():
                assert asyncio.get_running_loop().time() < deadline, (
                    "real provider barrier missing"
                )
                await asyncio.sleep(0.005)

        def notices(stream):
            return sum(
                value.get("method") in {"_echo_agent/event", "_echo_agent/gap"}
                and value.get("params", {}).get("stream", {}).get("id") == stream["id"]
                for line in (work / "notifications.jsonl").read_text().splitlines()
                if (value := json.loads(line))
            )

        try:
            agent = await sdk.create_agent()
            session_params = {
                "agent": agent.wire.to_dict(),
                "idempotency_id": spec["keys"]["session"],
            }
            first = await rpc("_echo_agent/session/create", session_params)
            sessions.append(first["session"])
            duplicate, concurrent = await asyncio.gather(
                rpc("_echo_agent/session/create", session_params),
                rpc("_echo_agent/session/create", session_params),
            )
            assert duplicate == concurrent == first
            changed = await failure(
                "_echo_agent/session/create",
                {
                    **session_params,
                    "working_dir": {
                        "encoding": "utf8",
                        "path": str(work / "changed-cwd"),
                    },
                },
            )
            facts["groups"]["session"] = {
                "first": first,
                "duplicate": duplicate,
                "concurrent": concurrent,
                "changed": changed,
                "creations": len(
                    json.loads((work / "state/host/session-index.json").read_text())
                ),
            }
            held = asyncio.Event()
            run_params = {
                "session": first["session"],
                "input": {"kind": "chat", "text": spec["prompt"]},
                "idempotency_id": spec["keys"]["run"],
            }
            run_first = await rpc("_echo_agent/run/start", run_params)
            await until(lambda: len(facts["calls"]) == 1)
            before = notices(run_first["stream"])
            run_concurrent = await rpc("_echo_agent/run/start", run_params)
            assert run_first == run_concurrent and len(facts["calls"]) == 1
            assert notices(run_first["stream"]) == before
            changed_run = await failure(
                "_echo_agent/run/start",
                {
                    **run_params,
                    "input": {"kind": "chat", "text": spec["changed_prompt"]},
                },
            )
            held.set()
            held = None
            settlement = await settled(run_first)
            await rpc("_echo_agent/run/get", {"run": run_first["run"]})
            completed_notices = notices(run_first["stream"])
            completed = await rpc("_echo_agent/run/start", run_params)
            await RunHandle(
                sdk,
                WireHandle.from_dict(run_first["run"]),
                WireHandle.from_dict(run_first["stream"]),
            ).close()
            after_local_close = await rpc("_echo_agent/run/start", run_params)
            facts["groups"]["run"] = {
                "first": run_first,
                "concurrent": run_concurrent,
                "completed": completed,
                "after_local_close": after_local_close,
                "changed": changed_run,
                "settlement": settlement,
                "provider_calls": len(facts["calls"]),
                "notifications_before_retry": completed_notices,
                "notifications_after_retry": notices(run_first["stream"]),
            }
            other_agent = await sdk.create_agent()
            other_session = await rpc(
                "_echo_agent/session/create",
                {
                    "agent": other_agent.wire.to_dict(),
                    "idempotency_id": spec["keys"]["session"],
                },
            )
            sessions.append(other_session["session"])
            prior = len(facts["calls"])
            other_run = await rpc(
                "_echo_agent/run/start",
                {**run_params, "session": other_session["session"]},
            )
            await settled(other_run)
            facts["groups"]["owner"] = {
                "distinct_session": other_session["session"] != first["session"],
                "distinct_run": other_run["run"] != run_first["run"],
                "provider_calls": len(facts["calls"]) - prior,
                "session": other_session,
                "run": other_run,
            }
            held = asyncio.Event()
            prior = len(facts["calls"])
            original = await rpc(
                "_echo_agent/run/start",
                {
                    "session": other_session["session"],
                    "input": {"kind": "chat", "text": spec["busy_prompt"]},
                },
            )
            await until(lambda: len(facts["calls"]) == prior + 1)
            keyed = {
                "session": other_session["session"],
                "input": {"kind": "chat", "text": spec["retry_prompt"]},
                "idempotency_id": spec["keys"]["busy"],
            }
            rejection = await failure("_echo_agent/run/start", keyed)
            rejected_count = len(facts["calls"]) - prior
            held.set()
            held = None
            await settled(original)
            retry = await rpc("_echo_agent/run/start", keyed)
            await settled(retry)
            facts["groups"]["busy"] = {
                "original": original,
                "rejection": rejection,
                "retry": retry,
                "provider_calls_while_rejected": rejected_count,
                "provider_calls_after_retry": len(facts["calls"]) - prior,
            }
            await rpc("_echo_agent/session/close", {"session": first["session"]})
            facts["groups"]["closed"] = {
                "session": await failure("_echo_agent/session/create", session_params),
                "run": await failure("_echo_agent/run/start", run_params),
            }
            facts["provider_calls"] = len(facts["calls"])
            facts["session_index_count"] = len(
                json.loads((work / "state/host/session-index.json").read_text())
            )
            await rpc(
                "_echo_agent/session/close", {"session": other_session["session"]}
            )
            await other_agent.close()
            await agent.close()
            await registration.close()
            (work / "observed.json").write_text(
                json.dumps(facts, indent=2), encoding="utf-8"
            )
            print("idempotency evidence:", work, await helper("--verify"))
        finally:
            if held is not None:
                held.set()
            for session in sessions:
                try:
                    await sdk.request("echo_agent/session/close", {"session": session})
                except EchoAgentError:
                    pass
            await registration.close()
