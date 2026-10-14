/** Content-guard decisions projected without owning guard execution. */
export type ContentGuardResult = {
    readonly kind: "pass";
} | {
    readonly kind: "detected";
    readonly piiTypes: readonly string[];
} | {
    readonly kind: "rejected";
    readonly piiTypes: readonly string[];
} | {
    readonly kind: "redacted";
    readonly content: string;
};
export declare function contentGuardPass(): ContentGuardResult;
export declare function contentGuardDetected(piiTypes: readonly string[]): ContentGuardResult;
export declare function contentGuardRejected(piiTypes: readonly string[]): ContentGuardResult;
export declare function contentGuardRedacted(content: string): ContentGuardResult;
export declare function contentGuardResultIsRejected(result: ContentGuardResult): boolean;
