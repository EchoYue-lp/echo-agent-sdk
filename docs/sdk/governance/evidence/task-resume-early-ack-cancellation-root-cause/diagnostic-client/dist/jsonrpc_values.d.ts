/** MCP JSON-RPC request/notification values without owning transport. */
export interface JsonRpcRequest {
    readonly jsonrpc: "2.0";
    readonly id: unknown | null;
    readonly method: string;
    readonly params: unknown | null;
}
export interface JsonRpcNotification {
    readonly jsonrpc: "2.0";
    readonly method: string;
    readonly params: unknown | null;
}
export declare function jsonRpcRequest(method: string, params?: unknown | null): JsonRpcRequest;
export declare function jsonRpcNotification(method: string, params?: unknown | null): JsonRpcNotification;
