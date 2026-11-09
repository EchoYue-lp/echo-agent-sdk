package com.echoagent.sdk;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
class A2AValueTest {
    @Test void retiredFamilyOperationsCannotResolve() throws Exception {
        var catalog = new FacadeCatalog(java.nio.file.Path.of("../shared/facade-operation-catalog.json"), java.nio.file.Path.of("../shared/contract-digests.json"));
        for (String operation : java.util.List.of("a2a.client.open", "lsp.manager.open", "lsp.client.hover"))
            assertThrows(IllegalArgumentException.class, () -> catalog.resolve(operation));
    }

    @Test
    void retiredA2AJwtMappingsStayOutsideExternalContract() throws Exception {
        var root = java.nio.file.Path.of("../..");
        var manifest = JsonSupport.MAPPER.readTree(java.nio.file.Files.readString(root.resolve("contracts/sdk/parity-manifest.json")));
        var accepted = JsonSupport.MAPPER.readTree(java.nio.file.Files.readString(root.resolve("contracts/sdk/accepted-external-contract.json")));
        var adr = java.nio.file.Files.readString(root.resolve("docs/adr/0042-sdk-a2a-lsp-retirement-boundary.md"));
        // 承接九项已批准退出负控；保留83退休family/catalog断言，不恢复旧构造。
        var retired = java.util.Map.ofEntries(
                java.util.Map.entry("echo_agent::a2a::auth::JwtClaims::subject", "method"),
                java.util.Map.entry("echo_agent::a2a::auth::JwtConfig", "struct"),
                java.util.Map.entry("echo_agent::a2a::auth::JwtConfig::disabled", "method"),
                java.util.Map.entry("echo_agent::a2a::auth::JwtConfig::hs256", "method"),
                java.util.Map.entry("echo_agent::a2a::auth::JwtConfig::impl<Debug>", "trait_impl"),
                java.util.Map.entry("echo_agent::a2a::auth::JwtConfig::is_enabled", "method"),
                java.util.Map.entry("echo_agent::a2a::auth::JwtConfig::rs256", "method"),
                java.util.Map.entry("echo_agent::a2a::auth::JwtConfig::with_audience", "method"),
                java.util.Map.entry("echo_agent::a2a::auth::JwtConfig::with_issuer", "method")
        );
        assertEquals(9, retired.size());
        for (var identity : retired.entrySet()) {
            var source = identity.getKey();
            var kind = identity.getValue();
            assertTrue(adr.contains("- Kind: " + kind + "\n- Source: " + source + "\n"), source);
            for (var entry : accepted.path("entries")) {
                for (var value : entry.path("source_paths")) {
                    assertTrue(!entry.path("kind").asText().equals(kind) || !value.asText().equals(source), source);
                }
            }
            for (var entry : manifest.path("entries")) {
                if (!entry.path("kind").asText().equals(kind)) continue;
                boolean matches = false;
                for (var value : entry.path("source_paths")) matches |= value.asText().equals(source);
                if (!matches) continue;
                assertEquals("host_or_rust_only", entry.path("sdk_scope").asText(), entry.path("path").asText());
                assertEquals("not_implemented", entry.path("languages").path("java").path("status").asText());
                assertEquals("sdk-parity/java/retired_a2a_lsp", entry.path("languages").path("java").path("contract_test").asText());
            }
        }
    }


}
