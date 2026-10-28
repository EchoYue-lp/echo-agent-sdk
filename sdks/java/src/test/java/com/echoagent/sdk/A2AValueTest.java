package com.echoagent.sdk;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
class A2AValueTest {
    @Test void retiredFamilyOperationsCannotResolve() throws Exception {
        var catalog = new FacadeCatalog(java.nio.file.Path.of("../shared/facade-operation-catalog.json"), java.nio.file.Path.of("../shared/contract-digests.json"));
        for (String operation : java.util.List.of("a2a.client.open", "lsp.manager.open", "lsp.client.hover"))
            assertThrows(IllegalArgumentException.class, () -> catalog.resolve(operation));
    }
}
