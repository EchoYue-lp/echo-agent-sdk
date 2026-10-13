import json
from pathlib import Path

import pytest

from echo_agent_sdk import from_wire, to_wire

CORPUS = json.loads(
    (
        Path(__file__).parents[2] / "shared/fixtures/testing-contract-values.json"
    ).read_text()
)


@pytest.mark.parametrize("item", CORPUS["cases"], ids=lambda item: item["name"])
def test_testing_accepted_payload_round_trip(item):
    encoded = to_wire(item["native_input"])
    assert encoded == item["expected_wire"]
    projected = from_wire(json.loads(json.dumps(encoded)))
    assert projected == item["expected_projection"]
    if item["name"] == "context-absence-distinct":
        assert "call_id" not in projected
    if item["name"] == "context-null-identities":
        assert projected["call_id"] is None
    if item["name"] == "delay-large-subsecond":
        assert projected["value"]["seconds"] == "18446744073709551615"
        assert projected["value"]["nanos"] == 999999999


@pytest.mark.parametrize("item", CORPUS["invalid_cases"], ids=lambda item: item["name"])
def test_testing_malformed_projection_fails_explicitly(item):
    with pytest.raises((TypeError, ValueError, OverflowError, RuntimeError)) as failure:
        from_wire(item["wire"])
    assert str(failure.value)
