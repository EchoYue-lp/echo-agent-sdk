import pytest

from echo_agent_sdk.catalog import FacadeCatalog


def test_lsp_diagnostics_is_not_a_product_operation():
    with pytest.raises(KeyError, match="absent"):
        FacadeCatalog().resolve("lsp.client.diagnostics")
