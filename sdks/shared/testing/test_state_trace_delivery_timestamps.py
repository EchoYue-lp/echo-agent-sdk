"""Observable precision and full-document boundaries of the State/Trace oracle."""

import pytest
from state_trace_delivery_fixture import compare_timestamp_documents, exact_timestamp

START = "2026-10-13T13:00:00.123456789+08:00"
UTC_START = "2026-10-13T05:00:00.123456789+00:00"
FINISH = "2026-10-13T13:00:01.987654321+08:00"
UTC_FINISH = "2026-10-13T05:00:01.987654321+00:00"


def test_same_instant_preserves_last_three_nanoseconds_and_accepts_z_input():
    expected = {"epoch_seconds": "1791867600", "nanos": 123456789}
    assert exact_timestamp(START) == expected
    assert exact_timestamp(UTC_START) == expected
    assert exact_timestamp("2026-10-13T05:00:00.123456789Z") == expected
    assert exact_timestamp(FINISH) == {
        "epoch_seconds": "1791867601",
        "nanos": 987654321,
    }
    assert exact_timestamp("1969-12-31T23:59:59.000000001Z") == {
        "epoch_seconds": "-1",
        "nanos": 1,
    }


@pytest.mark.parametrize(
    "changed",
    ["2026-10-13T05:00:00.123456788+00:00", "2026-10-13T05:00:01.123456789+00:00"],
)
def test_one_nanosecond_or_second_difference_is_rejected(changed):
    with pytest.raises(AssertionError):
        compare_timestamp_documents(
            {"timestamp": changed}, {"timestamp": START}, ["timestamp"]
        )


@pytest.mark.parametrize(
    "bad",
    [
        "2026-10-13T05:00:00.+00:00",
        "2026-10-13T05:00:00.1234567891+00:00",
        "2026-10-13T05:00:00.１２３+00:00",
        "２０２６-10-13T05:00:00Z",
        "2026-10-13T05:00:00Z\n",
        "2026-10-13T05:00:00+24:00",
        "2026-10-13T05:00:00+00:60",
        "2026-02-30T05:00:00Z",
        None,
    ],
)
def test_invalid_rfc3339_is_not_defaulted_or_truncated(bad):
    with pytest.raises((AssertionError, TypeError, ValueError)):
        exact_timestamp(bad)


def test_complete_fields_and_explicit_host_local_output():
    expected = {
        "started_at": START,
        "finished_at": FINISH,
        "nullable": None,
        "unknown": {"timestamp": "leave literal"},
        "values": ["a", "b"],
    }
    actual = {**expected, "started_at": UTC_START, "finished_at": UTC_FINISH}
    raw = dict(actual)
    rows = compare_timestamp_documents(
        actual, expected, ["started_at", "finished_at"], "UTC"
    )
    assert actual == raw and rows[0]["actual_raw"] == UTC_START
    assert rows[1]["actual_instant"]["nanos"] == 987654321
    compare_timestamp_documents(
        expected, expected, ["started_at", "finished_at"], "Asia/Shanghai"
    )
    with pytest.raises(AssertionError):
        compare_timestamp_documents(
            actual, expected, ["started_at", "finished_at"], "Asia/Shanghai"
        )
    with pytest.raises(AssertionError):
        compare_timestamp_documents(
            {**actual, "started_at": UTC_START.replace("+00:00", "Z")},
            expected,
            ["started_at", "finished_at"],
            "UTC",
        )
    for changed in [
        {**actual, "unknown": {"timestamp": "changed"}},
        {**actual, "values": ["b", "a"]},
        {**actual, "new_key": None},
        {key: value for key, value in actual.items() if key != "nullable"},
        {**actual, "nullable": False},
    ]:
        with pytest.raises(AssertionError):
            compare_timestamp_documents(
                changed, expected, ["started_at", "finished_at"]
            )


def test_optional_null_is_not_missing_and_unlisted_fields_cannot_be_normalized():
    value = {"started_at": START, "finished_at": None}
    assert compare_timestamp_documents(value, value, ["started_at", "finished_at"])[
        1
    ] == {"field": "finished_at", "actual_raw": None, "expected_raw": None}
    for invalid in [
        {"started_at": START},
        {"started_at": START, "finished_at": FINISH},
        {"started_at": None, "finished_at": None},
    ]:
        with pytest.raises(AssertionError):
            compare_timestamp_documents(invalid, value, ["started_at", "finished_at"])
    with pytest.raises(AssertionError):
        compare_timestamp_documents(
            {"claimed_at": START}, {"claimed_at": UTC_START}, ["claimed_at"]
        )
