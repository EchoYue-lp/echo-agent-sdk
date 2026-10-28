package com.echoagent.sdk;

import java.nio.file.Files;
import java.nio.file.Path;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class PackageResourcesTest {
    @Test void packagedCatalogUsesTheCanonicalResources() throws Exception {
        for (String name : java.util.List.of("facade-operation-catalog.json", "contract-digests.json")) {
            try (var resource = FacadeCatalog.class.getResourceAsStream("contracts/" + name)) {
                assertNotNull(resource, name);
                assertArrayEquals(Files.readAllBytes(Path.of("../shared", name)), resource.readAllBytes());
            }
        }
        var catalog = new FacadeCatalog(null, null);
        assertEquals("_echo_agent/memory/op", catalog.resolve("memory.store.put").method());
        assertFalse(catalog.operations().isEmpty());
    }

    @Test void isolatedJarLoadsCatalogWithoutSourceDirectories(@org.junit.jupiter.api.io.TempDir Path work) throws Exception {
        Path classes = Path.of(FacadeCatalog.class.getProtectionDomain().getCodeSource().getLocation().toURI());
        Path jar = work.resolve("echo-agent-sdk-source.jar");
        try (var output = new java.util.jar.JarOutputStream(Files.newOutputStream(jar));
             var files = Files.walk(classes)) {
            for (Path file : files.filter(Files::isRegularFile).toList()) {
                output.putNextEntry(new java.util.jar.JarEntry(classes.relativize(file).toString().replace('\\', '/')));
                Files.copy(file, output);
                output.closeEntry();
            }
        }
        var urls = new java.net.URL[] {
            jar.toUri().toURL(),
            com.fasterxml.jackson.databind.ObjectMapper.class.getProtectionDomain().getCodeSource().getLocation(),
            com.fasterxml.jackson.core.JsonFactory.class.getProtectionDomain().getCodeSource().getLocation(),
            com.fasterxml.jackson.annotation.JsonCreator.class.getProtectionDomain().getCodeSource().getLocation(),
        };
        // platform parent只含JDK；无法从本仓target/classes或shared借用资源。
        try (var isolated = new java.net.URLClassLoader(urls, ClassLoader.getPlatformClassLoader())) {
            Class<?> type = Class.forName("com.echoagent.sdk.FacadeCatalog", true, isolated);
            Object catalog = type.getConstructor(Path.class, Path.class).newInstance(null, null);
            Object operation = type.getMethod("resolve", String.class).invoke(catalog, "memory.store.put");
            assertEquals("_echo_agent/memory/op", operation.getClass().getMethod("method").invoke(operation));
            assertEquals(isolated, type.getClassLoader());
        }
    }
}
