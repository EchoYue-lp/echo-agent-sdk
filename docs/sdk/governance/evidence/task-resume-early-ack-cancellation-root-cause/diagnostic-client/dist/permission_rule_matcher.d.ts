/** Permission matcher values and pure matching helpers. */
import type { ToolPermission } from "./types.js";
export declare const RuleMatcherPermission: Readonly<{
    readonly Read: "read";
    readonly Write: "write";
    readonly Network: "network";
    readonly Execute: "execute";
    readonly Sensitive: "sensitive";
}>;
export type RuleMatcher = {
    readonly kind: "tool";
    readonly name: string;
} | {
    readonly kind: "pattern";
    readonly pattern: string;
} | {
    readonly kind: "permission";
    readonly permission: ToolPermission;
} | {
    readonly kind: "all";
};
export declare function ruleMatcherParse(value: string): RuleMatcher;
export declare function ruleMatcherDisplay(matcher: RuleMatcher): string;
export declare function ruleMatcherMatchesMatcherStr(matcher: RuleMatcher, value: string): boolean;
export declare function ruleMatcherMatches(matcher: RuleMatcher, toolName: string, permissions: readonly ToolPermission[]): boolean;
