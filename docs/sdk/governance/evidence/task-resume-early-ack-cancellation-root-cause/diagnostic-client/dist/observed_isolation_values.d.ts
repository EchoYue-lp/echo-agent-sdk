/** Observed Subagent isolation value without owning isolation execution. */
export declare class ObservedIsolation {
    readonly value: string;
    private constructor();
    static new(value: string): ObservedIsolation;
    static default(): ObservedIsolation;
    asStr(): string;
}
