export function ruleBehaviorAllow() {
    return Object.freeze({ kind: "allow" });
}
export function ruleBehaviorDeny(reason) {
    if (typeof reason !== "string")
        throw new TypeError("deny reason must be text");
    return Object.freeze({ kind: "deny", reason });
}
export function ruleBehaviorAsk(suggestions) {
    return Object.freeze({ kind: "ask", suggestions: Object.freeze([...suggestions]) });
}
export function ruleBehaviorParse(value) {
    if (typeof value !== "string")
        throw new TypeError("rule behavior must be text");
    switch (value) {
        case "allow": return ruleBehaviorAllow();
        case "deny": return ruleBehaviorDeny("denied by rule");
        case "ask": return ruleBehaviorAsk(["allow", "deny"]);
        default: throw new Error(`unknown permission rule behavior: ${value}`);
    }
}
export function ruleBehaviorToDecision(behavior) {
    if (behavior.kind === "allow")
        return Object.freeze({ kind: "allow" });
    if (behavior.kind === "deny")
        return Object.freeze({ kind: "deny", reason: behavior.reason });
    return Object.freeze({ kind: "ask", suggestions: Object.freeze([...behavior.suggestions]) });
}
//# sourceMappingURL=permission_rule_behavior.js.map