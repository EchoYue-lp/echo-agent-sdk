import { ThinkingLevel } from "./thinking.js";
export type ThinkingConfig = Readonly<{
    kind: "disabled";
}> | Readonly<{
    kind: "level";
    level: ThinkingLevel;
}> | Readonly<{
    kind: "budget_tokens";
    budgetTokens: number;
}>;
/** Provider-neutral thinking configuration and pure dialect projections. */
export declare const ThinkingConfig: Readonly<{
    Disabled: ThinkingConfig;
    Level(level: ThinkingLevel): ThinkingConfig;
    BudgetTokens(budgetTokens: number): ThinkingConfig;
    medium(): ThinkingConfig;
    parseSpec(spec: string): ThinkingConfig | undefined;
    toReasoningEffort(config: ThinkingConfig): string | undefined;
    toAnthropicEffort(config: ThinkingConfig): string | undefined;
    toAnthropicBudget(config: ThinkingConfig, maxTokens: number): number | undefined;
    toEnableThinking(config: ThinkingConfig): boolean;
    toGlmThinkingType(config: ThinkingConfig): "enabled" | "disabled";
    toGlmReasoningEffort(config: ThinkingConfig): string | undefined;
}>;
