from copy import deepcopy

import pytest

from echo_agent_sdk import PromptCacheLayout, SegmentRanges, to_wire


def _message(role: str, text: str) -> dict[str, object]:
    return {"role": role, "content": to_wire(text)}


def _tail(body: str) -> str:
    return (
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion="v1"\n\n'
        + body
    )


def test_prompt_cache_layout_projects_segments_and_ranges() -> None:
    layout = PromptCacheLayout.from_messages(
        [
            _message("system", "You are Echo Agent"),
            _message("system", "[Canonical context - restored]"),
            _message("user", "hello"),
            _message("assistant", "hi"),
            _message("user", _tail("turn 1")),
            _message("user", _tail("hook")),
        ],
        [{"name": "lookup"}],
    )

    assert len(layout.system) == 1
    assert len(layout.canonical) == 1
    assert len(layout.history) == 2
    assert len(layout.runtime_context) == 2
    assert len(layout.tools) == 1
    ranges = layout.segment_ranges()
    assert ranges == SegmentRanges(
        system=ranges.system,
        canonical=ranges.canonical,
        history=ranges.history,
        runtime_context=ranges.runtime_context,
    )
    assert (ranges.system.start, ranges.system.end) == (0, 1)
    assert (ranges.canonical.start, ranges.canonical.end) == (1, 2)
    assert (ranges.history.start, ranges.history.end) == (2, 4)
    assert (ranges.runtime_context.start, ranges.runtime_context.end) == (4, 6)


def test_prompt_cache_layout_leaves_canonical_empty_without_marker() -> None:
    layout = PromptCacheLayout.from_messages(
        [_message("system", "S"), _message("user", "hello")], []
    )
    assert layout.canonical == ()
    assert len(layout.history) == 1
    assert layout.segment_ranges().canonical.is_empty()


# 显式边界期望不复制生产分段算法。
_HEADER_CASES = [
    (
        "valid_tail",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion="v1"\n\n事实🙂',
        True,
    ),
    (
        "valid_empty_body",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion="v1"\n\n',
        True,
    ),
    (
        "valid_json_whitespace",
        '<echo-agent-context-projection-v2>\nmarker= " " \nplacement=tail\nversion= "v" \n\nbody',
        True,
    ),
    (
        "valid_json_unicode",
        '<echo-agent-context-projection-v2>\nmarker="\\u4e2d\\ud83d\\ude42"\nplacement=tail\nversion="\\u7248"\n\nbody',
        True,
    ),
    (
        "valid_crlf_header_lines",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\r\nplacement=tail\r\nversion="v1"\n\nbody',
        True,
    ),
    ("bare_prefix", "[runtime_context: user text]", False),
    ("bare_prefix_after_spaces", "  [runtime_context: user text]", False),
    (
        "legacy_v1",
        '<echo-agent-context-projection-v1>\nmarker="runtime"\nversion="body"',
        False,
    ),
    (
        "boundary",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=boundary\n\nbody',
        False,
    ),
    (
        "boundary_body_version",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=boundary\n\nversion="body"',
        False,
    ),
    (
        "unterminated_header",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion="v1"',
        False,
    ),
    (
        "missing_version",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\n\nversion="body"',
        False,
    ),
    (
        "extra_header",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion="v1"\nextra="x"\n\nbody',
        False,
    ),
    (
        "empty_marker",
        '<echo-agent-context-projection-v2>\nmarker=""\nplacement=tail\nversion="v1"\n\nbody',
        False,
    ),
    (
        "empty_version",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion=""\n\nbody',
        False,
    ),
    (
        "numeric_marker",
        '<echo-agent-context-projection-v2>\nmarker=1\nplacement=tail\nversion="v1"\n\nbody',
        False,
    ),
    (
        "numeric_version",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion=1\n\nbody',
        False,
    ),
    (
        "bad_json_escape",
        '<echo-agent-context-projection-v2>\nmarker="\\q"\nplacement=tail\nversion="v1"\n\nbody',
        False,
    ),
    (
        "trailing_marker_json",
        '<echo-agent-context-projection-v2>\nmarker="runtime" true\nplacement=tail\nversion="v1"\n\nbody',
        False,
    ),
    (
        "trailing_version_json",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion="v1" "v2"\n\nbody',
        False,
    ),
    (
        "unpaired_marker_surrogate",
        '<echo-agent-context-projection-v2>\nmarker="\\ud800"\nplacement=tail\nversion="v1"\n\nbody',
        False,
    ),
    (
        "unpaired_version_surrogate",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion="\\udc00"\n\nbody',
        False,
    ),
    (
        "leading_whitespace",
        ' <echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=tail\nversion="v1"\n\nbody',
        False,
    ),
    (
        "wrong_header_order",
        '<echo-agent-context-projection-v2>\nplacement=tail\nmarker="runtime"\nversion="v1"\n\nbody',
        False,
    ),
    (
        "extra_boundary_version",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=boundary\nversion="v1"\n\nbody',
        False,
    ),
    (
        "non_tail_placement",
        '<echo-agent-context-projection-v2>\nmarker="runtime"\nplacement=Tail\nversion="v1"\n\nbody',
        False,
    ),
]


