/** Provider-normalized LLM usage values without owning provider execution. */
export interface TokenUsageDetails {
    readonly cachedTokens?: bigint | null;
    readonly cacheWriteTokens?: bigint | null;
    readonly reasoningTokens?: bigint | null;
}
export interface UsageInput {
    readonly promptTokens?: bigint | null;
    readonly completionTokens?: bigint | null;
    readonly totalTokens?: bigint | null;
    readonly promptTokensDetails?: TokenUsageDetails | null;
    readonly inputTokensDetails?: TokenUsageDetails | null;
    readonly outputTokensDetails?: TokenUsageDetails | null;
    readonly cacheCreationInputTokens?: bigint | null;
    readonly cacheReadInputTokens?: bigint | null;
    readonly promptCacheHitTokens?: bigint | null;
    readonly promptCacheMissTokens?: bigint | null;
}
export declare class Usage {
    readonly input: Readonly<UsageInput>;
    constructor(input?: Readonly<UsageInput>);
    cachedPromptTokens(): bigint;
    cacheCreationPromptTokens(): bigint;
    effectivePromptTokens(): bigint;
    effectiveTotalTokens(): bigint;
    cacheHitRate(): number | undefined;
}
