"""Prepared candidate only. Run explicitly after coordinator adoption/review.

Uses public SDK operations against a real immutable Host. This code neither
models CAS nor constructs a second store/ledger. Faults only alter isolated
fixture directories with guarded restore. No operation has been executed yet.
"""
import argparse
import asyncio
import hashlib
import json
from pathlib import Path

from echo_agent_sdk import EchoAgentClient, EchoAgentError, wire_u64
from prepare_fixture import block, unblock


def canonical(value):
    # Native projection intentionally returns wire integers as decimal text.
    if isinstance(value, bool) or value is None:
        return value
    if isinstance(value, int):
        return str(value)
    if isinstance(value, list):
        return [canonical(item) for item in value]
    if isinstance(value, dict):
        return {key: canonical(item) for key, item in value.items()}
    return value


async def start(host, work):
    client = await EchoAgentClient.spawn(host, "--config", str(work / "host.json"))
    agent = await client.create_agent()
    session = await agent.create_session()
    return client, agent, session


async def must_error(call, operation):
    try:
        await call
    except EchoAgentError as error:
        assert error.code == "framework_error", (error.code, str(error))
        assert error.operation in (operation, "_echo_agent/state/op", "_echo_agent/trace/op", "_echo_agent/delivery/op")
        assert str(error)
        if operation.startswith("delivery."):
            assert "stale delivery claim" in str(error)
        return {"code": error.code, "operation": error.operation, "message": str(error)}
    raise AssertionError("expected failure, got successful result: " + operation)


async def state_scene(host, work, fixture):
    client, agent, session = await start(host, work)
    proof = {"errors": [], "checkpoints": {}}
    try:
        for label in ("a", "b"):
            scope, checkpoint = fixture["scope_" + label], fixture["checkpoint_" + label]
            assert await session.invoke("state.checkpoint.save", [scope, checkpoint]) == {"ok": True}
            read = await session.invoke("state.checkpoint.get", [checkpoint["conversation_id"]])
            assert canonical(read["checkpoint"]) == canonical(checkpoint)
            ids = await session.invoke("state.runtime.list", [scope])
            assert ids["runtime_state_ids"] == [checkpoint["conversation_id"]]
            proof["checkpoints"][label] = read["checkpoint"]
        owners = work / "state/runtime_state/_runtime_owners"
        original_files = {p.name: p.read_bytes() for p in owners.glob("*.json")}
        # Existing authority directory is blocked after successful Host bootstrap.
        block(work, "state/runtime_state/_runtime_owners")
        try:
            proof["errors"].append(await must_error(session.invoke("state.checkpoint.save",
                [fixture["scope_a"], fixture["checkpoint_a"]]), "state.checkpoint.save"))
        finally:
            unblock(work)
        assert {p.name: p.read_bytes() for p in owners.glob("*.json")} == original_files
        assert await session.invoke("state.checkpoint.save", [fixture["scope_a"], fixture["checkpoint_a"]]) == {"ok": True}
        # Identify owner by actual DTO, without duplicating path encoding.
        owner_a = next(p for p in owners.glob("*.json") if json.loads(p.read_text())["runtime_state_id"] == fixture["checkpoint_a"]["conversation_id"])
        backup = owner_a.read_bytes()
        owner_a.write_text("{malformed-authoritative-owner")
        try:
            proof["errors"].append(await must_error(session.invoke("state.checkpoint.get",
                [fixture["checkpoint_a"]["conversation_id"]]), "state.checkpoint.get"))
        finally:
            owner_a.write_bytes(backup)
        receipt = await session.invoke("state.scope.clear", [fixture["scope_a"]])
        assert receipt["scope_id"] == fixture["scope_a"]
        assert receipt["runtime_state_ids"] == [fixture["checkpoint_a"]["conversation_id"]]
        assert (await session.invoke("state.checkpoint.get", [fixture["checkpoint_a"]["conversation_id"]]))["checkpoint"] is None
        assert canonical((await session.invoke("state.checkpoint.get", [fixture["checkpoint_b"]["conversation_id"]]))["checkpoint"]) == canonical(fixture["checkpoint_b"])
        # Rebuildable index deletion affects projection only, never owners.
        index_dir = work / "state/runtime_state/_scope_index"
        for p in index_dir.glob("*.json"):
            data = json.loads(p.read_text())
            if data.get("scope_id") == fixture["scope_b"]:
                p.unlink()
        assert (await session.invoke("state.runtime.list", [fixture["scope_b"]]))["runtime_state_ids"] == [fixture["checkpoint_b"]["conversation_id"]]
    finally:
        await client.close()  # actual native Host ownership completes here
    client, agent, session = await start(host, work)
    try:
        recovered = (await session.invoke("state.checkpoint.get", [fixture["checkpoint_b"]["conversation_id"]]))["checkpoint"]
        assert canonical(recovered) == canonical(fixture["checkpoint_b"])
        proof["after_real_host_restart"] = recovered
    finally:
        await client.close()
    return proof


