package com.echoagent.sdk;

import java.util.List;
import java.util.Map;
import java.nio.file.Path;
import java.util.concurrent.TimeUnit;
import java.util.Base64;
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

    private static String encodeWindowsUnits(int[] units) {
        // Write little-endian units directly; String's charset encoder can replace isolated surrogates.
        var bytes = new byte[units.length * 2];
        for (int index = 0; index < units.length; index++) {
            bytes[index * 2] = (byte) units[index];
            bytes[index * 2 + 1] = (byte) (units[index] >>> 8);
        }
        return Base64.getEncoder().withoutPadding().encodeToString(bytes);
    }

    private static int[] appendWindowsChild(int[] units, String child) {
        int[] result = java.util.Arrays.copyOf(units, units.length + child.length() + 1);
        result[units.length] = 0x005C;
        for (int index = 0; index < child.length(); index++) result[units.length + 1 + index] = child.charAt(index);
        return result;
    }

    @Test
    public void windowsNativePluginRegistryPathRoundTrip() throws Exception {
        boolean windows = System.getProperty("os.name").toLowerCase(java.util.Locale.ROOT).startsWith("windows");
        String host = System.getenv("ECHO_AGENT_SDK_HOST");
        String config = System.getenv("ECHO_AGENT_SDK_CONFIG");
        if ("1".equals(System.getenv("ECHO_AGENT_SDK_REQUIRE_WINDOWS_NATIVE"))) {
            assertTrue(windows && host != null && !host.isBlank() && config != null && !config.isBlank(), "required Windows native Host/config/platform missing");
        }
        assumeTrue(windows && host != null && !host.isBlank(), "Windows native Host required");
        Path work = java.nio.file.Files.createTempDirectory(Path.of(System.getenv().getOrDefault("RUNNER_TEMP", System.getProperty("java.io.tmpdir"))), "sdk-java-windows-path-");
        var configuration = JsonSupport.MAPPER.readTree(Path.of(config).toFile());
        ((com.fasterxml.jackson.databind.node.ObjectNode) configuration.path("sdk_profile")).put("state_root", work.resolve("state").toString());
        var agentConfiguration = (com.fasterxml.jackson.databind.node.ObjectNode) configuration.path("default_agent").path("agent");
        if (agentConfiguration.has("memory_path")) agentConfiguration.put("memory_path", work.resolve("memory.jsonl").toString());
        Path configPath = work.resolve("host.json");
        JsonSupport.MAPPER.writeValue(configPath.toFile(), configuration);
        try (EchoAgentClient sdk = EchoAgentClient.spawn(host, List.of("--config", configPath.toString()), Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"), Map.of()).toCompletableFuture().get(10, TimeUnit.SECONDS)) {
            var agent = sdk.createAgent().toCompletableFuture().get(10, TimeUnit.SECONDS);
            var session = agent.createSession(work.toString()).toCompletableFuture().get(10, TimeUnit.SECONDS);
            int[] units = {0x0043, 0x003A, 0x005C, 0x4E2D, 0x6587, 0xD83D, 0xDE42, 0x002D, 0xD800, 0x0041, 0xDC00};
            var path = JsonSupport.MAPPER.createObjectNode().put("kind", "path");
            path.putObject("value").put("encoding", "windows").put("utf16_base64", encodeWindowsUnits(units)).put("display", "Z:/display-诱饵");
            String operation = "echo_core::plugin::registry::PluginRegistry::with_paths";
            String dataDir = "echo_core::plugin::registry::PluginRegistry::data_dir_for";
            // A mutable list carries the explicit null project_root.
            var arguments = new java.util.ArrayList<Object>();
            arguments.add(path); arguments.add(path); arguments.add(null);
            var registry = session.invoke(operation, arguments).toCompletableFuture().get(10, TimeUnit.SECONDS);
            var first = session.invoke(dataDir, List.of(WireHandle.fromJson(registry), "child-A")).toCompletableFuture().get(10, TimeUnit.SECONDS);
            int[] childA = appendWindowsChild(units, "child-A");
            assertEquals("path", first.path("kind").asText());
            assertEquals("windows", first.path("value").path("encoding").asText());
            assertEquals(encodeWindowsUnits(childA), first.path("value").path("utf16_base64").asText());
            assertNotEquals(path.path("value").path("display"), first.path("value").path("display"));
            arguments.set(1, first);
            var secondRegistry = session.invoke(operation, arguments).toCompletableFuture().get(10, TimeUnit.SECONDS);
            var second = session.invoke(dataDir, List.of(WireHandle.fromJson(secondRegistry), "child-B")).toCompletableFuture().get(10, TimeUnit.SECONDS);
            assertEquals("path", second.path("kind").asText());
            assertEquals("windows", second.path("value").path("encoding").asText());
            assertEquals(encodeWindowsUnits(appendWindowsChild(childA, "child-B")), second.path("value").path("utf16_base64").asText());
            var proof = JsonSupport.MAPPER.createObjectNode().put("platform", System.getProperty("os.name")).put("native_calls", 4);
            proof.set("first", first); proof.set("second", second);
            proof.set("input_units", JsonSupport.MAPPER.valueToTree(units));
            JsonSupport.MAPPER.writeValue(work.resolve("windows-path-proof.json").toFile(), proof);
            System.out.println("Windows native path evidence " + proof);
            System.out.println("Windows native path consumer: 1 passed, 0 failed, 0 skipped; 4 native calls");
            session.closeAsync().toCompletableFuture().get(10, TimeUnit.SECONDS);
            agent.closeAsync().toCompletableFuture().get(10, TimeUnit.SECONDS);
        }
    }
}
