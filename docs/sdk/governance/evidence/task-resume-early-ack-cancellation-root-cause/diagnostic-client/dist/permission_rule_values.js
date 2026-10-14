/** Permission rule source values projected without owning evaluation order. */
export const RuleSource = Object.freeze({
    Default: "default",
    LocalSettings: "localSettings",
    ProjectSettings: "projectSettings",
    UserSettings: "userSettings",
    Managed: "managed",
    CliArg: "cliArg",
    Session: "session",
});
export function ruleSourceParse(value) {
    if (typeof value !== "string")
        throw new TypeError("rule source must be text");
    const aliases = {
        default: RuleSource.Default,
        localSettings: RuleSource.LocalSettings,
        local_settings: RuleSource.LocalSettings,
        projectSettings: RuleSource.ProjectSettings,
        project_settings: RuleSource.ProjectSettings,
        userSettings: RuleSource.UserSettings,
        user_settings: RuleSource.UserSettings,
        manual: RuleSource.UserSettings,
        managed: RuleSource.Managed,
        cliArg: RuleSource.CliArg,
        cli_arg: RuleSource.CliArg,
        session: RuleSource.Session,
    };
    const source = aliases[value];
    if (source === undefined)
        throw new Error(`unknown permission rule source: ${value}`);
    return source;
}
//# sourceMappingURL=permission_rule_values.js.map