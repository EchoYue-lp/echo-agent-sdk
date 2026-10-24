package com.echoagent.sdk;

import java.nio.file.Path;
import java.nio.file.Files;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

class A2AHostTest {
    static <T> T await(CompletionStage<T> value) throws Exception {
        return value.toCompletableFuture().get(10, TimeUnit.SECONDS);
    }
    private static String host() {
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_HOST_E2E"))) assertNotNull(host);
        assumeTrue(host != null && System.getenv("ECHO_AGENT_SDK_CONFIG") != null);
        return host;
    }
    @Test void retiredFeaturesCannotNegotiate() {
        String host = host();
        for (String feature : List.of("a2a", "lsp")) {
            var error = assertThrows(Exception.class, () -> await(EchoAgentClient.spawn(host,
                    List.of("--config", System.getenv("ECHO_AGENT_SDK_CONFIG")),
                    Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of(), List.of(feature), List.of())));
            assertNotNull(error.getCause());
            assertEquals("feature_unavailable", EchoAgentException.from(error, "initialize").code());
        }
    }
    @Test void oldSourceDigestCannotNegotiate() throws Exception {
        String host = host();
        var retired = JsonSupport.MAPPER.readTree(Files.readString(Path.of("../shared/testing/a2a_lsp_retirement_contract.json")));
        var digests = (com.fasterxml.jackson.databind.node.ObjectNode) JsonSupport.MAPPER.readTree(Files.readString(Path.of("../shared/contract-digests.json")));
        digests.put("source_contract_digest", retired.path("old_source_contract_digest").asText());
        var path = Files.createTempDirectory("sdk-retired-digest-").resolve("contract-digests.json");
        Files.writeString(path, digests.toString());
        var error = assertThrows(Exception.class, () -> await(EchoAgentClient.spawn(host, List.of("--config", System.getenv("ECHO_AGENT_SDK_CONFIG")), Path.of("../shared/facade-operation-catalog.json"), path, Map.of())));
        assertEquals("extension_capability_mismatch", EchoAgentException.from(error, "initialize").code());
    }
    @Test void oldFamilyOperationsAndSourcesFailAgainstRetainedHost() throws Exception {
        String host = host();
        var retired = JsonSupport.MAPPER.readTree(Files.readString(Path.of("../shared/testing/a2a_lsp_retirement_contract.json")));
        try (var sdk = await(EchoAgentClient.spawn(host, List.of("--config", System.getenv("ECHO_AGENT_SDK_CONFIG")),
                Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()))) {
            var agent = await(sdk.createAgent());
            var session = await(agent.createSession());
            int count = 0;
            for (var family : retired.path("families")) {
                com.fasterxml.jackson.databind.JsonNode familyRoute = null;
                for (var route : retired.path("routes")) if (route.path("route").asText().equals("family:" + family.path("family").asText())) familyRoute = route;
                assertNotNull(familyRoute);
                for (var operation : family.path("operations")) {
                    String digest = null;
                    for (var item : familyRoute.path("operation_signatures")) if (item.path("operation").equals(operation)) digest = item.path("signature_digests").path(0).asText();
                    assertNotNull(digest);
                    var request = JsonSupport.MAPPER.createObjectNode().put("operation", operation.asText()).put("signature_digest", digest);
                    request.set("handle", session.wire().toJson());
                    request.putArray("arguments");
                    assertThrows(Exception.class, () -> await(sdk.request(family.path("methods").path(0).asText(), request)));
                    count++;
                }
            }
            assertEquals(39, count);
            for (var route : retired.path("routes")) if (!route.path("operation").isNull()) {
                var request = JsonSupport.MAPPER.createObjectNode().put("operation", route.path("operation").asText()).put("signature_digest", route.path("signature_digests").path(0).asText());
                request.set("handle", agent.wire().toJson());
                var argument = JsonSupport.MAPPER.createObjectNode().put("kind", "handle");
                argument.set("value", session.wire().toJson());
                request.putArray("arguments").add(argument);
                assertThrows(Exception.class, () -> await(sdk.request("_echo_agent/facade/invoke", request)));
            }
            for (String json : List.of(
                    "{\"kind\":\"variant\",\"value\":{\"type_id\":\"echo_core::plugin::capability::PluginCapability\",\"variant\":\"LspServer\",\"fields\":[]}}",
                    "{\"kind\":\"record\",\"value\":{\"type_id\":\"echo_agent::plugin::prepared::PluginWiringResult\",\"fields\":[{\"name\":\"lsp_discovered\",\"value\":{\"kind\":\"null\"}}]}}",
                    "{\"kind\":\"record\",\"value\":{\"type_id\":\"echo_core::plugin::registry::ResolvedComponents\",\"fields\":[{\"name\":\"lsp_config_file\",\"value\":{\"kind\":\"null\"}}]}}")) {
                var request = JsonSupport.MAPPER.createObjectNode().put("operation", "retirement.probe").put("signature_digest", "sha256:" + "0".repeat(64));
                request.set("handle", agent.wire().toJson());
                request.putArray("arguments").add(JsonSupport.MAPPER.readTree(json));
                var error = assertThrows(Exception.class, () -> await(sdk.request("_echo_agent/facade/invoke", request)));
                var failure = EchoAgentException.from(error, "invoke");
                assertEquals("invalid_value", failure.code());
                assertTrue(failure.getMessage().contains("invalid facade argument"));
            }
            await(session.closeAsync());
            var other = await(agent.createSession());
            await(other.closeAsync());
            await(agent.closeAsync());
        }
    }
}
