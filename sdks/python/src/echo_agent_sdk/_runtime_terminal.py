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
