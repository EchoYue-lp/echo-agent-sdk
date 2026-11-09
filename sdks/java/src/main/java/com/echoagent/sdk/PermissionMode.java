package com.echoagent.sdk;

/** Permission mode identifiers and policy helpers projected without evaluation. */
public enum PermissionMode {
    DEFAULT("default"), PLAN("plan"), ACCEPT_EDITS("auto-edit"),
    BYPASS_PERMISSIONS("full-auto"), AUTO("auto"), BUBBLE("bubble"),
    DONT_ASK("dont-ask"), STRICT_CONFIRM("strict");

    private final String wireName;

    PermissionMode(String wireName) { this.wireName = wireName; }

    public static PermissionMode parse(String value) {
        if (value == null) throw new IllegalArgumentException("permission mode must be text");
        // 只解析Framework当前canonical ID；不归一成已退役的兼容输入。
        return switch (value) {
            case "default" -> DEFAULT;
            case "plan" -> PLAN;
            case "auto-edit" -> ACCEPT_EDITS;
            case "full-auto" -> BYPASS_PERMISSIONS;
            case "auto" -> AUTO;
            case "bubble" -> BUBBLE;
            case "dont-ask" -> DONT_ASK;
            case "strict" -> STRICT_CONFIRM;
            default -> throw new IllegalArgumentException("invalid permission mode '" + value + "'");
        };
    }

    public boolean allowsWrite() { return this == BYPASS_PERMISSIONS || this == ACCEPT_EDITS; }

    public boolean requiresInteraction() {
        return this != BYPASS_PERMISSIONS && this != AUTO && this != DONT_ASK && this != ACCEPT_EDITS;
    }

    public boolean usesClassifier() { return this == AUTO; }

    @Override
    public String toString() { return wireName; }
}
