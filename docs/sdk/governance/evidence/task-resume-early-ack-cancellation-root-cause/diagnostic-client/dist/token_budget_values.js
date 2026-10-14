function finiteFraction(value) {
    if (!Number.isFinite(value) || value < 0 || value > 1) {
        throw new RangeError("token budget allocation must be finite and in [0, 1]");
    }
    return value;
}
export class TokenBudget {
    totalWindow;
    systemPct;
    toolPct;
    outputPct;
    safetyPct;
    constructor(totalWindow, systemPct, toolPct, outputPct, safetyPct) {
        this.totalWindow = totalWindow;
        this.systemPct = systemPct;
        this.toolPct = toolPct;
        this.outputPct = outputPct;
        this.safetyPct = safetyPct;
    }
    static new(totalWindow) {
        if (!Number.isSafeInteger(totalWindow) || totalWindow <= 0) {
            throw new RangeError("token budget total window must be greater than zero");
        }
        return new TokenBudget(totalWindow, 0.1, 0.05, 0.1, 0.1);
    }
    static default() {
        return new TokenBudget(128_000, 0.1, 0.05, 0.1, 0.1);
    }
    withAllocations(systemPct, toolPct, outputPct, safetyPct) {
        const system = finiteFraction(systemPct);
        const tool = finiteFraction(toolPct);
        const output = finiteFraction(outputPct);
        const safety = finiteFraction(safetyPct);
        if (system + tool + output + safety > 1) {
            throw new RangeError("token budget allocations exceed 1.0");
        }
        return new TokenBudget(this.totalWindow, system, tool, output, safety);
    }
    systemPromptBudget() { return Math.round(this.totalWindow * this.systemPct); }
    toolDefinitionsBudget() { return Math.round(this.totalWindow * this.toolPct); }
    outputBudget() { return Math.round(this.totalWindow * this.outputPct); }
    safetyBudget() { return Math.round(this.totalWindow * this.safetyPct); }
    conversationBudget() {
        return Math.round(this.totalWindow * Math.max(0, 1 - this.systemPct - this.toolPct - this.outputPct - this.safetyPct));
    }
    allocate(systemSize, toolDefsSize, conversationSize) {
        const effective = Math.max(0, this.totalWindow - this.outputBudget() - this.safetyBudget() - systemSize - toolDefsSize);
        const allocation = {
            systemFits: systemSize <= this.systemPromptBudget(),
            toolDefsFit: toolDefsSize <= this.toolDefinitionsBudget(),
            conversationFits: conversationSize <= effective,
            outputFits: this.outputBudget() > 0,
            conversationExcess: Math.max(0, conversationSize - effective),
            usagePct: ((systemSize + toolDefsSize + conversationSize) / this.totalWindow) * 100,
        };
        allocation.ok = () => allocation.systemFits && allocation.toolDefsFit && allocation.conversationFits && allocation.outputFits;
        allocation.needsCompression = () => allocation.conversationExcess > 0;
        return Object.freeze(allocation);
    }
    report(systemSize, toolDefsSize, conversationSize, estimatedOutput) {
        const allocation = this.allocate(systemSize, toolDefsSize, conversationSize);
        return Object.freeze({
            totalWindow: this.totalWindow,
            systemPrompt: systemSize,
            systemPromptBudget: this.systemPromptBudget(),
            toolDefinitions: toolDefsSize,
            toolDefinitionsBudget: this.toolDefinitionsBudget(),
            conversation: conversationSize,
            conversationBudget: this.conversationBudget(),
            estimatedOutput,
            outputBudget: this.outputBudget(),
            usagePct: allocation.usagePct,
            needsCompression: allocation.needsCompression(),
        });
    }
}
export class TokenBudgetConfig {
    totalWindow;
    systemPct;
    toolPct;
    outputPct;
    safetyPct;
    enabled;
    constructor(totalWindow, systemPct, toolPct, outputPct, safetyPct, enabled) {
        this.totalWindow = totalWindow;
        this.systemPct = systemPct;
        this.toolPct = toolPct;
        this.outputPct = outputPct;
        this.safetyPct = safetyPct;
        this.enabled = enabled;
    }
    static enabled() { return new TokenBudgetConfig(undefined, 0.1, 0.05, 0.1, 0.1, true); }
    static disabled() { return new TokenBudgetConfig(undefined, 0.1, 0.05, 0.1, 0.1, false); }
    withTotalWindow(window) {
        if (!Number.isSafeInteger(window) || window <= 0)
            throw new RangeError("token budget total window must be greater than zero");
        return new TokenBudgetConfig(window, this.systemPct, this.toolPct, this.outputPct, this.safetyPct, this.enabled);
    }
    build(fallbackWindow) {
        return TokenBudget.new(this.totalWindow ?? fallbackWindow).withAllocations(this.systemPct, this.toolPct, this.outputPct, this.safetyPct);
    }
}
/** Timeout policy in milliseconds; zero disables a boundary like Rust Duration. */
export class LlmTimeouts {
    request;
    firstChunk;
    idle;
    overall;
    constructor(request, firstChunk, idle, overall) {
        this.request = request;
        this.firstChunk = firstChunk;
        this.idle = idle;
        this.overall = overall;
    }
    static default() { return new LlmTimeouts(60_000, 30_000, 30_000, undefined); }
    requestTimeout() { return this.request; }
    firstChunkTimeout() { return this.firstChunk; }
    idleTimeout() { return this.idle; }
    overallTimeout() { return this.overall; }
    withRequestTimeout(timeoutMs) { return new LlmTimeouts(timeoutMs > 0 ? timeoutMs : undefined, this.firstChunk, this.idle, this.overall); }
    withoutRequestTimeout() { return new LlmTimeouts(undefined, this.firstChunk, this.idle, this.overall); }
    withFirstChunkTimeout(timeoutMs) { return new LlmTimeouts(this.request, timeoutMs > 0 ? timeoutMs : undefined, this.idle, this.overall); }
    withoutFirstChunkTimeout() { return new LlmTimeouts(this.request, undefined, this.idle, this.overall); }
    withIdleTimeout(timeoutMs) { return new LlmTimeouts(this.request, this.firstChunk, timeoutMs > 0 ? timeoutMs : undefined, this.overall); }
    withoutIdleTimeout() { return new LlmTimeouts(this.request, this.firstChunk, undefined, this.overall); }
    withOverallTimeout(timeoutMs) { return new LlmTimeouts(this.request, this.firstChunk, this.idle, timeoutMs > 0 ? timeoutMs : undefined); }
    withoutOverallTimeout() { return new LlmTimeouts(this.request, this.firstChunk, this.idle, undefined); }
}
//# sourceMappingURL=token_budget_values.js.map