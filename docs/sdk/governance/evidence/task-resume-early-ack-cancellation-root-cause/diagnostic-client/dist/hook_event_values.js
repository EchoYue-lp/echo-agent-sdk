/** Hook event names and matcher semantics projected as local immutable values. */
export const HookEventCategory = Object.freeze({
    Tool: "Tool",
    Lifecycle: "Lifecycle",
    Subagent: "Subagent",
    Task: "Task",
    Error: "Error",
    Evolution: "Evolution",
});
const EVENT_NAMES = [
    "PreToolUse",
    "PostToolUse",
    "PostToolUseFailure",
    "PermissionRequest",
    "PermissionDenied",
    "SessionStart",
    "SessionEnd",
    "Stop",
    "Notification",
    "UserPromptSubmit",
    "PreCompact",
    "PostCompact",
    "ConfigChange",
    "InstructionsLoaded",
    "PostToolBatch",
    "SubagentStart",
    "SubagentStop",
    "TaskCreated",
    "TaskStarted",
    "TaskCompleted",
    "StopFailure",
    "PluginLoaded",
    "PluginDisabled",
    "PostMemoryWrite",
    "MemoryLayerChange",
    "SkillCandidateDetected",
    "SkillLifecycleTransition",
    "SkillHealthCheck",
    "SkillPatchApplied",
    "SkillMergeApplied",
    "RulePromoted",
];
export const HookEvent = Object.freeze({
    PreToolUse: "PreToolUse",
    PostToolUse: "PostToolUse",
    PostToolUseFailure: "PostToolUseFailure",
    PermissionRequest: "PermissionRequest",
    PermissionDenied: "PermissionDenied",
    SessionStart: "SessionStart",
    SessionEnd: "SessionEnd",
    Stop: "Stop",
    Notification: "Notification",
    UserPromptSubmit: "UserPromptSubmit",
    PreCompact: "PreCompact",
    PostCompact: "PostCompact",
    ConfigChange: "ConfigChange",
    InstructionsLoaded: "InstructionsLoaded",
    PostToolBatch: "PostToolBatch",
    SubagentStart: "SubagentStart",
    SubagentStop: "SubagentStop",
    TaskCreated: "TaskCreated",
    TaskStarted: "TaskStarted",
    TaskCompleted: "TaskCompleted",
    StopFailure: "StopFailure",
    PluginLoaded: "PluginLoaded",
    PluginDisabled: "PluginDisabled",
    PostMemoryWrite: "PostMemoryWrite",
    MemoryLayerChange: "MemoryLayerChange",
    SkillCandidateDetected: "SkillCandidateDetected",
    SkillLifecycleTransition: "SkillLifecycleTransition",
    SkillHealthCheck: "SkillHealthCheck",
    SkillPatchApplied: "SkillPatchApplied",
    SkillMergeApplied: "SkillMergeApplied",
    RulePromoted: "RulePromoted",
    ALL: EVENT_NAMES,
});
const TOOL_EVENTS = new Set([
    HookEvent.PreToolUse,
    HookEvent.PostToolUse,
    HookEvent.PostToolUseFailure,
    HookEvent.PermissionRequest,
    HookEvent.PermissionDenied,
]);
const SUBAGENT_EVENTS = new Set([
    HookEvent.SubagentStart,
    HookEvent.SubagentStop,
]);
const TASK_EVENTS = new Set([
    HookEvent.TaskCreated,
    HookEvent.TaskStarted,
    HookEvent.TaskCompleted,
]);
const ERROR_EVENTS = new Set([HookEvent.StopFailure]);
const EVOLUTION_EVENTS = new Set([
    HookEvent.PostMemoryWrite,
    HookEvent.MemoryLayerChange,
    HookEvent.SkillCandidateDetected,
    HookEvent.SkillLifecycleTransition,
    HookEvent.SkillHealthCheck,
    HookEvent.SkillPatchApplied,
    HookEvent.SkillMergeApplied,
    HookEvent.RulePromoted,
]);
function assertEvent(event) {
    if (!EVENT_NAMES.includes(event)) {
        throw new TypeError(`invalid hook event: ${event}`);
    }
}
export function hookEventCategory(event) {
    assertEvent(event);
    if (TOOL_EVENTS.has(event))
        return HookEventCategory.Tool;
    if (SUBAGENT_EVENTS.has(event))
        return HookEventCategory.Subagent;
    if (TASK_EVENTS.has(event))
        return HookEventCategory.Task;
    if (ERROR_EVENTS.has(event))
        return HookEventCategory.Error;
    if (EVOLUTION_EVENTS.has(event))
        return HookEventCategory.Evolution;
    return HookEventCategory.Lifecycle;
}
export function hookEventAsStr(event) {
    assertEvent(event);
    return event;
}
export function hookEventFromName(name) {
    return EVENT_NAMES.includes(name) ? name : undefined;
}
export function hookEventIsToolEvent(event) {
    return TOOL_EVENTS.has(event);
}
export function hookEventSupportsMatcher(event) {
    assertEvent(event);
    return true;
}
//# sourceMappingURL=hook_event_values.js.map