/** Hook action configuration values without executing hooks. */
export type HookAction = {
    readonly type: "command";
    readonly command: string;
    readonly shell?: string | null;
    readonly timeout: bigint;
} | {
    readonly type: "prompt";
    readonly prompt: string;
} | {
    readonly type: "permission";
    readonly decision: string;
    readonly reason?: string | null;
    readonly suggestions: readonly string[];
} | {
    readonly type: "http";
    readonly url: string;
    readonly method?: string | null;
    readonly headers?: Readonly<Record<string, string>> | null;
    readonly timeout: bigint;
} | {
    readonly type: "mcp_tool";
    readonly server: string;
    readonly tool: string;
    readonly arguments?: unknown | null;
    readonly timeout: bigint;
} | {
    readonly type: "subagent";
    readonly name: string;
    readonly task?: string | null;
    readonly timeout: bigint;
} | {
    readonly type: "activate_skill";
    readonly skill: string;
    readonly reason: string;
};
export declare const hookCommand: (command: string, shell?: string | null, timeout?: bigint) => HookAction;
export declare const hookPrompt: (prompt: string) => HookAction;
export declare const hookPermission: (decision: string, reason?: string | null, suggestions?: readonly string[]) => HookAction;
export declare const hookHttp: (url: string, method?: string | null, headers?: Readonly<Record<string, string>> | null, timeout?: bigint) => HookAction;
export declare const hookMcpTool: (server: string, tool: string, args?: unknown | null, timeout?: bigint) => HookAction;
export declare const hookSubagent: (name: string, task?: string | null, timeout?: bigint) => HookAction;
export declare const hookActivateSkill: (skill: string, reason?: string) => HookAction;
export declare function hookActionKind(action: HookAction): HookAction["type"];
export declare function hookActionValidate(action: HookAction): void;
