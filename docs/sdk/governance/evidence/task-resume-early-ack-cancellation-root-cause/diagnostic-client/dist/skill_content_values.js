export class SkillContent {
    name;
    skillDir;
    instructions;
    allowedTools;
    resources;
    constructor(name, skillDir, instructions, allowedTools = [], resources = []) {
        this.name = name;
        this.skillDir = skillDir;
        this.instructions = instructions;
        this.allowedTools = allowedTools;
        this.resources = resources;
    }
    toPromptBlock() {
        let block = `<skill_content name="${this.name}">\n${this.instructions.trim()}\n\nSkill directory: ${this.skillDir}\nRelative paths in this skill are relative to the skill directory.`;
        if (this.allowedTools.length > 0) {
            block += `\n\n<allowed_tools>\nThis skill declares the following preferred/allowed tools: ${this.allowedTools.join(", ")}\nRuntime enforcement currently applies to the built-in skill tools such as read_skill_resource and run_skill_script.\n</allowed_tools>`;
        }
        if (this.resources.length > 0) {
            block += "\n\n<skill_resources>";
            for (const resource of this.resources)
                block += `\n  <file kind="${resource.kind}">${resource.relativePath}</file>`;
            block += "\n</skill_resources>";
        }
        return `${block}\n</skill_content>`;
    }
}
//# sourceMappingURL=skill_content_values.js.map