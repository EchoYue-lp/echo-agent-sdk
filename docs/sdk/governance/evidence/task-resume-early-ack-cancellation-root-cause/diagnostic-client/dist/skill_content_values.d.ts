/** Activated Skill content projection without loading or executing resources. */
export interface SkillResourceEntry {
    readonly kind: string;
    readonly relativePath: string;
}
export declare class SkillContent {
    readonly name: string;
    readonly skillDir: string;
    readonly instructions: string;
    readonly allowedTools: readonly string[];
    readonly resources: readonly SkillResourceEntry[];
    constructor(name: string, skillDir: string, instructions: string, allowedTools?: readonly string[], resources?: readonly SkillResourceEntry[]);
    toPromptBlock(): string;
}
