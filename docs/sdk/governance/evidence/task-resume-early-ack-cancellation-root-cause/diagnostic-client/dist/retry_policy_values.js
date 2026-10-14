/** Retry policy configuration and deterministic backoff projection. */
export class RetryPolicy {
    maxRetries;
    baseDelayMs;
    maxDelayMs;
    jitterEnabled;
    constructor(maxRetries, baseDelayMs, maxDelayMs, jitterEnabled) {
        this.maxRetries = maxRetries;
        this.baseDelayMs = baseDelayMs;
        this.maxDelayMs = maxDelayMs;
        this.jitterEnabled = jitterEnabled;
    }
    static new(maxRetries, baseDelayMs) {
        if (!Number.isSafeInteger(maxRetries) || maxRetries < 0 || !Number.isFinite(baseDelayMs) || baseDelayMs < 0) {
            throw new RangeError("retry policy values must be non-negative");
        }
        return new RetryPolicy(maxRetries, baseDelayMs, 60_000, false);
    }
    static default() { return new RetryPolicy(3, 500, 30_000, true); }
    static noRetry() { return new RetryPolicy(0, 0, 0, false); }
    maxDelay(delayMs) {
        if (!Number.isFinite(delayMs) || delayMs < 0)
            throw new RangeError("retry max delay must be non-negative");
        return new RetryPolicy(this.maxRetries, this.baseDelayMs, delayMs, this.jitterEnabled);
    }
    jitter(enabled) { return new RetryPolicy(this.maxRetries, this.baseDelayMs, this.maxDelayMs, enabled); }
    delayFor(attempt) {
        if (!Number.isSafeInteger(attempt) || attempt < 0)
            throw new RangeError("retry attempt must be non-negative");
        if (attempt === 0)
            return 0;
        const exponent = Math.min(attempt - 1, 10);
        const capped = Math.min(this.baseDelayMs * 2 ** exponent, this.maxDelayMs);
        return this.jitterEnabled && capped > 0 ? Math.floor(Math.random() * (capped + 1)) : capped;
    }
}
//# sourceMappingURL=retry_policy_values.js.map