async def trace_scene(host, work, fixture):
    client, agent, session = await start(host, work)
    proof = {"errors": []}
    try:
        # Failure before publication must not consume quota=1.
        blocked = work / "trace-open-parent"
        blocked.write_text("regular-file-block")
        proof["errors"].append(await must_error(session.invoke("trace.store.open", ["jsonl", str(blocked / "child")]), "trace.store.open"))
        store = (await session.invoke("trace.store.open", ["jsonl", fixture["trace_dir"]]))["resource"]
        assert await session.invoke("trace.run.save", [store, fixture["trace"]]) == {"ok": True}
        loaded = (await session.invoke("trace.run.load", [store, fixture["trace"]["run_id"]]))["run"]
        assert canonical(loaded) == canonical(fixture["trace_expected"])
        run_file = next(p for p in Path(fixture["trace_dir"]).glob("*.jsonl") if json.loads(p.read_text().splitlines()[0])["run_id"] == fixture["trace"]["run_id"])
        durable_before = run_file.read_bytes()
        assert "sk-abcdefghijklmnopqrstuvwxyz123456" not in durable_before.decode()
        assert canonical(json.loads(durable_before.decode().splitlines()[0])) == canonical(fixture["trace_expected"])
        block(work, "traces")
        try:
            proof["errors"].append(await must_error(session.invoke("trace.run.save", [store, fixture["trace"]]), "trace.run.save"))
            assert canonical((await session.invoke("trace.run.load", [store, fixture["trace"]["run_id"]]))["run"]) == canonical(loaded)
        finally:
            unblock(work)
        assert run_file.read_bytes() == durable_before
        assert await session.invoke("trace.run.save", [store, fixture["trace"]]) == {"ok": True}
        summaries = (await session.invoke("trace.run.list_session", [store, fixture["trace"]["session_id"]]))["summaries"]
        assert len(summaries) == 1
        for key in ("run_id", "parent_run_id", "session_id", "agent_name", "model", "provider", "turn_id", "execution_id", "status", "started_at", "finished_at", "token_usage"):
            assert canonical(summaries[0][key]) == canonical(fixture["trace_expected"][key])
        assert canonical(summaries[0]["total_duration_ms"]) == "9007199254740993"
        proof["loaded"] = loaded
        proof["durable_sha256"] = hashlib.sha256(run_file.read_bytes()).hexdigest()
    finally:
        await client.close()
    client, agent, session = await start(host, work)
    try:
        store = (await session.invoke("trace.store.open", ["jsonl", fixture["trace_dir"]]))["resource"]
        reopened = (await session.invoke("trace.run.load", [store, fixture["trace"]["run_id"]]))["run"]
        assert canonical(reopened) == canonical(fixture["trace_expected"])
        proof["after_real_host_restart"] = reopened
    finally:
        await client.close()
    return proof


def claim_input(claim):
    # Documented lossless native projections return integers as decimal text.
    # Preserve every other returned claim field; explicitly encode its accepted
    # u32 attempt as wire u64 before the Host's typed DTO parses it. This is a
    # public SDK value constructor, not a permissive Host string/u32 parser.
    assert set(claim) == {"message_id", "route", "payload", "attempt_id", "attempt", "claimed_at"}
    assert str(int(claim["attempt"])) == claim["attempt"]
    assert 0 <= int(claim["attempt"]) <= (1 << 32) - 1
    return dict(claim, attempt=wire_u64(claim["attempt"]))


