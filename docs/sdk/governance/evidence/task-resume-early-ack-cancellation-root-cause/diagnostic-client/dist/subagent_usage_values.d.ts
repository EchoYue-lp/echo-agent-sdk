/** Cumulative Subagent LLM usage values without owning provider execution. */
export declare class LlmUsageStats {
    model: string;
    promptTokens: bigint;
    completionTokens: bigint;
    totalTokens: bigint;
    cachedPromptTokens: bigint;
    cacheCreationPromptTokens: bigint;
    usageReported: boolean;
    callCount: bigint;
    record(model: string, promptTokens: bigint, completionTokens: bigint, totalTokens: bigint, cachedPromptTokens: bigint, cacheCreationPromptTokens: bigint, usageReported: boolean): void;
    toPayload(sessionId: string): Record<string, string | boolean>;
}
