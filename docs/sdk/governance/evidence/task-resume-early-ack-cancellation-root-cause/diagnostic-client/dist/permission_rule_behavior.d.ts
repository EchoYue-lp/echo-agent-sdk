/** Permission rule behavior values projected without owning evaluation. */
export type RuleBehavior = {
    readonly kind: "allow";
} | {
    readonly kind: "deny";
    readonly reason: string;
} | {
    readonly kind: "ask";
    readonly suggestions: readonly string[];
};
export type RuleDecision = {
    readonly kind: "allow";
} | {
    readonly kind: "deny";
    readonly reason: string;
} | {
    readonly kind: "ask";
    readonly suggestions: readonly string[];
};
export declare function ruleBehaviorAllow(): RuleBehavior;
export declare function ruleBehaviorDeny(reason: string): RuleBehavior;
export declare function ruleBehaviorAsk(suggestions: readonly string[]): RuleBehavior;
export declare function ruleBehaviorParse(value: string): RuleBehavior;
export declare function ruleBehaviorToDecision(behavior: RuleBehavior): RuleDecision;
