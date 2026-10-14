/** Team collaboration strategies projected without owning Team execution. */
export type TeamStrategy = {
    readonly kind: "manager_subagent";
} | {
    readonly kind: "pipeline";
    readonly members: readonly string[];
} | {
    readonly kind: "debate";
    readonly judge: string;
    readonly debaters: readonly string[];
} | {
    readonly kind: "swarm";
    readonly reducer: string;
};
export declare function teamStrategyManager(): TeamStrategy;
export declare function teamStrategyPipeline(members: readonly string[]): TeamStrategy;
export declare function teamStrategyDebate(judge: string, debaters: readonly string[]): TeamStrategy;
export declare function teamStrategySwarm(reducer: string): TeamStrategy;
export declare function teamStrategyName(strategy: TeamStrategy): string;
export declare function teamStrategyDescription(strategy: TeamStrategy): string;
