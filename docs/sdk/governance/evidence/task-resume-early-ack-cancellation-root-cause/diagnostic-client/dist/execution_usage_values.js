/** Return duration in milliseconds, matching Rust's absent-to-zero helper. */
export function executionUsageDurationMillis(usage) {
    return BigInt(usage.duration_ms ?? "0");
}
//# sourceMappingURL=execution_usage_values.js.map