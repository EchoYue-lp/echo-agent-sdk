/** Permission rule source values projected without owning evaluation order. */
export declare const RuleSource: Readonly<{
    readonly Default: "default";
    readonly LocalSettings: "localSettings";
    readonly ProjectSettings: "projectSettings";
    readonly UserSettings: "userSettings";
    readonly Managed: "managed";
    readonly CliArg: "cliArg";
    readonly Session: "session";
}>;
export type RuleSource = (typeof RuleSource)[keyof typeof RuleSource];
export declare function ruleSourceParse(value: string): RuleSource;
