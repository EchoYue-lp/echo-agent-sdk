export type ResourceLimitsValue = Readonly<{
    cpuTimeSecs?: bigint;
    memoryBytes?: bigint;
    maxOutputBytes?: bigint;
    maxProcesses?: number;
    network: boolean;
    readOnlyPaths: readonly string[];
    writablePaths: readonly string[];
}>;
/** Sandbox resource policy value; process execution remains Rust-owned. */
export declare const ResourceLimits: Readonly<{
    default(): ResourceLimitsValue;
    strict(): ResourceLimitsValue;
    unrestricted(): ResourceLimitsValue;
}>;
