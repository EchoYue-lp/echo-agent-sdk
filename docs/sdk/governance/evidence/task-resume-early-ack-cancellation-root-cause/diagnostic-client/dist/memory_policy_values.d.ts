/** Typed memory classifications and source policies projected from Rust. */
export declare const MemoryType: Readonly<{
    readonly UserPreference: "user_preference";
    readonly ProjectFact: "project_fact";
    readonly ArchitectureDecision: "architecture_decision";
    readonly DebuggingLesson: "debugging_lesson";
    readonly ErrorResolution: "error_resolution";
    readonly CommandPattern: "command_pattern";
    readonly ToolUsage: "tool_usage";
    readonly WorkflowPattern: "workflow_pattern";
    readonly SkillCandidate: "skill_candidate";
    readonly DeprecatedNote: "deprecated_note";
}>;
export type MemoryType = (typeof MemoryType)[keyof typeof MemoryType];
export declare const MemorySource: Readonly<{
    readonly UserCorrection: "user_correction";
    readonly ErrorResolution: "error_resolution";
    readonly RepeatedWorkflow: "repeated_workflow";
    readonly ExplicitSave: "explicit_save";
    readonly AutoExtracted: "auto_extracted";
    readonly L3Promotion: "l3_promotion";
}>;
export type MemorySource = (typeof MemorySource)[keyof typeof MemorySource];
export declare function memoryTypeDefaultStability(memoryType: MemoryType): number;
export declare function memoryTypeIsSkillEligible(memoryType: MemoryType): boolean;
export declare function memoryTypeIsRuleEligible(memoryType: MemoryType): boolean;
export declare function memorySourceDefaultConfidence(source: MemorySource): number;
export declare function memorySourceDefaultRecallWeight(source: MemorySource): number;
