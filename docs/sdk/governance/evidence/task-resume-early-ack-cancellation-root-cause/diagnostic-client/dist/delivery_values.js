/** Durable delivery outcome/phase values projected without owning the ledger. */
export const DeliveryOutcome = Object.freeze({
    Completed: "completed",
    Failed: "failed",
    Cancelled: "cancelled",
    Dropped: "dropped",
    OutcomeUnknown: "outcome_unknown",
});
export function deliveryOutcomeAsStr(outcome) {
    if (!Object.values(DeliveryOutcome).includes(outcome)) {
        throw new TypeError("invalid delivery outcome");
    }
    return outcome;
}
export const DeliveryPhase = Object.freeze({
    Persisted: "persisted",
    Claimed: "claimed",
    EffectStarted: "effect_started",
    MailboxAccepted: "mailbox_accepted",
    Drained: "drained",
    Deferred: "deferred",
    TurnSettled: "turn_settled",
});
export function deliveryPhaseAsStr(phase) {
    if (!Object.values(DeliveryPhase).includes(phase)) {
        throw new TypeError("invalid delivery phase");
    }
    return phase;
}
//# sourceMappingURL=delivery_values.js.map