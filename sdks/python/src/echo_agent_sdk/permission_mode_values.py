from __future__ import annotations

from enum import Enum


class PermissionMode(str, Enum):
    DEFAULT = "default"
    PLAN = "plan"
    ACCEPT_EDITS = "auto-edit"
    BYPASS_PERMISSIONS = "full-auto"
    AUTO = "auto"
    BUBBLE = "bubble"
    DONT_ASK = "dont-ask"
    STRICT_CONFIRM = "strict"

    @classmethod
    def parse(cls, value: str) -> PermissionMode:
        if not isinstance(value, str):
            raise TypeError("permission mode must be text")
        # 使用枚举原canonical值，不以strip/lower重新接纳退出的拼写。
        try:
            return cls(value)
        except ValueError as error:
            raise ValueError(f"invalid permission mode '{value}'") from error

    def allows_write(self) -> bool:
        return self in {self.BYPASS_PERMISSIONS, self.ACCEPT_EDITS}

    def requires_interaction(self) -> bool:
        return self not in {
            self.BYPASS_PERMISSIONS,
            self.AUTO,
            self.DONT_ASK,
            self.ACCEPT_EDITS,
        }

    def uses_classifier(self) -> bool:
        return self is self.AUTO

    def __str__(self) -> str:
        return self.value
