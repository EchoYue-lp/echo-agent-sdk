/** Observed Subagent isolation value without owning isolation execution. */
export class ObservedIsolation {
    value;
    constructor(value) {
        this.value = value;
        Object.freeze(this);
    }
    static new(value) {
        if (typeof value !== "string")
            throw new TypeError("observed isolation must be text");
        const trimmed = value.trim();
        return new ObservedIsolation(trimmed ? Array.from(trimmed).slice(0, 512).join("") : "unknown");
    }
    static default() {
        return new ObservedIsolation("unknown");
    }
    asStr() {
        return this.value;
    }
}
//# sourceMappingURL=observed_isolation_values.js.map