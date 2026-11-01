package com.echoagent.sdk;

import org.junit.jupiter.api.Test;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertThrows;

class LspHostTest {
    @Test
    void lspOperationIsAbsentFromCatalog() throws Exception {
        var catalog = new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"));
        assertThrows(IllegalArgumentException.class, () -> catalog.resolve("lsp.manager.open"));
    }
}
