/** Durable Subagent command identities without owning live-control state. */
export class SubagentAttemptIdentity {
    taskId;
    executionId;
    attempt;
    constructor(taskId, executionId, attempt) {
        this.taskId = taskId;
        this.executionId = executionId;
        this.attempt = attempt;
        if (!taskId.trim())
            throw new Error("invalid identity: task_id");
        if (!executionId.trim())
            throw new Error("invalid identity: execution_id");
        if (!Number.isInteger(attempt) || attempt < 0 || attempt > 0xffffffff) {
            throw new Error("invalid identity: attempt");
        }
        Object.freeze(this);
    }
}
export class SubagentCommandIdentity {
    runId;
    taskId;
    executionId;
    planRevision;
    attempt;
    commandId;
    constructor(runId, taskId, executionId, planRevision, attempt, commandId) {
        this.runId = runId;
        this.taskId = taskId;
        this.executionId = executionId;
        this.planRevision = planRevision;
        this.attempt = attempt;
        this.commandId = commandId;
        this.validate();
        Object.freeze(this);
    }
    validate() {
        if (!this.runId.trim())
            throw new Error("invalid identity: run_id");
        if (this.planRevision <= 0n)
            throw new Error("invalid identity: plan_revision");
        if (!this.commandId.trim())
            throw new Error("invalid identity: command_id");
        new SubagentAttemptIdentity(this.taskId, this.executionId, this.attempt);
    }
    attemptIdentity() {
        return new SubagentAttemptIdentity(this.taskId, this.executionId, this.attempt);
    }
}
//# sourceMappingURL=subagent_command_identity_values.js.map