/** Subagent context inheritance values projected without owning context state. */
export type ContextInheritanceMode = "sync" | "fork" | "teammate" | "team";
export interface ContextInheritance {
    readonly inheritTools: readonly string[] | null;
    readonly inheritHistory: bigint | null;
    readonly inheritMemory: boolean;
    readonly injectMetadata: Readonly<Record<string, string>>;
}
export declare function contextInheritanceSyncDefault(): ContextInheritance;
export declare function contextInheritanceFreshDefault(): ContextInheritance;
export declare function contextInheritanceForkDefault(): ContextInheritance;
export declare function contextInheritanceTeammateDefault(): ContextInheritance;
export declare function contextInheritanceForMode(mode: ContextInheritanceMode): ContextInheritance;
