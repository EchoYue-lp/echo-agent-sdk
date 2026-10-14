/** Subagent command/status values projected without owning dispatch state. */
export const SubagentCommandPhase = Object.freeze({
    Persisted: "persisted",
    MailboxAccepted: "mailbox_accepted",
    Drained: "drained",
    TurnSettled: "turn_settled",
});
export function subagentCommandPhaseAsStr(phase) {
    if (!Object.values(SubagentCommandPhase).includes(phase)) {
        throw new TypeError("invalid subagent command phase");
    }
    return phase;
}
export function subagentCommandPhaseParse(value) {
    if (typeof value !== "string")
        return undefined;
    return Object.values(SubagentCommandPhase).find((phase) => phase === value);
}
export const SubagentStatus = Object.freeze({
    Running: "running",
    Completed: "completed",
    Failed: "failed",
    Cancelled: "cancelled",
    TimedOut: "timed_out",
});
export function subagentStatusAsStr(status) {
    if (!Object.values(SubagentStatus).includes(status)) {
        throw new TypeError("invalid subagent status");
    }
    return status;
}
export function subagentStatusParse(value) {
    if (typeof value !== "string")
        throw new TypeError("subagent status must be text");
    const status = Object.values(SubagentStatus).find((candidate) => candidate === value);
    if (status === undefined)
        throw new Error(`unknown Subagent status: ${value}`);
    return status;
}
//# sourceMappingURL=subagent_values.js.map