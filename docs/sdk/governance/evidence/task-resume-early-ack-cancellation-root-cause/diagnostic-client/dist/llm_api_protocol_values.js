/** HTTP API protocols used by supported LLM endpoints. */
export const LlmApiProtocol = Object.freeze({
    ChatCompletions: "chat_completions",
    Responses: "responses",
    Anthropic: "anthropic",
});
export function llmApiProtocolEndpointPath(protocol) {
    switch (protocol) {
        case LlmApiProtocol.Responses: return "responses";
        case LlmApiProtocol.Anthropic: return "messages";
        case LlmApiProtocol.ChatCompletions: return "chat/completions";
    }
}
export function llmApiProtocolTryFromEndpoint(endpoint) {
    const base = (endpoint.split(/[?#]/u)[0] ?? endpoint).replace(/\/+$/u, "");
    if (base.endsWith("/responses"))
        return LlmApiProtocol.Responses;
    if (base.endsWith("/messages"))
        return LlmApiProtocol.Anthropic;
    if (base.endsWith("/chat/completions"))
        return LlmApiProtocol.ChatCompletions;
    return undefined;
}
export function llmApiProtocolFromEndpoint(endpoint) {
    const detected = llmApiProtocolTryFromEndpoint(endpoint);
    if (detected !== undefined)
        return detected;
    const base = (endpoint.split(/[?#]/u)[0] ?? endpoint);
    return base.includes("anthropic.com/") ? LlmApiProtocol.Anthropic : LlmApiProtocol.ChatCompletions;
}
//# sourceMappingURL=llm_api_protocol_values.js.map