export function contentGuardPass() {
    return Object.freeze({ kind: "pass" });
}
export function contentGuardDetected(piiTypes) {
    return Object.freeze({ kind: "detected", piiTypes: Object.freeze([...piiTypes]) });
}
export function contentGuardRejected(piiTypes) {
    return Object.freeze({ kind: "rejected", piiTypes: Object.freeze([...piiTypes]) });
}
export function contentGuardRedacted(content) {
    if (typeof content !== "string")
        throw new TypeError("redacted content must be text");
    return Object.freeze({ kind: "redacted", content });
}
export function contentGuardResultIsRejected(result) {
    return result.kind === "rejected";
}
//# sourceMappingURL=content_guard_values.js.map