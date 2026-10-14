/** Subagent context snapshot values without owning tools, messages, or stores. */
export declare class SubagentContext {
    readonly toolDefinitions: readonly unknown[];
    readonly messages: readonly unknown[];
    readonly storePresent: boolean;
    readonly parentGoal: string | null;
    readonly allowedTools: readonly string[] | null;
    constructor(toolDefinitions?: readonly unknown[], messages?: readonly unknown[], storePresent?: boolean, parentGoal?: string | null, allowedTools?: readonly string[] | null);
    static empty(): SubagentContext;
    hasContent(): boolean;
}
