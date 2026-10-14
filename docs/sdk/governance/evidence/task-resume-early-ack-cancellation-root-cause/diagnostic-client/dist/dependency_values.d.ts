/** Skill dependency/source values projected without probing or loading. */
export declare const DependencyKind: Readonly<{
    readonly Binary: "binary";
    readonly PythonPkg: "python_pkg";
    readonly NodeModule: "node_module";
}>;
export type DependencyKind = (typeof DependencyKind)[keyof typeof DependencyKind];
export declare const SkillSource: Readonly<{
    readonly Local: "local";
    readonly Mcp: "mcp";
}>;
export type SkillSource = (typeof SkillSource)[keyof typeof SkillSource];
