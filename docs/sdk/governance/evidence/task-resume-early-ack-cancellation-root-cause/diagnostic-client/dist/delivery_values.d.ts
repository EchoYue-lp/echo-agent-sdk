/** Durable delivery outcome/phase values projected without owning the ledger. */
export declare const DeliveryOutcome: Readonly<{
    readonly Completed: "completed";
    readonly Failed: "failed";
    readonly Cancelled: "cancelled";
    readonly Dropped: "dropped";
    readonly OutcomeUnknown: "outcome_unknown";
}>;
export type DeliveryOutcome = (typeof DeliveryOutcome)[keyof typeof DeliveryOutcome];
export declare function deliveryOutcomeAsStr(outcome: DeliveryOutcome): string;
export declare const DeliveryPhase: Readonly<{
    readonly Persisted: "persisted";
    readonly Claimed: "claimed";
    readonly EffectStarted: "effect_started";
    readonly MailboxAccepted: "mailbox_accepted";
    readonly Drained: "drained";
    readonly Deferred: "deferred";
    readonly TurnSettled: "turn_settled";
}>;
export type DeliveryPhase = (typeof DeliveryPhase)[keyof typeof DeliveryPhase];
export declare function deliveryPhaseAsStr(phase: DeliveryPhase): string;
