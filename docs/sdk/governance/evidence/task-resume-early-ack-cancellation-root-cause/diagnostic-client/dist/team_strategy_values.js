export function teamStrategyManager() {
    return Object.freeze({ kind: "manager_subagent" });
}
export function teamStrategyPipeline(members) {
    return Object.freeze({ kind: "pipeline", members: Object.freeze([...members]) });
}
export function teamStrategyDebate(judge, debaters) {
    return Object.freeze({ kind: "debate", judge, debaters: Object.freeze([...debaters]) });
}
export function teamStrategySwarm(reducer) {
    return Object.freeze({ kind: "swarm", reducer });
}
export function teamStrategyName(strategy) {
    return strategy.kind;
}
export function teamStrategyDescription(strategy) {
    switch (strategy.kind) {
        case "manager_subagent": return "Manager plans typed tasks, Subagents execute them, and the manager synthesizes";
        case "pipeline": return "Subagents execute in sequence";
        case "debate": return "Debaters propose independently and a judge synthesizes";
        case "swarm": return "Subagents inspect independently and a reducer synthesizes";
    }
}
//# sourceMappingURL=team_strategy_values.js.map