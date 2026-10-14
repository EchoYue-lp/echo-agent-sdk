export type ProviderCapabilities = Readonly<{
    streamingToolCalls: boolean;
    namedSseEvents: boolean;
    reasoningContent: boolean;
    imageInput: boolean;
    systemAsTopLevel: boolean;
    ndjsonStreaming: boolean;
    toolSupport: boolean;
    structuredOutput: boolean;
    requiresVersionHeader: boolean;
    supportsParallelToolCalls: boolean;
    supportsToolChoiceNone: boolean;
    tokenizerName?: string;
}>;
declare function openaiCompatible(): ProviderCapabilities;
declare function anthropic(): ProviderCapabilities;
declare function ollama(): ProviderCapabilities;
export declare const ProviderCapabilities: Readonly<{
    openaiCompatible: typeof openaiCompatible;
    anthropic: typeof anthropic;
    ollama: typeof ollama;
    fromProviderName(name: string): ProviderCapabilities;
}>;
export {};
