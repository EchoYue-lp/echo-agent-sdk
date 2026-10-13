from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from .wire import WireHandle


class EchoAgentError(RuntimeError):
    """Typed SDK/ACP failure without parsing human-readable ACP messages."""

    def __init__(
        self,
        code: str,
        message: str,
        retryable: str = "never",
        operation: str | None = None,
        details: Any = None,
        handle: WireHandle | None = None,
    ) -> None:
        super().__init__(message)
        self.code = code
        self.message = message
        self.retryable = retryable
        self.operation = operation
        self.details = details
        self.handle = handle

    @classmethod
    def from_exception(
        cls, error: BaseException, operation: str | None = None
    ) -> EchoAgentError:
        data = getattr(error, "data", None)
        if not isinstance(data, dict):
            data = {}
        from .wire import WireHandle

        return cls(
            str(data.get("code", "transport_error")),
            str(data.get("message", str(error))),
            str(data.get("retryable", "never")),
            str(data["operation"])
            if isinstance(data.get("operation"), str)
            else operation,
            data.get("details"),
            WireHandle.from_dict(data["handle"])
            if data.get("handle") is not None
            else None,
        )
