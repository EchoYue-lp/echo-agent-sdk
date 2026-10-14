/** Subagent context snapshot values without owning tools, messages, or stores. */
export class SubagentContext {
    toolDefinitions;
    messages;
    storePresent;
    parentGoal;
    allowedTools;
    constructor(toolDefinitions = [], messages = [], storePresent = false, parentGoal = null, allowedTools = null) {
        this.toolDefinitions = toolDefinitions;
        this.messages = messages;
        this.storePresent = storePresent;
        this.parentGoal = parentGoal;
        this.allowedTools = allowedTools;
    }
    static empty() {
        return new SubagentContext();
    }
    hasContent() {
        return this.toolDefinitions.length > 0
            || this.messages.length > 0
            || this.storePresent
            || this.parentGoal !== null
            || this.allowedTools !== null;
    }
}
//# sourceMappingURL=subagent_context_values.js.map