export function jsonRpcRequest(method, params = null) {
    return Object.freeze({ jsonrpc: "2.0", id: null, method, params });
}
export function jsonRpcNotification(method, params = null) {
    return Object.freeze({ jsonrpc: "2.0", method, params });
}
//# sourceMappingURL=jsonrpc_values.js.map