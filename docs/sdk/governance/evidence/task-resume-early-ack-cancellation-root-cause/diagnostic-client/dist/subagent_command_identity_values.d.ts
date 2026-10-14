/** Durable Subagent command identities without owning live-control state. */
export declare class SubagentAttemptIdentity {
    readonly taskId: string;
    readonly executionId: string;
    readonly attempt: number;
    constructor(taskId: string, executionId: string, attempt: number);
}
export declare class SubagentCommandIdentity {
    readonly runId: string;
    readonly taskId: string;
    readonly executionId: string;
    readonly planRevision: bigint;
    readonly attempt: number;
    readonly commandId: string;
    constructor(runId: string, taskId: string, executionId: string, planRevision: bigint, attempt: number, commandId: string);
    validate(): void;
    attemptIdentity(): SubagentAttemptIdentity;
}
