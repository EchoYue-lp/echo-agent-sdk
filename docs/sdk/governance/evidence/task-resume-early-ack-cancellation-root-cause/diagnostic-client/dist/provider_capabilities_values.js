function openaiCompatible() {
    return Object.freeze({ streamingToolCalls: true, namedSseEvents: false, reasoningContent: true, imageInput: true, systemAsTopLevel: false, ndjsonStreaming: false, toolSupport: true, structuredOutput: true, requiresVersionHeader: false, supportsParallelToolCalls: true, supportsToolChoiceNone: true });
}
function anthropic() {
    return Object.freeze({ streamingToolCalls: false, namedSseEvents: true, reasoningContent: false, imageInput: true, systemAsTopLevel: true, ndjsonStreaming: false, toolSupport: true, structuredOutput: false, requiresVersionHeader: true, supportsParallelToolCalls: true, supportsToolChoiceNone: false, tokenizerName: "claude" });
}
function ollama() {
    return Object.freeze({ streamingToolCalls: false, namedSseEvents: false, reasoningContent: false, imageInput: false, systemAsTopLevel: false, ndjsonStreaming: true, toolSupport: true, structuredOutput: false, requiresVersionHeader: false, supportsParallelToolCalls: false, supportsToolChoiceNone: false });
}
export const ProviderCapabilities = Object.freeze({
    openaiCompatible,
    anthropic,
    ollama,
    fromProviderName(name) {
        switch (name.toLowerCase()) {
            case "anthropic": return anthropic();
            case "ollama": return ollama();
            default: return openaiCompatible();
        }
    },
});
//# sourceMappingURL=provider_capabilities_values.js.map