package com.echoagent.sdk;

import org.junit.jupiter.api.Test;
import java.nio.file.Path;
import static org.junit.jupiter.api.Assertions.assertThrows;

class A2AHostTest {
    @Test
    void retiredA2AOperationIsAbsentFromCatalog() throws Exception {
        var catalog = new FacadeCatalog(Path.of("../shared/facade-operation-catalog.json"), Path.of("../shared/contract-digests.json"));
        assertThrows(IllegalArgumentException.class, () -> catalog.resolve("a2a.client.open"));
    }
}
