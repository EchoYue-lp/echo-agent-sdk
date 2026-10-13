/** Terminal Subagent hook status values projected without owning hooks. */
export const SubagentStopStatus = Object.freeze({
    Completed: "completed",
    Failed: "failed",
    Cancelled: "cancelled",
    TimedOut: "timed_out",
});
export function subagentStopStatusAsStr(status) {
    if (!Object.values(SubagentStopStatus).includes(status)) {
        throw new TypeError("invalid Subagent stop status");
    }
    return status;
}
//# sourceMappingURL=subagent_stop_values.js.map