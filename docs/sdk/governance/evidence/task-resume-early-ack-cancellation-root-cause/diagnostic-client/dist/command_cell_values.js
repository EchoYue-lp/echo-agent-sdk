/** Command-cell renderer phases projected without owning execution. */
export const CommandCellPhase = Object.freeze({
    Prepared: "prepared",
    Queued: "queued",
    Running: "running",
    Succeeded: "succeeded",
    Failed: "failed",
    Cancelled: "cancelled",
    LaunchFailed: "launch_failed",
});
export function commandCellPhaseAsStr(phase) {
    if (!Object.values(CommandCellPhase).includes(phase)) {
        throw new TypeError("invalid command cell phase");
    }
    return phase;
}
export function commandCellPhaseIsTerminal(phase) {
    return phase === CommandCellPhase.Succeeded
        || phase === CommandCellPhase.Failed
        || phase === CommandCellPhase.Cancelled
        || phase === CommandCellPhase.LaunchFailed;
}
//# sourceMappingURL=command_cell_values.js.map