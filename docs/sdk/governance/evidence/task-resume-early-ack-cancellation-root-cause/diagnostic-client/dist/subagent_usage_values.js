/** Cumulative Subagent LLM usage values without owning provider execution. */
export class LlmUsageStats {
    model = "";
    promptTokens = 0n;
    completionTokens = 0n;
    totalTokens = 0n;
    cachedPromptTokens = 0n;
    cacheCreationPromptTokens = 0n;
    usageReported = false;
    callCount = 0n;
    record(model, promptTokens, completionTokens, totalTokens, cachedPromptTokens, cacheCreationPromptTokens, usageReported) {
        this.model = model;
        this.promptTokens += promptTokens;
        this.completionTokens += completionTokens;
        this.totalTokens += totalTokens;
        this.cachedPromptTokens += cachedPromptTokens;
        this.cacheCreationPromptTokens += cacheCreationPromptTokens;
        this.usageReported ||= usageReported;
        this.callCount += 1n;
    }
    toPayload(sessionId) {
        return {
            session_id: sessionId,
            model: this.model || "unknown",
            prompt_tokens: this.promptTokens.toString(),
            completion_tokens: this.completionTokens.toString(),
            total_tokens: this.totalTokens.toString(),
            cached_prompt_tokens: this.cachedPromptTokens.toString(),
            cache_creation_prompt_tokens: this.cacheCreationPromptTokens.toString(),
            usage_reported: this.usageReported,
            call_count: this.callCount.toString(),
        };
    }
}
//# sourceMappingURL=subagent_usage_values.js.map