async def delivery_scene(host, work, fixture):
    client, agent, session = await start(host, work)
    proof = {"errors": [], "durability": "in-memory same-instance only"}
    try:
        ledger = (await session.invoke("delivery.ledger.open"))["resource"]
        message = fixture["delivery"]
        await session.invoke("delivery.enqueue", [ledger, message["message_id"], message["route"], message["payload"]])
        answers = await asyncio.gather(session.invoke("delivery.claim_next", [ledger]), session.invoke("delivery.claim_next", [ledger]))
        claims = sorted([x["claim"] for x in answers if x["claim"] is not None], key=lambda claim: int(claim["attempt"]))
        # Existing framework pre-effect claims may be superseded. Authority
        # uniqueness means only the newest exact claim can admit an effect.
        assert len(claims) == 2
        first, latest = claims
        (work / "delivery-concurrent-claims.json").write_text(json.dumps(answers, ensure_ascii=False, indent=2) + "\n")
        for claimed in claims:
            assert set(claimed) == {"message_id", "route", "payload", "attempt_id", "attempt", "claimed_at"}
            assert claimed["message_id"] == message["message_id"] and claimed["payload"] == message["payload"]
            assert claimed["route"] == message["route"] and claimed["attempt_id"] and claimed["claimed_at"]
        assert [str(claimed["attempt"]) for claimed in claims] == ["1", "2"]
        assert first["attempt_id"] != latest["attempt_id"]
        before_superseded = await session.invoke("delivery.snapshot", [ledger])
        assert before_superseded["records"][0]["attempt_id"] == latest["attempt_id"]
        before_sequence = (await session.invoke("delivery.recover", [ledger]))["last_applied_sequence"]
        proof["errors"].append(await must_error(session.invoke("delivery.transition",
            [ledger, claim_input(first), "effect_started", "turn-superseded"]), "delivery.transition"))
        assert await session.invoke("delivery.snapshot", [ledger]) == before_superseded
        assert (await session.invoke("delivery.recover", [ledger]))["last_applied_sequence"] == before_sequence
        await session.invoke("delivery.transition", [ledger, claim_input(latest), "effect_started", "turn-before-defer"])
        # Once the effect was started, neither strict FIFO claim_next call may
        # supersede it; this is the actual single in-flight admission boundary.
        after_effect = await asyncio.gather(session.invoke("delivery.claim_next", [ledger]), session.invoke("delivery.claim_next", [ledger]))
        assert all(answer["claim"] is None for answer in after_effect)
        await session.invoke("delivery.defer", [ledger, claim_input(latest), "延迟重试🙂", "2000-01-01T00:00:00Z"])
        second = (await session.invoke("delivery.claim_next", [ledger]))["claim"]
        assert second["attempt_id"] != latest["attempt_id"] and str(second["attempt"]) == "3"
        proof["concurrent_claims"] = claims
        proof["effect_started_no_reclaim"] = after_effect
        before = await session.invoke("delivery.snapshot", [ledger])
        sequence = (await session.invoke("delivery.recover", [ledger]))["last_applied_sequence"]
        for operation, args in [("delivery.transition", [ledger, claim_input(first), "effect_started", "turn-旧"]),
                                ("delivery.defer", [ledger, claim_input(first), "旧请求", "2000-01-01T00:00:00Z"]),
                                ("delivery.settle", [ledger, claim_input(first), "completed", None, "turn-旧"])]:
            proof["errors"].append(await must_error(session.invoke(operation, args), operation))
            assert await session.invoke("delivery.snapshot", [ledger]) == before
            assert (await session.invoke("delivery.recover", [ledger]))["last_applied_sequence"] == sequence
        await session.invoke("delivery.transition", [ledger, claim_input(second), "effect_started", "turn-新"])
        await session.invoke("delivery.settle", [ledger, claim_input(second), "completed", None, "turn-新"])
        snapshot = await session.invoke("delivery.snapshot", [ledger])
        assert len(snapshot["records"]) == 1
        record = snapshot["records"][0]
        assert record["outcome"] == "completed" and record["attempt_id"] == second["attempt_id"]
        recovered = await session.invoke("delivery.recover", [ledger])
        assert await session.invoke("delivery.snapshot", [ledger]) == snapshot
        assert await session.invoke("delivery.recover", [ledger]) == recovered
        proof.update({"claim1": first, "claim2": second, "snapshot": snapshot, "recovery": recovered})
    finally:
        await client.close()
    return proof


async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", required=True)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--scene", choices=["state", "trace", "delivery"], required=True)
    args = parser.parse_args()
    fixture = json.loads((args.work / "fixture.json").read_text())
    result = await {"state": state_scene, "trace": trace_scene, "delivery": delivery_scene}[args.scene](args.host, args.work, fixture)
    (args.work / (args.scene + "-observed.json")).write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")

if __name__ == "__main__":
    asyncio.run(main())
