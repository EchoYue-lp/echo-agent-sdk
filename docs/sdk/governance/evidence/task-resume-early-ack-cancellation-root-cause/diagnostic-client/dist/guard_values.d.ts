/** Guard decisions projected without owning guard execution. */
export type GuardDecision = {
    readonly kind: "pass";
} | {
    readonly kind: "block";
    readonly reason: string;
} | {
    readonly kind: "warn";
    readonly reasons: readonly string[];
} | {
    readonly kind: "transform";
    readonly content: string;
    readonly reasons: readonly string[];
};
export declare function guardDecisionPass(): GuardDecision;
export declare function guardDecisionBlock(reason: string): GuardDecision;
export declare function guardDecisionWarn(reasons: readonly string[]): GuardDecision;
export declare function guardDecisionTransform(content: string, reasons: readonly string[]): GuardDecision;
export declare function guardDecisionIsBlocked(decision: GuardDecision): boolean;
