import pytest

from echo_agent_sdk.catalog import FacadeCatalog


def test_lsp_family_is_retired_from_catalog():
    for operation in (
        "lsp.manager.open",
        "lsp.client.hover",
        "lsp.client.goto_definition",
    ):
        with pytest.raises(KeyError, match="absent"):
            FacadeCatalog().resolve(operation)
