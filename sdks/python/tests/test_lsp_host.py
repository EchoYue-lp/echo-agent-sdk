import pytest

from echo_agent_sdk.catalog import FacadeCatalog


def test_lsp_methods_are_retired_from_the_language_catalog():
    for operation in (
        "lsp.manager.open",
        "lsp.client.hover",
        "lsp.client.goto_definition",
    ):
        with pytest.raises(KeyError, match="absent"):
            FacadeCatalog().resolve(operation)