@pytest.mark.parametrize(
    "name,text,dynamic", _HEADER_CASES, ids=[case[0] for case in _HEADER_CASES]
)
def test_prompt_cache_layout_v2_header(name: str, text: str, dynamic: bool) -> None:
    messages = [
        _message("system", "S"),
        _message("user", "history"),
        _message("user", text),
    ]
    tools = [{"name": "second"}, {"name": "first"}]
    before = deepcopy((messages, tools))
    layout = PromptCacheLayout.from_messages(messages, tools)
    assert layout.history == tuple(messages[1:2] if dynamic else messages[1:]), name
    assert layout.runtime_context == tuple(messages[2:] if dynamic else []), name
    assert layout.tools == tuple(tools)
    assert layout.segment_ranges().runtime_context.start == (2 if dynamic else 3)
    assert layout.segment_ranges().runtime_context.end == 3
    assert (messages, tools) == before
    assert not hasattr(layout, "dynamic")


def test_prompt_cache_layout_stops_at_ordinary_history_and_any_system_role() -> None:
    system = _message("system", "S")
    history = _message("user", "history")
    fact = _message("user", _tail("fact"))
    late_system = _message("system", _tail("system fact"))
    for messages in ([system, fact, history], [system, history, fact, late_system]):
        layout = PromptCacheLayout.from_messages(messages, [])
        assert layout.history == tuple(messages[1:])
        assert layout.runtime_context == ()
    messages = [system, history, fact, late_system, fact]
    layout = PromptCacheLayout.from_messages(messages, [])
    assert layout.history == tuple(messages[1:4])
    assert layout.runtime_context == (fact,)
    assert (
        layout.segment_ranges().runtime_context.start,
        layout.segment_ranges().runtime_context.end,
    ) == (4, 5)


def test_prompt_cache_layout_only_first_text_part_can_supply_envelope() -> None:
    for parts, expected in (
        (
            [
                {"type": "text", "text": _tail("fact")},
                {"type": "text", "text": "extra"},
            ],
            1,
        ),
        (
            [
                {
                    "type": "image_url",
                    "image_url": {"url": "data:image/png;base64,AA=="},
                },
                {"type": "text", "text": _tail("fact")},
            ],
            0,
        ),
        (
            [
                {"type": "text", "text": "<echo-agent-context-projection-v2>\n"},
                {
                    "type": "text",
                    "text": 'marker="runtime"\nplacement=tail\nversion="v1"\n\nbody',
                },
            ],
            0,
        ),
    ):
        layout = PromptCacheLayout.from_messages(
            [_message("system", "S"), {"role": "user", "content": to_wire(parts)}], []
        )
        assert len(layout.runtime_context) == expected


def test_prompt_cache_layout_empty_and_entirely_system() -> None:
    for messages in ([], [_message("system", _tail("fact")), _message("system", "S")]):
        layout = PromptCacheLayout.from_messages(messages, [])
        assert layout.system == tuple(messages)
        assert layout.history == ()
        assert layout.runtime_context == ()


