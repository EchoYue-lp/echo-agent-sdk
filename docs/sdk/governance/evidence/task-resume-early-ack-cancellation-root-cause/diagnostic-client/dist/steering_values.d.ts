/** Steering lifecycle values projected without owning an Agent turn. */
export declare const AgentSteerPhase: Readonly<{
    readonly Accepted: "accepted";
    readonly Drained: "drained";
    readonly TurnSettled: "turn_settled";
}>;
export type AgentSteerPhase = (typeof AgentSteerPhase)[keyof typeof AgentSteerPhase];
export declare const AgentSteerTurnOutcome: Readonly<{
    readonly Completed: "completed";
    readonly Cancelled: "cancelled";
    readonly Failed: "failed";
    readonly Dropped: "dropped";
}>;
export type AgentSteerTurnOutcome = (typeof AgentSteerTurnOutcome)[keyof typeof AgentSteerTurnOutcome];
export type AgentSteerState = {
    readonly kind: "accepted";
} | {
    readonly kind: "drained";
} | {
    readonly kind: "turn_settled";
    readonly outcome: AgentSteerTurnOutcome;
    readonly drained: boolean;
};
export declare function agentSteerStateAccepted(): AgentSteerState;
export declare function agentSteerStateDrained(): AgentSteerState;
export declare function agentSteerStateTurnSettled(outcome: AgentSteerTurnOutcome, drained: boolean): AgentSteerState;
export declare function agentSteerStatePhase(state: AgentSteerState): AgentSteerPhase;
export declare function agentSteerStateWasDrained(state: AgentSteerState): boolean;
export declare function agentSteerTurnOutcomeAsStr(outcome: AgentSteerTurnOutcome): string;
export declare function agentSteerTurnOutcomeParse(value: string): AgentSteerTurnOutcome | undefined;
