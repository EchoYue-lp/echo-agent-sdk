import pytest

from echo_agent_sdk import AgentComponentDescriptor, EchoAgentError
from echo_agent_sdk.client import AgentComponentCall
from echo_agent_sdk.managed import (
    ConversationApplyTranscriptProjectionWithContextResult,
    ConversationProjectionAuthority,
    ConversationProjectionLifecycle,
    PersistenceCallContext,
    RuntimeStateExpectedVersionManaged,
    TranscriptProjectionApplyReceipt,
    TranscriptProjectionApplyStatusApplied,
    decode,
)


def test_typed_managed_receipt_keeps_large_revisions_and_status():
    authority = ConversationProjectionAuthority(
        "中文",
        "18446744073709551615",
        "9007199254740993",
        ConversationProjectionLifecycle.LIVE,
        "0",
    )
    receipt = TranscriptProjectionApplyReceipt(
        "operation", "digest", authority, TranscriptProjectionApplyStatusApplied()
    )
    result = ConversationApplyTranscriptProjectionWithContextResult(receipt)
    from echo_agent_sdk.client import AgentComponentOutcome

    wire = result.to_wire()
    assert AgentComponentOutcome(result).to_wire()["result"]["value"]["result"] == wire
    assert wire["value"]["receipt"]["authority"]["revision"] == "9007199254740993"
    assert wire["value"]["receipt"]["status"] == "applied"
    assert (
        TranscriptProjectionApplyReceipt.from_wire(wire["value"]["receipt"]) == receipt
    )
    assert RuntimeStateExpectedVersionManaged("9007199254740993").to_wire() == {
        "managed": {"revision": "9007199254740993"}
    }


def test_component_descriptor_accepts_only_appropriate_managed_capabilities():
    descriptor = AgentComponentDescriptor(
        "conversation_store",
        "store",
        conversation_projection="atomic_v1",
        persistence_call="absolute_deadline_v1",
    )
    assert (
        descriptor.to_wire()["capabilities"]["conversation_projection"] == "atomic_v1"
    )
    with pytest.raises(ValueError):
        AgentComponentDescriptor("guard", "wrong", conversation_projection="atomic_v1")


def test_managed_context_is_typed_during_reverse_dispatch():
    call = AgentComponentCall.from_payload(
        {
            "extension": {"id": "ext", "generation": "1", "kind": "extension"},
            "invocation_id": "invoke",
            "deadline": {"seconds": "1", "nanos": 0},
            "invocation": {
                "operation": "agent_component_call",
                "input": {
                    "component": "runtime_state_store",
                    "call": {
                        "operation": "runtime_load_scope_authority_with_context",
                        "input": {
                            "scope_id": "scope",
                            "context": {
                                "absolute_deadline_unix_ms": "9223372036854775807"
                            },
                        },
                    },
                },
            },
        }
    )
    assert call.request.context == PersistenceCallContext("9223372036854775807")
    with pytest.raises(EchoAgentError):
        decode("PersistenceCallContext", {"absolute_deadline_unix_ms": "01"})


def test_native_effects_and_interrupted_callback_identity():
    from echo_agent_sdk import AgentCallbackCall, ToolEffect, ToolResult

    result = ToolResult.success_result("ok").with_effects(
        [
            ToolEffect.file_read("/中文🙂"),
            ToolEffect.test_run("测试", False, (1 << 64) - 1),
            ToolEffect.subagent_run("a", "task", "cancelled"),
        ]
    )
    assert result.to_wire()["effects"][1]["failure_count"] == "18446744073709551615"
    raw = {
        "extension": {"id": "e", "generation": "1", "kind": "extension"},
        "invocation_id": "transport",
        "invocation": {
            "operation": "callback_on_tool_interrupted",
            "input": {
                "agent": "a",
                "call_id": "canonical",
                "tool": "t",
                "input": {"kind": "null"},
                "error": "cancelled",
            },
        },
    }
    call = AgentCallbackCall.from_payload(raw)
    assert call.call_id == "canonical"
    assert call.completed()["result"]["value"] is None
    with pytest.raises(EchoAgentError):
        AgentCallbackCall.from_payload(
            {
                **raw,
                "invocation": {
                    **raw["invocation"],
                    "input": {**raw["invocation"]["input"], "call_id": None},
                },
            }
        )


def test_schema_five_checkpoint_facts_remain_exact_history_bytes():
    from echo_agent_sdk.managed import AgentCheckpoint

    history = '{"schema_version":5,"execution_facts":[{"call_id":"run/tool/1","effects":[{"effect":"file_edit","path":"中文🙂"}]}]}'
    checkpoint = AgentCheckpoint(
        "conversation",
        history,
        None,
        (),
        None,
        {"encoding": "unix", "bytes_base64": "L3RtcC//"},
        {"unix_seconds": "-1", "nanos": 999999999},
    )
    encoded = checkpoint.to_wire()
    assert AgentCheckpoint.from_wire(encoded).messages_json == history
    assert encoded["working_dir"]["encoding"] == "unix"
    assert encoded["timestamp"]["nanos"] == 999999999


