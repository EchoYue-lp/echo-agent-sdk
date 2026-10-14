const freeze = (action) => Object.freeze(action);
export const hookCommand = (command, shell = null, timeout = 300n) => freeze({ type: "command", command, shell, timeout });
export const hookPrompt = (prompt) => freeze({ type: "prompt", prompt });
export const hookPermission = (decision, reason = null, suggestions = []) => freeze({ type: "permission", decision, reason, suggestions: Object.freeze([...suggestions]) });
export const hookHttp = (url, method = null, headers = null, timeout = 300n) => freeze({ type: "http", url, method, headers: headers ? Object.freeze({ ...headers }) : null, timeout });
export const hookMcpTool = (server, tool, args = null, timeout = 300n) => freeze({ type: "mcp_tool", server, tool, arguments: args, timeout });
export const hookSubagent = (name, task = null, timeout = 300n) => freeze({ type: "subagent", name, task, timeout });
export const hookActivateSkill = (skill, reason = "") => freeze({ type: "activate_skill", skill, reason });
export function hookActionKind(action) { return action.type; }
export function hookActionValidate(action) {
    if (action.type === "command") {
        if (!action.command)
            throw new Error("Command hook has empty command string");
        if (Array.from(action.command).length > 32 * 1024)
            throw new Error("Command hook exceeds max length");
        if (action.timeout > 3600n)
            throw new Error("Command hook timeout exceeds maximum");
    }
    else if (action.type === "prompt") {
        if (!action.prompt)
            throw new Error("Prompt hook has empty prompt string");
    }
    else if (action.type === "permission") {
        if (!["allow", "deny", "ask"].includes(action.decision))
            throw new Error("Permission hook has invalid decision");
    }
    else if (action.type === "http") {
        if (!action.url)
            throw new Error("Http hook has empty url");
        const parsed = new URL(action.url);
        if (parsed.protocol !== "https:" && !(parsed.protocol === "http:" && ["localhost", "127.0.0.1", "::1"].includes(parsed.hostname)))
            throw new Error("Http hook must use https unless it targets a local address");
        if (action.timeout > 3600n)
            throw new Error("Http hook timeout exceeds maximum");
    }
    else if (action.type === "mcp_tool") {
        if (!action.server)
            throw new Error("McpTool hook has empty server name");
        if (!action.tool)
            throw new Error("McpTool hook has empty tool name");
        if (action.timeout > 3600n)
            throw new Error("McpTool hook timeout exceeds maximum");
    }
    else if (action.type === "subagent") {
        if (!action.name)
            throw new Error("Subagent hook has empty subagent name");
        if (action.timeout > 3600n)
            throw new Error("Subagent hook timeout exceeds maximum");
    }
    else if (!action.skill) {
        throw new Error("ActivateSkill hook has empty skill name");
    }
}
//# sourceMappingURL=hook_action_values.js.map