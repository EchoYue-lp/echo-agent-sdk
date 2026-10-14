const MAX_U32 = 0xffffffffn;
const sat = (value) => value > MAX_U32 ? MAX_U32 : value;
export class Usage {
    input;
    constructor(input = {}) {
        this.input = input;
        validateUsage(input);
    }
    cachedPromptTokens() {
        return this.input.promptTokensDetails?.cachedTokens
            ?? this.input.inputTokensDetails?.cachedTokens
            ?? this.input.cacheReadInputTokens
            ?? this.input.promptCacheHitTokens
            ?? 0n;
    }
    cacheCreationPromptTokens() {
        return this.input.promptTokensDetails?.cacheWriteTokens
            ?? this.input.inputTokensDetails?.cacheWriteTokens
            ?? this.input.cacheCreationInputTokens
            ?? 0n;
    }
    effectivePromptTokens() {
        const prompt = this.input.promptTokens ?? 0n;
        if (this.input.cacheReadInputTokens != null || this.input.cacheCreationInputTokens != null) {
            return sat(prompt + this.cachedPromptTokens() + this.cacheCreationPromptTokens());
        }
        return prompt;
    }
    effectiveTotalTokens() {
        const completion = this.input.completionTokens ?? 0n;
        if (this.input.cacheReadInputTokens != null || this.input.cacheCreationInputTokens != null) {
            return sat(this.effectivePromptTokens() + completion);
        }
        return this.input.totalTokens ?? sat(this.effectivePromptTokens() + completion);
    }
    cacheHitRate() {
        const total = this.effectivePromptTokens();
        return total === 0n ? undefined : Number(this.cachedPromptTokens()) / Number(total);
    }
}
function validateUsage(input) {
    const values = [
        input.promptTokens, input.completionTokens, input.totalTokens,
        input.cacheCreationInputTokens, input.cacheReadInputTokens,
        input.promptCacheHitTokens, input.promptCacheMissTokens,
        input.promptTokensDetails?.cachedTokens, input.promptTokensDetails?.cacheWriteTokens,
        input.promptTokensDetails?.reasoningTokens, input.inputTokensDetails?.cachedTokens,
        input.inputTokensDetails?.cacheWriteTokens, input.inputTokensDetails?.reasoningTokens,
        input.outputTokensDetails?.cachedTokens, input.outputTokensDetails?.cacheWriteTokens,
        input.outputTokensDetails?.reasoningTokens,
    ];
    if (values.some((value) => value != null && (typeof value !== "bigint" || value < 0n || value > MAX_U32))) {
        throw new RangeError("usage token values must fit u32");
    }
}
//# sourceMappingURL=usage_values.js.map