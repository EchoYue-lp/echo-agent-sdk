import { ProviderCapabilities } from "./provider_capabilities_values.js";
import { ThinkingProtocol } from "./thinking_protocol_values.js";
import { resolveThinkingProfile } from "./thinking_profile_values.js";
function freezeOverride(input = {}) {
    return Object.freeze({
        ...input,
        excludedTools: Object.freeze([...new Set(input.excludedTools ?? [])]),
    });
}
export function modelProfileOverride(input = {}) {
    return freezeOverride(input);
}
function cloneCapabilities(capabilities, structuredOutput) {
    return Object.freeze({ ...capabilities, structuredOutput });
}
/** Infer the Rust model context-window table without contacting a provider. */
export function inferContextWindow(provider, modelName) {
    const providerLower = provider.trim().toLowerCase();
    const model = modelName.toLowerCase();
    if ((providerLower === "openai" || providerLower === "azure-openai") && model.startsWith("gpt-5.6"))
        return 1_050_000;
    if ((providerLower === "anthropic" && (model.startsWith("claude-fable-5") || model.startsWith("claude-opus-4-8") || model.startsWith("claude-sonnet-5")))
        || (providerLower === "deepseek" && model.startsWith("deepseek-v4"))
        || (["dashscope", "qwen", "aliyun", "alibaba"].includes(providerLower) && (model.startsWith("qwen3.7-max") || model.startsWith("qwen3.7-plus")))
        || (providerLower === "zhipu" && model.startsWith("glm-5.2")))
        return 1_000_000;
    if (providerLower === "moonshot" && (model.startsWith("kimi-k2.7") || model.startsWith("kimi-k2.6")))
        return 256_000;
    return undefined;
}
function freezeProfile(profile) {
    return Object.freeze({
        ...profile,
        thinkingLevels: Object.freeze([...profile.thinkingLevels]),
        excludedTools: Object.freeze([...profile.excludedTools]),
    });
}
function createProfile(modelName, provider, capabilities) {
    const lower = modelName.toLowerCase();
    const capabilitiesSnapshot = Object.freeze({ ...capabilities });
    const thinking = resolveThinkingProfile(provider, modelName);
    const maxOutputTokens = lower.includes("qwen3-235b")
        ? 131_072
        : lower.startsWith("gpt-5") || lower.startsWith("o3") || lower.startsWith("o4")
            ? 16_384
            : lower.startsWith("claude-")
                ? 8_192
                : undefined;
    const tokenizerName = lower.startsWith("gpt-5") || lower.startsWith("gpt-4.5")
        ? "o200k_base"
        : lower.startsWith("gpt-4") || lower.startsWith("gpt-3")
            ? "cl100k_base"
            : capabilities.tokenizerName;
    return freezeProfile({
        provider,
        modelName,
        capabilities: capabilitiesSnapshot,
        supportsReasoning: thinking.protocol !== ThinkingProtocol.None && capabilitiesSnapshot.reasoningContent,
        thinkingProtocol: thinking.protocol,
        thinkingLevels: thinking.levels,
        supportsImages: capabilitiesSnapshot.imageInput && !lower.startsWith("o3-mini") && !lower.startsWith("o1-mini") && !lower.startsWith("o1-preview"),
        supportsTools: capabilitiesSnapshot.toolSupport,
        maxOutputTokens,
        supportsStreaming: true,
        supportsParallelToolCalls: capabilitiesSnapshot.supportsParallelToolCalls,
        supportsToolChoiceNone: capabilitiesSnapshot.supportsToolChoiceNone,
        contextWindow: inferContextWindow(provider, modelName),
        excludedTools: [],
        promptSuffix: undefined,
        tokenizerName: tokenizerName ?? capabilitiesSnapshot.tokenizerName,
    });
}
/** Rust-backed model policy projection; transport and execution remain host-owned. */
export const ModelProfile = Object.freeze({
    new(modelName, provider, capabilities) {
        return createProfile(modelName, provider, capabilities);
    },
    fromProviderName(modelName, provider) {
        return createProfile(modelName, provider, ProviderCapabilities.fromProviderName(provider));
    },
});
function normalize(value) { return value.trim().toLowerCase(); }
function selectorKey(provider, model) { return `${normalize(provider)}:${normalize(model)}`; }
function applyOverride(profile, override) {
    const structuredOutput = override.supportsStructuredOutput === undefined
        ? profile.capabilities.structuredOutput
        : override.supportsStructuredOutput;
    return freezeProfile({
        ...profile,
        capabilities: override.supportsStructuredOutput === undefined ? profile.capabilities : cloneCapabilities(profile.capabilities, structuredOutput),
        supportsParallelToolCalls: override.supportsParallelToolCalls ?? profile.supportsParallelToolCalls,
        supportsToolChoiceNone: override.supportsToolChoiceNone ?? profile.supportsToolChoiceNone,
        contextWindow: override.contextWindow ?? profile.contextWindow,
        excludedTools: [...new Set([...profile.excludedTools, ...override.excludedTools])],
        promptSuffix: override.promptSuffix ?? profile.promptSuffix,
    });
}
export class ModelProfileResolver {
    providerDefaults = new Map();
    exactModels = new Map();
    static new() { return new ModelProfileResolver(); }
    registerProviderDefault(provider, profile) {
        this.providerDefaults.set(normalize(provider), freezeOverride(profile));
        return this;
    }
    registerExact(provider, model, profile) {
        this.exactModels.set(selectorKey(provider, model), freezeOverride(profile));
        return this;
    }
    resolve(provider, model, capabilities) {
        let profile = createProfile(model, provider, capabilities);
        const providerDefault = this.providerDefaults.get(normalize(provider));
        if (providerDefault)
            profile = applyOverride(profile, providerDefault);
        const exact = this.exactModels.get(selectorKey(provider, model));
        if (exact)
            profile = applyOverride(profile, exact);
        return profile;
    }
}
//# sourceMappingURL=model_profile_values.js.map