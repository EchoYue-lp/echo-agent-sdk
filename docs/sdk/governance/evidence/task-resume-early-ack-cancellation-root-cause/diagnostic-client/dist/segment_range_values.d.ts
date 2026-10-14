/** Message-cache segment range values without owning cache state. */
export declare class SegmentRange {
    readonly start: bigint;
    readonly end: bigint;
    constructor(start?: bigint, end?: bigint);
    len(): bigint;
    isEmpty(): boolean;
}
