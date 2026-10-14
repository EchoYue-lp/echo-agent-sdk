/** HTTP API protocols used by supported LLM endpoints. */
export declare const LlmApiProtocol: Readonly<{
    readonly ChatCompletions: "chat_completions";
    readonly Responses: "responses";
    readonly Anthropic: "anthropic";
}>;
export type LlmApiProtocol = (typeof LlmApiProtocol)[keyof typeof LlmApiProtocol];
export declare function llmApiProtocolEndpointPath(protocol: LlmApiProtocol): string;
export declare function llmApiProtocolTryFromEndpoint(endpoint: string): LlmApiProtocol | undefined;
export declare function llmApiProtocolFromEndpoint(endpoint: string): LlmApiProtocol;
