/** Task terminal status values projected without owning task execution. */
export const TaskTerminalStatus = Object.freeze({
    Completed: "completed",
    Failed: "failed",
    Cancelled: "cancelled",
    TimedOut: "timed_out",
    Skipped: "skipped",
});
export function taskTerminalStatusAsStr(status) {
    if (!Object.values(TaskTerminalStatus).includes(status)) {
        throw new TypeError("invalid task terminal status");
    }
    return status;
}
//# sourceMappingURL=task_terminal_values.js.map