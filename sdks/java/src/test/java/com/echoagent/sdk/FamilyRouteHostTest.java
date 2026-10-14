package com.echoagent.sdk;

import java.util.List;
import java.util.Map;
import java.nio.file.Path;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

public class FamilyRouteHostTest {
    @Test
    public void nativeCallsUseCatalogWireMethod() throws Exception {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) {
            assertTrue(host != null && !host.isBlank(), "required family-route Host missing");
        }
        assumeTrue(host != null && !host.isBlank(), "real Host binary required");
        try (EchoAgentClient sdk = EchoAgentClient.spawn(host,
                List.of("--config", System.getenv("ECHO_AGENT_SDK_CONFIG")),
                Path.of("../shared/facade-operation-catalog.json"),
                Path.of("../shared/contract-digests.json"), Map.of())
                .toCompletableFuture().get(10, TimeUnit.SECONDS)) {
            var agent = sdk.createAgent().toCompletableFuture().get(10, TimeUnit.SECONDS);
            var session = agent.createSession().toCompletableFuture().get(10, TimeUnit.SECONDS);
            String text = "中😀 alice@example.com 尾";
            String expected = "中😀 [REDACTED:EMAIL] 尾";
            assertEquals(expected, sdk.call("content-guard.redact_exact", session.wire(), List.of(text)).toCompletableFuture().get(10, TimeUnit.SECONDS).textValue());
            assertEquals(expected, sdk.family("content_guard", "content-guard.redact_exact", session.wire(), List.of(text)).toCompletableFuture().get(10, TimeUnit.SECONDS).textValue());
            assertThrows(IllegalArgumentException.class, () -> sdk.family("memory", "content-guard.redact_exact", session.wire(), List.of(text)));
            session.close();
            agent.close();
        }
    }
}
