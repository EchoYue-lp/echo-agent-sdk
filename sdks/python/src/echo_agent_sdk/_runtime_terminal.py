"""读取Framework V6执行事实；不创建运行状态，不由文本推断结束。"""

from collections.abc import Mapping
from typing import Any

from .errors import EchoAgentError
from .wire import MAX_U64, from_wire


def runtime_terminal(envelope: Mapping[str, Any]) -> bool:
    payload = envelope.get("payload", {})
    if (
        not isinstance(payload, Mapping)
        or payload.get("event_type") != "execution_status"
    ):
        return False
    try:
        value = from_wire(payload.get("data"))
    except (TypeError, ValueError, OverflowError, EchoAgentError) as error:
        raise EchoAgentError(
            "serialization_violation",
            f"malformed runtime execution status: {error}",
            operation="_echo_agent/event",
        ) from error
    return _runtime_status_terminal(value)


def _runtime_status_terminal(value: Any) -> bool:
    """复用原状态校验；不改变身份、顺序、原因或执行权威。"""
    invalid = EchoAgentError(
        "serialization_violation",
        "malformed runtime execution status",
        operation="_echo_agent/event",
    )
    if not isinstance(value, dict):
        raise invalid

    def text(value: Any) -> bool:
        return isinstance(value, str) and bool(value.strip())

    if (
        not text(value.get("execution_id"))
        or (value.get("request_id") is not None and not text(value["request_id"]))
        or value.get("phase") not in ("Thinking", "Acting", "Terminal")
        or value.get("status")
        not in (
            "started",
            "waiting",
            "tool_pending",
            "candidate",
            "settled",
            "cancelled",
            "failed",
        )
        or (value.get("reason") is not None and not isinstance(value["reason"], str))
    ):
        raise invalid
    sequence = value.get("sequence")
    if (
        isinstance(sequence, str)
        and sequence.isascii()
        and sequence.isdigit()
        and len(sequence) <= 20
        and (sequence == "0" or not sequence.startswith("0"))
    ):
        sequence = int(sequence)
    if type(sequence) is not int or not 0 <= sequence <= MAX_U64:
        raise invalid
    return value["phase"] == "Terminal" and value["status"] in {
        "settled",
        "cancelled",
        "failed",
    }


def validate_reverse_agent_value(value: Any, complete: bool) -> None:
    if not isinstance(value, Mapping) or value.get("kind") != "agent":
        return
    event = value.get("value")
    if not isinstance(event, Mapping) or event.get("event") != "execution_status":
        return
    data = event.get("data")
    if (
        not isinstance(data, dict)
        or not isinstance(data.get("sequence"), str)
        or set(data)
        - {"execution_id", "request_id", "sequence", "phase", "status", "reason"}
    ):
        raise ValueError(
            "reverse execution status requires closed native wire fields and u64 text"
        )
    terminal = _runtime_status_terminal(data)
    terminal_kind = data.get("status") in {"settled", "failed", "cancelled"}
    if terminal_kind != complete or (terminal_kind and not terminal):
        raise ValueError(
            "reverse execution status must match chunk or terminal completion"
        )
