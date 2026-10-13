/** Prompt section diagnostics projected without owning prompt compilation. */
export interface PromptSectionDiagnostic {
    readonly id: string;
    readonly source: string;
}
export declare class PromptDiagnostics {
    readonly sections: PromptSectionDiagnostic[];
    constructor(sections?: readonly PromptSectionDiagnostic[]);
    record(id: string, source: string): void;
    count(id: string): number;
}
