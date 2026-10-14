/** Skill validation report values without running validation. */
export class SkillValidationReport {
    path;
    violations;
    warnings;
    constructor(path, violations = [], warnings = []) {
        this.path = path;
        this.violations = violations;
        this.warnings = warnings;
        Object.freeze(this.violations);
        Object.freeze(this.warnings);
        Object.freeze(this);
    }
    isValid() {
        return this.violations.length === 0;
    }
}
//# sourceMappingURL=skill_validation_values.js.map