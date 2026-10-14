/** ACP runtime values projected without owning connection or ledger state. */
export const ConnectionMode = Object.freeze({
    Standard: "standard",
    Extended: "extended",
});
export const ExtensionSettlement = Object.freeze({
    Answered: "answered",
    TimedOut: "timed_out",
    Cancelled: "cancelled",
    Disconnected: "disconnected",
});
export function extensionSettlementIsAnswered(settlement) {
    if (!Object.values(ExtensionSettlement).includes(settlement)) {
        throw new TypeError("invalid extension settlement");
    }
    return settlement === ExtensionSettlement.Answered;
}
export function acpLedgerLimits(maxEvents, maxBytes) {
    for (const [name, value] of [["maxEvents", maxEvents], ["maxBytes", maxBytes]]) {
        if (typeof value !== "bigint" || value < 0n) {
            throw new RangeError(`${name} must be a non-negative bigint`);
        }
    }
    return Object.freeze({ maxEvents, maxBytes });
}
export function acpLedgerLimitsDefault() {
    return acpLedgerLimits(10000n, 8388608n);
}
export function acpAdapterConfigDefault(version = "0.2.0") {
    return Object.freeze({
        name: "echo-agent",
        title: "echo-agent",
        version,
        maxSessions: 128n,
        maxPromptChars: 1000000n,
        maxUpdateChars: 1000000n,
        maxUpdatesPerTurn: 10000n,
        maxTotalUpdateChars: 8000000n,
        maxExtensionConcurrency: 8n,
        shutdownTimeout: Object.freeze({ seconds: "5", nanos: 0 }),
    });
}
export function acpAdapterConfigValidate(config) {
    if (config === null || typeof config !== "object") {
        throw new TypeError("ACP adapter config must be an object");
    }
    if (!config.name.trim() || !config.title.trim() || !config.version.trim()) {
        throw new Error("ACP adapter name, title, and version must not be empty");
    }
    for (const [name, value] of [
        ["maxSessions", config.maxSessions],
        ["maxPromptChars", config.maxPromptChars],
        ["maxUpdateChars", config.maxUpdateChars],
        ["maxUpdatesPerTurn", config.maxUpdatesPerTurn],
        ["maxTotalUpdateChars", config.maxTotalUpdateChars],
        ["maxExtensionConcurrency", config.maxExtensionConcurrency],
    ]) {
        if (typeof value !== "bigint" || value <= 0n) {
            throw new Error("ACP adapter resource limits must be positive");
        }
    }
    if (!config.shutdownTimeout
        || typeof config.shutdownTimeout.seconds !== "string"
        || !/^\d+$/.test(config.shutdownTimeout.seconds)
        || !Number.isInteger(config.shutdownTimeout.nanos)
        || config.shutdownTimeout.nanos < 0
        || config.shutdownTimeout.nanos >= 1_000_000_000
        || (config.shutdownTimeout.seconds === "0" && config.shutdownTimeout.nanos === 0)) {
        throw new Error("ACP adapter shutdown timeout must be positive");
    }
}
export const ExtensionLeaseError = Object.freeze({
    AdmissionClosed: "extension admission is closed",
    ConcurrencyLimit: "extension concurrency limit reached",
    ExclusiveConflict: "extension is already executing an exclusive invocation",
});
export function extensionLeaseErrorAsStr(error) {
    if (!Object.values(ExtensionLeaseError).includes(error)) {
        throw new TypeError("invalid extension lease error");
    }
    return error;
}
//# sourceMappingURL=acp_values.js.map