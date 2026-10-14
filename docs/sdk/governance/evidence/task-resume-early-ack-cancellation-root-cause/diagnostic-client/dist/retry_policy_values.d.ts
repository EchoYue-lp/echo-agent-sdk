/** Retry policy configuration and deterministic backoff projection. */
export declare class RetryPolicy {
    readonly maxRetries: number;
    readonly baseDelayMs: number;
    readonly maxDelayMs: number;
    readonly jitterEnabled: boolean;
    private constructor();
    static new(maxRetries: number, baseDelayMs: number): RetryPolicy;
    static default(): RetryPolicy;
    static noRetry(): RetryPolicy;
    maxDelay(delayMs: number): RetryPolicy;
    jitter(enabled: boolean): RetryPolicy;
    delayFor(attempt: number): number;
}
