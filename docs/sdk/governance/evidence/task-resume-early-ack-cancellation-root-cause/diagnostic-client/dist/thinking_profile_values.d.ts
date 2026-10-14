import { ThinkingLevel } from "./thinking.js";
import { ThinkingProtocol } from "./thinking_protocol_values.js";
export type ThinkingProfile = Readonly<{
    protocol: ThinkingProtocol;
    levels: readonly ThinkingLevel[];
    supportsManualControl(): boolean;
}>;
/** Resolve the provider/model thinking dialect without contacting a provider. */
export declare function resolveThinkingProfile(provider: string, modelName: string, apiProtocol?: string, endpoint?: string): ThinkingProfile;
export declare const ThinkingProfile: Readonly<{
    "new"(protocol: ThinkingProtocol, levels: readonly ThinkingLevel[]): ThinkingProfile;
    unknown(): ThinkingProfile;
}>;