_CANONICAL_PART_CASES = [
    ("single_text", [{"type": "text", "text": "Canonical context"}], True),
    (
        "split_text_no_separator",
        [{"type": "text", "text": "Canonical "}, {"type": "text", "text": "context"}],
        True,
    ),
    (
        "split_text_no_inserted_space",
        [{"type": "text", "text": "Canonical"}, {"type": "text", "text": "context"}],
        False,
    ),
    (
        "ignored_media_between_text",
        [
            {"type": "text", "text": "Canonical"},
            {"type": "file", "name": "ignored", "content": "AA=="},
            {"type": "text", "text": " context"},
        ],
        True,
    ),
    (
        "media_names_are_not_text",
        [
            {"type": "file", "name": "Canonical context", "content": "AA=="},
            {"type": "image_url", "image_url": {"url": "Canonical context"}},
        ],
        False,
    ),
    ("empty_parts", [], False),
    ("empty_text", [{"type": "text", "text": ""}], False),
    (
        "resource_title_missing_optionals",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "title": "Canonical context",
                },
            }
        ],
        True,
    ),
    (
        "resource_name",
        [
            {
                "type": "resource_link",
                "resource": {"name": "Canonical context", "uri": "resource://report"},
            }
        ],
        True,
    ),
    (
        "resource_uri",
        [
            {
                "type": "resource_link",
                "resource": {"name": "report", "uri": "Canonical context"},
            }
        ],
        True,
    ),
    (
        "resource_description",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "description": "Canonical context",
                },
            }
        ],
        True,
    ),
    (
        "resource_mime",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "mime_type": "Canonical context",
                },
            }
        ],
        True,
    ),
    (
        "resource_annotations_value",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "annotations": {"note": "Canonical context"},
                },
            }
        ],
        True,
    ),
    (
        "resource_annotations_key",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "annotations": {"Canonical context": True},
                },
            }
        ],
        True,
    ),
    (
        "resource_meta",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "meta": {"note": "Canonical context"},
                },
            }
        ],
        True,
    ),
    (
        "resource_minimal_no_marker",
        [
            {
                "type": "resource_link",
                "resource": {"name": "report", "uri": "resource://report"},
            }
        ],
        False,
    ),
    (
        "resource_unknown_field_ignored",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "extra": "Canonical context",
                },
            }
        ],
        False,
    ),
    (
        "resource_prevents_text_join",
        [
            {"type": "text", "text": "Canonical"},
            {
                "type": "resource_link",
                "resource": {"name": "report", "uri": "resource://report"},
            },
            {"type": "text", "text": " context"},
        ],
        False,
    ),
    (
        "resource_fields_do_not_join",
        [
            {
                "type": "resource_link",
                "resource": {"name": "Canonical", "uri": " context"},
            }
        ],
        False,
    ),
    (
        "resource_json_newline_not_space",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "title": "Canonical\ncontext",
                },
            }
        ],
        False,
    ),
    (
        "resource_empty_optionals",
        [
            {
                "type": "resource_link",
                "resource": {
                    "name": "report",
                    "uri": "resource://report",
                    "description": "",
                    "mime_type": "",
                    "title": "",
                    "annotations": None,
                    "meta": None,
                    "size": None,
                },
            }
        ],
        False,
    ),
    (
        "text_after_resource",
        [
            {
                "type": "resource_link",
                "resource": {"name": "report", "uri": "resource://report"},
            },
            {"type": "text", "text": "Canonical context"},
        ],
        True,
    ),
]


@pytest.mark.parametrize(
    "name,parts,canonical",
    _CANONICAL_PART_CASES,
    ids=[case[0] for case in _CANONICAL_PART_CASES],
)
def test_prompt_cache_layout_canonical_text_view(
    name: str, parts: list[dict[str, object]], canonical: bool
) -> None:
    system = {"role": "system", "content": to_wire(parts)}
    before = deepcopy(system)
    layout = PromptCacheLayout.from_messages([system], [])
    assert layout.system == (() if canonical else (system,)), name
    assert layout.canonical == ((system,) if canonical else ()), name
    assert layout.history == ()
    assert layout.runtime_context == ()
    assert layout.segment_ranges().system.end == (0 if canonical else 1)
    assert layout.segment_ranges().canonical.end == 1
    assert system == before


def test_prompt_cache_layout_canonical_keeps_system_boundary_and_dynamic_first_part() -> (
    None
):
    canonical = {
        "role": "system",
        "content": to_wire([{"type": "text", "text": "Canonical context"}]),
    }
    system = _message("system", "S")
    history = _message("user", "history")
    fact = _message("user", _tail("fact"))
    layout = PromptCacheLayout.from_messages(
        [system, canonical, system, history, fact], []
    )
    assert layout.system == (system,)
    assert layout.canonical == (canonical, system)
    assert layout.history == (history,)
    assert layout.runtime_context == (fact,)
    late = PromptCacheLayout.from_messages([system, history, canonical], [])
    assert late.canonical == ()
    assert late.history == (history, canonical)
    assert late.runtime_context == ()
    resource_then_text = {
        "role": "user",
        "content": to_wire(
            [
                {
                    "type": "resource_link",
                    "resource": {
                        "name": "report",
                        "uri": "resource://report",
                        "title": _tail("fact"),
                    },
                },
                {"type": "text", "text": _tail("fact")},
            ]
        ),
    }
    assert (
        PromptCacheLayout.from_messages([resource_then_text], []).runtime_context == ()
    )
