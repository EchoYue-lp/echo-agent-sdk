"""Replay the original Java ACP timeout; reject unrelated transport failures."""

import copy

import pytest
from tool_value_effect_fixture import _verify_external_error


@pytest.fixture
def actual_timeout_row():
    # Exact error payload from the retained be5 whole-language failure.
    return {
        "id": "research-semantic_scholar_search",
        "operation": "semantic_scholar_search",
        "expected": "external",
        "error": {
            "code": "transport_error",
            "message": "Did not observe any item or terminal signal within 30000ms in 'source(MonoDeferContextual)' (and no fallback has been configured)",
            "operation": "_echo_agent/research/op",
            "retryable": "never",
            "handle": None,
            "details": None,
        },
    }


def test_original_timeout_is_preserved_as_external_error(actual_timeout_row):
    before = copy.deepcopy(actual_timeout_row)
    assert (
        _verify_external_error(actual_timeout_row, actual_timeout_row)
        == "transport_timeout"
    )
    assert actual_timeout_row == before and "result" not in actual_timeout_row


@pytest.mark.parametrize(
    "field,value",
    [
        ("operation", "_echo_agent/files/op"),
        ("retryable", "always"),
        ("message", ""),
        ("message", "ACP connection closed"),
        (
            "message",
            "Did not observe any item or terminal signal within 1ms in 'source(MonoDeferContextual)' (and no fallback has been configured)",
        ),
        ("code", "closed_handle"),
        ("handle", {"id": "gone"}),
        ("details", {"cause": "disconnected"}),
    ],
)
def test_unrelated_or_malformed_error_is_rejected(actual_timeout_row, field, value):
    actual_timeout_row["error"][field] = value
    with pytest.raises(AssertionError):
        _verify_external_error(actual_timeout_row, actual_timeout_row)


def test_missing_error_and_fabricated_result_are_rejected(actual_timeout_row):
    for row in (
        {},
        {**actual_timeout_row, "result": {"success": True}},
        {**actual_timeout_row, "error": None},
    ):
        with pytest.raises((AssertionError, KeyError)):
            _verify_external_error(row, actual_timeout_row)


def test_framework_tool_cause_fields_are_checked(actual_timeout_row):
    row = copy.deepcopy(actual_timeout_row)
    row["error"] = {
        "code": "framework_error",
        "message": "Tool Error: Tool 'semantic_scholar_search' execution failed: Semantic Scholar API error (429 Too Many Requests)",
        "operation": "semantic_scholar_search",
        "retryable": "never",
        "handle": None,
        "details": {
            "fields": [
                {"key": "framework_failure_category", "value": "tool"},
                {"key": "framework_terminal_kind", "value": "failed"},
                {"key": "framework_failure_code", "value": "tool"},
                {"key": "framework_retryable", "value": "false"},
            ]
        },
    }
    assert _verify_external_error(row, row) == "framework_error"
    row["error"]["details"]["fields"][0]["value"] = "transport"
    with pytest.raises(AssertionError):
        _verify_external_error(row, row)
