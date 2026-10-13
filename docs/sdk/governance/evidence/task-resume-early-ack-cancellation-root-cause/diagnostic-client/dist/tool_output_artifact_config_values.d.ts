/** Tool output artifact configuration without owning artifact writing. */
export declare class ToolOutputArtifactConfigValue {
    readonly rootDir: string;
    readonly retention: string;
    readonly thresholdBytes: bigint;
    readonly maxAgeSecs: bigint | null;
    private constructor();
    static new(rootDir: string, retention: string): ToolOutputArtifactConfigValue;
    static default(): ToolOutputArtifactConfigValue;
    withThresholdBytes(value: bigint): ToolOutputArtifactConfigValue;
    withMaxAgeSecs(value: bigint | null): ToolOutputArtifactConfigValue;
}
