"""Native Host evidence for accepted memory and permission authorities."""

import asyncio
import copy
import hashlib
import json
import os
import sys
import urllib.request
from pathlib import Path

import pytest

from echo_agent_sdk import EchoAgentClient, EchoAgentError, from_wire, to_wire
from echo_agent_sdk.client import StoreCall

TESTING = Path(__file__).resolve().parents[2] / "shared" / "testing"
HOST = os.environ.get("ECHO_AGENT_SDK_HOST", "").strip() or None
if os.environ.get("ECHO_AGENT_SDK_REQUIRE_HOST_E2E") == "1" and not HOST:
    raise RuntimeError("required memory/permission Host binary is missing")
pytestmark = [
    pytest.mark.asyncio,
    pytest.mark.skipif(not HOST, reason="real Host required"),
]
SOURCE = "echo_agent::agent::react::ReactAgent::set_store"
NEW = "echo_state::memory::store::InMemoryStore::new"
PUTRAW = "echo_state::memory::store::InMemoryStore::put_raw"


def http(url, path, value=None):
    request = urllib.request.Request(
        url + path,
        data=None if value is None else json.dumps(value).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(request, timeout=5) as response:
        return json.load(response)


ITEM_TIMESTAMPS = ("created_at", "updated_at", "last_accessed", "expires_at")


def decode_store_input(operation, request, from_wire):
    value = copy.deepcopy(request)
    if operation == "store_put":
        value["value"] = from_wire(value["value"])
    return value


def encode_store_outcome(outcome, to_wire):
    if outcome.get("outcome") != "result":
        return copy.deepcopy(outcome)
    result = copy.deepcopy(outcome["result"])
    operation = result["operation"]
    value = result["value"]

    def item(raw):
        encoded = copy.deepcopy(raw)
        encoded["value"] = to_wire(encoded["value"])
        for name in ITEM_TIMESTAMPS:
            if encoded[name] is not None:
                encoded[name] = str(encoded[name])
        return encoded

    if operation == "store_get" and value is not None:
        value = item(value)
    elif operation in {"store_search", "store_search_with", "store_list"}:
        value = [item(raw) for raw in value]
    elif operation in {"store_prune_expired", "store_dedup_by_content"}:
        value = str(value)
    return {"outcome": "result", "result": {"operation": operation, "value": value}}


async def test_memory_permission_live_authorities_and_effects(tmp_path):
    work = tmp_path
    model = await asyncio.create_subprocess_exec(
        sys.executable,
        str(TESTING / "memory_permission_fixture.py"),
        "--work",
        str(work),
    )
    results = []
    try:

        async def wait_for_condition_1():
            while not (work / "ready").exists():
                await asyncio.sleep(0.01)

        await asyncio.wait_for(wait_for_condition_1(), 5)
        url = json.loads((work / "backend.json").read_text())["url"]
        async with await EchoAgentClient.spawn(
            HOST, "--config", str(work / "host.json")
        ) as sdk:
            human_calls = []

            async def human(call, cancel):
                human_calls.append(copy.deepcopy(call))
                args = call["invocation"]["input"].get("args")
                parameters = from_wire(args) if args is not None else {}
                if parameters.get("phase") in ("deny", "allow"):
                    answer = (
                        {"variant": "rejected", "reason": "fixture denies real write"}
                        if parameters["phase"] == "deny"
                        else {"variant": "approved_with_scope", "scope": "once"}
                    )
                    return {
                        "outcome": "result",
                        "result": {"operation": "human_loop_request", "value": answer},
                    }
                return {
                    "outcome": "result",
                    "result": {
                        "operation": "human_loop_request",
                        "value": {
                            "variant": "modified_args",
                            "args": to_wire(
                                {"path": str(work / "modified.txt"), "中文": "🌊"}
                            ),
                            "scope": "once",
                        },
                    },
                }

            humanreg = await sdk.register_human_loop_provider(
                "fixture-human",
                {"kind": "human_loop_provider", "descriptor_version": 1},
                human,
            )
            agent = await sdk.create_agent()
            a, b = (
                await agent.create_session(str(work)),
                await agent.create_session(str(work)),
            )

            async def invoke(session, op, args=()):
                value = await session.invoke(op, args)
                results.append(
                    {"session": session.wire.id, "operation": op, "value": value}
                )
                return value

            store = await sdk.call(NEW, agent.wire, [a.wire])
            ns = ["sdk", "中文"]
            for key in ("one", "two"):
                await invoke(
                    a,
                    "memory.resource.put",
                    [store, ns, key, {"content": "duplicate-needle中文🌊"}],
                )
            await sdk.call(
                PUTRAW,
                agent.wire,
                [
                    a.wire,
                    store,
                    {
                        "namespace": ns,
                        "key": "expired",
                        "value": {"content": "expired"},
                        "created_at": 1,
                        "updated_at": 1,
                        "score": None,
                        "importance": 5.0,
                        "last_accessed": None,
                        "expires_at": 2,
                    },
                ],
            )
            assert (
                int(
                    (await invoke(a, "memory.resource.get", [store, ns, "expired"]))[
                        "expires_at"
                    ]
                )
                == 2
            )
            assert (
                len(
                    (
                        await invoke(
                            a,
                            "memory.resource.search_with",
                            [
                                store,
                                ns,
                                {
                                    "text": "duplicate-needle",
                                    "limit": 5,
                                    "mode": "Keyword",
                                },
                            ],
                        )
                    )["items"]
                )
                == 2
            )
            assert (
                int(await invoke(a, "memory.resource.prune_expired", [store, ns])) == 1
            )
            assert (
                int(await invoke(a, "memory.resource.prune_expired", [store, ns])) == 0
            )
            assert (
                await invoke(a, "memory.resource.get", [store, ns, "expired"]) is None
            )
            assert (
                int(await invoke(a, "memory.resource.dedup_by_content", [store, ns]))
                == 1
            )
            assert (
                int(await invoke(a, "memory.resource.dedup_by_content", [store, ns]))
                == 0
            )
            survivors = (await invoke(a, "memory.resource.list", [store, ns]))["items"]
            assert len(survivors) == 1 and survivors[0]["key"] in {"one", "two"}
            assert survivors[0]["value"] == {"content": "duplicate-needle中文🌊"}
            assert (
                await invoke(a, "memory.resource.list_namespaces", [store, ["sdk"]])
            )["namespaces"] == [ns]
            try:
                await invoke(b, "memory.resource.get", [store, ns, "one"])
            except EchoAgentError as error:
                results.append(
                    {
                        "negative": "foreign-store-owner",
                        "code": error.code,
                        "operation": error.operation,
                        "handle": error.handle.to_dict() if error.handle else None,
                        "details": error.details,
                        "retryable": error.retryable,
                    }
                )
            else:
                raise AssertionError("foreign Store ownership accepted")

            async def register(owner):
                async def callback(call, cancel):
                    if isinstance(call, dict):
                        call = StoreCall.from_payload(call)
                    op = call.operation
                    request = decode_store_input(op, call.request, from_wire)
                    outcome = await asyncio.to_thread(
                        http,
                        url,
                        "/store",
                        {"authority": owner, "operation": op, "input": request},
                    )
                    return encode_store_outcome(outcome, to_wire)

                return await sdk.register_store(
                    owner,
                    {
                        "kind": "store",
                        "descriptor_version": 1,
                        "search_modes": [] if owner == "A" else ["keyword"],
                    },
                    callback,
                )

            ra = await register("A")
            await sdk.call(SOURCE, agent.wire, [a.wire, ra.wire])
            raw = {"中文": "🌊", "__proto__": {"id": "domain", "generation": "literal"}}
            await invoke(a, "memory.store.put", [ns, "key", raw])
            assert (await invoke(a, "memory.store.get", [ns, "key"]))["value"] == raw
            await ra.close()
            rb = await register("B")
            await sdk.call(SOURCE, agent.wire, [a.wire, rb.wire])
            assert await invoke(a, "memory.store.get", [ns, "key"]) is None
            await invoke(a, "memory.store.put", [ns, "key", "B-new"])
            assert (
                len((await invoke(a, "memory.store.search", [ns, "B-new", 5]))["items"])
                == 1
            )
            assert (await invoke(a, "memory.store.delete", [ns, "key"]))["deleted"]
            assert not (await invoke(a, "memory.store.delete", [ns, "key"]))["deleted"]
            await asyncio.to_thread(http, url, "/fault", {"operation": "store_put"})
            try:
                await invoke(a, "memory.store.put", [ns, "fault", "never"])
            except EchoAgentError as error:
                results.append(
                    {
                        "negative": "reverse-store-prewrite-fault",
                        "code": error.code,
                        "operation": error.operation,
                        "handle": error.handle.to_dict() if error.handle else None,
                        "details": error.details,
                        "retryable": error.retryable,
                    }
                )
            else:
                raise AssertionError("reverse callback failure swallowed")
            assert await invoke(a, "memory.store.get", [ns, "fault"]) is None
            await invoke(a, "memory.store.put", [ns, "fault", "retrycommitted"])
            assert (await invoke(a, "memory.store.get", [ns, "fault"]))[
                "value"
            ] == "retrycommitted"
            # Fresh default Store Session: raw set_store is intentionally distinct.
            await rb.close()
            c = await agent.create_session(str(work))
            corpus = json.loads((work / "scenarios.json").read_text())["scenarios"][2][
                "steps"
            ]
            for step in corpus[:3]:
                await invoke(c, step["operation"], step["args"])
            original = await invoke(
                c, "memory.store.get", [["agent", "memories"], "approved-active"]
            )
            run = await c.start_run("SDK-Recall")
            settlement = await run.wait(timeout=10)
            assert settlement["settled"]
            inspection = await asyncio.to_thread(http, url, "/inspect")
            (work / "inspection-before-recall-assertion.json").write_text(
                json.dumps(inspection, ensure_ascii=False, indent=2)
            )
            serialized = json.dumps(
                inspection["model_requests"][-1], ensure_ascii=False
            )
            assert "SDK-Recall-Active 中文🌊" in serialized
            assert (
                "SDK-Recall-Draft" not in serialized
                and "SDK-Recall-Legacy" not in serialized
            )
            latest = await invoke(
                c, "memory.store.get", [["agent", "memories"], "approved-active"]
            )
            assert latest["value"]["content"] == original["value"]["content"]
            assert (
                latest["value"]["meta"]["provenance"]
                == original["value"]["meta"]["provenance"]
            )
            journal = await run.replay("0", 512)
            memory_events = [
                event
                for event in journal["events"]
                if event["payload"]["event_type"] == "memory_recalled"
            ]
            assert len(memory_events) == 1 and from_wire(
                memory_events[0]["payload"]["data"]
            ) == {"count": "1"}
            assert [int(event["sequence"]) for event in journal["events"]] == list(
                range(1, len(journal["events"]) + 1)
            )
            assert len({event["event_id"] for event in journal["events"]}) == len(
                journal["events"]
            )
            assert (
                sum(
                    event["payload"]["event_type"] == "final_answer"
                    for event in journal["events"]
                )
                == 1
            )
            results.append(
                {
                    "actual_model_recall": True,
                    "model_request": inspection["model_requests"][-1],
                    "settled": settlement,
                    "journal": journal,
                }
            )
            rule = json.loads((work / "scenarios.json").read_text())["scenarios"][3][
                "steps"
            ][2]["args"][0]
            await invoke(a, "permission.clear_rules")
            await invoke(a, "permission.clear_cache")
            await invoke(a, "permission.add_rule", [rule])
            assert await invoke(a, "permission.all_rules") == [rule]
            assert await invoke(b, "permission.all_rules") == []
            context = json.loads((work / "scenarios.json").read_text())["scenarios"][3][
                "steps"
            ][4]["args"][-1]
            context["session_id"] = a.acp_session_id
            context["permission"]["working_directory"] = str(work)
            context["permission"]["affected_files"] = [str(work / "effect.txt")]
            check = await invoke(
                a,
                "permission.check_with_permissions_result_in_mode_and_context",
                [
                    "candidate_write",
                    {"path": str(work / "effect.txt")},
                    ["write"],
                    "default",
                    context,
                ],
            )
            assert check["decision"] == "deny"
            assert await invoke(a, "permission.remove_rule", [rule]) is True
            assert await invoke(a, "permission.remove_rule", [rule]) is False
            scope = "same-scope"
            await invoke(
                a,
                "permission.record_approval",
                [scope, "candidate_write", {"x": 1}, "session"],
            )
            assert (
                await invoke(
                    a, "permission.is_approved", [scope, "candidate_write", {"x": 1}]
                )
                is True
            )
            assert (
                await invoke(
                    a, "permission.is_approved", [scope, "candidate_write", {"x": 2}]
                )
                is False
            )
            assert (
                await invoke(
                    b, "permission.is_approved", [scope, "candidate_write", {"x": 1}]
                )
                is False
            )
            await invoke(
                b,
                "permission.record_approval",
                [scope, "candidate_write", {"x": 1}, "session"],
            )
            await invoke(a, "permission.revoke_cache", [scope, "candidate_write"])
            assert (
                await invoke(
                    a, "permission.is_approved", [scope, "candidate_write", {"x": 1}]
                )
                is False
            )
            assert (
                await invoke(
                    b, "permission.is_approved", [scope, "candidate_write", {"x": 1}]
                )
                is True
            )
            await invoke(
                a,
                "permission.record_approval",
                [scope, "candidate_write", {"x": 1}, "once"],
            )
            assert (
                await invoke(
                    a, "permission.is_approved", [scope, "candidate_write", {"x": 1}]
                )
                is False
            )
            await invoke(
                a,
                "permission.record_approval",
                [scope, "candidate_write", {"x": 1}, "session_tool"],
            )
            assert (
                await invoke(
                    a, "permission.is_approved", [scope, "candidate_write", {"x": 2}]
                )
                is True
            )
            await invoke(a, "permission.revoke_cache", [scope, "candidate_write"])
            stats = await invoke(a, "permission.stats")
            results.append(
                {"actual_ttl_ms": stats["ttl_ms"], "ttl_expiration_proof": False}
            )
            assert int(await invoke(a, "permission.cleanup_expired")) == 0
            await invoke(a, "permission.clear_cache")
            contextcheck = await invoke(
                a,
                "permission.check_with_permissions_result_in_mode_and_context",
                [
                    "candidate_write",
                    {"path": str(work / "context.txt")},
                    ["write"],
                    "strict",
                    context,
                ],
            )
            assert contextcheck["decision"] == "allow" and contextcheck[
                "updated_input"
            ] == {"path": str(work / "modified.txt"), "中文": "🌊"}
            assert (
                human_calls
                and human_calls[-1]["invocation"]["input"]["session_id"]
                == a.acp_session_id
            )
            human_request = human_calls[-1]["invocation"]["input"]
            assert human_request["request_id"] == context["request_id"]
            assert human_request["agent_name"] == "memory-fixture"
            assert human_request["timeout"] == {"seconds": "1", "nanos": 0}
            assert from_wire(human_request["args"]) == {
                "path": str(work / "context.txt")
            }
            assert from_wire(human_request["approval_context"]) == context["permission"]
            results.append(
                {
                    "permission_reverse_context": human_calls[-1],
                    "returned_check": contextcheck,
                    "approval_receipt_present": "approval_receipt" in contextcheck,
                }
            )
            await a.close()
            assert (
                await invoke(
                    b, "permission.is_approved", [scope, "candidate_write", {"x": 1}]
                )
                is True
            )
            tool_calls = []

            async def write_tool(call, cancel):
                op = call["invocation"]["operation"]
                if op == "tool_validate_parameters":
                    return {
                        "outcome": "result",
                        "result": {"operation": op, "value": None},
                    }
                parameters = from_wire(call["invocation"]["input"]["parameters"])
                target = Path(parameters["path"])
                assert target.parent == work
                target.write_text("accepted permission effect 中文🌊")
                tool_calls.append({"path": str(target), "phase": parameters["phase"]})
                return {
                    "outcome": "result",
                    "result": {
                        "operation": op,
                        "value": {
                            "kind": {"kind": "text"},
                            "success": True,
                            "output": "written",
                            "truncated": False,
                            "effects": [{"effect": "file_edit", "path": str(target)}],
                        },
                    },
                }

            tool = await sdk.register_extension(
                "tool",
                "candidate-write",
                {
                    "kind": "tool",
                    "descriptor_version": 1,
                    "name": "candidate_write",
                    "description": "write one test fixture file",
                    "parameters": to_wire(
                        {
                            "type": "object",
                            "properties": {
                                "path": {"type": "string"},
                                "phase": {"type": "string"},
                            },
                            "required": ["path", "phase"],
                        }
                    ),
                    "schema_revision": "1",
                    "required_input_modalities": [],
                    "required_permissions": ["write"],
                    "risk_level": "standard",
                    "supports_streaming": False,
                    "exempt_from_batch_timeout": False,
                    "allows_parallel_batch_execution": False,
                    "manages_own_timeout": False,
                },
                write_tool,
            )
            denied = await agent.create_session(str(work))
            allowed = await agent.create_session(str(work))
            denial_run = await denied.start_run("permission-effect-deny")
            denial_settled = await denial_run.wait(timeout=10)
            assert (
                denial_settled["settled"]
                and not (work / "effect-deny.txt").exists()
                and not tool_calls
            )
            approval_run = await allowed.start_run("permission-effect-allow")
            approval_settled = await approval_run.wait(timeout=10)
            assert (
                approval_settled["settled"]
                and (work / "effect-allow.txt").read_text()
                == "accepted permission effect 中文🌊"
            )
            assert len(tool_calls) == 1 and tool_calls[0]["phase"] == "allow"
            results.append(
                {
                    "actual_permission_effect": {
                        "denied_file_exists": False,
                        "denied_tool_calls": 0,
                        "approved_tool_calls": tool_calls,
                        "denial_settlement": denial_settled,
                        "approval_settlement": approval_settled,
                    },
                    "human_requests": human_calls[-2:],
                }
            )
            backend_inspection = await asyncio.to_thread(http, url, "/inspect")
            results.append({"backend_inspection": backend_inspection})
            await denied.close()
            await allowed.close()
            await tool.close()
            await b.close()
            await c.close()
            await agent.close()
            await ra.close()
            await rb.close()
            await humanreg.close()
        (work / "python-real-evidence.json").write_text(
            json.dumps(
                {
                    "host": HOST,
                    "host_sha256": hashlib.sha256(Path(HOST).read_bytes()).hexdigest(),
                    "work": str(work),
                    "results": results,
                    "limitations": [
                        "TTL expiration not observed",
                        "approval receipt omitted by existing adapter",
                    ],
                },
                ensure_ascii=False,
                indent=2,
            )
            + "\n"
        )
        print("PY actual memory/permission evidence rows", len(results), "work", work)
    finally:
        if model.returncode is None:
            model.terminate()
        await model.wait()
