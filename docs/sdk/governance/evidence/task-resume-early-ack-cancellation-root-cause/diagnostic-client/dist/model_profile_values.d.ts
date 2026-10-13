import { ProviderCapabilities } from "./provider_capabilities_values.js";
import { ThinkingLevel } from "./thinking.js";
import { ThinkingProtocol } from "./thinking_protocol_values.js";
export type ModelProfile = Readonly<{
    provider: string;
    modelName: string;
    capabilities: ProviderCapabilities;
    supportsReasoning: boolean;
    thinkingProtocol: ThinkingProtocol;
    thinkingLevels: readonly ThinkingLevel[];
    supportsImages: boolean;
    supportsTools: boolean;
    maxOutputTokens?: number;
    supportsStreaming: boolean;
    supportsParallelToolCalls: boolean;
    supportsToolChoiceNone: boolean;
    contextWindow?: number;
    excludedTools: readonly string[];
    promptSuffix?: string;
    tokenizerName?: string;
}>;
export type ModelProfileOverride = Readonly<{
    supportsParallelToolCalls?: boolean;
    supportsToolChoiceNone?: boolean;
    supportsStructuredOutput?: boolean;
    contextWindow?: number;
    excludedTools: readonly string[];
    promptSuffix?: string;
}>;
type ModelProfileOverrideInput = Omit<ModelProfileOverride, "excludedTools"> & {
    excludedTools?: readonly string[];
};
export declare function modelProfileOverride(input?: ModelProfileOverrideInput): ModelProfileOverride;
/** Infer the Rust model context-window table without contacting a provider. */
export declare function inferContextWindow(provider: string, modelName: string): number | undefined;
/** Rust-backed model policy projection; transport and execution remain host-owned. */
export declare const ModelProfile: Readonly<{
    "new"(modelName: string, provider: string, capabilities: ProviderCapabilities): ModelProfile;
    fromProviderName(modelName: string, provider: string): ModelProfile;
}>;
export declare class ModelProfileResolver {
    private readonly providerDefaults;
    private readonly exactModels;
    static new(): ModelProfileResolver;
    registerProviderDefault(provider: string, profile: ModelProfileOverride): ModelProfileResolver;
    registerExact(provider: string, model: string, profile: ModelProfileOverride): ModelProfileResolver;
    resolve(provider: string, model: string, capabilities: ProviderCapabilities): ModelProfile;
}
export {};
