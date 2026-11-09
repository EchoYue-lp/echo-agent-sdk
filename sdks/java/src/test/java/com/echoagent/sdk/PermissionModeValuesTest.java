package com.echoagent.sdk;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import static org.junit.jupiter.api.Assertions.*;

/** 仅收敛输入；八模式策略与canonical输出保持，不调用Host。 */
class PermissionModeValuesTest {
    private record Case(String id, boolean write, boolean interaction, boolean classifier) {}
    private static final List<Case> MODES = List.of(
            new Case("default", false, true, false), new Case("plan", false, true, false),
            new Case("auto-edit", true, false, false), new Case("full-auto", true, false, false),
            new Case("auto", false, false, true), new Case("bubble", false, true, false),
            new Case("dont-ask", false, false, false), new Case("strict", false, true, false));

    @Test void exactCanonicalRoundtripAndPolicyArePreserved() {
        var checks = new ArrayList<Executable>();
        for (Case value : MODES) checks.add(() -> {
            var mode = PermissionMode.parse(value.id());
            assertEquals(value.id(), mode.toString());
            assertEquals(mode, PermissionMode.parse(mode.toString()));
            assertEquals(value.write(), mode.allowsWrite());
            assertEquals(value.interaction(), mode.requiresInteraction());
            assertEquals(value.classifier(), mode.usesClassifier());
        });
        assertEquals("default", PermissionMode.DEFAULT.toString());
        assertEquals(8, PermissionMode.values().length);
        assertAll(checks);
    }

    @Test void retiredAliasesCaseWhitespaceAndUnknownAreRejected() {
        var values = new ArrayList<>(List.of("ask", "accept-edits", "autoedit", "acceptedits", "fullauto", "bypass", "bypass-permissions", "bypasspermissions", "dontask", "strict-confirm", "strict-confirmation", "unknown", "", "__proto__", "toString"));
        for (Case mode : MODES) values.addAll(List.of(mode.id().toUpperCase(Locale.ROOT), " " + mode.id(), mode.id() + " ", "\t" + mode.id() + "\n"));
        var checks = new ArrayList<Executable>();
        for (String value : values) checks.add(() -> assertThrows(IllegalArgumentException.class, () -> PermissionMode.parse(value), value));
        assertAll(checks);
    }

    @Test void nullIsRejectedWithoutDefaulting() {
        assertThrows(IllegalArgumentException.class, () -> PermissionMode.parse(null));
    }
}