def test_task_execution_cas_preserves_absent_vs_explicit_null_result():
    from echo_agent_sdk.client import (
        AgentComponentCall,
        AgentComponentOutcome,
        RevisionedTaskStoreResult,
    )
    from echo_agent_sdk.managed import (
        RevisionedTaskGraph,
        RuntimePlanSnapshot,
        TaskExecution,
        TaskExecutionCommit,
        TaskGraphContext,
        TaskGraphExecutionMode,
        WireTaskStatusRunning,
    )

    absent = TaskExecution("task", WireTaskStatusRunning(), 0, None, None, None, None)
    supplied = TaskExecution(
        "task", WireTaskStatusRunning(), 0, None, None, {"kind": "null"}, None
    )
    assert "result" not in absent.to_wire()
    assert supplied.to_wire()["result"] == {"kind": "null"}
    graph = RevisionedTaskGraph(
        RuntimePlanSnapshot("9007199254740993", ()),
        TaskGraphContext(
            "goal", (), (), TaskGraphExecutionMode.PARALLEL, {"kind": "null"}
        ),
    )
    commit = TaskExecutionCommit("9007199254740993", {"task": absent}, graph)
    raw = {
        "extension": {"id": "e", "generation": "1", "kind": "extension"},
        "invocation_id": "i",
        "deadline": {"seconds": "1", "nanos": 0},
        "invocation": {
            "operation": "agent_component_call",
            "input": {
                "component": "revisioned_task_store",
                "call": {
                    "operation": "revisioned_task_compare_and_commit_execution",
                    "input": {"scope_id": "scope", "commit": commit.to_wire()},
                },
            },
        },
    }
    call = AgentComponentCall.from_payload(raw)
    assert call.request.commit.expected_revision == "9007199254740993"
    result = AgentComponentOutcome(
        RevisionedTaskStoreResult.execution_committed(graph)
    ).to_wire()
    assert (
        result["result"]["value"]["result"]["value"]["graph"]["snapshot"]["revision"]
        == "9007199254740993"
    )


def test_all_eleven_managed_context_operations_preserve_shared_typed_corpus():
    import json
    from pathlib import Path

    from echo_agent_sdk import managed
    from echo_agent_sdk.client import AgentComponentCall, AgentComponentOutcome

    corpus = json.loads(
        (
            Path(__file__).resolve().parents[2]
            / "shared/fixtures/managed-context-corpus.json"
        ).read_text()
    )
    assert len(corpus) == 11
    for item in corpus:
        payload = {
            "extension": {"id": "e", "generation": "1", "kind": "extension"},
            "invocation_id": item["scenario_id"],
            "deadline": {"seconds": "1", "nanos": 0},
            "invocation": {
                "operation": "agent_component_call",
                "input": {
                    "component": item["component"],
                    "call": {"operation": item["operation"], "input": item["input"]},
                },
            },
        }
        call = AgentComponentCall.from_payload(payload)
        assert call.request.to_wire() == item["input"]
        _, field, kind = managed.MANAGED_OPERATIONS[item["operation"]]
        typed = managed.decode(kind, item["result"][field])
        name = "".join(part.title() for part in item["operation"].split("_")) + "Result"
        outcome = AgentComponentOutcome(getattr(managed, name)(typed)).to_wire()
        assert outcome["result"]["value"]["result"]["value"] == item["result"]


def test_managed_checkpoints_reject_unknown_and_relative_path_encodings():
    from echo_agent_sdk.managed import decode

    with pytest.raises(EchoAgentError):
        decode("WirePath", {"encoding": "unix_bytes", "base64": "L3RtcC//"})
    with pytest.raises(EchoAgentError):
        decode("WirePath", {"encoding": "unix", "bytes_base64": "cmVsYXRpdmU"})
    with pytest.raises(EchoAgentError):
        decode("WirePath", {"encoding": "utf8", "path": "relative"})
    assert (
        decode("WirePath", {"encoding": "unix", "bytes_base64": "L3RtcC//"})[
            "bytes_base64"
        ]
        == "L3RtcC//"
    )


def test_execution_cas_is_explicit_boolean_and_component_bound():
    assert (
        AgentComponentDescriptor("revisioned_task_store", "crud").to_wire()[
            "capabilities"
        ]["execution_cas_v1"]
        is False
    )
    assert (
        AgentComponentDescriptor(
            "revisioned_task_store", "cas", execution_cas_v1=True
        ).to_wire()["capabilities"]["execution_cas_v1"]
        is True
    )
    for invalid in ("true", 1, None):
        with pytest.raises(ValueError):
            AgentComponentDescriptor(
                "revisioned_task_store", "cas", execution_cas_v1=invalid
            )
    with pytest.raises(ValueError):
        AgentComponentDescriptor("guard", "cas", execution_cas_v1=True)


def test_typed_settlement_event_checks_u32_without_relaxing_wire_integer_policy():
    from echo_agent_sdk import from_wire, managed, to_wire

    payload = {
        "status": "deferred",
        "operation_id": "commit",
        "conversation_id": "session",
        "generation_id": "generation",
        "attempt": (1 << 32) - 1,
        "error_class": "outcome_unknown",
        "detail": "lost 中文🙂",
    }
    wire = to_wire(payload)
    assert from_wire(wire)["attempt"] == str((1 << 32) - 1)
    assert managed.decode_settlement_event(wire).to_wire() == payload
    import copy

    wrong = copy.deepcopy(wire)
    next(entry for entry in wrong["value"] if entry["key"]["value"] == "attempt")[
        "value"
    ]["value"] = str(1 << 32)
    with pytest.raises(EchoAgentError):
        managed.decode_settlement_event(wrong)
    duplicate = copy.deepcopy(wire)
    duplicate["value"].append(duplicate["value"][0])
    with pytest.raises(EchoAgentError):
        managed.decode_settlement_event(duplicate)
    missing = copy.deepcopy(wire)
    missing["value"].pop()
    with pytest.raises(EchoAgentError):
        managed.decode_settlement_event(missing)
