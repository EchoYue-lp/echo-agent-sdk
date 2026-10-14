/** Subagent command/status values projected without owning dispatch state. */
export declare const SubagentCommandPhase: Readonly<{
    readonly Persisted: "persisted";
    readonly MailboxAccepted: "mailbox_accepted";
    readonly Drained: "drained";
    readonly TurnSettled: "turn_settled";
}>;
export type SubagentCommandPhase = (typeof SubagentCommandPhase)[keyof typeof SubagentCommandPhase];
export declare function subagentCommandPhaseAsStr(phase: SubagentCommandPhase): string;
export declare function subagentCommandPhaseParse(value: string): SubagentCommandPhase | undefined;
export declare const SubagentStatus: Readonly<{
    readonly Running: "running";
    readonly Completed: "completed";
    readonly Failed: "failed";
    readonly Cancelled: "cancelled";
    readonly TimedOut: "timed_out";
}>;
export type SubagentStatus = (typeof SubagentStatus)[keyof typeof SubagentStatus];
export declare function subagentStatusAsStr(status: SubagentStatus): string;
export declare function subagentStatusParse(value: string): SubagentStatus;
