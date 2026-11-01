package com.echoagent.sdk;

import org.junit.jupiter.api.Test;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertThrows;

class LspDiagnosticsHostTest {
    @Test
    void lspDiagnosticsIsAbsentFromCatalog() throws Exception {
        var catalog = new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"));
        assertThrows(IllegalArgumentException.class, () -> catalog.resolve("lsp.client.diagnostics"));
    }
}
