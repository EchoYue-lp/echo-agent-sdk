/** User-facing reasoning effort levels with Rust-compatible parsing. */
export declare const ThinkingLevel: Readonly<{
    readonly None: "none";
    readonly Minimal: "minimal";
    readonly Low: "low";
    readonly Medium: "medium";
    readonly High: "high";
    readonly Xhigh: "xhigh";
    readonly Max: "max";
}>;
export type ThinkingLevel = (typeof ThinkingLevel)[keyof typeof ThinkingLevel];
export declare function thinkingLevelParse(value: string): ThinkingLevel | undefined;
