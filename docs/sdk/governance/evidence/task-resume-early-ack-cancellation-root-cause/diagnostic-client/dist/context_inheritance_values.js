export function contextInheritanceSyncDefault() {
    return freezeContext(null, null, false, {});
}
export function contextInheritanceFreshDefault() {
    return contextInheritanceSyncDefault();
}
export function contextInheritanceForkDefault() {
    return freezeContext(null, 2n, true, {});
}
export function contextInheritanceTeammateDefault() {
    return freezeContext([], 2n, false, {});
}
export function contextInheritanceForMode(mode) {
    switch (mode) {
        case "sync": return contextInheritanceSyncDefault();
        case "fork": return contextInheritanceForkDefault();
        case "teammate":
        case "team": return contextInheritanceTeammateDefault();
    }
}
function freezeContext(inheritTools, inheritHistory, inheritMemory, injectMetadata) {
    if (inheritHistory !== null && (inheritHistory < 0n || typeof inheritHistory !== "bigint")) {
        throw new RangeError("inherit history must be a non-negative bigint");
    }
    return Object.freeze({
        inheritTools: inheritTools === null ? null : Object.freeze([...inheritTools]),
        inheritHistory,
        inheritMemory,
        injectMetadata: Object.freeze({ ...injectMetadata }),
    });
}
//# sourceMappingURL=context_inheritance_values.js.map