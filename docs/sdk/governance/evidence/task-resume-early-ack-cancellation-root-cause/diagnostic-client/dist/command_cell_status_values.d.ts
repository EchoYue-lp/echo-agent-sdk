/** Command-cell terminal/artifact status values projected without ownership. */
export declare const CommandCellTerminalCause: Readonly<{
    readonly Exited: "exited";
    readonly TimedOut: "timed_out";
    readonly Cancelled: "cancelled";
    readonly LaunchFailed: "launch_failed";
    readonly WaitFailed: "wait_failed";
    readonly OutputDrainFailed: "output_drain_failed";
}>;
export type CommandCellTerminalCause = (typeof CommandCellTerminalCause)[keyof typeof CommandCellTerminalCause];
export declare function commandCellTerminalCauseAsStr(cause: CommandCellTerminalCause): string;
export declare const CommandCellArtifactStatus: Readonly<{
    readonly NotRequested: "not_requested";
    readonly Writing: "writing";
    readonly BelowThreshold: "below_threshold";
    readonly Available: "available";
    readonly Failed: "failed";
}>;
export type CommandCellArtifactStatus = (typeof CommandCellArtifactStatus)[keyof typeof CommandCellArtifactStatus];
export declare function commandCellArtifactStatusAsStr(status: CommandCellArtifactStatus): string;
