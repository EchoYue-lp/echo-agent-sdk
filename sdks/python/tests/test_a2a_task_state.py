import pytest

import echo_agent_sdk as sdk
from echo_agent_sdk.catalog import FacadeCatalog


def test_retired_values_and_operations_are_absent():
    for name in (
        "A2AMessage",
        "A2ATask",
        "AgentCard",
        "AgentCardBuilder",
        "TaskState",
        "JwtConfig",
        "JwtClaims",
    ):
        assert not hasattr(sdk, name), name
    catalog = FacadeCatalog()
    for operation in ("a2a.client.open", "lsp.manager.open", "lsp.client.hover"):
        with pytest.raises(KeyError, match="absent"):
            catalog.resolve(operation)
    assert sdk.EchoAgentClient
