/** Skill validation report values without running validation. */
export declare class SkillValidationReport {
    readonly path: string;
    readonly violations: readonly string[];
    readonly warnings: readonly string[];
    constructor(path: string, violations?: readonly string[], warnings?: readonly string[]);
    isValid(): boolean;
}
