/** Task terminal status values projected without owning task execution. */
export declare const TaskTerminalStatus: Readonly<{
    readonly Completed: "completed";
    readonly Failed: "failed";
    readonly Cancelled: "cancelled";
    readonly TimedOut: "timed_out";
    readonly Skipped: "skipped";
}>;
export type TaskTerminalStatus = (typeof TaskTerminalStatus)[keyof typeof TaskTerminalStatus];
export declare function taskTerminalStatusAsStr(status: TaskTerminalStatus): string;
