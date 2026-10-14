/** Terminal Subagent hook status values projected without owning hooks. */
export declare const SubagentStopStatus: Readonly<{
    readonly Completed: "completed";
    readonly Failed: "failed";
    readonly Cancelled: "cancelled";
    readonly TimedOut: "timed_out";
}>;
export type SubagentStopStatus = (typeof SubagentStopStatus)[keyof typeof SubagentStopStatus];
export declare function subagentStopStatusAsStr(status: SubagentStopStatus): string;
