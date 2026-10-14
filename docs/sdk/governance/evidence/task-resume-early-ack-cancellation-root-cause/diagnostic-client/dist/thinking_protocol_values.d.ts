/** Provider thinking wire protocols projected as local values. */
export declare const ThinkingProtocol: Readonly<{
    readonly None: "none";
    readonly ModelManaged: "model_managed";
    readonly OpenaiReasoningEffort: "openai_reasoning_effort";
    readonly DeepseekReasoningEffort: "deepseek_reasoning_effort";
    readonly AnthropicEffort: "anthropic_effort";
    readonly AnthropicThinkingBudget: "anthropic_thinking_budget";
    readonly AnthropicAdaptive: "anthropic_adaptive";
    readonly EnableThinkingFlag: "enable_thinking_flag";
    readonly ThinkingType: "thinking_type";
    readonly GlmReasoningEffort: "glm_reasoning_effort";
    readonly OllamaThink: "ollama_think";
}>;
export type ThinkingProtocol = (typeof ThinkingProtocol)[keyof typeof ThinkingProtocol];
export declare function thinkingProtocolEmitsField(protocol: ThinkingProtocol): boolean;
