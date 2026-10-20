package com.echoagent.sdk;

import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.util.HexFormat;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import static org.junit.jupiter.api.Assertions.*;

class FileSha256Test {
    @TempDir Path work;

    @Test void exactEmptyUnicodeAndChunkBoundary() throws Exception {
        byte[] tail = "中文🙂".getBytes(StandardCharsets.UTF_8);
        byte[] boundary = new byte[1024 * 1024 + tail.length];
        java.util.Arrays.fill(boundary, 0, 1024 * 1024, (byte) 'a');
        System.arraycopy(tail, 0, boundary, 1024 * 1024, tail.length);
        for (byte[] bytes : new byte[][] {new byte[0], "中文😀\n".getBytes(StandardCharsets.UTF_8), boundary}) {
            Path file = work.resolve("input.bin");
            Files.write(file, bytes);
            assertEquals(HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256").digest(bytes)), FileSha256.sha256(file));
        }
    }

    @Test void sparseAboveWholeFileLimit() throws Exception {
        Path path = work.resolve("large.bin");
        long size = (1L << 31) + 1;
        try (var file = new RandomAccessFile(path.toFile(), "rw")) {
            file.setLength(size);
            file.seek(0);
            file.write("start 中文😀\n".getBytes(StandardCharsets.UTF_8));
            file.seek(size - 3);
            file.write("END".getBytes(StandardCharsets.UTF_8));
        }
        assertEquals("05aa6dd2c62008228e2cb1ab20553b85be8565f3ddf82f0527661905c4015750", FileSha256.sha256(path));
    }

    @Test void missingPathAndDirectoryErrorsRemainVisible() {
        assertThrows(NoSuchFileException.class, () -> FileSha256.sha256(work.resolve("missing")));
        assertThrows(IOException.class, () -> FileSha256.sha256(work));
    }
}
