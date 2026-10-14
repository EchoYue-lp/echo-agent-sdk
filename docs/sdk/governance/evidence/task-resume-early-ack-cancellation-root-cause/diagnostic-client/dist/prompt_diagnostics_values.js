export class PromptDiagnostics {
    sections;
    constructor(sections = []) {
        this.sections = sections.map((section) => Object.freeze({ ...section }));
    }
    record(id, source) {
        this.sections.push(Object.freeze({ id, source }));
    }
    count(id) {
        return this.sections.filter((section) => section.id === id).length;
    }
}
//# sourceMappingURL=prompt_diagnostics_values.js.map