/** Tool output artifact configuration without owning artifact writing. */
export class ToolOutputArtifactConfigValue {
    rootDir;
    retention;
    thresholdBytes;
    maxAgeSecs;
    constructor(rootDir, retention, thresholdBytes, maxAgeSecs) {
        this.rootDir = rootDir;
        this.retention = retention;
        this.thresholdBytes = thresholdBytes;
        this.maxAgeSecs = maxAgeSecs;
        Object.freeze(this);
    }
    static new(rootDir, retention) {
        return new ToolOutputArtifactConfigValue(rootDir, retention, 1048576n, null);
    }
    static default() {
        const temp = process.env.TMPDIR || "/tmp";
        return new ToolOutputArtifactConfigValue(`${temp}/echo_agent_artifacts/tool-logs`, "temporary_1h", 1048576n, 3600n);
    }
    withThresholdBytes(value) {
        return new ToolOutputArtifactConfigValue(this.rootDir, this.retention, value > 0n ? value : 1n, this.maxAgeSecs);
    }
    withMaxAgeSecs(value) {
        return new ToolOutputArtifactConfigValue(this.rootDir, this.retention, this.thresholdBytes, value);
    }
}
//# sourceMappingURL=tool_output_artifact_config_values.js.map