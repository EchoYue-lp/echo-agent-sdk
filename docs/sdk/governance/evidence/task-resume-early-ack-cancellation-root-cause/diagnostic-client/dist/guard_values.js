export function guardDecisionPass() {
    return Object.freeze({ kind: "pass" });
}
export function guardDecisionBlock(reason) {
    if (typeof reason !== "string")
        throw new TypeError("block reason must be text");
    return Object.freeze({ kind: "block", reason });
}
export function guardDecisionWarn(reasons) {
    return Object.freeze({ kind: "warn", reasons: Object.freeze([...reasons]) });
}
export function guardDecisionTransform(content, reasons) {
    if (typeof content !== "string")
        throw new TypeError("transformed content must be text");
    return Object.freeze({ kind: "transform", content, reasons: Object.freeze([...reasons]) });
}
export function guardDecisionIsBlocked(decision) {
    return decision.kind === "block";
}
//# sourceMappingURL=guard_values.js.map