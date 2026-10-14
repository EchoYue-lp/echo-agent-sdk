export class EchoAgentError extends Error {
    code;
    retryable;
    operation;
    details;
    handle;
    constructor(code, message, retryable = "never", operation, details, handle) {
        super(message);
        this.name = "EchoAgentError";
        this.code = code;
        this.retryable = retryable;
        this.operation = operation;
        this.details = details;
        this.handle = handle;
    }
    static fromUnknown(error, operation) {
        if (error instanceof EchoAgentError)
            return error;
        const candidate = error;
        const data = candidate && typeof candidate.data === "object" && candidate.data !== null
            ? candidate.data
            : undefined;
        const code = typeof data?.code === "string" ? data.code : "transport_error";
        const message = typeof data?.message === "string"
            ? data.message
            : typeof candidate?.message === "string" ? candidate.message : String(error);
        const retryable = typeof data?.retryable === "string" ? data.retryable : "never";
        const resolvedOperation = typeof data?.operation === "string" ? data.operation : operation;
        return new EchoAgentError(code, message, retryable, resolvedOperation, data?.details, data?.handle);
    }
}
//# sourceMappingURL=errors.js.map