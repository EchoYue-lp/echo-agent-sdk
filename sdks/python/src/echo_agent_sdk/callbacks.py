from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass
from typing import Any

from .errors import EchoAgentError
from .wire import WireHandle, wire_u64


@dataclass(frozen=True, slots=True)
class CallbackThinkStart:
    agent: str
    messages: tuple[Mapping[str, Any], ...]


@dataclass(frozen=True, slots=True)
class CallbackThinkEnd:
    agent: str
    steps: tuple[Mapping[str, Any], ...]
    prompt_tokens: str
    completion_tokens: str


@dataclass(frozen=True, slots=True)
class CallbackToolStart:
    agent: str
    tool: str
    args: Mapping[str, Any]
    call_id: str | None = None


@dataclass(frozen=True, slots=True)
class CallbackToolEnd:
    agent: str
    tool: str
    result: str
    call_id: str | None = None


@dataclass(frozen=True, slots=True)
class CallbackToolError:
    agent: str
    tool: str
    error: str
    call_id: str | None = None


@dataclass(frozen=True, slots=True)
class CallbackToolInterrupted:
    agent: str
    call_id: str
    tool: str
    input: Mapping[str, Any]
    error: str


@dataclass(frozen=True, slots=True)
class CallbackFinalAnswer:
    agent: str
    answer: str


@dataclass(frozen=True, slots=True)
class CallbackIteration:
    agent: str
    iteration: str


CallbackRequest = (
    CallbackThinkStart
    | CallbackThinkEnd
    | CallbackToolStart
    | CallbackToolEnd
    | CallbackToolError
    | CallbackToolInterrupted
    | CallbackFinalAnswer
    | CallbackIteration
)


@dataclass(frozen=True, slots=True)
class AgentCallbackCall:
    operation: str
    agent: str
    payload: Mapping[str, Any]
    extension: WireHandle
    invocation_id: str
    context: Mapping[str, Any] | None
    call_id: str | None = None
    request: CallbackRequest | None = None

    @classmethod
    def from_payload(cls, raw: Mapping[str, Any]):
        invocation = raw.get("invocation")
        if not isinstance(invocation, Mapping) or not isinstance(
            invocation.get("input"), Mapping
        ):
            raise EchoAgentError(
                "serialization_violation", "callback invocation malformed"
            )
        operation = invocation.get("operation")
        data = invocation["input"]
        fields = {
            "callback_on_think_start": {"agent", "messages"},
            "callback_on_think_end": {
                "agent",
                "steps",
                "prompt_tokens",
                "completion_tokens",
            },
            "callback_on_tool_start": {"agent", "call_id", "tool", "args"},
            "callback_on_tool_end": {"agent", "call_id", "tool", "result"},
            "callback_on_tool_error": {"agent", "call_id", "tool", "error"},
            "callback_on_tool_interrupted": {
                "agent",
                "call_id",
                "tool",
                "input",
                "error",
            },
            "callback_on_final_answer": {"agent", "answer"},
            "callback_on_iteration": {"agent", "iteration"},
        }
        if (
            operation not in fields
            or set(data) - fields[operation]
            or not isinstance(data.get("agent"), str)
        ):
            raise EchoAgentError("serialization_violation", "callback fields malformed")
        call_id = data.get("call_id")
        if call_id is not None and (not isinstance(call_id, str) or not call_id):
            raise EchoAgentError("serialization_violation", "canonical call id invalid")
        if operation.startswith("callback_on_tool_"):
            if not isinstance(data.get("tool"), str):
                raise EchoAgentError(
                    "serialization_violation", "tool callback requires tool"
                )
            if operation == "callback_on_tool_interrupted" and (
                call_id is None
                or not isinstance(data.get("input"), Mapping)
                or not isinstance(data["input"].get("kind"), str)
            ):
                raise EchoAgentError(
                    "serialization_violation",
                    "interrupted callback requires canonical call and full input",
                )
        for field in fields[operation] & {
            "prompt_tokens",
            "completion_tokens",
            "iteration",
            "max_iterations",
        }:
            value = data.get(field)
            if not isinstance(value, str):
                raise EchoAgentError(
                    "serialization_violation",
                    "callback counters require canonical text",
                )
            wire_u64(value)
        request_types = {
            "callback_on_think_start": CallbackThinkStart,
            "callback_on_think_end": CallbackThinkEnd,
            "callback_on_tool_start": CallbackToolStart,
            "callback_on_tool_end": CallbackToolEnd,
            "callback_on_tool_error": CallbackToolError,
            "callback_on_tool_interrupted": CallbackToolInterrupted,
            "callback_on_final_answer": CallbackFinalAnswer,
            "callback_on_iteration": CallbackIteration,
        }
        if operation in {"callback_on_think_start", "callback_on_think_end"}:
            collection = "messages" if operation.endswith("_start") else "steps"
            if not isinstance(data.get(collection), list):
                raise EchoAgentError(
                    "serialization_violation", "callback collection requires array"
                )
        if operation in {
            "callback_on_tool_end",
            "callback_on_tool_error",
            "callback_on_tool_interrupted",
            "callback_on_final_answer",
        }:
            field = (
                "result"
                if operation.endswith("_end")
                else "answer"
                if operation.endswith("_answer")
                else "error"
            )
            if not isinstance(data.get(field), str):
                raise EchoAgentError(
                    "serialization_violation", "callback outcome requires text"
                )
        if operation == "callback_on_tool_start" and (
            not isinstance(data.get("args"), Mapping)
            or not isinstance(data["args"].get("kind"), str)
        ):
            raise EchoAgentError(
                "serialization_violation", "callback args require WireValue"
            )
        try:
            request = request_types[operation](**data)
        except TypeError as error:
            raise EchoAgentError("serialization_violation", str(error)) from error
        return cls(
            operation,
            data["agent"],
            dict(data),
            WireHandle.from_dict(raw.get("extension")),
            str(raw["invocation_id"]),
            raw.get("context"),
            call_id,
            request,
        )

    def completed(self):
        return {
            "outcome": "result",
            "result": {"operation": self.operation, "value": None},
        }
