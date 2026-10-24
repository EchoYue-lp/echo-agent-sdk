import echo_agent_sdk as sdk


def test_a2a_jwt_exports_are_retired():
    assert not hasattr(sdk, "JwtConfig")
    assert not hasattr(sdk, "JwtClaims")
    assert sdk.EchoAgentClient
