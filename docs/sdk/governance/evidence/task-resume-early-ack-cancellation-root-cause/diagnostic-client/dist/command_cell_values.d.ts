/** Command-cell renderer phases projected without owning execution. */
export declare const CommandCellPhase: Readonly<{
    readonly Prepared: "prepared";
    readonly Queued: "queued";
    readonly Running: "running";
    readonly Succeeded: "succeeded";
    readonly Failed: "failed";
    readonly Cancelled: "cancelled";
    readonly LaunchFailed: "launch_failed";
}>;
export type CommandCellPhase = (typeof CommandCellPhase)[keyof typeof CommandCellPhase];
export declare function commandCellPhaseAsStr(phase: CommandCellPhase): string;
export declare function commandCellPhaseIsTerminal(phase: CommandCellPhase): boolean;
