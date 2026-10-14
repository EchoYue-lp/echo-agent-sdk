/** Steering lifecycle values projected without owning an Agent turn. */
export const AgentSteerPhase = Object.freeze({
    Accepted: "accepted",
    Drained: "drained",
    TurnSettled: "turn_settled",
});
export const AgentSteerTurnOutcome = Object.freeze({
    Completed: "completed",
    Cancelled: "cancelled",
    Failed: "failed",
    Dropped: "dropped",
});
export function agentSteerStateAccepted() {
    return Object.freeze({ kind: AgentSteerPhase.Accepted });
}
export function agentSteerStateDrained() {
    return Object.freeze({ kind: AgentSteerPhase.Drained });
}
export function agentSteerStateTurnSettled(outcome, drained) {
    if (!Object.values(AgentSteerTurnOutcome).includes(outcome)) {
        throw new TypeError("invalid steering turn outcome");
    }
    if (typeof drained !== "boolean")
        throw new TypeError("drained must be boolean");
    return Object.freeze({ kind: AgentSteerPhase.TurnSettled, outcome, drained });
}
export function agentSteerStatePhase(state) {
    return state.kind;
}
export function agentSteerStateWasDrained(state) {
    return state.kind === AgentSteerPhase.Drained
        || (state.kind === AgentSteerPhase.TurnSettled && state.drained);
}
export function agentSteerTurnOutcomeAsStr(outcome) {
    if (!Object.values(AgentSteerTurnOutcome).includes(outcome)) {
        throw new TypeError("invalid steering turn outcome");
    }
    return outcome;
}
export function agentSteerTurnOutcomeParse(value) {
    if (typeof value !== "string")
        return undefined;
    return Object.values(AgentSteerTurnOutcome).find((outcome) => outcome === value);
}
//# sourceMappingURL=steering_values.js.map