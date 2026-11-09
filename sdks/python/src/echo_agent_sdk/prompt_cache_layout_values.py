from __future__ import annotations

import json
from collections.abc import Mapping, Sequence
from dataclasses import dataclass, field
from typing import Any

from .segment_range_values import SegmentRange
from .wire import from_wire


@dataclass(frozen=True, slots=True)
class SegmentRanges:
    """Message-cache segment indexes without owning cache state."""

    system: SegmentRange = field(default_factory=SegmentRange)
    canonical: SegmentRange = field(default_factory=SegmentRange)
    history: SegmentRange = field(default_factory=SegmentRange)
    runtime_context: SegmentRange = field(default_factory=SegmentRange)


@dataclass(frozen=True, slots=True)
class PromptCacheLayout:
    """Read-only prompt layout projection; cache placement stays in Rust."""

    system: tuple[Mapping[str, Any], ...]
    canonical: tuple[Mapping[str, Any], ...]
    history: tuple[Mapping[str, Any], ...]
    runtime_context: tuple[Mapping[str, Any], ...]
    tools: tuple[Mapping[str, Any], ...]

    @classmethod
    def from_messages(
        cls,
        messages: Sequence[Mapping[str, Any]],
        tools: Sequence[Mapping[str, Any]],
    ) -> PromptCacheLayout:
        if not isinstance(messages, Sequence) or isinstance(messages, (str, bytes)):
            raise TypeError("messages must be a sequence")
        if not isinstance(tools, Sequence) or isinstance(tools, (str, bytes)):
            raise TypeError("tools must be a sequence")
        message_values = _snapshot_values(messages, "messages")
        tool_values = _snapshot_values(tools, "tools")

        system_end = next(
            (
                index
                for index, message in enumerate(message_values)
                if message.get("role") != "system"
            ),
            len(message_values),
        )
        canonical_start = next(
            (
                index
                for index, message in enumerate(message_values[:system_end])
                if (_message_text(message) or "").find("Canonical context") >= 0
            ),
            system_end,
        )

        runtime_start = len(message_values)
        # 旧公开字段名保持；其值只对应框架严格识别的连续 dynamic 尾段。
        while runtime_start > system_end:
            message = message_values[runtime_start - 1]
            if message.get("role") == "system" or not _is_dynamic_tail_message(message):
                break
            runtime_start -= 1

        return cls(
            tuple(message_values[: min(canonical_start, system_end)]),
            tuple(message_values[canonical_start:system_end])
            if canonical_start < system_end
            else (),
            tuple(message_values[system_end:runtime_start]),
            tuple(message_values[runtime_start:]),
            tuple(tool_values),
        )

    def segment_ranges(self) -> SegmentRanges:
        system = len(self.system)
        canonical = len(self.canonical)
        history = len(self.history)
        runtime_context = len(self.runtime_context)
        canonical_end = system + canonical
        history_end = canonical_end + history
        return SegmentRanges(
            system=SegmentRange(0, system),
            canonical=SegmentRange(system, canonical_end),
            history=SegmentRange(canonical_end, history_end),
            runtime_context=SegmentRange(history_end, history_end + runtime_context),
        )


def _snapshot_values(
    values: Sequence[Mapping[str, Any]], name: str
) -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    for value in values:
        if not isinstance(value, Mapping):
            raise TypeError(f"{name} entries must be objects")
        result.append(dict(value))
    return result


def _message_text(message: Mapping[str, Any]) -> str | None:
    content = from_wire(message.get("content"))
    if isinstance(content, str):
        return content or None
    if not isinstance(content, list):
        return None
    texts = []
    for part in content:
        if not isinstance(part, Mapping):
            continue
        if part.get("type") == "text" and isinstance(part.get("text"), str):
            texts.append(part["text"])
        elif part.get("type") == "resource_link" and isinstance(
            part.get("resource"), Mapping
        ):
            resource = part["resource"]
            # 与 LinkedResource 字段一致：缺失 Option 为 null，忽略未知扩展字段。
            fields = {
                name: resource.get(name)
                for name in (
                    "annotations",
                    "description",
                    "mime_type",
                    "name",
                    "size",
                    "title",
                    "uri",
                    "meta",
                )
            }
            texts.append(
                "[Linked resource]\n"
                + json.dumps(fields, ensure_ascii=False, separators=(",", ":"))
            )
    # 完整文字视图只用于 canonical，不能供 dynamic 拼接信封。
    return "".join(texts) if texts else None


def _is_dynamic_tail_message(message: Mapping[str, Any]) -> bool:
    content = from_wire(message.get("content"))
    text = content if isinstance(content, str) else None
    # 不拼接 Text parts：信封必须完整存在于首个 Text part。
    if isinstance(content, list) and content:
        first = content[0]
        if isinstance(first, Mapping) and first.get("type") == "text":
            value = first.get("text")
            text = value if isinstance(value, str) else None
    prefix = "<echo-agent-context-projection-v2>\n"
    if text is None or not text.startswith(prefix):
        return False
    header, separator, _ = text[len(prefix) :].partition("\n\n")
    if not separator:
        return False
    # str.splitlines 还识别 Unicode 分隔符；框架只按 LF/CRLF 切行。
    lines = [line.removesuffix("\r") for line in header.split("\n")]
    if (
        len(lines) != 3
        or not lines[0].startswith("marker=")
        or lines[1] != "placement=tail"
        or not lines[2].startswith("version=")
    ):
        return False
    return _is_projection_string(lines[0][len("marker=") :]) and _is_projection_string(
        lines[2][len("version=") :]
    )


def _is_projection_string(raw: str) -> bool:
    try:
        value = json.loads(raw)
    except ValueError:
        return False
    # json.loads 允许未配对 surrogate；框架 serde_json::String 不接受。
    return (
        isinstance(value, str)
        and bool(value)
        and not any(0xD800 <= ord(character) <= 0xDFFF for character in value)
    )
