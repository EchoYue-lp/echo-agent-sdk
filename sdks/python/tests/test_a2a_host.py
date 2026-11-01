import pytest

from echo_agent_sdk.catalog import FacadeCatalog


def test_retired_a2a_operations_are_not_catalog_entries():
    for operation in ("a2a.client.open", "a2a.discover", "a2a.task.stream.open"):
        with pytest.raises(KeyError, match="absent"):
            FacadeCatalog().resolve(operation)
