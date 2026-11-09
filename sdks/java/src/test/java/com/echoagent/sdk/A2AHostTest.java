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
    private static void fixture(Path work, boolean verify) throws Exception {
        var command = new java.util.ArrayList<>(List.of("python3", "-B", Path.of("../shared/testing/retirement_plugin_fixture.py").toAbsolutePath().toString(), "--work", work.toString()));
        if (verify) command.add("--verify");
        var output = work.resolve(verify ? "verify.out" : "fixture.out");
        var error = work.resolve(verify ? "verify.err" : "fixture.err");
        var process = new ProcessBuilder(command).redirectOutput(output.toFile()).redirectError(error.toFile()).start();
        try {
            assertTrue(process.waitFor(10, TimeUnit.SECONDS));
            assertEquals(0, process.exitValue(), Files.readString(error));
        } finally {
            if (process.isAlive()) { process.destroyForcibly(); assertTrue(process.waitFor(5, TimeUnit.SECONDS)); }
        }
    }

    @Test void refusalLayersPreserveRealPluginAndFamilyOperations() throws Exception {
        String host = host();
        Path work = Files.createTempDirectory("sdk-retired-typed-java-");
        fixture(work, false);
        var plugin = JsonSupport.MAPPER.readTree(Files.readString(work.resolve("fixture.out"))).path("root").asText();
        var retired = JsonSupport.MAPPER.readTree(Files.readString(Path.of("../shared/testing/a2a_lsp_retirement_contract.json")));
        var catalog = JsonSupport.MAPPER.readTree(Files.readString(Path.of("../shared/facade-operation-catalog.json")));
        try (var sdk = await(EchoAgentClient.spawn("python3", List.of(Path.of("../python/tests/protocol_host_proxy.py").toAbsolutePath().toString(), "--host", host, "--work", work.toString(), "--mode", "normal"), null, null, Map.of()))) {
            var agent = await(sdk.createAgent());
            var session = await(agent.createSession());
            assertEquals("echo-agent", await(session.invoke("echo_core::agent::Agent::name", List.of())).asText());
            String operation = "echo_core::plugin::registry::PluginRegistry::validate_plugin_dir";
            com.fasterxml.jackson.databind.JsonNode route = null;
            for (var item : catalog.path("routes")) if (operation.equals(item.path("operation").asText())) route = item;
            assertNotNull(route);
            var request = JsonSupport.MAPPER.createObjectNode().put("operation", operation).put("signature_digest", route.path("signature_digests").get(0).asText());
            request.set("handle", agent.wire().toJson());
            var sessionArgument = JsonSupport.MAPPER.createObjectNode().put("kind", "handle");
            sessionArgument.set("value", session.wire().toJson());
            var path = JsonSupport.MAPPER.createObjectNode().put("kind", "path");
            path.putObject("value").put("encoding", "utf8").put("path", plugin);
            request.putArray("arguments").add(sessionArgument).add(path);
            await(sdk.request("_echo_agent/facade/invoke", request));
            assertEquals("中😀 [REDACTED:EMAIL] 尾", await(sdk.call("content-guard.redact_exact", session.wire(), List.of("中😀 alice@example.com 尾"))).asText());
            for (var family : retired.path("families")) {
                com.fasterxml.jackson.databind.JsonNode old = null;
                for (var item : retired.path("routes")) if (("family:" + family.path("family").asText()).equals(item.path("route").asText())) old = item;
                assertNotNull(old);
                for (var signature : old.path("operation_signatures")) {
                    var params = JsonSupport.MAPPER.createObjectNode().put("operation", signature.path("operation").asText()).put("signature_digest", signature.path("signature_digests").get(0).asText());
                    params.set("handle", session.wire().toJson()); params.putArray("arguments");
                    assertThrows(Exception.class, () -> await(sdk.request(family.path("methods").get(0).asText(), params)));
                }
            }
            for (var old : retired.path("routes")) if (!old.path("operation").isNull()) {
                var params = JsonSupport.MAPPER.createObjectNode().put("operation", old.path("operation").asText()).put("signature_digest", old.path("signature_digests").get(0).asText());
                params.set("handle", agent.wire().toJson()); params.putArray("arguments").add(sessionArgument);
                assertThrows(Exception.class, () -> await(sdk.request("_echo_agent/facade/invoke", params)));
            }
            var values = new java.util.ArrayList<com.fasterxml.jackson.databind.JsonNode>();
            for (String json : List.of(
                    "{\"kind\":\"variant\",\"value\":{\"type_id\":\"echo_core::plugin::capability::PluginCapability\",\"variant\":\"LspServer\",\"fields\":[]}}",
                    "{\"kind\":\"record\",\"value\":{\"type_id\":\"echo_agent::plugin::prepared::PluginWiringResult\",\"fields\":[{\"name\":\"lsp_discovered\",\"value\":{\"kind\":\"null\"}}]}}",
                    "{\"kind\":\"record\",\"value\":{\"type_id\":\"echo_core::plugin::registry::ResolvedComponents\",\"fields\":[{\"name\":\"lsp_config_file\",\"value\":{\"kind\":\"null\"}}]}}",
                    "{\"kind\":\"variant\",\"value\":{\"type_id\":\"echo_core::plugin::capability::PluginCapability\",\"variant\":\"McpServer\",\"fields\":[]}}",
                    "{\"kind\":\"record\",\"value\":{\"type_id\":\"echo_agent::plugin::prepared::PluginWiringResult\",\"fields\":[{\"name\":\"diagnostics\",\"value\":{\"kind\":\"list\",\"value\":[]}}]}}",
                    "{\"kind\":\"record\",\"value\":{\"type_id\":\"echo_core::plugin::registry::ResolvedComponents\",\"fields\":[{\"name\":\"diagnostics\",\"value\":{\"kind\":\"list\",\"value\":[]}}]}}")) values.add(JsonSupport.MAPPER.readTree(json));
            for (var value : values) {
                var params = JsonSupport.MAPPER.createObjectNode().put("operation", "retirement.probe").put("signature_digest", "sha256:" + "0".repeat(64));
                params.set("handle", agent.wire().toJson()); params.putArray("arguments").add(value);
                assertThrows(Exception.class, () -> await(sdk.request("_echo_agent/facade/invoke", params)));
            }
            assertEquals("echo-agent", await(session.invoke("echo_core::agent::Agent::name", List.of())).asText());
            await(session.closeAsync()); await(agent.closeAsync());
        } finally {
            var proof = JsonSupport.MAPPER.createObjectNode().put("host", host).put("host_sha256", FileSha256.sha256(Path.of(host))).put("proxy_mode", "normal").put("qualification", "actual preserved Host; no Frame source read or Cargo");
            Files.writeString(work.resolve("host-used.json"), proof.toString());
            System.out.println("retirement raw frame evidence: " + work);
        }
        fixture(work, true);
    }

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
