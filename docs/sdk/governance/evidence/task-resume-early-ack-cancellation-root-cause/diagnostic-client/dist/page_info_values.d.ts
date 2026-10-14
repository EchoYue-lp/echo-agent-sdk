import { ToolResultValue } from "./helpers.js";
/** Pagination metadata projection without owning collection state. */
export declare class PageInfo {
    readonly nextCursor: string | null;
    readonly truncated: boolean;
    readonly totalKnown: boolean;
    readonly total: bigint | null;
    readonly returned: bigint;
    constructor(nextCursor?: string | null, truncated?: boolean, totalKnown?: boolean, total?: bigint | null, returned?: bigint);
    applyTo(result: ToolResultValue): ToolResultValue;
}
