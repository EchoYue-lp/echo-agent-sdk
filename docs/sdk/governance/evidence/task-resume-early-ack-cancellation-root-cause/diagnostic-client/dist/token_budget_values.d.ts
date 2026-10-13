/** Local token budget policy values; model execution remains Rust-owned. */
export type TokenAllocation = Readonly<{
    systemFits: boolean;
    toolDefsFit: boolean;
    conversationFits: boolean;
    outputFits: boolean;
    conversationExcess: number;
    usagePct: number;
}> & {
    ok(): boolean;
    needsCompression(): boolean;
};
export type BudgetReport = Readonly<{
    totalWindow: number;
    systemPrompt: number;
    systemPromptBudget: number;
    toolDefinitions: number;
    toolDefinitionsBudget: number;
    conversation: number;
    conversationBudget: number;
    estimatedOutput: number;
    outputBudget: number;
    usagePct: number;
    needsCompression: boolean;
}>;
export declare class TokenBudget {
    readonly totalWindow: number;
    private readonly systemPct;
    private readonly toolPct;
    private readonly outputPct;
    private readonly safetyPct;
    private constructor();
    static new(totalWindow: number): TokenBudget;
    static default(): TokenBudget;
    withAllocations(systemPct: number, toolPct: number, outputPct: number, safetyPct: number): TokenBudget;
    systemPromptBudget(): number;
    toolDefinitionsBudget(): number;
    outputBudget(): number;
    safetyBudget(): number;
    conversationBudget(): number;
    allocate(systemSize: number, toolDefsSize: number, conversationSize: number): TokenAllocation;
    report(systemSize: number, toolDefsSize: number, conversationSize: number, estimatedOutput: number): BudgetReport;
}
export declare class TokenBudgetConfig {
    readonly totalWindow: number | undefined;
    readonly systemPct: number;
    readonly toolPct: number;
    readonly outputPct: number;
    readonly safetyPct: number;
    readonly enabled: boolean;
    private constructor();
    static enabled(): TokenBudgetConfig;
    static disabled(): TokenBudgetConfig;
    withTotalWindow(window: number): TokenBudgetConfig;
    build(fallbackWindow: number): TokenBudget;
}
/** Timeout policy in milliseconds; zero disables a boundary like Rust Duration. */
export declare class LlmTimeouts {
    private readonly request;
    private readonly firstChunk;
    private readonly idle;
    private readonly overall;
    private constructor();
    static default(): LlmTimeouts;
    requestTimeout(): number | undefined;
    firstChunkTimeout(): number | undefined;
    idleTimeout(): number | undefined;
    overallTimeout(): number | undefined;
    withRequestTimeout(timeoutMs: number): LlmTimeouts;
    withoutRequestTimeout(): LlmTimeouts;
    withFirstChunkTimeout(timeoutMs: number): LlmTimeouts;
    withoutFirstChunkTimeout(): LlmTimeouts;
    withIdleTimeout(timeoutMs: number): LlmTimeouts;
    withoutIdleTimeout(): LlmTimeouts;
    withOverallTimeout(timeoutMs: number): LlmTimeouts;
    withoutOverallTimeout(): LlmTimeouts;
}
