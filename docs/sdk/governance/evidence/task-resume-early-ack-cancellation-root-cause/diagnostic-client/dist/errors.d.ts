import type { WireHandle } from "./types.js";
export declare class EchoAgentError extends Error {
    readonly code: string;
    readonly retryable: string;
    readonly operation?: string;
    readonly details?: unknown;
    readonly handle?: WireHandle;
    constructor(code: string, message: string, retryable?: string, operation?: string, details?: unknown, handle?: WireHandle);
    static fromUnknown(error: unknown, operation?: string): EchoAgentError;
}
